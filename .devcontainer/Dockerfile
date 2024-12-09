FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Set non-interactive mode for APT
ENV DEBIAN_FRONTEND=noninteractive

# Install necessary packages including Docker CLI
# trunk-ignore(hadolint/DL3008)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential=12.9ubuntu3 \
    curl=7.81.0-1ubuntu1.15 \
    git \
    libssl-dev \
    pkg-config \
    lldb \
    python3-pip \
    python3-venv \
    libpq-dev \
    docker.io && \
    rm -rf /var/lib/apt/lists/*

# Reset DEBIAN_FRONTEND
ENV DEBIAN_FRONTEND=dialog

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}"
ENV PYTHONPATH="${PYTHONPATH}:/workspace"

# Create non-root user and add to docker group
RUN useradd -m -s /bin/bash devuser && \
    groupadd docker || true && \
    usermod -aG docker devuser

USER devuser