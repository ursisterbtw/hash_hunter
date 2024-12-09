FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Set non-interactive mode for APT
ENV DEBIAN_FRONTEND=noninteractive

# Install necessary packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    libssl-dev \
    pkg-config \
    lldb \
    python3-pip \
    python3-venv \
    libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Reset DEBIAN_FRONTEND if needed
ENV DEBIAN_FRONTEND=dialog

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}"
ENV PYTHONPATH="${PYTHONPATH}:/workspace"
