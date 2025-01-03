#!/bin/bash

# Initialize swarm
docker swarm init

# Deploy the stack
docker stack deploy -c swarm-compose.yml hash_hunter

# Scale services if needed
docker service scale hash_hunter_rust=8
docker service scale hash_hunter_python=4

# Monitor services
#!/bin/bash

# Initialize swarm
docker swarm init

# Deploy the stack
docker stack deploy -c swarm-compose.yml hash_hunter

# Scale services if needed
docker service scale hash_hunter_rust=8
docker service scale hash_hunter_python=4

# Monitor services
watch docker service ls