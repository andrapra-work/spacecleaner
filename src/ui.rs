use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use std::path::Path;
use walkdir::WalkDir;

fn calculate_directory_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total_size += metadata.len();
            }
        }
    }
    
    Ok(total_size)
}

pub async fn run_interactive_mode(dry_run: bool, yes: bool) -> Result<()> {
    println!("{}", style("🎯 Interactive Cleanup Mode").bold().magenta());
    
    if dry_run {
        println!("{}", style("🔍 DRY RUN MODE - No files will actually be deleted").yellow());
    }
    
    loop {
        println!("\n{}", style("What would you like to do?").bold());
        
        let options = vec![
            "📊 Scan storage usage",
            "🚀 Quick cleanup (safe caches)",
            "🗂️  Clean all caches",
            "🐳 Docker cleanup",
            "🎯 Custom cleanup menu",
            "❌ Exit",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&options)
            .interact()?;
        
        match selection {
            0 => {
                // Scan storage usage
                crate::scanner::show_storage_info().await?;
                
                println!("\n{}", style("💡 Tip: Use other options to free up space!").dim());
            }
            1 => {
                // Quick cleanup
                crate::cleaners::run_quick_cleanup(dry_run, yes).await?;
            }
            2 => {
                // Clean all caches
                crate::cleaners::caches::cleanup_all_caches(dry_run, yes).await?;
            }
            3 => {
                // Docker cleanup
                crate::cleaners::docker::cleanup_docker(dry_run, yes).await?;
            }
            4 => {
                // Custom cleanup menu
                run_custom_cleanup_menu(dry_run, yes).await?;
            }
            5 => {
                // Exit
                println!("{}", style("👋 Thanks for using SpaceCleaner!").green());
                break;
            }
            _ => unreachable!(),
        }
        
        // Ask if user wants to continue
        if !yes {
            let continue_cleanup = Confirm::new()
                .with_prompt("Continue with more cleanup options?")
                .default(true)
                .interact()?;
            
            if !continue_cleanup {
                break;
            }
        }
    }
    
    Ok(())
}

async fn run_custom_cleanup_menu(dry_run: bool, yes: bool) -> Result<()> {
    println!("\n{}", style("🎯 Custom Cleanup Menu").bold().cyan());
    
    let options = vec![
        "🍺 Homebrew cache",
        "🐍 pip cache", 
        "📦 npm cache",
        "🎼 Composer cache",
        "⚙️  node-gyp cache",
        "🎭 Playwright cache",
        "🌐 Browser caches",
        "🗑️  System temp files",
        "📱 Development caches",
        "🔙 Back to main menu",
    ];
    
    loop {
        println!("\n{}", style("Select items to clean:").bold());
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose cleanup target")
            .default(0)
            .items(&options)
            .interact()?;
        
        match selection {
            0 => {
                let size = crate::cleaners::caches::cleanup_homebrew(dry_run, yes).await?;
                show_cleanup_result("Homebrew cache", size);
            }
            1 => {
                let size = crate::cleaners::caches::cleanup_pip(dry_run, yes).await?;
                show_cleanup_result("pip cache", size);
            }
            2 => {
                let size = crate::cleaners::caches::cleanup_npm(dry_run, yes).await?;
                show_cleanup_result("npm cache", size);
            }
            3 => {
                let size = crate::cleaners::caches::cleanup_composer(dry_run, yes).await?;
                show_cleanup_result("Composer cache", size);
            }
            4 => {
                let size = crate::cleaners::caches::cleanup_node_gyp(dry_run, yes).await?;
                show_cleanup_result("node-gyp cache", size);
            }
            5 => {
                let size = crate::cleaners::caches::cleanup_playwright(dry_run, yes).await?;
                show_cleanup_result("Playwright cache", size);
            }
            6 => {
                let size = crate::cleaners::caches::cleanup_browser_caches(dry_run, yes).await?;
                show_cleanup_result("Browser caches", size);
            }
            7 => {
                let size = crate::cleaners::caches::cleanup_temp_files(dry_run, yes).await?;
                show_cleanup_result("System temp files", size);
            }
            8 => {
                cleanup_development_caches(dry_run, yes).await?;
            }
            9 => {
                // Back to main menu
                break;
            }
            _ => unreachable!(),
        }
        
        // Ask if user wants to continue in custom menu
        if !yes {
            let continue_custom = Confirm::new()
                .with_prompt("Clean more items?")
                .default(true)
                .interact()?;
            
            if !continue_custom {
                break;
            }
        }
    }
    
    Ok(())
}

async fn cleanup_development_caches(dry_run: bool, yes: bool) -> Result<()> {
    println!("\n{}", style("📱 Development Cache Cleanup").bold().cyan());
    
    let dev_caches = vec![
        ("Gradle cache", "~/.gradle/caches"),
        ("Maven cache", "~/.m2/repository"),
        ("Cargo cache", "~/.cargo/registry"),
        ("Go module cache", "~/go/pkg/mod"),
        ("Android build cache", "~/.android/build-cache"),
    ];
    
    for (name, path) in dev_caches {
        let expanded_path = shellexpand::tilde(path);
        let path_obj = std::path::Path::new(expanded_path.as_ref());
        
        if path_obj.exists() {
            let size = calculate_directory_size(path_obj)?;
            
            if size > 1024 * 1024 { // > 1MB
                if dry_run {
                    println!("  Would clean {}: {}", name, crate::utils::format_size(size));
                } else {
                    if yes || Confirm::new()
                        .with_prompt(&format!("Clean {}? ({})", name, crate::utils::format_size(size)))
                        .default(false) // Conservative default for dev caches
                        .interact()?
                    {
                        match std::fs::remove_dir_all(&*expanded_path) {
                            Ok(_) => {
                                println!("  {} Cleaned {}: {}", 
                                    style("✓").green(),
                                    name,
                                    crate::utils::format_size(size)
                                );
                            }
                            Err(e) => {
                                println!("  {} Could not clean {}: {}", 
                                    style("✗").red(),
                                    name,
                                    e
                                );
                            }
                        }
                    }
                }
            } else {
                println!("  {} {}: {} (too small to clean)", 
                    style("ℹ").blue(),
                    name,
                    crate::utils::format_size(size)
                );
            }
        } else {
            println!("  {} {}: Not found", style("ℹ").dim(), name);
        }
    }
    
    Ok(())
}

fn show_cleanup_result(name: &str, size: u64) {
    if size > 0 {
        println!("  {} Cleaned {}: {}", 
            style("✓").green(),
            name,
            crate::utils::format_size(size)
        );
    } else {
        println!("  {} {}: Nothing to clean", 
            style("ℹ").blue(),
            name
        );
    }
}