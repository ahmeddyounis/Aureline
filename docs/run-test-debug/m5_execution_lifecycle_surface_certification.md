# M5 execution-lifecycle component surface certification (M05-827)

This contract is the certification capstone that **closes** the M5 execution-lifecycle
component lane. Where the freeze matrix
(`schemas/ui/m5-execution-lifecycle-component-matrix.schema.json`) defines the reusable
run/attempt-header, input-request-prompt, artifact-publish-row, rerun-comparison-sheet,
and debug-hierarchy primitives, the 821–824 implementation lanes resolve their
per-surface truth, and the M05-826 accessibility capstone certifies keyboard /
screen-reader / CLI / export parity per family, **M05-827 keys on the claimed M5
execution surface** and certifies that the shared component family behaves
consistently on every consumer.

- **Schema:** `schemas/ui/m5-execution-lifecycle-surface-certification.schema.json`
- **Module:** `crates/aureline-runtime/src/certify_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_truth_across_claimed_execution_surfaces/`
- **Canonical bundle every surface cites:**
  `artifacts/release/m5-execution-lifecycle-component-proof/support_export.json`
- **Release proof:** `artifacts/release/m5-execution-lifecycle-surface-certification-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-execution-lifecycle-surface-certification/`

## What a row certifies

Each row keys on one of twelve `claimed_surface` values — the nine interactive
execution consumers (`task_execution`, `test_execution`, `request_execution`,
`database_execution`, `notebook_execution`, `preview_execution`, `ai_execution`,
`publish_execution`, `debug_execution`) plus three release-evidence surfaces
(`support_export_replay`, `docs_help_embeds`, `release_proof`). A surface declares the
component groups it consumes and carries one truth axis per group:

| Component group | Frozen families | Truth axis |
| --- | --- | --- |
| `run_attempt` | run/attempt header | `run_attempt_truth` |
| `input_request` | input-request prompt | `input_request_truth` |
| `artifact_publish` | artifact-publish row | `artifact_publish_truth` |
| `rerun_review` | rerun comparison sheet | `rerun_review_truth` |
| `debug_hierarchy` | debug session header, thread/process tree, dump/crash artifact card | `debug_hierarchy_truth` |

Each gated axis is `certified`, `disclosed_narrowed`, `blocked`, or `not_applicable`
(the last exactly when the surface does not consume that group). An always-applicable
`export_parity` axis certifies the support / release export.

## Certification statuses

- **`certified` (green).** Every consumed axis is certified and the surface asserts
  its declared interactive claim with no narrowing.
- **`narrowed_disclosed` (yellow).** At least one consumed axis is
  `disclosed_narrowed`, and the surface auto-narrows its interactive claim
  (`full_interactive` → `review_required` → `read_only` → `inspect_only`) with a
  `claim_auto_narrow` block that names the binding component group and its frozen
  downgrade trigger and preserves the canonical component identity.
- **`blocked` (red).** The surface hides drift, over-asserts control, drops export
  truth, carries a non-current execution path without narrowing, or fails to reference
  its canonical component families. Blocked surfaces may not ship.

## Acceptance criteria

- **AC1 — certify or auto-narrow.** `claim_is_honest` requires the effective claim to
  never exceed the declared claim; a certified surface carries no narrow block, and a
  narrowed surface carries an honest block bound to a reduced consumed group with its
  frozen trigger.
- **AC2 — degraded paths narrow visibly.** `unsupported_paths_narrowed` forces a
  narrowed claim whenever any `compatibility_notes` entry across the
  local / remote / container / managed / provider-backed execution paths is not
  `current`; `export_preserves_truth` keeps the support / release export
  screenshot-free with every mandatory export field.
- **AC3 — anchored to a reusable component family.** Every row cites the one canonical
  `certification_bundle_ref`, references the canonical families of each consumed group
  (`references_canonical_families`), and keeps each gated axis applicable exactly when
  its group is consumed (`axes_match_consumed_groups`).

## Regenerating the artifacts

```sh
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_surface_certification -- support \
  > artifacts/release/m5-execution-lifecycle-surface-certification-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_surface_certification -- csv \
  > artifacts/release/m5-execution-lifecycle-surface-certification-proof/matrix.csv
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_surface_certification -- summary \
  > artifacts/release/m5-execution-lifecycle-surface-certification-proof/report.md
```

The checked-in support export is the `include_str!` canonical for
`current_m5_execution_surface_cert_export()`; the `on_disk_export_matches_builder`
test asserts it stays byte-aligned with `seeded_m5_execution_surface_cert_packet()`.
The fixtures under `fixtures/ui/m5-execution-lifecycle-surface-certification/` are
byte-identical copies.
