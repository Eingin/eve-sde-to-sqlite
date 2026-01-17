# EVE SDE to SQLite Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI that downloads EVE Online SDE data and converts JSONL files to a normalized SQLite database with table filtering support.

**Architecture:** CLI with subcommands (sync, download, convert, list-tables). Downloads SDE zip from EVE API with caching. Parses JSONL files in parallel waves based on FK dependencies. Writes to SQLite with batch inserts. Supports --include/--exclude for table filtering with auto-dependency resolution.

**Tech Stack:** Rust, clap (CLI), serde/serde_json (parsing), rusqlite (SQLite), reqwest (HTTP), zip (extraction), rayon (parallelism), indicatif (progress bars), anyhow (errors)

---

## Phase 1: Project Setup

### Task 1.1: Initialize Cargo Project

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/lib.rs`

**Step 1: Initialize cargo project**

Run:
```bash
cd ~/Developer/eve-sde-to-sqlite
cargo init
```

**Step 2: Update Cargo.toml with dependencies**

Replace `Cargo.toml` with:
```toml
[package]
name = "eve-sde-to-sqlite"
version = "0.1.0"
edition = "2021"
description = "Convert EVE Online SDE JSONL files to SQLite database"
license = "MIT"

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
directories = "5"

[dev-dependencies]
tempfile = "3"
```

**Step 3: Create minimal lib.rs**

Create `src/lib.rs`:
```rust
pub mod cli;
```

**Step 4: Create minimal main.rs**

Create `src/main.rs`:
```rust
use anyhow::Result;

fn main() -> Result<()> {
    println!("eve-sde-to-sqlite");
    Ok(())
}
```

**Step 5: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 6: Commit**

```bash
git add Cargo.toml src/
git commit -m "chore: initialize cargo project with dependencies"
```

---

### Task 1.2: Implement CLI Argument Parsing

**Files:**
- Create: `src/cli.rs`
- Modify: `src/main.rs`
- Modify: `src/lib.rs`

**Step 1: Create CLI module with clap derive**

Create `src/cli.rs`:
```rust
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "eve-sde-to-sqlite")]
#[command(version, about = "Convert EVE Online SDE to SQLite database")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download (if needed) and convert to SQLite
    Sync {
        /// Output SQLite database path
        output_db: PathBuf,

        /// Only include these tables (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        include: Option<Vec<String>>,

        /// Exclude these tables (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,

        /// Force re-download even if cached
        #[arg(short, long)]
        force: bool,

        /// Custom cache directory
        #[arg(short, long)]
        cache_dir: Option<PathBuf>,
    },

    /// Download latest SDE zip file
    Download {
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Force re-download even if cached
        #[arg(short, long)]
        force: bool,
    },

    /// Convert local JSONL files to SQLite database
    Convert {
        /// Directory containing JSONL files
        input_dir: PathBuf,

        /// Output SQLite database path
        output_db: PathBuf,

        /// Only include these tables (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        include: Option<Vec<String>>,

        /// Exclude these tables (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        exclude: Option<Vec<String>>,
    },

    /// List all available table names
    ListTables,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
```

**Step 2: Update lib.rs**

Update `src/lib.rs`:
```rust
pub mod cli;

pub use cli::{Cli, Commands};
```

**Step 3: Update main.rs to use CLI**

Update `src/main.rs`:
```rust
use anyhow::Result;
use eve_sde_to_sqlite::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse_args();

    match cli.command {
        Commands::Sync { output_db, include, exclude, force, cache_dir } => {
            println!("Sync: {:?}", output_db);
            println!("  include: {:?}", include);
            println!("  exclude: {:?}", exclude);
            println!("  force: {}", force);
            println!("  cache_dir: {:?}", cache_dir);
        }
        Commands::Download { output, force } => {
            println!("Download: {:?}, force: {}", output, force);
        }
        Commands::Convert { input_dir, output_db, include, exclude } => {
            println!("Convert: {:?} -> {:?}", input_dir, output_db);
            println!("  include: {:?}", include);
            println!("  exclude: {:?}", exclude);
        }
        Commands::ListTables => {
            println!("Available tables:");
            println!("  (not implemented yet)");
        }
    }

    Ok(())
}
```

**Step 4: Verify CLI parsing works**

Run: `cargo run -- --help`
Expected: Shows help with subcommands

Run: `cargo run -- sync test.db --include types,groups`
Expected: Prints parsed arguments

**Step 5: Commit**

```bash
git add src/
git commit -m "feat: add CLI argument parsing with clap"
```

---

## Phase 2: Schema Registry

### Task 2.1: Define Table Schema Types

**Files:**
- Create: `src/schema/mod.rs`
- Create: `src/schema/types.rs`
- Modify: `src/lib.rs`

**Step 1: Create schema types**

Create `src/schema/types.rs`:
```rust
use std::collections::HashSet;

/// Supported languages for localized text
pub const LANGUAGES: &[&str] = &["en", "de", "es", "fr", "ja", "ko", "ru", "zh"];

/// Column data type
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Integer,
    Real,
    Text,
    Boolean,
    /// Localized text expands to multiple columns (name_en, name_de, etc.)
    Localized,
    /// JSON blob stored as text
    Json,
}

/// Column definition
#[derive(Debug, Clone)]
pub struct Column {
    pub name: &'static str,
    pub col_type: ColumnType,
    pub nullable: bool,
}

impl Column {
    pub const fn new(name: &'static str, col_type: ColumnType) -> Self {
        Self { name, col_type, nullable: true }
    }

    pub const fn required(name: &'static str, col_type: ColumnType) -> Self {
        Self { name, col_type, nullable: false }
    }
}

/// Foreign key reference
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub column: &'static str,
    pub references_table: &'static str,
    pub references_column: &'static str,
}

impl ForeignKey {
    pub const fn new(column: &'static str, references_table: &'static str) -> Self {
        Self {
            column,
            references_table,
            references_column: "id",
        }
    }
}

/// Table schema definition
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: &'static str,
    pub source_file: &'static str,
    pub columns: &'static [Column],
    pub foreign_keys: &'static [ForeignKey],
    /// Child tables derived from nested arrays
    pub child_tables: &'static [&'static str],
}

impl TableSchema {
    /// Get all tables this table depends on (FK parents)
    pub fn dependencies(&self) -> HashSet<&'static str> {
        self.foreign_keys
            .iter()
            .map(|fk| fk.references_table)
            .collect()
    }
}
```

**Step 2: Create schema mod.rs**

Create `src/schema/mod.rs`:
```rust
pub mod types;

pub use types::*;
```

**Step 3: Update lib.rs**

Update `src/lib.rs`:
```rust
pub mod cli;
pub mod schema;

pub use cli::{Cli, Commands};
```

**Step 4: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/schema/
git commit -m "feat: add schema type definitions"
```

---

### Task 2.2: Define All Table Schemas

**Files:**
- Create: `src/schema/tables.rs`
- Modify: `src/schema/mod.rs`

**Step 1: Create table definitions**

