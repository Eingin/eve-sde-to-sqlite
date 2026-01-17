# EVE SDE to SQLite CLI Tool - Design Document

## Overview

A Rust CLI tool that downloads and converts EVE Online Static Data Export (SDE) JSONL files into a SQLite database optimized for app backend use cases with fast lookups and joins.

## Requirements

- Download latest SDE zip from EVE API endpoint (or use local directory)
- Convert all 55 JSONL files to normalized SQLite tables
- **Filter tables via `--include` or `--exclude` flags for slimmer databases**
- Localized text stored as separate columns per language (`name_en`, `name_de`, etc.)
- Nested arrays normalized into separate junction/child tables
- Foreign key constraints enforced for referential integrity
- Fail fast on errors with transaction rollback
- Progress bar per file during processing
- Parallel file parsing with sequential writes

## EVE API Endpoints

### Automation URLs

| Purpose | URL |
|---------|-----|
| Latest build info | `https://developers.eveonline.com/static-data/tranquility/latest.jsonl` |
| Latest JSONL zip (redirect) | `https://developers.eveonline.com/static-data/eve-online-static-data-latest-jsonl.zip` |
| Specific build zip | `https://developers.eveonline.com/static-data/tranquility/eve-online-static-data-{build}-jsonl.zip` |
| Schema changelog | `https://developers.eveonline.com/static-data/tranquility/schema-changelog.yaml` |
| Changes since last build | `https://developers.eveonline.com/static-data/tranquility/changes/{build}.jsonl` |

### Latest Build Response

```json
{"_key": "sde", "buildNumber": 3168731, "releaseDate": "2026-01-16T11:13:49Z"}
```

### HTTP Caching

The API supports `ETag` and `Last-Modified` headers. The `X-SDE-Build-Number` header is included in responses.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     eve-sde-to-sqlite                       │
├─────────────────────────────────────────────────────────────┤
│  CLI Layer (clap)                                           │
│    └── Subcommands: download, convert, sync                 │
├─────────────────────────────────────────────────────────────┤
│  Downloader (reqwest)                                       │
│    └── Fetch latest.jsonl for build number                  │
│    └── Download zip with progress bar                       │
│    └── Extract to temp/cache directory                      │
├─────────────────────────────────────────────────────────────┤
│  Schema Registry                                            │
│    └── Defines table schemas for each JSONL file            │
│    └── Maps nested structures → normalized tables           │
├─────────────────────────────────────────────────────────────┤
│  JSONL Parser (serde_json)                                  │
│    └── Stream-parse each file line by line                  │
│    └── Extract localized fields → separate columns          │
├─────────────────────────────────────────────────────────────┤
│  SQLite Writer (rusqlite)                                   │
│    └── Create tables with FK constraints                    │
│    └── Batch inserts within transaction                     │
│    └── Fail fast on any error, rollback                     │
├─────────────────────────────────────────────────────────────┤
│  Progress (indicatif)                                       │
│    └── Download progress bar                                │
│    └── Per-file conversion progress bar                     │
└─────────────────────────────────────────────────────────────┘
```

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled"] }
rayon = "1.10"
indicatif = { version = "0.17", features = ["rayon"] }
anyhow = "1"
crossbeam-channel = "0.5"
reqwest = { version = "0.12", features = ["blocking", "stream"] }
zip = "2"
tempfile = "3"
directories = "5"   # For cache directory (~/.cache/eve-sde-to-sqlite)
```

## CLI Interface

```
eve-sde-to-sqlite 0.1.0
Convert EVE Online SDE to SQLite database

USAGE:
    eve-sde-to-sqlite <COMMAND>

COMMANDS:
    download    Download latest SDE zip file
    convert     Convert JSONL files to SQLite database
    sync        Download (if needed) and convert to SQLite (default workflow)
    help        Print help

EXAMPLES:
    # Download and convert in one step (most common)
    eve-sde-to-sqlite sync eve.db

    # Download only
    eve-sde-to-sqlite download --output ./sde-data/

    # Convert from local directory
    eve-sde-to-sqlite convert ./sde-data/ eve.db

    # Force re-download even if cached
    eve-sde-to-sqlite sync eve.db --force
```

### Subcommand: sync (default workflow)

