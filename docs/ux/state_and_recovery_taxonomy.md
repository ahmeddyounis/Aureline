# Empty-, loading-, degraded-, and lifecycle-state copy taxonomy, failure-tier placement, and recovery-surface mapping

This document is the **shell-wide contract** for how any protected
Aureline surface names its state and places recovery affordances
when that state is empty, loading, degraded, restricted, paused,
failing, or post-failure. It exists so every panel, editor, view,
list, prompt, durable job row, attention item, and status surface
uses **one copy taxonomy, one failure-tier placement rule, and one
recovery-surface mapping** — so designers, docs writers, and
implementation owners do not mint surface-local state words or
invent surface-local recovery routes as M1 surfaces proliferate.

The taxonomy is normative at the state-placement and copy-axis
level. It does **not** decide the final microcopy — the
[UX Design System Style Guide](../../.t2/docs/Aureline_UX_Design_System_Style_Guide.md)
and the
[shell-interaction-safety contract](./shell_interaction_safety_contract.md)
own the rendering words. Where this document disagrees with the
PRD, TAD, TDD, UI/UX spec, or milestones document, those sources
win and this document plus its companion artifacts MUST update in
the same change.

The companion artifacts are:

- [`/artifacts/ux/failure_tier_matrix.yaml`](../../artifacts/ux/failure_tier_matrix.yaml)
  — machine-readable failure-tier matrix mapping inline issue,
  contextual degraded state, workflow block, session recovery, and
  escalation-surface rows to surfaces, copy axes, and recovery-
  ladder rungs.
- [`/fixtures/ux/state_copy_examples/`](../../fixtures/ux/state_copy_examples/)
  — worked examples covering empty, loading, degraded, lifecycle,
  and failure-tier rows for workspace, extension, remote session,
  AI action, and update/rollback surfaces.

This taxonomy rides alongside — it does **not** re-mint — the
vocabularies already frozen in:

