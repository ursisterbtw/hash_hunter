#!/bin/bash

# Install Railway CLI if not present
if ! command -v railway &> /dev/null; then
    curl -fsSL https://railway.app/install.sh | sh
fi

# Login to Railway (if not already logged in)
railway login

# Initialize Railway project if not already initialized
if [ ! -f railway.toml ]; then
    railway init hash-hunter
fi

# Link to existing project
railway link

# Deploy all services
railway up --detach

# Monitor deployments
railway status

# Watch logs
railway logs