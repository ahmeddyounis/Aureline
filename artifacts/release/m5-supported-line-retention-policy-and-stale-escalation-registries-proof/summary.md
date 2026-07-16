# M5 Supported-Line Retention-Policy and Stale-Escalation Registries

- Packet: `m5-supported-line-retention-policy-and-stale-escalation-registries:stable:0001`
- Label: `M5 supported-line retention-policy and stale-escalation registries carrying one retention policy per B147 supported-line proof artifact class — a public-proof-ledger policy, a migration-scoreboard policy, a transparency-report policy, a correction-archive policy, a truth-feed policy, and an ORR-history policy, tracked against exact build / release-line identity — each naming its accountable owner and backup, review cadence, retention window, archive class, and destruction-or-long-term-retention rule so every class can be inspected in one checked-in policy packet, public-safe classes separated from internal-only incident / security ones, exact-build provenance preserved so a claim never runs ahead of it, canonical / accessible / audit resolution-form coverage, and typed stale-escalation blockers (a missing scheduled snapshot, a stale line feed, or a snapshot mismatched with the active supported-line matrix) that block a supported line from staying green on expired evidence while exposing the active snapshot age and provenance across release / help, docs, support, procurement, and partner surfaces`
- Consumer surfaces: 6
- Artifact classes: public_proof_ledger_policy, migration_scoreboard_policy, transparency_report_policy, correction_archive_policy, truth_feed_policy, orr_history_policy, policy_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the public-proof-ledger-policy retention rule — its owner and backup, review cadence, retention window, archive class, and disposition — from the shared registry and raises the missing-scheduled-snapshot escalation for that class; a policy missing its exact-build provenance and an escalation that keeps a claim ahead of current proof degrade honestly instead of leaving a stale line to read as still green
  - Retention-policy entries: 2 / stale-escalation entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the migration-scoreboard-policy retention rule and raises the matrix-mismatch escalation while keeping the active escalation reason visible; a line widening its claim on stale proof and a resolution-form gap on an escalation are caught before a screenshot can reintroduce a still-green reading
  - Retention-policy entries: 2 / stale-escalation entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the correction-archive-policy retention rule (public-facing) while keeping its published cadence matched to current exact-build proof and reports the stale-escalation outcome; a policy that is a hand-copied per-entry assumption and an escalation on an unclassified escalation scope degrade honestly
  - Retention-policy entries: 2 / stale-escalation entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the transparency-report-policy retention rule and raises the stale-line-feed escalation bound to the registry; an unstated registry token on a policy is caught before it can drift
  - Retention-policy entries: 2 / stale-escalation entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved retention-policy and stale-escalation truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the ORR-history-policy retention rule and the matrix-mismatch escalation stay inspectable off-renderer
  - Retention-policy entries: 1 / stale-escalation entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved retention-policy and stale-escalation truth, so a hand-copied constant, an unstated registry token, a stay-green-on-stale-proof attempt, or a claim running ahead of current proof is visible in evidence — the truth-feed-policy retention rule and the stale-line-feed escalation, each exposing the active snapshot age and provenance — rather than hidden behind a shiproom note or private materials
  - Retention-policy entries: 1 / stale-escalation entries: 1
