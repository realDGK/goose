#!/bin/bash

# Script to add additional configuration files to the Goose AI backup

echo "Adding additional configuration files to the Goose AI backup..."

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Create a directory for external config files
mkdir -p config_backup/.config/goose

# Copy config files from .config/goose
echo "Copying files from ~/.config/goose..."
cp -r ~/.config/goose/* config_backup/.config/goose/

# Add a README file explaining how to restore these files
cat > config_backup/README.md << 'EOF'
# External Configuration Files

This directory contains configuration files that were originally stored outside the main Goose AI directory.

## Restoration Instructions

To fully restore your Goose AI setup, copy these files to their original locations:

```bash
# Copy config files
mkdir -p ~/.config/goose
cp -r config_backup/.config/goose/* ~/.config/goose/
```

These files contain your personal settings, API keys (if any), and other configuration options.
EOF

# Add the files to git
echo "Adding files to git..."
git add config_backup/

# Commit the changes
echo "Committing changes..."
git commit -m "Add external configuration files"

# Push to GitHub
echo "Pushing to GitHub..."
git push origin main

echo "Additional configuration files have been added to the backup!"
