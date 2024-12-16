# Setup Hash Hunter

```sh
sudo apt-get update
sudo apt-get install -y docker.io
```

```sh
   sudo docker run -d hash_hunter_image
```

## Build and Push Hash Hunter

```sh
  docker build -t gcr.io/[PROJECT-ID]/hash-hunter .
  docker push gcr.io/[PROJECT-ID]/hash-hunter
```

## Automate with Cloud Run

Alternatively, use Cloud Run to deploy containers:

```sh
  gcloud run deploy hash-hunter \
    --image gcr.io/[PROJECT-ID]/hash-hunter \
    --platform managed \
    --region us-central1 \
    --allow-unauthenticated
```

## Leveraging Oracle Cloud Free Tier

Set Up Oracle Cloud
Offerings: Up to 4 CPUs and 24GB RAM.
Setup: Oracle Free Tier
Deployment Steps
Create a Compute Instance:
Select Ampere ARM instance for better performance.

Install Docker and Run Container:

```sh
sudo yum update -y
sudo yum install -y docker
sudo systemctl start docker
sudo docker run -d gcr.io/[PROJECT-ID]/hash-hunter
```

## Using AWS Free Tier

Set Up AWS
Offerings: t2.micro instance with 750 hours per month.
Setup: AWS Free Tier
Deployment Steps
Launch an EC2 Instance:
Choose Amazon Linux 2 AMI.

Launch Docker Container:

## Implementing Redis for Coordination

If you need to coordinate tasks between workers:
Set Up Redis
Use Redis Labs: Offers a free Redis database.
Install Redis: Alternatively, install on one of your instances.
Update Your Application
Modify main.rs to report progress or coordinate via Redis:

```
// Add Redis dependency
[dependencies]
redis = "0.21.5"

// In your code
let client = redis::Client::open("redis://your_redis_server/")?;
let mut con = client.get_async_connection().await?;
redis::cmd("INCR").arg("total_attempts").query_async(&mut con).await?;
```

## Scaling with Docker Swarm or Kubernetes

For more advanced orchestration:
Docker Swarm: Simple and good for small clusters.
Kubernetes: More complex but highly scalable.
Example with Docker Swarm
Initialize Swarm:

```sh
   docker swarm init
```

Deploy Stack:

```yaml
   # docker-compose.yml
   version: '3'
   services:
     hash_hunter:
       image: hash_hunter_image
       deploy:
         replicas: 5
         resources:
           limits:
             cpus: '0.5'
             memory: 1G
```

## Sample Code Adjustments

Modify main.rs to Support Distributed Environment
Add command-line arguments to specify Redis server details:

```rs
use clap::Parser;

#[derive(Parser)]
struct Args {
    // Existing arguments
    // ...
    #[clap(long)]
    redis_url: Option<String>,
}
```
