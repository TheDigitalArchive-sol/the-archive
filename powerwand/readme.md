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
