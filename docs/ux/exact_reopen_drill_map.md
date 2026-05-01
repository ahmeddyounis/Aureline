# Exact-reopen drill map: from transient delivery to one canonical durable target

This document is the **drill map** that proves every transient or
OS-level delivery for the durable-attention corpus reopens to one
canonical durable identity rather than a generic home screen,
dashboard, or per-surface placeholder. It is read alongside the
durable-attention corpus and the job-row retention contract; this
document does **not** re-mint delivery-surface, attention,
quiet-hours, suppression, dedupe, reopen-target, dismissal, or
durable-job-lifecycle vocabulary.

The drill map is normative. Where this document disagrees with the
UI / UX Spec or with the upstream attention-routing, notification-
delivery, durable-work, durable-job-envelope, or OS-notification /
quiet-hours contracts, the source spec wins and this document plus
its companion artifacts must change in the same patch. Where a
downstream surface ships a private reopen verb, a private
fallback target, or a per-device durable identity for one of the
corpus rows, this document wins and the surface is non-conforming.

## Companion artifacts

- [`/artifacts/ux/durable_attention_corpus.yaml`](../../artifacts/ux/durable_attention_corpus.yaml)
  binds every covered scenario to a closed source-subsystem,
  target-kind, state-class, activity-partition, canonical event id,
  canonical object target, and retention packet.
- [`/schemas/ux/job_row_retention.schema.json`](../../schemas/ux/job_row_retention.schema.json)
  freezes the per-partition retention rule, the per-row retention
  packet, and the audit-event record consumed by tooling, support
  export, and review.
- [`/fixtures/ux/durable_attention_cases/`](../../fixtures/ux/durable_attention_cases/)
  contains worked YAML fixtures one per corpus row binding the
  job-row record, the retention packet, and the reopen-drill block.

## Upstream contracts

This drill map composes with existing owners and does not replace
them:

- [`docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns delivery-surface class, attention class, interruptibility
  tier, quiet-hours mode, suppression reason, privacy payload
  class, dedupe-key scheme, dismissal verb, reopen-target kind,
  durable-job lifecycle state, badge class, and source-subsystem
  vocabulary.
- [`docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md)
  owns canonical event lineage, routing, dedupe collapse,
  redaction, durable linkbacks, and the action taxonomy.
- [`docs/ux/durable_work_contract.md`](./durable_work_contract.md)
  owns the durable-work row anatomy, state classes, progress-form
  grammar, activity-center partition rules, and the linkback /
  audit / support-export invariants.
- [`docs/ux/durable_job_envelope_contract.md`](./durable_job_envelope_contract.md)
  owns the durable-job envelope every desktop or companion
  affordance mirrors.
- [`docs/ux/os_notification_and_quiet_hours_contract.md`](./os_notification_and_quiet_hours_contract.md)
  owns suppression-record, privacy-safe payload, and desktop-
  summary-affordance vocabularies.
