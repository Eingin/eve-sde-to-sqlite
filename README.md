# eve-sde-to-sqlite

A fast CLI tool to download and convert EVE Online's Static Data Export (SDE) to a normalized SQLite database.

## Features

- Downloads the latest SDE directly from EVE API
- Caches downloads to avoid redundant transfers
- Converts JSONL files to a normalized SQLite schema
- Localized text fields expanded to per-language columns (`name_en`, `name_de`, etc.)
- Foreign key relationships between tables
- Selective table inclusion/exclusion with automatic dependency resolution

## Installation

```bash
# Clone and build
git clone https://github.com/youruser/eve-sde-to-sqlite
cd eve-sde-to-sqlite
cargo build --release

# Binary will be at target/release/eve-sde-to-sqlite
```

## Usage

### Download and Convert (Recommended)

```bash
# Download latest SDE and create SQLite database
eve-sde-to-sqlite sync eve.db

# Include only specific tables (dependencies auto-resolved)
eve-sde-to-sqlite sync eve.db --include types,groups,categories

# Exclude specific tables
eve-sde-to-sqlite sync eve.db --exclude blueprints,certificates

# Force re-download even if cached
eve-sde-to-sqlite sync eve.db --force
```

### Convert Local Files

If you already have JSONL files extracted:

```bash
eve-sde-to-sqlite convert /path/to/sde-jsonl eve.db
eve-sde-to-sqlite convert /path/to/sde-jsonl eve.db --include types,groups
```

### Download Only

```bash
eve-sde-to-sqlite download
eve-sde-to-sqlite download --output /custom/path
```

### List Available Tables

```bash
eve-sde-to-sqlite list-tables
```

## Available Tables

The tool supports 41 tables covering:

| Category | Tables |
|----------|--------|
| **Core** | categories, groups, types, meta_groups |
| **Market** | market_groups |
| **Industry** | blueprints, blueprint_materials, blueprint_products, blueprint_skills |
| **Dogma** | dogma_attributes, dogma_effects, dogma_units, dogma_attribute_categories, type_dogma_attributes |
| **Materials** | type_materials |
| **NPCs** | factions, npc_corporations, races, bloodlines, ancestries |
| **Characters** | character_attributes, certificates |
| **Map** | map_regions, map_constellations, map_solar_systems, map_stars, map_planets, map_moons, map_asteroid_belts, map_stargates |
| **Stations** | npc_stations, station_operations, station_services |
| **Skins** | skins, skin_licenses, skin_materials |
| **Other** | icons, graphics, agent_types, corporation_activities, translation_languages |

## Database Schema

### Localized Fields

Fields with translations are expanded into separate columns:
- `name` → `name_en`, `name_de`, `name_es`, `name_fr`, `name_ja`, `name_ko`, `name_ru`, `name_zh`
- `description` → `description_en`, `description_de`, etc.

### Foreign Keys

All relationships are enforced with foreign key constraints. When using `--include`, required parent tables are automatically added.

### Example Queries

```sql
-- Find a type with its group and category
SELECT t.id, t.name_en, g.name_en as group_name, c.name_en as category_name
FROM types t
JOIN groups g ON t.group_id = g.id
JOIN categories c ON g.category_id = c.id
WHERE t.name_en = 'Rifter';

-- Find a solar system with constellation and region
SELECT s.name_en as system, c.name_en as constellation, r.name_en as region
FROM map_solar_systems s
JOIN map_constellations c ON s.constellation_id = c.id
JOIN map_regions r ON c.region_id = r.id
WHERE s.name_en = 'Jita';

-- Find all ships (category 6 = Ship)
SELECT t.id, t.name_en, g.name_en as group_name
FROM types t
JOIN groups g ON t.group_id = g.id
WHERE g.category_id = 6 AND t.published = 1;

-- Get dogma attributes for a specific type
SELECT t.name_en, da.name, tda.value
FROM type_dogma_attributes tda
JOIN types t ON tda.type_id = t.id
JOIN dogma_attributes da ON tda.attribute_id = da.id
WHERE t.name_en = 'Rifter';

-- Get reprocessing materials for an ore
SELECT t1.name_en as ore, t2.name_en as mineral, tm.quantity
FROM type_materials tm
JOIN types t1 ON tm.type_id = t1.id
JOIN types t2 ON tm.material_type_id = t2.id
WHERE t1.name_en = 'Veldspar';

-- Get blueprint manufacturing requirements
SELECT b.id, t1.name_en as blueprint, bm.activity, t2.name_en as material, bm.quantity
FROM blueprint_materials bm
JOIN blueprints b ON bm.blueprint_id = b.id
JOIN types t1 ON b.id = t1.id
JOIN types t2 ON bm.type_id = t2.id
WHERE t1.name_en LIKE 'Rifter%' AND bm.activity = 'manufacturing';
```

## Cache Location

Downloaded SDE files are cached at:
- **macOS**: `~/Library/Caches/eve-sde-to-sqlite/`
- **Linux**: `~/.cache/eve-sde-to-sqlite/`
- **Windows**: `%LOCALAPPDATA%\eve-sde-to-sqlite\cache\`

Use `--cache-dir` to specify a custom location.

## Development

### Running Tests

Unit tests:
```bash
cargo test
```

Integration tests (require JSONL test data):
```bash
# Download and extract SDE first, then:
EVE_SDE_TEST_DATA=/path/to/jsonl-files cargo test --test integration_test -- --ignored
```

### Building

```bash
cargo build --release
```

## License

MIT