Create `src/schema/tables.rs`:
```rust
use super::types::*;

// =============================================================================
// Independent Tables (no FK dependencies)
// =============================================================================

pub static CATEGORIES: TableSchema = TableSchema {
    name: "categories",
    source_file: "categories.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("published", ColumnType::Boolean),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static DOGMA_ATTRIBUTE_CATEGORIES: TableSchema = TableSchema {
    name: "dogma_attribute_categories",
    source_file: "dogmaAttributeCategories.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("description", ColumnType::Text),
        Column::new("name", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static DOGMA_UNITS: TableSchema = TableSchema {
    name: "dogma_units",
    source_file: "dogmaUnits.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("description", ColumnType::Text),
        Column::new("display_name", ColumnType::Text),
        Column::new("name", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static ICONS: TableSchema = TableSchema {
    name: "icons",
    source_file: "icons.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("description", ColumnType::Text),
        Column::new("icon_file", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static GRAPHICS: TableSchema = TableSchema {
    name: "graphics",
    source_file: "graphics.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("description", ColumnType::Text),
        Column::new("graphic_file", ColumnType::Text),
        Column::new("sof_faction_name", ColumnType::Text),
        Column::new("sof_hull_name", ColumnType::Text),
        Column::new("sof_race_name", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static AGENT_TYPES: TableSchema = TableSchema {
    name: "agent_types",
    source_file: "agentTypes.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static STATION_SERVICES: TableSchema = TableSchema {
    name: "station_services",
    source_file: "stationServices.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("service_name", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static CORPORATION_ACTIVITIES: TableSchema = TableSchema {
    name: "corporation_activities",
    source_file: "corporationActivities.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static META_GROUPS: TableSchema = TableSchema {
    name: "meta_groups",
    source_file: "metaGroups.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("icon_suffix", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static CHARACTER_ATTRIBUTES: TableSchema = TableSchema {
    name: "character_attributes",
    source_file: "characterAttributes.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("short_description", ColumnType::Localized),
        Column::new("notes", ColumnType::Text),
        Column::new("icon_id", ColumnType::Integer),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static TRANSLATION_LANGUAGES: TableSchema = TableSchema {
    name: "translation_languages",
    source_file: "translationLanguages.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static SKIN_MATERIALS: TableSchema = TableSchema {
    name: "skin_materials",
    source_file: "skinMaterials.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("display_name", ColumnType::Localized),
        Column::new("material_set_id", ColumnType::Integer),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

// =============================================================================
// Level 1 Dependencies
// =============================================================================

pub static RACES: TableSchema = TableSchema {
    name: "races",
    source_file: "races.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("ship_type_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("icon_id", "icons"),
    ],
    child_tables: &[],
};

pub static GROUPS: TableSchema = TableSchema {
    name: "groups",
    source_file: "groups.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("category_id", ColumnType::Integer),
        Column::new("published", ColumnType::Boolean),
        Column::new("anchorable", ColumnType::Boolean),
        Column::new("anchored", ColumnType::Boolean),
        Column::new("fittable_non_singleton", ColumnType::Boolean),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("use_base_price", ColumnType::Boolean),
    ],
    foreign_keys: &[
        ForeignKey::new("category_id", "categories"),
        ForeignKey::new("icon_id", "icons"),
    ],
    child_tables: &[],
};

pub static DOGMA_ATTRIBUTES: TableSchema = TableSchema {
    name: "dogma_attributes",
    source_file: "dogmaAttributes.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("description", ColumnType::Text),
        Column::new("display_name", ColumnType::Localized),
        Column::new("tooltip_title", ColumnType::Localized),
        Column::new("tooltip_description", ColumnType::Localized),
        Column::new("attribute_category_id", ColumnType::Integer),
        Column::new("data_type", ColumnType::Integer),
        Column::new("default_value", ColumnType::Real),
        Column::new("display_when_zero", ColumnType::Boolean),
        Column::new("high_is_good", ColumnType::Boolean),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("published", ColumnType::Boolean),
        Column::new("stackable", ColumnType::Boolean),
        Column::new("unit_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("attribute_category_id", "dogma_attribute_categories"),
        ForeignKey::new("icon_id", "icons"),
        ForeignKey::new("unit_id", "dogma_units"),
    ],
    child_tables: &[],
};

pub static DOGMA_EFFECTS: TableSchema = TableSchema {
    name: "dogma_effects",
    source_file: "dogmaEffects.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
        Column::new("description", ColumnType::Text),
        Column::new("display_name", ColumnType::Localized),
        Column::new("effect_category", ColumnType::Integer),
        Column::new("effect_name", ColumnType::Text),
        Column::new("guid", ColumnType::Text),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("is_assistance", ColumnType::Boolean),
        Column::new("is_offensive", ColumnType::Boolean),
        Column::new("is_warp_safe", ColumnType::Boolean),
        Column::new("published", ColumnType::Boolean),
        Column::new("range_chance", ColumnType::Boolean),
        Column::new("electronic_chance", ColumnType::Boolean),
        Column::new("propulsion_chance", ColumnType::Boolean),
        Column::new("disallow_auto_repeat", ColumnType::Boolean),
        Column::new("distribution", ColumnType::Integer),
        Column::new("duration_attribute_id", ColumnType::Integer),
        Column::new("discharge_attribute_id", ColumnType::Integer),
        Column::new("falloff_attribute_id", ColumnType::Integer),
        Column::new("range_attribute_id", ColumnType::Integer),
        Column::new("tracking_speed_attribute_id", ColumnType::Integer),
        Column::new("fitting_usage_chance_attribute_id", ColumnType::Integer),
        Column::new("resistance_attribute_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("icon_id", "icons"),
    ],
    child_tables: &[],
};

pub static MAP_REGIONS: TableSchema = TableSchema {
    name: "map_regions",
    source_file: "mapRegions.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("faction_id", ColumnType::Integer),
        Column::new("center_x", ColumnType::Real),
        Column::new("center_y", ColumnType::Real),
        Column::new("center_z", ColumnType::Real),
        Column::new("max_x", ColumnType::Real),
        Column::new("max_y", ColumnType::Real),
        Column::new("max_z", ColumnType::Real),
        Column::new("min_x", ColumnType::Real),
        Column::new("min_y", ColumnType::Real),
        Column::new("min_z", ColumnType::Real),
        Column::new("name_id", ColumnType::Integer),
        Column::new("description_id", ColumnType::Integer),
        Column::new("nebula", ColumnType::Integer),
        Column::new("wormhole_class_id", ColumnType::Integer),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static MARKET_GROUPS: TableSchema = TableSchema {
    name: "market_groups",
    source_file: "marketGroups.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("parent_group_id", ColumnType::Integer),
        Column::new("has_types", ColumnType::Boolean),
    ],
    foreign_keys: &[
        ForeignKey::new("icon_id", "icons"),
        ForeignKey::new("parent_group_id", "market_groups"),
    ],
    child_tables: &[],
};

pub static STATION_OPERATIONS: TableSchema = TableSchema {
    name: "station_operations",
    source_file: "stationOperations.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("operation_name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("activity_id", ColumnType::Integer),
        Column::new("border", ColumnType::Real),
        Column::new("corridor", ColumnType::Real),
        Column::new("fringe", ColumnType::Real),
        Column::new("hub", ColumnType::Real),
        Column::new("ratio", ColumnType::Real),
        Column::new("manufacturing_factor", ColumnType::Real),
        Column::new("research_factor", ColumnType::Real),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static SKINS: TableSchema = TableSchema {
    name: "skins",
    source_file: "skins.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("internal_name", ColumnType::Text),
        Column::new("skin_material_id", ColumnType::Integer),
        Column::new("allow_ccp_devs", ColumnType::Boolean),
        Column::new("visible_serenity", ColumnType::Boolean),
        Column::new("visible_tranquility", ColumnType::Boolean),
    ],
    foreign_keys: &[
        ForeignKey::new("skin_material_id", "skin_materials"),
    ],
    child_tables: &[],
};

// =============================================================================
// Level 2 Dependencies
// =============================================================================

pub static BLOODLINES: TableSchema = TableSchema {
    name: "bloodlines",
    source_file: "bloodlines.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("race_id", ColumnType::Integer),
        Column::new("corporation_id", ColumnType::Integer),
        Column::new("charisma", ColumnType::Integer),
        Column::new("intelligence", ColumnType::Integer),
        Column::new("memory", ColumnType::Integer),
        Column::new("perception", ColumnType::Integer),
        Column::new("willpower", ColumnType::Integer),
        Column::new("icon_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("race_id", "races"),
        ForeignKey::new("icon_id", "icons"),
    ],
    child_tables: &[],
};

pub static FACTIONS: TableSchema = TableSchema {
    name: "factions",
    source_file: "factions.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("short_description", ColumnType::Localized),
        Column::new("corporation_id", ColumnType::Integer),
        Column::new("militia_corporation_id", ColumnType::Integer),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("size_factor", ColumnType::Real),
        Column::new("unique_name", ColumnType::Boolean),
    ],
    foreign_keys: &[
        ForeignKey::new("icon_id", "icons"),
    ],
    child_tables: &[],
};

pub static NPC_CORPORATIONS: TableSchema = TableSchema {
    name: "npc_corporations",
    source_file: "npcCorporations.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("ceo_id", ColumnType::Integer),
        Column::new("station_id", ColumnType::Integer),
        Column::new("ticker_name", ColumnType::Text),
        Column::new("unique_name", ColumnType::Boolean),
        Column::new("deleted", ColumnType::Boolean),
        Column::new("extent", ColumnType::Text),
        Column::new("has_player_personnel_manager", ColumnType::Boolean),
        Column::new("initial_price", ColumnType::Real),
        Column::new("member_limit", ColumnType::Integer),
        Column::new("min_security", ColumnType::Real),
        Column::new("minimum_join_standing", ColumnType::Real),
        Column::new("send_char_termination_message", ColumnType::Boolean),
        Column::new("shares", ColumnType::Integer),
        Column::new("size", ColumnType::Text),
        Column::new("tax_rate", ColumnType::Real),
    ],
    foreign_keys: &[],
    child_tables: &[],
};

pub static MAP_CONSTELLATIONS: TableSchema = TableSchema {
    name: "map_constellations",
    source_file: "mapConstellations.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("region_id", ColumnType::Integer),
        Column::new("faction_id", ColumnType::Integer),
        Column::new("center_x", ColumnType::Real),
        Column::new("center_y", ColumnType::Real),
        Column::new("center_z", ColumnType::Real),
        Column::new("max_x", ColumnType::Real),
        Column::new("max_y", ColumnType::Real),
        Column::new("max_z", ColumnType::Real),
        Column::new("min_x", ColumnType::Real),
        Column::new("min_y", ColumnType::Real),
        Column::new("min_z", ColumnType::Real),
        Column::new("radius", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("region_id", "map_regions"),
    ],
    child_tables: &[],
};

pub static TYPES: TableSchema = TableSchema {
    name: "types",
    source_file: "types.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("group_id", ColumnType::Integer),
        Column::new("graphic_id", ColumnType::Integer),
        Column::new("icon_id", ColumnType::Integer),
        Column::new("market_group_id", ColumnType::Integer),
        Column::new("meta_group_id", ColumnType::Integer),
        Column::new("mass", ColumnType::Real),
        Column::new("volume", ColumnType::Real),
        Column::new("radius", ColumnType::Real),
        Column::new("packaged_volume", ColumnType::Real),
        Column::new("portion_size", ColumnType::Integer),
        Column::new("capacity", ColumnType::Real),
        Column::new("base_price", ColumnType::Real),
        Column::new("published", ColumnType::Boolean),
        Column::new("race_id", ColumnType::Integer),
        Column::new("faction_id", ColumnType::Integer),
        Column::new("sof_faction_name", ColumnType::Text),
        Column::new("sound_id", ColumnType::Integer),
        Column::new("variation_parent_type_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("group_id", "groups"),
        ForeignKey::new("graphic_id", "graphics"),
        ForeignKey::new("icon_id", "icons"),
        ForeignKey::new("market_group_id", "market_groups"),
        ForeignKey::new("meta_group_id", "meta_groups"),
        ForeignKey::new("race_id", "races"),
    ],
    child_tables: &[],
};

// =============================================================================
// Level 3 Dependencies
// =============================================================================

pub static ANCESTRIES: TableSchema = TableSchema {
    name: "ancestries",
    source_file: "ancestries.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("short_description", ColumnType::Localized),
        Column::new("bloodline_id", ColumnType::Integer),
        Column::new("charisma", ColumnType::Integer),
        Column::new("intelligence", ColumnType::Integer),
        Column::new("memory", ColumnType::Integer),
        Column::new("perception", ColumnType::Integer),
        Column::new("willpower", ColumnType::Integer),
        Column::new("icon_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("bloodline_id", "bloodlines"),
        ForeignKey::new("icon_id", "icons"),
    ],
    child_tables: &[],
};

pub static MAP_SOLAR_SYSTEMS: TableSchema = TableSchema {
    name: "map_solar_systems",
    source_file: "mapSolarSystems.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("constellation_id", ColumnType::Integer),
        Column::new("region_id", ColumnType::Integer),
        Column::new("star_id", ColumnType::Integer),
        Column::new("security_status", ColumnType::Real),
        Column::new("security_class", ColumnType::Text),
        Column::new("luminosity", ColumnType::Real),
        Column::new("radius", ColumnType::Real),
        Column::new("border", ColumnType::Boolean),
        Column::new("corridor", ColumnType::Boolean),
        Column::new("fringe", ColumnType::Boolean),
        Column::new("hub", ColumnType::Boolean),
        Column::new("international", ColumnType::Boolean),
        Column::new("regional", ColumnType::Boolean),
        Column::new("position_x", ColumnType::Real),
        Column::new("position_y", ColumnType::Real),
        Column::new("position_z", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("constellation_id", "map_constellations"),
        ForeignKey::new("region_id", "map_regions"),
    ],
    child_tables: &[],
};

pub static BLUEPRINTS: TableSchema = TableSchema {
    name: "blueprints",
    source_file: "blueprints.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("blueprint_type_id", ColumnType::Integer),
        Column::new("max_production_limit", ColumnType::Integer),
        Column::new("copying_time", ColumnType::Integer),
        Column::new("manufacturing_time", ColumnType::Integer),
        Column::new("research_material_time", ColumnType::Integer),
        Column::new("research_time_time", ColumnType::Integer),
        Column::new("invention_time", ColumnType::Integer),
        Column::new("reaction_time", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("blueprint_type_id", "types"),
    ],
    child_tables: &["blueprint_materials", "blueprint_products", "blueprint_skills"],
};

pub static SKIN_LICENSES: TableSchema = TableSchema {
    name: "skin_licenses",
    source_file: "skinLicenses.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("license_type_id", ColumnType::Integer),
        Column::new("skin_id", ColumnType::Integer),
        Column::new("duration", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("license_type_id", "types"),
        ForeignKey::new("skin_id", "skins"),
    ],
    child_tables: &[],
};

pub static CERTIFICATES: TableSchema = TableSchema {
    name: "certificates",
    source_file: "certificates.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("group_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("group_id", "groups"),
    ],
    child_tables: &[],
};

// =============================================================================
// Level 4 Dependencies (Map objects)
// =============================================================================

pub static MAP_STARS: TableSchema = TableSchema {
    name: "map_stars",
    source_file: "mapStars.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("type_id", ColumnType::Integer),
        Column::new("age", ColumnType::Real),
        Column::new("life", ColumnType::Real),
        Column::new("locked", ColumnType::Boolean),
        Column::new("luminosity", ColumnType::Real),
        Column::new("radius", ColumnType::Real),
        Column::new("spectral_class", ColumnType::Text),
        Column::new("temperature", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("solar_system_id", "map_solar_systems"),
        ForeignKey::new("type_id", "types"),
    ],
    child_tables: &[],
};

pub static MAP_PLANETS: TableSchema = TableSchema {
    name: "map_planets",
    source_file: "mapPlanets.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("type_id", ColumnType::Integer),
        Column::new("celestial_index", ColumnType::Integer),
        Column::new("orbit_id", ColumnType::Integer),
        Column::new("orbit_index", ColumnType::Integer),
        Column::new("radius", ColumnType::Real),
        Column::new("position_x", ColumnType::Real),
        Column::new("position_y", ColumnType::Real),
        Column::new("position_z", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("solar_system_id", "map_solar_systems"),
        ForeignKey::new("type_id", "types"),
    ],
    child_tables: &[],
};

pub static MAP_MOONS: TableSchema = TableSchema {
    name: "map_moons",
    source_file: "mapMoons.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("planet_id", ColumnType::Integer),
        Column::new("type_id", ColumnType::Integer),
        Column::new("celestial_index", ColumnType::Integer),
        Column::new("orbit_id", ColumnType::Integer),
        Column::new("orbit_index", ColumnType::Integer),
        Column::new("radius", ColumnType::Real),
        Column::new("position_x", ColumnType::Real),
        Column::new("position_y", ColumnType::Real),
        Column::new("position_z", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("solar_system_id", "map_solar_systems"),
        ForeignKey::new("planet_id", "map_planets"),
        ForeignKey::new("type_id", "types"),
    ],
    child_tables: &[],
};

pub static MAP_ASTEROID_BELTS: TableSchema = TableSchema {
    name: "map_asteroid_belts",
    source_file: "mapAsteroidBelts.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("planet_id", ColumnType::Integer),
        Column::new("type_id", ColumnType::Integer),
        Column::new("celestial_index", ColumnType::Integer),
        Column::new("orbit_id", ColumnType::Integer),
        Column::new("orbit_index", ColumnType::Integer),
        Column::new("position_x", ColumnType::Real),
        Column::new("position_y", ColumnType::Real),
        Column::new("position_z", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("solar_system_id", "map_solar_systems"),
        ForeignKey::new("planet_id", "map_planets"),
        ForeignKey::new("type_id", "types"),
    ],
    child_tables: &[],
};

pub static MAP_STARGATES: TableSchema = TableSchema {
    name: "map_stargates",
    source_file: "mapStargates.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("type_id", ColumnType::Integer),
        Column::new("destination_stargate_id", ColumnType::Integer),
        Column::new("destination_solar_system_id", ColumnType::Integer),
        Column::new("position_x", ColumnType::Real),
        Column::new("position_y", ColumnType::Real),
        Column::new("position_z", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("solar_system_id", "map_solar_systems"),
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("destination_solar_system_id", "map_solar_systems"),
    ],
    child_tables: &[],
};

pub static NPC_STATIONS: TableSchema = TableSchema {
    name: "npc_stations",
    source_file: "npcStations.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("type_id", ColumnType::Integer),
        Column::new("owner_id", ColumnType::Integer),
        Column::new("operation_id", ColumnType::Integer),
        Column::new("orbit_id", ColumnType::Integer),
        Column::new("celestial_index", ColumnType::Integer),
        Column::new("orbit_index", ColumnType::Integer),
        Column::new("reprocessing_efficiency", ColumnType::Real),
        Column::new("reprocessing_hangar_flag", ColumnType::Integer),
        Column::new("reprocessing_stations_take", ColumnType::Real),
        Column::new("use_operation_name", ColumnType::Boolean),
        Column::new("position_x", ColumnType::Real),
        Column::new("position_y", ColumnType::Real),
        Column::new("position_z", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("solar_system_id", "map_solar_systems"),
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("owner_id", "npc_corporations"),
        ForeignKey::new("operation_id", "station_operations"),
    ],
    child_tables: &[],
};

// =============================================================================
// Junction Tables (normalized from nested arrays)
// =============================================================================

pub static TYPE_DOGMA_ATTRIBUTES: TableSchema = TableSchema {
    name: "type_dogma_attributes",
    source_file: "typeDogma.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("attribute_id", ColumnType::Integer),
        Column::required("value", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("attribute_id", "dogma_attributes"),
    ],
    child_tables: &[],
};

pub static TYPE_MATERIALS: TableSchema = TableSchema {
    name: "type_materials",
    source_file: "typeMaterials.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("material_type_id", ColumnType::Integer),
        Column::required("quantity", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("material_type_id", "types"),
    ],
    child_tables: &[],
};

pub static BLUEPRINT_MATERIALS: TableSchema = TableSchema {
    name: "blueprint_materials",
    source_file: "blueprints.jsonl",
    columns: &[
        Column::required("blueprint_id", ColumnType::Integer),
        Column::required("activity", ColumnType::Text),
        Column::required("type_id", ColumnType::Integer),
        Column::required("quantity", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("blueprint_id", "blueprints"),
        ForeignKey::new("type_id", "types"),
    ],
    child_tables: &[],
};

pub static BLUEPRINT_PRODUCTS: TableSchema = TableSchema {
    name: "blueprint_products",
    source_file: "blueprints.jsonl",
    columns: &[
        Column::required("blueprint_id", ColumnType::Integer),
        Column::required("activity", ColumnType::Text),
        Column::required("type_id", ColumnType::Integer),
        Column::required("quantity", ColumnType::Integer),
        Column::new("probability", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("blueprint_id", "blueprints"),
        ForeignKey::new("type_id", "types"),
    ],
    child_tables: &[],
};

pub static BLUEPRINT_SKILLS: TableSchema = TableSchema {
    name: "blueprint_skills",
    source_file: "blueprints.jsonl",
    columns: &[
        Column::required("blueprint_id", ColumnType::Integer),
        Column::required("activity", ColumnType::Text),
        Column::required("type_id", ColumnType::Integer),
        Column::required("level", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("blueprint_id", "blueprints"),
        ForeignKey::new("type_id", "types"),
    ],
    child_tables: &[],
};

// =============================================================================
// Schema Registry
// =============================================================================

/// All table schemas in dependency order
pub static ALL_TABLES: &[&TableSchema] = &[
    // Wave 1: No dependencies
    &CATEGORIES,
    &DOGMA_ATTRIBUTE_CATEGORIES,
    &DOGMA_UNITS,
    &ICONS,
    &GRAPHICS,
    &AGENT_TYPES,
    &STATION_SERVICES,
    &CORPORATION_ACTIVITIES,
    &META_GROUPS,
    &CHARACTER_ATTRIBUTES,
    &TRANSLATION_LANGUAGES,
    &SKIN_MATERIALS,
    // Wave 2: Level 1 deps
    &RACES,
    &GROUPS,
    &DOGMA_ATTRIBUTES,
    &DOGMA_EFFECTS,
    &MAP_REGIONS,
    &MARKET_GROUPS,
    &STATION_OPERATIONS,
    &SKINS,
    // Wave 3: Level 2 deps
    &BLOODLINES,
    &FACTIONS,
    &NPC_CORPORATIONS,
    &MAP_CONSTELLATIONS,
    &TYPES,
    // Wave 4: Level 3 deps
    &ANCESTRIES,
    &MAP_SOLAR_SYSTEMS,
    &BLUEPRINTS,
    &SKIN_LICENSES,
    &CERTIFICATES,
    // Wave 5: Level 4 deps (map objects)
    &MAP_STARS,
    &MAP_PLANETS,
    &MAP_MOONS,
    &MAP_ASTEROID_BELTS,
    &MAP_STARGATES,
    &NPC_STATIONS,
    // Junction tables
    &TYPE_DOGMA_ATTRIBUTES,
    &TYPE_MATERIALS,
    &BLUEPRINT_MATERIALS,
    &BLUEPRINT_PRODUCTS,
    &BLUEPRINT_SKILLS,
];

/// Get table schema by name
pub fn get_table(name: &str) -> Option<&'static TableSchema> {
    ALL_TABLES.iter().find(|t| t.name == name).copied()
}

/// Get all table names
pub fn table_names() -> Vec<&'static str> {
    ALL_TABLES.iter().map(|t| t.name).collect()
}
```