- [`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md)
  and
  [`/artifacts/ux/startup_state_copy_review.yaml`](../../artifacts/ux/startup_state_copy_review.yaml)
  — startup-state token set (`startup_state:first_run`,
  `startup_state:warming_startup`, `startup_state:partial_startup`,
  `startup_state:offline_startup`,
  `startup_state:unsupported_startup`, and the empty-state /
  placeholder-transition token). Rows in this taxonomy that
  intersect startup state cite the startup-state token by ref.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — delivery-surface, attention-class, badge, interruptibility-
  tier, quiet-hours, suppression-reason, and reopen-semantics
  vocabulary. Recovery surfaces in this taxonomy pick from the
  already-frozen delivery-surface and interruptibility-tier sets;
  they do not mint new surface kinds.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — authority, consequence, revert, representation, focus-return,
  and responsive-fallback vocabularies. Degraded-state copy names
  its consequence posture by ref to the interaction-safety
  vocabulary when an action is still available on the degraded
  row.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  and
  [`/artifacts/governance/truth_class_matrix.yaml`](../../artifacts/governance/truth_class_matrix.yaml)
  — the ten-token cross-surface degraded-state vocabulary
  (`Warming`, `Cached`, `Partial`, `Stale`, `Offline`,
  `PolicyBlocked`, `Limited`, `Unsupported`, `Experimental`,
  `RetestPending`) and the truth-class matrix. This taxonomy
  does not re-mint those tokens; it maps them onto **surface
  placement** and **copy axes**.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  — recovery-ladder rung ids (`rung.safe_mode`,
  `rung.extension_quarantine`, `rung.open_without_restore`,
  `rung.cache_index_repair`, `rung.restricted_mode_fallback`,
  `rung.none_required`). Recovery-surface rows cite rungs by id
  rather than restating troubleshooting prose.
- [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  — inline / contextual overlay / panel / window-attached sheet /
  dialog modal / full-surface takeover escalation ladder. When a
  state row promotes to a workflow block or session recovery, the
  promotion path is the navigation/escalation ladder's monotonic
  climb — no shortcut to a modal is minted here.
- [`/docs/accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  — keyboard completeness, focus order/return, screen-reader,
  reduced motion, IME/bidi input, and accessibility-tree capture.
  Every state row in this taxonomy reserves an accessibility hook
  so state copy, recovery affordance, and last-failure reason are
  keyboard-reachable and announced.

## Who reads this taxonomy

- **Product writers** drafting empty-state, loading, degraded,
  restricted, paused, failure, and post-failure copy for any
  protected shell surface. Writers quote the row id for axis
  alignment and let the UX Style Guide decide the final words.
- **Shell / panel / editor / view / list / inspector authors**
  placing state rows, loading chrome, inline issues, contextual
  degraded banners, workflow blocks, session-recovery sheets,
  escalation surfaces, and last-failure affordances.
- **Docs, support, release-evidence, and diagnostics authors**
  quoting state-row ids when a help topic, support packet, or
  release note names a capability posture so the three surfaces
  resolve to the same taxonomy.
- **Accessibility and parity reviewers** reading each axis
  mechanically so state-copy, recovery-route, and last-failure
  visibility stay keyboard-reachable, announced, and exportable.

## 1. Scope

- One **empty-state contract** for any surface whose current
  content set is zero — by design, by filter, by permission, by
  readiness, by warm-up, by offline posture, or by failure. Every
  empty row names its area purpose, its emptiness cause, and at
  least one next-safe action.
- One **loading-state contract** for skeleton rows, progressive
  partial results, top-of-pane progress indicators, and inline
  placeholder shapes. Anti-patterns (whole-shell spinners,
  indeterminate spinners with no cancel, content-reflow during
  load, hiding useful-now chrome behind "Loading…") are frozen as
  non-conforming.
- One **failure-tier matrix** mapping every failure-bearing row
  onto exactly one of five placement tiers: inline issue,
  contextual degraded, workflow block, session recovery,
  escalation surface. Promotion between tiers is monotonic and
  trigger-bearing; silent escalation is non-conforming.
- One **lifecycle-state mapping** for workspace, extension, remote
  session, AI action, and update/rollback rows — each with a
  minimum recovery-action set, a last-failure reason pointer, and
  a promotion path into the failure-tier matrix.
- Controlled-label rules for the strings `Partially ready`,
  `Degraded`, and `Read-only degraded`, plus the keyboard- and
  support-export-reachable rule for last-failure reasons.
- A compatibility posture with the later event-lineage, timeline,
  and notification-routing contracts so escalation language does
  not fork per surface.

## 2. Out of scope

- Final microcopy and localization. The taxonomy pins axes; the
  UX Style Guide and localization work own the rendered words.
- New delivery surfaces. Every recovery-surface row resolves to a
  delivery-surface class already frozen in the attention/activity
  taxonomy. A row that needs a new delivery surface opens a new
  decision row rather than landing here.
- New failure tokens, truth classes, or degraded-state tokens.
  The matrix picks from the already-frozen vocabularies (see §3).
- Automated repair flows. Recovery-ladder rungs are referenced by
  id; their behaviour lives in the recovery-ladder packet.

## 3. Frozen vocabulary (re-exported)

Every row in §5–§9 resolves to values from the following
already-frozen vocabularies. Minting a state, tier, or recovery
route outside these sets is non-conforming and opens a new
decision row.

- **Truth classes and degraded-state tokens** — the ten-token
  cross-surface set in
  [`truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  (`Warming`, `Cached`, `Partial`, `Stale`, `Offline`,
  `PolicyBlocked`, `Limited`, `Unsupported`, `Experimental`,
  `RetestPending`) and the truth classes
  (`user_authored_durable_truth`, `workspace_vcs_truth`,
  `runtime_observed_truth`, `derived_indexed_truth`,
  `session_collaboration_truth`, `ai_inferred_truth`).
- **Delivery-surface classes** — from the attention/activity
  taxonomy: `toast`, `contextual_banner`, `status_item`,
  `durable_job_row`, `attention_item`,
  `activity_center_digest_card`, `digest_group_row`,
  `os_notification`, `os_badge_app_icon`,
  `lock_screen_summary`, `companion_push`, `not_delivered_held`.
- **Interruptibility tiers** — `tier_ambient`, `tier_transient`,
  `tier_durable`, `tier_actionable`, `tier_blocking_trust`,
  `tier_critical_safety`.
- **Startup-state tokens** — `startup_state:first_run`,
  `…:reopen_with_pending_restore`, `…:restore_failed`,
  `…:restore_skipped`, `…:open_without_restore`,
  `…:warming_startup`, `…:partial_startup`,
  `…:offline_startup`, `…:unsupported_startup`,
  `…:empty_state_or_placeholder_transition`.
- **Recovery-ladder rung ids** — `rung.safe_mode`,
  `rung.extension_quarantine`, `rung.open_without_restore`,
  `rung.cache_index_repair`, `rung.restricted_mode_fallback`,
  `rung.none_required`.
- **User-impact label classes** — from the entry/restore audit:
  `no_durable_work_yet`, `pending_user_review`,
  `work_continues_with_narrowed_capability`,
  `prior_work_preserved_but_not_rerun`,
  `prior_work_not_recoverable_by_automation`,
  `no_recovery_on_irreversible_path`.
- **Escalation-ladder positions** — from the navigation /
  escalation contract: `inline`, `contextual_overlay`,
  `panel`, `window_attached_sheet`, `dialog_modal`,
  `full_surface_takeover`.
- **Next-step decision hooks** — from the entry / restore object
  model §1.7.

## 4. Truthfulness posture (normative)

Every state row, every recovery affordance, and every last-failure
reason obeys the following posture. A surface whose rendered state
violates any of these rules is non-conforming and denies with the
stated reason.

1. **No vague placeholders on protected paths.** Generic copy
   "Working…", "Loading…", "Something went wrong", "Error",
   "Failed", "Try again" is forbidden on protected paths when a
   more precise state token is known. A surface that cannot
   resolve its state denies with `state_row_unresolved_axis` and
   routes to the owning repair hook rather than fabricating a
   generic message. (Composes with the forbidden-generic-copy rule
   in
   [`truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
   §Forbidden generic copy.)
2. **Name what still works, what is reduced, what is the safest
   next action.** Every degraded, restricted, paused, or failed
   row names (a) the **preserved capability** (what the user can
   still do), (b) the **narrowed or blocked capability** (what is
   reduced and why), and (c) at least one **next-safe action
   hook** from the frozen set. A row that omits any of the three
   denies with `state_row_axis_missing`.
3. **Keyboard-reachable and announced.** Every state row's
   placement, its recovery affordance, and the last-failure
   reason MUST be keyboard-reachable through the focus order that
   the accessibility packet covers, and its announcement label
   MUST be addressable by assistive technology. A row that
   renders recovery only on hover or only via a drag affordance
   denies with `state_row_recovery_not_keyboard_reachable`.
4. **Last-failure reason persists and escapes.** Every failing or
   post-failure row MUST expose its last-failure reason through
   (a) a keyboard-reachable inspector / reveal affordance on the
   row, (b) a support-export field that preserves the failure
   class and code by ref, and (c) an export-safe description that
   honours the suspicious-content and representation vocabularies
   (no raw secret material, no raw external content). A row that
   discards the last-failure reason on dismissal denies with
   `last_failure_reason_discarded_on_dismiss`.
5. **Monotonic promotion between tiers.** Placement may only
   climb the failure-tier ladder (inline issue → contextual
   degraded → workflow block → session recovery → escalation
   surface). A tier skip requires a trigger (severity change,
   duration threshold crossed, required authority change,
   irreversible consequence). A silent skip denies with
   `failure_tier_escalated_without_trigger`.
6. **Recovery routes are cited by id.** Every recovery affordance
   cites a recovery-ladder rung id (or `rung.none_required`) and a
   support-packet family by ref; free-text troubleshooting is
   non-conforming (`recovery_route_by_freetext_only`).
7. **Controlled labels are typed.** The labels `Partially ready`,
   `Degraded`, and `Read-only degraded` are reserved for the rows
   in §8. A surface that renders them outside those rows denies
   with `controlled_label_misapplied`.
8. **Escalation language is stable.** State copy, lifecycle row
   label, last-failure reason, and recovery verb MUST resolve to
   ids that the later event-lineage, timeline, and notification-
   routing contracts can cite without renaming. Surface-local
   string copies that lose the id on export deny with
   `escalation_language_forked_per_surface`.

## 5. Empty-state contract

An empty state is any protected surface whose current content set
is zero. Every empty state names three axes — even when the
shell renders them on one line.

### 5.1 Required axes

Every empty row pins one value from each axis. Free-form
placeholders are non-conforming.

- **`area_purpose`** — what the surface is *for* in one phrase,
  independent of content. Resolved against the shell zone map and
  the navigation/escalation contract so the area name matches the
  command palette, activity rail, and breadcrumbs.
- **`emptiness_cause`** — why the surface is empty right now.
  One of the closed set:
  - `no_work_started` (first-run, no-items-authored-yet).
  - `filter_narrowed_to_zero` (filter / scope / search narrowed
    the set to zero).
  - `permission_or_policy_blocked` (admission, trust, or admin
    policy hides the set).
  - `not_ready_warming` (dependency warming; pairs with
    `Warming` token).
  - `not_ready_partial` (partial index, partial restore, partial
    capability; pairs with `Partial`).
  - `offline_local_only_view` (owner unreachable; pairs with
    `Offline`).
  - `unsupported_on_this_surface` (client-scope or platform
    scope excludes the surface; pairs with `Unsupported`).
  - `failed_last_attempt` (a prior attempt failed and the set
    is empty because of the failure; last-failure reason is
    attached).
- **`next_best_action`** — at least one next-safe action hook
  from the frozen `next_step_decision_hook` set (e.g.,
  `review_archetype_match`, `set_up_later`, `open_minimal`,
  `locate_missing_target`, `remove_from_recents`,
  `continue_in_restricted_mode`, `reconnect_required`,
  `reauth_required`, `safe_mode`), plus a typed command id from
  the command graph (`cmd.*`) when a command exists.

### 5.2 Placement rules (normative)

1. Empty states live on the owning surface's primary content
   plane. A whole-window takeover for an empty list is non-
   conforming (`empty_state_promoted_to_full_surface_without_trigger`).
2. Empty states MUST NOT block keyboard-reachable chrome (menu,
   breadcrumb, palette, inspector). The chrome remains reachable
   and the next-best-action is focus-bearing on first render.
3. Empty states render the truth class of the surface via a typed
   badge, not via copy — the truth class of a zero-result search
   is the same class as a non-zero result; copy differentiates
   cause, not identity.
4. Empty states that intersect a startup-state token quote the
   startup-state row by ref and do not restate the cause in
   free-form copy.

### 5.3 Forbidden empty-state patterns

- "Nothing here yet" without naming area purpose or cause.
- "No results" without naming the filter or scope that excluded
  the set.
- Marketing copy ("Start your journey") on a first-run empty
  view; the `set_up_later` and `open_minimal` hooks are
  advertised verbatim instead.
- A spinner in place of an empty view when the true state is a
  filter- or permission-narrowed zero-result — the empty cause
  is named, not hidden behind loading chrome.

## 6. Loading-state contract

A loading state is any surface whose content set is being
computed, warmed, fetched, or materialised. The contract governs
how the shell renders in-flight work *while preserving useful
immediate chrome*.

### 6.1 Allowed loading patterns

- **`skeleton_row`** — a content-shaped placeholder that matches
  the eventual row's anatomy (avatar size, column widths, line
  counts). Skeletons MUST collapse to the actual row without
  reflow; a skeleton that rearranges rows on arrival denies with
  `skeleton_reflow_on_arrival`.
