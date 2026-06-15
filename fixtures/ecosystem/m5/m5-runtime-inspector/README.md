# Fixtures: M5 runtime inspector cards

This directory contains fixture metadata for the `m5_runtime_inspector` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-runtime-inspector.json`

## Coverage

- Eight inspector cards cover the framework-pack, docs-pack, local-model-pack,
  recipe-pack, template, bridge-backed package, side-loaded package, and
  mirrored-registry families, so one runtime-inspection model is proven across every
  marketed M5 artifact family.
- Each card carries a `governance_family_ref` that resolves to its row in
  `artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`.
- Every load state is exercised: `loaded_healthy`, `loaded_degraded`, `load_failed`,
  `source_missing`, `quarantine_held`, and `operator_disabled`.
- Every disposition is exercised: `running_healthy`, `running_degraded`,
  `showing_last_known_good`, `fresh_review_required`, `operator_disabled`, and
  `quarantined`.
- Activation buckets cover `cold` and `warm`; memory pressure covers `healthy`,
  `elevated`, `over_budget`, and `not_applicable`; capability exercise state covers
  `declared_exercised`, `declared_unused` (over-grant), and `undeclared_exercised`
  (policy violation).

## Guardrails proven by the corpus

- **No local or untrusted package inherits a trusted badge.** The unsigned side-loaded
  model pack, the operator-disabled side-load, and the quarantined mirrored variant all
  render `unsigned_local_only`. A genuinely `signed_verified` framework pack still
  renders `enterprise_approved`, so the cap reflects provenance rather than blanketing
  every card to local-only.
- **A widening hot reload forces a fresh review.** The recipe pack runs healthy but its
  pending hot reload widens permissions; it recomputes to `fresh_review_required`, and
  its restart and reload actions are disabled until a fresh review clears it.
- **The inspector stays useful when failing.** The template card lost its source path
  and the bridge card failed to load; both keep a last-known-good revision, runtime,
  host, and badge visible, and that last-good badge never exceeds the current cap.
- **Crash history and capabilities are never hidden.** The disabled side-load keeps its
  undeclared-capability and crash records on the card, and the quarantined mirrored
  variant keeps its crash-loop and over-budget memory record visible; both still expose
  `view_logs` and a review-routed `re_enable`.
- **Undeclared capability use is surfaced.** The side-load exercised a secret-read
  capability it never declared; that appears as an `undeclared_capability_exercised`
  review trigger even while the operator hold owns the disposition.

## Validation

`M5RuntimeInspector::validate()` is the CI-facing gate. It checks the closed
vocabularies, signature-ref consistency, the last-known-good requirement, action gating
(logs always available, re-enable on held cards, restart/reload held under a fresh
review), and — crucially — recomputes the rendered trust tier, the review-trigger set,
and the disposition from each card's facts and flags any drift. The executable proof
lives in `crates/aureline-ecosystem/src/m5_runtime_inspector/tests.rs`.
