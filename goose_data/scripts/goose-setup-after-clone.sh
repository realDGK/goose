#!/bin/bash

# Goose AI Setup Script
# Run this after cloning the repository to set up your Goose AI environment

echo "Setting up Goose AI environment..."

# Create necessary directories
echo "Creating necessary directories..."
mkdir -p ~/.config/goose

# Copy external configuration files
if [ -d "config_backup/.config/goose" ]; then
  echo "Copying configuration files..."
  cp -r config_backup/.config/goose/* ~/.config/goose/
else
  echo "Warning: External configuration files not found in backup"
fi

# Copy useful scripts to home directory
if [ -d "scripts/local" ]; then
  echo "Copying local scripts to home directory..."
  cp scripts/local/*.sh ~/
  chmod +x ~/*.sh
else
  echo "Warning: Local scripts directory not found"
fi

# Create symlinks for commonly used files
echo "Creating symlinks for easy access..."
ln -sf $(pwd)/config.yaml ~/goose-config.yaml

# Check for required tools and suggest installation if missing
echo "Checking for required tools..."
tools=("cargo" "npm" "rustc")

for tool in "${tools[@]}"; do
  if ! command -v $tool &> /dev/null; then
    echo "⚠️ $tool is not installed. You may need to install it."
  else
    echo "✅ $tool is installed"
  fi
done

# Set up VS Code integration if VS Code is installed
if command -v code &> /dev/null; then
  echo "Setting up VS Code integration..."
  cat > ~/goose-ai.code-workspace << 'EOF'
{
  "folders": [
    {
      "path": "."
    }
  ],
  "settings": {
    "editor.formatOnSave": true,
    "files.exclude": {
      "**/.git": false,
      "**/target": true,
      "**/node_modules": true
    },
    "git.enableSmartCommit": true,
    "git.confirmSync": false,
    "git.autofetch": true
  },
  "extensions": {
    "recommendations": [
      "rust-lang.rust-analyzer",
      "github.vscode-pull-request-github",
      "eamodio.gitlens"
    ]
  }
}
EOF
  echo "VS Code workspace file created at ~/goose-ai.code-workspace"
else
  echo "VS Code not found. Skipping VS Code integration."
fi

echo "Setup complete! Your Goose AI environment is ready."
echo "To start working with Goose AI, follow the instructions in the README.md file."
