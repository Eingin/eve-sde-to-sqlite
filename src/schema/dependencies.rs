use super::tables::{get_table, ALL_TABLES};
use super::types::TableSchema;
use std::collections::{HashMap, HashSet, VecDeque};

/// Resolves table dependencies for filtering
pub struct DependencyResolver {
    /// Map of table name -> tables it depends on
    deps: HashMap<&'static str, HashSet<&'static str>>,
    /// Map of table name -> tables that depend on it (reserved for future use)
    #[allow(dead_code)]
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
                reverse_deps.entry(dep).or_default().insert(table.name);
            }
        }

        Self { deps, reverse_deps }
    }

    /// Given a set of requested tables, resolve all required dependencies
    /// Returns tables in dependency order (parents before children)
    pub fn resolve_includes(
        &self,
        requested: &[&str],
    ) -> Result<Vec<&'static TableSchema>, String> {
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
                let parent_excluded = table
                    .foreign_keys
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
    fn topological_sort(
        &self,
        included: &HashSet<&str>,
    ) -> Result<Vec<&'static TableSchema>, String> {
        let mut result = Vec::new();
        let mut visited: HashSet<&str> = HashSet::new();
        let mut temp_visited: HashSet<&str> = HashSet::new();

        for table_name in included {
            if !visited.contains(table_name) {
                self.visit(
                    table_name,
                    included,
                    &mut visited,
                    &mut temp_visited,
                    &mut result,
                )?;
            }
        }

        Ok(result)
    }

    fn visit<'a>(
        &self,
        name: &'a str,
        included: &HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
        temp_visited: &mut HashSet<&'a str>,
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
                // Skip self-references (e.g., market_groups.parent_group_id -> market_groups)
                if *dep != name && included.contains(dep) {
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