- **`progressive_partial_results`** — rows stream in as they
  resolve. The surface names the warming posture explicitly
  (e.g., `Partial index` chip) so partial rows do not imply full
  truth. Composes with the search-readiness vocabulary.
- **`top_of_pane_progress_indicator`** — a progress bar, chip, or
  count badge at the surface's top edge that reports warming,
  partial, or retesting posture while the user keeps working
  below it. The indicator names what is warming and the expected
  ready signal (or an unbounded marker with an explicit cancel
  route).
- **`inline_placeholder`** — a per-row or per-field placeholder
  that names the missing piece (e.g., "Semantic lookup pending")
  rather than a generic "Loading…". Pairs with the `Warming` or
  `RetestPending` tokens.
- **`not_delivered_held_placeholder`** — reserved for quiet-hours
  or admin-suppression holds. The placeholder names the hold
  reason and the release condition.

### 6.2 Required rules

1. Every loading surface keeps the useful-now chrome interactive.
   Editor, buffer, save pipeline, and recovery journal remain
   live; the inspector, palette, and activity rail remain
   keyboard-reachable. A surface that blocks the shell on its
   loading state denies with
   `whole_shell_spinner_forbidden`.
2. Every loading state names what is warming and what remains
   authoritative. A progressive partial list MUST distinguish
   partial rows from authoritative rows (e.g., chip on the
   surface; typed `result_truth_label` on the row).
