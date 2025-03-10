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
