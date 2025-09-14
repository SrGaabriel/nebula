import os
from dotenv import load_dotenv
import subprocess
import sys

load_dotenv()

database_host = os.getenv("DB_HOST")
database_port = os.getenv("DB_PORT")
database_user = os.getenv("DB_USER")
database_password = os.getenv("DB_PASSWORD")
database_name = os.getenv("DB_NAME")

if not all([database_host, database_port, database_user, database_password, database_name]):
    print("Error: One or more required environment variables are missing.")
    sys.exit(1)

database_url = f"postgres://{database_user}:{database_password}@{database_host}:{database_port}/{database_name}"

command = ["sea-orm-cli", "migrate"] + sys.argv[1:] + ["-u", database_url]
print("Running command:", " ".join(command))
subprocess.run(command)
sys.exit(0)