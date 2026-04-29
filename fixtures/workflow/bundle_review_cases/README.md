# Bundle change-preview fixtures

Seed corpus for the contract frozen in
[`/docs/workflow/bundle_change_review_contract.md`](../../../docs/workflow/bundle_change_review_contract.md)
and the schema at
[`/schemas/workflow/bundle_change_preview.schema.json`](../../../schemas/workflow/bundle_change_preview.schema.json).

Each file is a single YAML `bundle_change_preview_record`. The
fixtures exercise the closed `preview_intent_class`,
`preview_origin_class`, `preview_state_class`, `change_kind`,
`change_axis`, `settings_or_token_axis`,
`trust_or_permission_axis`, `compatibility_axis`,
`side_effect_class`, `review_action_id`,
`action_rendered_state`, `rollback_checkpoint_linkage_class`,
`rollback_path_class`, and
`non_reversibility_justification_class` vocabularies plus the
review-sheet action-set and rollback-checkpoint linkage rules.

Every fixture:

- emits exactly one preview record with a stable `preview_id`
  and a monotonic `minted_at`;
- declares the `target_bundle` identity ref and (when
  applicable) the `previous_bundle` identity ref;
- emits all five change-entry arrays (`component_change_entries`,
  `settings_or_token_change_entries`,
  `trust_or_permission_change_entries`,
  `compatibility_change_entries`, `side_effect_envelope`) as keys
  — empty arrays are valid but missing keys are not;
- emits all six review-sheet actions (`review.compare`,
  `review.confirm`, `review.cancel`, `review.set_up_later`,
  `review.inspect_change_source`,
  `review.create_rollback_checkpoint`) with a typed
  `action_rendered_state` and `keyboard_reachable = true`;
- emits exactly one `rollback_checkpoint_linkage` block whose
  `linkage_class` either resolves to a single attributable
  rollback handle (workspace / profile / user / appearance, or
  paired workspace + appearance) or is explicitly marked
  non-reversible with a `non_reversibility_justification_class`,
  a justification summary ref, and a `decision_row_ref`;
- carries no raw signing keys, raw certificate material, raw
  repository URLs, raw absolute paths, raw secrets, raw token
  values, raw setting bodies, or raw user-authored content.

## Cases

| Fixture | `preview_intent_class` | `bundle_class` | `rollback_checkpoint_linkage_class` | Notable invariants exercised |
| --- | --- | --- | --- | --- |
| [`fresh_install_certified_launch_bundle.yaml`](./fresh_install_certified_launch_bundle.yaml) | `fresh_install_preview` | `launch_bundle` | `single_attributable_workspace_checkpoint` | All thirteen component change-axes; trust grant on workspace; full side-effect envelope; six review actions enabled. |
| [`update_paired_appearance_and_workspace.yaml`](./update_paired_appearance_and_workspace.yaml) | `update_to_newer_revision_preview` | `launch_bundle` | `paired_workspace_and_appearance_checkpoint` | `revision_bumped` and `unchanged_visible` change kinds; `appearance_theme_package` and `appearance_token_overlay` deltas; mixed visual + non-visual axes. |
| [`imported_user_blocked_pending_review.yaml`](./imported_user_blocked_pending_review.yaml) | `fresh_install_preview` | `imported_user_bundle` | `single_attributable_workspace_checkpoint` | Ingress review blocked; `review.confirm` `visible_disabled` with `disabled_reason_code = trust_review_required`; `change_kind = blocked_pending_review`. |
| [`local_draft_non_reversible.yaml`](./local_draft_non_reversible.yaml) | `fresh_install_preview` | `local_draft_bundle` | `non_reversible_with_justification` | `filesystem_scaffold_overwrite_existing`; `reversible_in_rollback = false`; `local_draft_no_prior_state` justification with decision row ref; `review.create_rollback_checkpoint` `visible_disabled`. |
| [`set_up_later_deferred.yaml`](./set_up_later_deferred.yaml) | `set_up_later_deferred_preview` | `org_approved_bundle` | `single_attributable_workspace_checkpoint` | `preview_state_class = preview_set_up_later_deferred`; mirror-only packaging; `review.confirm` `visible_disabled` with `rollback_checkpoint_unavailable` until resume. |
| [`appearance_only_visual_update.yaml`](./appearance_only_visual_update.yaml) | `update_to_newer_revision_preview` | `design_partner_bundle` | `single_appearance_checkpoint_only_for_visual` | Visual-only update through the appearance-checkpoint contract; rollback path `surface_reload_then_revert`. |
| [`apply_failed_rolled_back.yaml`](./apply_failed_rolled_back.yaml) | `fresh_install_preview` | `launch_bundle` | `single_attributable_workspace_checkpoint` | Apply failed mid-flight; preview record persists with the same checkpoint linkage; post-rollback action ids hidden where not applicable. |

The case-index manifest at
[`./manifest.yaml`](./manifest.yaml) maps each file to the
contract sections it exercises.