3. Every long-running load (> the `tier_durable` threshold in the
   attention/activity taxonomy) MUST mirror to a durable surface
   — activity centre, durable job row, or status item — even
   when the local surface also shows progress. A toast-only
   long-running load denies with
   `toast_only_forbidden_for_durable_work`.
4. Every unbounded load MUST expose a cancel route and, where
   recovery is meaningful, a repair route (cache reset, re-index,
   restart warm-up). A loading state without cancel routing
   denies with `load_without_cancel_or_repair`.

### 6.3 Forbidden loading patterns

- **Whole-shell spinners.** The activity rail, command palette,
  menu bar, keyboard routing, or breadcrumbs going dark behind a
  global spinner while any load is in flight.
- **Indeterminate spinner in place of cause.** An indeterminate
  spinner on a path where a precise warming / partial / stale /
  retesting token is known.
- **"Working…" / "Loading…" / "Please wait" alone.** These
  strings are non-conforming on protected paths unless paired
  with a typed token and cause phrase. A surface that renders
  them bare denies with `vague_loading_copy_on_protected_path`.
- **Content-reflow during load.** Skeletons whose final layout
  differs from the arrived row, or progressive lists that
  reorder above-the-fold rows after load. Layout MUST be stable
  from first paint.
- **Hiding useful-now chrome.** A loading overlay that occludes
  the editor, palette, inspector, or breadcrumbs past the
  `tier_transient` threshold.

## 7. Failure-tier matrix (placement)

Every failure-bearing row lives on **exactly one** of five
placement tiers. The matrix is frozen; minting a new tier is
breaking and opens a new decision row. The machine-readable
matrix in
[`/artifacts/ux/failure_tier_matrix.yaml`](../../artifacts/ux/failure_tier_matrix.yaml)
is the source for tooling; this section is the reviewer-facing
narrative.

### 7.1 `tier.inline_issue`

