# Powerwand Repo contains the POC of how to Publish a book in the archive

## Setup the environment
Edit the env.example file and run the following command
```bash
cp env.example .env
```

## Deploy Solana Programs
```bash
# This will update the pubkeys in .env
cargo make book-storage-deploy
```
