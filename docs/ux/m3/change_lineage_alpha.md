# Change-lineage alpha: the landing-state inspector

The alpha change-lineage family is the **review-time projection** of one
underlying change-object record. Where the change-object record (frozen
in [`docs/review/m3/change_objects_alpha.md`](../../review/m3/change_objects_alpha.md))
names a branch, worktree, or patch stack with stable lineage and a
landing-state summary, the change-lineage record adds the **conflict-state**
and **publish-readiness** truth the landing-state inspector renders so a
reviewer can answer four questions from a single row **before** publish,
merge, or apply runs:

1. **Which scope am I operating on?** `active_scope_class` distinguishes
   the main worktree from a side worktree from a stacked patch set so a
   user can never widen mutation by accident.
2. **Where will the change land?** `target_summary` re-projects the
   change-object landing target — landing-state class, landing-action
   class, target ref, mutation authority, remote visibility, and required
   network egress.
3. **What does the lineage look like?** `ancestry_view` re-projects the
   base ref, divergence class, commits-ahead / commits-behind counts, and
   the ancestor chain so support packets and review surfaces quote one
   lineage truth.
4. **Is it ready, and what is in the way?** `conflict_state` and
   `publish_readiness` carry closed-vocabulary classes and a bounded list
   of blockers so the inspector can name the next reviewable step.

The companion schema lives at:

- [`/schemas/review/change_lineage.schema.json`](../../../schemas/review/change_lineage.schema.json)

The canonical fixtures live under:

- [`/fixtures/review/m3/change_lineage/`](../../../fixtures/review/m3/change_lineage/)

The headless validator that gates every fixture lives at:

- [`/ci/check_change_lineage_alpha.py`](../../../ci/check_change_lineage_alpha.py)

The Rust types are exported from `aureline_review::change_inspector`,
defined in
[`crates/aureline-review/src/change_inspector/mod.rs`](../../../crates/aureline-review/src/change_inspector/mod.rs).
The integration test
[`crates/aureline-review/tests/change_lineage_alpha.rs`](../../../crates/aureline-review/tests/change_lineage_alpha.rs)
replays every fixture and proves the closed acceptance states. The first
shell consumer is
[`crates/aureline-shell/src/review/change_inspector/mod.rs`](../../../crates/aureline-shell/src/review/change_inspector/mod.rs),
which renders deterministic landing-state rows directly from the
checked-in alpha fixtures and a matching CLI / headless plaintext export.

## 1 Why freeze this now

The change-object record (`docs/review/m3/change_objects_alpha.md`)
already names *where* a change will land. It deliberately stays narrower
than the underlying Git mutation, publish, branch, and conflict-handoff
services. It does **not** answer two review questions every change
orchestration surface still needs:

- **Am I on the main worktree, a side worktree, or a stacked patch set?**
  Mutations on the wrong scope quietly widen authority. The inspector
  must call the scope out, by name, on every row.
- **Is the change ready, and if not, what is in the way?** A clean
  ancestry is not enough. Conflict state and publish readiness — and the
  named blockers behind a blocked state — have to ride alongside the
  target identity so review and support packets read one truth.

The change-lineage record freezes those answers without inventing a new
collaboration / cloud-control-plane surface beyond the M3 bounded beta
foundations explicitly claimed.

## 2 Record shape

Every change-lineage row is one `change_lineage_alpha_record` carrying:

| Block | Required content |
| --- | --- |
| `change_lineage_id` | Opaque, stable id quoted by support and CLI surfaces. |
| `change_object_ref` | Opaque `change_object_id` from `fixtures/workspace/m3/change_objects/` the lineage row projects. |
| `change_object_kind` | One of `branch`, `worktree`, `patch_stack` (matches the change-object record). |
| `active_scope_class` | One of `main_worktree`, `side_worktree`, `stacked_patch_set`, `detached_inspection`, `active_scope_unknown_requires_review`. |
| `operator_caveat` | One reviewable sentence reminding the user which scope they are about to mutate. |
| `target_summary` | Re-projects `landing_state_class`, `landing_action_class`, `target_ref`, `target_kind`, `mutation_authority_class`, `remote_visibility_class`, `required_network_egress_class`, and the reviewable `pending_writes_summary`. |
| `ancestry_view` | Re-projects `base_ref`, `base_kind`, `divergence_class`, optional commits-ahead / commits-behind counts, and the unique-`ancestor_ref` chain. |
| `conflict_state` | `conflict_state_class`, `conflict_path_count`, and optional reviewable notes. |
| `publish_readiness` | `publish_readiness_class`, a unique list of `blockers`, and optional reviewable notes. |
| `consumer_surfaces` | Non-empty list drawn from `change_inspector`, `change_object_inspector`, `activity_center`, `review_preview`, `cli_headless_entry`, `support_export`, `docs_review`; must include `change_inspector`. |
| `support_export` | Packet refs and the closed `raw_path_export_allowed = raw_branch_name_export_allowed = raw_remote_url_export_allowed = raw_diff_body_export_allowed = false`. |
| `review_invariants` | All of `target_ref_pinned`, `ancestry_pinned`, `conflict_state_inspectable`, `publish_readiness_inspectable`, `no_hidden_target_mutation` must be `true`. |

## 3 Frozen rules

The validator and the integration test both enforce:

