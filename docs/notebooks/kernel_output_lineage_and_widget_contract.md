# Notebook kernel-output lineage, widget-trust, and execution-queue contract

This document freezes the notebook-specific lineage layer that ties one
output back to its notebook document, its cell, its cell-execution run,
its kernel session, its replay capture (when present), its widget-trust
state (when applicable), and the export/support packets that may carry
it forward.

It is normative. A notebook surface, a notebook export packet, a support
bundle, an automation evidence packet, an AI evidence packet, or any
review surface that names a notebook output MUST emit the
machine-readable records linked below. The records explain whether the
output is live, captured, replayed, orphaned, or widget-gated; whether
its widget runtime is trusted, blocked, gated, replay-only, unavailable,
or unsupported; and which queue state produced it. The records carry
refs only — raw notebook JSON bodies, raw cell source bytes, raw output
bytes, raw widget state, raw kernel-protocol frames, raw absolute paths,
raw URLs, raw hostnames, raw cookies, and raw credential material never
cross these records.

Companion artifacts:

- [`/schemas/notebooks/kernel_output_lineage.schema.json`](../../schemas/notebooks/kernel_output_lineage.schema.json)
  defines `notebook_kernel_output_lineage_record`.
- [`/schemas/notebooks/widget_trust_state.schema.json`](../../schemas/notebooks/widget_trust_state.schema.json)
  defines `notebook_widget_trust_state_record`.
- [`/fixtures/notebooks/kernel_output_cases/`](../../fixtures/notebooks/kernel_output_cases/)
  contains worked YAML cases for live trusted output, replayed output
  after kernel loss, blocked widget, queued remote execution, and an
  orphaned output preserved in review/export.

## Composition, not redefinition

This contract composes with existing notebook contracts:

