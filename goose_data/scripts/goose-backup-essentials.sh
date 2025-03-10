#!/bin/bash

# Script to back up only essential files from Goose AI

echo "Backing up essential Goose AI files..."

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Make sure the scripts/local directory exists
mkdir -p scripts/local

# Copy important scripts
cp /home/scott/test-goose.sh scripts/local/
cp /home/scott/push-to-github.sh scripts/local/

# Create a list of essential files to back up
cat > .essential-files << 'EOF'
README.md
CONTRIBUTING.md
LICENSE
.gitignore
config.yaml
scripts/local/
Justfile
EOF

# Create a setup script
cat > scripts/local/setup.sh << 'EOF'
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
EOF

# Make the setup script executable
chmod +x scripts/local/setup.sh

# Reset the repository
git reset

# Add only essential files
cat .essential-files | while read file; do
  if [ -e "$file" ]; then
    echo "Adding $file to git..."
    git add "$file"
  fi
done

# Add any custom configuration files
if [ -e "config.yaml" ]; then
  git add config.yaml
fi

# Commit changes
git commit -m "Backup essential files and customizations"

# Push to GitHub
echo "Pushing to GitHub..."
git push -f origin main

echo "Essential backup complete!"
