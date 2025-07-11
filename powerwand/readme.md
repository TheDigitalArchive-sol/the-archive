# Powerwand Repo contains the POC of how to Publish a book in the archive

## Build Wasm
Run:
```bash
wasm-pack build --release --target web
```

## Build The Release
```bash
cargo make build
```

## Run The Release
```bash
cargo make run
```

## Deploy Programs on Chain
```bash
cargo make book-storage-deploy
```

# How To Test the UI

## Run the Dev-Local UI
```bash
cd hogwarts-lib-rs/app
npm run dev
```

## Wallet Setup
```bash
# Connect the wallet from the UI
# Faucet the Wallet (local-net)
solana airdrop 1000 <address_here>
# Wait for confirmation and check balance
```

