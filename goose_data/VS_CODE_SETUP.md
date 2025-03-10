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
