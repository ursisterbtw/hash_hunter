FROM mcr.microsoft.com/devcontainers/base:ubuntu

# add healthcheck
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

# create non-root user
RUN useradd -m -s /bin/bash devuser
USER devuser

# Set non-interactive mode for APT
ENV DEBIAN_FRONTEND=noninteractive

# Install necessary packages
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
    libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Reset DEBIAN_FRONTEND if needed
ENV DEBIAN_FRONTEND=dialog

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}"
ENV PYTHONPATH="${PYTHONPATH}:/workspace"
