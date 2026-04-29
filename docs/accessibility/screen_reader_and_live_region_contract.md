# Screen-reader announcement, live-region, and assistive-tech parity contract

Status: seeded

This contract turns dynamic accessibility guidance into one reusable
announcement and assistive-technology parity surface. It applies to shell,
editor, task, trust, remote, collaboration, AI, VCS, and repair workflows that
change state while a user is navigating with a screen reader, keyboard, high
zoom, high contrast, reduced motion, or a mixed input method.

Companion artifacts:

- [`/schemas/accessibility/announcement_event.schema.json`](../../schemas/accessibility/announcement_event.schema.json)
  defines the reusable event, fixture case, and support-export fields.
- [`/fixtures/accessibility/announcement_cases/`](../../fixtures/accessibility/announcement_cases)
  contains worked announcement and live-region cases.
- [`/docs/accessibility/accessibility_tree_contract.md`](./accessibility_tree_contract.md),
  [`/schemas/accessibility/tree_node.schema.json`](../../schemas/accessibility/tree_node.schema.json),
  and
  [`/schemas/accessibility/a11y_inspector_snapshot.schema.json`](../../schemas/accessibility/a11y_inspector_snapshot.schema.json)
  define the semantic tree and inspector fields that carry
  live-region node refs and announcement contract versions.
- [`/docs/accessibility/collection_announcement_contract.md`](./collection_announcement_contract.md)
  defines the dense-collection projection that supplies row position,
  selected-count, hidden-member, stale-query, and batch-scope facts to
  live-region events and durable help surfaces.
- [`/artifacts/accessibility/assistive_tech_coverage_rows.yaml`](../../artifacts/accessibility/assistive_tech_coverage_rows.yaml)
  defines launch-critical assistive-technology coverage rows.
- [`/artifacts/accessibility/assistive_tech_matrix.yaml`](../../artifacts/accessibility/assistive_tech_matrix.yaml),
  [`/artifacts/accessibility/platform_input_matrix.yaml`](../../artifacts/accessibility/platform_input_matrix.yaml),
  [`/artifacts/accessibility/accessibility_tree_coverage_rows.yaml`](../../artifacts/accessibility/accessibility_tree_coverage_rows.yaml),
  and [`/fixtures/accessibility/task_corpus_manifest.yaml`](../../fixtures/accessibility/task_corpus_manifest.yaml)
  remain the canonical platform, tree, and task-row registries.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Section 23 and
  Appendix BU define structured announcements and assistive-technology
  regression journeys.
