#!/bin/bash

# Script to set up configuration files after cloning

echo "Setting up configuration files..."

# Create config directory if it doesn't exist
mkdir -p ~/.config/goose
mkdir -p $(pwd)/goose_data/config

# Copy example files to actual locations
for example in $(pwd)/goose_data/config/*.example; do
  # Get the filename without .example
  basefilename=$(basename "$example" .example)
  targetfile=~/.config/goose/"$basefilename"
  
  # Only create if it doesn't exist
  if [ ! -f "$targetfile" ]; then
    echo "Creating $targetfile from example..."
    cp "$example" "$targetfile"
    echo "⚠️ Please edit $targetfile to add your actual values"
  else
    echo "File $targetfile already exists, not overwriting"
  fi
done

echo "Configuration setup complete!"
echo "Remember to update the actual configuration files with your personal values."
