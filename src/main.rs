use anyhow::Result;
use eve_sde_to_sqlite::{
    cli::{Cli, Commands},
    download::ensure_sde_downloaded,
    filter::resolve_tables,
    schema::table_names,
    ui::{Phase, SilentUi, Ui, UiApp},
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
            if cli.quiet {
                let mut ui = SilentUi::new();
                run_sync(&mut ui, output_db, include, exclude, force, cache_dir)?;
            } else {
                let mut ui = UiApp::new()?;
                run_sync(
                    &mut ui,
                    output_db.clone(),
                    include,
                    exclude,
                    force,
                    cache_dir,
                )?;
                ui.finish("Complete")?;
            }
        }

        Commands::Download { output, force } => {
            if cli.quiet {
                let mut ui = SilentUi::new();
                run_download(&mut ui, output, force)?;
            } else {
                let mut ui = UiApp::new()?;
                run_download(&mut ui, output, force)?;
                ui.finish("Complete")?;
            }
        }

        Commands::Convert {
            input_dir,
            output_db,
            include,
            exclude,
        } => {
            if cli.quiet {
                let mut ui = SilentUi::new();
                run_convert(&mut ui, input_dir, output_db, include, exclude)?;
            } else {
                let mut ui = UiApp::new()?;
                run_convert(
                    &mut ui,
                    input_dir.clone(),
                    output_db.clone(),
                    include,
                    exclude,
                )?;
                ui.finish("Complete")?;
            }
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

fn run_sync(
    ui: &mut impl Ui,
    output_db: std::path::PathBuf,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    force: bool,
    cache_dir: Option<std::path::PathBuf>,
) -> Result<()> {
    let start = Instant::now();

    // Download SDE if needed
    let (input_dir, build_number) = ensure_sde_downloaded(cache_dir, force, ui)?;

    // Resolve table filters
    let tables = resolve_tables(include, exclude)?;
    ui.log(format!("Selected {} tables for import", tables.len()));

    // Convert to SQLite
    ui.set_phase(Phase::Converting);
    ui.log("Converting to SQLite...");
    let record_count = convert_to_sqlite(&input_dir, &output_db, tables, ui)?;

    let elapsed = start.elapsed();
    let summary = format!(
        "Created {:?} ({} records) from SDE build {} in {:.1}s",
        output_db,
        record_count,
        build_number,
        elapsed.as_secs_f64()
    );
    ui.log(&summary);
    println!("{}", summary);

    Ok(())
}

fn run_download(ui: &mut impl Ui, output: Option<std::path::PathBuf>, force: bool) -> Result<()> {
    let (path, build_number) = ensure_sde_downloaded(output, force, ui)?;
    let summary = format!("SDE build {} downloaded to {:?}", build_number, path);
    ui.log(&summary);
    println!("{}", summary);

    Ok(())
}

fn run_convert(
    ui: &mut impl Ui,
    input_dir: std::path::PathBuf,
    output_db: std::path::PathBuf,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
) -> Result<()> {
    let start = Instant::now();

    // Resolve table filters
    let tables = resolve_tables(include, exclude)?;
    ui.log(format!("Selected {} tables for import", tables.len()));

    // Convert to SQLite
    ui.set_phase(Phase::Converting);
    ui.set_info(format!("Output: {:?}", output_db));
    ui.log("Converting to SQLite...");
    let record_count = convert_to_sqlite(&input_dir, &output_db, tables, ui)?;

    let elapsed = start.elapsed();
    let summary = format!(
        "Created {:?} ({} records) in {:.1}s",
        output_db,
        record_count,
        elapsed.as_secs_f64()
    );
    ui.log(&summary);
    println!("{}", summary);

    Ok(())
}
