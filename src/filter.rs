use crate::schema::{DependencyResolver, TableSchema};
use anyhow::{anyhow, bail, Result};

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
            let tables = resolver.resolve_includes(&refs).map_err(|e| anyhow!(e))?;
            
            println!("Including {} tables:", tables.len());
            for t in &tables {
                println!("  - {}", t.name);
            }
            
            Ok(tables)
        }
        (None, Some(exclude_list)) => {
            let refs: Vec<&str> = exclude_list.iter().map(|s| s.as_str()).collect();
            println!("Excluding tables: {:?}", refs);
            let tables = resolver.resolve_excludes(&refs).map_err(|e| anyhow!(e))?;
            
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
