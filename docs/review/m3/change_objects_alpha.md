# Change-object alpha: branches, worktrees, and patch stacks

The alpha change-object family is the durable, reviewable record that names a
**branch**, a **worktree**, or a **patch stack** with a stable id, lineage,
and landing-state summary so the change-object inspector, the Activity
Center, review previews, CLI / headless entry, docs, and support packets
read **one** truth about where the change will land **before** publish,
merge, or apply.

The record is intentionally narrower than the underlying Git mutation,
publish, branch, or conflict-handoff services that already live in
`aureline_git` — those still own *what was mutated*. The change-object
record owns **review-before-action**: what the user is shown about the
landing target, the lineage, the mutation authority, the remote visibility,
the egress envelope, and the pending writes ahead of any decision to
publish, merge, or apply.

The companion schema lives at:

- [`/schemas/workspace/change_object.schema.json`](../../../schemas/workspace/change_object.schema.json)

The canonical fixtures live under:

- [`/fixtures/workspace/m3/change_objects/`](../../../fixtures/workspace/m3/change_objects/)

The headless validator that gates every fixture lives at:

- [`/ci/check_change_object_alpha.py`](../../../ci/check_change_object_alpha.py)

The Rust types are exported from `aureline_git::change_objects`, defined in
[`crates/aureline-git/src/change_objects/mod.rs`](../../../crates/aureline-git/src/change_objects/mod.rs).
The integration test
[`crates/aureline-git/tests/change_object_alpha.rs`](../../../crates/aureline-git/tests/change_object_alpha.rs)
replays every fixture and proves the closed acceptance states. The first
shell consumer is
[`crates/aureline-shell/src/change_object_inspector/mod.rs`](../../../crates/aureline-shell/src/change_object_inspector/mod.rs),
which renders a deterministic landing-state row directly from the checked-in
alpha fixtures and a matching CLI / headless plaintext export.

## 1 Why freeze this now

Git status, branch, commit, publish, and conflict-handoff services already
answer *what was mutated locally*. They do not answer the **review** question
every change-orchestration surface asks first:

- which branch, worktree, or patch stack is the change actually riding on;
- where will it land — local-only, pending publish to a remote, pending
  merge into a base, pending patch apply, already landed locally, already
  landed publicly;
- which `landing_action_class` does the user have to approve next — publish,
  merge, or apply;
- which mutation authority owns the destination — local-only, provider-bound,
  or managed-workspace-bound;
- whether the action requires remote visibility and a network-egress
  envelope, or stays local-only.

The alpha change-object record freezes those answers without inventing a
new collaboration / cloud-control-plane surface beyond the M3 bounded beta
foundations.

## 2 Record shape

Every change object is one `change_object_alpha_record` carrying:

| Block | Required content |
| --- | --- |
| `change_object_id` | Opaque, stable id quoted by support and CLI surfaces. |
| `change_object_kind` | One of `branch`, `worktree`, `patch_stack`. |
| `lineage` | `base_ref`, `base_kind`, `divergence_class`, optional commits-ahead / commits-behind counts, and a unique-`ancestor_ref` ancestor chain. |
| `landing_state` | `landing_state_class`, `landing_action_class`, `target_ref`, `target_kind`, `mutation_authority_class`, `remote_visibility_class`, `required_network_egress_class`, and a reviewable `pending_writes_summary`. |
| `branch` / `worktree` / `patch_stack` | Exactly the variant block that matches the kind; the other two are absent. |
| `consumer_surfaces` | Non-empty list drawn from `change_object_inspector`, `activity_center`, `review_preview`, `cli_headless_entry`, `support_export`, `docs_review`; must include `change_object_inspector`. |
| `support_export` | Packet refs and the closed `raw_path_export_allowed = raw_branch_name_export_allowed = raw_remote_url_export_allowed = raw_diff_body_export_allowed = false`. |
| `review_invariants` | All of `inspectable_before_publish`, `inspectable_before_merge`, `inspectable_before_apply`, `no_hidden_target_mutation` must be `true`. |

## 3 Frozen rules

The validator and the integration test both enforce:

1. **Stable ids and lineage.** Every change object carries a `change_object_id`,
   a closed-vocabulary `divergence_class`, and a unique-`ancestor_ref` ancestor
   chain so support packets and review exports quote the same lineage truth.
