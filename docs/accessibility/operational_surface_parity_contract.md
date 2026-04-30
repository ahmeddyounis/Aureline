# Worktree, Sequence, Stash, and Recovery Operational-Surface Parity Contract

Status: seeded

This contract makes risky version-control and recovery surfaces meet the
same accessibility bar as the editor instead of becoming second-class
operational UIs. It applies to worktree managers, stash shelves,
sequence editors (interactive rebase, cherry-pick series, revert series,
patch-series restack, merge replay), recovery panes and recovery
banners, conflict resolution views, force-push or publish review
sheets, and patch apply or revert review sheets.

The contract does not invent a second accessibility language. It binds
existing announcement-event, accessibility-tree, and focus/zoom/pointer
contracts onto the closed action and state vocabulary risky operational
surfaces require so a screen-reader, keyboard-only, high-zoom, or
larger-text user can complete continue, abort, skip, restore, conflict,
checkpoint, and recovery flows without ambiguity.

Contract identity:

- `operational_surface_parity_contract_id:
  aureline.accessibility.operational_surface_parity`
- `operational_surface_parity_contract_revision: 1`
- `operational_surface_state_schema_version: 1`

Companion artifacts:

- [`/schemas/accessibility/operational_surface_state.schema.json`](../../schemas/accessibility/operational_surface_state.schema.json)
  defines `operational_surface_parity_row_record`,
  `operational_surface_action_record`,
  `operational_surface_state_record`,
  `operational_surface_case_record`, and
  `operational_surface_export_projection_record`.
- [`/artifacts/accessibility/operational_surface_rows.yaml`](../../artifacts/accessibility/operational_surface_rows.yaml)
  enumerates the per-surface parity rows, required actions, required
  states, and parity-dimension claims.
- [`/fixtures/accessibility/operational_surface_cases/`](../../fixtures/accessibility/operational_surface_cases/)
  contains seed cases for an interactive rebase sequence step, a stash
  restore, a worktree switch, and a corruption-recovery pane.
- [`/docs/accessibility/screen_reader_and_live_region_contract.md`](./screen_reader_and_live_region_contract.md)
  owns announcement event types, live-region channels, dedupe and
  coalescing, stable phrasing, and silence rules. Operational-surface
  rows reuse `vcs_worktree_state_changed`,
  `vcs_stash_state_changed`, `vcs_sequence_state_changed`,
  `recovery_state_changed`, and `conflict_state_changed` rather than
  minting per-surface narration.
- [`/docs/accessibility/accessibility_tree_contract.md`](./accessibility_tree_contract.md)
  owns role/name/state mapping, virtualization rules, position truth,
  and inspector snapshots. Operational-surface rows reuse the existing
  `list`, `tree`, `table_grid`, `diff_review`, and `status_notification`
  surface families.
- [`/docs/accessibility/focus_zoom_and_pointer_independence_contract.md`](./focus_zoom_and_pointer_independence_contract.md)
  owns focus ownership, focus return, visible focus indicators, the 50%
  to 400% zoom contract, and pointer-independence equivalents.
  Operational-surface rows cite `focus_owner_snapshot_record` and
  `keyboard_equivalent` records rather than redefining them.
- [`/docs/vcs/history_edit_and_recovery_contract.md`](../vcs/history_edit_and_recovery_contract.md),
  [`/schemas/vcs/sequence_step.schema.json`](../../schemas/vcs/sequence_step.schema.json),
  and
  [`/schemas/vcs/recovery_object.schema.json`](../../schemas/vcs/recovery_object.schema.json)
  own the underlying history-edit, sequence-editor, and recovery object
  models that operational-surface state records cite.
- [`/docs/ux/prompt_grammar_contract.md`](../ux/prompt_grammar_contract.md)
  owns destructive, trust, approval, and consent-renewal prompt grammar
  that publish/promote/rollback review sheets reuse.
