#!/bin/bash

# Goose AI Backup Script
# This script backs up your customized Goose AI installation to GitHub

echo "Starting Goose AI backup process..."

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Make sure we're on the main branch
git checkout main

# Check which files are tracked
echo "Checking status of repository..."
git status

# Add all changes to the repository (including new files)
echo "Adding all changes to git..."
git add .

# Prompt for a commit message
echo "Enter a commit message for this backup (e.g., 'Backup with customizations'):"
read commit_message

# Default message if none provided
if [ -z "$commit_message" ]; then
  commit_message="Goose AI backup $(date '+%Y-%m-%d %H:%M:%S')"
fi

# Commit the changes
echo "Committing changes with message: $commit_message"
git commit -m "$commit_message"

# Push to GitHub
echo "Pushing changes to GitHub..."
git push -u origin main

echo "Backup complete! Your Goose AI customizations have been backed up to GitHub."
echo "Repository URL: https://github.com/realDGK/goose"