- [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  owns per-`quiet_hours_mode` suppression rules.

## Who reads this document

- **Shell, activity-center, notification, OS-shim, and companion
  authors** verifying that an activation of any transient delivery
  resolves to the same durable identity that the canonical
  attention item or durable job row already names.
- **Support, evidence, and parity-audit tooling** mechanically
  joining `corpus_row_id`, `canonical_event_id`,
  `canonical_object_target_ref`, `job_row_id_ref`,
  `retention_packet_ref`, and `canonical_reopen_target_ref` across
  the corpus, the retention schema, and the worked fixtures.
- **Reviewers** confirming that completed, held, or suppressed
  rows remain reviewable after the user looks away — the central
  acceptance fact for the durable-attention work.

## Frozen vocabularies (re-exported)

Adding a value below is additive-minor and bumps the corresponding
upstream schema version; repurposing a value is breaking and
requires a new decision row.

### Reopen target kinds

Re-exported from
[`/schemas/ux/activity_event_envelope.schema.json`](../../schemas/ux/activity_event_envelope.schema.json):

- `canonical_object_target_exact`
- `review_sheet`
- `diff_view`
- `evidence_packet_row`
- `activity_center_item`
- `history_lane_row`
- `attention_item_exact`
- `durable_job_row_exact`
- `placeholder_announced`
- `reopen_denied_requires_revalidation`

### Source delivery surfaces (drill origins)

Re-exported from
[`/schemas/ux/job_row.schema.json`](../../schemas/ux/job_row.schema.json)
(`linkback_source_surface`):

- `toast`
- `badge`
- `os_notification`
- `lock_screen_summary`
- `companion_push`
- `status_item`
- `dialog_sheet`
- `review_sheet`
- `task_surface`
- `command_palette`
- `automation_cli`
- `activity_center`
- `history_lane`
- `support_export`

### Activity partitions

Re-exported from
[`/schemas/ux/job_row.schema.json`](../../schemas/ux/job_row.schema.json):

- `current_work`
- `needs_attention`
- `completed`
- `suppressed_held`

## Drill rules (frozen)

1. **One canonical durable identity per row.** Every drill row
   names exactly one `canonical_reopen_target_ref` and one
   `canonical_reopen_target_kind`. Every transient or OS-level
   surface either resolves to that target or denies with
   `reopen_denied_requires_revalidation`. Reopen to a generic home
   screen is non-conforming
   (`denial_reason = reopen_to_generic_home_forbidden`).
2. **Transient dismissal preserves durable state.** Dismissing a
   toast, OS notification, badge, lock-screen summary, digest
   group row, or companion push MUST NOT clear the durable row,
   the canonical event lineage, the audit trail, the held /
   suppressed count, or the partition the row currently lives in.
   Only the named `release_trigger_class` satisfies retention.
3. **Cross-client deliveries share one lineage.** Companion push,
   managed admin surface, and remote-agent deliveries reopen to
   the same attention item, durable job row, or canonical object
   the desktop product names, joined by
   `cross_client_canonical_event_id`. Per-device durable identity
   is forbidden (`denial_reason = companion_cross_client_divergence`).
4. **Lock-screen and companion shortcuts cannot bypass review.**
   High-risk or mutating actions (AI apply approval, trust
   escalation, secret broker decisions, destructive recovery)
   MUST route through the in-product review sheet or modal even
   when activated from a notification shortcut
   (`denial_reason = high_risk_action_bypassed_review`).
5. **Revalidation rather than silent replay.** Wake-from-sleep,
   display reconnect, policy-epoch rotation, and provider-grant
   narrowing trigger `reopen_denied_requires_revalidation` rather
   than silently replaying a mutating action.
6. **Held and suppressed rows are reopen targets, not voids.**
   Held quiet-hours digests reopen on the durable row; admin-
   suppressed audit rows reopen on the durable row when the user
   has policy permission and otherwise to the
   `placeholder_announced` reopen target with a reviewable
   announcement.

## Activity-center partition retention summary

This summary mirrors the partition rules in
[`/schemas/ux/job_row_retention.schema.json`](../../schemas/ux/job_row_retention.schema.json).
The retention packet on each row narrows but never widens these
defaults.

| Activity partition | Included state classes | Retention window | Default release trigger | Stays reviewable until |
| --- | --- | --- | --- | --- |
| `current_work` | `running`, `queued_waiting` | `live_until_terminal` | `terminal_state_reached` | The work reaches a terminal state and the row moves to `completed` or `needs_attention`. |
| `needs_attention` | `needs_approval`, `attention_required`, `partially_completed` | `retained_with_unread_until_resolved` | `user_resolve` | The user resolves, dismisses to history, or the policy condition behind the row clears. |
| `completed` | `completed` | `retained_for_completion_audit_window` | `user_dismiss_to_history` | A bounded audit window passes or the user explicitly dismisses to history; export and evidence remain available afterwards. |
| `suppressed_held` | `quiet_hours_held`, `policy_suppressed` | `retained_until_release_digest` (held) / `retained_until_policy_revoked_or_explicit_clear` (policy) | `mode_exit_grouped_digest_release` / `policy_revoked` | The mode exits and held items release as one grouped digest, or the policy is revoked / the user explicitly clears the suppressed audit row. |

Transient chrome (`toast`, `os_notification`, `os_badge_app_icon`,
`lock_screen_summary`, `companion_push`, `digest_group_row`,
`status_item`, `contextual_banner`) is dismissible without
clearing the durable row. Dismissing the chrome moves nothing on
its own.

## Reopen drill matrix (all corpus rows)

The matrix below is dense by design: a tooling pass joins the
corpus row id and source surface to one cell, and the cell names
the reopen target kind, the canonical reopen target ref, and the
revalidation posture. Cells reading "n/a" mean the surface does
not deliver this row class by contract.

Legend:

- **CK** = `canonical_object_target_exact`
- **AI** = `attention_item_exact`
- **DJ** = `durable_job_row_exact`
- **AC** = `activity_center_item`
- **HL** = `history_lane_row`
- **RV** = `reopen_denied_requires_revalidation`
- **PA** = `placeholder_announced`
- **RS** = `review_sheet`
- **DV** = `diff_view`
- **EV** = `evidence_packet_row`

| corpus_row_id | toast | badge | os_notification | lock_screen_summary | companion_push | status_item | activity_center | history_lane |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `dur:row:build_run` | DJ | AC | DJ | DJ (redacted) | n/a | DJ | DJ | n/a |
| `dur:row:test_run` | AI | AC | AI | AI (redacted) | AI (cross-client) | DJ | AI | HL |
| `dur:row:debug_session` | DJ | AC | RV (post wake-from-sleep) / DJ | RV / PA (locked) | n/a | DJ | DJ | n/a |
| `dur:row:update_or_download` | DJ | AC | DJ | DJ (redacted) | n/a | DJ | DJ | HL (after completion) |
| `dur:row:index_rebuild` | DJ | AC | DJ | DJ (redacted) | n/a | DJ | DJ | n/a |
| `dur:row:transport_reconnect` | AI | AC | AI | AI (redacted) | AI (cross-client) | AI | AI | HL |
| `dur:row:extension_crash` | AI | AC | AI | AI (redacted) | AI (cross-client) | AI | AI | HL |
| `dur:row:ai_approval_pending` | AI | AC | AI → RS (review) | AI (privacy-safe) / RV when locked | AI (cross-client) → RS | AI | AI | HL |
| `dur:row:quiet_hours_held` | n/a (held) | AC (held badge) | n/a (held) | n/a (held) | n/a (held) | DJ (count visible) | DJ | HL (after release) |
| `dur:row:suppressed_security_notice` | n/a (suppressed) | AC (held badge) | PA (admin-narrowed) / DJ (when permitted) | PA | PA / DJ (when permitted) | DJ | DJ | EV |
| `dur:row:companion_delivered_alert` | AI | AC (mentions) | AI (redacted) | AI (privacy-safe) | AI (cross-client) | AI | AI | HL |

Cells that read `RV` MUST surface a typed
`revalidation_required_reason_label`; cells that read `PA` MUST
surface a typed `placeholder_announcement_label`. Both classes are
auditable rather than silent.

## Per-row drills

Each row below records:

- the source surfaces that deliver the event;
- per-source reopen target kinds and the canonical convergence;
- transient-chrome dismissal posture (which chrome MAY be
  dismissed without clearing the durable row);
- the named `release_trigger_class` that releases retention;
- audit obligations.

### Row: build run {#row-build-run}

- **Corpus row**: `dur:row:build_run`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:build:debug-profile`,
  `durable_job_id_ref = ux:durable-job:build:debug-profile`,
  `job_row_id_ref = ux:job-row:build:debug-profile:running`,
  `canonical_reopen_target_kind = durable_job_row_exact`,
  `canonical_reopen_target_ref = ux:reopen:build:debug-profile:durable`.
- **Source surfaces and reopen kinds**:
  - `durable_job_row` → `durable_job_row_exact` (canonical).
  - `status_item` → `durable_job_row_exact`.
  - `os_badge_app_icon` → `activity_center_item`.
  - `toast` → `durable_job_row_exact`.
- **Transient chrome dismissible without clearing the row**:
  `toast`, `os_badge_app_icon`, `os_notification` (when present),
  `lock_screen_summary` (when present).
- **Release trigger**: `terminal_state_reached`. The row moves
  to `completed` (success) or `needs_attention`
  (`attention_required` / `partially_completed` on failure).
- **Audit**: `full_audit_trail_required`; the row's lineage
  preserves `canonical_event_id` from queue through completion.

### Row: test run {#row-test-run}

- **Corpus row**: `dur:row:test_run`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:test-run:integration-suite`,
  `canonical_reopen_target_kind = attention_item_exact`,
  `canonical_reopen_target_ref = ux:reopen:test:integration-suite:attention`.
- **Source surfaces and reopen kinds**:
  - `attention_item` → `attention_item_exact` (canonical).
  - `durable_job_row` → `durable_job_row_exact`.
  - `os_badge_app_icon` → `activity_center_item`.
  - `os_notification` → `attention_item_exact` (redacted summary).
  - `toast` (mirror) → `attention_item_exact`.
  - `companion_push` (cross-client) → `attention_item_exact`.
- **Transient chrome dismissible without clearing the row**:
  `toast`, `os_notification`, `os_badge_app_icon`,
  `lock_screen_summary`, `companion_push`.
- **Release trigger**: `user_resolve` (re-run, fix, or accept
  failure) or `user_dismiss_to_history`.
- **Audit**: `full_audit_trail_required`; the row stays in
  `needs_attention` until release.

### Row: debug session {#row-debug-session}

- **Corpus row**: `dur:row:debug_session`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:debug-session:cli-runner`,
  `canonical_reopen_target_kind = durable_job_row_exact`,
  `canonical_reopen_target_ref = ux:reopen:debug:cli-runner:durable`.
- **Source surfaces and reopen kinds**:
  - `durable_job_row` → `durable_job_row_exact` (canonical).
  - `status_item` → `durable_job_row_exact`.
  - `os_badge_app_icon` → `activity_center_item`.
  - `os_notification` → `reopen_denied_requires_revalidation`
    after wake-from-sleep, display reconnect, or policy-epoch
    rotation; otherwise `durable_job_row_exact`.
- **Transient chrome dismissible without clearing the row**:
  `os_badge_app_icon`, `os_notification` (when present).
- **Release trigger**: `terminal_state_reached` (session ends or
  is detached).
- **Audit**: `full_audit_trail_required`; revalidation reasons
  carry typed labels.

### Row: update or download {#row-update-or-download}

- **Corpus row**: `dur:row:update_or_download`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:update-install:platform-2026.05`,
  `canonical_reopen_target_kind = durable_job_row_exact`,
  `canonical_reopen_target_ref = ux:reopen:update:platform-2026.05:durable`.
- **Source surfaces and reopen kinds**:
  - `durable_job_row` → `durable_job_row_exact` (canonical).
  - `status_item` → `durable_job_row_exact`.
  - `os_badge_app_icon` → `activity_center_item`.
  - `toast` (mirror) → `durable_job_row_exact`.
  - `lock_screen_summary` → `durable_job_row_exact` (redacted).
- **Transient chrome dismissible without clearing the row**:
  `toast`, `os_badge_app_icon`, `lock_screen_summary`,
  `os_notification`.
- **Release trigger**: `terminal_state_reached`. The row moves
  to `completed` after install confirmation.
- **Audit**: `full_audit_trail_required`; download / install
  affects recovery posture and carries an evidence ref.

### Row: index rebuild {#row-index-rebuild}

- **Corpus row**: `dur:row:index_rebuild`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:workspace-index:primary`,
  `canonical_reopen_target_kind = durable_job_row_exact`,
  `canonical_reopen_target_ref = ux:reopen:index:workspace:durable`.
- **Source surfaces and reopen kinds**:
  - `durable_job_row` → `durable_job_row_exact` (canonical).
  - `status_item` → `durable_job_row_exact` (queue reason
    visible).
  - `os_badge_app_icon` → `activity_center_item`.
- **Transient chrome dismissible without clearing the row**:
  `os_badge_app_icon`. Status items reflect indexer state and
  are not dismissed by user gesture.
- **Release trigger**: `terminal_state_reached`.
- **Audit**: `full_audit_trail_required`; queue reason is
  recorded on the row.

### Row: transport reconnect {#row-transport-reconnect}

- **Corpus row**: `dur:row:transport_reconnect`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:transport-session:remote-mirror`,
  `canonical_reopen_target_kind = attention_item_exact`,
  `canonical_reopen_target_ref = ux:reopen:transport:remote-mirror:attention`.
- **Source surfaces and reopen kinds**:
  - `attention_item` → `attention_item_exact` (canonical).
  - `contextual_banner` → `attention_item_exact`.
  - `status_item` → `attention_item_exact`.
  - `durable_job_row` → `durable_job_row_exact`.
- **Transient chrome dismissible without clearing the row**:
  `os_notification` (when present), `os_badge_app_icon`. The
  contextual banner stays visible while transport is degraded.
- **Release trigger**: `user_resolve` (transport recovers and
  the user acknowledges) or `user_dismiss_to_history`.
- **Audit**: `full_audit_trail_required`; last-known-good state
  and last failure reason are exported.

### Row: extension crash {#row-extension-crash}

- **Corpus row**: `dur:row:extension_crash`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:extension:contoso-linter`,
  `canonical_reopen_target_kind = attention_item_exact`,
  `canonical_reopen_target_ref = ux:reopen:extension:contoso-linter:attention`.
- **Source surfaces and reopen kinds**:
  - `attention_item` → `attention_item_exact` (canonical).
  - `contextual_banner` → `attention_item_exact` (degraded
    disclosure).
  - `status_item` → `attention_item_exact`.
  - `os_notification` → `attention_item_exact` (redacted).
- **Transient chrome dismissible without clearing the row**:
  `toast`, `os_notification`, `os_badge_app_icon`. The contextual
  banner stays visible while the extension is unavailable.
- **Release trigger**: `extension_recovered` or
  `user_dismiss_to_history` (the user accepts the loss for the
  session).
- **Audit**: `full_audit_trail_required`; recovery actions are
  logged on the same canonical event id.

### Row: AI approval pending {#row-ai-approval-pending}

- **Corpus row**: `dur:row:ai_approval_pending`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:ai-apply:rename-symbol-batch`,
  `canonical_reopen_target_kind = attention_item_exact`,
  `canonical_reopen_target_ref = ux:reopen:ai-apply:rename-symbol:attention`.
- **Source surfaces and reopen kinds**:
  - `attention_item` → `attention_item_exact` (canonical).
  - `durable_job_row` → `durable_job_row_exact`.
  - `os_notification` → `attention_item_exact`, then routes
    through `review_sheet` for the approval gesture.
  - `os_badge_app_icon` → `activity_center_item`.
  - `lock_screen_summary` → `attention_item_exact` (privacy-safe)
    or `reopen_denied_requires_revalidation` if locked.
  - `companion_push` (cross-client) → `attention_item_exact`,
    then `review_sheet`.
- **High-risk no-bypass rule**: OS, lock-screen, or companion
  shortcuts MUST route through the in-product review sheet to
  approve the apply. Direct approval from the OS surface is
  non-conforming
  (`denial_reason = high_risk_action_bypassed_review`).
- **Transient chrome dismissible without clearing the row**:
  `toast`, `os_notification`, `os_badge_app_icon`,
  `lock_screen_summary`, `companion_push`.
- **Release trigger**: `user_resolve` (approve, deny, or open
  review and act) or `user_dismiss_to_history`.
- **Audit**: `full_audit_trail_required`; review-sheet activation
  preserves the same canonical event id.

### Row: quiet-hours held {#row-quiet-hours-held}

- **Corpus row**: `dur:row:quiet_hours_held`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:task-runner:quiet-hours-digest`,
  `canonical_reopen_target_kind = durable_job_row_exact`,
  `canonical_reopen_target_ref = ux:reopen:task-completions:quiet-hours-held:durable`.
- **Source surfaces and reopen kinds**:
  - `durable_job_row` → `durable_job_row_exact` (canonical).
  - `not_delivered_held` envelopes are emitted but not delivered
    until the mode exits.
  - `digest_group_row` (after release) → `durable_job_row_exact`
    or `activity_center_item` for the digest target.
  - `os_badge_app_icon` → `activity_center_item` (held /
    suppressed badge).
- **Transient chrome dismissible without clearing the row**:
  `digest_group_row` (after release), `os_badge_app_icon`. Held
  envelopes are not dismissible by user gesture; they release on
  mode exit.
- **Release trigger**: `mode_exit_grouped_digest_release`. The
  row moves to `completed` (or `needs_attention` if items inside
  the held burst require follow-up).
- **Audit**: `full_audit_trail_required`; durable history is
  preserved across every quiet-hours mode.

### Row: suppressed security notice {#row-suppressed-security-notice}

- **Corpus row**: `dur:row:suppressed_security_notice`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:trust-decision:secret-broker`,
  `canonical_reopen_target_kind = durable_job_row_exact`,
  `canonical_reopen_target_ref = ux:reopen:trust:secret-broker:audit`.
- **Source surfaces and reopen kinds**:
  - `durable_job_row` → `durable_job_row_exact` (canonical).
  - `contextual_banner` → `durable_job_row_exact` (in-product
    redacted disclosure).
  - `not_delivered_held` envelopes are emitted with
    `suppression_reasons = [admin_policy_suppression]`.
  - `os_badge_app_icon` → `activity_center_item` (held /
    suppressed badge); admin policy MAY narrow OS fanout
    further.
  - `os_notification`, `lock_screen_summary`, `companion_push`
    deliver as `placeholder_announced` when admin-narrowed, or
    as the canonical durable target when the user has policy
    permission.
- **Critical-safety tier**: `tier_critical_safety` does not
  silently suppress the durable row. Admin suppression MAY
  narrow OS-level fanout but MAY NOT block the in-product
  durable row
  (`denial_reason = admin_suppression_blocked_critical_safety`).
- **Transient chrome dismissible without clearing the row**:
  `os_badge_app_icon`, `os_notification`, `lock_screen_summary`,
  `companion_push`, `digest_group_row`. Held envelopes release
  only by `policy_revoked` or `user_explicit_clear`.
- **Release trigger**: `policy_revoked` (admin lifts
  suppression) or `user_explicit_clear` (the user with policy
  permission acknowledges the audit row).
- **Audit**: `policy_required_audit_trail`; the audit row is
  reviewable in evidence packets.

### Row: companion-delivered alert {#row-companion-delivered-alert}

- **Corpus row**: `dur:row:companion_delivered_alert`
- **Canonical durable identity**:
  `canonical_object_target_ref = obj:review:pr-1428`,
  `canonical_reopen_target_kind = attention_item_exact`,
  `canonical_reopen_target_ref = ux:reopen:collab:review-mention:pr-1428:attention`.
- **Cross-client lineage**: companion push and OS notification
  share `cross_client_canonical_event_id =
  ux:event:collab:review-mention:pr-1428`. Per-device durable
  identities are forbidden.
- **Source surfaces and reopen kinds**:
  - `attention_item` → `attention_item_exact` (canonical).
  - `companion_push` → `attention_item_exact` (cross-client
    lineage).
  - `os_notification` → `attention_item_exact` (redacted).
  - `os_badge_app_icon` → `activity_center_item` (mentions
    badge).
  - `lock_screen_summary` → `attention_item_exact`
    (privacy-safe scoped) or
    `reopen_denied_requires_revalidation` if the device is
    locked and the workspace requires re-auth.
  - `toast` (mirror) → `attention_item_exact`.
- **Transient chrome dismissible without clearing the row**:
  `toast`, `os_notification`, `os_badge_app_icon`,
  `lock_screen_summary`, `companion_push`. Dismissing the
  companion push on one device does not clear the desktop
  attention item.
- **Release trigger**: `user_resolve` (reply, mark resolved, or
  open review) or `user_dismiss_to_history`.
- **Audit**: `full_audit_trail_required`.

## Convergence assertion table

The table below mirrors the `convergence_assertions` block in
[`/artifacts/ux/durable_attention_corpus.yaml`](../../artifacts/ux/durable_attention_corpus.yaml).
Tooling treats every row as a typed conformance gate.

| Assertion id | Statement |
| --- | --- |
| `dur:assert:reopen_target_identity_singular` | Every delivery surface for a corpus row resolves to one `canonical_reopen_target_ref` or denies with `reopen_denied_requires_revalidation`. Reopen to a generic home screen is non-conforming. |
| `dur:assert:transient_dismissal_preserves_durable_row` | Dismissing transient chrome MUST NOT clear the durable row, the canonical event lineage, the audit trail, or the held / suppressed count. Only the named `release_trigger_class` releases the row. |
| `dur:assert:retention_partition_aligned_with_state_class` | Each row's `activity_partition` agrees with the `state_class → activity_partition` mapping in [`/schemas/ux/job_row.schema.json`](../../schemas/ux/job_row.schema.json) and the partition rule in [`/schemas/ux/job_row_retention.schema.json`](../../schemas/ux/job_row_retention.schema.json). |
| `dur:assert:held_or_suppressed_audit_trail` | Held and policy-suppressed rows MUST emit a `not_delivered_held` envelope and an `audit_event_id` even when no interruption was delivered. Durable history is preserved across every quiet-hours mode. |
| `dur:assert:cross_client_lineage_singular` | Cross-client deliveries share one `cross_client_canonical_event_id` with the desktop attention item. Per-device durable identities are forbidden. |

## Non-conforming patterns

The drill map flags the following as contract violations and the
corresponding `denial_reason`:

- a transient surface activation that opens a generic home
  screen instead of the canonical durable target —
  `reopen_to_generic_home_forbidden`;
- dismissing a toast, OS notification, badge, lock-screen
  summary, companion push, or digest group row that also clears
  the durable row, attention item, or audit trail —
  `transient_dismissal_attempted_to_clear_durable_row`;
- companion push minting its own per-device durable identity
  separate from the desktop attention item —
  `companion_cross_client_divergence`;
- an OS-level shortcut completing a high-risk or mutating action
  without re-entering the in-product review sheet —
  `high_risk_action_bypassed_review`;
- admin suppression silently blocking a `tier_critical_safety`
  durable row —
  `admin_suppression_blocked_critical_safety`;
- a held envelope that did not emit an audit trail —
  `held_event_missing_audit_trail`;
- a quiet-hours hold that discarded durable history —
  `quiet_hours_hold_discarded_durable_history`;
- a retention packet whose `activity_partition` disagrees with
  its `state_class` —
  `retention_packet_partition_disagrees_with_state_class`;
- a retention packet that drops `canonical_event_id` lineage —
  `retention_packet_drops_canonical_event_lineage`.

## Reuse guarantee

Owning subsystems and review tooling MUST resolve every transient
delivery they ship through the corpus row, the retention packet,
and the drill section above. They MUST NOT mint per-surface
reopen verbs, per-device durable identities, or shortcut
mutation paths for any covered scenario. Adding a new corpus row
is additive-minor and bumps `corpus_id` schema version; renaming
a row id, repurposing a reopen target, or widening a transient
chrome class to clear durable state is breaking and requires a
new decision row.
