# M5 Stable-Line Correction-Report and Train-Comparison Registries

- Packet: `m5-stable-line-correction-report-and-train-comparison-registries:stable:0001`
- Label: `M5 post-launch correction-report and train-comparison registries publishing one typed correction report per release train — one section per operating signal: top adoption blockers, crash / support signals, compatibility-report freshness deltas, bundle drift, public-truth deltas, and backport exceptions or deferrals — each linked to its correction packets, supported-line defect-ledger entries, and current claim rows, with rollback posture preserved so onboarding / migration / support language never runs ahead of the linked correction evidence, canonical / accessible / audit resolution-form coverage, and a machine-readable train-comparison (corrected-issue, remaining-narrowed-claim, or open-exception-closure) that lets operators compare trains to see which supported-line issues were corrected, which narrowed claims remain, and which exceptions still need explicit closure, naming the active comparison reason across release / help, support, shiproom, executive-steering, program-governance, and public-proof surfaces`
- Consumer surfaces: 6
- Report sections: crash_support_signal_section, bundle_drift_section, adoption_blocker_section, compatibility_freshness_section, public_truth_delta_section, backport_exception_section, report_section_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the train's crash / support-signal section to one typed correction-report object — the affected line rows, report section, linked correction packets / defect-ledger entries / claim rows, rollback target, and owning roster — from the shared registry and proves the corrected-issue comparison for that train; a correction-report object missing its linked evidence and a comparison that keeps support language ahead of the linked correction degrade honestly instead of leaving a field signal to read as silently resolved
  - Correction-report entries: 2 / train-comparison entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the bundle-drift section and the open-exception-closure comparison while keeping the active comparison reason visible; a train widening its claim while its correction is unresolved and a resolution-form gap on a comparison are caught before a screenshot can reintroduce a silently-resolved reading
  - Correction-report entries: 2 / train-comparison entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the compatibility-freshness-delta section while keeping its compatibility / known-issues claim matched to the linked correction evidence and reports the train-comparison outcome; a correction-report entry that is a hand-copied per-entry assumption and a comparison on an unclassified comparison scope degrade honestly
  - Correction-report entries: 2 / train-comparison entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the top adoption-blocker section and the remaining-narrowed-claim comparison bound to the registry; an unstated registry token on a correction-report entry is caught before it can drift
  - Correction-report entries: 2 / train-comparison entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved correction-report and train-comparison truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the backport-exception section and the open-exception-closure comparison stay inspectable off-renderer
  - Correction-report entries: 1 / train-comparison entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved correction-report and train-comparison truth, so a hand-copied constant, an unstated registry token, a widen-over-unresolved-train attempt, or support language running ahead of the linked correction is visible in evidence — a corrected issue, a remaining narrowed claim, or an open exception still needing closure — rather than hidden behind a screenshot
  - Correction-report entries: 1 / train-comparison entries: 1
