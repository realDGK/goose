#!/bin/bash

# Script to organize Goose AI files by moving external files into goose_data

echo "Organizing Goose AI files..."

# Change to home directory
cd /home/scott

# Create necessary directories in goose_data if they don't exist
mkdir -p goose-ai/goose_data/config
mkdir -p goose-ai/goose_data/scripts
mkdir -p goose-ai/goose_data/backups

# First, copy config files from .config/goose to goose_data/config
echo "Copying config files from ~/.config/goose to goose_data/config..."
cp -r ~/.config/goose/* goose-ai/goose_data/config/

# Next, copy all the script files we created to goose_data/scripts
echo "Moving script files to goose_data/scripts..."
cp goose-*.sh goose-ai/goose_data/scripts/
cp GOOSE-BACKUP-INSTRUCTIONS.md goose-ai/goose_data/backups/
cp custom-README.md goose-ai/goose_data/backups/

# Create a README file explaining the organization
cat > goose-ai/goose_data/README.md << 'EOF'
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
EOF

# Add the files to git
echo "Would you like to add these organized files to git? (y/n)"
read add_to_git

if [ "$add_to_git" = "y" ]; then
  echo "Adding files to git..."
  cd /home/scott/goose-ai
  git add goose_data/
  
  # Commit the changes
  git commit -m "Organize files into goose_data directory"
  
  # Push to GitHub
  git push origin main
  
  echo "Changes committed and pushed to GitHub!"
else
  echo "Files organized but not added to git. You can manually add them later."
fi

echo "File organization complete!"
