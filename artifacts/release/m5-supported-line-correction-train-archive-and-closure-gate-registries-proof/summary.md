# M5 Correction-Train-Archive and Closure-Gate Registries

- Packet: `m5-supported-line-correction-train-archive-and-closure-gate-registries:stable:0001`
- Label: `M5 correction-train-archive and closure-gate registries archiving one correction-train archive per shipped correction packet on each active stable or LTS-candidate line — one row per corrective action class: a hotfix packet, a backport packet, a rollback outcome, an advisory publication, a public-communication bundle, and a revocation record, tracked against exact build / release-line identity — each bound to one supported-line identity with its bug-ID / defect-ledger / release-artifact-graph joins and the public-claim or support-window state the correction affected, public-safe advisory and public-communication history separated from internal-only hotfix / backport / rollback / revocation incident payloads, exact-build provenance preserved so a correction claim never runs ahead of it, canonical / accessible / audit resolution-form coverage, and a machine-readable closure-gate event (missing-archive-coverage, broken-exact-build-join, or untraceable-correction-line) that turns missing archive coverage, a broken exact-build join, or a correction that can no longer be traced back to its archived evidence into a typed event on the active line, blocking correction-line closure until fixed and naming the active gate reason across release / help, docs, support, and procurement surfaces`
- Consumer surfaces: 6
- Action classes: hotfix_packet_archive, backport_packet_archive, rollback_outcome_record, advisory_publication, public_communication_bundle, revocation_record, archive_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the line's archived hotfix packet to one typed correction-train archive — the corrective action class, its rollback outcome, the public communication state, and the owning roster — from the shared registry and proves the missing-archive-coverage closure-gate event for that line; a correction-train archive missing its exact-build provenance and a gate event that keeps a correction claim ahead of archived provenance degrade honestly instead of leaving a shipped correction to read as still green
  - Correction-train-archive entries: 2 / closure-gate entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the backport-packet-archive record and the untraceable-correction-line closure-gate event while keeping the active gate reason visible; a line widening its claim on stale archive evidence and a resolution-form gap on a gate event are caught before a screenshot can reintroduce a still-green reading
  - Correction-train-archive entries: 2 / closure-gate entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the advisory-publication record (public-facing) while keeping its published communication matched to archived exact-build provenance and reports the closure-gate outcome; a correction-train archive that is a hand-copied per-entry assumption and a gate event on an unclassified gate scope degrade honestly
  - Correction-train-archive entries: 2 / closure-gate entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the rollback-outcome-record and the broken-exact-build-join closure-gate event bound to the registry; an unstated registry token on a correction-train archive is caught before it can drift
  - Correction-train-archive entries: 2 / closure-gate entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved correction-archive and closure-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the revocation-record archive and the untraceable-correction-line gate event stay inspectable off-renderer
  - Correction-train-archive entries: 1 / closure-gate entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved correction-archive and closure-gate truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-archive attempt, or a correction claim running ahead of archived provenance is visible in evidence — missing archive coverage, a broken exact-build join, or an untraceable correction line — rather than hidden behind a shiproom note or private materials
  - Correction-train-archive entries: 1 / closure-gate entries: 1
