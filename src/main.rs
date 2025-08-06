use clap::{Parser, Subcommand};
use anyhow::Result;
use console::style;

mod scanner;
mod cleaners;
mod ui;
mod utils;

#[derive(Parser)]
#[command(
    name = "spacecleaner",
    about = "Fast storage cleanup tool for macOS and Linux",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(long, help = "Show what would be deleted without actually deleting")]
    dry_run: bool,
    
    #[arg(short, long, help = "Skip confirmation prompts")]
    yes: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan and show current storage usage
    Scan,
    /// Interactive cleanup menu
    Interactive,
    /// Quick cleanup of common safe caches
    Quick,
    /// Clean Docker images, containers, and volumes
    Docker,
    /// Clean all cache directories
    Caches,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{}", style("🧹 SpaceCleaner - Fast Storage Cleanup Tool").bold().cyan());
    println!();

    match &cli.command {
        Some(Commands::Scan) => {
            scanner::show_storage_info().await?;
        },
        Some(Commands::Interactive) => {
            ui::run_interactive_mode(cli.dry_run, cli.yes).await?;
        },
        Some(Commands::Quick) => {
            cleaners::run_quick_cleanup(cli.dry_run, cli.yes).await?;
        },
        Some(Commands::Docker) => {
            cleaners::docker::cleanup_docker(cli.dry_run, cli.yes).await?;
        },
        Some(Commands::Caches) => {
            cleaners::caches::cleanup_all_caches(cli.dry_run, cli.yes).await?;
        },
        None => {
            // Default to interactive mode
            ui::run_interactive_mode(cli.dry_run, cli.yes).await?;
        }
    }

    Ok(())
}