**Step 2: Update schema mod.rs**

Update `src/schema/mod.rs`:
```rust
pub mod tables;
pub mod types;

pub use tables::*;
pub use types::*;
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/schema/
git commit -m "feat: add table schema definitions for all SDE tables"
```

---

### Task 2.3: Implement Dependency Resolution

**Files:**
- Create: `src/schema/dependencies.rs`
- Modify: `src/schema/mod.rs`

**Step 1: Create dependency resolver**

Create `src/schema/dependencies.rs`:
```rust
use std::collections::{HashMap, HashSet, VecDeque};
use super::tables::{ALL_TABLES, get_table};
use super::types::TableSchema;

/// Resolves table dependencies for filtering
pub struct DependencyResolver {
    /// Map of table name -> tables it depends on
    deps: HashMap<&'static str, HashSet<&'static str>>,
    /// Map of table name -> tables that depend on it
    reverse_deps: HashMap<&'static str, HashSet<&'static str>>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        let mut deps: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
        let mut reverse_deps: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        for table in ALL_TABLES {
            let table_deps = table.dependencies();
            deps.insert(table.name, table_deps.clone());
            
            for dep in table_deps {
                reverse_deps
                    .entry(dep)
                    .or_default()
                    .insert(table.name);
            }
        }

        Self { deps, reverse_deps }
    }

    /// Given a set of requested tables, resolve all required dependencies
    /// Returns tables in dependency order (parents before children)
    pub fn resolve_includes(&self, requested: &[&str]) -> Result<Vec<&'static TableSchema>, String> {
        let mut included: HashSet<&str> = HashSet::new();
        let mut queue: VecDeque<&str> = requested.iter().copied().collect();

        // Add all requested tables and their dependencies
        while let Some(table_name) = queue.pop_front() {
            if included.contains(table_name) {
                continue;
            }

            // Validate table exists
            if get_table(table_name).is_none() {
                return Err(format!("Unknown table: {}", table_name));
            }

            included.insert(table_name);

            // Add parent dependencies
            if let Some(table_deps) = self.deps.get(table_name) {
                for dep in table_deps {
                    if !included.contains(dep) {
                        queue.push_back(dep);
                    }
                }
            }

            // Add child tables
            if let Some(table) = get_table(table_name) {
                for child in table.child_tables {
                    if !included.contains(child) {
                        queue.push_back(child);
                    }
                }
            }
        }

        // Return in dependency order
        self.topological_sort(&included)
    }

    /// Given a set of tables to exclude, return remaining tables in order
    pub fn resolve_excludes(&self, excluded: &[&str]) -> Result<Vec<&'static TableSchema>, String> {
        // Validate all excluded tables exist
        for name in excluded {
            if get_table(name).is_none() {
                return Err(format!("Unknown table: {}", name));
            }
        }

        let excluded_set: HashSet<&str> = excluded.iter().copied().collect();
        let mut included: HashSet<&str> = HashSet::new();

        for table in ALL_TABLES {
            if !excluded_set.contains(table.name) {
                // Check if parent is excluded - if so, skip this table too
                let parent_excluded = table.foreign_keys
                    .iter()
                    .any(|fk| excluded_set.contains(fk.references_table));
                
                if !parent_excluded {
                    included.insert(table.name);
                }
            }
        }

        self.topological_sort(&included)
    }

    /// Return all tables in dependency order
    pub fn all_tables_ordered(&self) -> Vec<&'static TableSchema> {
        ALL_TABLES.to_vec()
    }

    /// Topological sort of tables by dependencies
    fn topological_sort(&self, included: &HashSet<&str>) -> Result<Vec<&'static TableSchema>, String> {
        let mut result = Vec::new();
        let mut visited: HashSet<&str> = HashSet::new();
        let mut temp_visited: HashSet<&str> = HashSet::new();

        for table_name in included {
            if !visited.contains(table_name) {
                self.visit(table_name, included, &mut visited, &mut temp_visited, &mut result)?;
            }
        }

        Ok(result)
    }

    fn visit(
        &self,
        name: &str,
        included: &HashSet<&str>,
        visited: &mut HashSet<&str>,
        temp_visited: &mut HashSet<&str>,
        result: &mut Vec<&'static TableSchema>,
    ) -> Result<(), String> {
        if temp_visited.contains(name) {
            return Err(format!("Circular dependency detected at: {}", name));
        }
        if visited.contains(name) {
            return Ok(());
        }

        temp_visited.insert(name);

        if let Some(deps) = self.deps.get(name) {
            for dep in deps {
                if included.contains(dep) {
                    self.visit(dep, included, visited, temp_visited, result)?;
                }
            }
        }

        temp_visited.remove(name);
        visited.insert(name);

        if let Some(table) = get_table(name) {
            result.push(table);
        }

        Ok(())
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_types_includes_parents() {
        let resolver = DependencyResolver::new();
        let tables = resolver.resolve_includes(&["types"]).unwrap();
        let names: Vec<_> = tables.iter().map(|t| t.name).collect();
        
        assert!(names.contains(&"types"));
        assert!(names.contains(&"groups"));
        assert!(names.contains(&"categories"));
        
        // Parents should come before children
        let types_pos = names.iter().position(|&n| n == "types").unwrap();
        let groups_pos = names.iter().position(|&n| n == "groups").unwrap();
        let categories_pos = names.iter().position(|&n| n == "categories").unwrap();
        
        assert!(categories_pos < groups_pos);
        assert!(groups_pos < types_pos);
    }

    #[test]
    fn test_unknown_table_error() {
        let resolver = DependencyResolver::new();
        let result = resolver.resolve_includes(&["nonexistent"]);
        assert!(result.is_err());
    }
}
```

