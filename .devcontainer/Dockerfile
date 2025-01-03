FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Include lld linker to improve build times
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y

# Set DOCKER_BUILDKIT=1 to ensure proper output handling
ENV DOCKER_BUILDKIT=1

# Add to shell rc files to ensure the env var is always available
RUN echo "export DOCKER_BUILDKIT=1" >> /etc/bash.bashrc && \
    echo "export DOCKER_BUILDKIT=1" >> ~/.bashrc
