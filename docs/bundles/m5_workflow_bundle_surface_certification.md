# M5 workflow-bundle component surface certification (M05-851)

This contract is the certification capstone that **closes** the M5 workflow-bundle
component lane (batch B99). Where the freeze matrix
(`schemas/ui/m5-workflow-bundle-component-matrix.schema.json`) defines the reusable
start-center bundle card, certified-archetype badge group, bundle detail page,
install/update review sheet, drift banner, local-override row, rollback/remove card,
class-disclosure card, and claim-narrowing row primitives, the M05-845–849
implementation lanes resolve their per-surface truth, and the M05-850 accessibility
capstone certifies keyboard / screen-reader / CLI / export parity per family,
**M05-851 keys on the claimed M5 stack-entry / migration surface** and certifies that
the shared component family behaves consistently on every consumer.

- **Schema:** `schemas/ui/m5-workflow-bundle-surface-certification.schema.json`
- **Module:** `crates/aureline-workspace/src/certify_workflow_bundle_launch_badge_detail_review_drift_and_rollback_component_truth_across_claimed_stack_entry_and_migration_surfaces/`
- **Canonical bundle every surface cites:**
  `artifacts/release/m5-workflow-bundle-component-proof/support_export.json`
- **Release proof:** `artifacts/release/m5-workflow-bundle-surface-certification-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-workflow-bundle-surface-certification/`

## What a row certifies

Each row keys on one of nine `claimed_surface` values — the six interactive bundle
consumers (`start_center_picker`, `onboarding_flow`, `migration_center`, `docs_help`,
`diagnostics`, `cli_headless`) plus three release-evidence surfaces
(`support_export_replay`, `docs_help_embeds`, `release_proof`). A surface declares the
component groups it consumes and carries one truth axis per group:

| Component group | Frozen families | Truth axis |
| --- | --- | --- |
| `launch_wedge` | start-center bundle card, certified-archetype badge group | `launch_wedge_truth` |
| `detail_review` | bundle detail page, install/update review sheet | `detail_review_truth` |
| `drift_override` | drift banner, local-override row | `drift_override_truth` |
| `rollback_remove` | rollback/remove card | `rollback_remove_truth` |
| `class_disclosure` | class-disclosure card, claim-narrowing row | `class_disclosure_truth` |

The five groups together cover all nine frozen `M5WorkflowBundleComponentFamily`
values. Each gated axis is `certified`, `disclosed_narrowed`, `blocked`, or
`not_applicable` (the last exactly when the surface does not consume that group). An
always-applicable `export_parity` axis certifies the support / release export.

## Certification statuses

- **`certified` (green).** Every consumed axis is certified and the surface asserts
  its declared bundle-support claim with no narrowing.
- **`narrowed_disclosed` (yellow).** At least one consumed axis is
  `disclosed_narrowed`, and the surface auto-narrows its bundle-support claim
  (`certified` → `supported` → `limited` → `retest_pending` → `imported` →
  `mirror_only` → `offline_cache_only` → `policy_blocked`) with a `claim_auto_narrow`
  block that names the binding component group and its frozen downgrade trigger and
  preserves the canonical component identity.
- **`blocked` (red).** The surface hides drift, over-asserts support, drops export
  truth, carries a non-current distribution path without narrowing, or fails to
  reference its canonical component families. Blocked surfaces may not ship.

## Acceptance criteria

- **AC1 — certify or auto-narrow.** `claim_is_honest` requires the effective claim to
  never exceed the declared claim; a certified surface carries no narrow block, and a
  narrowed surface carries an honest block bound to a reduced consumed group with its
  frozen trigger. An unsupported or degraded bundle path narrows visibly instead of
  inheriting a full-truth label from a healthier lane.
- **AC2 — degraded paths narrow visibly.** `unsupported_paths_narrowed` forces a
  narrowed claim whenever any `compatibility_notes` entry across the
  native / mirror / offline / managed / imported distribution paths is not `current`;
  `export_preserves_truth` keeps the support / release export screenshot-free with
  every mandatory export field.
- **AC3 — anchored to a reusable component family.** Every row cites the one canonical
  `certification_bundle_ref` (the frozen M05-844 release proof), references the
  canonical families of each consumed group (`references_canonical_families`), and
  keeps each gated axis applicable exactly when its group is consumed
  (`axes_match_consumed_groups`). M5 stack-entry and migration claims are anchored to
  this reusable component family rather than feature-local registry or onboarding
  chrome.

## Regenerating the artifacts

```sh
GEN_BUNDLE_SURFACE_CERT_ARTIFACTS=1 cargo test -p aureline-workspace generate_artifacts
```

The gated `generate_artifacts` test writes the release proof
(`support_export.json`, `matrix.csv`, `report.md`) and byte-identical fixtures under
`fixtures/ui/m5-workflow-bundle-surface-certification/`. The checked-in support export
is the `include_str!` canonical for `current_m5_bundle_surface_cert_export()`; the
`on_disk_export_matches_builder` test asserts it stays byte-aligned with
`seeded_m5_bundle_surface_cert_packet()`.
