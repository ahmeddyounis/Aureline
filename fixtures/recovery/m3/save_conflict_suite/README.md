# Difficult filesystem, external-change, and save-conflict regression suite

Protected fixture corpus for the regression-suite that runs the difficult
filesystem-identity, external-change, and save-conflict scenarios on the
three claimed desktop platform rows (`linux_desktop`, `macos_desktop`,
`windows_desktop`). Each fixture is one
`save_conflict_suite_case_record` bound to:

- one scenario class from the closed list:
  `external_change`, `save_conflict`, `permission_loss`, `alias_drift`,
  `difficult_save_path`,
- one claimed platform-row class,
- one anchor `filesystem_identity_beta_case` fixture under
  `/fixtures/recovery/m3/filesystem_identity/` that proves the underlying
  identity contract,
- one expected save / conflict-resolution behavior,
- one expected regression outcome (`pass`, `downgrade_required`,
  `blocked_until_fix`), and
- one closed downgrade label from the suite vocabulary.

A failing row downgrades using the closed `downgrade_label` list; no
ad-hoc vocabulary is admitted. Open gaps are drawn from the closed
`open_gap_class` enumeration so platform-specific gaps stay reviewable.

Boundary schema:
[`schemas/recovery/save_conflict_suite_beta.schema.json`](../../../../schemas/recovery/save_conflict_suite_beta.schema.json).

Crate consumer:
[`crates/aureline-vfs/src/save_conflict_suite/mod.rs`](../../../../crates/aureline-vfs/src/save_conflict_suite/mod.rs).

Reviewer matrix doc:
[`docs/state/m3/save_conflict_beta_matrix.md`](../../../../docs/state/m3/save_conflict_beta_matrix.md).

Baseline report:
[`artifacts/support/m3/save_conflict_report.md`](../../../../artifacts/support/m3/save_conflict_report.md).
