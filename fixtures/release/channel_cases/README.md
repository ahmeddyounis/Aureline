# Channel-and-branch seed cases

These fixtures are the seed cases the channel-and-branch contract at
[`docs/release/channel_and_branch_contract.md`](../../../docs/release/channel_and_branch_contract.md)
defines. Each file is a `channel_case_record` instance that projects
onto the `channel_row`, `side_by_side_admission_matrix`,
`patch_and_backport_lane_register`, and
`freeze_posture_admission_matrix` rows already frozen in
[`/artifacts/release/channel_matrix.yaml`](../../../artifacts/release/channel_matrix.yaml)
and the `artifact_family_versioning_row` rows in
[`/artifacts/release/artifact_family_versioning.yaml`](../../../artifacts/release/artifact_family_versioning.yaml)
— the cases do not introduce a new JSON schema.

Every case:

- names one stable `channel_case_id`;
- binds one or more `channel_row_refs` and (where applicable) one
  `side_by_side_pair_ref`;
- declares an explicit `outcome_class` from
  `{compatible_side_by_side, forbidden_side_by_side, downgrade_via_named_path,
   freeze_posture_blocks_promotion, last_known_good_repair_path,
   patch_lane_admission, portable_no_machine_global_mutation,
   marketplace_metadata_signed_index_required}`;
- cites the `state_root_map.yaml` rows or
  `install_topology_matrix.yaml` cards that carry the supporting
  evidence;
- carries an `expected_promotion_decision_class` so reviewers can
  resolve "promote / hold / refuse" mechanically;
- declares four reviewer-facing explanation strings (user-facing
  summary, release-notes summary, support summary, promotion
  summary) so release / support / docs surfaces render verbatim
  without minting prose.

## Case list

- `compatible_side_by_side_stable_preview.json` —
  `channel_case:release.side_by_side.compatible_stable_preview`
- `forbidden_two_stables_one_host.json` —
  `channel_case:release.side_by_side.forbidden_two_stables_one_host`
- `portable_and_installed_stable_no_handler_collision.json` —
  `channel_case:release.side_by_side.installed_and_portable_no_handler_collision`
- `downgrade_lts_via_admin_pinned_floor.json` —
  `channel_case:release.downgrade.lts_via_admin_pinned_floor`
- `last_known_good_repair_after_failed_stable_update.json` —
  `channel_case:release.repair.last_known_good_after_failed_stable_update`
- `hard_freeze_blocks_preview_promotion.json` —
  `channel_case:release.freeze.hard_freeze_blocks_preview_promotion`
- `backport_stable_to_lts_security_only.json` —
  `channel_case:release.backport.stable_to_lts_security_only`
- `backport_forbidden_feature_invention_on_hotfix.json` —
  `channel_case:release.backport.forbidden_feature_invention_on_hotfix`
- `marketplace_metadata_requires_signed_index.json` —
  `channel_case:release.versioning.marketplace_metadata_requires_signed_index`

Every case cites its channel-row, branch-posture, side-by-side pair,
and (where applicable) versioning row by stable ref so release and
support review can pivot in O(1) from one channel case → one
channel row → one side-by-side pair / patch lane / versioning row →
one promotion decision.
