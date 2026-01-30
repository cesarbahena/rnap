# Architecture

DNAp currently has a small backend-first Rust shape:

- `src/app.rs`: domain/application model and behavior for Insulator, Genome, Tf, GeneFamily, Allele mutation, transcription, splicing, and Transcriptome cursor metadata.
- `src/session.rs`: replaceable session boundary plus local user-level state storage for development workflows.
- `src/cli.rs`: thin `dna` command parser/dispatcher. Dynamic Sequence flags are parsed here and sent to application behavior as structured mutations.
- `src/main.rs`: binary entry point only.
- `src/lib.rs`: module boundary and application re-exports.

Current runtime boundaries:

- Tenant identity: `Insulator`.
- Project scope: `Genome`.
- Actor identity: `Tf`.
- Tenant content: GeneFamily, Locus, Allele, Mutation, Transcriptome cursor, Exon.
- Local CLI session: actor and scope only; no real auth secrets.
- Authorization: not implemented yet; reserved for Histone/HistoneMark slices.
- Persistence: local JSON state adapter for CLI workflow testing; not product storage architecture.

`dna epigenetics` is the current superadmin bootstrap namespace. Future real login/auth should replace the session provider, not the normal workflow commands.
