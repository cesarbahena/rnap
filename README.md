# RNAP - Requirements Normalization and Assessment Platform

A human-in-the-loop system that transforms software requirements into unambiguous, implementation-ready artifacts. Uses mutation-based state tracking where every decision is explicitly recorded, traceable, and verifiable before development begins.

## Core Concepts

RNAP uses biological metaphors for its data model:

| Concept | Description |
|---------|-------------|
| **Genome** | Tenant boundary |
| **DNA** | Structured requirements (product intent) |
| **Genotype** | Schema definition (document type) |
| **Gene** | Spec document instance |
| **Mutation** | Append-only change to a trait |
| **Chromosome** | Domain node in the graph |
| **Quiasma** | Relationship between domain nodes |
| **Organism** | Actor (human, team, service) |
| **mRNA** | Frozen snapshot of pending mutations |
| **tRNA** | Mutable tasklist |
| **Ribosome** | QA pipeline |
| **Phenotype** | Implementation snapshot |
| **Protein** | Evaluation result (pass/fail) |
| **sRNA** | Atomic learnings |

## Architecture

```
DNA (what we want)
  → Gene (spec doc)
    → Mutations (changes to traits)
      → mRNA (frozen pending mutations)
        → tRNA (tasklist)
          → Phenotype (implementation snapshot)
            → Protein (evaluation)
              → sRNA (learnings)
```

## Crate Structure

```
crates/
├── rnap-genome/      # Tenant boundary
├── rnap-genotype/    # Schema definitions
├── rnap-gene/        # Spec documents
├── rnap-chromosome/  # Domain nodes
├── rnap-organism/    # Actors
├── rnap-quiasma/     # Relationships
├── rnap-locus/       # View projections
├── rnap-dna/         # Requirements
├── rnap-chromatine/  # Document references
├── rnap-histone/     # Architecture decisions
├── rnap-mrna/        # Frozen snapshots
├── rnap-trna/        # Tasklists
├── rnap-srna/        # Learnings
├── rnap-ribosome/    # QA pipelines
├── rnap-phenome/     # QA profiles
├── rnap-phenotype/   # Implementation snapshots
├── rnap-storage/      # Repository implementations
└── rnap-cli/         # Command-line interface
```

## Prerequisites

- Rust (latest stable)
- Docker / Docker Compose
- PostgreSQL 17+

## Setup

### 1. Start Database

```bash
docker compose up -d
```

### 2. Run Migrations

```bash
cargo sqlx migrate run
```

### 3. Seed Initial Data

```bash
cargo run -- seed
```

### 4. Build

```bash
cargo build
```

## CLI Commands

### Create a Gene

```bash
cargo run -- create FEAT "user authentication"
# Output: Created gene: FEAT-0001-user-authentication
```

### Transcribe (View State)

```bash
cargo run -- transcribe FEAT-0001-user-authentication
```

### Mutate (Add Changes)

```bash
cargo run -- mutate FEAT-0001-user-authentication title="Login flow" "initial requirement"
```

## Development

### Run Tests

```bash
cargo test
```

### Database Reset

```bash
docker compose down -v      # Destroy volume
docker compose up -d        # Recreate
cargo sqlx migrate run      # Reapply migrations
cargo run -- seed          # Reseed data
```

## Database Conventions

- **Domain-first, schema-later**: Build Rust structs in domain crates, persist when tests demand it
- **Sequential migrations**: Use `0001_`, `0002_`, etc. in `migrations/` folder
- **No auto-timestamps**: Explicit `created_at` columns managed by the application
- **One table per migration**: Clean, focused migrations
- **Seeds separate**: Data seeding happens in `seeds/` folder, not migrations

## License

MIT
