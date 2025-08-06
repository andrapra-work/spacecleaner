use anyhow::Result;
use console::style;
use dialoguer::Confirm;
use std::process::Command;

pub async fn cleanup_docker(dry_run: bool, yes: bool) -> Result<()> {
    println!("{}", style("🐳 Docker Cleanup").bold().blue());
    
    // Check if Docker is available
    if !is_docker_available() {
        println!("  {} Docker not found or not running", style("ℹ").blue());
        return Ok(());
    }
    
    // Get current Docker usage
    let docker_info = get_docker_usage().await?;
    
    println!("\n{}", style("Current Docker Usage:").bold());
    println!("  Images: {} ({})", docker_info.image_count, docker_info.image_size);
    println!("  Containers: {} ({})", docker_info.container_count, docker_info.container_size);
    println!("  Volumes: {} ({})", docker_info.volume_count, docker_info.volume_size);
    println!("  Build Cache: {}", docker_info.build_cache_size);
    
    if docker_info.reclaimable_size.is_empty() {
        println!("  {} No reclaimable space found", style("ℹ").blue());
        return Ok(());
    }
    
    println!("  {} Reclaimable: {}", 
        style("💰").yellow(), 
        docker_info.reclaimable_size
    );
    
    if dry_run {
        println!("\n{} Would run: docker system prune -a", style("🔍").cyan());
        return Ok(());
    }
    
    // Offer cleanup options
    println!("\n{}", style("Cleanup Options:").bold());
    
    let cleanup_all = yes || Confirm::new()
        .with_prompt("Remove all unused images, containers, networks, and build cache?")
        .default(true)
        .interact()?;
    
    if cleanup_all {
        run_docker_cleanup(true).await?;
    } else {
        // Individual cleanup options
        let clean_images = Confirm::new()
            .with_prompt("Remove unused images?")
            .default(true)
            .interact()?;
            
        let clean_containers = Confirm::new()
            .with_prompt("Remove stopped containers?")
            .default(true)
            .interact()?;
            
        let clean_volumes = Confirm::new()
            .with_prompt("Remove unused volumes?")
            .default(false) // More dangerous
            .interact()?;
            
        let clean_build_cache = Confirm::new()
            .with_prompt("Remove build cache?")
            .default(true)
            .interact()?;
        
        run_selective_docker_cleanup(clean_images, clean_containers, clean_volumes, clean_build_cache).await?;
    }
    
    Ok(())
}

#[derive(Debug)]
struct DockerUsage {
    image_count: String,
    image_size: String,
    container_count: String,
    container_size: String,
    volume_count: String,
    volume_size: String,
    build_cache_size: String,
    reclaimable_size: String,
}

async fn get_docker_usage() -> Result<DockerUsage> {
    let output = Command::new("docker")
        .args(&["system", "df"])
        .output()?;
    
    let output_str = String::from_utf8(output.stdout)?;
    parse_docker_system_df(&output_str)
}

fn parse_docker_system_df(output: &str) -> Result<DockerUsage> {
    let mut usage = DockerUsage {
        image_count: "0".to_string(),
        image_size: "0B".to_string(),
        container_count: "0".to_string(),
        container_size: "0B".to_string(),
        volume_count: "0".to_string(),
        volume_size: "0B".to_string(),
        build_cache_size: "0B".to_string(),
        reclaimable_size: "0B".to_string(),
    };
    
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if line.starts_with("Images") && parts.len() >= 6 {
            usage.image_count = format!("{} total, {} active", parts[1], parts[2]);
            usage.image_size = parts[4].to_string();
            if parts[5] != "0B" {
                usage.reclaimable_size = parts[5].to_string();
            }
        } else if line.starts_with("Containers") && parts.len() >= 6 {
            usage.container_count = format!("{} total, {} running", parts[1], parts[2]);
            usage.container_size = parts[4].to_string();
        } else if line.starts_with("Local Volumes") && parts.len() >= 6 {
            usage.volume_count = format!("{} total, {} active", parts[2], parts[3]);
            usage.volume_size = parts[5].to_string();
        } else if line.starts_with("Build Cache") && parts.len() >= 5 {
            usage.build_cache_size = parts[3].to_string();
        }
    }
    
    Ok(usage)
}

async fn run_docker_cleanup(all: bool) -> Result<()> {
    println!("\n{} Running Docker cleanup...", style("🧹").cyan());
    
    let args = if all {
        vec!["system", "prune", "-a", "-f"]
    } else {
        vec!["system", "prune", "-f"]
    };
    
    let output = Command::new("docker")
        .args(&args)
        .output()?;
    
    let output_str = String::from_utf8(output.stdout)?;
    
    if output.status.success() {
        // Parse the output to show what was cleaned
        if let Some(total_line) = output_str.lines().last() {
            if total_line.starts_with("Total reclaimed space:") {
                println!("  {} {}", style("✓").green(), total_line);
            }
        }
        
        // Show some of the cleanup details
        let lines: Vec<&str> = output_str.lines().collect();
        for line in lines.iter().take(5) {
            if line.starts_with("Deleted") {
                println!("  {}", style(line).dim());
            }
        }
        
        if lines.len() > 5 {
            println!("  {} ... and more", style("").dim());
        }
    } else {
        let error_str = String::from_utf8(output.stderr)?;
        return Err(anyhow::anyhow!("Docker cleanup failed: {}", error_str));
    }
    
    Ok(())
}

async fn run_selective_docker_cleanup(
    clean_images: bool,
    clean_containers: bool,
    clean_volumes: bool,
    clean_build_cache: bool,
) -> Result<()> {
    let _total_freed = String::new();
    
    if clean_containers {
        println!("  {} Removing stopped containers...", style("🧹").cyan());
        let output = Command::new("docker")
            .args(&["container", "prune", "-f"])
            .output()?;
        
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            if let Some(total_line) = output_str.lines().last() {
                if total_line.starts_with("Total reclaimed space:") {
                    println!("    {} {}", style("✓").green(), total_line);
                }
            }
        }
    }
    
    if clean_images {
        println!("  {} Removing unused images...", style("🧹").cyan());
        let output = Command::new("docker")
            .args(&["image", "prune", "-a", "-f"])
            .output()?;
        
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            if let Some(total_line) = output_str.lines().last() {
                if total_line.starts_with("Total reclaimed space:") {
                    println!("    {} {}", style("✓").green(), total_line);
                }
            }
        }
    }
    
    if clean_volumes {
        println!("  {} Removing unused volumes...", style("🧹").cyan());
        let output = Command::new("docker")
            .args(&["volume", "prune", "-f"])
            .output()?;
        
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            if let Some(total_line) = output_str.lines().last() {
                if total_line.starts_with("Total reclaimed space:") {
                    println!("    {} {}", style("✓").green(), total_line);
                }
            }
        }
    }
    
    if clean_build_cache {
        println!("  {} Removing build cache...", style("🧹").cyan());
        let output = Command::new("docker")
            .args(&["builder", "prune", "-a", "-f"])
            .output()?;
        
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            if let Some(total_line) = output_str.lines().last() {
                if total_line.starts_with("Total reclaimed space:") {
                    println!("    {} {}", style("✓").green(), total_line);
                }
            }
        }
    }
    
    Ok(())
}

fn is_docker_available() -> bool {
    Command::new("docker")
        .args(&["version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}