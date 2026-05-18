# Scanner Import Beta

This document defines the shared scanner-import path for SARIF and structured
scanner feeds. Imported findings can populate Problems, review packets, CLI
JSON, support exports, and release evidence, but they remain imported evidence
until a compatible local confirmation or trusted rerun is linked.

## Contracts

- [`/crates/aureline-runtime/src/scanner_import/`](../../crates/aureline-runtime/src/scanner_import/)
  owns normalization, delta generation, Problems rows, CLI projection, support
  export, and release packet construction.
- [`/schemas/quality/scanner_import_session.schema.json`](../../schemas/quality/scanner_import_session.schema.json)
  defines scanner import sessions plus CLI, support, and release projections.
- [`/schemas/quality/diagnostic_delta_packet.schema.json`](../../schemas/quality/diagnostic_delta_packet.schema.json)
  defines `new`, `resolved`, `persisting`, `suppressed`, `waived`, and
  `unmapped` delta state.
- [`/schemas/quality/review_quality_packet.schema.json`](../../schemas/quality/review_quality_packet.schema.json)
  defines diff-scoped review quality packets with imported/local labels,
  local-confirmation actions, fidelity labels, and raw-payload backlink policy.
- [`/fixtures/quality/scanner_import_beta/`](../../fixtures/quality/scanner_import_beta/)
  contains the structured scanner fixture; the SARIF fixture remains in
  [`/fixtures/quality/sarif_alpha/`](../../fixtures/quality/sarif_alpha/).

## Truth Rules

- Imported findings are always read-only. A local confirmation adds a distinct
  confirmation ref; it does not rewrite the original imported evidence chain.
- Raw scanner bodies and source text are not embedded in ordinary packets.
  Packets carry opaque payload refs unless policy marks the backlink redacted
  or omitted.
- Stale imports, contextual remaps, unmapped anchors, rule-pack/profile
  mismatches, and redacted payload backlinks are explicit fidelity states.
- Delta claims are valid only when baseline family, rule pack, profile/tool,
  and mapping quality are compatible. Otherwise packets preserve the evidence
  but block exact-delta and live-local parity claims.
- Release packets include baseline/delta counts, active release-visible debt,
  imported/local confirmation counts, raw-payload backlink policy, and a parity
  note suitable for release review without copying raw scanner payloads.

## Verification

Run:

```sh
cargo test -p aureline-runtime --test scanner_import_beta
cargo test -p aureline-shell --test scanner_import_alpha
```
