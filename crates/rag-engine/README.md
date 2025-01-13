## Running with Mirrord

[Mirrord](https://mirrord.dev)

```sh
curl -fsSL https://raw.githubusercontent.com/metalbear-co/mirrord/main/scripts/install.sh | bash
```

```sh
mirrord exec --steal -n bionic-gpt --target deployment/bionic-rag-engine cargo run -- --bin rag-engine 
```