#!/bin/bash

# This script helps push the Goose AI changes to GitHub
# Replace YOUR_TOKEN with your GitHub Personal Access Token

# Set the GitHub token
GITHUB_TOKEN="YOUR_TOKEN"

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Configure Git to use the token
git remote set-url origin https://${GITHUB_TOKEN}@github.com/realDGK/goose-ai.git

# Push the changes
git push -u origin main

echo "Changes pushed to GitHub successfully!"
