#!/bin/bash

# Script to update the README in the repository

echo "Updating README in Goose AI repository..."

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Copy the custom README
cp /home/scott/custom-README.md README.md

# Commit and push the change
git add README.md
git commit -m "Update README with customization information"
git push origin main

echo "README updated successfully!"