2. **Inspectable landing target.** `landing_state` always names the target
   ref, target kind, landing-state class, landing-action class, mutation
   authority, remote visibility, and required network-egress class. The
   pending-writes summary is a reviewable sentence — never a raw command
   line or raw diff body.
3. **No hidden authority widening.** `pending_publish_to_remote` MUST pair
   with a non-`no_remote_attached` remote-visibility class, a non
   `no_network_egress_required` egress class, and a `publish` (or
   review-required) landing-action class. `pending_merge_into_base` MUST
   declare `landing_action_class = merge`. `pending_patch_apply` MUST
   declare `landing_action_class = apply`. `local_only_no_remote_yet` MUST
   keep the remote visibility detached, the egress envelope closed, and
   the mutation authority local-only.
4. **Variant exclusivity.** The variant block matches the
   `change_object_kind`. A `branch` record never carries `worktree` or
   `patch_stack` variant blocks; same for the other two kinds.
5. **Consumer wiring.** `consumer_surfaces` always includes
   `change_object_inspector` so the first product surface stays bound.
6. **Closed export.** Raw paths, raw branch names, raw remote URLs, and
   raw diff bodies are never exported through the change object. Support
   packets quote opaque ref labels and class tokens only.
7. **Pre-execution review.** The record is inspectable before publish,
   merge, and apply, and writes nothing on its own. The Git mutation,
   publish, branch, and conflict-handoff services still own the actual
   writes.

## 4 Vocabulary, by block

### 4.1 `landing_state_class`

- `local_only_no_remote_yet`
- `pending_publish_to_remote`
- `pending_merge_into_base`
- `pending_patch_apply`
- `landed_locally_only`
- `landed_publicly`
- `degraded_unknown_target_requires_review`

### 4.2 `landing_action_class`

- `publish`
- `merge`
- `apply`
- `inspect_only`
- `action_class_unknown_requires_review`

### 4.3 `mutation_authority_class`

- `local_only`
- `provider_bound`
- `managed_workspace_bound`
- `mirror_cached`
- `mutation_authority_unknown_requires_review`

### 4.4 `remote_visibility_class`

- `no_remote_attached`
- `remote_attached_private`
- `remote_attached_team`
- `remote_attached_public`
- `remote_visibility_unknown_requires_review`

### 4.5 `required_network_egress_class`

- `no_network_egress_required`
- `first_party_origin_only`
- `team_managed_mirror_only`
- `provider_bound_origin_required`
- `managed_workspace_envelope_only`
- `egress_envelope_unknown_requires_review`

## 5 Fixtures

The checked-in fixtures under
[`/fixtures/workspace/m3/change_objects/`](../../../fixtures/workspace/m3/change_objects/)
cover the three kinds and a spread of landing states:

| Fixture | Kind | Landing state | What it proves |
| --- | --- | --- | --- |
| `branch_local_pending_publish.json` | `branch` | `pending_publish_to_remote` | A local branch ahead of base, pending publish, with a bound provider origin. |
| `branch_pending_merge.json` | `branch` | `pending_merge_into_base` | A local branch pending merge into the team base under managed-workspace authority. |
| `branch_landed_publicly.json` | `branch` | `landed_publicly` | Inspect-only record for a branch already landed publicly. |
| `worktree_linked_local_only.json` | `worktree` | `local_only_no_remote_yet` | A linked local worktree with no remote attached and zero egress. |
| `patch_stack_provider_pull_request.json` | `patch_stack` | `pending_patch_apply` | A three-patch stack pending apply onto an open provider pull request. |

Every fixture keeps raw paths, raw branch names, raw remote URLs, and raw
diff bodies closed; only opaque ref labels and closed-vocabulary tokens
cross the boundary.

## 6 Consumer wiring

The first product surface bound to this record is the shell change-object
inspector in
[`crates/aureline-shell/src/change_object_inspector/mod.rs`](../../../crates/aureline-shell/src/change_object_inspector/mod.rs).
It builds a deterministic landing-state row per fixture and exports a
matching plaintext block for CLI / headless / docs / support consumers,
proving the bundle is inspectable and not doc-only.

## 7 Out of scope

- Full M6 collaboration and full cloud-control-plane productization.
- Generating diff bodies, raw command lines, or raw remote URLs.
- Mutating branches, worktrees, or patch stacks. The Git mutation,
  publish, branch, and conflict-handoff services still own writes.
