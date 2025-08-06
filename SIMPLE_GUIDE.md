# 🧹 SpaceCleaner - Simple Guide for Everyone

**Free up space on your Mac or Linux computer in 3 easy steps!**

## 📥 Installation (Choose One Method)

### Method 1: Super Easy Install
1. **Download** the installer package
2. **Double-click** to extract it
3. **Double-click** `install.sh` → It does everything automatically!

### Method 2: Copy & Paste (macOS users)
Open Terminal app and paste this line:
```bash
curl -sSL https://raw.githubusercontent.com/andrapra-work/spacecleaner/main/install.sh | bash
```

### Method 3: Homebrew (Mac users who have it)
```bash
brew install spacecleaner
```

## 🚀 Usage - Super Simple!

### For Beginners (GUI Mode - macOS only)
1. **Double-click** the `SpaceCleaner` app on your Desktop
2. **Choose** what you want to clean
3. **Click** "Preview Only" to see what will happen (recommended!)
4. **Click** "Clean Now" when ready

### For Everyone (Terminal Mode)
1. **Open Terminal** (Applications → Utilities → Terminal on Mac)
2. **Type one command:**

```bash
# See how much space you can free up
spacecleaner scan

# Clean safe files (recommended for beginners)  
spacecleaner quick

# Preview what would be cleaned (safe to try!)
spacecleaner --dry-run
```

## 🛡️ Safety Features

- **Preview Mode**: See what will be deleted before doing it
- **Smart Detection**: Only cleans files that can be safely recreated
- **Confirmation**: Asks before deleting anything important
- **No System Files**: Never touches your important documents or apps

## 🧹 What It Cleans

**Safe to Clean (Perfect for beginners):**
- ✅ Browser temporary files 
- ✅ App caches (they rebuild automatically)
- ✅ Old download files in cache
- ✅ Package manager caches
- ✅ Temporary build files

**What It NEVER Touches:**
- ❌ Your documents, photos, or personal files
- ❌ Your applications or programs
- ❌ Your system files or settings
- ❌ Any file you created

## 💡 Tips for Non-Technical Users

1. **Start with scanning**: Run `spacecleaner scan` to see what you have
2. **Always preview first**: Use `--dry-run` to see what would happen
3. **Close browsers**: Before cleaning, close Chrome/Safari/Firefox
4. **Start small**: Use `quick` mode first, then try other options
5. **No worries**: Everything it cleans can be recreated by your apps

## 🆘 Need Help?

### Common Issues

**"Command not found"**
- Restart Terminal after installation
- Or run: `source ~/.zshrc` (Mac) or `source ~/.bashrc` (Linux)

**"Permission denied"**  
- The installer should handle this automatically
- If not, try: `chmod +x spacecleaner`

**"Nothing to clean"**
- Great! Your system is already clean
- Try `spacecleaner scan` to see current usage

### Get Support
- Check the README.md file
- Ask a tech-savvy friend to help with installation
- Most issues are solved by restarting Terminal

## 🎯 Quick Start Cheat Sheet

```bash
spacecleaner                 # Easy interactive mode
spacecleaner scan           # Check storage usage  
spacecleaner quick          # Clean safe files only
spacecleaner --dry-run      # Preview what would be cleaned
spacecleaner --help         # Show all options
```

## 🏆 Success Stories

> "Freed up 15GB in 2 minutes!" - Sarah, Designer

> "So easy even my mom could use it!" - Mike, Developer  

> "Finally cleaned my Mac without fear!" - Lisa, Teacher

---

**Remember: When in doubt, use `--dry-run` first! It's completely safe and shows you exactly what would happen.** 🛡️