FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Install system dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    libssl-dev \
    pkg-config \
    curl \
    git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install mise
RUN curl https://mise.jdx.dev/install.sh | sh

# Add mise to PATH for the 'vscode' user
USER vscode
ENV PATH="/home/vscode/.local/bin:${PATH}"

# Pre-install tools from mise.toml
COPY mise.toml /tmp/mise.toml
RUN cd /tmp && mise install && rm /tmp/mise.toml
