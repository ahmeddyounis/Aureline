# M5 last-supported-snapshot and archive-export-gate registries

This lane ships last-supported snapshot and retirement archive bundles for a retiring surface across the first
consumer surfaces — release-center, help / docs, support, marketplace / registry, install / update, and
partner / procurement — over the frozen
[M5 retired-state matrix](./m5-retired-state-ops.md), so migration, audit, procurement, and support can inspect what
was retired without keeping the retired surface live forever, rather than destroying its docs, schemas, and evidence
at cutoff. It emits one export-safe *last-supported snapshot* per retiring object — binding the docs / help truth,
schema / contract set, known-limits snapshot, compatibility report, provenance / SBOM reference, and support-article
links for the final supported build or line state to one object identity joined to its exact build — and one typed
*archive-export gate* per object that blocks its archive bundle from being handed off while it carries a live vendor
dependency, would leak a secret or internal-only detail, or is not bound back to the retirement manifest and review
packet. It records the *last-supported-snapshot* grammar (one classified snapshot field per preserved fact —
docs / help truth, schema / contract set, known-limits snapshot, compatibility report, provenance / SBOM reference,
or support-article links — carrying its owning team and joined to the exact build, retirement manifest, and review
packet) and the *archive-export-gate* grammar (the export-safety blocker a bundle is stopped by —
live-dependency-present, internal-only-or-secret-leak, or unbound-manifest-or-review-packet, naming the active block
reason) into registry resolvers that produce export-safe, honest projections, so self-hosted, offline, and
procurement / support consumers open one export-safe historical reference for the same object.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_last_supported_snapshot_and_archive_export_gate_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-last-supported-snapshot-and-archive-export-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-last-supported-snapshot.schema.json`](../../schemas/program/m5-last-supported-snapshot.schema.json)
  (reused from the frozen retired-state matrix — the last-supported snapshot each retiring object is recorded against)
  and
  [`schemas/program/m5-archive-export-gate.schema.json`](../../schemas/program/m5-archive-export-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-last-supported-snapshot-and-archive-export-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  last-supported snapshot / archive-export loop — it demonstrates one snapshot / archive-export-gate loop end to end
  for the first retirement-bearing surfaces.
- **Narrowed fixtures:**
  `fixtures/release/m5-last-supported-snapshot-and-archive-export-gate-registries/`
  (`last_supported_snapshot_beta_narrowed.json`, `archive_export_gate_preview_narrowed.json`).

## Two registries

1. **Last-supported snapshot** (`resolve_last_supported_snapshot_entry`) — captures one snapshot field per retiring
   object: the classification (docs / help truth, schema / contract set, known-limits snapshot, compatibility report,
   provenance / SBOM reference, support-article links) and its canonical mode, the exact-build joins (repo rows,
   bundle IDs, install topology, toolchain envelope), the compatibility / known-limits state, the archival / rollback
   route, and the owning team. A clean entry names a canonical registry token, a classified snapshot field, and a
   retirement role, covers the canonical / accessible / audit resolution forms, publishes a complete object joined to
   its exact build, and keeps a public-facing compatibility / support field matched to the archived successor.
   Otherwise it degrades honestly.
2. **Archive-export gate** (`resolve_archive_export_gate_entry`) — surfaces a bundle's export-safety blockers before it
   can be handed off. A clean entry names a classified archive-export scope (live-dependency-present,
   internal-only-or-secret-leak, or unbound-manifest-or-review-packet) and provides the complete gate object; a gate
   that would hand off a bundle carrying a live dependency, leak a secret / internal-only detail, or drop its binding
   back to the retirement manifest and review packet degrades.

## Acceptance criteria (proven by resolved examples)

- **At least one exact-build last-supported bundle can be produced for a seeded retirement candidate and opened
  without live service dependencies.** Clean snapshot entries cover the canonical docs-help-truth /
  schema-contract-set / known-limits-snapshot / compatibility-report / support-article-links / provenance-SBOM fields
  and the first release-center / help-docs / support / marketplace-registry / install-update surfaces, an
  object-incomplete example degrades, and no clean snapshot entry published an incomplete object.
- **Historical bundles identify the final supported version / channel and the successor path without contradiction.**
  A snapshot whose exact-build joins are not preserved degrades, an unbound example degrades, a clean bounded snapshot
  entry is present, and no clean entry is unbounded or unbound.
- **Retirement archives exclude live secrets / internal-only detail while retaining enough evidence for support,
  audit, and procurement / reference use.** Clean archive-export-gate entries cover the live-dependency-present /
  internal-only-or-secret-leak / unbound-manifest-or-review-packet scopes with full resolution-form coverage while
  providing the complete gate object, and a gate that would hand off an unsafe bundle or drop its manifest binding
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_last_supported_snapshot_and_archive_export_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_last_supported_snapshot_and_archive_export_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_last_supported_snapshot_and_archive_export_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_last_supported_snapshot_and_archive_export_gate_registries -- last-supported-snapshot-table
cargo run -p aureline-ui --example dump_m5_last_supported_snapshot_and_archive_export_gate_registries -- fixture-last-supported-snapshot-beta-narrowed
cargo run -p aureline-ui --example dump_m5_last_supported_snapshot_and_archive_export_gate_registries -- fixture-archive-export-gate-preview-narrowed
```
