//! Table schema definitions for all EVE Online SDE tables

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
    indexes: &[Index::on(&["name_en"]), Index::on(&["published"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["name"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["name"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[],
    child_tables: &[],
    array_source: None,
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
    indexes: &[],
    child_tables: &[],
    array_source: None,
};

pub static AGENT_TYPES: TableSchema = TableSchema {
    name: "agent_types",
    source_file: "agentTypes.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
    ],
    foreign_keys: &[],
    indexes: &[],
    child_tables: &[],
    array_source: None,
};

pub static STATION_SERVICES: TableSchema = TableSchema {
    name: "station_services",
    source_file: "stationServices.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("service_name", ColumnType::Text),
    ],
    foreign_keys: &[],
    indexes: &[Index::on(&["service_name"])],
    child_tables: &[],
    array_source: None,
};

pub static CORPORATION_ACTIVITIES: TableSchema = TableSchema {
    name: "corporation_activities",
    source_file: "corporationActivities.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
    ],
    foreign_keys: &[],
    indexes: &[Index::on(&["name_en"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["name_en"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["name_en"])],
    child_tables: &[],
    array_source: None,
};

pub static TRANSLATION_LANGUAGES: TableSchema = TableSchema {
    name: "translation_languages",
    source_file: "translationLanguages.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
    ],
    foreign_keys: &[],
    indexes: &[],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["display_name_en"]),
        Index::on(&["material_set_id"]),
    ],
    child_tables: &[],
    array_source: None,
};

pub static LANDMARKS: TableSchema = TableSchema {
    name: "landmarks",
    source_file: "landmarks.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("description", ColumnType::Localized),
        Column::new("importance", ColumnType::Integer),
        Column::new("location_id", ColumnType::Integer),
        Column::new("position_x", ColumnType::Real),
        Column::new("position_y", ColumnType::Real),
        Column::new("position_z", ColumnType::Real),
    ],
    foreign_keys: &[],
    indexes: &[Index::on(&["name_en"]), Index::on(&["location_id"])],
    child_tables: &[],
    array_source: None,
};

pub static NPC_CORPORATION_DIVISIONS: TableSchema = TableSchema {
    name: "npc_corporation_divisions",
    source_file: "npcCorporationDivisions.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("display_name", ColumnType::Text),
        Column::new("internal_name", ColumnType::Text),
        Column::new("leader_type_name", ColumnType::Localized),
    ],
    foreign_keys: &[],
    indexes: &[Index::on(&["name_en"])],
    child_tables: &[],
    array_source: None,
};

pub static PLANET_RESOURCES: TableSchema = TableSchema {
    name: "planet_resources",
    source_file: "planetResources.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("power", ColumnType::Integer),
    ],
    foreign_keys: &[],
    indexes: &[],
    child_tables: &[],
    array_source: None,
};

pub static CLONE_GRADES: TableSchema = TableSchema {
    name: "clone_grades",
    source_file: "cloneGrades.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Text),
    ],
    foreign_keys: &[],
    indexes: &[],
    child_tables: &["clone_grade_skills"],
    array_source: None,
};

