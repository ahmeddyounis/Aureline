# Clone Review Sheet, Destination-Collision Sheet, and Post-Clone Trust-Stage Contract

This document freezes the **clone review sheet** every clone
activation renders before bytes land, the **destination-collision
sheet** that resolves collisions on the chosen destination, and the
**post-clone trust-stage contract** that keeps repository
materialization separate from trust grant, dependency restore,
extension recommendation, and hook / task execution.

The contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI / UX Spec, or design-system style guide, those sources win
and this document plus its schema and fixtures update in the same
change. Where a Start Center, palette, drag-drop preview,
system-open handoff, CLI / headless front-end, or workspace
switcher mints a parallel clone-review surface, collision sheet,
or post-clone hand-off, this contract wins and the surface is
non-conforming.

The companion artifacts are:

- [`/schemas/ux/clone_review.schema.json`](../../schemas/ux/clone_review.schema.json)
  — boundary schema for the three records: `clone_review_record`,
  `destination_collision_sheet_record`, and
  `post_clone_trust_stage_record`.
- [`/fixtures/ux/clone_review_cases/`](../../fixtures/ux/clone_review_cases/)
  — worked cases for normalized-URL and host-posture review,
  OAuth + LFS + submodule disclosure, mirror / partial / shallow
  clone disclosure, the four destination-collision classes (existing
  path, existing repo / worktree, existing workspace file, previously
  cloned target, policy-blocked destination), and the post-clone
  trust-stage outcomes (non-durable staging pending admission,
  durable target after reviewed admission).

This contract composes with, and does not replace:

- [`/docs/ux/project_entry_contract.md`](./project_entry_contract.md)
  and [`/schemas/ux/open_flow_sheet.schema.json`](../../schemas/ux/open_flow_sheet.schema.json)
  for the `clone_remote_target` open-flow sheet that hosts the clone
  review. The `clone_review_record` is the clone-specific projection
  of the eight required disclosure axes (project_entry_contract.md
  §5.2) onto a single clone activation. The clone review never
  re-opens the verb matrix, never widens the trust posture, and never
  bypasses the §5.3 pre-commit invariants.
