#!/usr/bin/env python3
import asyncio
import json
import os
import subprocess
from dataclasses import dataclass
from typing import List, Optional

import boto3
import oci
from google.cloud import run_v2
from loguru import logger


@dataclass
class CloudConfig:
    provider: str
    region: str
    instance_type: str
    cpu_count: int
    memory_gb: float
    
class CloudOrchestrator:
    def __init__(self):
        self.configs = self._load_cloud_configs()
        self.docker_image = "hash_hunter:latest"
        self.total_instances = 0
        
    def _load_cloud_configs(self) -> List[CloudConfig]:
        return [
            # Oracle Cloud Free Tier
            CloudConfig("oracle", "us-phoenix-1", "VM.Standard.A1.Flex", 4, 24),
            # AWS Free Tier
            CloudConfig("aws", "us-east-1", "t2.micro", 1, 1),
            # GCP Cloud Run
            CloudConfig("gcp", "us-central1", "cloud-run", 2, 4),
        ]

    async def deploy_oracle_cloud(self, config: CloudConfig) -> None:
        logger.info("Deploying to Oracle Cloud...")
        # Reference setup steps from .github/workflows/setup.md lines 31-47
        try:
            # Initialize Oracle Cloud SDK
            oci_config = oci.config.from_file()
            compute_client = oci.core.ComputeClient(oci_config)
            
            # Launch instance with Docker pre-installed
            instance_details = {
                "compartmentId": os.getenv("OCI_COMPARTMENT_ID"),
                "displayName": f"hash_hunter_{self.total_instances}",
                "availabilityDomain": "AD-1",
                "shape": config.instance_type,
                "metadata": {
                    "user_data": self._get_cloud_init_script(),
                }
            }
            
            compute_client.launch_instance(instance_details)
            self.total_instances += 1
            
        except Exception as e:
            logger.error(f"Oracle Cloud deployment failed: {e}")

    def _get_cloud_init_script(self) -> str:
        # Reference Docker setup from Dockerfile lines 1-20
        return """#!/bin/bash
        yum update -y
        yum install -y docker
        systemctl start docker
        docker pull {self.docker_image}
        docker run -d {self.docker_image} -p 000000000000 -c -s 100000 -m 100000000000 -i 10000 -y
        """

    async def deploy_aws(self, config: CloudConfig) -> None:
        logger.info("Deploying to AWS...")
        try:
            ec2 = boto3.client('ec2')
            
            # Launch t2.micro instance
            response = ec2.run_instances(
                ImageId='ami-0c55b159cbfafe1f0',  # Amazon Linux 2
                InstanceType=config.instance_type,
                MinCount=1,
                MaxCount=1,
                UserData=self._get_cloud_init_script(),
                TagSpecifications=[{
                    'ResourceType': 'instance',
                    'Tags': [{'Key': 'Name', 'Value': f'hash_hunter_{self.total_instances}'}]
                }]
            )
            self.total_instances += 1
            
        except Exception as e:
            logger.error(f"AWS deployment failed: {e}")

    async def deploy_swarm(self) -> None:
        logger.info("Deploying local Docker Swarm...")
        try:
            # Reference swarm setup from swarming/swarm-init.sh lines 1-27
            commands = [
                "docker swarm init",
                "docker stack deploy -c swarm-compose.yml hash_hunter",
                "docker service scale hash_hunter_rust=8",
                "docker service scale hash_hunter_python=4"
            ]
            
            for cmd in commands:
                subprocess.run(cmd.split(), check=True)
                
        except Exception as e:
            logger.error(f"Swarm deployment failed: {e}")

    async def deploy_cloud_run(self, config: CloudConfig) -> None:
        logger.info("Deploying to Cloud Run...")
        try:
            # Reference Cloud Run setup from .github/workflows/setup.md lines 19-29
            client = run_v2.ServicesClient()
            
            service = {
                "template": {
                    "containers": [{
                        "image": f"gcr.io/{os.getenv('GCP_PROJECT')}/hash-hunter",
                        "resources": {
                            "limits": {
                                "cpu": str(config.cpu_count),
                                "memory": f"{config.memory_gb}Gi"
                            }
                        }
                    }]
                }
            }
            
            client.create_service(
                parent=f"projects/{os.getenv('GCP_PROJECT')}/locations/{config.region}",
                service=service,
                service_id=f"hash-hunter-{self.total_instances}"
            )
            self.total_instances += 1
            
        except Exception as e:
            logger.error(f"Cloud Run deployment failed: {e}")

    async def orchestrate(self) -> None:
        logger.info("Starting cloud orchestration...")
        
        tasks = []
        for config in self.configs:
            if config.provider == "oracle":
                tasks.append(self.deploy_oracle_cloud(config))
            elif config.provider == "aws":
                tasks.append(self.deploy_aws(config))
            elif config.provider == "gcp":
                tasks.append(self.deploy_cloud_run(config))
        
        # Add local swarm deployment
        tasks.append(self.deploy_swarm())
        
        await asyncio.gather(*tasks)
        logger.success(f"Deployed {self.total_instances} instances across cloud providers")

if __name__ == "__main__":
    orchestrator = CloudOrchestrator()
    asyncio.run(orchestrator.orchestrate())