- `.t2/docs/Aureline_Technical_Design_Document.md` Sections 8.13 and 8.44
  require accessibility parity across inclusive surfaces and risky Git /
  recovery workflows.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Sections 19.1, 19.5, 19.11,
  Appendix G, and Appendix CD define live-region behavior, stable message IDs,
  and dense-surface accessibility templates.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` Sections 28.3-28.5
  name the screen-reader coverage set and critical announcement classes.

## Contract Identity

Every event emitted under this contract carries:

- `announcement_contract_id: aureline.accessibility.screen_reader_live_region`
- `announcement_contract_revision: 1`
- `announcement_event_schema_version: 1`
- `message_id` using stable placeholder-based copy, never concatenated prose
- one assistive-technology coverage row ref when the event is under test
- one platform review row ref when the event participates in a parity claim

The contract is locale-safe: localized strings may change, but event types,
message IDs, placeholders, support/export fields, and dedupe keys remain stable.

## Event Types

All critical dynamic narration uses `announcement_event_record`. Surfaces must
not mint local string-only announcement shapes for these events.

| Event type | Default live region | Must include | Durable fallback |
|---|---|---|---|
| `diagnostic_entered_active_line` | polite | severity, line/object label, quick-fix availability when true | Problems row and editor status summary |
| `diagnostic_left_active_line` | polite or silent | prior diagnostic scope when the user is navigating diagnostics | Problems row and status summary |
| `breakpoint_added` / `breakpoint_removed` | polite | line/object label and verified posture if known | Gutter row and breakpoint list |
| `breakpoint_verified` / `breakpoint_unverified` | polite | breakpoint identity and reason when unverified | Breakpoint list and debug header |
| `task_started` / `task_completed` / `task_failed` / `task_blocked` | polite, assertive only when blocking current work | target label, outcome, blocked reason, next action | Activity row, task header, terminal summary |
| `trust_entered_restricted_mode` / `trust_granted` / `trust_revoked` | assertive when execution is blocked, otherwise polite | workspace scope, changed capability, safe next action | Trust banner and policy detail |
| `remote_attach_connecting` / `remote_attach_connected` / `remote_attach_degraded` / `remote_attach_disconnected` | polite, assertive when the active target becomes unusable | target label, local/remote boundary, available continuation | Remote status row and reconnect detail |
| `collaboration_control_requested` / `collaboration_control_granted` / `collaboration_control_revoked` | assertive when local control changes, otherwise polite | actor refs or roles, lane, authority ceiling, next safe action | Session activity lane and control detail |
| `ai_patch_ready` | polite | proposal source, review requirement, validation state | Patch review header and evidence panel |
| `ai_patch_apply_succeeded` / `ai_patch_revert_succeeded` | polite | outcome and undo/review path when available | Review header, activity row, command result packet |
| `ai_patch_apply_failed` | assertive when the active apply is blocked | typed failure class and recovery path | Failure detail and evidence panel |
| `vcs_worktree_state_changed` / `vcs_stash_state_changed` / `vcs_sequence_state_changed` | polite, assertive when a destructive or blocking state is entered | exact working context, dirty/conflict/checkpoint state, plain-language next verb | Worktree manager, stash shelf, sequence editor |
| `recovery_state_changed` / `conflict_state_changed` | polite, assertive for unresolved blockers | affected scope, remaining blockers, continue/abort/skip/restore labels | Recovery pane or conflict view |

## Live-region Policy

Each event declares a live-region policy instead of relying on per-surface
widget behavior.

Channels:

- `polite` is the default for progress, completion, diagnostics, breakpoint
  updates, remote state, AI patch readiness, and non-blocking VCS state changes.
- `assertive` is reserved for safety-critical or blocking changes: trust
  revocation, restricted-mode entry that blocks the current action, control
  loss, failed apply that leaves a reviewed patch in limbo, and recovery states
  where continuing without action risks data loss.
- `silent` is allowed only when the visible or focused semantic state already
  announces the same meaning, or when the event is an intermediate update that
  is superseded by a coalesced final announcement.

Dedupe and coalescing rules:

- A repeated event with the same `dedupe_key` and unchanged `meaning_hash` is
  suppressed.
- A rapid burst on the same target uses `coalesce_window_ms` and emits the last
  meaning-bearing state, plus a count or summary only when it helps.
- Progress updates do not announce every percentage, row append, poll, or task
  phase. They announce start, blocked states, meaningful milestones, and final
  outcome.
- Superseded intermediate states must not produce contradictory output. For
  example, a task that starts, retries, and then fails should not leave a
  queued "completed" announcement behind.
- If focus is inside the durable detail surface that already exposes row
  identity and state, background live-region delivery may be `silent` with
  `silent_reason: focused_surface_already_conveys_state`.

Silence is correct when:

- the user directly moved focus to a row whose accessible name and description
  already include the new state;
- the event only refreshes a timestamp, spinner, byte count, or unchanged
  provider identity;
- a visible state changed but the semantic meaning did not change;
- the event is an intermediate member of a burst whose terminal state is still
  within the coalescing window;
- structured output is active and narration would pollute a machine-readable
  payload.

Silence is incorrect when:

- trust, control, remote execution, patch apply, conflict, or recovery posture
  changes in a way that affects what the user may safely do next;
- the only visible cue is color, motion, icon shape, or a transient toast;
- a keyboard-triggered action changed a breakpoint, diagnostic focus, task
  state, or VCS recovery state and no focused row reports the outcome.

## Stable Phrasing

Announcement copy uses controlled message IDs and placeholders:

```text
a11y.announcement.task.failed
Task {target_label} failed. {next_action_label}
```

Rules:

- Prefer identity plus reason: "Task tests failed. Open output." is better than
  repeating stable chrome.
- Use the same nouns in UI copy, screen-reader output, CLI summaries, support
  packets, and repro bundles for the same state.
- Avoid raw paths, raw URLs, raw command lines, raw provider payloads, raw
  terminal bytes, raw AI prompts, secrets, or customer-owned identifiers.
- Keep action labels verb-first and specific: `Continue`, `Abort`, `Skip`,
  `Restore checkpoint`, `Review conflict`, `Open output`, `Review patch`.
- `Continue`, `Abort`, `Skip`, `Restore`, and conflict states remain separate
  verbs with separate command routes, keyboard routes, audit events, and
  message IDs.

## Cross-surface Reuse

One canonical event can be projected through multiple surfaces: live region,
status item, toast, durable job row, activity center, support bundle, release
packet, or repro bundle. These projections share:

- `canonical_event_id`
- `target_ref`
- `dedupe_key`
- `message_id`
- `announcement_contract_id`
- `announcement_contract_revision`
- `assistive_tech_coverage_row_ref` when under test

Surfaces may choose different visual treatment, but they must not rewrite the
event meaning. A status item, toast, and support export that describe the same
failed task must resolve to the same event type and target.

## High-risk Operational Surfaces

Accessibility parity applies to risky VCS and repair surfaces with the same
seriousness as the editor:

- worktree managers
- stash shelves
- sequence editors for rebase, cherry-pick, revert, and patch series
- recovery panes and recovery banners
- conflict resolution views
- force-push and publish review sheets
- patch apply and revert review sheets

Minimum contract for these surfaces:

- full keyboard traversal for row movement, details, context menu, reorder,
  continue, abort, skip, restore, and conflict review;
- zoom resilience from 50% through 400%, with state, blockers, counts, and
  recovery controls preserved before secondary actions;
- screen-reader labels that name working context, ref or checkpoint posture,
  unresolved blockers, and available next verbs;
- live-region output only for meaning changes, not every reorder repaint or
  poll;
- durable fallback rows so a user can reopen the exact worktree, stash,
  sequence step, recovery checkpoint, or conflict after an announcement.

## Assistive-tech Parity Rows

The seed coverage rows are defined in
[`/artifacts/accessibility/assistive_tech_coverage_rows.yaml`](../../artifacts/accessibility/assistive_tech_coverage_rows.yaml).
They cover:

- NVDA on the claimed Windows desktop profile;
- JAWS as an explicit unclaimed Windows row until current evidence lands;
- VoiceOver on the claimed macOS desktop profile;
- Orca on the claimed Linux GNOME profiles.

Rows may be `passed`, `degraded`, `partial`, `blocked`, `failed`,
`pending_evidence`, or `unclaimed`. `partial` means the launch-critical journey
can complete but one required dimension is not yet fully covered; where the
older platform matrix needs a shared result-state value, `partial` maps to
`degraded` with an explicit note.

## Support and Export Fields

Accessibility packets, repro bundles, support bundles, and release evidence
must be able to record the announcement contract and assistive-technology row
under test without carrying private material. The schema therefore requires:

- `announcement_contract_id`
- `announcement_contract_revision`
- `announcement_event_schema_version`
- `schema_ref`
- `assistive_tech_coverage_row_ref`
- `platform_review_row_ref`
- `assistive_technology_row_ref`
- `task_corpus_ref`
- `checklist_refs`
- `raw_private_material_excluded: true`

Exports may include message IDs, placeholder keys, state classes, counts,
redaction class, and bounded labels. They must not include raw source text, raw
paths, raw URLs, raw terminal output, raw prompts, credential material, or
unredacted user identifiers.

## Fixture Expectations

Announcement fixtures must exercise both delivered and suppressed output:

- task burst coalescing suppresses intermediate task updates and emits one
  terminal result;
- trust-mode changes announce the capability change and safe next action;
- breakpoint toggles produce keyboard-confirming output without conflicting
  gutter and list phrases;
- shared-control handoff announces authority changes once and never infers
  control from presence alone;
- AI patch readiness and apply/revert success reuse the same schema as shell,
  task, and VCS events.

The fixtures are seed examples, not screen-reader automation. Platform adapter
implementation and full AT automation remain outside this contract.