pub static PLANET_SCHEMATICS: TableSchema = TableSchema {
    name: "planet_schematics",
    source_file: "planetSchematics.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("cycle_time", ColumnType::Integer),
    ],
    foreign_keys: &[],
    indexes: &[Index::on(&["name_en"])],
    child_tables: &["planet_schematic_pins", "planet_schematic_types"],
    array_source: None,
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
    foreign_keys: &[ForeignKey::new("icon_id", "icons")],
    indexes: &[Index::on(&["name_en"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["category_id"]),
        Index::on(&["name_en"]),
        Index::on(&["published"]),
    ],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["unit_id"]),
        Index::on(&["name"]),
        Index::on(&["published"]),
    ],
    child_tables: &[],
    array_source: None,
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
    foreign_keys: &[ForeignKey::new("icon_id", "icons")],
    indexes: &[
        Index::on(&["icon_id"]),
        Index::on(&["name"]),
        Index::on(&["published"]),
    ],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["faction_id"]), Index::on(&["name_en"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["parent_group_id"]),
        Index::on(&["icon_id"]),
        Index::on(&["name_en"]),
    ],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["activity_id"]),
        Index::on(&["operation_name_en"]),
    ],
    child_tables: &[],
    array_source: None,
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
    foreign_keys: &[ForeignKey::new("skin_material_id", "skin_materials")],
    indexes: &[Index::on(&["skin_material_id"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["race_id"]),
        Index::on(&["corporation_id"]),
        Index::on(&["name_en"]),
    ],
    child_tables: &[],
    array_source: None,
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
    foreign_keys: &[ForeignKey::new("icon_id", "icons")],
    indexes: &[
        Index::on(&["icon_id"]),
        Index::on(&["solar_system_id"]),
        Index::on(&["corporation_id"]),
        Index::on(&["militia_corporation_id"]),
        Index::on(&["name_en"]),
    ],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["name_en"]), Index::on(&["ticker_name"])],
    child_tables: &[],
    array_source: None,
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
    foreign_keys: &[ForeignKey::new("region_id", "map_regions")],
    indexes: &[
        Index::on(&["region_id"]),
        Index::on(&["faction_id"]),
        Index::on(&["name_en"]),
    ],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["group_id"]),
        Index::on(&["graphic_id"]),
        Index::on(&["icon_id"]),
        Index::on(&["market_group_id"]),
        Index::on(&["meta_group_id"]),
        Index::on(&["race_id"]),
        Index::on(&["name_en"]),
        Index::on(&["published"]),
    ],
    child_tables: &[],
    array_source: None,
};

pub static COMPRESSIBLE_TYPES: TableSchema = TableSchema {
    name: "compressible_types",
    source_file: "compressibleTypes.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("compressed_type_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("id", "types"),
        ForeignKey::new("compressed_type_id", "types"),
    ],
    indexes: &[Index::on(&["compressed_type_id"])],
    child_tables: &[],
    array_source: None,
};

pub static NPC_CHARACTERS: TableSchema = TableSchema {
    name: "npc_characters",
    source_file: "npcCharacters.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("name", ColumnType::Localized),
        Column::new("bloodline_id", ColumnType::Integer),
        Column::new("corporation_id", ColumnType::Integer),
        Column::new("race_id", ColumnType::Integer),
        Column::new("location_id", ColumnType::Integer),
        Column::new("ceo", ColumnType::Boolean),
        Column::new("gender", ColumnType::Boolean),
        Column::new("start_date", ColumnType::Text),
        Column::new("unique_name", ColumnType::Boolean),
    ],
    foreign_keys: &[
        ForeignKey::new("bloodline_id", "bloodlines"),
        ForeignKey::new("corporation_id", "npc_corporations"),
        ForeignKey::new("race_id", "races"),
    ],
    indexes: &[
        Index::on(&["bloodline_id"]),
        Index::on(&["corporation_id"]),
        Index::on(&["race_id"]),
        Index::on(&["name_en"]),
    ],
    child_tables: &[],
    array_source: None,
};

