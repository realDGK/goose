# Goose AI Backup & Maintenance Guide

This guide explains how to back up your customized Goose AI installation to GitHub and set up VS Code for ongoing development.

## Setup Summary

I've created several scripts to help you manage your Goose AI installation:

1. **goose-prep-backup.sh**: Prepares your installation for backup by copying important files
2. **goose-backup.sh**: Performs the actual backup to GitHub
3. **goose-maintenance.sh**: An all-in-one tool for managing your Goose AI installation
4. **goose-ai.code-workspace**: VS Code workspace file for easy project management

## Backup Instructions

### One-Time Backup

To perform a one-time backup of your Goose AI installation:

1. Open a WSL terminal
2. Run the preparation script:
   ```bash
   cd /home/scott
   ./goose-prep-backup.sh
   ```
3. Run the backup script:
   ```bash
   ./goose-backup.sh
   ```
4. Enter a commit message when prompted

### Regular Maintenance

For regular maintenance, use the maintenance script:

1. Open a WSL terminal
2. Run the maintenance script:
   ```bash
   cd /home/scott
   ./goose-maintenance.sh
   ```
3. Select option 2 to back up to GitHub

## VS Code Integration

To set up VS Code:

1. Open VS Code
2. Use "File > Open Workspace from File..."
3. Navigate to and select `/home/scott/goose-ai.code-workspace`
4. VS Code will load the project with recommended settings and extensions

Once opened, you can use the built-in Git features in VS Code:
- Source Control tab (Ctrl+Shift+G) to commit changes
- Status bar for Git operations
- Terminal (Ctrl+`) to run the maintenance script

## Restoring from Backup

If you need to restore from your GitHub backup:

1. Clone your repository:
   ```bash
   git clone https://github.com/realDGK/goose.git
   ```
2. Run the setup script:
   ```bash
   cd goose
   ./scripts/local/setup.sh
   ```

## What's Included in the Backup

The backup includes:
- All your customized Goose AI files
- Important scripts from your home directory
- A setup script for easy restoration
- Configuration files for VS Code

## What's Excluded

To keep the repository size manageable, these are excluded:
- Model files (these can be redownloaded)
- Build artifacts
- Temporary files
- Backup files

## Troubleshooting

If you encounter issues:

1. Check if your GitHub credentials are set up correctly
2. Ensure you have proper permissions for the repository
3. Run `git status` to see what changes are pending
4. Check if Git is properly installed in WSL

## Ongoing Development

For ongoing development:
1. Use the maintenance script to create feature branches
2. Commit changes regularly
3. Use VS Code's Git integration for visualization
4. Run the backup script before making major changes

Remember to back up regularly to avoid data loss!
