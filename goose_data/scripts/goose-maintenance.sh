#!/bin/bash

# Goose AI Maintenance Script
# This script helps with regular maintenance and backup of your Goose AI installation

# Colors for better readability
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}===== Goose AI Maintenance Tool =====${NC}"
echo "Please select an option:"
echo -e "${YELLOW}1${NC}) Check for changes"
echo -e "${YELLOW}2${NC}) Backup to GitHub"
echo -e "${YELLOW}3${NC}) Update from original repository"
echo -e "${YELLOW}4${NC}) Create a new feature branch"
echo -e "${YELLOW}5${NC}) Run Goose AI"
echo -e "${YELLOW}6${NC}) Exit"

read -p "Enter your choice (1-6): " choice

case $choice in
  1)
    # Check for changes
    cd /home/scott/goose-ai
    echo -e "${GREEN}Checking for changes in Goose AI...${NC}"
    git status
    ;;
  2)
    # Backup to GitHub
    cd /home/scott/goose-ai
    echo -e "${GREEN}Backing up Goose AI to GitHub...${NC}"
    
    # Prepare the backup first
    /home/scott/goose-prep-backup.sh
    
    # Add all changes
    git add .
    
    # Commit with timestamp
    commit_msg="Backup $(date '+%Y-%m-%d %H:%M:%S')"
    read -p "Enter commit message [$commit_msg]: " user_msg
    if [ ! -z "$user_msg" ]; then
      commit_msg="$user_msg"
    fi
    
    git commit -m "$commit_msg"
    git push -u origin main
    echo -e "${GREEN}Backup complete!${NC}"
    ;;
  3)
    # Update from original repository
    cd /home/scott/goose-ai
    echo -e "${GREEN}Checking for updates from original repository...${NC}"
    
    # Check if upstream remote exists, if not add it
    if ! git remote | grep -q "upstream"; then
      git remote add upstream https://github.com/block/goose.git
    fi
    
    git fetch upstream
    echo -e "${YELLOW}Warning: This might overwrite your customizations.${NC}"
    read -p "Do you want to continue? (y/n): " confirm
    if [ "$confirm" = "y" ]; then
      git merge upstream/main
      echo -e "${GREEN}Update complete!${NC}"
    else
      echo "Update cancelled."
    fi
    ;;
  4)
    # Create a new feature branch
    cd /home/scott/goose-ai
    echo -e "${GREEN}Creating a new feature branch...${NC}"
    read -p "Enter branch name: " branch_name
    if [ ! -z "$branch_name" ]; then
      git checkout -b "$branch_name"
      echo -e "${GREEN}Created and switched to branch: $branch_name${NC}"
    else
      echo "Branch creation cancelled (no name provided)."
    fi
    ;;
  5)
    # Run Goose AI
    echo -e "${GREEN}Running Goose AI...${NC}"
    /home/scott/test-goose.sh
    ;;
  6)
    # Exit
    echo "Exiting maintenance tool."
    exit 0
    ;;
  *)
    echo "Invalid option. Please try again."
    ;;
esac
