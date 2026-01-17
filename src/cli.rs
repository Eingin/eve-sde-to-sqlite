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
