# Bundle drift / merge / remove-bundle review fixtures

Seed corpus for the contract frozen in
[`/docs/workflow/bundle_drift_and_removal_contract.md`](../../../docs/workflow/bundle_drift_and_removal_contract.md)
and the schema at
[`/schemas/workflow/bundle_drift_row.schema.json`](../../../schemas/workflow/bundle_drift_row.schema.json).

Each file is a single YAML `bundle_drift_row_record`. The
fixtures exercise the closed `drift_state_class`, `drift_axis`,
`drift_subject_kind`, `drift_severity_class`,
`resolve_action_id`, `resolve_blocker_class`,
`asset_ownership_class`, `safe_to_remove_class`,
`retained_local_override_class`, `claim_narrowing_class`,
`successor_bundle_suggestion_class`, `provenance_record_class`,
`drift_row_state_class`, and
`remove_bundle_review_state_class` vocabularies plus the
resolve-action set, the remove-bundle review block, the
claim-narrowing rules, and the recovery-link rule that recovery
rides through the apply-time `rollback_checkpoint_linkage`.

Every fixture:

- emits exactly one drift-row record with a stable
  `drift_row_id` and a monotonic `minted_at`;
- declares the `target_bundle` identity ref, the
  `applied_revision_ref`, the `canonical_revision_ref` (or
  null only when the registry is unreachable and
  `drift_state_class = drift_state_unknown`), and the
  `applied_change_preview_ref` so the row reads the apply-time
  rollback linkage by reference;
- emits exactly one `drift_state_class`, `drift_axis`,
  `drift_subject_kind`, `drift_severity_class`,
  `asset_ownership_class`, `claim_narrowing_class`, and
  `successor_bundle_suggestion` block;
- emits all five resolve actions (`resolve.keep_local`,
  `resolve.adopt_bundle`, `resolve.compare`,
  `resolve.rebase_to_bundle`, `resolve.ignore_this_drift`)
  with a typed `action_rendered_state` and
  `keyboard_reachable = true`; `resolve.adopt_bundle` and
  `resolve.rebase_to_bundle` resolve `destination_ref` to a
  new `bundle_change_preview_record` ref when enabled, and
  to `null` when `hidden_not_applicable` (typically only on
  `unmanaged_addition` rows);
- emits at least one `provenance_record_class` entry so the
  row is later auditable as a typed lifecycle event;
- emits a typed `drift_row_state_class`, with
  `superseded_by_drift_row_id`,
  `superseded_by_remove_bundle_review_id`, or
  `resolved_via_change_preview_ref` populated when the
  state requires it;
- carries no raw signing keys, raw certificate material,
  raw repository URLs, raw absolute paths, raw secrets,
  raw token values, raw setting bodies, or raw user-
  authored content.

Fixtures that embed a `remove_bundle_review` block:

- enumerate `removable_assets[]` with a typed
  `asset_ownership_class` and `safe_to_remove_class` per
  entry;
- enumerate `retained_local_overrides[]` with a typed
  `retained_local_override_class` per entry, citing a
  `target_user_authored_record_ref` for inlined overrides
  or a `decision_row_ref` for consent-dropped overrides;
- cite a `recovery_link` that resolves to the apply-time
  `bundle_change_preview_record` and the matching
  `rollback_checkpoint_linkage_ref`; non-reversible
  apply-time linkages cite a
  `non_reversibility_justification_class` plus a
  `decision_row_ref`;
- emit `remove_review_completed_partial_user_data_retained`
  whenever `retained_local_overrides[]` is non-empty.

## Cases

| Fixture | `drift_state_class` | `drift_axis` | `drift_severity_class` | `claim_narrowing_class` | Notable invariants exercised |
| --- | --- | --- | --- | --- | --- |
| [`bundle_version_drift_certified_launch.yaml`](./bundle_version_drift_certified_launch.yaml) | `bundle_version_drift` | `extension_set` | `narrowing_review_recommended` | `narrows_support_class_one_rung` | Workspace pinned to r1 while channel advanced to r2; manifest's successor recommendation inherited verbatim; adopt and rebase route through new change-preview records. |
| [`missing_artifact_extension_uninstalled.yaml`](./missing_artifact_extension_uninstalled.yaml) | `missing_artifact` | `extension_set` | `narrowing_review_recommended` | `narrows_certification_target_pending_retest` | TypeScript language server uninstalled locally; certification target binding suspends pending retest. |
| [`local_override_editor_setting.yaml`](./local_override_editor_setting.yaml) | `local_override` | `settings_or_token` | `informational_no_narrowing` | `no_narrowing_informational` | User overlay on `editor.format_on_save`; `shared_user_overlay_on_bundle` ownership; preserved-local-override summary ref required. |
| [`unmanaged_addition_user_extension.yaml`](./unmanaged_addition_user_extension.yaml) | `unmanaged_addition` | `extension_set` | `informational_no_narrowing` | `no_narrowing_informational` | User-installed extra extension; `user_owned` ownership; adopt and rebase rendered `hidden_not_applicable`. |
| [`mirror_mismatch_signed_offline.yaml`](./mirror_mismatch_signed_offline.yaml) | `mirror_mismatch` | `compatibility_or_runtime` | `narrowing_review_required` | `narrows_support_class_one_rung` | Signed-offline-bundle posture; mirror cannot reach origin; compare, adopt, and rebase `visible_disabled` with typed `resolve_blocker_class`. |
| [`evidence_stale_certification_retest_pending.yaml`](./evidence_stale_certification_retest_pending.yaml) | `evidence_stale` | `evidence_link` | `narrowing_review_required` | `narrows_certification_target_pending_retest` | Benchmark evidence past freshness window; adopt and rebase `visible_disabled` with `evidence_stale_blocks_adopt`. |
| [`remove_bundle_review_certified_with_overlays.yaml`](./remove_bundle_review_certified_with_overlays.yaml) | `bundle_version_drift` (precipitating) | `extension_set` | `narrowing_review_recommended` | `narrows_support_class_one_rung` | Embedded `remove_bundle_review` block with `safe_to_remove_no_user_data`, `safe_to_remove_user_overlay_preserved`, and `review_required_user_data_co_resident` removable assets; user overlays inlined to a user-authored record; recovery rides through the apply-time workspace rollback checkpoint. |
| [`remove_bundle_review_local_draft.yaml`](./remove_bundle_review_local_draft.yaml) | `unmanaged_addition` (precipitating) | `template_or_scaffold_ref` | `informational_no_narrowing` | `no_narrowing_informational` | Local-draft bundle removal; `not_safe_to_remove_user_owned` user-authored files preserved through workspace-scope retention; recovery cites `local_draft_no_prior_state` non-reversibility justification with a decision row. |

The case-index manifest at
[`./manifest.yaml`](./manifest.yaml) maps each file to the
contract sections it exercises.