- [`/artifacts/accessibility/assistive_tech_coverage_rows.yaml`](../../artifacts/accessibility/assistive_tech_coverage_rows.yaml),
  [`/artifacts/accessibility/accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml),
  and
  [`/artifacts/accessibility/shell_conformance_checklist.yaml`](../../artifacts/accessibility/shell_conformance_checklist.yaml)
  remain the canonical assistive-technology, tree-coverage, and shell
  conformance registries this contract cites.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` Section 11 requires keyboard access,
  visible focus, zoom from 50% to 400%, larger text, cursor controls,
  and assistive-technology reachability across product surfaces.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Section 23 and
  Appendix BU require structured announcements and assistive-technology
  regression journeys that explicitly cover risky VCS and recovery
  workflows.
- `.t2/docs/Aureline_Technical_Design_Document.md` Sections 8.13 and
  8.44 require accessibility parity across inclusive surfaces, history
  edits, conflict resolution, and recovery panes; mouse-only,
  drag-only, hover-only, and color-only critical paths are forbidden.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Sections 19.1, 19.5, 19.11,
  Appendix G, and Appendix CD define live-region behavior, stable
  message IDs, and dense-surface accessibility templates that
  operational surfaces must continue to meet.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` Sections 28.3-28.5
  name the screen-reader coverage set and critical announcement classes
  that include worktree, stash, sequence, conflict, and recovery
  posture.

## Scope

In scope:

- the seven operational surface families enumerated in
  `operational_surface_family`;
- closed action and state vocabularies risky flows must use;
- reuse of announcement, accessibility-tree, and focus/zoom/pointer
  contracts;
- support and export hooks that name the exact operational surface row,
  state class, action class, recovery object, announcement event, tree
  node, and focus snapshot under test.

Out of scope:

- platform-specific assistive-tech adapter implementation;
- the underlying VCS, recovery, or merge-queue state machine, which is
  owned by the history-edit and recovery contracts;
- byte-exact vendor-specific screen-reader strings.

## Closed Operational-Surface Families

The contract is closed to seven families. Each family is a row in the
parity artifact and must satisfy the full parity-dimension matrix:

| Family | Examples |
|---|---|
| `worktree_manager` | Worktree list, switch, create, remove, repair, dirty review. |
| `stash_shelf` | Stash entries, apply, pop, drop, compare, branch-from-stash. |
| `sequence_editor` | Interactive rebase, cherry-pick series, revert series, patch-series restack, merge replay. |
| `recovery_pane` | Corruption recovery, reflog browse, checkpoint restore, recovery banners. |
| `conflict_resolution_view` | Three-way conflict review for sequence steps, applies, reverts, merges. |
| `force_push_or_publish_review_sheet` | Force push, publish, promote, rollback, yank review sheets. |
| `patch_apply_or_revert_review_sheet` | AI-suggested patch apply or revert review and confirmation sheets. |

Adding a new family is additive-minor and requires a schema bump plus
matching artifact, fixture, and assistive-technology coverage rows.
Repurposing an existing value is breaking and requires a governed
decision row.

## Closed Action Vocabulary

`Continue`, `Abort`, `Skip`, `Restore checkpoint`, `Review conflict`,
`Open details`, `Open output`, `Preview consequence`, `Open diff`,
`Open recovery pane`, `Create checkpoint`, `Switch target`, and `Retry`
remain separate verbs. Each value:

- has its own `action_class` and exact one-to-one `action_label`
  rendered in plain language;
- has its own command route and keyboard route, recorded as a
  `keyboard_route` block citing a non-hover help surface;
- has its own audit event id and announcement event reuse;
- denies drag-only, hover-only, or color-only dependence as the only
  way to invoke the action;
- denies ambiguous copy such as `Are you sure?`, `Continue`, or `OK`
  when used as the destructive primary on a publish, promote,
  rollback, or revoke surface (see prompt-grammar contract).

The schema enforces label-class agreement: `action_class: continue`
must render `action_label: Continue`, `action_class: skip` must render
`action_label: Skip`, and so on. Surfaces may not silently downgrade
`Restore checkpoint` to `Restore` or merge `Skip` into `Continue`.

## Closed State Vocabulary

The closed fifteen-class state vocabulary captures every user-facing
state risky operational surfaces must announce and label:

| `state_class` | `state_label` | Required when |
|---|---|---|
| `idle_inspect_only` | `Idle` | Surface open with no operation pending. |
| `dirty_pending_review` | `Dirty` | Worktree dirty, stash created, sequence prepared but unconfirmed. |
| `in_progress` | `In progress` | Sequence step running, apply running, push in flight. |
| `paused_pending_conflict` | `Conflict` | Conflict blocks continuation. |
| `paused_pending_protected_path` | `Protected path blocker` | Protected-path policy blocks continuation. |
| `paused_pending_signoff` | `Missing signoff` | Required signoff missing. |
| `paused_pending_base_or_head_identity_changed` | `Base or head identity changed` | Auto-continue blocked because base/head identity drifted. |
| `blocking_destructive_preview` | `Destructive preview` | Destructive preview is the user's last review checkpoint. |
| `checkpoint_created` | `Checkpoint created` | A recovery checkpoint exists for this surface. |
| `checkpoint_expired` | `Checkpoint expired` | Recovery checkpoint expired before use. |
| `approval_invalidated` | `Approval invalidated` | Approval, signoff, or trust scope changed mid-flight. |
| `recovery_available` | `Recovery available` | Recovery pane offers an actionable recovery object. |
| `recovery_in_progress` | `Recovery in progress` | Recovery operation is running. |
| `completed` | `Completed` | Operation completed without rollback. |
| `aborted_rolled_back` | `Aborted` | Operation aborted; recovery objects restored. |

The schema enforces a one-to-one pairing between `state_class` and
`state_label` so no surface can ship `Conflict` styling that announces
as `In progress` or vice versa.

`paused_pending_conflict`, `paused_pending_protected_path`,
`paused_pending_signoff`, and
`paused_pending_base_or_head_identity_changed` mirror the auto-continue
blockers in the history-edit contract verbatim. Auto-continue is
structurally blocked while any of those state classes are present;
operational surfaces never collapse them into a single ambiguous
"blocked" badge.

## Required Actions Per Surface

Each parity row's `required_actions` block names the action classes a
surface MUST expose. The seed rows are:

| Family | Required actions |
|---|---|
| `worktree_manager` | `switch_target`, `open_diff`, `preview_consequence`, `abort`, `restore_checkpoint`, `open_recovery_pane`, `open_details`. |
| `stash_shelf` | `continue` (apply or pop), `abort` (drop), `preview_consequence` (compare), `restore_checkpoint`, `review_conflict`, `open_details`. |
| `sequence_editor` | `continue`, `abort`, `skip`, `restore_checkpoint`, `review_conflict`, `open_diff`, `open_details`, `create_checkpoint`. |
| `recovery_pane` | `restore_checkpoint`, `continue`, `abort`, `open_details`, `open_output`, `open_diff`. |
| `conflict_resolution_view` | `review_conflict`, `continue`, `abort`, `skip`, `restore_checkpoint`, `open_diff`. |
| `force_push_or_publish_review_sheet` | `preview_consequence`, `continue`, `abort`, `open_diff`, `open_details`, `restore_checkpoint`. |
| `patch_apply_or_revert_review_sheet` | `preview_consequence`, `continue`, `abort`, `restore_checkpoint`, `open_diff`, `open_details`, `retry`. |

Surfaces may expose additional non-required actions, but they MUST NOT
hide required actions behind hover-only, drag-only, or color-only
affordances. Every required action carries a keyboard route and a
non-hover help surface ref.

## Required States Per Surface

Each parity row's `required_states` block names the state classes a
surface MUST be able to render. The seed rows are:

| Family | Required states |
|---|---|
| `worktree_manager` | `idle_inspect_only`, `dirty_pending_review`, `blocking_destructive_preview`, `checkpoint_created`, `recovery_available`, `completed`, `aborted_rolled_back`. |
| `stash_shelf` | `idle_inspect_only`, `dirty_pending_review`, `paused_pending_conflict`, `checkpoint_created`, `completed`, `aborted_rolled_back`. |
| `sequence_editor` | `idle_inspect_only`, `in_progress`, `paused_pending_conflict`, `paused_pending_protected_path`, `paused_pending_signoff`, `paused_pending_base_or_head_identity_changed`, `checkpoint_created`, `completed`, `aborted_rolled_back`. |
| `recovery_pane` | `idle_inspect_only`, `recovery_available`, `recovery_in_progress`, `checkpoint_created`, `checkpoint_expired`, `completed`, `aborted_rolled_back`. |
| `conflict_resolution_view` | `idle_inspect_only`, `paused_pending_conflict`, `in_progress`, `completed`, `aborted_rolled_back`. |
| `force_push_or_publish_review_sheet` | `idle_inspect_only`, `blocking_destructive_preview`, `approval_invalidated`, `paused_pending_signoff`, `completed`, `aborted_rolled_back`. |
| `patch_apply_or_revert_review_sheet` | `idle_inspect_only`, `blocking_destructive_preview`, `in_progress`, `paused_pending_conflict`, `approval_invalidated`, `completed`, `aborted_rolled_back`. |

## Parity Dimensions

Every parity row reports its state for each closed dimension below.
The schema requires all 13 dimensions to appear in every
`operational_surface_parity_row_record`.

| Dimension | What it certifies |
|---|---|
| `screen_reader_announcement` | The surface emits the correct announcement event class for each state change with the correct urgency, dedupe, and message id, reusing the announcement contract. |
| `accessibility_tree_role_name_state` | Tree nodes carry the correct role, name, position, and state mapping for rows, action buttons, and live-region nodes. |
| `full_keyboard_traversal` | Every required action and every row movement reaches via keyboard alone, including row movement, details, context menu, reorder, continue, abort, skip, restore, and conflict review. |
| `focus_order_and_return` | Focus order is logical, focus return on overlay close is preserved or denied, and focus indicators meet the 2 px / 3:1 minimum. |
| `zoom_50_to_400_percent` | State, blockers, counts, and recovery controls remain reachable from 50% to 400% zoom; secondary actions may move to overflow but never disappear. |
| `larger_text_and_line_height` | Larger text, increased line height, and cursor settings do not collapse or clip state badges, blocker reasons, or counts. |
| `plain_language_action_labels` | Every required action uses the closed `action_label` vocabulary; ambiguous copy is denied. |
| `plain_language_state_labels` | Every required state uses the closed `state_label` vocabulary; color-only state cues are forbidden as the sole signal. |
| `non_color_state_cue` | State is communicated by shape, label, outline, or text in addition to color. |
| `live_region_dedupe_and_coalesce` | Repeated polling, repaints, and reorders do not spam screen readers; only meaning changes announce. |
| `durable_fallback_row` | A durable surface (worktree manager, stash shelf, sequence editor, recovery pane, conflict view) preserves identity and state when a transient announcement is silenced or coalesced. |
| `no_drag_only_dependency` | No required action is drag-only, hover-only, or pointer-only; pointer gestures may remain accelerators. |
| `support_export_repro` | Support packets and repro bundles can name the exact parity row, action, state, recovery object, announcement event, tree node, and focus snapshot under test. |

A row reports `parity_state` per dimension from the closed seven-class
vocabulary `passed`, `degraded`, `partial`, `blocked`, `failed`,
`pending_evidence`, or `unclaimed`. `partial` maps to `degraded` when
composing with older result-state vocabularies.

## Reuse Rules

This contract MUST NOT mint a second accessibility language. Each
operational-surface record reuses canonical accessibility records
through opaque refs:

- `announcement_event_id_ref` cites a record under
  `schemas/accessibility/announcement_event.schema.json`. Worktree,
  stash, and sequence state changes use
  `vcs_worktree_state_changed`, `vcs_stash_state_changed`, or
  `vcs_sequence_state_changed`. Recovery panes use
  `recovery_state_changed`. Conflict resolution views use
  `conflict_state_changed`. Force-push, publish, and patch apply review
  sheets reuse `ai_patch_apply_succeeded`,
  `ai_patch_apply_failed`, `ai_patch_revert_succeeded`,
  `task_completed`, `task_failed`, or `task_blocked` as appropriate.
- `tree_node_ref` cites a record under
  `schemas/accessibility/tree_node.schema.json`. Required surface
  families map to `list`, `tree`, `table_grid`, `diff_review`, or
  `status_notification`.
- `focus_owner_snapshot_ref` cites a record under
  `schemas/accessibility/focus_owner.schema.json` carrying the focus
  owner, return path, overlay stack, and zoom context for the
  operational surface.
- `recovery_object_record_ref` cites a record under
  `schemas/vcs/recovery_object.schema.json` so a reviewer can confirm a
  named recovery object exists before any topology-changing operation
  proceeds.
- `command_ref` on every `keyboard_route` cites the surface command
  registry; non-hover help surface refs point at command palette,
  keyboard help, accessible description, screen-reader action menu,
  status bar, context menu, or docs help.

If a surface cannot supply the required refs, it MUST report the
affected parity dimension as `degraded`, `partial`, `blocked`, or
`failed` rather than passing silently.

## Live-Region Composition

Operational surfaces inherit live-region rules from the announcement
contract:

- `polite` for ambient and status changes such as a worktree switch
  preview or a stash apply success;
- `assertive` for safety-critical changes that block the current
  action: paused-pending-conflict, paused-pending-signoff,
  approval-invalidated, blocking-destructive-preview entry, and
  recovery-in-progress;
- `silent` only when the focused durable surface already conveys the
  same meaning, with `silent_reason: focused_surface_already_conveys_state`.

Reorder repaint, polling refresh, spinner ticks, and unchanged-meaning
recomputation MUST be silent. A repeated event with the same dedupe
key and unchanged meaning hash is suppressed. A burst on the same
target uses `coalesce_window_ms` and emits the last meaning-bearing
state.

The same canonical event is projected through live region, status
item, durable row, activity center, support packet, and release
evidence. Surfaces may differ in visual treatment but never in
meaning.

## Keyboard And Pointer Independence

Every parity row's `required_actions` ship with `keyboard_equivalent`
records under the focus/zoom/pointer contract. The minimum routes are:

- worktree manager: move between rows, inspect dirty/target, open diff
  or terminal for selected worktree, remove or repair only after
  preview, switch target;
- stash shelf: move between entries, inspect path scope and untracked
  summary, apply / pop / drop / compare / create branch through
  distinct commands;
- sequence editor: move by step, reorder via keyboard, continue /
  abort / skip / edit / restore from checkpoint, open conflict detail
  for current step;
- recovery pane: move through recovery options, inspect checkpoint or
  reflog source, restore / continue / abort / export diagnostics /
  open details;
- conflict resolution view: move between conflicts, inspect base /
  current / incoming / result summaries, choose accept-left /
  accept-right / accept-both / manual / regenerate, return to
  originating file or sequence step;
- force-push or publish review sheet: preview consequence, switch
  target, abort, continue with named primary action label;
- patch apply or revert review sheet: preview consequence, continue,
  abort, retry, restore checkpoint.

Drag and pointer actions remain accelerators only. They are conforming
when the target verb is visible before commit, the keyboard or command
path reaches the same action class, modifier behavior is exposed near
the target, destructive or broad actions route through preview /
confirmation / checkpoint, and support evidence cites the equivalent
command route.

## Zoom And Larger-Text Resilience

Operational surfaces must remain functional from 50% to 400% zoom and
under larger text and platform scaling. At each zoom checkpoint:

- 50%: hit targets, focus ring, caret, and active region remain
  perceivable; state badges and counts remain readable;
- 100%: standard density and typography apply;
- 150%-200%: row heights, gutters, and status surfaces reflow without
  losing focus order; required actions remain on focused rows or move
  to focused-action menu with keyboard route preserved;
- 300%-400%: at least one risky surface row plus its required actions,
  state, blocker, and recovery control remain visible. Secondary
  decoration may move to overflow; required actions never disappear
  silently.

Disclosure rows from the focus/zoom/pointer contract apply: any
control that moves to overflow, reduces to icon-with-label, hides
until region focus, or becomes unavailable at the current scale carries
a `focus_disclosure_record` with `can_continue_without_pointer: true`.

## Plain-Language Buttons And State

The schema enforces label-class agreement on every action and state
record. Surfaces MUST NOT:

- ship `Are you sure?`, `Continue`, or `OK` as the destructive primary
  on a publish / promote / rollback / revoke / force-push surface; the
  prompt-grammar contract owns the resulting-state primary copy and
  this contract enforces the action-class to action-label mapping;
- collapse `Continue`, `Skip`, and `Abort` into one ambiguous primary;
- collapse `Restore checkpoint` into a generic `Restore`;
- present `Conflict`, `Protected path blocker`, `Missing signoff`, or
  `Base or head identity changed` as one undifferentiated `Blocked`
  badge.

## Support And Export

Operational-surface records carry a `support_export_context` block
that:

- pins `operational_surface_parity_contract_id`,
  `operational_surface_parity_contract_revision`, and
  `schema_ref: schemas/accessibility/operational_surface_state.schema.json`;
- pins the companion contract ids:
  `aureline.accessibility.screen_reader_live_region`,
  `aureline.accessibility.tree_node_taxonomy`, and
  `aureline.accessibility.focus_zoom_pointer_independence`;
- names a `task_corpus_ref` and at least one `checklist_ref`;
- declares `redaction_class` from the closed five-class vocabulary;
- forces `raw_private_material_excluded: true`.

Support packets, repro bundles, and release evidence may carry parity
row ids, action class, state class, recovery-object class, message ids,
counts, opaque refs, and bounded labels. They MUST NOT carry raw paths,
raw URLs, raw command lines, raw terminal bytes, raw prompts, raw
patch bodies, raw commit messages, raw author identity strings, raw
provider payloads, credentials, or unredacted user identifiers.

## Denial Vocabulary

The closed `denial_reason` vocabulary names the failure modes a parity
review can cite without inventing prose:

- `operational_surface_family_unresolved`
- `action_class_label_mismatch`
- `state_class_label_mismatch`
- `missing_recovery_object_ref`
- `missing_announcement_event_ref`
- `missing_accessibility_tree_node_ref`
- `missing_focus_owner_snapshot_ref`
- `missing_keyboard_route_for_action`
- `drag_only_dependency_in_critical_path`
- `color_only_state_cue_only`
- `live_region_spam_or_contradictory_output`
- `ambiguous_button_copy_for_continue_abort_skip_restore`
- `support_packet_redaction_violation`
- `schema_version_lagging`

A fixture or production capture that triggers any denial reason cannot
claim parity for the affected dimension.

## Fixture Expectations

Operational-surface fixtures must exercise:

- an interactive rebase sequence with at least one
  `paused_pending_conflict` step, a checkpoint, and the closed action
  set continue / abort / skip / restore_checkpoint / review_conflict;
- a stash restore that announces apply success and recovery path once
  and stays silent on row recomputation;
- a worktree switch that previews consequence, exposes
  `dirty_pending_review`, and offers a restore checkpoint before
  destructive removal;
- a corruption recovery pane offering a recovery object, restore,
  continue, abort, export diagnostics, and open details with focus
  return on overlay close.

Each fixture cites the announcement schema, the focus/zoom/pointer
schema, the tree-node schema, and the recovery-object schema through
opaque refs. None of the fixtures stand in for assistive-technology
adapter implementation; that remains out of scope for this contract.

## Change Discipline

Adding a new operational-surface family, action class, action label,
state class, state label, parity dimension, blocker class, recovery
object class, or denial reason is additive-minor and bumps
`operational_surface_state_schema_version`. Repurposing an existing
value is breaking and requires a new decision row. The schema, this
document, the artifact, and at least one fixture or coverage row must
be updated in the same change.
