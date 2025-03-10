#!/bin/bash

# Goose AI Backup Preparation Script
# This script copies important files from the parent directory into the goose-ai directory

echo "Preparing for Goose AI backup..."

# Create a scripts directory if it doesn't exist
mkdir -p /home/scott/goose-ai/scripts/local

# Copy important scripts from parent directory
echo "Copying external scripts..."
cp /home/scott/test-goose.sh /home/scott/goose-ai/scripts/local/
cp /home/scott/push-to-github.sh /home/scott/goose-ai/scripts/local/

# Create a setup file to make reinstallation easier
echo "Creating setup script..."
cat > /home/scott/goose-ai/scripts/local/setup.sh << 'EOF'
#!/bin/bash

# Goose AI Setup Script
# This script helps set up Goose AI after cloning from GitHub

echo "Setting up Goose AI..."

# Copy scripts to the parent directory
cp ./scripts/local/test-goose.sh /home/scott/
cp ./scripts/local/push-to-github.sh /home/scott/

# Make the scripts executable
chmod +x /home/scott/test-goose.sh
chmod +x /home/scott/push-to-github.sh

# Run any necessary installation steps
# Add installation commands here if needed

echo "Setup complete! Your Goose AI installation is ready to use."
EOF

# Make all scripts executable
chmod +x /home/scott/goose-ai/scripts/local/*.sh

echo "Preparation complete! Now run the goose-backup.sh script to back up to GitHub."
