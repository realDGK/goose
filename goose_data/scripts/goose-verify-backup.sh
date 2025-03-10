#!/bin/bash

# Script to verify the Goose AI backup

echo "Verifying Goose AI backup..."

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Check what files are tracked in git
echo "Currently tracked files in git:"
git ls-files | sort > /tmp/tracked_files.txt

# Count tracked files
tracked_count=$(wc -l < /tmp/tracked_files.txt)
echo "Total tracked files: $tracked_count"

# Check for important directories that might be missing
echo -e "\nChecking for important directories..."

if grep -q "config_backup/" /tmp/tracked_files.txt; then
  echo "✅ External configuration files are included in the backup"
else
  echo "❌ External configuration files are NOT included in the backup"
  echo "   Run ./goose-add-config-files.sh to add them"
fi

if grep -q "scripts/local/" /tmp/tracked_files.txt; then
  echo "✅ Local scripts are included in the backup"
else
  echo "❌ Local scripts are NOT included in the backup"
fi

# Check for important project files
echo -e "\nChecking for important project files..."
important_files=("README.md" "Cargo.toml" "config.yaml" ".gitignore")

for file in "${important_files[@]}"; do
  if grep -q "^$file$" /tmp/tracked_files.txt; then
    echo "✅ $file is included in the backup"
  else
    echo "❌ $file is NOT included in the backup"
  fi
done

# Check if any large files are being tracked (could cause issues)
echo -e "\nChecking for potentially large files..."
large_extensions=(".bin" ".gguf" ".model" ".weights" ".tar" ".tar.gz" ".zip")

for ext in "${large_extensions[@]}"; do
  large_files=$(grep "$ext$" /tmp/tracked_files.txt)
  if [ -n "$large_files" ]; then
    echo "⚠️ Found potential large files with extension $ext:"
    echo "$large_files"
  else
    echo "✅ No large files with extension $ext found"
  fi
done

echo -e "\nBackup verification complete!"