/// Sovereignty upgrades - simple table with flattened nested "fuel" object
pub static SOVEREIGNTY_UPGRADES: TableSchema = TableSchema {
    name: "sovereignty_upgrades",
    source_file: "sovereigntyUpgrades.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("fuel_type_id", ColumnType::Integer),
        Column::new("fuel_hourly_upkeep", ColumnType::Integer),
        Column::new("fuel_startup_cost", ColumnType::Integer),
        Column::new("mutually_exclusive_group", ColumnType::Text),
        Column::new("power_allocation", ColumnType::Integer),
        Column::new("workforce_allocation", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("id", "types"),
        ForeignKey::new("fuel_type_id", "types"),
    ],
    indexes: &[Index::on(&["fuel_type_id"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["bloodline_id"]),
        Index::on(&["icon_id"]),
        Index::on(&["name_en"]),
    ],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["constellation_id"]),
        Index::on(&["star_id"]),
        Index::on(&["name_en"]),
        Index::on(&["security_status"]),
    ],
    child_tables: &[],
    array_source: None,
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
    foreign_keys: &[ForeignKey::new("blueprint_type_id", "types")],
    indexes: &[Index::on(&["blueprint_type_id"])],
    child_tables: &[
        "blueprint_materials",
        "blueprint_products",
        "blueprint_skills",
    ],
    array_source: None,
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
    indexes: &[Index::on(&["skin_id"])],
    child_tables: &[],
    array_source: None,
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
    foreign_keys: &[ForeignKey::new("group_id", "groups")],
    indexes: &[Index::on(&["name_en"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["solar_system_id"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["solar_system_id"]), Index::on(&["type_id"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["planet_id"]), Index::on(&["type_id"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["planet_id"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["solar_system_id"]),
        Index::on(&["destination_stargate_id"]),
        Index::on(&["type_id"]),
    ],
    child_tables: &[],
    array_source: None,
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
    indexes: &[
        Index::on(&["solar_system_id"]),
        Index::on(&["operation_id"]),
        Index::on(&["type_id"]),
        Index::on(&["owner_id"]),
    ],
    child_tables: &[],
    array_source: None,
};

pub static AGENTS_IN_SPACE: TableSchema = TableSchema {
    name: "agents_in_space",
    source_file: "agentsInSpace.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("dungeon_id", ColumnType::Integer),
        Column::new("solar_system_id", ColumnType::Integer),
        Column::new("spawn_point_id", ColumnType::Integer),
        Column::new("type_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("solar_system_id", "map_solar_systems"),
        ForeignKey::new("type_id", "types"),
    ],
    indexes: &[Index::on(&["solar_system_id"]), Index::on(&["type_id"])],
    child_tables: &[],
    array_source: None,
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
    indexes: &[Index::on(&["type_id"]), Index::on(&["attribute_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "dogmaAttributes",
        parent_id_column: "type_id",
    }),
};

pub static TYPE_DOGMA_EFFECTS: TableSchema = TableSchema {
    name: "type_dogma_effects",
    source_file: "typeDogma.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("effect_id", ColumnType::Integer),
        Column::new("is_default", ColumnType::Boolean),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("effect_id", "dogma_effects"),
    ],
    indexes: &[Index::on(&["type_id"]), Index::on(&["effect_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "dogmaEffects",
        parent_id_column: "type_id",
    }),
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
    indexes: &[Index::on(&["type_id"]), Index::on(&["material_type_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "materials",
        parent_id_column: "type_id",
    }),
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
    indexes: &[
        Index::on(&["blueprint_id"]),
        Index::on(&["type_id"]),
        Index::on(&["activity"]),
    ],
    child_tables: &[],
    array_source: Some(ArraySource::BlueprintActivity {
        activity_column: "activity",
        array_field: "materials",
    }),
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
    indexes: &[
        Index::on(&["blueprint_id"]),
        Index::on(&["type_id"]),
        Index::on(&["activity"]),
    ],
    child_tables: &[],
    array_source: Some(ArraySource::BlueprintActivity {
        activity_column: "activity",
        array_field: "products",
    }),
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
    indexes: &[
        Index::on(&["blueprint_id"]),
        Index::on(&["type_id"]),
        Index::on(&["activity"]),
    ],
    child_tables: &[],
    array_source: Some(ArraySource::BlueprintActivity {
        activity_column: "activity",
        array_field: "skills",
    }),
};

pub static CLONE_GRADE_SKILLS: TableSchema = TableSchema {
    name: "clone_grade_skills",
    source_file: "cloneGrades.jsonl",
    columns: &[
        Column::required("clone_grade_id", ColumnType::Integer),
        Column::required("type_id", ColumnType::Integer),
        Column::required("level", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("clone_grade_id", "clone_grades"),
        ForeignKey::new("type_id", "types"),
    ],
    indexes: &[Index::on(&["clone_grade_id"]), Index::on(&["type_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "skills",
        parent_id_column: "clone_grade_id",
    }),
};

pub static CONTRABAND_TYPE_FACTIONS: TableSchema = TableSchema {
    name: "contraband_type_factions",
    source_file: "contrabandTypes.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("faction_id", ColumnType::Integer).json("_key"),
        Column::new("attack_min_sec", ColumnType::Real),
        Column::new("confiscate_min_sec", ColumnType::Real),
        Column::new("fine_by_value", ColumnType::Real),
        Column::new("standing_loss", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("faction_id", "factions"),
    ],
    indexes: &[Index::on(&["type_id"]), Index::on(&["faction_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "factions",
        parent_id_column: "type_id",
    }),
};

pub static CONTROL_TOWER_RESOURCES: TableSchema = TableSchema {
    name: "control_tower_resources",
    source_file: "controlTowerResources.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("resource_type_id", ColumnType::Integer),
        Column::new("purpose", ColumnType::Integer),
        Column::new("quantity", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("resource_type_id", "types"),
    ],
    indexes: &[Index::on(&["type_id"]), Index::on(&["resource_type_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "resources",
        parent_id_column: "type_id",
    }),
};

pub static DYNAMIC_ITEM_ATTRIBUTES: TableSchema = TableSchema {
    name: "dynamic_item_attributes",
    source_file: "dynamicItemAttributes.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("attribute_id", ColumnType::Integer).json("_key"),
        Column::new("min", ColumnType::Real),
        Column::new("max", ColumnType::Real),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("attribute_id", "dogma_attributes"),
    ],
    indexes: &[Index::on(&["type_id"]), Index::on(&["attribute_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "attributeIDs",
        parent_id_column: "type_id",
    }),
};

pub static PLANET_SCHEMATIC_PINS: TableSchema = TableSchema {
    name: "planet_schematic_pins",
    source_file: "planetSchematics.jsonl",
    columns: &[
        Column::required("schematic_id", ColumnType::Integer),
        Column::required("pin_type_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("schematic_id", "planet_schematics"),
        ForeignKey::new("pin_type_id", "types"),
    ],
    indexes: &[Index::on(&["schematic_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::SimpleIntArray {
        array_field: "pins",
        parent_id_column: "schematic_id",
        value_column: "pin_type_id",
    }),
};

pub static PLANET_SCHEMATIC_TYPES: TableSchema = TableSchema {
    name: "planet_schematic_types",
    source_file: "planetSchematics.jsonl",
    columns: &[
        Column::required("schematic_id", ColumnType::Integer),
        Column::required("type_id", ColumnType::Integer).json("_key"),
        Column::new("is_input", ColumnType::Boolean),
        Column::new("quantity", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("schematic_id", "planet_schematics"),
        ForeignKey::new("type_id", "types"),
    ],
    indexes: &[Index::on(&["schematic_id"]), Index::on(&["type_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "types",
        parent_id_column: "schematic_id",
    }),
};

// =============================================================================
// Complex Nested Tables (require special parser handling)
// =============================================================================

/// Role bonuses for types (from typeBonus.jsonl roleBonuses array)
/// Format: {"_key": 582, "roleBonuses": [{"bonus": 300.0, "bonusText": {...}, "importance": 1, "unitID": 105}], ...}
pub static TYPE_ROLE_BONUSES: TableSchema = TableSchema {
    name: "type_role_bonuses",
    source_file: "typeBonus.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::new("bonus", ColumnType::Real),
        Column::new("bonus_text", ColumnType::Localized),
        Column::new("importance", ColumnType::Integer),
        Column::new("unit_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("unit_id", "dogma_units"),
    ],
    indexes: &[Index::on(&["type_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "roleBonuses",
        parent_id_column: "type_id",
    }),
};

/// Trait bonuses for types based on skills (from typeBonus.jsonl types array)
/// Format: {"_key": 582, "types": [{"_key": 3330, "_value": [{"bonus": 10.0, "bonusText": {...}, ...}]}]}
/// NOTE: Requires special parser handling - the "types" array contains objects with _key (skill type)
/// and _value (array of bonuses). Each bonus row needs type_id (parent _key), skill_type_id (nested _key),
/// and the bonus fields.
pub static TYPE_TRAIT_BONUSES: TableSchema = TableSchema {
    name: "type_trait_bonuses",
    source_file: "typeBonus.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("skill_type_id", ColumnType::Integer),
        Column::new("bonus", ColumnType::Real),
        Column::new("bonus_text", ColumnType::Localized),
        Column::new("importance", ColumnType::Integer),
        Column::new("is_positive", ColumnType::Boolean),
        Column::new("unit_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("skill_type_id", "types"),
        ForeignKey::new("unit_id", "dogma_units"),
    ],
    indexes: &[Index::on(&["type_id"]), Index::on(&["skill_type_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::NestedKeyValue {
        array_field: "types",
        parent_id_column: "type_id",
        nested_key_column: "skill_type_id",
    }),
};

/// Mastery requirements for types (from masteries.jsonl)
/// Format: {"_key": 582, "_value": [{"_key": 0, "_value": [96, 139, 85, 87, 94]}, {"_key": 1, "_value": [...]}]}
/// NOTE: Requires special parser handling - double-nested structure where:
/// - Parent _key is the type_id
/// - First level _key (0-4) is the mastery_level
/// - Inner _value array contains certificate_ids
pub static TYPE_MASTERIES: TableSchema = TableSchema {
    name: "type_masteries",
    source_file: "masteries.jsonl",
    columns: &[
        Column::required("type_id", ColumnType::Integer),
        Column::required("mastery_level", ColumnType::Integer),
        Column::required("certificate_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("type_id", "types"),
        ForeignKey::new("certificate_id", "certificates"),
    ],
    indexes: &[Index::on(&["type_id"])],
    child_tables: &[],
    array_source: Some(ArraySource::DoubleNested {
        parent_id_column: "type_id",
        level_key_column: "mastery_level",
    }),
};

// =============================================================================
// Dbuff Collections (damage buff system)
// =============================================================================

/// Dbuff collections main table
/// Format: {"_key": 1, "aggregateMode": "Maximum", "developerDescription": "...", ...}
pub static DBUFF_COLLECTIONS: TableSchema = TableSchema {
    name: "dbuff_collections",
    source_file: "dbuffCollections.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        Column::new("aggregate_mode", ColumnType::Text),
        Column::new("developer_description", ColumnType::Text),
    ],
    foreign_keys: &[],
    indexes: &[],
    child_tables: &[
        "dbuff_item_modifiers",
        "dbuff_location_modifiers",
        "dbuff_location_group_modifiers",
    ],
    array_source: None,
};

/// Dbuff item modifiers junction table
/// Format: {"_key": 1, "itemModifiers": [{"dogmaAttributeID": 37}, ...]}
pub static DBUFF_ITEM_MODIFIERS: TableSchema = TableSchema {
    name: "dbuff_item_modifiers",
    source_file: "dbuffCollections.jsonl",
    columns: &[
        Column::required("collection_id", ColumnType::Integer),
        Column::required("dogma_attribute_id", ColumnType::Integer).json("dogmaAttributeID"),
    ],
    foreign_keys: &[
        ForeignKey::new("collection_id", "dbuff_collections"),
        ForeignKey::new("dogma_attribute_id", "dogma_attributes"),
    ],
    indexes: &[
        Index::on(&["collection_id"]),
        Index::on(&["dogma_attribute_id"]),
    ],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "itemModifiers",
        parent_id_column: "collection_id",
    }),
};

/// Dbuff location modifiers junction table
/// Format: {"_key": 1, "locationModifiers": [{"dogmaAttributeID": 68}, ...]}
pub static DBUFF_LOCATION_MODIFIERS: TableSchema = TableSchema {
    name: "dbuff_location_modifiers",
    source_file: "dbuffCollections.jsonl",
    columns: &[
        Column::required("collection_id", ColumnType::Integer),
        Column::required("dogma_attribute_id", ColumnType::Integer).json("dogmaAttributeID"),
    ],
    foreign_keys: &[
        ForeignKey::new("collection_id", "dbuff_collections"),
        ForeignKey::new("dogma_attribute_id", "dogma_attributes"),
    ],
    indexes: &[
        Index::on(&["collection_id"]),
        Index::on(&["dogma_attribute_id"]),
    ],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "locationModifiers",
        parent_id_column: "collection_id",
    }),
};

/// Dbuff location group modifiers junction table
/// Format: {"_key": 1, "locationGroupModifiers": [{"dogmaAttributeID": 20, "groupID": 46}, ...]}
pub static DBUFF_LOCATION_GROUP_MODIFIERS: TableSchema = TableSchema {
    name: "dbuff_location_group_modifiers",
    source_file: "dbuffCollections.jsonl",
    columns: &[
        Column::required("collection_id", ColumnType::Integer),
        Column::required("dogma_attribute_id", ColumnType::Integer).json("dogmaAttributeID"),
        Column::required("group_id", ColumnType::Integer),
    ],
    foreign_keys: &[
        ForeignKey::new("collection_id", "dbuff_collections"),
        ForeignKey::new("dogma_attribute_id", "dogma_attributes"),
        ForeignKey::new("group_id", "groups"),
    ],
    indexes: &[
        Index::on(&["collection_id"]),
        Index::on(&["dogma_attribute_id"]),
        Index::on(&["group_id"]),
    ],
    child_tables: &[],
    array_source: Some(ArraySource::Simple {
        array_field: "locationGroupModifiers",
        parent_id_column: "collection_id",
    }),
};

// =============================================================================
// Freelance Job Schemas (complex nested structure - simplified for now)
// =============================================================================

/// Freelance job schemas - simplified table (complex nested localized content)
/// Full structure has deeply nested localized content that may need custom handling
pub static FREELANCE_JOB_SCHEMAS: TableSchema = TableSchema {
    name: "freelance_job_schemas",
    source_file: "freelanceJobSchemas.jsonl",
    columns: &[
        Column::required("id", ColumnType::Integer),
        // TODO: Expand with more columns as needed - structure is complex with
        // deeply nested localized content
    ],
    foreign_keys: &[],
    indexes: &[],
    child_tables: &[],
    array_source: None,
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
    &LANDMARKS,
    &NPC_CORPORATION_DIVISIONS,
    &PLANET_RESOURCES,
    &CLONE_GRADES,
    &PLANET_SCHEMATICS,
    &DBUFF_COLLECTIONS,
    &FREELANCE_JOB_SCHEMAS,
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
    &SOVEREIGNTY_UPGRADES,
    // Wave 4: Level 3 deps
    &ANCESTRIES,
    &MAP_SOLAR_SYSTEMS,
    &BLUEPRINTS,
    &SKIN_LICENSES,
    &CERTIFICATES,
    &COMPRESSIBLE_TYPES,
    &NPC_CHARACTERS,
    // Wave 5: Level 4 deps (map objects)
    &MAP_STARS,
    &MAP_PLANETS,
    &MAP_MOONS,
    &MAP_ASTEROID_BELTS,
    &MAP_STARGATES,
    &NPC_STATIONS,
    &AGENTS_IN_SPACE,
    // Wave 6: Junction tables (from nested arrays)
    &TYPE_DOGMA_ATTRIBUTES,
    &TYPE_DOGMA_EFFECTS,
    &TYPE_MATERIALS,
    &BLUEPRINT_MATERIALS,
    &BLUEPRINT_PRODUCTS,
    &BLUEPRINT_SKILLS,
    &CLONE_GRADE_SKILLS,
    &CONTRABAND_TYPE_FACTIONS,
    &CONTROL_TOWER_RESOURCES,
    &DYNAMIC_ITEM_ATTRIBUTES,
    &PLANET_SCHEMATIC_PINS,
    &PLANET_SCHEMATIC_TYPES,
    // Wave 7: Complex nested junction tables (require special parser handling)
    &TYPE_ROLE_BONUSES,
    &TYPE_TRAIT_BONUSES, // NOTE: Requires NestedKeyValue parser support
    &TYPE_MASTERIES,     // NOTE: Requires DoubleNested parser support
    &DBUFF_ITEM_MODIFIERS,
    &DBUFF_LOCATION_MODIFIERS,
    &DBUFF_LOCATION_GROUP_MODIFIERS,
];

/// Get table schema by name
pub fn get_table(name: &str) -> Option<&'static TableSchema> {
    ALL_TABLES.iter().find(|t| t.name == name).copied()
}

/// Get all table names
pub fn table_names() -> Vec<&'static str> {
    ALL_TABLES.iter().map(|t| t.name).collect()
}