**Step 2: Update schema mod.rs**

Update `src/schema/mod.rs`:
```rust
pub mod dependencies;
pub mod tables;
pub mod types;

pub use dependencies::*;
pub use tables::*;
pub use types::*;
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 4: Commit**

```bash
git add src/schema/
git commit -m "feat: add dependency resolution for table filtering"
```

---

## Phase 3: Download Module

### Task 3.1: Implement HTTP Client

**Files:**
- Create: `src/download/mod.rs`
- Create: `src/download/client.rs`
- Modify: `src/lib.rs`

**Step 1: Create client module**

Create `src/download/client.rs`:
```rust
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::{Read, Write};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};

const LATEST_URL: &str = "https://developers.eveonline.com/static-data/tranquility/latest.jsonl";
const ZIP_URL: &str = "https://developers.eveonline.com/static-data/eve-online-static-data-latest-jsonl.zip";

#[derive(Debug, Deserialize)]
pub struct SdeInfo {
    #[serde(rename = "_key")]
    pub key: String,
    #[serde(rename = "buildNumber")]
    pub build_number: u64,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
}

pub struct SdeClient {
    client: Client,
}

impl SdeClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("eve-sde-to-sqlite")
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { client })
    }

    /// Fetch the latest SDE build info
    pub fn fetch_latest_info(&self) -> Result<SdeInfo> {
        let response = self.client
            .get(LATEST_URL)
            .send()
            .context("Failed to fetch latest SDE info")?;

        let text = response.text().context("Failed to read response")?;
        let info: SdeInfo = serde_json::from_str(&text)
            .context("Failed to parse SDE info")?;
        
        Ok(info)
    }

    /// Download the SDE zip file to the given path
    pub fn download_zip(&self, dest: &Path) -> Result<()> {
        let response = self.client
            .get(ZIP_URL)
            .send()
            .context("Failed to start download")?;

        let total_size = response.content_length().unwrap_or(0);
        
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"));
        pb.set_message("Downloading SDE");

        let mut file = std::fs::File::create(dest)
            .context("Failed to create destination file")?;

        let mut downloaded: u64 = 0;
        let mut buffer = [0u8; 8192];
        let mut reader = response;

        loop {
            let bytes_read = reader.read(&mut buffer)
                .context("Failed to read from response")?;
            
            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read])
                .context("Failed to write to file")?;
            
            downloaded += bytes_read as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");
        Ok(())
    }
}

