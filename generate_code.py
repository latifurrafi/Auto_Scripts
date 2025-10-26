import os
from datetime import datetime
from dotenv import load_dotenv
import google.generativeai as genai

# Load environment variables
load_dotenv()

# Get API key
api_key = os.getenv("GEMINI_API_KEY")
if not api_key:
    raise ValueError("Missing GEMINI_API_KEY environment variable. Please set it in GitHub secrets or .env file")

# Configure Gemini
genai.configure(api_key=api_key)
model = genai.GenerativeModel("gemini-2.0-flash")

# Define the languages you want to generate
languages = {
    "python": {
        "extension": "py",
        "prompt": "Write a small, creative, and useful Python script that teaches a programming concept or does something fun."
    },
    "go": {
        "extension": "go",
        "prompt": "Write an innovative and short Go (Golang) program that demonstrates an interesting programming idea."
    },
    "rust": {
        "extension": "rs",
        "prompt": "Write a short, unique, and clever Rust program that showcases an interesting feature of the language."
    }
}

# Create a folder for today’s outputs
date_folder = datetime.now().strftime("%Y-%m-%d")
os.makedirs(date_folder, exist_ok=True)

# Generate and save code for each language
for lang, info in languages.items():
    print(f"⚙️ Generating {lang.capitalize()} code...")
    response = model.generate_content(info["prompt"])

    filename = f"{lang}_{date_folder}.{info['extension']}"
    filepath = os.path.join(date_folder, filename)

    with open(filepath, "w") as f:
        f.write(response.text.strip())

    print(f"✅ Saved: {filepath}")

print("\n🎉 All codes generated and saved successfully!")
