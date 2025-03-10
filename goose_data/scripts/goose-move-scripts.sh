#!/bin/bash

# Script to move all utility scripts to goose_data/scripts

echo "Moving utility scripts to goose_data/scripts directory..."

# Change to home directory
cd /home/scott

# Create scripts directory in goose_data if it doesn't exist
mkdir -p goose-ai/goose_data/scripts

# Copy all goose-related scripts from home directory
echo "Copying scripts..."
cp goose-*.sh goose-ai/goose_data/scripts/
cp test-goose.sh goose-ai/goose_data/scripts/ 2>/dev/null || true
cp push-to-github.sh goose-ai/goose_data/scripts/ 2>/dev/null || true
cp GOOSE-BACKUP-INSTRUCTIONS.md goose-ai/goose_data/scripts/ 2>/dev/null || true

# Make all scripts executable
chmod +x goose-ai/goose_data/scripts/*.sh

# Create a README file explaining the scripts
cat > goose-ai/goose_data/scripts/README.md << 'EOF'
# Goose AI Utility Scripts

This directory contains utility scripts for managing, backing up, and restoring your Goose AI installation.

## Main Scripts

- `test-goose.sh`: Script to test Goose AI functionality
- `push-to-github.sh`: Helper script for pushing to GitHub
- `goose-backup.sh`: Script for backing up Goose AI
- `goose-setup-after-clone.sh`: Script for setting up after cloning the repository

## Running These Scripts

Make the scripts executable before running:

```bash
chmod +x *.sh
```

Then run a script with:

```bash
./script-name.sh
```

These scripts help you maintain and manage your Goose AI installation.
EOF

# Add the files to git
echo "Would you like to add these moved scripts to git? (y/n)"
read add_to_git

if [ "$add_to_git" = "y" ]; then
  echo "Adding files to git..."
  cd /home/scott/goose-ai
  git add goose_data/scripts/
  
  # Commit the changes
  git commit -m "Move utility scripts to goose_data/scripts directory"
  
  # Push to GitHub
  git push origin main
  
  echo "Changes committed and pushed to GitHub!"
else
  echo "Scripts moved but not added to git. You can manually add them later."
fi

echo "Script movement complete!"
