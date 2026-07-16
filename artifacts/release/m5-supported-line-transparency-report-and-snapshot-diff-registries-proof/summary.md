# M5 Transparency-Report and Snapshot-Diff Registries

- Packet: `m5-supported-line-transparency-report-and-snapshot-diff-registries:stable:0001`
- Label: `M5 transparency-report and snapshot-diff registries publishing one export-safe transparency / upstream-health report per active supported line — one section per upstream-health dimension: critical-upstream status, backup maintainer coverage, signer-quorum health, emergency-authority coverage, sustainment / sponsor posture, and unresolved red-risk dependencies — each bound to one supported-line identity with public-safe health separated from internal-only incident / security detail, posture preserved so support language never runs ahead of current public proof, canonical / accessible / audit resolution-form coverage, and a machine-readable report snapshot diff (health-status-change, coverage-narrowing, or red-risk-drift) that turns a shift against the prior published snapshot into a typed diff event showing trend and drift, naming the active diff reason across release / help, About, docs, support, and procurement surfaces`
- Consumer surfaces: 6
- Report sections: critical_upstream_status_section, backup_maintainer_coverage_section, signer_quorum_health_section, emergency_authority_coverage_section, sustainment_sponsor_posture_section, red_risk_dependency_section, report_section_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the line's critical-upstream-status section to one typed transparency-report object — the affected line rows, upstream-health state, linked critical-upstream register refs, and owning roster — from the shared registry and proves the health-status-change diff for that line; a transparency-report object missing its linked upstream register and a diff that keeps support language ahead of current proof degrade honestly instead of leaving a claim to read as still green
  - Correction-report entries: 2 / snapshot-diff entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the backup-maintainer-coverage section and the red-risk-drift diff while keeping the active diff reason visible; a line widening its health claim on stale proof and a resolution-form gap on a diff are caught before a screenshot can reintroduce a still-green reading
  - Correction-report entries: 2 / snapshot-diff entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the emergency-authority-coverage section (public-facing) while keeping its published health claim matched to current public proof and reports the snapshot-diff outcome; a transparency-report entry that is a hand-copied per-entry assumption and a diff on an unclassified diff scope degrade honestly
  - Correction-report entries: 2 / snapshot-diff entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the signer-quorum-health section and the coverage-narrowing diff bound to the registry; an unstated registry token on a transparency-report entry is caught before it can drift
  - Correction-report entries: 2 / snapshot-diff entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved transparency-report and snapshot-diff truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the red-risk-dependency section and the red-risk-drift diff stay inspectable off-renderer
  - Correction-report entries: 1 / snapshot-diff entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved transparency-report and snapshot-diff truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-health attempt, or support language running ahead of current proof is visible in evidence — a health-status change, a coverage narrowing, or a red-risk drift — rather than hidden behind a screenshot
  - Correction-report entries: 1 / snapshot-diff entries: 1