```
eve-sde-to-sqlite sync <OUTPUT_DB>

Downloads latest SDE (if not cached) and converts to SQLite.

ARGS:
    <OUTPUT_DB>    Output SQLite database path

OPTIONS:
    -i, --include <TABLES>  Only include these tables (comma-separated)
    -e, --exclude <TABLES>  Exclude these tables (comma-separated)
    -f, --force             Force re-download even if cached
    -c, --cache-dir         Custom cache directory (default: ~/.cache/eve-sde-to-sqlite)
    -h, --help              Print help

EXAMPLES:
    # Only include specific tables (dependencies auto-included)
    eve-sde-to-sqlite sync eve.db --include types,blueprints,groups

    # Exclude large map tables for smaller database
    eve-sde-to-sqlite sync eve.db --exclude mapMoons,mapPlanets,mapAsteroidBelts
```

### Subcommand: download

```
eve-sde-to-sqlite download [OPTIONS]

Downloads latest SDE zip file.

OPTIONS:
    -o, --output <DIR>    Output directory (default: ~/.cache/eve-sde-to-sqlite)
    -f, --force           Force re-download even if cached
    -h, --help            Print help
```

### Subcommand: convert

```
eve-sde-to-sqlite convert <INPUT_DIR> <OUTPUT_DB>

Converts local JSONL files to SQLite database.

ARGS:
    <INPUT_DIR>    Directory containing JSONL files
    <OUTPUT_DB>    Output SQLite database path

OPTIONS:
    -i, --include <TABLES>  Only include these tables (comma-separated)
    -e, --exclude <TABLES>  Exclude these tables (comma-separated)
    -h, --help              Print help
```

### Subcommand: list-tables

```
eve-sde-to-sqlite list-tables

Lists all available tables that can be used with --include/--exclude.

OUTPUT:
    categories
    groups
    types
    ...
```

## Download Flow

```
1. Check latest build number
   GET https://developers.eveonline.com/static-data/tranquility/latest.jsonl
   → {"_key": "sde", "buildNumber": 3168731, "releaseDate": "..."}

2. Check cache
   ~/.cache/eve-sde-to-sqlite/3168731/
   If exists and valid → skip download

3. Download zip (with progress bar)
   GET https://developers.eveonline.com/static-data/eve-online-static-data-latest-jsonl.zip
   → Follow 302 redirect
   → Stream to temp file with progress bar
   "Downloading SDE build 3168731 [=====>    ] 45.2 MB / 89.1 MB"

4. Extract zip
   → Extract to ~/.cache/eve-sde-to-sqlite/3168731/
   "Extracting [=====>    ] 32/55 files"

5. Return path to extracted directory
```

## Table Mapping Strategy

### Primary Tables (1:1 with JSONL files)

| JSONL File | Table Name | Primary Key |
|------------|------------|-------------|
| `types.jsonl` | `types` | `id` (from `_key`) |
| `groups.jsonl` | `groups` | `id` |
| `categories.jsonl` | `categories` | `id` |
| `mapRegions.jsonl` | `map_regions` | `id` |
| `mapSolarSystems.jsonl` | `map_solar_systems` | `id` |
| `mapConstellations.jsonl` | `map_constellations` | `id` |
| `mapPlanets.jsonl` | `map_planets` | `id` |
| `mapMoons.jsonl` | `map_moons` | `id` |
| `mapStars.jsonl` | `map_stars` | `id` |
| `mapStargates.jsonl` | `map_stargates` | `id` |
| `mapAsteroidBelts.jsonl` | `map_asteroid_belts` | `id` |
| `factions.jsonl` | `factions` | `id` |
| `races.jsonl` | `races` | `id` |
| `bloodlines.jsonl` | `bloodlines` | `id` |
| `ancestries.jsonl` | `ancestries` | `id` |
| `blueprints.jsonl` | `blueprints` | `id` |
| `dogmaAttributes.jsonl` | `dogma_attributes` | `id` |
| `dogmaEffects.jsonl` | `dogma_effects` | `id` |
| `dogmaUnits.jsonl` | `dogma_units` | `id` |
| `dogmaAttributeCategories.jsonl` | `dogma_attribute_categories` | `id` |
| `icons.jsonl` | `icons` | `id` |
| `graphics.jsonl` | `graphics` | `id` |
| `marketGroups.jsonl` | `market_groups` | `id` |
| `metaGroups.jsonl` | `meta_groups` | `id` |
| `npcCorporations.jsonl` | `npc_corporations` | `id` |
| `npcStations.jsonl` | `npc_stations` | `id` |
| `npcCharacters.jsonl` | `npc_characters` | `id` |
| `skins.jsonl` | `skins` | `id` |
| `skinLicenses.jsonl` | `skin_licenses` | `id` |
| `skinMaterials.jsonl` | `skin_materials` | `id` |
| `certificates.jsonl` | `certificates` | `id` |
| `stationOperations.jsonl` | `station_operations` | `id` |
| `stationServices.jsonl` | `station_services` | `id` |
| `planetSchematics.jsonl` | `planet_schematics` | `id` |
| `planetResources.jsonl` | `planet_resources` | `id` |

