# M5 experiment run rows and environment fingerprint cards

The experiment run row and the environment fingerprint card are two of the eight governed
experiment components frozen by the
[M5 experiment-component matrix](m5_experiment_component_matrix.md). This lane implements those
two families as two co-equal control vectors in one export-safe packet,
[`ExperimentRunRowEnvironmentFingerprintControlsPacket`](../../crates/aureline-notebook/src/implement_experiment_run_rows_and_environment_fingerprint_cards_with_run_origin_code_revision_execution_target_and_outcome_truth_across_claimed_m5_notebook_and_data_surfaces/mod.rs),
so a claimed M5 notebook, experiment-dashboard, comparison, data-catalog, share-review, or CLI
surface can project a run row and a fingerprint card that make run identity and reproducibility
context **explicit before compare or export** — never inferred, and never widening export scope
or exposing raw production-like data by default.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never
asserted.

### `resolve_run_origin`

Given a run row's run origin kind, the resolver derives an **origin class**:

- `notebook_cell` / `script_task` → `local_run` (first-party)
- `scheduled_task` → `managed_run` (first-party)
- `imported_run` → `imported_run` (must carry an explicit imported note), not first-party
- `manual_attach` → `manually_attached` (must carry an explicit manual-attach note), not
  first-party
- `unknown_origin` → `origin_unknown` (must carry an explicit unknown-origin note), not
  first-party

A user can therefore always tell whether they are looking at a **local, managed, imported, or
manually attached run** before trusting a downstream comparison or share action; an imported,
manually attached, or unknown-origin run can never read as a first-party run.

### `resolve_fingerprint_capture`

Given a fingerprint card's capture state, the resolver derives a **capture class**:

- `captured_complete` → `captured` (reliably captured)
- `captured_partial` → `partially_captured` (must carry an explicit partial note), not reliably
  captured
- `pinned` → `pinned` (reliably captured)
- `captured_missing` / `drifted` / `unavailable` → `uncaptured` (must carry an explicit
  uncaptured note), not reliably captured

A missing, drifted, or unavailable fingerprint can never read as a captured environment, so
reproducibility is never overstated.

## Identity, revision, execution, and deep links

- **Run identity and revision** — every run row names its run id, its origin, its `queued` /
  `running` / `succeeded` / `failed` / `canceled` / `stale` status, its commit or workspace
  revision, its execution origin, and its start/end window, so run identity and revision stay
  **always explicit**.
- **Open / compare / export** — every run row offers the mandatory `open_run`, `compare_runs`,
  and `export_run` actions, plus `open_deep_link`, `inspect_fingerprint`, and `copy_run_id` as
  appropriate.
- **Captured environment** — every fingerprint card names its interpreter or kernel, its
  package/toolchain summary, its execution target, its hardware/profile class (with an explicit
  unavailable note when hardware detail is not available on this build), and its capture
  freshness.
- **Inspect / export** — every fingerprint card offers the mandatory `inspect_fingerprint` and
  `export_fingerprint` actions, plus `open_deep_link`, `compare_environments`,
  `copy_fingerprint_id`, and `pin_environment` as appropriate.
- **Stable deep links** — every next step names a stable `run_object`, `notebook_location`,
  `dataset_catalog_anchor`, or `docs_anchor` deep link with a resolvable reference. A component
  that offers a deep-link action must name a resolvable kind, so a next step is never an
  ephemeral overlay or hidden route.

## Hard invariants

Every component keeps four bools `false`, and validation flags any that is `true`:

- `masks_provenance_or_sensitivity_state` — dataset provenance and sensitivity posture stay
  visible.
- `hides_run_origin_or_revision` — where a run came from and its code revision stay explicit.
- `implies_apples_to_apples_without_parity` — a comparison is never implied comparable without
  parity evidence.
- `invents_alternate_state_label` — no surface invents a second word for a governed origin,
  status, or capture state.

No component widens export scope or exposes raw payloads by default; cached, offline, and
local-only state stays visible.

## Coverage

The checked-in support export exercises every origin class, every run origin kind, and every
run status state across the six seeded run rows, and every capture class, every fingerprint
scope class, and every fingerprint state across the six seeded fingerprint cards.

## Source of truth and artifacts

- Boundary schema: [`schemas/ui/m5-experiment-run-row-environment-fingerprint-controls.schema.json`](../../schemas/ui/m5-experiment-run-row-environment-fingerprint-controls.schema.json)
- Support export: [`artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/support_export.json`](../../artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/support_export.json)
- Matrix CSV: [`artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/matrix.csv`](../../artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/matrix.csv)
- Design report: [`artifacts/design/m5-experiment-run-row-environment-fingerprint.md`](../../artifacts/design/m5-experiment-run-row-environment-fingerprint.md)
- Scenario fixtures: [`fixtures/ui/m5-experiment-run-row-environment-fingerprint-controls/`](../../fixtures/ui/m5-experiment-run-row-environment-fingerprint-controls/)

Regenerate every artifact and fixture from the single seed with the headless emitter:

```sh
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- support-export
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- csv
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- report
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- fixture-run-row-imported
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- fixture-fingerprint-card-uncaptured
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_run_row_environment_fingerprint_primitive -- validate
```
