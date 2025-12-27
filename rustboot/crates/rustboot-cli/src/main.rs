mod commands;

use clap::{Parser, Subcommand};
use commands::{add, new};

#[derive(Parser)]
#[command(name = "rustboot")]
#[command(about = "CLI tool for scaffolding Rustboot projects", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Rustboot project
    New {
        /// Name of the project
        name: String,

        /// Path where the project should be created (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Add a feature to an existing Rustboot project
    Add {
        /// Feature to add (database, auth, api)
        feature: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, path } => {
            new::execute(&name, path.as_deref())?;
        }
        Commands::Add { feature } => {
            add::execute(&feature)?;
        }
    }

    Ok(())
}