- [`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md)
  and [`/schemas/workspace/source_locator.schema.json`](../../schemas/workspace/source_locator.schema.json)
  / [`/schemas/workspace/checkout_plan.schema.json`](../../schemas/workspace/checkout_plan.schema.json)
  for the upstream `source_locator_record`, `checkout_plan_record`,
  and bootstrap-queue items. The clone review previews those records;
  it does not redefine `locator_class`, `transport_class`,
  `auth_mode_class`, `acquisition_posture`,
  `declared_freshness_class`, `signer_continuity_class`, or any
  bootstrap-item class.
- [`/docs/workspace/materialization_and_staging_policy.md`](../workspace/materialization_and_staging_policy.md)
  and [`/schemas/workspace/materialization_class.schema.json`](../../schemas/workspace/materialization_class.schema.json)
  for the workspace-level target-materialization vocabulary and
  temp-location disclosure rules that keep non-durable staging distinct
  from durable workspace state across UI, support/export, and recovery
  flows.
- [`/docs/ux/host_identity_contract.md`](./host_identity_contract.md)
  and [`/schemas/contexts/host_identity_chip.schema.json`](../../schemas/contexts/host_identity_chip.schema.json)
  for the host-identity chip the clone review reads when it discloses
  host posture. The clone review never invents a host-class value.
- [`/docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  and [`/docs/adr/0001-identity-mode-trust.md`](../adr/0001-identity-mode-trust.md)
  / [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for trust-state, authority-delta, and restricted-mode vocabulary.
  The clone review previews the resulting trust posture; the trust
  transition is owned by the trust-prompt surface and fires only
  after the post-clone admission step.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for the surface-class taxonomy the clone review and collision
  sheets render inside (`window_attached_sheet`, `full_sheet`,
  `dedicated_review_surface`, `cli_headless_text_block`).

## Who reads this contract

- **Shell, Start Center, palette, drag-drop, system-open, CLI, and
  workspace-switcher authors** wiring the `clone` verb. Every clone
  activation emits one `clone_review_record`, optionally one
  `destination_collision_sheet_record`, and one
  `post_clone_trust_stage_record` BEFORE the cloned tree is
  materialized into a durable target or the trust prompt fires.
- **Designers** sizing the clone review and collision copy so
  Clone never smuggles in trust, setup, or execution side effects.
- **Docs, support, accessibility, and measurement authors**
  attributing clone behavior to the same record family the shell
  renders, so a CLI clone, a system-open handoff, and a Start Center
  click trace to the same clone-review and post-clone trust-stage
  rows.

## 1. Scope

This contract freezes:

- One `clone_review_record` (§3) every clone activation emits before
  bytes land. The record carries the closed disclosure axes (§3.2):
  normalized remote URL, host / certificate posture, auth mode,
  branch / ref posture, clone depth, LFS state, submodule state,
  destination disposition, next-step choice, and the staging-
  durability label required when bytes land in non-durable staging.
- One `destination_collision_sheet_record` (§4) every clone
  activation that resolves to an already-occupied destination emits
  BEFORE bytes land. The record carries the closed
  `destination_collision_class` set, the closed safe-action set
  (`reuse_existing_repo`, `add_existing_to_workspace`,
  `clone_elsewhere`, `reveal_in_filesystem`, `cancel_no_change`),
  the temp-directory and explicit-overwrite invariants, and the
  policy-blocked path.
- One `post_clone_trust_stage_record` (§5) every clone activation
  emits AFTER bytes land in their materialization target and BEFORE
  the trust prompt fires, dependencies restore, extensions are
  recommended, or repo hooks / tasks execute. The record carries
  the four "no implicit grant" booleans, the materialization
  durability class, the safest next action, and the typed
  reviewed-step requirements that gate every side effect after
  clone.
- The cross-surface invariants (§6) so Start Center, palette,
  drag-drop, system open, CLI / headless, and workspace switcher
  remain semantically aligned with the same three records.

## 2. Out of Scope

- git transport implementation, partial-clone / sparse-checkout /
  shallow-clone fetch logic, LFS smudge implementation, submodule
  recursion, mirror caching, and credential-handle resolution.
  The bootstrap and source-acquisition contracts own the
  acquisition engine; this contract only freezes how the clone
  review and post-clone hand-off render the typed inputs.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and shell-interaction-safety contract own the strings.
- Platform file-picker chrome for picking a destination path. The
  desktop-affordance contract owns that overlay; the destination
  the user picks lands on the `clone_review_record` as the
  destination disposition.
- Workspace-trust prompt visuals, policy-review takeover,
  rollback-checkpoint inspection, and dependency-install review.
  Those are owned by the trust-prompt, policy-review,
  rollback-checkpoint, and package-action contracts; the
  post-clone trust-stage record only **routes** the user into them.

## 3. Clone review record

Every clone activation emits one `clone_review_record` BEFORE bytes
land in any destination — staging, durable, or otherwise. The
clone review is the body the `open_flow_sheet_record` of class
`clone_remote_target` renders inside; it does not replace the
open-flow sheet, it is the typed projection of its
`clone_remote_target`-specific disclosures.

A surface that commits a clone without first emitting one
`clone_review_record` is non-conforming. A surface that emits the
record but commits before all required axes (§3.2) resolve is
non-conforming.

### 3.1 Required fields

- `record_kind = clone_review_record`.
- `clone_review_schema_version = 1`.
- `clone_review_id` (opaque, stable for the activation).
- `entry_chooser_row_ref` — back-link to the
  `entry_chooser_row_record` that activated the clone.
- `open_flow_sheet_ref` — back-link to the parent
  `open_flow_sheet_record` of class `clone_remote_target`.
- `source_locator_ref` — opaque ref into the upstream
  `source_locator_record`. Raw URLs never appear; the locator is
  the canonical source-of-truth for normalized URL, mirror /
  upstream identity, signer continuity, and freshness class.
- `normalized_remote_disclosure` (§3.2.1).
- `host_posture_disclosure` (§3.2.2).
- `auth_mode_disclosure` (§3.2.3).
- `branch_or_ref_disclosure` (§3.2.4).
- `clone_depth_disclosure` (§3.2.5).
- `lfs_state_disclosure` (§3.2.6).
- `submodule_state_disclosure` (§3.2.7).
- `destination_disclosure` (§3.2.8).
- `next_step_choice_class` — exactly one of the closed set (§3.3).
- `staging_durability_disclosure` (§3.4).
- `post_clone_trust_stage_ref` — opaque ref to the
  `post_clone_trust_stage_record` the activation will emit on
  commit. Required on every clone review; a clone review without a
  paired post-clone trust-stage record is non-conforming.
- `fallback_actions[]` — at least one typed fallback drawn from
  the open-flow sheet `fallback_action_class` set
  (project_entry_contract.md §5.2). A clone review whose only
  affordance is "Clone now" is non-conforming.
- `next_step_decision_hooks[]` — at least one; drawn from the
  entry-restore §1.7 closed set re-exported on
  `/schemas/ux/open_flow_sheet.schema.json`.
- `presentation_label` (redaction-aware, ≤ 1024 graphemes).

Optional fields: `presentation_subtitle`,
`destination_collision_sheet_ref` (required only when the chosen
destination resolves a collision; see §4),
`checkout_plan_ref` (opaque ref into the upstream
`checkout_plan_record` for the inspectable bootstrap envelope),
`host_identity_chip_ref` (opaque ref into the host-identity-chip
record).

### 3.2 Required disclosure axes

Every `clone_review_record` carries the eight axes below. A surface
that cannot populate an axis MUST deny the activation with a typed
`review_requirement` (re-using the open-flow sheet
`policy_restrictions_disclosure.required_reviews` set) instead of
defaulting to a generic "Clone anyway" path.

#### 3.2.1 `normalized_remote_disclosure`

- `normalized_remote_url_label` — redaction-aware presentation
  label (≤ 1024 graphemes) for the normalized remote URL. Raw
  credentials never appear.
- `source_locator_ref` — required.
- `mirror_or_proxy_label` — redaction-aware label, only when the
  upstream `transport_class` is `mirror` or `proxy`.
- `upstream_origin_label` — redaction-aware label for the
  upstream origin a mirror / proxy fronts.
- `freshness_class` — re-export of `declared_freshness_class`
  (`live_origin`, `mirror_fresh`, `mirror_lagged`, `mirror_stale`,
  `offline_snapshot`, `signed_offline_bundle`,
  `unknown_freshness`). Required on every clone review; a clone
  review that hides `mirror_lagged` or `mirror_stale` is
  non-conforming.

#### 3.2.2 `host_posture_disclosure`

- `host_label` — redaction-aware label.
- `host_class` — re-export of host-identity-chip `host_class`
  (`local_desktop`, `remote_host`, `container_devcontainer`,
  `managed_workspace`, `browser_runtime_bridge`,
  `service_plane`).
- `certificate_class` — closed set (§3.5.1).
- `host_identity_chip_ref` — required when host posture exposes a
  boundary-change cue (e.g. mirror, proxy, managed cloud).
- `summary` — redaction-aware label.

#### 3.2.3 `auth_mode_disclosure`

- `auth_mode_class` — re-export of `auth_mode_class`
  (`ssh_agent`, `pat_handle`, `oauth_handle`,
  `device_code_handle`, `managed_session_ticket`,
  `connected_provider_ticket`, `anonymous`,
  `inherit_local_identity`, `none`, `other`).
- `credential_handle_ref` — opaque, optional. Raw secrets never
  cross this boundary.
- `browser_handoff_required` — boolean; set true when the auth
  mode resolves to a system-browser or device-code handoff before
  bytes land.
- `summary`.

#### 3.2.4 `branch_or_ref_disclosure`

- `branch_or_ref_class` — closed set (§3.5.2).
- `branch_or_ref_label` — redaction-aware.
- `summary`.

#### 3.2.5 `clone_depth_disclosure`

- `clone_depth_class` — closed set (§3.5.3).
- `shallow_depth_n` — integer, optional. Required when
  `clone_depth_class = shallow_n_commits`.
- `partial_filter_class` — closed set (§3.5.4). Required when
  `clone_depth_class = partial_clone_filtered`.
- `single_branch_only` — boolean.
- `summary`.

#### 3.2.6 `lfs_state_disclosure`

- `lfs_class` — closed set (§3.5.5).
- `summary`.

A clone review MUST NOT collapse `lfs_pointer_only_clone` or
`lfs_hydrate_pending` into `no_lfs_required`; the upstream
`source_acquisition_and_bootstrap_seed.md` rule §2.7 / §2.8 binds
this distinction.

#### 3.2.7 `submodule_state_disclosure`

- `submodule_class` — closed set (§3.5.6).
- `summary`.

A clone review MUST NOT collapse `submodule_init_pending`,
`submodule_init_partial`, or `submodule_init_blocked` into
`no_submodules`.

#### 3.2.8 `destination_disclosure`

- `destination_path_label` — redaction-aware; never a raw
  absolute path.
- `destination_disposition` — re-export of
  `destination_disposition` (`no_write`,
  `write_to_labelled_staging`, `write_to_user_destination`,
  `write_to_durable_workspace`, `write_to_admin_owned_root`).
- `collision_class` — re-export of `collision_class` (`no_collision`,
  `reuse_existing`, `add_existing`, `clone_elsewhere`,
  `reveal_only`, `destination_blocked`).
- `destination_collision_sheet_ref` — required when
  `collision_class != no_collision`.
- `staging_label` — redaction-aware label, required when
  `destination_disposition = write_to_labelled_staging`.
- `summary`.

### 3.3 `next_step_choice_class` (closed)

The user's chosen post-clone next step. The set re-projects the
verb-matrix `clone` resulting modes (project_entry_contract.md §3)
into a single resolved choice. A clone review that commits without
exactly one resolved class is non-conforming.

- `clone_only` — clone bytes only; no open, no add, no review
  follow-on. Pairs with `resulting_mode = clone_only`.
- `clone_and_review` — clone, then route through the post-clone
  review surface (e.g. trust review, archetype review, policy
  review). Pairs with `resulting_mode = clone_then_review`.
- `clone_and_open` — clone, then route into the open-flow for the
  cloned root. The trust prompt still fires before the open
  commits; clone never grants trust. Pairs with
  `resulting_mode = clone_then_open`.
- `clone_and_add` — clone, then add the cloned root into the
  active workspace via the `add_root` verb. The
  `add_root_to_active_workspace` sheet still fires before the root
  is admitted. Pairs with `resulting_mode = clone_then_add`.

### 3.4 `staging_durability_disclosure`

Every clone activation that uses staging or temporary
materialization before a final destination is chosen MUST label the
write as non-durable. A surface that hides staging behind generic
progress copy is non-conforming.

- `staging_class` — closed set:
  - `non_durable_staging` — bytes land in labelled staging only.
    Promotion to a durable destination requires a separate
    reviewed step.
  - `durable_destination_direct` — bytes land directly in the
    user-chosen durable destination. No staging involved; the
    user has confirmed the destination before commit.
  - `no_write` — review-only / cancel path; no bytes land.
- `non_durable_label_visible` — boolean. MUST be `true` when
  `staging_class = non_durable_staging`. A clone review that emits
  `non_durable_staging` with `non_durable_label_visible = false`
  is non-conforming.
- `durable_promotion_requires_review` — boolean. MUST be `true`
  when `staging_class = non_durable_staging`. Promotion to a
  durable destination is always a separate reviewed step.
- `staging_label` — redaction-aware label that names the staging
  area to the user. Required when `staging_class =
  non_durable_staging`.
- `summary`.

### 3.5 Closed sets

#### 3.5.1 `certificate_class`

- `trusted_chain_verified`
- `pinned_chain_verified`
- `self_signed_user_pinned`
- `self_signed_user_review_required`
- `certificate_unknown_authority`
- `certificate_expired`
- `certificate_invalid`
- `certificate_not_evaluated`

A clone review whose `certificate_class` is
`self_signed_user_review_required`,
`certificate_unknown_authority`, `certificate_expired`, or
`certificate_invalid` MUST deny commit until the user accepts a
typed certificate-review hook (a `next_step_decision_hook` of
`review_trust_and_open` is the seeded route).

#### 3.5.2 `branch_or_ref_class`

- `default_branch` — server-declared default branch (e.g. HEAD).
- `named_branch` — branch the user picked or the row pinned.
- `named_tag` — tag.
- `commit_sha` — pinned commit.
- `named_ref_pinned` — non-branch / non-tag named ref (e.g.
  `refs/pull/*`).
- `ref_undeclared` — the row did not pin a ref; the chooser
  resolves to `default_branch` on commit. A row that emits
  `ref_undeclared` AND `single_branch_only = true` is
  non-conforming.

#### 3.5.3 `clone_depth_class`

- `full_history`
- `shallow_n_commits`
- `partial_clone_filtered`
- `sparse_checkout`
- `single_branch_only`
- `mixed_strategy` — combination of two or more depth strategies
  (e.g. shallow + sparse). Surfaces MUST disclose each component
  in the summary; a `mixed_strategy` review that hides components
  is non-conforming.

#### 3.5.4 `partial_filter_class`

- `none`
- `blob_none`
- `blob_size_limit`
- `tree_zero`
- `custom`

#### 3.5.5 `lfs_class`

- `no_lfs_required`
- `lfs_pointer_only_clone`
- `lfs_hydrate_pending`
- `lfs_hydrate_excluded`
- `lfs_unknown`

#### 3.5.6 `submodule_class`

- `no_submodules`
- `submodule_init_pending`
- `submodule_init_partial`
- `submodule_init_complete`
- `submodule_init_skipped`
- `submodule_init_blocked`

### 3.6 Acceptance rules (clone-row level)

1. **Clone never implies trust.** A clone-review row whose
   `host_posture_disclosure` resolves and whose certificate /
   auth disclosures resolve still does not grant trust; the
   `post_clone_trust_stage_record` fires before any trust
   transition. A row that promotes `clone_then_open` straight to
   trusted without an admission step is non-conforming.
2. **Clone never installs.** A clone-review row whose
   `next_step_choice_class` is `clone_and_open` MUST NOT trigger
   dependency restore, extension recommendation, or repo-hook
   execution at clone time. Those side effects are owned by the
   post-clone trust-stage record (§5) and the open-flow that runs
   AFTER admission.
3. **Clone never restores dependencies.** Package restore,
   toolchain install, generator install / run, devcontainer
   attach, prebuild attach, and extension restore MUST be deferred
   to the post-clone trust-stage record; the clone review only
   previews them.
4. **Clone never executes repo hooks.** Repository hooks
   (post-checkout, post-clone, configured tasks) MUST NOT execute
   at clone time. They are gated behind the post-clone admission.
5. **Staging is labelled.** When `staging_class =
   non_durable_staging`, the surface MUST render the staging
   label verbatim and MUST NOT collapse staging into the final
   durable target in progress copy or in the post-clone
   confirmation row.
6. **Destination is explicit.** The clone review MUST disclose
   the destination path label and disposition before commit; an
   activation that commits before resolving the destination
   (e.g. defaults to "somewhere in temp") is non-conforming.

## 4. Destination-collision sheet record

A clone activation whose chosen destination resolves to a
non-`no_collision` `collision_class` MUST emit one
`destination_collision_sheet_record` BEFORE bytes land. The
collision sheet is the typed projection of the destination's prior
state onto the safe-action set; it never hides overwrite, never
opaquely picks a temp directory, and never silently reuses the
existing target.

A surface that resolves a collision without emitting the record is
non-conforming.

### 4.1 Required fields

- `record_kind = destination_collision_sheet_record`.
- `destination_collision_schema_version = 1`.
- `destination_collision_id`.
- `clone_review_ref` — back-link to the parent
  `clone_review_record`.
- `destination_path_label` — redaction-aware; never raw.
- `destination_collision_class` — exactly one of the closed set
  (§4.2).
- `safe_actions[]` — non-empty set drawn from the closed set
  (§4.3). The set MUST NOT include `clone_elsewhere` only when the
  collision is `destination_blocked_by_policy`; in that case at
  least one of `cancel_no_change`, `inspect_only`,
  `request_admin_help`, or `return_to_start_center` MUST appear.
- `temp_directory_disclosed` — boolean. MUST be `true` whenever
  the collision sheet's resolution lands bytes in a temp / staging
  area. A collision sheet that resolves to a temp directory
  without disclosing the staging label is non-conforming.
- `explicit_overwrite_required` — boolean. MUST be `true`
  whenever the resolution would overwrite existing durable bytes.
  An implicit-overwrite resolution is non-conforming.
- `summary` — redaction-aware.

Optional fields: `existing_target_descriptor_ref` (opaque ref to
the prior `filesystem_identity_record` /
`workspace_file_manifest_ref` / `recovery_checkpoint_ref` the
collision targets), `previously_cloned_target_ref` (opaque ref to
the prior `project_entry_action_record` whose target matches),
`policy_narrowing_ref` (opaque ref into the
`checkout_plan_record.policy_narrowing` envelope when the
collision is policy-blocked).

### 4.2 `destination_collision_class` (closed)

- `existing_path_non_empty` — destination path exists and is not
  empty; not a repo, not a workspace file.
- `existing_path_empty_writable` — destination path exists and
  is empty / writable; reuse may proceed without overwrite.
- `existing_path_read_only` — destination path exists but is
  read-only; clone cannot proceed without elevation.
- `existing_repo_root_match` — destination is already a repo root
  whose origin matches the clone target. `reuse_existing_repo` is
  the safe default.
- `existing_repo_root_mismatch` — destination is already a repo
  root whose origin differs from the clone target.
- `existing_worktree_at_path` — destination is a git worktree of
  another repo.
- `existing_workspace_file_at_path` — destination contains an
  Aureline workspace / workset manifest.
- `previously_cloned_target_match` — Aureline has previously
  cloned this exact remote into this exact destination; the row
  routes to `add_existing_to_workspace` or `reuse_existing_repo`.
- `previously_cloned_target_mismatch` — Aureline has previously
  cloned this remote, but to a different destination.
- `destination_blocked_by_policy` — admin / fleet policy or trust
  policy denies writes to this destination.

### 4.3 `safe_action_class` (closed)

- `reuse_existing_repo` — admit the existing repo as the clone
  target without re-fetching durable bytes; the clone review
  resolves to `clone_only` or `clone_and_open` against the
  existing root.
- `add_existing_to_workspace` — re-route the activation through
  the `add_root` verb; the existing root is admitted as a workspace
  root.
- `clone_elsewhere` — change the destination path; the clone
  review re-renders the destination disclosure and re-evaluates
  the collision class.
- `reveal_in_filesystem` — open the existing target in the OS
  shell so the user can inspect it before deciding.
- `cancel_no_change` — cancel the clone; nothing changes.
- `inspect_only` — open the existing target read-only without
  admitting trust.
- `request_admin_help` — escalate to a typed admin-help review
  hook (used by `existing_path_read_only` and
  `destination_blocked_by_policy`).
- `locate_missing_target` — re-pick an existing target whose
  expected location moved.
- `return_to_start_center` — return to the Start Center / chooser
  surface that originated the clone.

### 4.4 Acceptance rules (collision-sheet level)

1. **Reveal before overwrite.** The collision sheet MUST surface
   `reveal_in_filesystem` whenever the existing target is a
   directory, repo root, or workspace file. A sheet that omits
   reveal on those classes is non-conforming.
2. **No opaque temp directories.** When the resolution requires
   temp / staging materialization, the sheet MUST set
   `temp_directory_disclosed = true` and MUST cite the staging
   label on the parent `clone_review_record`. A resolution that
   silently writes to a system-default temp path is
   non-conforming.
3. **No hidden overwrite.** When the resolution would overwrite
   existing durable bytes, the sheet MUST set
   `explicit_overwrite_required = true` and MUST require an
   explicit user confirmation distinct from the primary commit
   affordance.
4. **Policy block routes through admin help.** A sheet whose
   `destination_collision_class = destination_blocked_by_policy`
   MUST include `request_admin_help` (or `return_to_start_center`
   when the deployment envelope makes admin help unavailable) in
   its safe actions. A policy-blocked sheet that exposes
   `clone_elsewhere` to an admin-restricted location is
   non-conforming.
5. **Repo root match prefers reuse.** A sheet whose
   `destination_collision_class = existing_repo_root_match` MUST
   list `reuse_existing_repo` first in `safe_actions[]`. The
   re-clone path is allowed only after explicit user choice.
6. **Workspace-file match prefers add-existing.** A sheet whose
   `destination_collision_class =
   existing_workspace_file_at_path` MUST list
   `add_existing_to_workspace` first in `safe_actions[]`.

## 5. Post-clone trust-stage record

Every clone activation emits one `post_clone_trust_stage_record`
AFTER bytes land in the materialization target (staging or
durable) and BEFORE any post-clone side effect. The record exists
so a clone activation cannot smuggle in trust grant, dependency
restore, extension recommendation, or hook / task execution as a
"part of clone" step.

### 5.1 Required fields

- `record_kind = post_clone_trust_stage_record`.
- `post_clone_trust_stage_schema_version = 1`.
- `stage_id`.
- `clone_review_ref` — back-link to the parent
  `clone_review_record`.
- `materialization_class` — closed set (§5.2).
- `materialization_durability` — closed set (§5.3).
- `trust_grant_at_clone` — boolean. MUST be `false`. A record that
  emits `true` is non-conforming.
- `dependency_restore_at_clone` — boolean. MUST be `false`. A
  record that emits `true` is non-conforming.
- `extension_recommendation_at_clone` — boolean. MUST be `false`.
- `hook_or_task_execution_at_clone` — boolean. MUST be `false`.
- `reviewed_step_required_to_open` — boolean. MUST be `true`. A
  clone activation that exposes an "Open" affordance with no
  reviewed step is non-conforming.
- `safest_next_action` — exactly one of the closed set (§5.4).
  The post-clone surface's primary affordance MUST resolve to this
  value.
- `post_clone_actions_offered[]` — non-empty subset of the
  `safest_next_action` set (§5.4). A surface that offers only the
  primary action (no fallback) is non-conforming.
- `exact_target_identity_preserved` — boolean. MUST be `true`. A
  post-clone surface that paraphrases the target identity (e.g.
  "the cloned repo" instead of the canonical normalized URL +
  destination path label) is non-conforming.
- `summary` — redaction-aware.

Optional fields: `target_identity_label` (redaction-aware label
the surface renders to make the exact target identity visible),
`checkpoint_linked_recovery_class` (re-export of the
`checkpoint_linked_recovery_class` set; required when the
materialization landed durable bytes).

### 5.2 `materialization_class` (closed)

- `bytes_in_non_durable_staging` — bytes landed in labelled
  staging only. Pairs with `staging_class = non_durable_staging`.
- `bytes_in_user_destination_pending_review` — bytes landed in
  the user-chosen destination but the destination has not yet
  been admitted as a durable workspace.
- `bytes_in_durable_workspace` — bytes landed in a durable
  workspace and the user has explicitly chosen this disposition.
- `materialization_failed` — clone failed mid-flight; bytes are
  partial. The post-clone surface routes to recovery hooks (see
  `safest_next_action`).

### 5.3 `materialization_durability` (closed)

- `non_durable_until_open` — staging is non-durable until the
  user commits a reviewed open / add / review step. Pairs with
  `materialization_class = bytes_in_non_durable_staging` and
  `bytes_in_user_destination_pending_review`.
- `durable_after_review` — bytes are durable after the user
  reviewed the trust prompt and admission. Pairs with
  `materialization_class = bytes_in_durable_workspace`.
- `durable_after_user_choice` — the user explicitly chose a
  durable destination disposition (no staging) and admission
  fired immediately after the destination was chosen. Pairs with
  `staging_class = durable_destination_direct`.

### 5.4 `safest_next_action` (closed)

- `review_trust_and_open` — route into the trust-prompt surface,
  then into the open-flow for the cloned root.
- `compare_before_open` — open the cloned tree read-only for
  comparison before admitting trust.
- `inspect_only` — open the cloned tree read-only without
  admitting trust.
- `set_up_later` — defer dependency restore, extension
  recommendation, and hook execution; keep the cloned root
  inspectable.
- `open_minimal` — open the cloned root in a minimal-
  capability mode (no dependency restore, no extension activation,
  no hooks).
- `return_to_start_center` — return to the Start Center; the
  clone is preserved in staging or in the chosen destination but
  no admission fires.
- `roll_back_clone` — discard staging or the chosen destination
  and return to the prior workspace. Required when
  `materialization_class = materialization_failed`.

### 5.5 Acceptance rules (post-clone trust-stage level)

1. **Materialization is separate from trust.** The post-clone
   record fires AFTER bytes land. Trust admission is a separate,
   user-initiated step routed through the trust-prompt surface.
   A surface that grants trust as a side effect of clone is
   non-conforming.
2. **No silent dependency restore.** Package restore, toolchain
   install, generator install / run, devcontainer attach,
   prebuild attach, and extension restore MUST NOT fire until
   the user commits a reviewed open / admission. The post-clone
   record offers `set_up_later` and `open_minimal` so the user
   can defer side effects.
3. **No silent extension recommendations.** Extension prompts
   MUST NOT fire as part of clone. They are owned by the
   `extension_prompt` surface that fires after admission.
4. **No silent repo-hook execution.** post-checkout / post-clone
   / configured-task hooks MUST NOT fire at clone time. They are
   gated by the post-clone trust-stage record and execute only
   after the user admits trust and explicitly chooses an open /
   add disposition.
5. **Exact target identity preserved.** The post-clone surface
   MUST render the exact normalized remote URL label and
   destination path label that the clone review committed. Vague
   copy ("Clone complete") that hides target identity is
   non-conforming.
6. **Safest next action visible.** The post-clone surface's
   primary affordance MUST resolve to `safest_next_action`. The
   primary affordance for a non-durable staging clone MUST be
   `review_trust_and_open`, `inspect_only`, or `set_up_later`;
   it MUST NOT be a one-click `Open and run` shortcut.
7. **Failure routes to roll-back.** A
   `materialization_class = materialization_failed` record MUST
   include `roll_back_clone` in
   `post_clone_actions_offered[]`. The post-clone surface MUST
   NOT auto-retry execution against partial bytes.

## 6. Cross-surface invariants

The clone review, collision sheet, and post-clone trust-stage
records project into every chooser surface that exposes the `clone`
verb (Start Center, palette, drag-drop preview, system-open
handoff, CLI / headless preview, deep-link intent review,
workspace-switcher). The following invariants keep the projection
sound:

1. **One clone-review record per clone activation.** Every clone
   activation emits exactly one `clone_review_record`. A surface
   that commits a clone without emitting the record is
   non-conforming.
2. **One collision-sheet record per non-`no_collision` clone
   activation.** When the chosen destination resolves to a
   non-`no_collision` class, the surface MUST emit one
   `destination_collision_sheet_record` BEFORE bytes land.
3. **One post-clone trust-stage record per clone activation.**
   Every clone activation emits exactly one
   `post_clone_trust_stage_record` AFTER materialization and
   BEFORE any post-clone side effect. A clone activation that
   skips this record is non-conforming.
4. **Same record across surfaces.** When the same clone
   activation can be reached from Start Center, palette,
   drag-drop preview, system-open handoff, and CLI / headless
   preview, the records emitted on each surface MUST agree on
   the eight required disclosure axes (§3.2), the resolved
   `next_step_choice_class`, the staging-durability disclosure,
   and the post-clone `materialization_class` /
   `materialization_durability` / `safest_next_action`.
   Surface-local chrome (icon, accelerator, accent) may differ;
   semantics may not.
5. **CLI / headless render the same axes.** A CLI clone
   preview (e.g. `aureline clone --preview` /
   `aureline clone --dry-run`) MUST render the same disclosure
   axes as the GUI clone-review sheet. Collapsing axes (e.g.
   omitting `lfs_state_disclosure` because the CLI does not
   render badges) is non-conforming.
6. **Drag-drop is a chooser surface, not a shortcut.** A drop
   that resolves to a remote URL MUST emit one
   `clone_review_record` and (when relevant) one
   `destination_collision_sheet_record` BEFORE bytes land. A drop
   that silently materializes bytes bypasses §3.6 and §4.4 and is
   non-conforming.
7. **Deep links route through the clone review.** A deep link
   that resolves to a clone activation MUST emit a
   `clone_review_record` (rendered inside the
   `deep_link_intent_review` open-flow sheet) before any commit;
   it MUST NOT call a private clone opener.
8. **Staging labels survive surface translation.** When a
   `non_durable_staging` clone is rendered in the CLI, the
   notification / progress row, the support bundle, or the
   history view, the staging label MUST remain visible. A
   surface that renders the durable destination as the staging
   label is non-conforming.

## 7. Fixture corpus

The fixture corpus under
`/fixtures/ux/clone_review_cases/` contains worked records for the
required clone-review, collision, and post-clone trust-stage
scenarios:

- `clone_review_https_default_branch.yaml` — baseline HTTPS clone
  review with OAuth, default branch, full history, no LFS / no
  submodules; staging non-durable; next step `clone_and_review`.
- `clone_review_oauth_lfs_submodules.yaml` — OAuth clone with
  LFS pointer-only and pending submodules; staging non-durable;
  next step `clone_and_review`.
- `clone_review_mirror_partial_clone.yaml` — mirror-served
  clone with `partial_clone_filtered` (`blob_none`),
  shallow-history components, and `mirror_lagged` freshness;
  staging non-durable; next step `clone_and_open`.
- `clone_review_self_signed_certificate_review.yaml` — clone
  whose `certificate_class =
  self_signed_user_review_required`; commit denied until the
  user accepts a typed certificate-review hook.
- `destination_collision_existing_repo_root_match.yaml` — repo
  root matches the clone target; safe actions =
  `[reuse_existing_repo, reveal_in_filesystem, cancel_no_change,
  inspect_only]`.
- `destination_collision_existing_workspace_file.yaml` —
  workspace-file at path; safe actions =
  `[add_existing_to_workspace, reveal_in_filesystem,
  cancel_no_change]`.
- `destination_collision_existing_path_non_empty.yaml` —
  non-empty path; safe actions = `[clone_elsewhere,
  reveal_in_filesystem, cancel_no_change, inspect_only]`;
  `explicit_overwrite_required = true` if the user chooses
  overwrite.
- `destination_collision_destination_blocked_by_policy.yaml` —
  policy-blocked destination; safe actions =
  `[cancel_no_change, request_admin_help,
  return_to_start_center]`; `clone_elsewhere` not exposed.
- `post_clone_trust_stage_non_durable_staging.yaml` —
  post-clone record for a staging-only materialization; safest
  next action = `review_trust_and_open`; all four "no implicit
  grant" booleans = `false`.
- `post_clone_trust_stage_durable_after_review.yaml` —
  post-clone record after a reviewed admission landed bytes in a
  durable workspace; `materialization_durability =
  durable_after_review`; safest next action = `set_up_later` or
  `open_minimal` depending on profile.
- `post_clone_trust_stage_materialization_failed.yaml` —
  failed materialization; safest next action = `roll_back_clone`.

Each fixture is a YAML document validated by the
`/schemas/ux/clone_review.schema.json` boundary schema and includes
a `__fixture__` prelude naming the scenario, the record kind
exercised, the disclosure axes covered, and the contract sections
asserted.

## 8. Versioning and change control

The schema declares
`clone_review_schema_version = 1`,
`destination_collision_schema_version = 1`, and
`post_clone_trust_stage_schema_version = 1`. Adding a new
`certificate_class`, `branch_or_ref_class`, `clone_depth_class`,
`partial_filter_class`, `lfs_class`, `submodule_class`,
`next_step_choice_class`, `staging_class`,
`destination_collision_class`, `safe_action_class`,
`materialization_class`, `materialization_durability`, or
`safest_next_action` is **additive-minor** and bumps the schema
version. Repurposing an existing value, weakening a §3.6, §4.4, or
§5.5 invariant, or relaxing the four "no implicit grant" booleans
in §5 is **breaking** and requires a new ADR row plus a coordinated
update of the project-entry contract (§5 of
`project_entry_contract.md`), the open-flow sheet schema, the
source-acquisition seed, and the workspace-entry route matrix.
