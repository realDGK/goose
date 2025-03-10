#!/bin/bash

# Simple script to push to GitHub
# Run this if the main script fails but the repository is set up

echo "Pushing to GitHub..."
cd /home/scott/goose-ai

# Get the current branch name
current_branch=$(git branch --show-current)
echo "Current branch: $current_branch"

if [ "$current_branch" != "main" ]; then
  echo "Creating and switching to main branch..."
  git checkout -b main
fi

echo "Force pushing to GitHub..."
git push -f origin main

echo "Push complete! Check your repository at https://github.com/realDGK/goose"
