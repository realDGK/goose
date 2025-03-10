FROM nvidia/cuda:12.4.0-base-ubuntu22.04

# Install minimal dependencies
RUN apt-get update && apt-get install -y \
    curl wget git build-essential ca-certificates bzip2 software-properties-common \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Add the universe repository
RUN add-apt-repository universe

# Install Ollama 0.1.26 and set env variables
ENV OLLAMA_CUDA=1
ENV NVIDIA_VISIBLE_DEVICES=all

# The key fix: use a specific version that works
RUN wget https://github.com/ollama/ollama/releases/download/v0.1.26/ollama-linux-amd64 -O /usr/local/bin/ollama && \
    chmod +x /usr/local/bin/ollama && \
    /usr/local/bin/ollama --version

# Create the 'scott' user with UID 1000 and a home directory
RUN groupadd -g 1000 scott && \
    useradd -u 1000 -g scott -m -s /bin/bash scott

# Create directories AND SET OWNERSHIP
RUN mkdir -p /home/scott/.config/goose /shared/code && \
    chown -R scott:scott /home/scott/.config/goose /shared/code

# Copy entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Set the user
USER scott

WORKDIR /home/scott/goose-ai

EXPOSE 11434
ENTRYPOINT ["/entrypoint.sh"]
