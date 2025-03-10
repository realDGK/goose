#!/bin/bash

# Comprehensive script to finalize Goose AI backup

echo "=== Finalizing Goose AI Backup ==="
echo "This script will perform all remaining steps to complete your backup."

# Step 1: Add goose_data and workspace folders
echo -e "\n1. Adding goose_data and workspace folders..."
chmod +x /home/scott/goose-add-data-folders.sh
/home/scott/goose-add-data-folders.sh

# Step 2: Move all scripts to goose_data/scripts
echo -e "\n2. Moving utility scripts to goose_data/scripts..."
chmod +x /home/scott/goose-move-scripts.sh
/home/scott/goose-move-scripts.sh

# Step 3: Copy external config files
echo -e "\n3. Copying external configuration files..."
mkdir -p /home/scott/goose-ai/goose_data/config
cp -r /home/scott/.config/goose/* /home/scott/goose-ai/goose_data/config/ 2>/dev/null || true

# Step 4: Set up VS Code integration
echo -e "\n4. Setting up VS Code integration..."
chmod +x /home/scott/goose-setup-vscode.sh
/home/scott/goose-setup-vscode.sh

# Step 5: Verify the backup
echo -e "\n5. Verifying the backup..."
cd /home/scott/goose-ai
echo "Total tracked files in repository: $(git ls-files | wc -l)"
echo "goose_data files: $(git ls-files goose_data/ | wc -l)"
echo "Local scripts: $(git ls-files scripts/local/ | wc -l)"
echo "VS Code setup: $(git ls-files | grep -c vscode)"

# Step 6: Create a clone test script
echo -e "\n6. Creating a clone test script..."
cat > /home/scott/goose-ai/goose_data/scripts/test-clone.sh << 'EOF'
#!/bin/bash

# Script to test cloning and setting up Goose AI

echo "Testing Goose AI clone and setup process..."

# Create a temporary directory
TEMP_DIR="/tmp/goose-test-clone"
mkdir -p "$TEMP_DIR"

# Clone the repository
echo "Cloning the repository..."
git clone https://github.com/realDGK/goose.git "$TEMP_DIR/goose-ai"

# Run the setup script
echo "Running setup script..."
cd "$TEMP_DIR/goose-ai"
[ -f "goose_data/scripts/goose-setup-after-clone.sh" ] && \
  chmod +x goose_data/scripts/goose-setup-after-clone.sh && \
  ./goose_data/scripts/goose-setup-after-clone.sh

echo "Test clone complete!"
echo "The test clone is available at: $TEMP_DIR/goose-ai"
echo "You can inspect it to verify everything was properly backed up."
echo "Remember to delete this test directory when finished: rm -rf $TEMP_DIR"
EOF

# Make the test script executable
chmod +x /home/scott/goose-ai/goose_data/scripts/test-clone.sh

# Commit these final changes
echo -e "\n7. Committing final changes..."
cd /home/scott/goose-ai
git add goose_data/
git commit -m "Finalize backup with all configuration files and scripts"
git push origin main

echo -e "\n=== Backup Finalization Complete ==="
echo "Your Goose AI installation is now fully backed up to GitHub."
echo "To open in VS Code, run: code /home/scott/goose-ai.code-workspace"
echo "To test the clone process, run: ./goose-ai/goose_data/scripts/test-clone.sh"
