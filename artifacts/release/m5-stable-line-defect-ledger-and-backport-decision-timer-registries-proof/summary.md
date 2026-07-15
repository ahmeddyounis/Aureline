# M5 Stable-Line Defect-Ledger and Backport-Decision-Timer Registries

- Packet: `m5-stable-line-defect-ledger-and-backport-decision-timer-registries:stable:0001`
- Label: `M5 supported-line defect-ledger and backport-decision-timer registries with one typed defect-ledger recording, for each supported-line defect on the active stable line — a crash-recovery defect, a rollback/update defect, a support-export defect, a migration/import defect, a compatibility-regression defect, or a security/data-loss defect — its exact affected line rows, defect class, yes/no/defer backport decision and decision age, correction-packet state, rollback target, and owning release / support roster — onboarding / migration / support language never running ahead of a recorded backport decision, canonical / accessible / audit resolution-form coverage, and a machine-readable backport-decision-timer (missing-backport-decision, overdue-backport-decision, or narrowed-support-claim) that raises a visible alert and narrows the affected claim automatically when a decision is missing or overdue, naming the active alert reason across release / help, support, shiproom, executive-steering, program-governance, and public-proof surfaces`
- Consumer surfaces: 6
- Ledger defects: crash_recovery_defect, rollback_update_defect, support_export_defect, migration_import_defect, compatibility_regression_defect, security_or_data_loss_defect, defect_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the active supported line's crash-recovery defect to one typed defect-ledger object — the affected line rows, defect class, backport decision and decision age, rollback target, correction-packet state, and owning roster — from the shared registry and proves the missing-backport-decision alert for that defect; a defect-ledger object missing its fields and an alert that keeps support language ahead of a recorded decision degrade honestly instead of leaving a supported-line defect to read as silently serviced
  - Defect-ledger entries: 2 / backport-decision-timer entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the rollback/update defect and the narrowed-support-claim alert while keeping the active alert reason visible; a line widening its claim while a defect's backport decision is missing or overdue and a resolution-form gap on an alert are caught before a screenshot can reintroduce a silently-serviced reading
  - Defect-ledger entries: 2 / backport-decision-timer entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the migration/import defect while keeping its onboarding / migration claim matched to a recorded backport decision and reports the backport-decision-timer outcome; a defect-ledger entry that is a hand-copied per-entry assumption and an alert on an unclassified alert scope degrade honestly
  - Defect-ledger entries: 2 / backport-decision-timer entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the support-export defect and the overdue-backport-decision alert bound to the registry; an unstated registry token on a defect-ledger entry is caught before it can drift
  - Defect-ledger entries: 2 / backport-decision-timer entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved defect-ledger and backport-decision-timer truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the security-or-data-loss defect and the narrowed-support-claim alert stay inspectable off-renderer
  - Defect-ledger entries: 1 / backport-decision-timer entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved defect-ledger and backport-decision-timer truth, so a hand-copied constant, an unstated registry token, a widen-over-unresolved-defect attempt, or support language running ahead of a recorded decision is visible in evidence — a missing, overdue, or narrowed backport decision — rather than hidden behind a screenshot
  - Defect-ledger entries: 1 / backport-decision-timer entries: 1