Scope: a single row, cell, or line-range in the owning surface.
Surface: inline (within the row). Interruptibility:
`tier_ambient` or `tier_transient`. Examples: one-file save
error, one suggestion that failed to apply, one LSP diagnostic
arriving late, one search result marked `partial_index`.

- **Placement.** Adjacent to the offending row; never occupies a
  banner, panel, or sheet.
- **Required axes.** Cause token, preserved capability, one
  recovery action (`Retry this`, `Reveal last failure`,
  `Locate alternate target`), keyboard-reachable last-failure
  inspector, escalation trigger to the next tier.
- **Recovery routes.** `rung.none_required` or
  `rung.cache_index_repair` (for derived-truth rows).
- **Escalation trigger.** Repeated inline failures (three or
  more within a grouped-burst window per the activity-taxonomy
  dedupe rules), duration threshold, or a severity change on
  the underlying subsystem.

### 7.2 `tier.contextual_degraded`

Scope: the surface as a whole is degraded (not just one row).
Surface: `contextual_banner` or `status_item` on the owning
surface. Interruptibility: `tier_durable` (mirrors to the
activity centre). Examples: workspace in `Partially ready`
posture, index in `partial_index` readiness, extension in
`quarantined` posture on a running workspace, remote session
in `Offline` on a local-only-capable surface.

- **Placement.** Surface-level banner or pinned status item that
  names the posture verbatim (`Partially ready`, `Degraded`,
  `Read-only degraded`) with the truthful cause token beneath.
- **Required axes.** Cause token, preserved capability list,
  narrowed capability list, at least one next-safe action hook,
  at least one recovery-ladder rung ref, support-packet family
  ref, last-failure-reason inspector.
- **Recovery routes.** `rung.cache_index_repair`,
  `rung.extension_quarantine`, `rung.open_without_restore`,
  `rung.restricted_mode_fallback` as applicable.
- **Escalation trigger.** Consequence-bearing action blocked on
  the current posture, repeated recovery failure, required
  authority change, or irreversible-consequence path entered.

### 7.3 `tier.workflow_block`

Scope: the user's active workflow (typing, running, reviewing,
approving, publishing) cannot safely continue on the current
posture. Surface: `window_attached_sheet` or panel the user
reached through the navigation/escalation ladder; `dialog_modal`
only when the consequence is irreversible per the
navigation/escalation contract. Interruptibility:
`tier_actionable` or `tier_blocking_trust`.

- **Placement.** Window-attached sheet or full-height panel on
  the owning surface; the user's existing work remains visible
  and recoverable behind the sheet (focus-return on dismiss
  returns to the row that triggered the block).
- **Required axes.** Cause token, preserved capability, blocked
  workflow + reason, at least one next-safe action hook (per
  navigation/escalation rules), recovery-ladder rung ref,
  support-packet family ref, escalation path to session
  recovery if the block cannot be resolved in-place.
- **Recovery routes.** Any rung appropriate to the posture;
  `rung.restricted_mode_fallback` is the default when the user
  chooses to continue without restoring the blocked capability.
- **Escalation trigger.** User elects to halt the workflow, the
  underlying subsystem crashes, or the block cannot be
  resolved without a session-level repair.

### 7.4 `tier.session_recovery`

Scope: the whole session needs a repair pass before the user
returns to normal work (crash loop, corrupt restorable state,
managed-workspace refusal to start, pinned extension crash
loop). Surface: `dialog_modal` for the recovery-ladder entry
step plus a follow-up `full_surface_takeover` for the repair
pass; never a toast or a contextual banner.

- **Placement.** Dialog modal owned by the shell's top-level
  window (focus-return to the startup surface after resolution).
  The recovery-ladder rungs are the verbs; `Undo` is forbidden
  on `compensating_rollback` / `regenerate_from_canonical_source`
  classes.
- **Required axes.** Startup-state row ref (when applicable),
  cause token, preserved capability, a named rung-id,
  evidence-packet family ref, and the session-recovery outcome
  (`safe_mode_entered`, `extension_quarantined`,
  `open_without_restore_committed`, `cache_repaired`,
  `restricted_mode_entered`) by ref.
- **Recovery routes.** `rung.safe_mode`,
  `rung.extension_quarantine`, `rung.open_without_restore`,
  `rung.cache_index_repair`, `rung.restricted_mode_fallback`.
- **Escalation trigger.** Rung failure repeated across the
  configured threshold, or any required authority / policy
  change that the session cannot honour without an external
  (account, admin, platform) intervention.

### 7.5 `tier.escalation_surface`

Scope: the failure exceeds what the product can resolve
autonomously; the user needs support, admin policy, platform
help, or issue handoff. Surface: escalation sheet / Support
Centre entry / object-handoff packet surface linked from the
preceding tier; never a surprise modal that the user did not
reach through an explicit escalation affordance.