impl Default for SdeClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
```

**Step 2: Create download mod.rs**

Create `src/download/mod.rs`:
```rust
pub mod client;

pub use client::*;
```

**Step 3: Update lib.rs**

Update `src/lib.rs`:
```rust
pub mod cli;
pub mod download;
pub mod schema;

pub use cli::{Cli, Commands};
```

**Step 4: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 5: Commit**

```bash
git add src/download/
git commit -m "feat: add HTTP client for SDE download"
```

---

### Task 3.2: Implement Cache Management

**Files:**
- Create: `src/download/cache.rs`
- Modify: `src/download/mod.rs`

**Step 1: Create cache module**

Create `src/download/cache.rs`:
```rust
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::fs;

pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new(custom_dir: Option<PathBuf>) -> Result<Self> {
        let cache_dir = match custom_dir {
            Some(dir) => dir,
            None => {
                let proj_dirs = ProjectDirs::from("", "", "eve-sde-to-sqlite")
                    .context("Could not determine cache directory")?;
                proj_dirs.cache_dir().to_path_buf()
            }
        };

        fs::create_dir_all(&cache_dir)
            .context("Failed to create cache directory")?;

        Ok(Self { cache_dir })
    }

    /// Get the cache directory path
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Get path to build-specific directory
    pub fn build_dir(&self, build_number: u64) -> PathBuf {
        self.cache_dir.join(build_number.to_string())
    }

    /// Check if a build is already cached
    pub fn is_cached(&self, build_number: u64) -> bool {
        let build_dir = self.build_dir(build_number);
        build_dir.exists() && build_dir.join("types.jsonl").exists()
    }

    /// Get path to zip file for a build
    pub fn zip_path(&self, build_number: u64) -> PathBuf {
        self.cache_dir.join(format!("{}.zip", build_number))
    }

    /// Clean up old cached builds, keeping only the specified one
    pub fn cleanup_old_builds(&self, keep_build: u64) -> Result<()> {
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(build) = name.parse::<u64>() {
                        if build != keep_build {
                            fs::remove_dir_all(&path).ok();
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
```

**Step 2: Update download mod.rs**

Update `src/download/mod.rs`:
```rust
pub mod cache;
pub mod client;

pub use cache::*;
pub use client::*;
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/download/
git commit -m "feat: add cache management for SDE downloads"
```

---

### Task 3.3: Implement Zip Extraction

**Files:**
- Create: `src/download/extract.rs`
- Modify: `src/download/mod.rs`

**Step 1: Create extract module**

Create `src/download/extract.rs`:
```rust
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::Path;
use zip::ZipArchive;

/// Extract a zip file to the destination directory
pub fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = File::open(zip_path)
        .context("Failed to open zip file")?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)
        .context("Failed to read zip archive")?;

    fs::create_dir_all(dest_dir)
        .context("Failed to create destination directory")?;

    let total_files = archive.len();
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} files")
        .unwrap()
        .progress_chars("=>-"));
    pb.set_message("Extracting");

    for i in 0..total_files {
        let mut file = archive.by_index(i)
            .context("Failed to read file from archive")?;

        // Get the file name, stripping any directory prefix
        let name = file.name();
        let file_name = Path::new(name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(name);

        // Only extract .jsonl files
        if !file_name.ends_with(".jsonl") {
            pb.inc(1);
            continue;
        }

        let dest_path = dest_dir.join(file_name);
        let mut dest_file = File::create(&dest_path)
            .with_context(|| format!("Failed to create file: {:?}", dest_path))?;

        io::copy(&mut file, &mut dest_file)
            .with_context(|| format!("Failed to extract: {}", file_name))?;

        pb.inc(1);
    }

    pb.finish_with_message("Extraction complete");
    Ok(())
}
```

**Step 2: Update download mod.rs**

Update `src/download/mod.rs`:
```rust
pub mod cache;
pub mod client;
pub mod extract;