- [`/docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
  owns notebook document identity, stable cell identity, four-axis
  trust, kernel-session states, kernel transports, execution-queue
  admission classes, output lineage classes, widget trust states, and
  no-auto-rerun posture.
- [`/docs/notebooks/output_viewer_truth_contract.md`](./output_viewer_truth_contract.md)
  owns notebook output viewer state, heavy-output degradation,
  buffering/freeze, accessibility, and inclusion policy. The viewer
  state record cites a kernel-output lineage record by ref; this
  contract owns that lineage record.
- [`/docs/runtime/context_cache_and_terminal_restore_contract.md`](../runtime/context_cache_and_terminal_restore_contract.md)
  owns execution-context cache and terminal-restore metadata, including
  the rule that restored metadata never auto-reruns commands.
- [`/docs/execution/task_event_and_evidence_contract.md`](../execution/task_event_and_evidence_contract.md)
  owns task channels, task events, and evidence-link records.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  owns support-bundle scope, consent, retention, and redaction
  envelope.

When these sources disagree, the source contract wins and this document
MUST be updated in the same change. This contract does not ship notebook
runtime execution, widget sandboxes, or a final notebook viewer
implementation. It freezes the row shapes those implementations must
read and write.

## Why a separate lineage record

The notebook output viewer state record from
`output_viewer_truth_contract.md` is per-surface UI state: it changes
every time the user scrolls, freezes, virtualizes, detaches, or expands
an output. The lineage record is per-output truth: it changes when the
output is produced, captured, replayed, orphaned, or its widget trust
moves. A single lineage record may be cited from many viewer-state
records, many include-policy records, many task-event records, and
many export/support packets. Keeping the lineage record separate keeps
output truth and runtime truth from drifting apart.

A lineage record:

- never carries raw runtime metadata, raw protocol frames, raw widget
  comm state, or raw kernel-spec JSON;
- never invents a trust posture beyond the four-axis notebook trust
  model from ADR-0022;
- never promotes a captured, replayed, orphaned, or widget-gated output
  to live;
- never auto-reruns a cell to "refresh" a stale or replayed output.

## Output lineage record family

Notebook kernel-output lineage has one record:
`notebook_kernel_output_lineage_record`. It carries:

| Field group | Purpose |
|---|---|
| Identity refs | Notebook document, stable cell id, cell-execution id, output id, output-block descriptor. |
| Kernel refs | Kernel session id, kernelspec ref, kernel transport class, execution-context ref, target identity witness ref, environment fingerprint ref, data snapshot ref. |
| Lineage class | Re-export of ADR-0022 `notebook_output_lineage_class` plus the `notebook_output_trust_state` posture. |
| Output block | Output block class and MIME-bundle descriptor ref. |
| Queue state | Execution-queue state class and admission class. |
| Replay capture | Replay capture class, replay capture ref, and replay disclosure. |
| Widget refs | Widget-trust state record ref and widget runtime ref when the output is a widget. |
| Export/support refs | Refs to export packets, support packets, automation evidence packets, AI evidence packets, viewer state records, and task-event envelopes that cite this lineage. |
| Provenance | Source hash ref, renderer/schema ref, policy epoch ref, audit event refs. |
| Disclosure | Reviewer-facing label and redaction class. |

The record is a notebook-specific projection: the same conceptual
output may also be reachable through a generic output viewer object
(per `docs/ux/output_log_viewer_contract.md`), through a task-event
envelope (per `docs/execution/task_event_and_evidence_contract.md`),
and through a support-bundle item. The lineage record cites those by
ref; it does not redefine them.

### Output lineage classes (re-export)

Every lineage record MUST cite one `notebook_output_lineage_class` from
ADR-0022. The values are re-exported verbatim:

| Class | Meaning |
|---|---|
| `live_output_from_current_session` | Output was produced by the current kernel session's current cell-execution run. |
| `captured_output_from_prior_session` | Output was saved into the notebook from a prior kernel session. |
| `replayed_from_captured_output` | The surface re-rendered the captured output without re-executing the kernel. |
| `orphaned_no_kernel_binding` | The output exists in the document but cannot be bound to a kernel session. |
| `widget_gated_output` | The output is a widget; live binding is gated by the widget-trust axis. |

Trust posture re-exports `notebook_output_trust_state` from ADR-0022 and
MUST agree pairwise with the lineage class:

- `live_output_from_current_session` ↔ `output_trust_live_from_current_session`
- `captured_output_from_prior_session` ↔ `output_trust_captured_from_prior_session`
- `replayed_from_captured_output` ↔ `output_trust_replayed_from_captured_output`
- `orphaned_no_kernel_binding` ↔ `output_trust_orphaned_no_kernel_binding`
- `widget_gated_output` ↔ `output_trust_widget_gated`

A surface that cites a lineage class without the matching trust posture
is non-conforming.

### Output block class

The lineage record carries a closed `output_block_class` that names what
kind of output is bound to the lineage. This is independent of how the
output is rendered (which the viewer state owns) and of how it is
exported (which the include policy owns).

| Class | Meaning |
|---|---|
| `stream_text_block` | Stream output (stdout/stderr) from the kernel. |
| `error_traceback_block` | Error/traceback output. |
| `display_data_block` | Display-data MIME bundle (text, image, html, table, etc). |
| `execute_result_block` | Execute-result MIME bundle. |
| `widget_view_block` | Widget output bound to a widget runtime. |
| `update_display_data_block` | Update-display data targeting an existing display id. |
| `clear_output_marker_block` | Clear-output marker that pruned a prior output. |
| `imported_output_block` | Output imported from a captured or external source (CI artifact, support replay) without a live kernel run. |

`widget_view_block` outputs MUST cite a widget-trust state record;
non-widget classes MUST NOT cite a widget-trust state record.

## Execution-queue state

The lineage record carries a closed `execution_queue_state_class` that
names the queue state of the cell-execution run that produced (or would
have produced) the output. It composes with — but does not replace —
ADR-0022's `notebook_execution_queue_admission_class`. Admission is the
host's accept/deny decision; queue state is the run's progress.

| Class | Meaning | Required signals |
|---|---|---|
| `not_queued` | The output exists outside the queue lane (captured/imported/orphaned). | No queue record ref needed. |
| `queued_admitted_pending_dispatch` | Cell admitted; waiting for the kernel to pick it up. | Queue record ref required; kernel session ref present. |
| `queued_waiting_on_kernel` | Admission was deferred because no kernel was available. | Admission class is `admitted_deferred_no_kernel`; kernel session ref MAY be null. |
| `running_current_cell` | Kernel is executing the cell now. | Kernel session ref required; cell-execution ref required. |
| `completed_current_session` | Cell finished in the current kernel session. | Cell-execution ref required; lineage class is `live_output_from_current_session` or `captured_output_from_prior_session`. |
| `cancelled_by_user` | User cancelled the queued or running cell. | Auto-rerun forbidden. |
| `cancelled_by_trust_downgrade` | Document/kernel/widget-trust downgrade revoked the run. | Trust transition audit ref required. |
| `cancelled_by_policy` | Admin policy or content-class policy denied execution. | Policy ref required. |
| `replaying_from_capture` | The viewer/export is replaying captured output instead of running the cell. | Replay capture ref required; lineage class is `replayed_from_captured_output`. |
| `orphaned_no_kernel_binding` | The cell-execution lineage cannot be bound to a kernel session. | Kernel session ref MUST be null; lineage class is `orphaned_no_kernel_binding`. |
| `remote_unavailable_session_lost` | Remote kernel transport is gone; the run cannot continue without a fresh remote session. | Transport class is a remote class; remote-agent session ref present but unreachable. |

The `cancelled_*` and `orphaned_*` and `remote_unavailable_*` states
MUST set `auto_rerun_forbidden = true`. A queue state class that allows
auto-rerun without an explicit user decision is non-conforming.

## Replay-capture posture

A replay capture is an evidence-only re-rendering of a captured output.
The lineage record carries a closed `replay_capture_class`:

| Class | Meaning |
|---|---|
| `no_replay` | The output is not being replayed. |
| `replay_from_captured_output` | The current rendering is a replay of a captured output from a prior session. |
| `replay_from_imported_capture` | The replay source is imported (CI artifact, support replay, share bundle); never authoritative. |
| `replay_blocked_no_kernel` | Replay was requested but no kernel is available; the surface stays on captured/orphaned semantics. |
| `replay_blocked_identity_drift` | Replay was requested but the kernel identity changed; replay is blocked until the user admits a fresh kernel. |
| `replay_blocked_trust_or_policy` | Replay was requested but trust or policy denies replay; the surface stays on captured semantics. |

A `replay_*` value other than `no_replay` MUST set
`auto_rerun_forbidden = true`. A replay never promotes captured output
to live.

## Widget-trust state record family

Notebook widget trust has one record:
`notebook_widget_trust_state_record`. It carries:

| Field group | Purpose |
|---|---|
| Identity refs | Notebook document, stable cell id, output id, widget runtime ref, kernel session ref. |
| Trust class | `widget_trust_class` plus the underlying ADR-0022 `notebook_widget_trust_state`. |
| Explanation | A typed explanation class plus a reviewer-facing label that names why the widget is trusted, blocked, gated, replay-only, runtime-unavailable, or unsupported. |
| Safe-open actions | A non-empty closed list of `widget_safe_open_action_class` describing the user's next safe options. |
| Lineage refs | Optional ref to the kernel-output lineage record this widget belongs to and refs to audit events. |
| Disclosure | Redaction class and capture timestamp. |

### Widget-trust classes

The contract reserves six closed `widget_trust_class` values:

| Class | Meaning |
|---|---|
| `widget_trusted_live_binding_admitted` | The user (or policy) admitted live widget binding. |
| `widget_blocked_static_fallback` | Widget live binding is blocked; only a static MIME-bundle preview renders. |
| `widget_gated_pending_admission` | Widget live binding is gated; the user can review the static preview and admit it. |
| `widget_replay_only_evidence` | Widget output is replay-evidence only; no live binding will be offered (e.g. inside a support replay or imported capture). |
| `widget_unavailable_runtime` | The widget runtime is unavailable (kernel gone, runtime not installed, transport down). |
| `widget_unsupported_widget` | The widget class is not supported by this build/policy; static fallback or text fallback only. |

These classes are a richer projection of ADR-0022's underlying
`notebook_widget_trust_state`. They MUST agree pairwise:

- `widget_trusted_live_binding_admitted` ↔ underlying
  `widget_trust_admitted_with_preview`.
- `widget_blocked_static_fallback` ↔ `widget_trust_denied_by_default`,
  `widget_trust_suppressed_by_trust`, `widget_trust_suppressed_by_policy`,
  or `widget_trust_suppressed_by_content_class`.
- `widget_gated_pending_admission` ↔ `widget_trust_denied_by_default`
  while a preview admission is offered.
- `widget_replay_only_evidence` ↔ underlying may be any blocked or
  denied state; the distinguishing fact is that the surface is a replay
  surface rather than a live one.
- `widget_unavailable_runtime` ↔ `widget_trust_unavailable_no_kernel`
  or a runtime-missing variant.
- `widget_unsupported_widget` ↔ `widget_trust_suppressed_by_content_class`
  or a null underlying state when the class is not modelled by the
  ADR-0022 vocabulary.

A surface that contradicts these pairings is non-conforming.

### Required explanation

Every widget-trust state record MUST carry a closed
`widget_trust_explanation_class`:

- `user_admitted_after_preview`
- `default_denied_for_safety`
- `denied_by_workspace_or_document_trust`
- `denied_by_admin_policy`
- `denied_by_content_class_safe_preview_detector`
- `replay_only_no_live_runtime_offered`
- `runtime_unavailable_no_kernel`
- `runtime_unavailable_widget_runtime_missing`
- `runtime_unavailable_remote_transport_lost`
- `unsupported_widget_class`
- `unsupported_widget_protocol_version`

It MUST also carry a `widget_trust_explanation_label` (reviewer-facing,
redaction-aware string). A record with an empty or generic explanation
is non-conforming.

### Safe-open actions

Every widget-trust state record MUST carry a non-empty list of
`widget_safe_open_action_class` values:

| Action | Meaning |
|---|---|
| `keep_static_fallback` | Continue showing the static MIME-bundle preview. |
| `admit_widget_with_preview` | Admit live binding after reviewing the preview. |
| `downgrade_to_textual_fallback` | Render a textual representation of the widget. |
| `omit_widget_from_export` | Exclude live widget state from this export/support capture. |
| `install_widget_runtime` | Offer the user an install/admit path for the missing widget runtime. |
| `ensure_kernel_available` | Offer an action that resolves the missing kernel (start, attach, reconnect). |
| `reconnect_remote_session` | Offer a reconnect path for the remote kernel transport. |
| `contact_admin_for_policy_review` | Surface that admin policy denies widget binding and suggest a review path. |
| `replay_only_inspect_evidence` | Make clear the surface is replay; offer a typed inspect-only path. |
| `no_action_unsupported_widget` | The widget class is not supported; nothing else can be offered. |

Every record MUST list at least one safe-open action. A record with
zero safe-open actions or a single `no_action_unsupported_widget` paired
with a class other than `widget_unsupported_widget` is non-conforming.

## Required pairings (frozen)

A lineage record's class MUST imply queue, replay, kernel, and widget
state per the table below. The schema enforces these pairings via
`allOf`.

| Lineage class | Required queue state | Required replay class | Kernel session ref | Widget-trust state ref |
|---|---|---|---|---|
| `live_output_from_current_session` | `running_current_cell` or `completed_current_session` | `no_replay` | non-null | null unless `output_block_class = widget_view_block` |
| `captured_output_from_prior_session` | `not_queued` or `completed_current_session` | `no_replay` | may be null (prior session ref) | null unless widget |
| `replayed_from_captured_output` | `replaying_from_capture` | `replay_from_captured_output` or `replay_from_imported_capture` | may be null | null unless widget |
| `orphaned_no_kernel_binding` | `orphaned_no_kernel_binding` | any `replay_blocked_*` value or `no_replay` | MUST be null | null unless widget |
| `widget_gated_output` | any non-`running_current_cell` value (no live binding) | any value other than `no_replay` is permitted only when the widget is in replay/blocked state | may be null | non-null |

A lineage record that violates this table is non-conforming. A widget
output's lineage MUST cite a widget-trust state record; a non-widget
output MUST NOT.

## Export and support packet linkage

A lineage record carries opaque refs to the export/support packets that
include or omit the output:

- `export_packet_refs` for notebook saves, paired-text exports, clean
  notebook exports, report exports, artifact share bundles, and
  detached-viewer snapshots. The refs resolve to
  `notebook_output_include_policy_record` ids per
  `output_viewer_truth_contract.md`.
- `support_packet_refs` for support bundles. The refs resolve to
  support-bundle item ids per `docs/support/support_bundle_contract.md`.
- `automation_evidence_packet_refs` for evidence packets cited by
  task-event envelopes per
  `docs/execution/task_event_and_evidence_contract.md`.
- `ai_evidence_packet_refs` for AI evidence packets that cite the
  output as evidence.
- `viewer_state_refs` for `notebook_output_viewer_state_record`
  instances that currently render or have rendered the output.

A lineage record MUST be cited by at least one of: a viewer state ref,
an export packet ref, a support packet ref, an automation evidence
packet ref, or an AI evidence packet ref. A lineage record cited by
nothing is dead inventory and is non-conforming. (For pre-execution
queue states the citing record may be a viewer state record that
renders a placeholder for the queued cell.)

Export, support, and AI evidence flows MUST preserve the lineage record
verbatim through any redaction step. They MAY drop payload bytes; they
MUST NOT drop the lineage class, the trust posture, the queue state,
the replay class, the widget-trust ref, or the auto-rerun-forbidden
flag. A consumer that strips these fields under redaction is
non-conforming.

## Required invariants

- Every notebook output cited from any surface, packet, or record
  resolves to exactly one `notebook_kernel_output_lineage_record`.
- Lineage class, queue state class, replay class, widget-trust state,
  kernel-session refs, and auto-rerun-forbidden flags survive redaction.
- A captured, replayed, orphaned, blocked-widget, runtime-unavailable,
  unsupported-widget, or queued-but-not-running output never appears as
  a live trusted runtime result in any fixture, viewer state, or export
  packet.
- Replay and orphan posture forbid auto-rerun; the user always picks
  the next step.
- Widget outputs cite a widget-trust state record; non-widget outputs
  do not.
- Raw notebook JSON, raw cell source, raw output bytes, raw widget
  state, raw kernel-protocol frames, raw paths, raw URLs, raw
  hostnames, raw cookies, and raw credentials never cross these
  records.