- **Placement.** Reached through the explicit
  `Reveal last failure`, `Open support center`,
  `Send object issue handoff`, or `Escalate to admin policy`
  affordances. Focus-return restores the triggering row or the
  session-recovery surface.
- **Required axes.** Cause token, last-failure reason (by ref
  and preserved on export), recovery-ladder rung history (by
  id), support-packet family and packet id,
  object-handoff-packet ref when the evidence leaves the
  product.
- **Recovery routes.** `rung.none_required` on the product
  side; the packet itself carries escalation-to-human
  expectations.
- **Escalation trigger.** No further product-side rung
  applies; the user explicitly asks to escalate; admin
  policy or platform support is required to unblock the row.

### 7.6 Promotion rules (normative)

1. A row's placement MUST climb monotonically
   (`inline_issue` → `contextual_degraded` → `workflow_block`
   → `session_recovery` → `escalation_surface`). A skip
   requires a trigger (severity change, duration threshold,
   required authority change, irreversible consequence).
2. A row MUST NOT demote silently. A placement that drops from
   `workflow_block` back to `inline_issue` requires an explicit
   resolution event (repair succeeded, restore committed,
   workflow abandoned by user). A silent demotion denies with
   `failure_tier_demoted_without_resolution_event`.
3. The navigation/escalation contract's inline-first rule still
   applies: start at the lowest tier that truthfully represents
   the scope. Starting at `dialog_modal` for a single-row
   failure denies with
   `failure_tier_started_above_scope`.
4. Every promotion preserves the previous tier's last-failure
   reason by ref; the user can always walk back down the
   ladder to see the originating row.

## 8. Lifecycle-state mapping (required per subsystem)

Each subsystem below pins a closed lifecycle-state set, a
minimum recovery-action set, and a last-failure-reason
visibility rule. Rows resolve to the failure-tier matrix on
promotion. Controlled labels `Partially ready`, `Degraded`, and
`Read-only degraded` are defined here and are reserved for
these rows (§8.6).

### 8.1 Workspace

- **States.** `workspace.starting`, `workspace.ready`,
  `workspace.partially_ready`, `workspace.degraded`,
  `workspace.read_only_degraded`, `workspace.recovering`,
  `workspace.quarantined`, `workspace.stopped`.
- **Minimum recovery actions.** `Open without restore`,
  `Enter safe mode`, `Repair cache / index`,
  `Continue in restricted mode`, `Locate missing target`.
- **Last-failure reason.** Keyboard-reachable on a status-item
  reveal; preserved on support export
  (`recovery_ladder_packet`, `managed_workspace_evidence`).
- **Failure-tier mapping.** `partially_ready` and `degraded`
  live on `tier.contextual_degraded`; `recovering` and
  `quarantined` promote to `tier.session_recovery`;
  `stopped` with no recoverable path promotes to
  `tier.escalation_surface`.

### 8.2 Extension

- **States.** `extension.loading`,
  `extension.activated`, `extension.partially_activated`,
  `extension.degraded`, `extension.read_only_degraded`,
  `extension.quarantined`, `extension.unactivated_by_policy`,
  `extension.uninstalled`.
- **Minimum recovery actions.** `Reload extension`,
  `Quarantine extension`, `Reveal last failure`,
  `Open extension details`, `Continue without extension`.
- **Last-failure reason.** Keyboard-reachable on the
  extension's row in the activity centre; preserved on
  support export (`recovery_ladder_packet`,
  `object_issue_handoff`).
- **Failure-tier mapping.** `partially_activated` and
  `degraded` on `tier.contextual_degraded`; `quarantined`
  and repeated crash-loop promote to
  `tier.session_recovery` via `rung.extension_quarantine`;
  policy-blocked extensions live on
  `tier.contextual_degraded` with a policy-source label.

### 8.3 Remote session

- **States.** `remote.reachable`, `remote.reconnecting`,
  `remote.reauth_pending`, `remote.read_only_degraded`,
  `remote.offline`, `remote.policy_blocked`.
- **Minimum recovery actions.** `Reconnect`, `Reauthenticate`,
  `Continue locally`, `Open without restore`,
  `Reveal last failure`.
- **Last-failure reason.** Keyboard-reachable on the remote-
  session status item; preserved on support export
  (`auth_evidence_packet`, `recovery_ladder_packet`).
- **Failure-tier mapping.** `reconnecting` and `reauth_pending`
  on `tier.contextual_degraded`; `offline` on a remote-
  required workflow promotes to `tier.workflow_block`;
  `policy_blocked` promotes to `tier.session_recovery` via
  `rung.restricted_mode_fallback`.

### 8.4 AI action

- **States.** `ai.drafting`, `ai.awaiting_review`,
  `ai.partial_results`, `ai.retest_pending`, `ai.degraded`,
  `ai.rolled_back`, `ai.blocked_by_policy`.
