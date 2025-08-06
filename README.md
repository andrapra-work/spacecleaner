# 🧹 SpaceCleaner

**Fast storage cleanup tool for macOS and Linux**

SpaceCleaner is a blazing-fast command-line tool written in Rust that helps you reclaim disk space by cleaning various cache directories and unused files safely.

## ✨ Features

- **🚀 Blazing Fast**: Built in Rust with parallel directory scanning
- **🎯 Interactive Mode**: Menu-driven interface for guided cleanup
- **🔍 Dry Run Mode**: Preview what will be deleted before actually deleting
- **🐳 Docker Integration**: Clean Docker images, containers, and build cache
- **🗂️ Comprehensive Cache Cleaning**: Supports 15+ different cache types
- **📊 Storage Analysis**: Detailed breakdown of disk usage
- **🛡️ Safety First**: Conservative defaults and confirmation prompts

## 📦 Installation

### Build from source (recommended)

```bash
# Clone and build
cd ~/Code/spacecleaner
cargo build --release

# Install to PATH
cargo install --path .
```

### Direct execution
```bash
cd ~/Code/spacecleaner
cargo run -- --help
```

## 🚀 Usage

### Commands

```bash
# Interactive mode (default)
spacecleaner

# Scan current storage usage
spacecleaner scan

# Quick cleanup of safe caches
spacecleaner quick

# Clean all cache directories
spacecleaner caches

# Docker-specific cleanup
spacecleaner docker

# Dry run mode (preview only)
spacecleaner --dry-run

# Skip confirmations
spacecleaner --yes quick
```

### Interactive Mode

The interactive mode provides a menu-driven interface:

```
🎯 Interactive Cleanup Mode

What would you like to do?
> 📊 Scan storage usage
  🚀 Quick cleanup (safe caches)
  🗂️  Clean all caches
  🐳 Docker cleanup
  🎯 Custom cleanup menu
  ❌ Exit
```

## 🧹 What It Cleans

### Safe Caches (Quick Cleanup)
- **Homebrew Cache**: Package manager cache
- **pip Cache**: Python package cache
- **npm Cache**: Node.js package cache
- **Composer Cache**: PHP package cache  
- **node-gyp Cache**: Node.js build cache

### All Cache Cleanup
- All of the above plus:
- **Playwright Cache**: Browser automation cache
- **Browser Caches**: Chrome, Safari, Firefox (when closed)
- **System Temp Files**: Old temporary files (7+ days)
- **Development Caches**: Gradle, Maven, Cargo, Go modules

### Docker Cleanup
- **Unused Images**: Dangling and unused images
- **Stopped Containers**: Non-running containers
- **Unused Volumes**: Unattached volumes (optional)
- **Build Cache**: Docker build cache

## 📊 Example Output

```bash
$ spacecleaner scan

🧹 SpaceCleaner - Fast Storage Cleanup Tool

📊 Analyzing Storage Usage...

💾 Disk Usage:
  Total:     228 GiB
  Used:      138 GiB (60%)
  Available: 55 GiB

🗂️  Cache Directory Sizes:
  .npm                      5.13 GiB
  Caches/Google             2.98 GiB
  .gradle                   2.50 GiB
  Downloads                 2.32 GiB
  node_modules              452 MiB
  
🐳 Docker Usage:
  Images:     9 total, 9 active
  Containers: 9 total, 9 running
  Total Size: 8.17 MiB
```

## 🛡️ Safety Features

- **Dry Run Mode**: Always test with `--dry-run` first
- **Conservative Defaults**: Asks for confirmation on potentially risky operations
- **Smart Detection**: Only cleans what can be safely regenerated
- **Browser Safety**: Won't clean browser caches while browsers are running
- **Selective Cleanup**: Choose exactly what to clean

## 🏗️ Architecture

```
spacecleaner/
├── src/
│   ├── main.rs           # CLI argument parsing
│   ├── scanner.rs        # Disk usage analysis
│   ├── cleaners/         # Cleanup modules
│   │   ├── caches.rs     # Cache directory cleanup
│   │   ├── docker.rs     # Docker cleanup
│   │   └── mod.rs        # Quick cleanup orchestration
│   ├── ui.rs            # Interactive terminal UI
│   └── utils.rs         # Helper functions
```

## 🚧 Platform Support

- **✅ macOS**: Full support for macOS cache directories
- **✅ Linux**: Full support for Linux cache directories  
- **🔄 Windows**: Planned for future release

## ⚡ Performance

- **Parallel Scanning**: Uses tokio for concurrent directory traversal
- **Minimal Memory**: Streaming directory iteration
- **Fast Execution**: Rust's zero-cost abstractions
- **Small Binary**: ~2MB executable

## 📝 License

MIT License - feel free to use and modify!

## 🤝 Contributing

1. Fork the repository at [github.com/andrapra-work/spacecleaner](https://github.com/andrapra-work/spacecleaner)
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 👨‍💻 Author

Created by **Andra Pradana Ardiansyah** ([@andrapra-work](https://github.com/andrapra-work))

## 🔗 Related Tools

- [Docker System Prune](https://docs.docker.com/config/pruning/)
- [Homebrew Cleanup](https://docs.brew.sh/Manpage#cleanup-options-formulae)
- [npm cache clean](https://docs.npmjs.com/cli/v8/commands/npm-cache)

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**⚠️ Always run with `--dry-run` first to see what will be deleted!**