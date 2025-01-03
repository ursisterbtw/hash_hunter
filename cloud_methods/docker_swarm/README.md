# Swarming

## Setup

```bash
docker build -t hash_hunter:latest .
docker build -t hash_hunter_py:latest -f Dockerfile.python .
```

## Run

```bash
cd swarming

chmod +x scripts/swarm-init.sh
./scripts/swarm-init.sh
```