- **Minimum recovery actions.** `Reveal citations`,
  `Retry with narrower scope`, `Discard draft`,
  `Roll back apply`, `Open support export`.
- **Last-failure reason.** Keyboard-reachable on the AI
  apply / review surface; preserved on support export
  (`recovery_ladder_packet`, `object_issue_handoff`). The
  last-failure reason MUST name the inference vs. cited
  evidence boundary — an AI row that silently promotes
  inferred content to durable truth denies with
  `ai_inferred_truth_promoted_without_citation`.
- **Failure-tier mapping.** `partial_results` and
  `retest_pending` on `tier.inline_issue` or
  `tier.contextual_degraded` depending on scope;
  `rolled_back` on `tier.contextual_degraded` with the
  rollback evidence preserved; `blocked_by_policy` on
  `tier.contextual_degraded` with the policy source label.

### 8.5 Update / rollback

- **States.** `update.checking`, `update.available`,
  `update.downloading`, `update.ready_to_restart`,
  `update.applying`, `update.rolled_back`,
  `update.partial_apply`, `update.failed`.
- **Minimum recovery actions.** `Retry`, `Roll back`,
  `Enter safe mode`, `Reveal release notes`,
  `Reveal last failure`.
- **Last-failure reason.** Keyboard-reachable on the
  update status item; preserved on support export
  (`recovery_ladder_packet`, `object_issue_handoff`).
- **Failure-tier mapping.** `downloading` and `applying`
  live on `tier.inline_issue` with a durable mirror
  (long-running work). `partial_apply` and
  `rolled_back` on `tier.contextual_degraded`.
  `failed` on a user-initiated restart-to-apply promotes
  to `tier.session_recovery` via `rung.safe_mode`.

### 8.6 Controlled labels

The strings below are reserved for rows in §8.1–§8.5 and MUST
resolve to the lifecycle-state tokens above. A surface that
renders them for another class denies with
`controlled_label_misapplied`.

- **`Partially ready`** — subsystem is up but a named
  capability is narrowed. Pairs with `Partial` or `Warming`
  tokens. Row names preserved / narrowed capability sets.
- **`Degraded`** — subsystem is operating in a reduced
  posture (cache fallback, secondary provider, pinned
  version). Pairs with the appropriate
  `degraded_state_cause`. Row names at least one repair
  route.
- **`Read-only degraded`** — subsystem can read but cannot
  commit writes that would be authoritative. Pairs with
  the `Read-only` posture and explicitly names what writes
  are blocked and why. Mutations MUST be refused before they
  commit; silent no-op writes deny with
  `read_only_degraded_silent_noop`.

### 8.7 Last-failure reason rule (normative)

Every lifecycle row in §8 MUST expose its last-failure reason
through:

1. A **keyboard-reachable reveal affordance** on the row or in
   the row's status-item drawer (accessibility tree names the
   reveal verb verbatim).
2. A **support-export field** that preserves the failure class
   and code by ref (per the error-class routing contract),
   the timestamp (monotonic + wall-clock pair), and the
   recovery-ladder rung history.
3. An **export-safe description** that honours the suspicious-
   content and representation vocabularies (no raw secret
   material, no raw external content; redaction pass runs
   before bytes leave the product).

A row that discards its last-failure reason when the row
resolves, when the user dismisses the banner, or when the
session restarts denies with
`last_failure_reason_discarded_on_dismiss`.

## 9. Recovery-surface mapping

Every failure-tier row names a recovery-surface mapping so
docs, support, diagnostics, and release evidence resolve to
the same routes.

| Failure tier | Allowed delivery surfaces | Minimum recovery affordance | Interruptibility tier |
| --- | --- | --- | --- |
| `tier.inline_issue` | Inline row chrome, `status_item` | `Retry this`, `Reveal last failure` | `tier_ambient` / `tier_transient` |
| `tier.contextual_degraded` | `contextual_banner`, `status_item`, `activity_center_digest_card`, `durable_job_row` mirror | At least one rung id; `Reveal last failure`; controlled label | `tier_durable` |
| `tier.workflow_block` | `window_attached_sheet`, panel | At least one rung id; safe abandon; focus-return on dismiss | `tier_actionable` / `tier_blocking_trust` |
| `tier.session_recovery` | `dialog_modal` entry + `full_surface_takeover` repair surface | Rung id; outcome token; evidence-packet family | `tier_blocking_trust` |
| `tier.escalation_surface` | Support Centre, object-handoff packet surface, OS notification where critical | Packet ref; escalation outcome token | `tier_critical_safety` |

- OS notifications are allowed only at `tier.escalation_surface`
  and only when the critical-safety tier applies (trust-, policy-,
  recovery-critical items). A toast on a workflow-block tier
  denies with `toast_only_forbidden_for_durable_work`.
