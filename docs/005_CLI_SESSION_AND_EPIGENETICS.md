# 005 CLI Session And Epigenetics Bootstrap

## Capability

Run local CLI workflows through a replaceable session boundary and a temporary superadmin bootstrap namespace.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

This slice implements:

- `dna` binary entry point.
- `dna epigenetics` bootstrap namespace.
- Local user-level state store.
- Local session record for current actor and scope.
- Thin CLI calls into backend/application behavior.

## Behavior

- `dna epigenetics` is the temporary superadmin/control-plane namespace.
- `dna epigenetics init-insulator` provisions an Insulator and placement.
- `dna epigenetics init-genome` creates a Genome inside an Insulator.
- `dna epigenetics init-tf` creates a local Tf actor inside an Insulator.
- `dna epigenetics use` writes the active local session.
- The local session stores actor and scope: Tf, Insulator, and Genome.
- The local session contains no real auth secret.
- Future real login/auth should replace the session provider, not normal workflow command behavior.
- Normal workflow commands read the active session:
  - `dna mutate`
  - `dna transcribe`
  - `dna splice`
- CLI parsing stays thin over backend behavior.
- Dynamic Sequence flags are parsed by the CLI and passed as structured Sequence mutations.
- CLI state is user-level state, not project repository state.

## CLI

Bootstrap local context:

```sh
dna epigenetics init-insulator Acme --region us-east-1
dna epigenetics init-genome Billing --insulator Acme
dna epigenetics init-tf Cesar --insulator Acme
dna epigenetics use --insulator Acme --genome Billing --tf Cesar
```

Define a local work type for workflow testing:

```sh
dna epigenetics define-family FRS FeatureRequirements --artifact mRNA --sequence Summary --sequence Risk
```

Run normal workflow commands:

```sh
dna mutate --new FRS Checkout --summary Draft --risk Unknown
dna transcribe FRS-checkout
dna splice FRS-checkout BuildCheckout
dna mutate FRS-checkout --summary UpdatedDraft
dna transcribe FRS-checkout
dna splice FRS-checkout --lgtm
```

## Implementation Contract

- Keep domain/application behavior in `src/app.rs`.
- Keep local session/state in `src/session.rs`.
- Keep CLI parsing and command dispatch in `src/cli.rs`.
- Keep `src/lib.rs` as a module boundary.
- Keep `src/main.rs` as the binary entry point only.
- Do not add real auth, JWT, SSO, or token storage in this slice.

## Approved Tests

- Epigenetics bootstrap can create Insulator, Genome, Tf, active session, and GeneFamily.
- Normal CLI workflow commands use the active session.
- CLI workflow can mutate, transcribe, and splice an Allele.
- `dna splice --lgtm` expresses current `Unexpressed` Mutations without changing Exons.
- CLI transcribe output includes approval comments.
