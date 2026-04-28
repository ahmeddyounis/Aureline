# Durable-work row contract: state classes, progress grammar, partitions, and linkbacks

This document freezes the durable-work row contract for long-running,
queued, reviewable, completed, held, and suppressed work. It is the
render-level companion to the attention-routing and notification
contracts: those contracts define the event lineage and envelope; this
one defines the durable row a user can return to after a toast,
notification, modal handoff, compact badge, or status strip disappears.

The contract is normative. Where this document disagrees with the UI /
UX Spec sections it quotes, the source spec wins and this document plus
the schema and fixtures must change in the same patch. Where this
document disagrees with a downstream surface's private progress row,
activity row, or notification linkback vocabulary, this document wins
and the surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/job_row.schema.json`](../../schemas/ux/job_row.schema.json)
  defines the machine-readable `job_row_record` that backs durable work
  rows in the activity center, task surfaces, compact status overflow,
  support exports, and notification linkback resolution.
- [`/fixtures/ux/job_rows/`](../../fixtures/ux/job_rows/) contains
  worked examples for running, queued, approval-blocked,
  attention-required, completed, partially completed, quiet-hours held,
  and policy-suppressed rows.

## Upstream contracts

This contract composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns delivery-surface class, attention class, interruptibility tier,
  durable-job lifecycle state, quiet-hours mode, suppression reason,
  badge class, and source-subsystem vocabulary.
- [`notification_delivery_contract.md`](./notification_delivery_contract.md)
  owns canonical event ids, routing, dedupe, privacy payload classes,
  dismissal verbs, release-from-hold steps, and durable linkback rules.
- [`chronology_row_contract.md`](./chronology_row_contract.md) owns
  history rows and export previews after a row leaves immediate view.
- [`status_bar_contract.md`](./status_bar_contract.md) and
  [`status_strip_family_contract.md`](./status_strip_family_contract.md)
  own compact persistent state and overflow summaries.
- [`dialog_sheet_contract.md`](./dialog_sheet_contract.md) owns the
  handoff rule for long-running actions launched from dialogs.
- [`degraded_mode_pattern.md`](./degraded_mode_pattern.md) owns
  degraded-state disclosure and safe repair actions.

The `job_row_record` carries refs back to those owners rather than
renaming their fields.

## Scope

Frozen at this revision:

- one durable row shape for long-running or reviewable work;
- eight durable-work state classes;
- one progress-form grammar shared by activity center rows, task
  surfaces, compact status overflow, and support export previews;
- one activity-center partition contract for current work, needs
  attention, completed work, and suppressed or held work;
- linkback rules from toasts, badges, OS notifications, companion push,
  and compact status surfaces to the canonical durable row;
- minimum target, source, suppression, dedupe, and support-export
  fields needed for later audit.

Out of scope:

- building the full activity center UI;
- implementing notification adapters or OS-specific delivery;
- defining subsystem-specific run logs, output viewers, or evidence
  packets beyond opaque refs.

## Durable-work invariant

Long-running work is never represented only by a transient toast,
vanishing modal spinner, status-bar flicker, or OS notification. If work
can outlive the originating surface, can affect cost, policy, network,
trust, provider state, or recovery posture, or can produce logs,
evidence, artifacts, or follow-up actions, it must mint a durable row.

A durable row must remain reviewable after:

- the user dismisses a toast;
- quiet hours, focus mode, presentation mode, screen sharing, privacy
  mode, power-saver mode, or admin policy suppresses an interruption;
- the dialog or sheet that launched the work closes;
- the active editor group or panel changes;
- the product restarts and restores durable chronology.

## Required row anatomy

Every `job_row_record` carries the same top-level anatomy. A renderer
may condense the row in compact layouts, but it may not drop the backing
fields or replace them with prose-only state.

| Field | Required content | Non-conforming collapse |
| --- | --- | --- |
| `canonical_event_id` | Shared event id used by toasts, badges, OS notifications, durable rows, and exported history. | A per-surface notification id with no durable event lineage. |
| `durable_job_id_ref` | Ref to the durable job object or durable-job envelope record. | Treating the visible row id as the only durable identity. |
| `activity_partition` | One of `current_work`, `needs_attention`, `completed`, or `suppressed_held`. | One chronological pile where held, failed, and running rows look alike. |
| `state` | State class, outcome class, state label, reason, and required visible cues. | Generic `Working`, `Done`, or `Failed` strings with no class meaning. |
| `source` | Source subsystem, actor, execution origin, client scope, and originating surface. | Copy that says "system" without a stable owner. |
| `target` | Canonical target ref, target kind, scope label, object label, and source refs. | Raw paths, raw URLs, or display text as identity. |
| `progress` | Progress-form set, phase or queue reason, actor/subsystem label, progress bar or spinner posture, cancellation posture, detail or evidence ref. | Spinner-only rows for material work or progress bars with no label. |
| `impact` | Whether the work affects cost, policy, network, trust, provider state, or recovery posture. | Hiding billable, remote, policy, trust, or provider effects until after completion. |
| `actions` | Command-backed cancel, review, retry, open details, open evidence, acknowledge, or export actions. | Pointer-only controls, destructive shortcuts, or `Dismiss` as the only action for consequential work. |
| `linkbacks` | Linkback targets from transient and OS surfaces to the canonical durable row, owner, history, evidence, or export preview. | Notification click-through to a generic home screen. |
| `suppression` | Quiet-hours modes, suppression reasons, held count, policy source, release rule, and audit refs. | Dropping held events because no interruption was delivered. |
| `support_export` | Exportability, redaction posture, required export fields, dedupe refs, and raw-material exclusion. | Support exports that cannot explain source, target, suppression, or dedupe. |
| `accessibility` | Status announcement, keyboard reachability, focus return, and visible cue parity. | Color-only state or pointer-only inspection. |

## State classes

Every row resolves to exactly one `state_class`. These are display,
activity-center, support-export, and audit classes, not localized copy.

| `state_class` | Meaning | Required visible cues | Activity partition |
| --- | --- | --- | --- |
| `running` | Work is actively making progress in the foreground or background. | State label, phase label, progress form, actor/subsystem label, target label, elapsed age, open-details action; cancel action when cancellable. | `current_work` |
| `queued_waiting` | Work is accepted but not executing yet. | State label, queue reason, expected boundary, actor/subsystem label, target label, queued age, open-details action; cancel action when cancellable. | `current_work` |
| `needs_approval` | The next step is blocked on a user, policy, trust, provider, or admin decision. | State label, approval owner/source, expiry or timeout when available, review action, detail/evidence link, target label. | `needs_attention` |
| `attention_required` | Work failed, stalled, or completed with a follow-up burden. | State label, failure or blocker reason, dominant next action, history or evidence link, target label, source label. | `needs_attention` |
| `completed` | Work completed successfully and is historical but still reviewable. | State label, finish time, outcome summary, reopen/export action where relevant, target label. | `completed` |
| `partially_completed` | Some targets, phases, or items succeeded while others failed, were skipped, hidden, or excluded. | State label, included/excluded/failed counts, partial reason, open-review action, evidence link. | `needs_attention` |
| `quiet_hours_held` | The product intentionally delayed interruption because a quiet-hours, focus, presentation, screen-share, privacy, or power-saver mode applied. | State label, held count or class, suppression reason, release rule, audit/history path. | `suppressed_held` |
| `policy_suppressed` | An event exists but policy blocks full display, fanout, or action. | State label, policy source, remaining safe detail, suppressed action class, audit/export path. | `suppressed_held` |

Failure is intentionally represented through `attention_required` unless
the failed work is only one part of a mixed result, in which case the row
uses `partially_completed`. Completion and failure must both remain
reviewable after focus loss.

## Progress-form grammar

The `progress.forms` array names which forms are present on the row.
Rendering can vary by density, but export and accessibility must
preserve the same form classes.

| Form | Use | Required companion field |
| --- | --- | --- |
| `labeled_progress_bar` | Determinate work with measurable progress. | `progress.progress_bar` with label, numerator, denominator, and unit. |
| `spinner_only` | Truly indeterminate work, only when no meaningful progress unit exists and the row still names phase, source, and details. | `progress.indeterminate_reason_label`. |
| `queue_reason` | Queued or waiting work. | `progress.queue_reason_label` and `progress.expected_boundary_class`. |
| `phase_label` | Active or blocked phase. | `progress.phase_label`. |
| `actor_subsystem_label` | Human, system, extension, remote, AI, policy, or provider source. | `progress.actor_or_subsystem_label`. |
| `elapsed_or_queued_age` | How long the work has been running or waiting. | `progress.age_label`. |
| `approval_source` | Approval owner/source for blocked next step. | `progress.approval_source_label`. |
| `cancel_action` | Row offers a cancel path. | An `actions` entry with `action_kind = cancel_job`. |
| `open_detail_action` | Row opens the durable detail, owner, log, or history surface. | An `actions` entry with `action_kind = open_details`. |
| `detail_or_evidence_link` | Work affects cost, policy, network, trust, provider state, or recovery posture, or has logs/evidence. | `progress.detail_or_evidence_ref`. |
| `failure_or_partial_summary` | Failed or mixed outcome. | `state.reason_label` plus counts where applicable. |
| `completion_summary` | Successful terminal outcome. | `state.reason_label` and `timestamps.finished_at`. |
| `held_or_suppressed_reason` | Quiet-hours held or policy-suppressed rows. | `suppression.suppression_reasons` or `suppression.policy_source_ref`. |

Rules:

- Labeled progress bars are preferred for work expected to matter.
- Spinner-only is allowed only with a phase label, actor/subsystem label,
  target label, open-detail action, and an indeterminate reason.
- A row that is cancellable must expose `cancel_action`; a row that is
  not cancellable must say so through `progress.cancellability_class`.
- Every row must expose `open_detail_action`.
- Work that affects cost, policy, network, trust, provider state, or
  recovery posture must expose `detail_or_evidence_link`.
- Machine-readable output must not be polluted by progress chatter;
  progress belongs in this row, the task surface, or a separate stream.

## Activity-center partitions

The activity center may render tabs, sections, filters, or saved views,
but the backing partition values are fixed:

| `activity_partition` | Included state classes | Ordering rules |
| --- | --- | --- |
| `current_work` | `running`, `queued_waiting` | Sort by user-visible risk, then age or queue priority. Running work with policy, network, trust, provider, cost, or recovery impact sorts above routine background work. |
| `needs_attention` | `needs_approval`, `attention_required`, `partially_completed` | Sort by actionability, user risk, expiry, and source authority. Approval expiries and failed irreversible work outrank routine follow-up. |
| `completed` | `completed` | Sort by finish time with unread completion and pinned evidence surfaced before older routine rows. |
| `suppressed_held` | `quiet_hours_held`, `policy_suppressed` | Sort by safety relevance, release condition, policy source, and held count. Critical safety cannot be silently hidden. |

The activity center must keep these partitions distinct. A compact
layout may show one badge or overflow button, but expanding it must name
the partition and count class rather than rendering one ambiguous total.

## Linkback rules

Every transient, compact, OS-level, or companion delivery for durable
work resolves to the canonical durable row or an explicitly typed
placeholder. The linkback target is part of the event lineage and the
job row.

Required rules:

- Toasts that mirror durable work must link to `durable_job_row_exact`,
  `attention_item_exact`, `activity_center_item`, or `history_lane_row`.
- Badges derive from deduped durable items, not raw event count. Badge
  expansion must land on the partition and row class that produced the
  count.
- OS notifications and lock-screen summaries may use redacted labels,
  but activation must reopen the narrowest truthful durable destination
  or explain why revalidation is required.
- Companion push uses the same `canonical_event_id`; it may not mint a
  per-device durable identity.
- Dismissing a toast removes only the transient surface. It does not
  delete, acknowledge, resolve, suppress, or archive the durable row.
- `Acknowledge`, `Resolve`, `Dismiss`, `Snooze`, `Mute`, and `Suppress`
  retain the distinct semantics defined by the notification-delivery
  contract.

## Support export and audit fields

Durable rows carry enough structured data for support export,
notification dedupe, and later audit without raw private material.

Required export/audit facts:

- `canonical_event_id`, `event_lineage_id_ref`,
  `durable_job_id_ref`, and `job_row_id`;
- source subsystem, actor ref, execution origin, client scope, and
  originating surface;
- canonical target ref, target kind, scope label, and source record refs;
- state class, activity partition, outcome class, progress form classes,
  and visible cue classes;
- quiet-hours modes, suppression reasons, held count, policy source, and
  release rule;
- dedupe-key scheme, badge classes, grouped-burst ref, and source
  envelope refs;
- detail/evidence refs for cost-, policy-, network-, trust-, provider-,
  or recovery-affecting work;
- redaction class, export field refs, and explicit raw-material
  exclusion.

Raw bodies, raw paths, raw URLs, raw host names, raw provider payloads,
raw prompt/completion text, raw command bodies, raw logs, tokens,
credential material, and secret values never appear in a durable row.

## Non-conforming cases

The following are contract violations:

- a long-running action appears only as a toast, status flicker, or
  modal spinner;
- a completion or failure disappears after focus loss;
- a row has a progress bar with no label or a spinner with no phase,
  actor/subsystem, and detail path;
- a queued row does not name why it is waiting;
- a row that affects cost, policy, network, trust, provider state, or
  recovery posture has no detail or evidence link;
- a held or suppressed event has no audit trail;
- a badge count cannot expand to deduped durable rows;
- an OS notification opens a generic home screen instead of the
  canonical durable row, detail surface, or truthful placeholder;
- support export cannot explain source, target, state, suppression,
  dedupe, and redaction posture.
