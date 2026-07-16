# M5 transparency-report and snapshot-diff registries

This lane turns maintainer / upstream durability from an internal registry into durable, support-line-safe product
truth over the frozen
[M5 supported-line-transparency matrix](./m5-supported-line-transparency-ops.md). It publishes one export-safe
*transparency / upstream-health report* per active stable or LTS-candidate line so partner reviews, procurement
checks, support escalations, and OSS stewardship inherit current rather than tribal truth, and emits a typed
*report snapshot diff* whenever a published report changes — instead of letting a critical-upstream, maintainer,
signer, or red-risk shift drift into a forgotten repository-maintenance note. It records the *transparency-report*
grammar (the export-safe report published per active supported line — one typed section per upstream-health
dimension: critical-upstream status, backup maintainer coverage, signer-quorum health, emergency-authority
coverage, sustainment / sponsor posture, and unresolved red-risk dependencies — each bound to one supported-line
identity with public-safe health separated from internal-only incident / security detail) and the *snapshot-diff*
grammar (the typed diff event naming whether a report section moved to a worse or better health state versus the
prior published snapshot, narrowed its maintainer / signer / authority coverage, or surfaced a new or changed
unresolved red-risk dependency, naming the active diff reason) into registry resolvers that produce export-safe,
honest projections, so release / help, About, docs, support, and procurement surfaces resolve one canonical,
freshness-checked truth instead of re-synthesizing product truth by hand. The report and the diff are separated in
runtime and serialized state: the summarized health section, affected rows, linked upstream-register /
maintainer-coverage / signing-quorum refs, and posture live on the transparency-report entry, while the resolved
line identity, affected report-section reference, previous-versus-current snapshot reference, diff-scope state, and
active diff reason live on the snapshot-diff entry, and a line's posture stays preserved so support language never
runs ahead of current public proof.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_supported_line_transparency_report_and_snapshot_diff_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-supported-line-transparency-report-and-snapshot-diff-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-public-proof-freshness-ledger.schema.json`](../../schemas/program/m5-public-proof-freshness-ledger.schema.json)
  (reused from the frozen matrix — the public-proof freshness ledger each supported-line health record is recorded
  against) and
  [`schemas/program/m5-snapshot-diff.schema.json`](../../schemas/program/m5-snapshot-diff.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-supported-line-transparency-report-and-snapshot-diff-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first export-safe transparency
  report — it demonstrates one durable upstream-health proof loop end to end for at least one active supported line.
- **Narrowed fixtures:**
  `fixtures/release/m5-supported-line-transparency-report-and-snapshot-diff-registries/`
  (`transparency_report_beta_narrowed.json`, `snapshot_diff_preview_narrowed.json`).

## Two registries

1. **Transparency report** (`resolve_transparency_report_entry`) — builds one typed report section per
   upstream-health dimension, per active supported line: the health section and its canonical mode, the affected
   line rows, the linked critical-upstream / maintainer-coverage / signing-quorum / emergency-authority /
   sustainment refs, the health state, the reversibility target, and the owning roster, with public-safe health
   separated from internal-only incident / security detail. A clean entry names a canonical registry token, a
   classified section, and a transparency role, covers the canonical / accessible / audit resolution forms,
   publishes a complete object, preserves its posture before a claim widens, and keeps a public-facing section's
   health claim matched to current public proof. Otherwise it degrades honestly — a line widening its health claim
   on stale proof, or a public-facing section running its language ahead of current proof, degrades to
   `transparency_report_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-on-stale-health attempt must surface.
2. **Report snapshot diff** (`resolve_snapshot_diff_entry`) — turns a report change into a typed diff event against
   the prior published snapshot rather than a forgotten maintenance note. A clean entry names a classified diff
   scope (health-status-change, coverage-narrowing, or red-risk-drift) and provides the complete line-identity /
   affected-report-section / previous-versus-current-snapshot / diff-scope / active-reason diff object; a diff that
   would keep support language ahead of current proof, hide the diff, or let a gap masquerade as covered degrades to
   `snapshot_diff_runs_support_ahead_of_proof_or_drops_snapshot_diff`.

## Per-entry report reference

The summarized health section carries its canonical mode, and the resolver publishes the full report object, so the
registry — never a report merely assumed to still be current — is the single source of truth.
`transparency_report_object_is_complete` rejects an object missing any report field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening on stale health or language
running ahead of current proof, and `snapshot_diff_stays_honest` rejects a diff that has kept support language ahead
of current proof.

## Acceptance criteria (proven by resolved examples)

- **At least one export-safe transparency report exists for active supported lines and is linked to the
  critical-upstream register, maintainer coverage matrix, and signing quorum records.** Clean transparency-report
  entries cover the canonical critical-upstream-status / backup-maintainer-coverage / signer-quorum-health /
  emergency-authority-coverage / sustainment-sponsor-posture / red-risk-dependency sections and the first
  release-center / shiproom / executive-steering / program-governance / support surfaces, an object-incomplete
  example degrades, and no clean report entry published an incomplete object.
- **Red-risk upstream or signing gaps automatically surface on the affected line rather than remaining hidden in
  repository-maintenance notes.** A widen-on-stale-health example and an unbound example degrade, a clean report
  entry is present, and no clean entry is unbounded or unbound.
- **Public-safe and internal-only variants share one canonical record identity and do not diverge on core
  supported-line facts.** Clean snapshot-diff entries cover the health-status-change / coverage-narrowing /
  red-risk-drift diff scopes with full resolution-form coverage while providing the complete diff object — the
  resolved line identity and the active diff reason — and a diff that would keep support language ahead of current
  proof or drop the diff degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- support-export
cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- csv
cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- report
cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- transparency-report-table
cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- fixture-transparency-report-beta-narrowed
cargo run -p aureline-ui --example dump_m5_supported_line_transparency_report_and_snapshot_diff_registries -- fixture-snapshot-diff-preview-narrowed
```