pub use cache::*;
pub use client::*;
pub use extract::*;
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/download/
git commit -m "feat: add zip extraction with progress bar"
```

---

### Task 3.4: Implement Download Orchestration

**Files:**
- Modify: `src/download/mod.rs`

**Step 1: Add orchestration function**

Update `src/download/mod.rs`:
```rust
pub mod cache;
pub mod client;
pub mod extract;

pub use cache::*;
pub use client::*;
pub use extract::*;

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Download the SDE if not cached, return path to extracted directory
pub fn ensure_sde_downloaded(
    cache_dir: Option<PathBuf>,
    force: bool,
) -> Result<(PathBuf, u64)> {
    let cache = CacheManager::new(cache_dir)?;
    let client = SdeClient::new()?;

    // Get latest build info
    println!("Checking for latest SDE version...");
    let info = client.fetch_latest_info()?;
    println!("Latest build: {} ({})", info.build_number, info.release_date);

    let build_dir = cache.build_dir(info.build_number);

    // Check if already cached
    if !force && cache.is_cached(info.build_number) {
        println!("Using cached SDE from {:?}", build_dir);
        return Ok((build_dir, info.build_number));
    }

    // Download zip
    let zip_path = cache.zip_path(info.build_number);
    println!("Downloading SDE build {}...", info.build_number);
    client.download_zip(&zip_path)?;

    // Extract zip
    println!("Extracting to {:?}...", build_dir);
    extract_zip(&zip_path, &build_dir)?;

    // Clean up zip file
    std::fs::remove_file(&zip_path).ok();

    // Clean up old builds
    cache.cleanup_old_builds(info.build_number).ok();

    Ok((build_dir, info.build_number))
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/download/
git commit -m "feat: add download orchestration with caching"
```

---

## Phase 4: Conversion Module

### Task 4.1: Implement Table Filter

**Files:**
- Create: `src/filter.rs`
- Modify: `src/lib.rs`

**Step 1: Create filter module**

Create `src/filter.rs`:
```rust
use crate::schema::{DependencyResolver, TableSchema};
use anyhow::{bail, Result};

/// Resolves which tables to process based on include/exclude filters
pub fn resolve_tables(
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
) -> Result<Vec<&'static TableSchema>> {
    let resolver = DependencyResolver::new();

    match (include, exclude) {
        (Some(_), Some(_)) => {
            bail!("Cannot use both --include and --exclude at the same time");
        }
        (Some(include_list), None) => {
            let refs: Vec<&str> = include_list.iter().map(|s| s.as_str()).collect();
            println!("Resolving dependencies for: {:?}", refs);
            let tables = resolver.resolve_includes(&refs)?;
            
            println!("Including {} tables:", tables.len());
            for t in &tables {
                println!("  - {}", t.name);
            }
            
            Ok(tables)
        }
        (None, Some(exclude_list)) => {
            let refs: Vec<&str> = exclude_list.iter().map(|s| s.as_str()).collect();
            println!("Excluding tables: {:?}", refs);
            let tables = resolver.resolve_excludes(&refs)?;
            
            println!("Including {} tables (after exclusions):", tables.len());
            
            Ok(tables)
        }
        (None, None) => {
            let tables = resolver.all_tables_ordered();
            println!("Including all {} tables", tables.len());
            Ok(tables)
        }
    }
}
```

**Step 2: Update lib.rs**

Update `src/lib.rs`:
```rust
pub mod cli;
pub mod download;
pub mod filter;
pub mod schema;

pub use cli::{Cli, Commands};
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/filter.rs src/lib.rs
git commit -m "feat: add table filter resolution"
```

---

### Task 4.2: Implement SQL Schema Generator

**Files:**
- Create: `src/writer/mod.rs`
- Create: `src/writer/schema_gen.rs`
- Modify: `src/lib.rs`

**Step 1: Create schema generator**

Create `src/writer/schema_gen.rs`:
```rust
use crate::schema::{ColumnType, TableSchema, LANGUAGES};

/// Generate CREATE TABLE SQL for a table schema
pub fn generate_create_table(schema: &TableSchema) -> String {
    let mut sql = format!("CREATE TABLE {} (\n", schema.name);
    let mut columns = Vec::new();

    for col in schema.columns {
        match col.col_type {
            ColumnType::Localized => {
                // Expand localized columns to per-language columns
                for lang in LANGUAGES {
                    let col_name = format!("{}_{}", col.name, lang);
                    columns.push(format!("    {} TEXT", col_name));
                }
            }
            _ => {
                let sql_type = match col.col_type {
                    ColumnType::Integer => "INTEGER",
                    ColumnType::Real => "REAL",
                    ColumnType::Text => "TEXT",
                    ColumnType::Boolean => "INTEGER",
                    ColumnType::Json => "TEXT",
                    ColumnType::Localized => unreachable!(),
                };

                let null_constraint = if !col.nullable { " NOT NULL" } else { "" };
                let pk = if col.name == "id" { " PRIMARY KEY" } else { "" };
                
                columns.push(format!("    {} {}{}{}", col.name, sql_type, pk, null_constraint));
            }
        }
    }

    // Add foreign key constraints
    for fk in schema.foreign_keys {
        columns.push(format!(
            "    FOREIGN KEY ({}) REFERENCES {}({})",
            fk.column, fk.references_table, fk.references_column
        ));
    }

    sql.push_str(&columns.join(",\n"));
    sql.push_str("\n)");

    sql
}

