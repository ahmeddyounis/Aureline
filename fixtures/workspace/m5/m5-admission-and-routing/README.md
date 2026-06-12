# Fixtures: M5 admission and first-useful-work routing

This directory contains fixture metadata for the `m5_admission_and_routing_packet`.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-admission-and-routing.json`

and is validated by `schemas/workspace/m5-admission-and-routing.schema.json` and the typed model
in the `aureline-workspace` crate (`m5_admission_and_routing`).

## Coverage

- One admission checkpoint per M5 wedge: `notebook_workspace`, `data_and_api_workspace`,
  `profiler_workspace`, `framework_pack_workspace`, `docs_workspace`, `companion_workspace`,
  `sync_handoff_workspace`, and `local_folder_workspace`. Every wedge is covered
  (`covers_every_wedge`), so no wedge falls back to a feature-local empty state.
- All six admission classes are exercised and stay distinct: `certified` (2), `probable` (2),
  `mixed` (1), `unknown` (1), `restricted` (1), and `missing_prerequisite` (1). Only the two
  certified checkpoints set `presented_as_certified_support`; probable and mixed never read as
  certified support.
- Every detection source label is canonical for its class
  (`DetectionSource::is_canonical_for`), and no admission class or bundle recommendation
  out-ranks its archetype confidence (`AdmissionClass::permitted_under`,
  `BundleRecommendationSource::permitted_under`).
- All five first-useful-work routes are exercised: `guided_setup_offered`, `open_minimal`,
  `local_safe_fallback`, `set_up_later`, and `restricted_browse`. No route forces a wizard.
- All three setup timings are exercised — `blocking_now` (1), `recommended_soon` (6), and
  `optional_later` (6). Non-blocking setup is deferrable while minimal local-safe work stays
  available; seven of eight checkpoints keep local-safe work, and only the policy-`restricted`
  local folder is browse-only. The `missing_prerequisite` profiler keeps local-safe work even
  behind a `blocking_now` runtime gate.
- All four guardrail flags (`forces_wizard`, `auto_installs_packs`,
  `rewrites_layout_without_review`, `widens_trust_without_review`) are `false` on every
  checkpoint, and every setup item carries `auto_runs: false`. Four items are review-gated.
- Every checkpoint carries a `routing_provenance_ref`, an `archetype_evidence_ref`, and a
  `bundle_recommendation_ref` so support and help surfaces can reconstruct how the workspace was
  admitted and routed, plus diagnostics, support-export, help-surface, docs-badge, and
  release-evidence refs.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, full wedge coverage, the four guardrail flags, the no-auto-run guard, the
class/confidence and bundle/confidence ceilings, detection-source canonicality, the
certified-presentation and local-safe recomputations, route consistency, complete provenance,
per-item consistency, required caveats, and the recomputed summary. The unit tests in
`crates/aureline-workspace/src/m5_admission_and_routing/tests.rs` assert the embedded packet
validates clean and that every wedge, admission class, route, and setup timing is exercised.
