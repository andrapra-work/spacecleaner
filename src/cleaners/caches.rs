use anyhow::Result;
use console::style;
use dialoguer::Confirm;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
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

pub async fn cleanup_all_caches(dry_run: bool, yes: bool) -> Result<()> {
    println!("{}", style("🗂️  Cleaning All Cache Directories...").bold().cyan());
    
    let mut total_freed = 0u64;
    
    // Run all cleaners individually
    
    println!("\n{} {}...", style("🧹").cyan(), "Homebrew Cache");
    match cleanup_homebrew(dry_run, yes).await {
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
    
    println!("\n{} {}...", style("🧹").cyan(), "pip Cache");
    match cleanup_pip(dry_run, yes).await {
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
    
    println!("\n{} {}...", style("🧹").cyan(), "npm Cache");
    match cleanup_npm(dry_run, yes).await {
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
    
    println!("\n{} {}...", style("🧹").cyan(), "Composer Cache");
    match cleanup_composer(dry_run, yes).await {
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
    
    println!("\n{} {}...", style("🧹").cyan(), "node-gyp Cache");
    match cleanup_node_gyp(dry_run, yes).await {
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
    
    println!("\n{} {}...", style("🧹").cyan(), "Playwright Cache");
    match cleanup_playwright(dry_run, yes).await {
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
    
    println!("\n{} {}...", style("🧹").cyan(), "Browser Caches");
    match cleanup_browser_caches(dry_run, yes).await {
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
    
    println!("\n{} {}...", style("🧹").cyan(), "System Temp Files");
    match cleanup_temp_files(dry_run, yes).await {
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

pub async fn cleanup_homebrew(dry_run: bool, yes: bool) -> Result<u64> {
    let home = env::var("HOME")?;
    let cache_path = format!("{}/Library/Caches/Homebrew", home);
    
    cleanup_directory(&cache_path, "Homebrew cache", dry_run, yes).await
}

pub async fn cleanup_pip(dry_run: bool, yes: bool) -> Result<u64> {
    let home = env::var("HOME")?;
    let cache_path = format!("{}/Library/Caches/pip", home);
    
    cleanup_directory(&cache_path, "pip cache", dry_run, yes).await
}

pub async fn cleanup_npm(dry_run: bool, yes: bool) -> Result<u64> {
    let home = env::var("HOME")?;
    let cache_path = format!("{}/.npm", home);
    
    if Path::new(&cache_path).exists() {
        let size_before = calculate_directory_size(Path::new(&cache_path))?;
        
        if !dry_run {
            if yes || Confirm::new()
                .with_prompt("Clear npm cache?")
                .default(true)
                .interact()? 
            {
                Command::new("npm")
                    .args(&["cache", "clean", "--force"])
                    .output()?;
                
                let size_after = if Path::new(&cache_path).exists() {
                    calculate_directory_size(Path::new(&cache_path))?
                } else {
                    0
                };
                
                return Ok(size_before.saturating_sub(size_after));
            }
        } else {
            println!("  Would clean npm cache: {}", crate::utils::format_size(size_before));
            return Ok(size_before);
        }
    }
    
    Ok(0)
}

pub async fn cleanup_composer(dry_run: bool, yes: bool) -> Result<u64> {
    let home = env::var("HOME")?;
    let cache_path = format!("{}/Library/Caches/composer", home);
    
    cleanup_directory(&cache_path, "Composer cache", dry_run, yes).await
}

pub async fn cleanup_node_gyp(dry_run: bool, yes: bool) -> Result<u64> {
    let home = env::var("HOME")?;
    let cache_path = format!("{}/Library/Caches/node-gyp", home);
    
    cleanup_directory(&cache_path, "node-gyp cache", dry_run, yes).await
}

pub async fn cleanup_playwright(dry_run: bool, yes: bool) -> Result<u64> {
    let home = env::var("HOME")?;
    let cache_path = format!("{}/Library/Caches/ms-playwright", home);
    
    cleanup_directory(&cache_path, "Playwright cache", dry_run, yes).await
}

pub async fn cleanup_browser_caches(dry_run: bool, yes: bool) -> Result<u64> {
    let home = env::var("HOME")?;
    
    let browser_caches = vec![
        format!("{}/Library/Caches/Google/Chrome", home),
        format!("{}/Library/Caches/com.apple.Safari", home),
        format!("{}/Library/Caches/Firefox", home),
    ];
    
    let mut total_freed = 0u64;
    
    for cache_path in browser_caches {
        if Path::new(&cache_path).exists() {
            let size = calculate_directory_size(Path::new(&cache_path))?;
            if size > 0 {
                if dry_run {
                    println!("    Would clean {}: {}", 
                        Path::new(&cache_path).file_name().unwrap_or_default().to_string_lossy(),
                        crate::utils::format_size(size)
                    );
                    total_freed += size;
                } else {
                    let browser_name = Path::new(&cache_path).file_name()
                        .unwrap_or_default().to_string_lossy();
                    
                    if yes || Confirm::new()
                        .with_prompt(&format!("Clear {} cache? (Browser should be closed)", browser_name))
                        .default(false)
                        .interact()? 
                    {
                        if let Err(e) = fs::remove_dir_all(&cache_path) {
                            println!("    Warning: Could not clean {}: {} (browser may be running)", 
                                browser_name, e);
                        } else {
                            total_freed += size;
                        }
                    }
                }
            }
        }
    }
    
    Ok(total_freed)
}

pub async fn cleanup_temp_files(_dry_run: bool, _yes: bool) -> Result<u64> {
    // For safety, we'll only clean very specific temp locations
    let temp_dirs = vec![
        "/tmp",
        "/var/tmp",
    ];
    
    let total_freed = 0u64;
    
    for temp_dir in temp_dirs {
        if Path::new(temp_dir).exists() {
            // Only clean files older than 7 days for safety
            let output = Command::new("find")
                .args(&[temp_dir, "-type", "f", "-mtime", "+7", "-exec", "rm", "{}", "+"])
                .output();
                
            match output {
                Ok(_) => {
                    println!("    Cleaned old temp files from {}", temp_dir);
                    // Note: we can't easily calculate the size freed here
                }
                Err(e) => {
                    println!("    Could not clean {}: {}", temp_dir, e);
                }
            }
        }
    }
    
    Ok(total_freed)
}

async fn cleanup_directory(path: &str, name: &str, dry_run: bool, yes: bool) -> Result<u64> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        return Ok(0);
    }
    
    let size = calculate_directory_size(path_obj)?;
    
    if size == 0 {
        return Ok(0);
    }
    
    if dry_run {
        println!("  Would clean {}: {}", name, crate::utils::format_size(size));
        return Ok(size);
    }
    
    if yes || Confirm::new()
        .with_prompt(&format!("Clean {}? ({})", name, crate::utils::format_size(size)))
        .default(true)
        .interact()? 
    {
        fs::remove_dir_all(path)?;
        // Recreate the directory if it's a system cache
        if name.contains("Cache") {
            fs::create_dir_all(path).ok();
        }
        Ok(size)
    } else {
        Ok(0)
    }
}