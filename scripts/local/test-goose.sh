#!/bin/bash

# This script tests the modified Goose AI with the free Gemini model

# Set your OpenRouter API key
export OPENROUTER_API_KEY="YOUR_OPENROUTER_API_KEY"

# Run Goose with minimal display
/home/scott/.local/bin/goose session

echo "Test complete. If Goose was able to connect to the model, the fix was successful!"
