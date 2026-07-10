# M5 convention-diagnostic rows and derived-relationship banners

This contract implements the frozen `convention_diagnostic_row` and `derived_relationship_banner`
component families from the [M5 framework-component matrix](m5_framework_component_matrix.md) as two
reusable, co-equal control vectors — the **convention-diagnostic row** and the
**derived-relationship banner** — so a framework warning stays honest about certainty and evidence
and an inferred relationship never hides its approximation in the background.

The Rust validator in
`crates/aureline-templates/src/implement_convention_diagnostic_rows_and_derived_relationship_banners_with_diagnostic_class_affected_entity_or_file_certainty_detected_source_suggested_fix_or_open_docs_actions_support_class_caveats_and_open_raw_source_or_wider_graph_continuity`
is the authoritative gate; the
[boundary schema](../../../schemas/ui/m5-convention-diagnostic-derived-relationship-controls.schema.json)
documents the export shape.

## What the convention-diagnostic row names

A convention-diagnostic row names, before a user trusts it:

- **Diagnostic class** — the distinct problem class: `hard_contract_violation`, `pack_limitation`,
  `version_mismatch`, `heuristic_suspicion`, `deprecation_notice`, or `unknown_diagnostic`. A
  framework warning never collapses these into one generic warning state.
- **Affected entity / file** — the affected entity and the proving file that grounds it.
- **Confidence / severity** — the frozen convention confidence class (`verified`, `high_confidence`,
  `heuristic_convention`, `derived_by_convention`, `low_confidence`, `unknown`) and the frozen
  severity (`error`, `warning`, `hint`, `info`, `suppressed`, `stale`).
- **Detected source** — how the diagnostic was detected: `static_analysis`, `framework_contract`,
  `pack_manifest`, `runtime_probe`, or `heuristic_scan`.
- **Suggested fix / open-docs action** — `auto_fix_available`, `manual_fix_guidance`,
  `open_docs_only`, or `no_fix_available`, plus the suggested-fix label.
- **Support-class caveat** — `fully_supported`, `pack_limited`, `version_mismatch`,
  `bridged_behavior`, `heuristic_only`, or `unsupported`, so a pack limitation or bridged behavior
  never reads as fully-supported first-party truth.

## What the derived-relationship banner names

A derived-relationship banner names:

- The **relationship** and its frozen **derived-relationship class** — `exact_from_source`,
  `inferred_from_runtime`, `heuristic_link`, `derived_by_convention`, `partial_link`, or
  `unresolved_link`.
- The **source of inference** — `static_source`, `runtime_observation`, `naming_convention`,
  `dependency_graph`, or `manifest_declaration`.
- The **last refresh** — whether the inference is `current`, `imported`, `stale`, `never_refreshed`,
  or `unknown`.
- The **place of consumption** — where the inferred framework truth is consumed, so the banner
  appears exactly there rather than hiding the approximation in the background.
- The **relationship proving state** — `proving_source_linked`, `source_linked_partial`,
  `runtime_evidence_only`, `convention_only`, `no_proving_source`, or `unknown_proving`.

## Derived truth (never asserted)

Both components carry a derived **certainty posture** computed by
`resolve_convention_diagnostic_posture` and `resolve_derived_relationship_posture` from the frozen
classes:

- **Certainty posture** — `exact_from_source`, `runtime_confirmed`, `heuristic`, or
  `partial_or_unresolved`. This is the acceptance-criteria axis: a user can tell at a glance whether
  the claim is exact, runtime confirmed, a heuristic guess, or only partial. Only a `verified`
  confidence (diagnostic) or an `exact_from_source` class (banner) reads as exact; a heuristic
  suspicion or an inferred link may never read as exact.

Because these are derived, a heuristic suspicion can never read as an exact contract fact and an
inferred relationship can never read as an exact one.

## Proving source (never a hidden parallel model)

Every row and banner links back to a canonical proving source — one of `source_file`,
`source_symbol`, `runtime_trace`, or `docs_anchor` — rather than acting like a hidden parallel model.
A component with a source form must link to a resolvable proving source; an ungrounded diagnostic
(unknown confidence) or an unresolved / unknown-proving relationship (which has no source form) must
set `no_proving_source` and name why, so it can never pretend to link to a source it does not have.

## Hard invariants

Every diagnostic row keeps these `false`: `lets_heuristic_masquerade_as_exact`,
`collapses_distinct_diagnostics_into_generic_warning`, `acts_as_hidden_parallel_model`, and
`invents_alternate_state_label`. Every banner keeps these `false`:
`lets_heuristic_masquerade_as_exact`, `hides_approximation_in_background`,
`acts_as_hidden_parallel_model`, and `invents_alternate_state_label`.

The validator additionally rejects any component whose heuristic or partial posture claims
exact-from-source (`heuristic_claims_exact`).

## Export safety

Raw file bodies, raw source trees, pasted local paths, repository URLs, credentials, and secrets
never cross the export boundary. The canonical proof bundle lives at
`artifacts/release/m5-convention-diagnostic-derived-relationship-proof/` and the scenario fixtures at
`fixtures/ui/m5-convention-diagnostic-derived-relationship-controls/`, both regenerated
deterministically from the seed builders via
`cargo run -p aureline-templates --example dump_convention_diagnostic_relationship_controls`.
