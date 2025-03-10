#!/bin/bash

# Script to fix email privacy issue with GitHub

echo "Fixing email privacy issue for GitHub..."

# Change to the goose-ai directory
cd /home/scott/goose-ai

# Configure git to use GitHub's assigned no-reply email address
git config user.email "32527377+realDGK@users.noreply.github.com"
git config user.name "realDGK"

# Amend the last commit to use the new email
git commit --amend --reset-author --no-edit

echo "Email address updated. Now trying to push again..."
git push -f origin main

echo "Push attempt complete! Check your repository at https://github.com/realDGK/goose"
