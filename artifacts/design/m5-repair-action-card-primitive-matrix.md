# M5 Repair Action Card and Repair Preview Row Primitive — Design Matrix

Task **M05-857** / batch **B100**. Design-side companion to the machine-minted proof under
`artifacts/release/m5-repair-action-card-proof/` and the contract at
`docs/components/m5_repair_action_card_primitive_contract.md`.

One reusable repair primitive (action card + preview row) projected across nine claimed M5
recovery surfaces so every Doctor / support fix explains **what changes, what stays
untouched, where it runs, and how reversible it is** — before any mutation executes.

## Surfaces × shell zone × qualification

| # | Recovery surface | Shell zone | Qualification | Worked resolutions |
|---|------------------|------------|---------------|--------------------|
| 1 | Project Doctor Panel | `main_workspace` | stable | local exact-reversible env-config apply; no-writes index preview |
| 2 | Doctor Repair Card | `right_inspector` | stable | non-exact cache clear; policy-gated toolchain reinstall (request approval) |
| 3 | Guided Repair Wizard | `transient_overlay` | stable | remote host-env review (partial); local lockfile apply |
| 4 | Support-Bundle Repair Row | `bottom_panel` | stable | local permission apply; multi-target factory reset (out of band, manual reversal) |
| 5 | Environment Repair Prompt | `title_context_bar` | stable | managed irreversible env repair (request approval); container index review |
| 6 | Toolchain Repair Card | `right_inspector` | stable | non-exact toolchain reinstall; no-writes lockfile preview |
| 7 | Remote-Host Repair Card | `title_context_bar` | stable (Beta in narrowed fixture) | remote reconnect review; remote permission repair (explicit approval) |
| 8 | Repair Preview Sheet | `transient_overlay` | stable (Preview in narrowed fixture) | no-writes cache preview; local review-before-apply (both change lists) |
| 9 | Activity-Center Repair | `activity_rail` | stable | sandboxed index review; local cache apply |

## Anatomy

- **Repair-action-card parts** (mandatory ★): repair_class_label ★, target_scope_label,
  blast_radius_badge ★, target_boundary_badge ★, trust_requirement_badge,
  reversal_class_badge ★, action_label ★.
- **Repair-preview-row parts** (all mandatory ★): preview_only_flag ★,
  approval_required_flag ★, rerunnable_flag ★, factory_reset_out_of_band_flag ★,
  changed_class_list ★, unchanged_class_list ★.

Every preview row keeps the four pre-execution truths and both change-class lists so a
user can judge risk before a fix runs.

## Derived state vocabulary

- **Repair class** (8): reinstall_toolchain, repair_environment_config, rebuild_index,
  clear_cache, repair_permissions, regenerate_lockfile, reconnect_remote_target,
  factory_reset_component.
- **Blast radius** (reused): no_writes_preview, workspace_scoped, toolchain_scoped,
  host_environment_scoped, multi_target_scoped.
- **Host boundary → target boundary**: local_host → local_target; remote_ssh_host /
  virtual_machine_host → remote_target; container_host / managed_workspace_host /
  wasm_sandbox_host → managed_target.
- **Reversibility** (reused): fully_reversible_checkpoint (exact),
  reversible_with_backup, partially_reversible, irreversible_confirmed,
  reversal_requires_manual_steps.
- **Trust requirement**: no_elevation, local_confirmation, admin_elevation,
  policy_approval_required, managed_by_administrator (last two ⇒ requires approval).
- **Change classes**: toolchain_binaries, workspace_config, cache_artifacts, index_data,
  file_permissions, lockfile_state, remote_session_state, user_source_files.
- **Action-label class** (derived, priority): open_factory_reset_out_of_band → preview_only
  → request_policy_approval → review_off_device_repair → apply_non_exact_repair →
  apply_local_reversible.
- **Actions**: preview_repair (always), request_approval, apply_repair, rollback_repair,
  open_factory_reset, cancel_repair (always).

## Acceptance-criterion mapping

| AC | Guarantee | Proof lint |
|----|-----------|------------|
| AC1 | Review blast radius and reversibility before any mutation runs | `blast_radius_review_unproven` |
| AC2 | Remote / policy-gated / non-exact repairs never read like `Fix now` | `non_generic_label_unproven` |
| AC3 | Preview artifacts identify both changed and unchanged classes | `changed_unchanged_disclosure_unproven` |

## Hard invariants (all `false` on every row)

`understates_blast_radius`, `overstates_reversibility`, `masks_target_boundary`,
`hides_changed_or_unchanged_classes`.
