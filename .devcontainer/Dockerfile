FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Install system dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    libssl-dev \
    pkg-config \
    curl \
    git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Switch to vscode user for tool installation
USER vscode
ENV PATH="/home/vscode/.local/bin:/home/vscode/.local/share/mise/shims:${PATH}"

# Install mise
RUN curl https://mise.jdx.dev/install.sh | sh

# Pre-install tools from mise.toml
# Note: We copy the file to a temporary location first
COPY --chown=vscode:vscode mise.toml /tmp/mise.toml
RUN cd /tmp && mise install && rm /tmp/mise.toml
