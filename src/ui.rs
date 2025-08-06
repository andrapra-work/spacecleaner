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
            "🎯 Select specific caches to clean",
            "🐳 Docker cleanup",
            "⚙️  Advanced cleanup menu",
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
                // Select specific caches
                run_selective_cache_cleanup(dry_run, yes).await?;
            }
            4 => {
                // Docker cleanup
                crate::cleaners::docker::cleanup_docker(dry_run, yes).await?;
            }
            5 => {
                // Advanced cleanup menu
                run_custom_cleanup_menu(dry_run, yes).await?;
            }
            6 => {
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

async fn run_selective_cache_cleanup(dry_run: bool, yes: bool) -> Result<()> {
    println!("\n{}", style("🎯 Select Specific Caches to Clean").bold().cyan());
    println!("Scanning for cache directories...");
    
    // Get storage info with cache breakdown
    let storage_info = crate::scanner::get_storage_info().await?;
    
    if storage_info.cache_breakdown.is_empty() {
        println!("No cache directories found.");
        return Ok(());
    }
    
    // Sort caches by size (largest first)
    let mut sorted_caches: Vec<_> = storage_info.cache_breakdown.iter().collect();
    sorted_caches.sort_by(|a, b| b.1.cmp(a.1));
    
    // Create selection options
    let mut options = Vec::new();
    let mut cache_paths = Vec::new();
    
    for (name, size) in sorted_caches.iter() {
        if **size > 1024 * 1024 { // Only show caches > 1MB
            options.push(format!("🗂️  {} ({})", name, crate::utils::format_size(**size)));
            cache_paths.push(name.as_str());
        }
    }
    
    if options.is_empty() {
        println!("No significant cache directories found (> 1MB).");
        return Ok(());
    }
    
    options.push("✅ Done selecting".to_string());
    
    let mut total_freed = 0u64;
    let mut selected_caches = Vec::new();
    
    loop {
        println!("\n{}", style("Select caches to clean (you can select multiple):").bold());
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose cache to clean")
            .items(&options)
            .interact()?;
        
        if selection == options.len() - 1 {
            // "Done selecting" was chosen
            break;
        }
        
        let cache_name = cache_paths[selection];
        let cache_size = storage_info.cache_breakdown.get(cache_name).copied().unwrap_or(0);
        
        if selected_caches.contains(&cache_name) {
            println!("  {} {} already selected!", style("ℹ").blue(), cache_name);
            continue;
        }
        
        // Confirm selection
        if yes || Confirm::new()
            .with_prompt(&format!("Add {} ({}) to cleanup list?", 
                cache_name, 
                crate::utils::format_size(cache_size)
            ))
            .default(true)
            .interact()? 
        {
            selected_caches.push(cache_name);
            println!("  {} Added {} to cleanup list", style("✓").green(), cache_name);
        }
    }
    
    if selected_caches.is_empty() {
        println!("No caches selected for cleanup.");
        return Ok(());
    }
    
    // Show summary and confirm
    println!("\n{}", style("📋 Selected Caches for Cleanup:").bold().yellow());
    let mut total_size = 0u64;
    for cache_name in &selected_caches {
        let size = storage_info.cache_breakdown.get(*cache_name).copied().unwrap_or(0);
        total_size += size;
        println!("  🗂️  {} ({})", cache_name, crate::utils::format_size(size));
    }
    println!("  {} Total: {}", style("💰").yellow(), crate::utils::format_size(total_size));
    
    if dry_run {
        println!("\n{} DRY RUN - Nothing will actually be deleted", style("🔍").cyan());
        return Ok(());
    }
    
    // Final confirmation
    if !yes && !Confirm::new()
        .with_prompt(&format!("Clean {} selected caches? ({})", 
            selected_caches.len(),
            crate::utils::format_size(total_size)
        ))
        .default(false)
        .interact()? 
    {
        println!("Cleanup cancelled.");
        return Ok(());
    }
    
    // Perform cleanup
    println!("\n{} Starting cleanup...", style("🧹").cyan());
    for cache_name in selected_caches {
        let size_before = storage_info.cache_breakdown.get(cache_name).copied().unwrap_or(0);
        let cleaned_size = cleanup_specific_cache(cache_name, dry_run).await?;
        
        if cleaned_size > 0 {
            println!("  {} Cleaned {}: {}", 
                style("✓").green(),
                cache_name,
                crate::utils::format_size(cleaned_size)
            );
            total_freed += cleaned_size;
        } else {
            println!("  {} {}: Could not clean or already empty", 
                style("⚠").yellow(),
                cache_name
            );
        }
    }
    
    if total_freed > 0 {
        println!("\n{} Total freed: {}", 
            style("🎉").green(),
            crate::utils::format_size(total_freed)
        );
    }
    
    Ok(())
}

async fn cleanup_specific_cache(cache_name: &str, dry_run: bool) -> Result<u64> {
    use std::env;
    use std::path::Path;
    
    let home = env::var("HOME")?;
    let cache_path = if cache_name.starts_with("Caches/") {
        format!("{}/Library/{}", home, cache_name)
    } else if cache_name.starts_with('.') {
        format!("{}/{}", home, cache_name)
    } else {
        format!("{}/{}", home, cache_name)
    };
    
    let path = Path::new(&cache_path);
    if !path.exists() {
        return Ok(0);
    }
    
    let size_before = calculate_directory_size(path)?;
    
    if dry_run {
        return Ok(size_before);
    }
    
    // Try to remove the directory contents
    match std::fs::remove_dir_all(&cache_path) {
        Ok(_) => {
            // Recreate directory if it's a system cache
            if cache_name.starts_with("Caches/") {
                std::fs::create_dir_all(&cache_path).ok();
            }
            Ok(size_before)
        }
        Err(_) => {
            // If we can't remove the whole directory, try to clean individual files
            cleanup_directory_contents(&cache_path).await
        }
    }
}

async fn cleanup_directory_contents(path: &str) -> Result<u64> {
    use std::fs;
    use walkdir::WalkDir;
    
    let mut total_freed = 0u64;
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                let file_size = metadata.len();
                if fs::remove_file(entry.path()).is_ok() {
                    total_freed += file_size;
                }
            }
        }
    }
    
    Ok(total_freed)
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