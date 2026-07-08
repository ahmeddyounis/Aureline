# M5 shared-component-state taxonomy surface certification contract (M05-939)

Closing surface-certification capstone over the frozen M5 **shared-component-state-taxonomy /
interactive-state / selection-or-lock-state / degraded-state-application** component matrix
(M05-932). Where the freeze matrix defines the four reusable component families, the M05-933..936
primitive lanes narrow each one, the M05-937 consumer lane proves they are reusable across the
claimed shell / command / search / review / settings / provider / test / support consumers, and the
M05-938 accessibility / auto-narrowing capstone certifies keyboard / screen-reader / CLI / export
parity per family, this lane **certifies** that the shared component-state taxonomy truth holds on
every claimed M5 control, collection, and recovery surface — and auto-narrows any surface that
cannot sustain it.

- Module:
  `crates/aureline-design-system/src/certify_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_truth_on_every_claimed_m5_control_collection_and_recovery_surface`
- Boundary schema: `schemas/ui/m5-shared-component-state-taxonomy-certification.schema.json`
- Support export (canonical): `artifacts/release/m5-shared-component-state-taxonomy-certification/support_export.json`
- Matrix CSV: `artifacts/release/m5-shared-component-state-taxonomy-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-shared-component-state-taxonomy-certification/report.md`
- Fixtures (byte-identical to the artifacts): `fixtures/ui/m5-shared-component-state-taxonomy-certification/`

## What it certifies

The packet is keyed on the claimed **surface** a user interacts with, selects on, or recovers from,
not on the reusable component family it renders. The eight certified surfaces are the control
affordance, the dense collection, the blocked-action prompt, the settings / capability sheet, the
activity / recovery view, the command palette, the support / export bundle, and the CLI.

Each row certifies one surface across six truth axes — **visual, keyboard, screen-reader,
CLI/export, degraded-state, and state-boundary provenance** — and derives one verdict:

- **green** — every axis certified, the claimed state tier delivered;
- **yellow** — a truth axis is not current and the state claim narrows visibly to the weakest
  supported ceiling, with a bound reason and a frozen downgrade trigger;
- **red** — a degraded axis hides behind a full-truth claim inherited from a healthier state lane,
  CLI/export parity drops, lineage is dropped, or the narrowing is inconsistent. Red blocks release.

## Invariants

- **A degraded axis must produce a visible claim narrowing.** A surface that keeps an
  `ExactStateTruth` / `ReviewableStateGuidance` claim while its state cause is unresolved, its lock /
  read-only / disabled owner is unresolved, its degraded / warning / error recovery is unavailable,
  or its accessibility / export proof is stale, is over-claiming and blocks.
- **State truth never loses lineage.** A narrowed surface always preserves its state-cause / owner /
  block-reason / recovery lineage continuity rather than dropping it between a control, a dense
  collection, and a recovery view.
- **CLI/export parity is always-on.** Every certified surface must reconstruct the same typed-state
  / cause / owner / block-reason / recovery truth as text / JSON / Markdown from the same component
  identity; a screenshot-only export is prohibited.
- **One canonical bundle.** Every row cites exactly one canonical shared-component-state proof
  bundle (`artifacts/release/m5-shared-state-taxonomy-proof/support_export.json`) rather than cloning
  per-surface evidence.
- **Metadata-only.** Raw state copy, captured surface bodies, and credential-bearing material never
  cross this boundary.

## Certified matrix

Four surfaces deliver their claim (green) and four auto-narrow a not-current truth axis to a weaker
state ceiling (yellow); none hide drift (red). The four yellow surfaces certify the four spec
narrowing conditions — unresolved cause, unresolved owner, unavailable recovery, and stale proof.

| Surface | Claimed | Certified | Status | Binding axis |
| --- | --- | --- | --- | --- |
| control-affordance | exact_state_truth | exact_state_truth | green | — |
| dense-collection | exact_state_truth | exact_state_truth | green | — |
| command-palette | exact_state_truth | exact_state_truth | green | — |
| support-export | reviewable_state_guidance | reviewable_state_guidance | green | — |
| settings-capability-sheet | exact_state_truth | cause_narrowed_projection | yellow | state_boundary_provenance |
| blocked-action-prompt | exact_state_truth | owner_narrowed_projection | yellow | state_boundary_provenance |
| activity-recovery-view | exact_state_truth | recovery_narrowed_projection | yellow | state_boundary_provenance |
| cli-headless | exact_state_truth | stale_proof_projection | yellow | degraded_state |

All four frozen families (shared-component-state-taxonomy, interactive-state,
selection-or-lock-state, degraded-state-application) are certified on some surface.

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are generated from the single seeded
builder so they stay byte-aligned with the code:

```
GEN_STATE_CERT_ARTIFACTS=1 cargo test -p aureline-design-system --lib -- generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the
seeded builder.
