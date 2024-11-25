FROM mcr.microsoft.com/devcontainers/base:ubuntu

ENV DEBIAN_FRONTEND=noninteractive

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

ENV DEBIAN_FRONTEND=dialog

ENV PATH="/root/.cargo/bin:${PATH}"
ENV PYTHONPATH="${PYTHONPATH}:/workspace"
