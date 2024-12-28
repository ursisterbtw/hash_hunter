import asyncio
from dataclasses import dataclass
import json
import os
import aiohttp
import requests
import smtplib
import redis
from loguru import logger
from email.mime.text import MIMEText
from typing import Optional, Dict

from datetime import datetime
import aiofiles


@dataclass
class WalletResult:
    address: str
    private_key: str
    attempts: int
    rarity_score: float
    pattern_matched: str

class NotificationPipeline:
    def __init__(self, config_path: str = "config/notifications.json"):
        self.config = self._load_config(config_path)
        self.redis_client = redis.Redis(
            host=self.config.get("redis_host", "localhost"),
            port=self.config.get("redis_port", 6379),
            decode_responses=True
        )
        
    def _load_config(self, config_path: str) -> Dict:
        try:
            with open(config_path) as f:
                return json.load(f)
        except FileNotFoundError:
            logger.warning(f"Config file not found at {config_path}, using defaults")
            return {}

    async def notify(self, result: WalletResult):
        """Orchestrate all notifications"""
        try:
            # Store result first
            await self._store_result(result)
            
            # Send notifications in parallel
            tasks = [
                self._send_discord_webhook(result),
                self._send_telegram_alert(result),
                self._send_email_alert(result)
            ]
            
            await asyncio.gather(*tasks)
            
        except Exception as e:
            logger.error(f"Notification pipeline error: {e}")

    async def _store_result(self, result: WalletResult):
        """Store result in Redis and filesystem"""
        # Redis storage for quick lookup
        key = f"wallet:{result.address}"
        self.redis_client.hset(key, mapping={
            "address": result.address,
            "attempts": result.attempts,
            "rarity_score": result.rarity_score,
            "pattern_matched": result.pattern_matched,
            "timestamp": datetime.now().isoformat()
        })

        # Filesystem storage for persistence
        await self._save_to_file(result)

    async def _save_to_file(self, result: WalletResult):
        """Save wallet info to filesystem with consistent structure"""
        base_dir = "gen/wallets"
        os.makedirs(base_dir, exist_ok=True)
        
        # Create date-based directory structure
        date_dir = datetime.now().strftime("%Y/%m/%d")
        full_dir = f"{base_dir}/{date_dir}"
        os.makedirs(full_dir, exist_ok=True)
        
        filename = f"{full_dir}/{result.address}.json"
        
        data = {
            "address": result.address,
            "private_key": result.private_key,
            "attempts": result.attempts,
            "rarity_score": result.rarity_score,
            "pattern_matched": result.pattern_matched,
            "timestamp": datetime.now().isoformat(),
            "metadata": {
                "node_id": os.getenv("NODE_ID", "unknown"),
                "instance_type": os.getenv("INSTANCE_TYPE", "unknown")
            }
        }
        
        async with aiofiles.open(filename, 'w') as f:
            await f.write(json.dumps(data, indent=2))

    async def _send_discord_webhook(self, result: WalletResult):
        """Send notification to Discord"""
        if webhook_url := self.config.get("discord_webhook"):
            embed = {
                "title": "🎯 New Vanity Address Found!",
                "description": f"Pattern: {result.pattern_matched}",
                "fields": [
                    {"name": "Address", "value": f"`{result.address}`"},
                    {"name": "Attempts", "value": str(result.attempts)},
                    {"name": "Rarity Score", "value": f"{result.rarity_score:.4f}"}
                ],
                "color": 5814783
            }
            
            await self._make_request(webhook_url, {"embeds": [embed]})

    async def _send_telegram_alert(self, result: WalletResult):
        """Send notification to Telegram"""
        if (bot_token := self.config.get("telegram_bot_token")) and \
           (chat_id := self.config.get("telegram_chat_id")):
            message = (
                f"🎯 New Vanity Address Found!\n\n"
                f"Address: `{result.address}`\n"
                f"Pattern: {result.pattern_matched}\n"
                f"Attempts: {result.attempts:,}\n"
                f"Rarity: {result.rarity_score:.4f}"
            )
            
            url = f"https://api.telegram.org/bot{bot_token}/sendMessage"
            await self._make_request(url, {
                "chat_id": chat_id,
                "text": message,
                "parse_mode": "Markdown"
            })

    async def _make_request(self, url: str, data: Dict):
        """Make HTTP request with retry logic"""
        async with aiohttp.ClientSession() as session:
            for attempt in range(3):
                try:
                    async with session.post(url, json=data) as response:
                        response.raise_for_status()
                        return
                except Exception as e:
                    if attempt == 2:
                        logger.error(f"Request failed after 3 attempts: {e}")
                    await asyncio.sleep(1)