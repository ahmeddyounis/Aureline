# M5 Migration-Scoreboard and Scoreboard-Delta Registries

- Packet: `m5-supported-line-migration-scoreboard-and-scoreboard-delta-registries:stable:0001`
- Label: `M5 migration-scoreboard and scoreboard-delta registries publishing one versioned, scored migration scoreboard per active supported line — one row per importer / bridge outcome class: cleanly imported, translated to an equivalent, partially imported, shimmed through a compatibility shim, unsupported item category, and rollback-cleanliness result, tracked by source tool / version / archetype — each bound to one supported-line identity with rollback cleanliness, docs/help parity, and linked compatibility evidence, public-safe outcome classes separated from internal-only migration detail, rollback posture preserved so replacement-grade language never runs ahead of current migration truth, canonical / accessible / audit resolution-form coverage, and a machine-readable periodic scoreboard delta (field-pain-cluster, unsupported-category-growth, or docs-help-or-rollback-gap) that turns a shift against the last published scoreboard into a typed delta event showing where field pain, unsupported-item categories, docs/help gaps, or rollback failures are accumulating, naming the active delta reason across release / help, docs, support, and migration surfaces`
- Consumer surfaces: 6
- Outcome classes: imported_outcome_class, translated_outcome_class, partial_outcome_class, shimmed_outcome_class, unsupported_outcome_class, rollback_cleanliness_outcome_class, outcome_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the line's imported outcome class to one typed migration-scoreboard object — the affected migration rows, importer outcome state, linked compatibility-report refs, and owning roster — from the shared registry and proves the field-pain-cluster delta for that line; a migration-scoreboard object missing its linked compatibility evidence and a delta that keeps replacement-grade language ahead of current migration truth degrade honestly instead of leaving a switching claim to read as still green
  - Migration-scoreboard entries: 2 / scoreboard-delta entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the translated outcome class and the docs-help-or-rollback-gap delta while keeping the active delta reason visible; a line widening its replacement-grade claim on stale migration truth and a resolution-form gap on a delta are caught before a screenshot can reintroduce a still-green reading
  - Migration-scoreboard entries: 2 / scoreboard-delta entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the shimmed outcome class (public-facing) while keeping its published replacement-grade claim matched to current migration truth and reports the scoreboard-delta outcome; a migration-scoreboard entry that is a hand-copied per-entry assumption and a delta on an unclassified delta scope degrade honestly
  - Migration-scoreboard entries: 2 / scoreboard-delta entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the partial outcome class and the unsupported-category-growth delta bound to the registry; an unstated registry token on a migration-scoreboard entry is caught before it can drift
  - Migration-scoreboard entries: 2 / scoreboard-delta entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved migration-scoreboard and scoreboard-delta truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the rollback-cleanliness outcome class and the docs-help-or-rollback-gap delta stay inspectable off-renderer
  - Migration-scoreboard entries: 1 / scoreboard-delta entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved migration-scoreboard and scoreboard-delta truth, so a hand-copied constant, an unstated registry token, a widen-on-drifted-migration-truth attempt, or replacement-grade language running ahead of current evidence is visible in evidence — a field-pain cluster, an unsupported-category growth, or a docs-help-or-rollback gap — rather than hidden behind an anecdotal support thread
  - Migration-scoreboard entries: 1 / scoreboard-delta entries: 1
