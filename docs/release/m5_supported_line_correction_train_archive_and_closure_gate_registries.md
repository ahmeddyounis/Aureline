# M5 correction-train-archive and closure-gate registries

This lane makes every correction line auditable end to end so release, support, and procurement readers can see
what changed, why, and how it was recovered, over the frozen
[M5 supported-line-transparency matrix](./m5-supported-line-transparency-ops.md). It archives one
*correction-train archive* record per shipped correction packet on each active supported line — a hotfix packet, a
backport packet, a rollback outcome, an advisory publication, a public-communication bundle, or a revocation
record — so partner reviews, procurement checks, support escalations, and OSS stewardship inherit durable
correction lineage rather than scattered internal notes, and emits a typed *closure gate* event whenever a line's
correction archive has a coverage gap — instead of letting a shipped correction packet close with no archive record
or a broken exact-build join. It records the *correction-train-archive* grammar (one typed archive record per
shipped correction packet, tracked against exact build / release-line identity, each bound to one supported-line
identity with its bug-ID / defect-ledger / release-artifact-graph joins and the public-claim or support-window state
the correction affected, and public-safe advisory and public-communication history separated from internal-only
hotfix / backport / rollback / revocation incident payloads) and the *closure-gate* grammar (the archive-coverage
gap a correction line's closure is blocked on — missing archive coverage, a broken exact-build join, or an
untraceable correction line) into registry resolvers that produce export-safe, honest projections, so
release / help, docs, support, and procurement surfaces resolve one canonical, freshness-checked truth instead of
re-synthesizing correction truth by hand. The archive record and the closure gate are separated in runtime and
serialized state: the corrective action class, its exact-build provenance, the public communication state, the
rollback outcome, and the linked supported-line-matrix / active-claim / defect-ledger / release-artifact refs live
on the correction-train archive, while the resolved line identity, affected archive-record reference,
archived-versus-active-build reference, gate-scope state, and active gate reason live on the closure-gate event, and
missing archive coverage or a broken exact-build join blocks correction-line closure until it is fixed.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_supported_line_correction_train_archive_and_closure_gate_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/program/m5-supported-line-correction-train-archive-and-closure-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-correction-train-archive.schema.json`](../../schemas/program/m5-correction-train-archive.schema.json)
  (reused from the frozen matrix — the correction-train-archive record each shipped correction packet is archived
  against) and
  [`schemas/program/m5-closure-gate.schema.json`](../../schemas/program/m5-closure-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-supported-line-correction-train-archive-and-closure-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first supported-line
  correction-train archive — it demonstrates one durable correction-lineage loop end to end for at least one active
  supported line.
- **Narrowed fixtures:**
  `fixtures/release/m5-supported-line-correction-train-archive-and-closure-gate-registries/`
  (`correction_train_archive_beta_narrowed.json`, `closure_gate_preview_narrowed.json`).

## Two registries

1. **Correction-train archive** (`resolve_correction_train_archive_entry`) — archives one typed record per shipped
   correction packet, per active supported line: the corrective action class and its canonical mode, the archived
   evidence rows, the linked supported-line-matrix / active-claim / defect-ledger / release-artifact refs, the public
   communication state, the rollback outcome, and the owning roster, with public-safe advisory and
   public-communication history separated from internal-only hotfix / backport / rollback / revocation incident
   payloads. A clean record names a canonical registry token, a classified corrective action class, and a
   transparency role, covers the canonical / accessible / audit resolution forms, publishes a complete object,
   preserves its exact-build provenance before a claim widens, and keeps a public-facing action class's published
   communication matched to archived provenance. Otherwise it degrades honestly — a correction line widening its
   claim on stale archive evidence, or a public-facing action class running its published communication ahead of
   archived provenance, degrades to
   `correction_train_archive_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-on-stale-archive attempt must surface.
2. **Closure gate** (`resolve_closure_gate_entry`) — turns an archive-coverage gap on a correction line into a typed
   gate that blocks closure rather than a forgotten shiproom note. A clean entry names a classified gate scope
   (missing-archive-coverage, broken-exact-build-join, or untraceable-correction-line) and provides the complete
   line-identity / affected-archive-record / archived-versus-active-build / gate-scope / active-reason gate object;
   a gate event that would keep a claim ahead of archived provenance, hide the gap, or let missing coverage
   masquerade as covered degrades to
   `closure_gate_runs_support_ahead_of_proof_or_drops_closure_gate`.

## Per-record correction-archive reference

The archived corrective action class carries its canonical mode, and the resolver publishes the full archive object,
so the registry — never an archive merely assumed to still be current — is the single source of truth.
`correction_train_archive_object_is_complete` rejects an object missing any archive field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening on stale archive evidence or a
published communication running ahead of archived provenance, and `closure_gate_stays_honest` rejects a gate event
that has kept a claim ahead of archived provenance.

## Acceptance criteria (proven by resolved examples)

- **Every shipped correction packet on an active supported line has one archive record with exact-build identity,
  corrective action class, public communication state, and rollback outcome.** Clean correction-train-archive entries
  cover the canonical hotfix-packet-archive / backport-packet-archive / rollback-outcome-record / advisory-publication
  / public-communication-bundle / revocation-record action classes and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no clean
  archive entry published an incomplete object.
- **Support or procurement readers can trace a correction from an advisory or release note back to the archived
  evidence bundle without private shiproom materials.** A widen-on-stale-archive example and an unbound example
  degrade, a clean archive entry is present, and no clean entry is unbound or missing its exact-build provenance.
- **Missing archive coverage or broken exact-build joins block correction-line closure until fixed.** Clean
  closure-gate entries cover the missing-archive-coverage / broken-exact-build-join / untraceable-correction-line
  gate scopes with full resolution-form coverage while providing the complete gate object — the resolved line
  identity and the active gate reason — and a gate event that would keep a claim ahead of archived provenance or drop
  the gate degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_supported_line_correction_train_archive_and_closure_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_supported_line_correction_train_archive_and_closure_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_supported_line_correction_train_archive_and_closure_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_supported_line_correction_train_archive_and_closure_gate_registries -- correction-train-archive-table
cargo run -p aureline-ui --example dump_m5_supported_line_correction_train_archive_and_closure_gate_registries -- fixture-correction-train-archive-beta-narrowed
cargo run -p aureline-ui --example dump_m5_supported_line_correction_train_archive_and_closure_gate_registries -- fixture-closure-gate-preview-narrowed
```
