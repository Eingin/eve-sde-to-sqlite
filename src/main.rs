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
