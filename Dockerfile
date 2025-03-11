FROM nvidia/cuda:12.4.0-base-ubuntu22.04

# Set a more permissive umask for the entire container.
# This affects files created by ANY user within the container.
# 0000 means all permissions (read, write, execute) for everyone.
# This is VERY permissive.
RUN umask 0000

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

# --- Goose AI Installation (Added Section) ---
# We'll install Goose AI *after* setting the user and workdir
# This ensures pip installs packages in the user's home directory

# Install Python dependencies (if Goose AI has any)
# Assuming there's a requirements.txt.  Add this if needed!
# COPY --chown=scott:scott requirements.txt /home/scott/goose-ai/
# RUN pip install --no-cache-dir --user -r requirements.txt

# Copy Goose AI files (ensure correct ownership)
COPY --chown=scott:scott goose-ai /home/scott/goose-ai

# Install Goose AI (using pip install .  --user is important)
RUN pip install --no-cache-dir --user .


EXPOSE 11434
ENTRYPOINT ["/entrypoint.sh"]