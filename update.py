from datetime import datetime

with open("log.txt", "a") as f:
    f.write(f"Updated on {datetime.now()}\n")