### Junction/Child Tables (normalized from nested arrays)

| Parent | Child Table | Columns |
|--------|-------------|---------|
| `typeDogma.jsonl` | `type_dogma_attributes` | `type_id, attribute_id, value` |
| `typeDogma.jsonl` | `type_dogma_effects` | `type_id, effect_id, is_default` |
| `typeMaterials.jsonl` | `type_materials` | `type_id, material_type_id, quantity` |
| `typeBonus.jsonl` | `type_role_bonuses` | `type_id, bonus, bonus_text_en, ..., importance, unit_id` |
| `typeBonus.jsonl` | `type_skill_bonuses` | `type_id, skill_type_id, bonus, bonus_text_en, ..., importance, unit_id` |
| `blueprints.jsonl` | `blueprint_activities` | `blueprint_id, activity, time` |
| `blueprints.jsonl` | `blueprint_materials` | `blueprint_id, activity, type_id, quantity` |
| `blueprints.jsonl` | `blueprint_products` | `blueprint_id, activity, type_id, quantity, probability` |
| `blueprints.jsonl` | `blueprint_skills` | `blueprint_id, activity, type_id, level` |
| `masteries.jsonl` | `masteries` | `type_id, mastery_level, certificate_id` |
| `npcCorporationDivisions.jsonl` | `npc_corporation_divisions` | `corporation_id, division_id, size` |
| `contrabandTypes.jsonl` | `contraband_types` | `faction_id, type_id, standing_loss, fine_by_value` |

### Localized Columns Example

```sql
CREATE TABLE types (
    id INTEGER PRIMARY KEY,
    group_id INTEGER REFERENCES groups(id),
    mass REAL,
    volume REAL,
    radius REAL,
    portion_size INTEGER,
    published INTEGER,
    -- Localized name
    name_en TEXT,
    name_de TEXT,
    name_es TEXT,
    name_fr TEXT,
    name_ja TEXT,
    name_ko TEXT,
    name_ru TEXT,
    name_zh TEXT,
    -- Localized description
    description_en TEXT,
    description_de TEXT,
    description_es TEXT,
    description_fr TEXT,
    description_ja TEXT,
    description_ko TEXT,
    description_ru TEXT,
    description_zh TEXT
);
```

## Table Filtering

### Include/Exclude Logic

The CLI supports `--include` and `--exclude` flags to filter which tables are converted:

```bash
# Only convert types and blueprints (plus their FK dependencies)
eve-sde-to-sqlite sync eve.db --include types,blueprints

# Convert everything except large map tables
eve-sde-to-sqlite sync eve.db --exclude mapMoons,mapPlanets,mapAsteroidBelts
```

**Rules:**
- `--include` and `--exclude` are mutually exclusive
- Table names use snake_case (matching SQLite table names)
- Child/junction tables are automatically included with their parent

### Automatic Dependency Resolution

When using `--include`, the tool automatically includes FK parent tables:

```
User requests: --include types

Dependency chain:
  types → groups → categories

Auto-included tables:
  ✓ types
  ✓ groups (FK parent of types)
  ✓ categories (FK parent of groups)
```

**Example expansion:**

| User Request | Auto-Included Dependencies | Final Tables |
|--------------|---------------------------|--------------|
| `--include types` | groups, categories | types, groups, categories |
| `--include blueprints` | types, groups, categories | blueprints, blueprint_*, types, groups, categories |
| `--include map_solar_systems` | map_constellations, map_regions | map_solar_systems, map_constellations, map_regions |

### Filter Resolution Flow

```
1. Parse --include or --exclude list
2. If --include:
   a. Start with requested tables
   b. For each table, traverse FK dependencies
   c. Add all parent tables recursively
   d. Include child/junction tables of included parents
3. If --exclude:
   a. Start with all tables
   b. Remove excluded tables
   c. Remove orphaned child tables (parent excluded)
4. Validate no FK violations in final set
5. Print resolved table list before processing
```

### Output on Filtered Run

