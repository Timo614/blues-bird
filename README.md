## Blues bird project

This server was created as a simple demonstration of an Axum server running on Shuttle.rs. It's used for my birds project on Hackster.io. More information can be seen there: https://www.hackster.io/projects/b8b705

## Generating an API Key

Run the following to generate an API key:
```
python3 -c 'import secrets, string; print("API Secret:", "".join(secrets.choice(string.ascii_letters + string.digits + string.punctuation) for _ in range(64)))'
```

Create a file Secrets.toml with the following content:
```
API_KEY = "YOUR API SECRET HERE"
```

This file is ignored from git via the `.gitignore` file in the repository.
