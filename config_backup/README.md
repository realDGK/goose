# External Configuration Files

This directory contains configuration files that were originally stored outside the main Goose AI directory.

## Restoration Instructions

To fully restore your Goose AI setup, copy these files to their original locations:

```bash
# Copy config files
mkdir -p ~/.config/goose
cp -r config_backup/.config/goose/* ~/.config/goose/
```

These files contain your personal settings, API keys (if any), and other configuration options.
