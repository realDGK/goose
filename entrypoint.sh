#!/bin/bash
set -e
set -x

# Set OLLAMA_HOST for Ollama itself
export OLLAMA_HOST=0.0.0.0:11434

# Set OLLAMA_CUDA env variable
export OLLAMA_CUDA=1

# Verify GPU is available
echo "Checking for NVIDIA GPU..."
if command -v nvidia-smi &> /dev/null; then
    echo "NVIDIA GPU detected:"
    nvidia-smi
else
    echo "WARNING: NVIDIA GPU not detected. Container will run without GPU acceleration."
fi

# Start Ollama in the background, capturing output
echo "Starting Ollama server..."
ollama serve > /tmp/ollama.log 2>&1 &
ollama_pid=$!

# Wait for Ollama to start, with a timeout
echo "Waiting for Ollama to start..."
timeout=30
start_time=$(date +%s)
until curl -s $OLLAMA_HOST/api/tags >/dev/null 2>&1; do
    current_time=$(date +%s)
    elapsed_time=$((current_time - start_time))
    if [ "$elapsed_time" -ge "$timeout" ]; then
        echo "ERROR: Ollama server failed to start within $timeout seconds. Check /tmp/ollama.log for details."
        kill "$ollama_pid" || true  # Ensure Ollama process is killed
        exit 1
    fi
    echo "Waiting for Ollama server to be available..."
    sleep 5  # Adding a small delay before retrying
done

echo "Ollama server is running"

# Keep the container running
echo "Container setup complete, running forever..."
tail -f /dev/null