1. **Scope is named, by kind.** A `branch` change-object never opens a
   `stacked_patch_set` scope; a `worktree` record opens a `main_worktree`
   or `side_worktree` scope only; a `patch_stack` record opens the
   `stacked_patch_set` scope only.
2. **Ready states are conflict-free and action-aligned.**
   `ready_to_publish` requires `landing_action_class = publish` and
   `conflict_state_class = no_conflicts_detected`. `ready_to_merge` and
   `ready_to_apply` enforce the same alignment for their respective
   action classes. `not_applicable_inspect_only` requires
   `landing_action_class = inspect_only`.
3. **Blocked states quote the blocker.** `blocked_by_conflicts` requires
   a non-`no_conflicts_detected` conflict state. Every `blocked_by_*`
   (and `readiness_unknown_requires_review`) readiness class must declare
   at least one real blocker token; the inspector never claims a blocker
   it cannot name. Ready and inspect-only readiness classes carry exactly
   the `no_blockers` token.
4. **Conflict counts are honest.** `no_conflicts_detected` carries
   `conflict_path_count = 0`. Pending merge / rebase / apply conflict
   classes carry `conflict_path_count > 0`. The inspector cannot show a
   pending conflict without a count.
5. **Closed export.** Raw paths, raw branch names, raw remote URLs, and
   raw diff bodies are never exported through the change-lineage record.
   Support packets quote opaque ref labels, class tokens, and short
   reviewable sentences only.
6. **Consumer wiring.** `consumer_surfaces` always includes
   `change_inspector` so the first product surface stays bound.
7. **Pre-execution review.** The record is inspectable before publish,
   merge, and apply, and writes nothing on its own. The Git mutation,
   publish, branch, and conflict-handoff services still own the actual
   writes.

## 4 Vocabulary, by block

### 4.1 `active_scope_class`

- `main_worktree`
- `side_worktree`
- `stacked_patch_set`
- `detached_inspection`
- `active_scope_unknown_requires_review`

### 4.2 `conflict_state_class`

- `no_conflicts_detected`
- `merge_conflicts_pending_review`
- `rebase_conflicts_pending_review`
- `apply_conflicts_pending_review`
- `upstream_diverged_requires_rebase`
- `conflict_state_unknown_requires_review`

### 4.3 `publish_readiness_class`

- `ready_to_publish`
- `ready_to_merge`
- `ready_to_apply`
- `blocked_by_conflicts`
- `blocked_by_review_required`
- `blocked_by_authority`
- `not_applicable_inspect_only`
- `readiness_unknown_requires_review`

### 4.4 `readiness_blocker_class`

- `conflict_resolution_required`
- `rebase_required`
- `review_approval_required`
- `authority_widening_required`
- `remote_visibility_widening_required`
- `policy_review_required`
- `no_blockers`
- `blocker_class_unknown_requires_review`

Landing-state, landing-action, mutation-authority, remote-visibility,
network-egress, and divergence vocabularies are inherited verbatim from
the change-object record (`docs/review/m3/change_objects_alpha.md`).
Re-using the same closed tokens means review and support packets read one
truth without forking copy.

## 5 Fixtures

The checked-in fixtures under
[`/fixtures/review/m3/change_lineage/`](../../../fixtures/review/m3/change_lineage/)
cover the three change-object kinds, every required active scope, and a
spread of readiness states:

| Fixture | `change_object_kind` | `active_scope_class` | `publish_readiness_class` | Conflict state |
| --- | --- | --- | --- | --- |
| `branch_main_worktree_ready_to_publish.json` | `branch` | `main_worktree` | `ready_to_publish` | `no_conflicts_detected` |
| `branch_blocked_by_review_required.json` | `branch` | `main_worktree` | `blocked_by_review_required` | `no_conflicts_detected` |
| `branch_landed_publicly_inspect_only.json` | `branch` | `detached_inspection` | `not_applicable_inspect_only` | `no_conflicts_detected` |
| `worktree_side_worktree_inspect_only.json` | `worktree` | `side_worktree` | `not_applicable_inspect_only` | `no_conflicts_detected` |
| `patch_stack_blocked_by_conflicts.json` | `patch_stack` | `stacked_patch_set` | `blocked_by_conflicts` | `upstream_diverged_requires_rebase` |

Every fixture quotes the matching `change_object_ref` from
`fixtures/workspace/m3/change_objects/` so the inspector, review packets,
and support exports share one lineage truth without forking ids.

## 6 Consumer wiring

The first product surface bound to this record is the shell
landing-state inspector in
[`crates/aureline-shell/src/review/change_inspector/mod.rs`](../../../crates/aureline-shell/src/review/change_inspector/mod.rs).
It builds a deterministic landing-state row per fixture and exports a
matching plaintext block (`render_alpha_change_lineage_plaintext`) for
CLI / headless / docs / support consumers, proving the bundle is
inspectable and not doc-only.

## 7 Out of scope

- Full M6 collaboration and full cloud-control-plane productization.
- Generating diff bodies, raw command lines, or raw remote URLs.
- Mutating branches, worktrees, or patch stacks. The Git mutation,
  publish, branch, and conflict-handoff services still own writes.
- Re-deriving lineage data the change-object record already pins; the
  change-lineage row is a projection, not a parallel source of truth.
