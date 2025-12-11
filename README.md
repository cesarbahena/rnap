# RNAP

Mutation-sourced, genotype-driven SDLC system where DNA captures intent, genes are specs, chromosomes model domains, ribosomes evaluate, and proteins validate.

## Quick Start

```bash
docker compose up -d
cargo sqlx migrate run
cargo run -- seed
cargo run -- create FEAT "feature name"
cargo run -- transcribe FEAT-0001-feature-name
cargo run -- mutate FEAT-0001-feature-name title="value" "context"
```

## Requirements

- Rust (latest stable)
- PostgreSQL 17+
- Docker

## Commands

- `cargo run -- seed` - Seed initial data
- `cargo run -- create <kind> <name>` - Create a gene
- `cargo run -- transcribe <gene>` - View gene state
- `cargo run -- mutate <gene> key=value "context"` - Add mutation

## Development

```bash
cargo test
```

## Architecture

Domain crates in `crates/`, storage in `rnap-storage`, CLI in `rnap-cli`.
