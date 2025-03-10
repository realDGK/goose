# Goose Data Directory

This directory contains user-specific data and configuration for Goose AI.

## Directory Structure

- `config/`: Contains configuration files copied from ~/.config/goose
- `scripts/`: Contains utility scripts for managing Goose AI
- `backups/`: Contains backup documentation and files

## Restoring from this Directory

To restore your Goose AI configuration:

1. Copy config files back to their original location:
   ```
   mkdir -p ~/.config/goose
   cp -r goose_data/config/* ~/.config/goose/
   ```

2. Make scripts executable:
   ```
   chmod +x goose_data/scripts/*.sh
   ```

This organization keeps all your Goose AI customizations in one location for easier backup and restoration.
