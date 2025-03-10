# Configuration Files

This directory contains example configuration files that show the structure
but do not contain actual sensitive values.

## Sensitive Files (Not in Git)

The following files contain sensitive information and are NOT included in the Git repository:

- `env`: Contains API keys and other sensitive environment variables
- Other files with extensions: `.key`, `.token`, `.secret`

## Example Files

Example files with the `.example` extension are provided to show the structure:

- `env.example`: Template for environment variables

## Setting Up After Cloning

After cloning this repository, run the setup script:

```bash
./goose_data/scripts/setup-config.sh
```

This will create the configuration files from the examples, which you can then
edit to add your actual values.

## Adding New Configuration Files

When adding new configuration files that contain sensitive information:

1. Add the actual file to `.gitignore`
2. Create an example version with the `.example` extension
3. Update this README to document the new file
