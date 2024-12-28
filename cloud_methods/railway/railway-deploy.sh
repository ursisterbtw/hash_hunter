#!/bin/bash

# Install Railway CLI if not present
if ! command -v railway &> /dev/null; then
    curl -fsSL https://railway.app/install.sh | sh
fi

# Login to Railway (if not already logged in)
railway login

# Initialize Railway project
railway init hash-hunter

# Link to existing project
railway link

# Deploy all services
railway up

# Monitor deployments
railway status