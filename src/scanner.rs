use anyhow::Result;
use byte_unit::{Byte, UnitType};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use tokio::task;
use walkdir::WalkDir;

pub struct StorageInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub cache_breakdown: HashMap<String, u64>,
}

pub async fn show_storage_info() -> Result<()> {
    println!("{}", style("📊 Analyzing Storage Usage...").bold().yellow());
    
    // Get overall disk usage
    let storage = get_storage_info().await?;
    
    println!("\n{}", style("💾 Disk Usage:").bold().white());
    println!("  Total:     {}", format_size(storage.total));
    println!("  Used:      {} ({}%)", 
        format_size(storage.used),
        (storage.used * 100 / storage.total)
    );
    println!("  Available: {}", format_size(storage.available));
    
    // Show cache breakdown
    if !storage.cache_breakdown.is_empty() {
        println!("\n{}", style("🗂️  Cache Directory Sizes:").bold().white());
        let mut sorted: Vec<_> = storage.cache_breakdown.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        
        for (name, size) in sorted.iter().take(15) {
            if **size > 1024 * 1024 { // Only show caches > 1MB
                println!("  {:<25} {}", 
                    style(name).cyan(),
                    format_size(**size)
                );
            }
        }
    }
    
    // Check Docker usage
    if let Ok(docker_info) = get_docker_info().await {
        println!("\n{}", style("🐳 Docker Usage:").bold().white());
        println!("  Images:     {}", docker_info.images);
        println!("  Containers: {}", docker_info.containers);
        println!("  Total Size: {}", format_size(docker_info.total_size));
        if docker_info.reclaimable_size > 0 {
            println!("  Reclaimable: {} ({}%)", 
                format_size(docker_info.reclaimable_size),
                (docker_info.reclaimable_size * 100 / docker_info.total_size.max(1))
            );
        }
    }
    
    Ok(())
}

pub async fn get_storage_info() -> Result<StorageInfo> {
    let home = env::var("HOME")?;
    let home_path = Path::new(&home);
    
    // Get disk usage using df command
    let output = Command::new("df")
        .arg("-h")
        .arg(&home)
        .output()?;
    
    let df_output = String::from_utf8(output.stdout)?;
    let (total, used, available) = parse_df_output(&df_output)?;
    
    // Scan cache directories
    let cache_breakdown = scan_cache_directories(home_path).await?;
    
    Ok(StorageInfo {
        total,
        used,
        available,
        cache_breakdown,
    })
}

async fn scan_cache_directories(home: &Path) -> Result<HashMap<String, u64>> {
    let mut cache_breakdown = HashMap::new();
    
    // macOS cache directories
    let cache_dirs = vec![
        (home.join("Library/Caches"), "Library/Caches"),
        (home.join(".cache"), ".cache"), // Linux
        (home.join("Downloads"), "Downloads"),
        (home.join(".npm"), ".npm"),
        (home.join(".cargo"), ".cargo"),
        (home.join(".gradle"), ".gradle"),
        (home.join("node_modules"), "node_modules"),
    ];
    
    let pb = ProgressBar::new(cache_dirs.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap());
    
    for (path, name) in cache_dirs {
        pb.set_message(format!("Scanning {}", name));
        
        if path.exists() {
            let size = task::spawn_blocking({
                let path = path.clone();
                move || calculate_directory_size(&path)
            }).await??;
            
            if size > 0 {
                // Check for subdirectories in caches
                if name == "Library/Caches" && path.exists() {
                    if let Ok(entries) = fs::read_dir(&path) {
                        for entry in entries.flatten() {
                            if entry.file_type().map_or(false, |t| t.is_dir()) {
                                let subdir_size = task::spawn_blocking({
                                    let subdir_path = entry.path();
                                    move || calculate_directory_size(&subdir_path)
                                }).await??;
                                
                                if subdir_size > 1024 * 1024 { // > 1MB
                                    if let Some(subdir_name) = entry.file_name().to_str() {
                                        cache_breakdown.insert(
                                            format!("Caches/{}", subdir_name),
                                            subdir_size
                                        );
                                    }
                                }
                            }
                        }
                    }
                } else {
                    cache_breakdown.insert(name.to_string(), size);
                }
            }
        }
        
        pb.inc(1);
    }
    
    pb.finish_with_message("Cache scan complete");
    
    Ok(cache_breakdown)
}

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

#[derive(Debug)]
pub struct DockerInfo {
    pub images: String,
    pub containers: String,
    pub total_size: u64,
    pub reclaimable_size: u64,
}

async fn get_docker_info() -> Result<DockerInfo> {
    let output = Command::new("docker")
        .args(&["system", "df"])
        .output()?;
    
    let output_str = String::from_utf8(output.stdout)?;
    parse_docker_output(&output_str)
}

fn parse_docker_output(output: &str) -> Result<DockerInfo> {
    let lines: Vec<&str> = output.lines().collect();
    let mut images = String::new();
    let mut containers = String::new();
    let mut total_size = 0u64;
    let mut reclaimable_size = 0u64;
    
    for line in lines {
        if line.starts_with("Images") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                images = format!("{} total, {} active", parts[1], parts[2]);
                total_size += parse_size_string(parts[4]).unwrap_or(0);
                reclaimable_size += parse_size_string(parts[5]).unwrap_or(0);
            }
        } else if line.starts_with("Containers") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                containers = format!("{} total, {} running", parts[1], parts[2]);
            }
        }
    }
    
    Ok(DockerInfo {
        images,
        containers,
        total_size,
        reclaimable_size,
    })
}

fn parse_df_output(output: &str) -> Result<(u64, u64, u64)> {
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() < 2 {
        return Err(anyhow::anyhow!("Invalid df output"));
    }
    
    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 4 {
        return Err(anyhow::anyhow!("Invalid df output format"));
    }
    
    let total = parse_size_string(parts[1])?;
    let used = parse_size_string(parts[2])?;
    let available = parse_size_string(parts[3])?;
    
    Ok((total, used, available))
}

fn parse_size_string(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim();
    
    if size_str.ends_with("GB") || size_str.ends_with("Gi") {
        let num: f64 = size_str.trim_end_matches("GB").trim_end_matches("Gi").parse()?;
        Ok((num * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("MB") || size_str.ends_with("Mi") {
        let num: f64 = size_str.trim_end_matches("MB").trim_end_matches("Mi").parse()?;
        Ok((num * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("KB") || size_str.ends_with("Ki") {
        let num: f64 = size_str.trim_end_matches("KB").trim_end_matches("Ki").parse()?;
        Ok((num * 1024.0) as u64)
    } else if size_str.ends_with("B") {
        let num: u64 = size_str.trim_end_matches("B").parse()?;
        Ok(num)
    } else {
        // Try parsing as plain number (assuming bytes)
        size_str.parse::<u64>().map_err(|e| anyhow::anyhow!("Failed to parse size: {}", e))
    }
}

fn format_size(bytes: u64) -> String {
    let byte = Byte::from_u64(bytes);
    byte.get_appropriate_unit(UnitType::Binary).to_string()
}