#!/bin/bash

# Script to set up VS Code integration for the Goose AI repository

echo "Setting up VS Code integration for Goose AI..."

# Create a VS Code workspace file
cat > /home/scott/goose-ai.code-workspace << 'EOF'
{
  "folders": [
    {
      "path": "/home/scott/goose-ai"
    }
  ],
  "settings": {
    "editor.formatOnSave": true,
    "editor.codeActionsOnSave": {
      "source.fixAll": true
    },
    "files.exclude": {
      "**/.git": false,
      "**/node_modules": true,
      "**/target": true
    },
    "git.enableSmartCommit": true,
    "git.confirmSync": false,
    "git.autofetch": true,
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.checkOnSave.command": "clippy",
    "files.watcherExclude": {
      "**/target/**": true,
      "**/node_modules/**": true
    }
  },
  "extensions": {
    "recommendations": [
      "rust-lang.rust-analyzer",
      "github.vscode-pull-request-github",
      "eamodio.gitlens",
      "ms-azuretools.vscode-docker"
    ]
  }
}
EOF

echo "VS Code workspace file created at /home/scott/goose-ai.code-workspace"

# Also create a helper script for opening VS Code with the workspace
cat > /home/scott/goose-ai/goose_data/scripts/open-vscode.sh << 'EOF'
#!/bin/bash

# Script to open Goose AI in VS Code

echo "Opening Goose AI in VS Code..."
code /home/scott/goose-ai.code-workspace
EOF

chmod +x /home/scott/goose-ai/goose_data/scripts/open-vscode.sh

# Add a note to the README
cat > /home/scott/goose-ai/goose_data/VS_CODE_SETUP.md << 'EOF'
# VS Code Setup for Goose AI

## Opening the Project in VS Code

There are two ways to open this project in VS Code:

### Method 1: Using the Workspace File

1. Open VS Code
2. Go to File > Open Workspace from File...
3. Select `/home/scott/goose-ai.code-workspace`

### Method 2: Using the Script

1. Run the helper script:
   ```bash
   ./goose_data/scripts/open-vscode.sh
   ```

## Workspace Configuration

The workspace is configured with:

- Rust analyzer settings for optimal Rust development
- Git integration with autofetch enabled
- Performance optimizations for large Rust projects
- Recommended extensions

## Troubleshooting

If VS Code doesn't show the repository:

1. Make sure the path is correct in the workspace file
2. Try opening VS Code from WSL terminal: `code .`
3. Ensure the Remote WSL extension is installed in VS Code
EOF

# Add the files to git
echo "Would you like to add the VS Code setup files to git? (y/n)"
read add_to_git

if [ "$add_to_git" = "y" ]; then
  echo "Adding files to git..."
  cd /home/scott/goose-ai
  git add goose_data/scripts/open-vscode.sh
  git add goose_data/VS_CODE_SETUP.md
  
  # Commit the changes
  git commit -m "Add VS Code integration files"
  
  # Push to GitHub
  git push origin main
  
  echo "Changes committed and pushed to GitHub!"
else
  echo "VS Code setup files created but not added to git. You can manually add them later."
fi

echo "VS Code setup complete!"
echo "To open the project in VS Code, run: code /home/scott/goose-ai.code-workspace"
