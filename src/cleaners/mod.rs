pub mod caches;
pub mod docker;

use anyhow::Result;
use console::style;

pub async fn run_quick_cleanup(dry_run: bool, yes: bool) -> Result<()> {
    println!("{}", style("🚀 Running Quick Cleanup...").bold().green());
    println!("This will clean safe cache directories that can be easily regenerated.\n");
    
    let mut total_freed = 0u64;
    
    // Homebrew Cache
    println!("{} {}...", style("🧹").cyan(), "Homebrew Cache");
    match caches::cleanup_homebrew(dry_run, yes).await {
        Ok(size) => {
            if size > 0 {
                println!("  {} {}", style("✓").green(), crate::utils::format_size(size));
                total_freed += size;
            } else {
                println!("  {} Nothing to clean", style("ℹ").blue());
            }
        }
        Err(e) => println!("  {} Error: {}", style("✗").red(), e),
    }
    
    // pip Cache
    println!("{} {}...", style("🧹").cyan(), "pip Cache");
    match caches::cleanup_pip(dry_run, yes).await {
        Ok(size) => {
            if size > 0 {
                println!("  {} {}", style("✓").green(), crate::utils::format_size(size));
                total_freed += size;
            } else {
                println!("  {} Nothing to clean", style("ℹ").blue());
            }
        }
        Err(e) => println!("  {} Error: {}", style("✗").red(), e),
    }
    
    // npm Cache
    println!("{} {}...", style("🧹").cyan(), "npm Cache");
    match caches::cleanup_npm(dry_run, yes).await {
        Ok(size) => {
            if size > 0 {
                println!("  {} {}", style("✓").green(), crate::utils::format_size(size));
                total_freed += size;
            } else {
                println!("  {} Nothing to clean", style("ℹ").blue());
            }
        }
        Err(e) => println!("  {} Error: {}", style("✗").red(), e),
    }
    
    // Composer Cache
    println!("{} {}...", style("🧹").cyan(), "Composer Cache");
    match caches::cleanup_composer(dry_run, yes).await {
        Ok(size) => {
            if size > 0 {
                println!("  {} {}", style("✓").green(), crate::utils::format_size(size));
                total_freed += size;
            } else {
                println!("  {} Nothing to clean", style("ℹ").blue());
            }
        }
        Err(e) => println!("  {} Error: {}", style("✗").red(), e),
    }
    
    // node-gyp Cache
    println!("{} {}...", style("🧹").cyan(), "node-gyp Cache");
    match caches::cleanup_node_gyp(dry_run, yes).await {
        Ok(size) => {
            if size > 0 {
                println!("  {} {}", style("✓").green(), crate::utils::format_size(size));
                total_freed += size;
            } else {
                println!("  {} Nothing to clean", style("ℹ").blue());
            }
        }
        Err(e) => println!("  {} Error: {}", style("✗").red(), e),
    }
    
    if total_freed > 0 {
        println!("\n{} Total freed: {}", 
            style("🎉").green(),
            crate::utils::format_size(total_freed)
        );
    }
    
    Ok(())
}