```
$ eve-sde-to-sqlite sync eve.db --include types,blueprints

Resolving dependencies...
  Requested: types, blueprints
  Auto-included: groups, categories (FK dependencies)
  Child tables: type_dogma_attributes, type_materials, blueprint_materials, ...

Converting 12 tables (excluded 43):
  categories         [========] 47/47
  groups             [========] 1,892/1,892
  types              [========] 51,134/51,134
  ...

Created eve.db (12 tables, 142,847 records) in 1.2s
```

## Processing Flow

### Parallel Processing with Dependency Waves

```
1. CLI Invocation
   $ eve-sde-to-sqlite sync eve.db

2. Download Phase
   - Fetch latest.jsonl for build number
   - Check cache for existing download
   - Download and extract zip if needed
   - Show download + extraction progress bars

3. Discovery Phase
   - Scan extracted directory for *.jsonl files
   - Count total lines per file (for progress bars)
   - Build dependency graph

4. Schema Creation Phase (single transaction, sequential)
   - Create all tables in dependency order
   - Create indexes on foreign key columns
   - Commit schema transaction

5. Data Import Phase (parallel with dependency waves)
   
   Wave 1 (independent tables, parallel):
   ┌─────────────┬─────────────┬─────────────┬─────────────┐
   │ categories  │ races       │ dogma_units │ icons       │
   │ [=====>   ] │ [========>] │ [======>  ] │ [====>    ] │
   └─────────────┴─────────────┴─────────────┴─────────────┘
   
   Wave 2 (depends on Wave 1, parallel):
   ┌─────────────┬─────────────┬─────────────┐
   │ groups      │ factions    │ map_regions │
   │ [=====>   ] │ [========>] │ [======>  ] │
   └─────────────┴─────────────┴─────────────┘
   
   Wave 3 (depends on Wave 2, parallel):
   ┌─────────────┬──────────────────┬───────────────────┐
   │ types       │ map_solar_systems│ map_constellations│
   │ [=====>   ] │ [========>      ]│ [======>         ]│
   └─────────────┴──────────────────┴───────────────────┘
   
   ... more waves ...

6. Finalization
   - Print summary: "Created eve.db (45 tables, 658,978 records) in 3.2s"
   - Print build info: "SDE build 3168731 (2026-01-16)"
```

### Write Strategy

Parse files in parallel, write sequentially:
- Parse/transform files in parallel using `rayon`
- Queue rows via `crossbeam-channel` 
- Single writer thread batches inserts (1000 rows per batch)
- Gets speedup from parallel JSON parsing (CPU-bound)
- Avoids SQLite write lock contention

## Project Structure

```
eve-sde-to-sqlite/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point, arg parsing
│   ├── lib.rs               # Public API
│   ├── cli.rs               # Clap command definitions
│   ├── download/
│   │   ├── mod.rs           # Download orchestration
│   │   ├── client.rs        # HTTP client, API requests
│   │   ├── cache.rs         # Cache directory management
│   │   └── extract.rs       # Zip extraction
│   ├── discovery.rs         # Scan dir, count lines, build dep graph
│   ├── filter.rs            # Table filtering (--include/--exclude)
│   ├── schema/
│   │   ├── mod.rs           # Schema registry, table definitions
│   │   ├── tables.rs        # Individual table schemas
│   │   └── dependencies.rs  # Dependency graph, wave ordering, FK resolution
│   ├── parser/
│   │   ├── mod.rs           # JSONL parsing traits
│   │   ├── localized.rs     # Handle localized text extraction
│   │   └── nested.rs        # Flatten nested arrays to child rows
│   ├── writer/
│   │   ├── mod.rs           # SQLite writer, batch inserts
│   │   └── pool.rs          # Write queue for parallel parsing
│   └── progress.rs          # Multi-progress bar handling
```

## Cache Directory

Default: `~/.cache/eve-sde-to-sqlite/` (Linux/macOS) or `%LOCALAPPDATA%\eve-sde-to-sqlite\` (Windows)

```
~/.cache/eve-sde-to-sqlite/
├── 3168731/                  # Build number directory
│   ├── types.jsonl
│   ├── groups.jsonl
│   ├── ...
│   └── _sde.jsonl
├── 3167890/                  # Previous build (can be cleaned)
│   └── ...
└── latest.json               # Cached latest build info
```

## Data Source Reference

- EVE Online SDE documentation: https://developers.eveonline.com/docs/services/static-data/
- Latest build API: https://developers.eveonline.com/static-data/tranquility/latest.jsonl
- Current build number: 3168731
- Total files: 55 JSONL files
- Total records: ~660,000
- Largest files: mapMoons.jsonl (342K), mapPlanets.jsonl (68K), types.jsonl (51K)
