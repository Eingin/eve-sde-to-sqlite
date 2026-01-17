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