- `lock_screen_summary` and `companion_push` at
  `tier.escalation_surface` respect the lock-screen-safe and
  privacy-payload vocabularies in the attention/activity
  taxonomy. Raw content, secrets, or PII on the lock-screen
  payload denies with
  `lock_screen_payload_violates_privacy_class`.
- Every recovery surface reserves a typed hook for the future
  event-lineage, timeline, and notification-routing contracts
  so the same lineage carries the row's state history without
  renaming fields per surface.

## 10. Compatibility with event-lineage, timeline, and notification-routing

This taxonomy reserves the following integration points so the
later lineage / timeline / notification contracts bind without
renaming rows:

1. Every state row carries a stable **row id** (`state_row:*`)
   and a **parent lineage id** when it is a follow-up of a
   prior row (dedupe, repeat failure, post-recovery re-flare).
   The lineage id aligns with the activity-envelope
   `grouped_burst_id` so the event-lineage contract can fold
   the history without minting a parallel identity.
2. Every promotion between failure tiers emits a typed
   **promotion event** carrying the from-tier, to-tier,
   trigger class, and evidence ref. The timeline contract
   reads the promotion stream directly.
3. Every recovery outcome emits a typed **resolution event**
   carrying the rung id, outcome token, and preserved /
   narrowed capability deltas. Notification-routing reads the
   resolution stream for reopen semantics.
4. Controlled labels (`Partially ready`, `Degraded`,
   `Read-only degraded`) are stable tokens the later contracts
   can cite; surface-local string copies that lose the token
   on export deny with `controlled_label_lost_on_export`.

## 11. Acceptance mapping

- **No vague copy.** Every row in §5–§8 names axes that disprove
  vague "Working…" / "Something went wrong" copy. The
  companion matrix encodes this as a per-row invariant.
- **Recovery surfaces name preserved / narrowed / safest-next
  action.** Every recovery-surface row in §9 names its minimum
  recovery affordance; rows missing any leg deny with
  `recovery_surface_axis_missing`.
- **Keyboard-reachable last-failure.** §4.4 + §8.7 are the
  normative rules; the fixture corpus exercises them on
  workspace, extension, remote, AI, and update rows.
- **Reusable by onboarding, support, diagnostics, and release
  evidence.** The row ids, lifecycle tokens, failure-tier ids,
  and recovery-rung refs are the stable keys those downstream
  surfaces import. A surface that restates the basics in
  free-form copy denies with `taxonomy_restated_per_surface`.

## 12. Source anchors

- `.t2/docs/Aureline_PRD.md` — recovery-journal and autosave
  rules; exact session restore degradation posture; recovery
  ladder (safe mode → extension quarantine → cache reset →
  restricted mode); diagnostics and support export.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — Start Center
  primary actions; restore-class taxonomy; restore-fidelity
  rules; Support Centre; Doctor surface; durable job-row and
  attention-item contracts.
- `.t2/docs/Aureline_Technical_Design_Document.md` — appearance-
  session contract; durable job-row and banner truth; support
  packet summary fields.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` — critical
  component-state matrix; result-row / trust-prompt / job-row
  guidance; degraded-state visibility.

## 13. Linked artifacts

- Failure-tier matrix:
  [`/artifacts/ux/failure_tier_matrix.yaml`](../../artifacts/ux/failure_tier_matrix.yaml).
- State copy examples:
  [`/fixtures/ux/state_copy_examples/`](../../fixtures/ux/state_copy_examples/).
- Entry / restore placeholder truth audit:
  [`./entry_restore_truth_audit.md`](./entry_restore_truth_audit.md).
- Attention / activity taxonomy:
  [`./attention_activity_taxonomy.md`](./attention_activity_taxonomy.md).
- Shell-level interaction-safety contract:
  [`./shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md).
- Navigation and escalation contract:
  [`./navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md).
- Recovery-ladder packet:
  [`../support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
- Truth-and-degraded-state vocabulary:
  [`../governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md).
- Truth-class matrix:
  [`../../artifacts/governance/truth_class_matrix.yaml`](../../artifacts/governance/truth_class_matrix.yaml).
- Accessibility / IME packet template:
  [`../accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md).

## 14. Changing this taxonomy

- Adding a lifecycle row, an empty-state cause, a loading
  pattern, or a recovery-surface row is **additive-minor** and
  lands here plus the matrix plus a matching fixture in the
  same change. Axes MUST resolve to already-frozen vocabulary.
- Repurposing a failure-tier id, a controlled label, or a
  lifecycle-state token is **breaking** and opens a new
  decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- Copy and microcopy updates that do not change axes live in
  the UX Style Guide and the shell-interaction-safety
  contract; this taxonomy pins structure, not rendering.