/// Generate CREATE INDEX statements for foreign key columns
pub fn generate_indexes(schema: &TableSchema) -> Vec<String> {
    schema.foreign_keys
        .iter()
        .map(|fk| {
            format!(
                "CREATE INDEX idx_{}_{} ON {}({})",
                schema.name, fk.column, schema.name, fk.column
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::tables::TYPES;

    #[test]
    fn test_generate_create_table() {
        let sql = generate_create_table(&TYPES);
        assert!(sql.contains("CREATE TABLE types"));
        assert!(sql.contains("id INTEGER PRIMARY KEY"));
        assert!(sql.contains("name_en TEXT"));
        assert!(sql.contains("name_de TEXT"));
        assert!(sql.contains("FOREIGN KEY (group_id) REFERENCES groups(id)"));
    }

    #[test]
    fn test_generate_indexes() {
        let indexes = generate_indexes(&TYPES);
        assert!(indexes.iter().any(|i| i.contains("idx_types_group_id")));
    }
}
```

**Step 2: Create writer mod.rs**

Create `src/writer/mod.rs`:
```rust
pub mod schema_gen;

pub use schema_gen::*;
```

**Step 3: Update lib.rs**

Update `src/lib.rs`:
```rust
pub mod cli;
pub mod download;
pub mod filter;
pub mod schema;
pub mod writer;

pub use cli::{Cli, Commands};
```

**Step 4: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/writer/
git commit -m "feat: add SQL schema generator"
```

---

### Task 4.3: Implement JSONL Parser

**Files:**
- Create: `src/parser/mod.rs`
- Create: `src/parser/record.rs`
- Modify: `src/lib.rs`

**Step 1: Create record parser**

Create `src/parser/record.rs`:
```rust
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;

use crate::schema::{ColumnType, TableSchema, LANGUAGES};

/// A parsed row ready for insertion
pub struct ParsedRow {
    pub values: HashMap<String, SqlValue>,
}

#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
}

impl SqlValue {
    pub fn bind_to(&self, idx: usize, stmt: &mut rusqlite::Statement) -> rusqlite::Result<()> {
        use rusqlite::types::ToSqlOutput;
        match self {
            SqlValue::Null => stmt.raw_bind_parameter(idx, rusqlite::types::Null)?,
            SqlValue::Integer(i) => stmt.raw_bind_parameter(idx, i)?,
            SqlValue::Real(f) => stmt.raw_bind_parameter(idx, f)?,
            SqlValue::Text(s) => stmt.raw_bind_parameter(idx, s.as_str())?,
        }
        Ok(())
    }
}

/// Parse a JSON line into a row for the given table schema
pub fn parse_record(line: &str, schema: &TableSchema) -> Result<ParsedRow> {
    let json: Value = serde_json::from_str(line)
        .context("Failed to parse JSON")?;

    let mut values = HashMap::new();

    for col in schema.columns {
        match col.col_type {
            ColumnType::Localized => {
                // Handle localized fields
                let json_key = to_camel_case(col.name);
                if let Some(obj) = json.get(&json_key).and_then(|v| v.as_object()) {
                    for lang in LANGUAGES {
                        let col_name = format!("{}_{}", col.name, lang);
                        let value = obj.get(*lang)
                            .and_then(|v| v.as_str())
                            .map(|s| SqlValue::Text(s.to_string()))
                            .unwrap_or(SqlValue::Null);
                        values.insert(col_name, value);
                    }
                } else {
                    for lang in LANGUAGES {
                        let col_name = format!("{}_{}", col.name, lang);
                        values.insert(col_name, SqlValue::Null);
                    }
                }
            }
            _ => {
                let json_key = if col.name == "id" {
                    "_key".to_string()
                } else {
                    to_camel_case(col.name)
                };

                let value = extract_value(&json, &json_key, &col.col_type);
                values.insert(col.name.to_string(), value);
            }
        }
    }

    Ok(ParsedRow { values })
}

fn extract_value(json: &Value, key: &str, col_type: &ColumnType) -> SqlValue {
    let val = json.get(key);
    
    match val {
        None | Some(Value::Null) => SqlValue::Null,
        Some(v) => match col_type {
            ColumnType::Integer => {
                v.as_i64().map(SqlValue::Integer).unwrap_or(SqlValue::Null)
            }
            ColumnType::Real => {
                v.as_f64().map(SqlValue::Real).unwrap_or(SqlValue::Null)
            }
            ColumnType::Text => {
                v.as_str().map(|s| SqlValue::Text(s.to_string())).unwrap_or(SqlValue::Null)
            }
            ColumnType::Boolean => {
                v.as_bool().map(|b| SqlValue::Integer(if b { 1 } else { 0 })).unwrap_or(SqlValue::Null)
            }
            ColumnType::Json => {
                SqlValue::Text(v.to_string())
            }
            ColumnType::Localized => SqlValue::Null, // Handled separately
        }
    }
}

/// Convert snake_case to camelCase
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("group_id"), "groupId");
        assert_eq!(to_camel_case("solar_system_id"), "solarSystemId");
        assert_eq!(to_camel_case("name"), "name");
    }
}
```

**Step 2: Create parser mod.rs**

Create `src/parser/mod.rs`:
```rust
pub mod record;

pub use record::*;
```

**Step 3: Update lib.rs**

Update `src/lib.rs`:
```rust
pub mod cli;
pub mod download;
pub mod filter;
pub mod parser;
pub mod schema;
pub mod writer;

pub use cli::{Cli, Commands};
```

**Step 4: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/parser/
git commit -m "feat: add JSONL record parser"
```

---

### Task 4.4: Implement SQLite Writer

**Files:**
- Create: `src/writer/sqlite.rs`
- Modify: `src/writer/mod.rs`

**Step 1: Create SQLite writer**

Create `src/writer/sqlite.rs`:
```rust
use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::{parse_record, ParsedRow};
use crate::schema::{ColumnType, TableSchema, LANGUAGES};
use super::schema_gen::{generate_create_table, generate_indexes};

const BATCH_SIZE: usize = 1000;

pub struct SqliteWriter {
    conn: Connection,
}

impl SqliteWriter {
    pub fn new(db_path: &Path) -> Result<Self> {
        // Remove existing database if present
        if db_path.exists() {
            std::fs::remove_file(db_path)
                .context("Failed to remove existing database")?;
        }

        let conn = Connection::open(db_path)
            .context("Failed to create database")?;

        // Enable foreign keys and optimize for bulk insert
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -64000;"
        )?;

        Ok(Self { conn })
    }

    /// Create all tables for the given schemas
    pub fn create_tables(&self, schemas: &[&TableSchema]) -> Result<()> {
        println!("Creating {} tables...", schemas.len());
        
        for schema in schemas {
            let sql = generate_create_table(schema);
            self.conn.execute(&sql, [])
                .with_context(|| format!("Failed to create table: {}", schema.name))?;

            for index_sql in generate_indexes(schema) {
                self.conn.execute(&index_sql, [])
                    .with_context(|| format!("Failed to create index for: {}", schema.name))?;
            }
        }

        Ok(())
    }

    /// Import data from JSONL file for a single table
    pub fn import_table(
        &mut self,
        schema: &TableSchema,
        input_dir: &Path,
        progress: &ProgressBar,
    ) -> Result<u64> {
        let file_path = input_dir.join(schema.source_file);
        
        if !file_path.exists() {
            progress.finish_with_message(format!("{}: skipped (file not found)", schema.name));
            return Ok(0);
        }

        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open: {:?}", file_path))?;
        let reader = BufReader::new(file);

        // Build insert statement
        let columns = self.get_column_names(schema);
        let placeholders: Vec<&str> = columns.iter().map(|_| "?").collect();
        let insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            schema.name,
            columns.join(", "),
            placeholders.join(", ")
        );

        let tx = self.conn.transaction()?;
        let mut count: u64 = 0;
        let mut batch: Vec<ParsedRow> = Vec::with_capacity(BATCH_SIZE);

        for line in reader.lines() {
            let line = line.context("Failed to read line")?;
            if line.trim().is_empty() {
                continue;
            }

            let row = parse_record(&line, schema)
                .with_context(|| format!("Failed to parse record in {}", schema.source_file))?;
            
            batch.push(row);

            if batch.len() >= BATCH_SIZE {
                self.insert_batch(&tx, &insert_sql, &columns, &batch)?;
                count += batch.len() as u64;
                progress.set_position(count);
                batch.clear();
            }
        }

        // Insert remaining batch
        if !batch.is_empty() {
            self.insert_batch(&tx, &insert_sql, &columns, &batch)?;
            count += batch.len() as u64;
        }

        tx.commit()?;
        progress.set_position(count);
        progress.finish_with_message(format!("{}: {} records", schema.name, count));

        Ok(count)
    }

    fn get_column_names(&self, schema: &TableSchema) -> Vec<String> {
        let mut columns = Vec::new();
        
        for col in schema.columns {
            match col.col_type {
                ColumnType::Localized => {
                    for lang in LANGUAGES {
                        columns.push(format!("{}_{}", col.name, lang));
                    }
                }
                _ => {
                    columns.push(col.name.to_string());
                }
            }
        }

        columns
    }

    fn insert_batch(
        &self,
        tx: &rusqlite::Transaction,
        sql: &str,
        columns: &[String],
        batch: &[ParsedRow],
    ) -> Result<()> {
        let mut stmt = tx.prepare_cached(sql)?;

        for row in batch {
            for (idx, col_name) in columns.iter().enumerate() {
                let value = row.values.get(col_name)
                    .cloned()
                    .unwrap_or(crate::parser::SqlValue::Null);
                value.bind_to(idx + 1, &mut stmt)?;
            }
            stmt.raw_execute()?;
        }

        Ok(())
    }

    /// Finalize the database (VACUUM, etc.)
    pub fn finalize(self) -> Result<()> {
        println!("Finalizing database...");
        self.conn.execute("PRAGMA optimize;", [])?;
        Ok(())
    }
}

/// Convert JSONL files to SQLite with progress bars
pub fn convert_to_sqlite(
    input_dir: &Path,
    output_db: &Path,
    tables: Vec<&TableSchema>,
) -> Result<u64> {
    let mut writer = SqliteWriter::new(output_db)?;
    
    // Create all tables first
    writer.create_tables(&tables)?;

    // Set up progress bars
    let multi = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("{msg:30} [{bar:40.cyan/blue}] {pos}/{len}")
        .unwrap()
        .progress_chars("=>-");

    let mut total_records: u64 = 0;

    for schema in &tables {
        // Count lines for progress bar
        let file_path = input_dir.join(schema.source_file);
        let line_count = if file_path.exists() {
            BufReader::new(File::open(&file_path)?).lines().count() as u64
        } else {
            0
        };

        let pb = multi.add(ProgressBar::new(line_count));
        pb.set_style(style.clone());
        pb.set_message(schema.name.to_string());

        let count = writer.import_table(schema, input_dir, &pb)?;
        total_records += count;
    }

    writer.finalize()?;

    Ok(total_records)
}
```

**Step 2: Update writer mod.rs**

Update `src/writer/mod.rs`:
```rust
pub mod schema_gen;
pub mod sqlite;

pub use schema_gen::*;
pub use sqlite::*;
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/writer/
git commit -m "feat: add SQLite writer with batch inserts"
```

---

## Phase 5: Wire Everything Together

### Task 5.1: Implement Main Command Handlers

**Files:**
- Modify: `src/main.rs`

**Step 1: Update main.rs with full implementation**

Update `src/main.rs`:
```rust
use anyhow::Result;
use eve_sde_to_sqlite::{
    cli::{Cli, Commands},
    download::ensure_sde_downloaded,
    filter::resolve_tables,
    schema::table_names,
    writer::convert_to_sqlite,
};
use std::time::Instant;

fn main() -> Result<()> {
    let cli = Cli::parse_args();

    match cli.command {
        Commands::Sync {
            output_db,
            include,
            exclude,
            force,
            cache_dir,
        } => {
            let start = Instant::now();

            // Download SDE if needed
            let (input_dir, build_number) = ensure_sde_downloaded(cache_dir, force)?;

            // Resolve table filters
            let tables = resolve_tables(include, exclude)?;

            // Convert to SQLite
            println!("\nConverting to SQLite...");
            let record_count = convert_to_sqlite(&input_dir, &output_db, tables)?;

            let elapsed = start.elapsed();
            println!(
                "\nCreated {:?} ({} records) from SDE build {} in {:.1}s",
                output_db,
                record_count,
                build_number,
                elapsed.as_secs_f64()
            );
        }

        Commands::Download { output, force } => {
            let (path, build_number) = ensure_sde_downloaded(output, force)?;
            println!("SDE build {} downloaded to {:?}", build_number, path);
        }

        Commands::Convert {
            input_dir,
            output_db,
            include,
            exclude,
        } => {
            let start = Instant::now();

            // Resolve table filters
            let tables = resolve_tables(include, exclude)?;

            // Convert to SQLite
            println!("\nConverting to SQLite...");
            let record_count = convert_to_sqlite(&input_dir, &output_db, tables)?;

            let elapsed = start.elapsed();
            println!(
                "\nCreated {:?} ({} records) in {:.1}s",
                output_db,
                record_count,
                elapsed.as_secs_f64()
            );
        }

        Commands::ListTables => {
            println!("Available tables:\n");
            for name in table_names() {
                println!("  {}", name);
            }
        }
    }

    Ok(())
}
```

**Step 2: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire up CLI commands to core functionality"
```

---

### Task 5.2: Test End-to-End with Local Data

**Step 1: Test list-tables command**

Run: `cargo run -- list-tables`
Expected: Lists all table names

**Step 2: Test convert with local data**

Run:
```bash
cargo run -- convert /Users/antonioiaccarino/Downloads/eve-online-static-data-3168731-jsonl/ test.db --include categories,groups
```
Expected: Creates test.db with categories and groups tables

**Step 3: Verify database**

Run:
```bash
sqlite3 test.db ".tables"
sqlite3 test.db "SELECT COUNT(*) FROM categories;"
sqlite3 test.db "SELECT COUNT(*) FROM groups;"
```
Expected: Shows tables and record counts

**Step 4: Clean up test file**

Run: `rm test.db`

**Step 5: Commit**

```bash
git commit --allow-empty -m "test: verify end-to-end conversion works"
```

---

### Task 5.3: Test Full Sync Command

**Step 1: Test sync command**

Run:
```bash
cargo run -- sync eve.db --include types,groups,categories
```
Expected: Downloads SDE (if not cached), converts selected tables

**Step 2: Verify database contents**

Run:
```bash
sqlite3 eve.db ".tables"
sqlite3 eve.db "SELECT name_en FROM types LIMIT 5;"
```
Expected: Shows tables and sample data

**Step 3: Test exclude flag**

Run:
```bash
cargo run -- sync eve-slim.db --exclude map_moons,map_planets,map_asteroid_belts
```
Expected: Creates smaller database without large map tables

**Step 4: Compare database sizes**

Run: `ls -lh eve.db eve-slim.db`
Expected: eve-slim.db is smaller

**Step 5: Clean up**

Run: `rm eve.db eve-slim.db`

**Step 6: Commit**

```bash
git commit --allow-empty -m "test: verify sync command with filtering"
```

---

## Phase 6: Polish and Documentation

### Task 6.1: Add README

**Files:**
- Create: `README.md`

**Step 1: Create README**

Create `README.md`:
```markdown
# eve-sde-to-sqlite

Convert EVE Online Static Data Export (SDE) to SQLite database.

## Installation

```bash
cargo install --path .
```

## Usage

### Quick Start

Download and convert the latest SDE:

```bash
eve-sde-to-sqlite sync eve.db
```

### Commands

#### sync - Download and convert (recommended)

```bash
# Convert all tables
eve-sde-to-sqlite sync eve.db

# Only include specific tables (dependencies auto-included)
eve-sde-to-sqlite sync eve.db --include types,blueprints

# Exclude large tables for smaller database
eve-sde-to-sqlite sync eve.db --exclude map_moons,map_planets,map_asteroid_belts

# Force re-download even if cached
eve-sde-to-sqlite sync eve.db --force
```

#### download - Download only

```bash
eve-sde-to-sqlite download
eve-sde-to-sqlite download --output ./my-cache/
```

#### convert - Convert local files

```bash
eve-sde-to-sqlite convert ./sde-data/ eve.db
eve-sde-to-sqlite convert ./sde-data/ eve.db --include types,groups
```

#### list-tables - Show available tables

```bash
eve-sde-to-sqlite list-tables
```

## Table Filtering

When using `--include`, foreign key dependencies are automatically included:

```bash
eve-sde-to-sqlite sync eve.db --include types
# Also includes: groups, categories, icons, graphics, etc.
```

When using `--exclude`, child tables of excluded parents are also excluded.

## Database Schema

Tables are normalized from the JSONL format:
- Localized text fields become separate columns: `name_en`, `name_de`, etc.
- Nested arrays become junction tables: `blueprint_materials`, `type_dogma_attributes`, etc.
- Foreign key constraints are enforced

## License

MIT
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add README with usage instructions"
```

---

### Task 6.2: Final Build and Test

**Step 1: Build release version**

Run: `cargo build --release`
Expected: Builds successfully

**Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Run all tests**

Run: `cargo test`
Expected: All tests pass

**Step 4: Test release binary**

Run:
```bash
./target/release/eve-sde-to-sqlite --help
./target/release/eve-sde-to-sqlite list-tables
```
Expected: Shows help and table list

**Step 5: Final commit**

```bash
git add -A
git commit -m "chore: final polish and release build"
```

---

## Summary

This plan creates a complete Rust CLI tool with:

1. **CLI** - clap-based with sync/download/convert/list-tables subcommands
2. **Download** - HTTP client with progress bar, caching, zip extraction
3. **Schema** - 40+ table definitions with FK relationships
4. **Filtering** - --include/--exclude with automatic dependency resolution
5. **Parser** - JSONL to structured records with localization handling
6. **Writer** - SQLite with batch inserts, FK constraints, indexes

Total: ~25 tasks across 6 phases
