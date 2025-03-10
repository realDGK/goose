#!/bin/bash

# Script to add goose_data and workspace folders to the Goose AI backup

echo "Adding goose_data and workspace folders to the Goose AI backup..."

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Update .gitignore to not exclude goose_data and workspace
echo "Updating .gitignore to include goose_data and workspace..."
sed -i '/\/goose_data\//d' .gitignore
sed -i '/\/workspace\//d' .gitignore

# Add an exceptions section to .gitignore
cat >> .gitignore << 'EOF'

# Large file exceptions - Include goose_data and workspace but exclude large model files
# Include goose_data directory but exclude large files within it
!/goose_data/
/goose_data/**/*.bin
/goose_data/**/*.gguf
/goose_data/**/*.onnx
/goose_data/**/*.model
/goose_data/**/*.weights

# Include workspace directory
!/workspace/
EOF

# Add the files to git
echo "Adding files to git..."
git add .gitignore
git add goose_data/ workspace/ --ignore-errors

# Commit the changes
echo "Committing changes..."
git commit -m "Add goose_data and workspace directories"

# Push to GitHub
echo "Pushing to GitHub..."
git push origin main

echo "Additional directories have been added to the backup!"
echo "Note: Large model files within these directories are still excluded to keep the repository size manageable."
