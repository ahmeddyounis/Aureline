# M5 supported-line retention-policy and stale-escalation registries

This lane keeps the B147 supported-line proof artifacts alive and reviewable after they first ship, over the frozen
[M5 supported-line-transparency matrix](./m5-supported-line-transparency-ops.md). B147 landed the artifacts —
public-proof ledgers, transparency reports, migration scoreboards, ORR histories, correction-train archives, and the
supported-line truth feeds that bundle them — but nothing yet governs their retention, export, and review cadence, so
they risk decaying into one-off launch appendices. This lane carries one export-safe *retention policy* per B147
artifact class — a public-proof-ledger policy, a migration-scoreboard policy, a transparency-report policy, a
correction-archive policy, a truth-feed policy, and an ORR-history policy — each naming its accountable owner and
backup, review cadence, retention window, archive class, and destruction-or-long-term-retention rule, bound to exact
build / release-line identity, so every class can be inspected in one checked-in policy packet. It raises one typed
*stale escalation* per missed cadence — a missing scheduled snapshot, a stale line feed, or a snapshot mismatched with
the active supported-line matrix — so automation blocks a supported line from staying green on expired evidence, and
the checked-in policy packet exposes each snapshot's age and provenance so support bundles, docs/help/About truth, and
public-proof consumers pull the freshest permitted snapshot. It records the *retention-policy* grammar (one typed
policy per B147 artifact class, tracked against exact build / release-line identity, public-safe classes separated
from internal-only incident / security ones) and the *stale-escalation* grammar (the typed blocker raised when a
required snapshot is missing, stale, or mismatched with the active matrix) into registry resolvers that produce
export-safe, honest projections, so release / help, docs, support, procurement, and partner surfaces read one
canonical retention discipline instead of re-deriving it by hand. The retention policies and the stale escalations
are separated in runtime and serialized state: the artifact class, its owner and cadence, its retention window and
disposition, its exact-build provenance, and the linked supported-line-matrix / active-claim / proof-registry refs
live on the retention policy, while the affected artifact-class reference, the active supported-line matrix
reference, and the active escalation reason live on the stale escalation, and no export leaks internal-only incident /
security detail or lets a stale snapshot read as current.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_supported_line_retention_policy_and_stale_escalation_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/program/m5-supported-line-retention-policy-and-stale-escalation-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-retention-policy.schema.json`](../../schemas/program/m5-retention-policy.schema.json)
  (minted by this lane — the retention, export, and review-cadence rule per B147 artifact class) and
  [`schemas/program/m5-stale-escalation.schema.json`](../../schemas/program/m5-stale-escalation.schema.json)
  (minted by this lane — the typed blocker raised when a snapshot is missing, stale, or matrix-mismatched) as its
  canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-supported-line-retention-policy-and-stale-escalation-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the single policy packet in which
  each B147 artifact class's owner, cadence, retention window, archive class, and stale-escalation rule can be
  inspected, and the summary exposes the active snapshot age (proof-freshness SLO and last refresh) and provenance.
- **Narrowed fixtures:**
  `fixtures/release/m5-supported-line-retention-policy-and-stale-escalation-registries/`
  (`retention_policy_beta_narrowed.json`, `stale_escalation_preview_narrowed.json`).

## Two registries

1. **Retention policy** (`resolve_retention_policy_entry`) — carries one typed policy per B147 artifact class: the
   artifact class and its canonical mode, the accountable owner and backup, the review cadence, the retention window,
   the archive class, the disposition, and the linked supported-line-matrix / active-claim / proof-registry refs, with
   public-safe classes separated from internal-only incident / security ones. A clean policy names a canonical
   registry token, a classified artifact class, and a transparency role, covers the canonical / accessible / audit
   resolution forms, publishes a complete policy object, preserves its exact-build provenance before a claim widens,
   and keeps a public-facing class's published cadence matched to current proof. Otherwise it degrades honestly — a
   line widening its claim on stale proof, or a public-facing class running its published cadence ahead of current
   proof, degrades to `retention_policy_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the
   structured blocker reason a stay-green-on-stale-evidence attempt must surface.
2. **Stale escalation** (`resolve_stale_escalation_entry`) — raises one typed blocker per missed review cadence rather
   than a silent decay. A clean entry names a classified escalation scope (a missing scheduled snapshot, a stale line
   feed, or a matrix mismatch) and provides the complete affected-artifact-class / active-supported-line-matrix /
   active-escalation-reason blocker object; an escalation that would keep a claim ahead of current proof, leak
   internal-only detail, or let a stale snapshot masquerade as current degrades to
   `stale_escalation_runs_support_ahead_of_proof_or_drops_stale_escalation`.

## Per-record retention-policy reference

Each governed artifact class carries its canonical mode, and the resolver publishes the full policy object, so the
registry — never a policy merely assumed to still hold — is the single source of truth.
`retention_policy_object_is_complete` rejects an object missing any policy field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening on stale proof or a published
cadence running ahead of current proof, and `stale_escalation_stays_honest` rejects an escalation that has kept a
claim ahead of current proof.

## Acceptance criteria (proven by resolved examples)

- **Each new B147 artifact class has an owner, cadence, retention window, and stale-escalation rule that can be
  inspected in one checked-in policy packet.** Clean retention-policy entries cover the canonical
  public-proof-ledger-policy / migration-scoreboard-policy / transparency-report-policy / correction-archive-policy /
  truth-feed-policy / ORR-history-policy artifact classes and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no clean
  policy entry published an incomplete object.
- **Automation can detect missing scheduled snapshots or stale line feeds and raise a typed blocker before
  supported-line claims remain green on expired evidence.** A stay-green-on-stale-proof example and an unbound example
  degrade, a clean policy entry is present, and no clean entry is unbound or missing its exact-build provenance.
- **At least one docs/help/support/public-proof consumer shows the active snapshot age and provenance for a B147
  artifact class.** Clean stale-escalation entries cover the missing-scheduled-snapshot / stale-line-feed /
  matrix-mismatch escalation scopes with full resolution-form coverage while providing the complete blocker object —
  the affected artifact class and the active escalation reason — and the checked-in summary exposes the active
  snapshot age and provenance, while an escalation that would keep a claim ahead of current proof or drop the blocker
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_supported_line_retention_policy_and_stale_escalation_registries -- support-export
cargo run -p aureline-ui --example dump_m5_supported_line_retention_policy_and_stale_escalation_registries -- csv
cargo run -p aureline-ui --example dump_m5_supported_line_retention_policy_and_stale_escalation_registries -- report
cargo run -p aureline-ui --example dump_m5_supported_line_retention_policy_and_stale_escalation_registries -- retention-policy-table
cargo run -p aureline-ui --example dump_m5_supported_line_retention_policy_and_stale_escalation_registries -- fixture-retention-policy-beta-narrowed
cargo run -p aureline-ui --example dump_m5_supported_line_retention_policy_and_stale_escalation_registries -- fixture-stale-escalation-preview-narrowed
```
