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
