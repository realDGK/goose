#!/bin/bash

# Setup script for customized Goose AI
echo "Setting up customized Goose AI..."

# Copy scripts to the home directory
cp scripts/local/test-goose.sh ~/
cp scripts/local/push-to-github.sh ~/

# Make scripts executable
chmod +x ~/test-goose.sh
chmod +x ~/push-to-github.sh

echo "Setup complete!"
