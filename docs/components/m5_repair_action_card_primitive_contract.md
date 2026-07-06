# M5 Repair Action Card and Repair Preview Row Primitive Contract

The M5 repair-action-card / repair-preview-row primitive is one reusable component that
explains, on every claimed M5 recovery surface, **what a Doctor or support fix will
change, what it will leave untouched, where it runs, and whether reversal is exact,
compensating, regenerate, or manual** — before any mutation executes.

It narrows the `repair_action_card` family frozen in the M5 runtime-boundary component
matrix (`schemas/ui/m5-runtime-boundary-components.schema.json`) into a working resolver
plus a cross-surface parity matrix, and adds the reusable repair-preview row that carries
the four pre-execution truths.

- Packet schema: `schemas/ui/m5-repair-action-card.schema.json`
- Companion preview-row fragment: `schemas/ui/m5-repair-preview-row.schema.json`
- Support export (canonical): `artifacts/release/m5-repair-action-card-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-repair-action-card-proof/matrix.csv`
- Markdown report: `artifacts/components/m5-repair-action-card-primitive.md`
- Protected fixtures: `fixtures/ui/m5-repair-action-card-primitive/`
- Implementation: `crates/aureline-shell/src/implement_the_m5_repair_action_card_and_repair_preview_row_impact_scope_target_boundary_and_reversal_class_primitive/`

## Resolver

`resolve_repair_action(&M5RepairActionResolutionInput) -> Result<M5ResolvedRepairAction, …>`
takes one repair action's class, opaque target / scope, blast radius, host boundary,
reversibility, trust requirement, changed-versus-unchanged state classes, and the four
preview truths (`preview only`, `approval required`, `rerunnable`, `factory reset out of
band`), and derives:

- **Target boundary** — `local_target` / `remote_target` / `managed_target`, derived from
  the host boundary so a remote or managed repair is never masked as local.
- **Reversal honesty** — `reversal_is_exact` is true only for a checkpoint rollback; a
  backup, partial, irreversible, or manual reversal is never presented as exact.
- **Approval gate** — `requires_approval` combines the explicit `approval_required` flag
  with a policy / managed trust requirement.
- **Action-label class** — an honest, non-generic label so a preview-only, factory-reset,
  policy-gated, off-device, or non-exact repair never reads like a generic `Fix now`.
  Resolution order: factory reset → preview only → policy approval → off-device (remote /
  managed) → non-exact → ordinary local apply.
- **Available actions** — preview and cancel are always present so the blast radius and
  reversibility can be reviewed before any mutation runs; approve, apply, rollback, and
  factory-reset appear only where honest.

The resolver rejects empty titles / scopes, forbidden material (`://`, `secret`,
`password`, `api_key`, `bearer `), duplicate or overlapping change classes, and a
no-writes preview that claims changed state classes.

## Controlled vocabulary

| Group | Tokens |
| --- | --- |
| Recovery surfaces (9) | `project_doctor_panel`, `doctor_repair_card`, `guided_repair_wizard`, `support_bundle_repair_row`, `environment_repair_prompt`, `toolchain_repair_card`, `remote_host_repair_card`, `repair_preview_sheet`, `activity_center_repair` |
| Repair classes (8) | `reinstall_toolchain`, `repair_environment_config`, `rebuild_index`, `clear_cache`, `repair_permissions`, `regenerate_lockfile`, `reconnect_remote_target`, `factory_reset_component` |
| Blast radii (5, reused) | `no_writes_preview`, `workspace_scoped`, `toolchain_scoped`, `host_environment_scoped`, `multi_target_scoped` |
| Target boundaries (3) | `local_target`, `remote_target`, `managed_target` |
| Reversibility classes (5, reused) | `fully_reversible_checkpoint`, `reversible_with_backup`, `partially_reversible`, `irreversible_confirmed`, `reversal_requires_manual_steps` |
| Trust requirements (5) | `no_elevation`, `local_confirmation`, `admin_elevation`, `policy_approval_required`, `managed_by_administrator` |
| Change classes (8) | `toolchain_binaries`, `workspace_config`, `cache_artifacts`, `index_data`, `file_permissions`, `lockfile_state`, `remote_session_state`, `user_source_files` |
| Action-label classes (6) | `apply_local_reversible`, `preview_only`, `request_policy_approval`, `review_off_device_repair`, `apply_non_exact_repair`, `open_factory_reset_out_of_band` |
| Card parts (7; 5 mandatory) | `repair_class_label`, `target_scope_label`, `blast_radius_badge`, `target_boundary_badge`, `trust_requirement_badge`, `reversal_class_badge`, `action_label` |
| Preview-row parts (6; all mandatory) | `preview_only_flag`, `approval_required_flag`, `rerunnable_flag`, `factory_reset_out_of_band_flag`, `changed_class_list`, `unchanged_class_list` |
| Repair actions (6) | `preview_repair`, `request_approval`, `apply_repair`, `rollback_repair`, `open_factory_reset`, `cancel_repair` |
| Export fields (11; 7 mandatory) | `repair_identity`, `repair_class`, `target_scope`, `blast_radius`, `target_boundary`, `reversal_class`, `trust_requirement`, `changed_classes`, `unchanged_classes`, `preview_flags`, `available_actions` |

The blast radii, reversibility classes, host boundaries, accessibility routes,
qualification classes, and downgrade triggers are reused verbatim from the frozen
runtime-boundary matrix; the shell zones, responsive classes, window classes, and consumer
surfaces are reused from the frozen shell-zone matrix. No M5 surface invents a second
repair grammar.

## Hard invariants (per row, all must be false)

- `understates_blast_radius`
- `overstates_reversibility`
- `masks_target_boundary`
- `hides_changed_or_unchanged_classes`

## Acceptance-criterion lints (matrix-wide)

- **`blast_radius_review_unproven`** — at least one worked resolution must be a real
  mutation (a writing blast radius) whose projection carries a preview action and its
  changed classes, proving a user can review blast radius and reversibility before any
  Doctor / support mutation runs.
- **`non_generic_label_unproven`** — at least one worked resolution must earn a
  non-generic action label, proving a remote, policy-gated, or non-exact repair never
  reads like a generic `Fix now`.
- **`changed_unchanged_disclosure_unproven`** — at least one worked resolution must
  disclose both a changed-class list and an unchanged-class list, proving preview
  artifacts identify both classes so users can judge risk correctly.

## Support / export parity

The support export carries the preview and reversal vocabulary — the four preview truths,
the reversal class, the target boundary, and both change-class lists — for every worked
resolution, so the same repair can be explained outside the live UI. The checked-in export
is minted only by the headless emitter
(`aureline_shell_m5_repair_action_card_primitive`) and is asserted against the seed
builder in tests, so the in-code matrix, the artifact, and the fixtures never drift.
