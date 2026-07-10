# M5 generator-preview sheets and run-config scaffold cards

This contract implements the frozen `generator_preview_sheet` and `run_config_scaffold_card`
component families from the [M5 framework-component matrix](m5_framework_component_matrix.md) as two
reusable, co-equal control vectors — the **generator-preview sheet** and the **run-config scaffold
card** — so a framework-generated write or launch is review-first before it touches user code or
dispatches execution.

The Rust validator in
`crates/aureline-templates/src/implement_generator_preview_sheets_and_run_config_scaffold_cards_with_generator_version_file_effect_classes_dependency_config_impact_rollback_or_regenerate_posture_required_toolchains_and_local_container_ssh_managed_target_truth`
is the authoritative gate; the
[boundary schema](../../../schemas/ui/m5-generator-preview-run-config-controls.schema.json)
documents the export shape.

## What the generator-preview sheet names

A generator-preview sheet names, before a user applies it:

- **Generator identity / version** — the generator id, name, and version.
- **Parameters** — the parameters the generator will run with.
- **Created versus modified paths** — the file-effect class (`creates_file`, `modifies_file`,
  `creates_and_modifies`, or `no_file_change`) and the created / modified path counts, which must
  agree.
- **Managed versus user-owned files** — the file-ownership class (`managed_generated`, `user_owned`,
  `mixed_ownership`, or `unknown_ownership`) and its label, so a sheet never hides whether it writes
  managed-generated files or user-owned code.
- **Dependency / config impact** — the frozen generator impact class (`file_write`,
  `dependency_change`, `config_change`, `script_or_task_change`, `no_change`, `unknown_impact`) plus
  the dependency / config impact label, which is required whenever the sheet has a side effect.
- **Rollback or regenerate posture** — the recovery path (see below).

## What the run-config scaffold card names

A run-config scaffold card names, before a convenience action dispatches execution:

- **Target kind** — `web_app`, `api_server`, `cli_tool`, `test_suite`, `background_job`, or
  `unknown_target`, plus the target label.
- **Environment / profile** — the launch-profile class (`development`, `debug`, `production`, `test`,
  or `custom_profile`) plus the environment / profile label.
- **Launch command** — the exact launch command, always required so it is explicit before dispatch.
- **Required toolchain** — the required toolchain label plus its readiness (`toolchain_ready`,
  `toolchain_missing`, `toolchain_version_mismatch`, or `toolchain_unknown`), so which toolchain is
  required is visible before a convenience action runs.
- **Execution boundary** — the frozen execution boundary class (`local_process`, `container`,
  `ssh_remote`, `managed_workspace`, `cloud_remote`, or `unknown_boundary`) and a derived
  `is_local_execution` flag, so where the code will run never hides behind framework convenience
  language.

## Derived truth (never asserted)

Both components carry a derived **write-effect posture** computed by
`resolve_generator_preview_posture` (from the frozen impact class and apply posture) and
`resolve_run_config_scaffold_posture` (from the frozen mutation class):

- **Write-effect posture** — `no_op_preview`, `review_required_write`, `reversible_applied`, or
  `unknown_or_blocked`. This is the acceptance-criteria axis: a user can tell at a glance whether an
  action is a genuine no-op, a review-required write, a reversible applied write, or an unknown /
  blocked one. Any generator that changes files, dependencies, or config, and any run-config scaffold
  that creates / edits config or adds a dependency, has a side effect and can never claim a no-op
  write.

Because these are derived, a generator can never imply a safe or no-op write when it changes files,
dependencies, or config.

## Recovery (never a write without an undo path)

Every component keeps its recovery path explicit — one of `rollback`, `regenerate`,
`rollback_and_regenerate`, `forward_fix_only`, or `no_recovery_needed`. A component with a reversible
recovery path (rollback and / or regenerate) must name a reversible recovery kind and its reference; a
component without one — a blocked write, or an unknown action — must set a non-reversible kind and name
why it has no automatic undo, so it can never fake a recovery path it does not have.

## Hard invariants

Every generator-preview sheet keeps these `false`: `implies_no_op_write_without_review`,
`hides_dependency_or_config_impact`, `omits_rollback_or_regenerate_path`, and
`invents_alternate_state_label`. Every run-config scaffold card keeps these `false`:
`implies_no_op_write_without_review`, `hides_execution_boundary_or_toolchain`,
`omits_rollback_or_regenerate_path`, and `invents_alternate_state_label`.

The validator additionally rejects any component with a side effect that claims a no-op write
(`write_claims_no_op`) and any run-config card whose `is_local_execution` flag disagrees with its
execution boundary (`execution_boundary_misrepresented`).

## Export safety

Raw file bodies, raw generated trees, pasted local paths, repository URLs, credentials, and secrets
never cross the export boundary. The canonical proof bundle lives at
`artifacts/release/m5-generator-preview-run-config-proof/` and the scenario fixtures at
`fixtures/ui/m5-generator-preview-run-config-controls/`, both regenerated deterministically from the
seed builders via
`cargo run -p aureline-templates --example dump_generator_preview_run_config_controls`.
