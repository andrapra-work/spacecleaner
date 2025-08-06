#!/bin/bash

# SpaceCleaner GUI Wrapper
# Simple AppleScript-based GUI for macOS users

export PATH="$HOME/.local/bin:$PATH"

# Check if spacecleaner is installed
if ! command -v spacecleaner &> /dev/null; then
    osascript -e 'display dialog "SpaceCleaner is not installed. Please run the installer first." buttons {"OK"} default button 1'
    exit 1
fi

# Show main menu
choice=$(osascript << 'EOF'
tell application "System Events"
    set choices to {"🔍 Scan Storage Usage", "🚀 Quick Safe Cleanup", "🗂️ Clean All Caches", "🐳 Docker Cleanup", "❌ Cancel"}
    set selectedChoice to choose from list choices with title "🧹 SpaceCleaner" with prompt "What would you like to do?" default items {"🔍 Scan Storage Usage"} OK button name "Continue" cancel button name "Cancel"
    if selectedChoice is false then
        return "cancel"
    else
        return item 1 of selectedChoice
    end if
end tell
EOF
)

if [ "$choice" = "cancel" ]; then
    exit 0
fi

# Show safety warning for cleanup operations
if [[ "$choice" != *"Scan"* ]]; then
    safety=$(osascript << 'EOF'
tell application "System Events"
    set response to display dialog "⚠️ SAFETY FIRST ⚠️

Before cleaning:
• Close all browsers (Chrome, Safari, Firefox)
• Save your work in all applications
• This will delete cache files (they can be recreated)

Would you like to preview changes first? (Recommended)" buttons {"Preview Only", "Clean Now", "Cancel"} default button 1
    return button returned of response
end tell
EOF
    )
    
    case "$safety" in
        "Cancel") exit 0 ;;
        "Preview Only") DRY_RUN="--dry-run" ;;
        "Clean Now") DRY_RUN="" ;;
    esac
fi

# Execute the appropriate command
case "$choice" in
    *"Scan"*)
        osascript -e 'tell application "Terminal" to do script "spacecleaner scan; echo \"Press any key to close...\"; read"'
        ;;
    *"Quick"*)
        osascript -e "tell application \"Terminal\" to do script \"spacecleaner $DRY_RUN --yes quick; echo 'Press any key to close...'; read\""
        ;;
    *"All Caches"*)
        osascript -e "tell application \"Terminal\" to do script \"spacecleaner $DRY_RUN caches; echo 'Press any key to close...'; read\""
        ;;
    *"Docker"*)
        osascript -e "tell application \"Terminal\" to do script \"spacecleaner $DRY_RUN docker; echo 'Press any key to close...'; read\""
        ;;
esac