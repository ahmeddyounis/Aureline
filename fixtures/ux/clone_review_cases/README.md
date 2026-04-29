# Clone-review, destination-collision, and post-clone trust-stage fixtures

This corpus is the worked-example projection of
[`/docs/ux/clone_review_contract.md`](../../../docs/ux/clone_review_contract.md)
and the boundary schema:

- [`/schemas/ux/clone_review.schema.json`](../../../schemas/ux/clone_review.schema.json)

Each fixture is one record. `clone_review_record` fixtures validate
against the `clone_review_record` `$def`; the
`destination_collision_sheet_record` and
`post_clone_trust_stage_record` fixtures validate against their
respective `$def`. The schema's top-level `oneOf` discriminates by
`record_kind`.

## Pairings

| Clone-review fixture | Paired collision fixture | Paired post-clone trust-stage fixture |
|---|---|---|
| `clone_review_https_default_branch.yaml` | (no collision) | `post_clone_trust_stage_non_durable_staging.yaml` |
| `clone_review_oauth_lfs_submodules.yaml` | (no collision) | (re-uses `post_clone_trust_stage_non_durable_staging.yaml` semantics) |
| `clone_review_mirror_partial_clone.yaml` | (no collision) | `post_clone_trust_stage_materialization_failed.yaml` |
| `clone_review_self_signed_certificate_review.yaml` | (no collision) | (re-uses `post_clone_trust_stage_non_durable_staging.yaml` semantics; commit denied until certificate review) |
| (any clone-review fixture above) | `destination_collision_existing_repo_root_match.yaml` | (resolution: reuse / inspect; no clone bytes land) |
| (any clone-review fixture above) | `destination_collision_existing_workspace_file.yaml` | (resolution: reroute to add_root) |
| (any clone-review fixture above) | `destination_collision_existing_path_non_empty.yaml` | (resolution: clone_elsewhere / cancel; explicit overwrite required) |
| (any clone-review fixture above) | `destination_collision_destination_blocked_by_policy.yaml` | (resolution: admin help / cancel) |
| (any clone-review fixture above) | `destination_collision_previously_cloned_target_match.yaml` | (resolution: reuse / add_existing) |
| (durable clone) | (no collision) | `post_clone_trust_stage_durable_after_review.yaml` |

## Coverage matrix

Disclosure axes exercised: normalized remote URL with mirror /
upstream label, host posture with `trusted_chain_verified`,
`pinned_chain_verified`, `self_signed_user_review_required`,
auth modes `oauth_handle` (with and without browser handoff) and
`ssh_agent`, branch / ref classes `default_branch` and
`named_branch`, clone-depth classes `full_history` and
`partial_clone_filtered` (with `partial_filter_class = blob_none`),
LFS classes `no_lfs_required` and `lfs_pointer_only_clone`,
submodule classes `no_submodules` and `submodule_init_pending`,
destination dispositions `write_to_labelled_staging`, and staging
class `non_durable_staging` (with `non_durable_label_visible =
true`).

Next-step choices exercised: `clone_only`, `clone_and_review`,
`clone_and_open`, `clone_and_add` (the last via the
`add_existing_to_workspace` collision resolution).

Destination-collision classes exercised:
`existing_repo_root_match`, `existing_workspace_file_at_path`,
`existing_path_non_empty`, `destination_blocked_by_policy`,
`previously_cloned_target_match`. The remaining classes
(`existing_path_empty_writable`, `existing_path_read_only`,
`existing_repo_root_mismatch`, `existing_worktree_at_path`,
`previously_cloned_target_mismatch`) are reserved for downstream
collision-resolution fixtures.

Materialization classes exercised:
`bytes_in_non_durable_staging`, `bytes_in_durable_workspace`,
`materialization_failed`. The
`bytes_in_user_destination_pending_review` variant is reserved
for the durable-destination-direct collision resolution.

Materialization durability exercised: `non_durable_until_open`,
`durable_after_review`. The `durable_after_user_choice` variant
is reserved for the explicit durable-destination-direct
fixture.

Safest next actions exercised: `review_trust_and_open`,
`set_up_later`, `roll_back_clone`. The remaining values
(`compare_before_open`, `inspect_only`, `open_minimal`,
`return_to_start_center`) are reserved for downstream surfaces.

## Pre-clone / post-clone invariants

Every `clone_review_record` fixture asserts:

- `staging_durability_disclosure.staging_class =
  non_durable_staging` pairs with `non_durable_label_visible =
  true` and `durable_promotion_requires_review = true`.
- `destination_disclosure.collision_class != no_collision`
  pairs with `destination_collision_sheet_ref`.

Every `destination_collision_sheet_record` fixture asserts:

- `temp_directory_disclosed` is set in line with the resolution.
- `explicit_overwrite_required = true` only when the resolution
  would overwrite existing durable bytes.

Every `post_clone_trust_stage_record` fixture asserts:

- `trust_grant_at_clone = false`.
- `dependency_restore_at_clone = false`.
- `extension_recommendation_at_clone = false`.
- `hook_or_task_execution_at_clone = false`.
- `reviewed_step_required_to_open = true`.
- `exact_target_identity_preserved = true`.

A fixture that emits any of these as the opposite boolean is
non-conforming and the surface MUST not commit.

## Adding a fixture

1. Pick the smallest scenario that exercises a single new value
   (a new disclosure axis class, collision class, safe action,
   materialization class, materialization durability, or
   safest next action).
2. Write the fixture as a YAML document with the `__fixture__`
   prelude naming the scenario and the contract sections
   asserted.
3. Add the fixture to the pairing table and the coverage matrix
   above.
4. Validate the fixture against
   `/schemas/ux/clone_review.schema.json`.
