"""
License Manager for DocLens Commercial Edition
Handles license validation, activation, and trial management
"""

import json
import hashlib
import datetime
import os
import sys
from pathlib import Path
from typing import Optional, Dict, Any
import requests


class LicenseManager:
    """Manages software licensing for commercial edition"""
    
    def __init__(self, config_path: str = "build_config.json"):
        self.config_path = config_path
        self.config = self._load_config()
        
        # Get application directory
        if getattr(sys, 'frozen', False):
            # Running as compiled executable
            app_dir = Path(sys.executable).parent
        else:
            # Running as script
            app_dir = Path(__file__).parent
        
        # Create config folder if not exists
        config_dir = app_dir / "config"
        config_dir.mkdir(exist_ok=True)
        
        self.license_file = config_dir / "license.json"
        
    def _load_config(self) -> Dict[str, Any]:
        """Load build configuration"""
        if os.path.exists(self.config_path):
            with open(self.config_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        return {"edition": "free"}
    
    def is_commercial_edition(self) -> bool:
        """Check if this is commercial edition"""
        return self.config.get("edition") == "commercial"
    
    def is_trial_active(self) -> bool:
        """Check if trial period is still active"""
        if not self.is_commercial_edition():
            return False
            
        license_data = self._load_license()
        if not license_data:
            return False
            
        if license_data.get("type") == "full":
            return False
            
        install_date = datetime.datetime.fromisoformat(license_data.get("install_date", ""))
        trial_days = self.config.get("features", {}).get("trial_days", 30)
        expiry_date = install_date + datetime.timedelta(days=trial_days)
        
        return datetime.datetime.now() < expiry_date
    
    def get_trial_days_remaining(self) -> int:
        """Get remaining trial days"""
        if not self.is_commercial_edition():
            return 0
            
        license_data = self._load_license()
        if not license_data or license_data.get("type") == "full":
            return 0
            
        install_date = datetime.datetime.fromisoformat(license_data.get("install_date", ""))
        trial_days = self.config.get("features", {}).get("trial_days", 30)
        expiry_date = install_date + datetime.timedelta(days=trial_days)
        
        remaining = (expiry_date - datetime.datetime.now()).days
        return max(0, remaining)
    
    def is_licensed(self) -> bool:
        """Check if software is properly licensed"""
        if not self.is_commercial_edition():
            return True  # Free edition is always licensed
            
        license_data = self._load_license()
        if not license_data:
            return False
            
        # Check if full license
        if license_data.get("type") == "full":
            return self._validate_license_key(license_data.get("key", ""))
            
        # Check if trial is active
        return self.is_trial_active()
    
    def activate_license(self, license_key: str, email: str = "") -> bool:
        """Activate software with license key"""
        if not self.is_commercial_edition():
            return True
            
        # Validate license key format
        if not self._validate_license_key(license_key):
            return False
            
        # Online activation (if license server configured)
        license_server = self.config.get("license_server")
        if license_server:
            if not self._activate_online(license_key, email, license_server):
                return False
        
        # Save license
        license_data = {
            "type": "full",
            "key": license_key,
            "email": email,
            "activation_date": datetime.datetime.now().isoformat(),
            "product": self.config.get("product", "DocLens Pro"),
            "version": self.config.get("version", "1.0.0")
        }
        
        self._save_license(license_data)
        return True
    
    def start_trial(self) -> bool:
        """Start trial period"""
        if not self.is_commercial_edition():
            return False
            
        if self.license_file.exists():
            return False  # Trial already started
            
        license_data = {
            "type": "trial",
            "install_date": datetime.datetime.now().isoformat(),
            "product": self.config.get("product", "DocLens Pro"),
            "version": self.config.get("version", "1.0.0")
        }
        
        self._save_license(license_data)
        return True
    
    def _validate_license_key(self, key: str) -> bool:
        """Validate license key format and checksum"""
        if not key or len(key) < 20:
            return False
            
        # Simple validation - implement your own algorithm
        parts = key.split("-")
        if len(parts) != 5:
            return False
            
        # Verify checksum (last part)
        data = "-".join(parts[:-1])
        expected_checksum = hashlib.md5(data.encode()).hexdigest()[:4].upper()
        
        return parts[-1] == expected_checksum
    
    def _activate_online(self, license_key: str, email: str, server_url: str) -> bool:
        """Activate license online with license server"""
        try:
            response = requests.post(
                f"{server_url}/activate",
                json={
                    "license_key": license_key,
                    "email": email,
                    "product": self.config.get("product"),
                    "version": self.config.get("version"),
                    "machine_id": self._get_machine_id()
                },
                timeout=10
            )
            return response.status_code == 200
        except Exception:
            # Offline activation fallback
            return True
    
    def _get_machine_id(self) -> str:
        """Get unique machine identifier"""
        import platform
        import uuid
        
        machine_info = f"{platform.node()}-{uuid.getnode()}"
        return hashlib.sha256(machine_info.encode()).hexdigest()
    
    def _load_license(self) -> Optional[Dict[str, Any]]:
        """Load license data from file"""
        if not self.license_file.exists():
            return None
            
        try:
            with open(self.license_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        except Exception:
            return None
    
    def _save_license(self, license_data: Dict[str, Any]) -> None:
        """Save license data to file"""
        with open(self.license_file, 'w', encoding='utf-8') as f:
            json.dump(license_data, f, indent=2)
    
    def get_license_info(self) -> Dict[str, Any]:
        """Get current license information"""
        if not self.is_commercial_edition():
            return {
                "edition": "free",
                "license_type": "MIT",
                "status": "active"
            }
            
        license_data = self._load_license()
        if not license_data:
            return {
                "edition": "commercial",
                "license_type": "trial",
                "status": "not_activated",
                "trial_available": True
            }
            
        if license_data.get("type") == "trial":
            return {
                "edition": "commercial",
                "license_type": "trial",
                "status": "trial",
                "days_remaining": self.get_trial_days_remaining(),
                "is_active": self.is_trial_active()
            }
        
        return {
            "edition": "commercial",
            "license_type": "full",
            "status": "active",
            "email": license_data.get("email", ""),
            "activation_date": license_data.get("activation_date", "")
        }


def generate_license_key(customer_id: str, product_code: str = "DOCL") -> str:
    """
    Generate a license key for DocLens commercial edition
    Format: XXXX-XXXX-XXXX-XXXX-XXXX
    """
    import random
    import string
    
    # Part 1: Product code
    part1 = product_code.upper().ljust(4, '0')
    
    # Part 2: Customer ID hash
    customer_hash = hashlib.md5(customer_id.encode()).hexdigest()[:4].upper()
    part2 = customer_hash
    
    # Part 3: Random
    part3 = ''.join(random.choices(string.ascii_uppercase + string.digits, k=4))
    
    # Part 4: Timestamp
    timestamp = str(int(datetime.datetime.now().timestamp()))[-4:]
    part4 = timestamp
    
    # Part 5: Checksum
    data = f"{part1}-{part2}-{part3}-{part4}"
    checksum = hashlib.md5(data.encode()).hexdigest()[:4].upper()
    part5 = checksum
    
    return f"{part1}-{part2}-{part3}-{part4}-{part5}"


if __name__ == "__main__":
    # Example usage
    print("License Key Generator")
    print("=" * 50)
    
    customer_email = input("Customer Email: ")
    license_key = generate_license_key(customer_email)
    
    print(f"\nGenerated License Key:")
    print(f"  {license_key}")
    print(f"\nCustomer: {customer_email}")
    print("\nThis key can be used to activate the commercial edition.")
