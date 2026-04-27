# Notebook output viewer truth contract

This document freezes the notebook-specific truth layer for rich output
viewers, dataframe/table viewers, variable inspectors, detached output
panels, notebook exports, report exports, and support captures.

It is normative. A notebook surface that renders output, copies output,
exports output, summarizes output, or captures output for support MUST
emit the machine-readable records linked below. The records explain what
the reviewer is seeing, whether it is live or evidence from another
runtime moment, how heavy output degraded, and why an output was
included, summarized, detached, truncated, blocked, or omitted.

Companion artifacts:

- [`/schemas/notebooks/output_viewer_state.schema.json`](../../schemas/notebooks/output_viewer_state.schema.json)
  defines `notebook_output_viewer_state_record`.
- [`/schemas/notebooks/output_include_policy.schema.json`](../../schemas/notebooks/output_include_policy.schema.json)
  defines `notebook_output_include_policy_record`.
- [`/fixtures/notebooks/output_viewer_cases/`](../../fixtures/notebooks/output_viewer_cases/)
  contains worked YAML cases for live, captured, stale, replayed,
  orphaned, widget-gated, detached, truncated, and omitted outputs.

## Composition, not redefinition

This contract composes with existing notebook and output contracts:

- [`/docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
  owns notebook document identity, stable cell identity, four-axis trust,
  kernel-session states, execution-queue admission, output-lineage
  classes, widget-trust states, raw-JSON fallback, and notebook audit
  event ids.
- [`/docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
  owns the cross-surface output viewer object for output class, size
  bucket, viewer mode, origin class, trust posture, freshness, size
  disclosure, truncation, search/copy/export controls, and live-set
  composition.
- [`/docs/data/database_tooling_contract.md`](../data/database_tooling_contract.md)
  owns typed result-grid, dataframe handoff, row-count truth, truncation,
  and typed export posture for database-originated tables.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  owns support-bundle scope, consent, retention, and redaction envelope.

Notebook output viewer state records MUST cite those records by opaque
ref. They MUST NOT carry raw notebook JSON bodies, raw cell source, raw
output bytes, raw widget state, raw kernel protocol frames, raw absolute
paths, raw URLs, raw hostnames, raw cookies, or raw credential material.

## Record families

Notebook output truth has two records:

1. `notebook_output_viewer_state_record` describes what a notebook
   surface is rendering now: output truth state, viewer surface,
   output kind, heavy-output posture, live/capture/stale basis,
   lineage refs, queue/replay markers, widget-trust state, buffering
   and freeze posture, accessibility posture, provenance, and linked
   cross-surface output-viewer object.
2. `notebook_output_include_policy_record` describes what leaves the
   viewer through save, copy, export, report generation, share bundles,
   detached snapshots, or support capture: include decision, payload
   representation, truncation, summary/detached handling, redaction,
   reproducibility impact, support-review impact, and audit refs.

The split is intentional. A viewer can truthfully render a large live
dataframe while an export policy includes only metadata, or a support
capture can include a summarized stale output without changing what the
user saw in the notebook.

## Output truth states

Every notebook output surface MUST expose exactly one
`output_truth_state_class`.

| State | Meaning | Required reviewer cue |
|---|---|---|
| `live_output` | Output is bound to the current kernel session and current cell execution. | Live label with kernel/session ref and last update time. |
| `captured_output` | Output was saved in the notebook from a prior session. | Captured label with producing kernel/session refs when available. |
| `stale_output` | Output was once live or captured, but the source cell, kernel identity, environment, data snapshot, or target context has drifted. | Stale label naming the drift class. |
| `replayed_output` | Output was rendered from captured data without re-executing the kernel. | Replay label stating replay is evidence only. |
| `orphaned_output` | Output exists, but no producing kernel/session can be resolved. | Orphan label and no automatic rerun path. |
| `blocked_widget_output` | Output is a widget or active output gated by widget trust, sandbox, policy, or unavailable runtime. | Widget-blocked label plus static/text fallback state. |
| `intentionally_omitted_output` | The payload is absent by user choice, policy, redaction, or destination limits. | Omitted label with include-policy ref. |
| `unavailable_output` | The output cannot be rendered because a dependency is gone or unsupported. | Unavailable label with recovery or inspect-only path. |

These states do not replace ADR-0022 output lineage. The viewer state
MUST also cite `notebook_output_lineage_class` when an output exists:
`live_output_from_current_session`, `captured_output_from_prior_session`,
`replayed_from_captured_output`, `orphaned_no_kernel_binding`, or
`widget_gated_output`.

## Viewer surfaces

The same state record applies to:

- inline cell output areas;
- output galleries;
- dataframe and generic table viewers;
- variable explorer rows and detail panels;
- image, audio, video, HTML, and rich media viewers;
- detached viewers promoted from a notebook cell;
- raw or textual fallback viewers;
- export preview and support-capture preview surfaces.

Detached viewers MUST remain linked to the notebook document, cell,
output id, and producing or captured kernel/session refs. A detached
viewer MUST NOT imply that it is a new canonical artifact unless the
include policy records an explicit detached artifact reference.

## Heavy output degradation

Heavy output degradation MUST be typed and reviewable. A viewer that
cannot safely render an output inline MUST switch to a declared
`heavy_output_posture_class` and a declared `degradation_reason_class`.

| Posture | Allowed use |
|---|---|
| `within_inline_budget` | Full output is safe to render inline without threatening shell responsiveness. |
| `row_virtualized` | Rows are windowed; counts and selection scope disclose visible vs total or unknown total. |
| `row_and_column_virtualized` | Rows and columns are windowed; hidden columns remain searchable/exportable only if the policy says so. |
| `media_thumbnail_or_proxy` | Media is represented by a thumbnail, poster, waveform, or proxy. |
| `open_detail_required` | Inline surface shows a summary and promotes to a dedicated viewer for full review. |
| `summary_only_required` | Payload is summarized because size, policy, trust, or support scope prevents full rendering. |
| `raw_or_textual_fallback` | Rich renderer is unavailable or blocked; a raw/textual representation is shown with a fallback reason. |
| `render_blocked` | No renderable payload is shown; the block reason is typed and export policy is explicit. |

The state record carries declared budgets, including inline byte/row/cell
limits, visible row and column windows, media pixel budget, update-rate
budget, and whether the budget came from viewer policy, resource
governor, provider cap, user choice, or support/export redaction.

Silent truncation is non-conforming. Every truncated, sampled, paged,
summarized, detached, or fallback output MUST preserve a typed disclosure
in both the viewer state and any include policy that leaves the product.

## Live, captured, stale, and replay labels

Live labeling follows the current kernel session. A live output MUST cite
the active kernel session, cell execution, execution-context record, and
last update timestamp. If the kernel disconnects, restarts, changes
identity, or becomes unverifiable, the live label MUST transition away
from `live_output`.

Captured labeling follows saved output. Captured output MUST NOT become
live because a renderer rehydrates it. Re-rendering captured output emits
`replayed_output` when the replay path matters to review, support, or
export.

Stale labeling follows drift. A viewer MUST label output as stale when
any declared basis changes after capture:

- source cell bytes or cell identity;
- kernel session identity, kernelspec, or kernel transport;
- execution context, environment capsule, lockfile, or target identity;
- input dataset or database snapshot ref;
- notebook trust state, widget trust state, or policy epoch;
- renderer/schema version needed to interpret the output.

Unavailable labeling is distinct from stale labeling. A missing kernel,
missing widget runtime, missing detached artifact, unsupported MIME type,
or support bundle that carried metadata only is `unavailable_output`,
not stale output.

## Buffering and freeze semantics

Live outputs and variable inspectors MUST declare buffering and freeze
state:

- `live_following` means the viewer follows current updates.
- `user_frozen` means the reviewer froze the view; buffered changes are
  counted and not silently inserted above the anchor.
- `budget_frozen` means the viewer froze to protect latency, memory, or
  accessibility budget.
- `producer_rate_limited` means updates are throttled before they reach
  the viewer.
- `buffer_spilled_or_dropped` means chunks moved to a spill store or were
  dropped under a typed policy.
- `capture_finalized` means the stream ended or was captured as a static
  basis.

A frozen viewer MUST keep search, copy, export, and support capture
scoped to the declared visible, buffered, or full-source basis. A "jump
to latest" action MUST disclose whether it discards the current anchor,
changes the export basis, or exposes buffered changes.

## Accessibility minimums

Notebook output viewers MUST remain reviewable with keyboard and
assistive technology under the same state classes they show visually:

- tables/dataframes expose semantic headers, row/column counts or
  unknown-count labels, virtualized window position, and selected scope;
- variable explorer rows expose variable identity, type/shape summary,
  live/snapshot/stale/unavailable state, and kernel ref;
- media outputs expose text alternatives, captions/transcripts where
  available, or a typed `alt_text_missing` / `transcript_missing` state;
- live regions throttle announcements and never announce every row of a
  high-volume stream;
- blocked widgets and raw fallbacks expose the same block reason and
  fallback action to screen readers;
- detached viewers preserve focus return to the originating cell or
  nearest safe notebook anchor.

Accessibility fallback is not a loss of truth. If a rich output becomes
textual for accessibility, the viewer state MUST say so with
`accessibility_representation_class`.

## Provenance and lineage

Every state record MUST carry refs for the provenance it knows:

- notebook document, stable cell id, and output id;
- notebook output-lineage record, when an output exists;
- kernel session, kernelspec, execution context, and target identity
  witness, when known;
- cell execution id and execution-queue marker, when known;
- output-viewer object record from the general output viewer contract;
- live-set state record for live streams;
- widget-trust record and widget runtime state for widget outputs;
- MIME bundle descriptor, typed table/result-grid record, media proxy,
  detached artifact, or raw/text fallback descriptor;
- capture basis, source hash, renderer/schema version, policy epoch,
  redaction profile, and audit event refs.

Unknown provenance is allowed only when it is explicit. The state record
then uses `orphaned_output`, `unavailable_output`, or an unknown/metadata
only include decision instead of inventing a live or captured claim.

## Include and export policy

Every copy, save, export, report, detached snapshot, share, and support
capture MUST resolve a `notebook_output_include_policy_record` for each
output or output group.

| Include decision | Payload result | Reproducibility/support effect |
|---|---|---|
| `include_full_payload` | Complete admitted payload leaves the viewer. | May support exact review only if lineage and environment refs are present. |
| `include_metadata_only` | Payload omitted; refs, hashes, type, size, freshness, and lineage remain. | Support can reason about state but cannot inspect output body. |
| `include_summary` | Human and machine summary leaves the viewer. | Reproducibility is limited; summary is not evidence of full payload. |
| `include_truncated_payload` | A bounded subset leaves with truncation class. | Downstream readers MUST see that the payload is incomplete. |
| `include_detached_reference` | Payload is externalized to a detached artifact ref. | Review requires resolving the artifact and its redaction/retention state. |
| `exclude_by_default` | Payload is not included unless the user admits it. | Default for risky or large notebook outputs in review/support contexts. |
| `exclude_by_user_choice` | User intentionally excluded the payload. | Exclusion is audit-visible and reproducibility may be reduced. |
| `exclude_by_policy` | Policy or redaction blocks inclusion. | Support sees policy class, not payload. |
| `exclude_widget_not_trusted` | Widget live state or active payload is excluded. | Static/text fallback may be included separately if admitted. |
| `exclude_unavailable` | Payload is unavailable at capture/export time. | Support sees unavailable reason and last known refs. |

Notebook file save, paired text export, report export, artifact share,
clipboard copy, detached viewer snapshot, and support bundle capture are
separate destinations. A decision admitted for one destination MUST NOT
be reused for another without resolving the destination class again.

Support capture defaults to metadata and summaries unless the user or
administrator explicitly broadens scope under the support-bundle
contract. Export packets MUST preserve omission, truncation, detached,
widget-blocked, and stale/replay labels so downstream reviewers do not
reverse-engineer private runtime metadata.

## Raw fallback and active content

Raw fallback is a review state, not permission to leak raw bodies across
boundaries. A raw/textual fallback viewer may render a safe representation
in process, but the state and include-policy records carry refs, hashes,
MIME descriptors, and fallback reasons.

Active content follows the four-axis notebook trust model. Widget output
starts denied by default, can become static/text fallback, and only binds
live comm state after widget trust admits it. When the widget runtime is
unavailable, the state is `blocked_widget_output` or
`unavailable_output`, not captured live state.

## Required invariants

- The canonical notebook file remains the `.ipynb`; output viewer records
  are evidence and UI state, not canonical source.
- A viewer never promotes captured, replayed, stale, orphaned, blocked,
  omitted, or unavailable output to live.
- Heavy output degradation is typed before the UI exceeds rendering,
  memory, update-rate, or accessibility budgets.
- Every export/support path states whether output was embedded, omitted,
  summarized, truncated, detached, or metadata-only.
- Output-to-kernel lineage, queue/replay markers, widget-trust states,
  orphan labels, and omission reasons remain available after the kernel
  is gone or widget runtime is unavailable.
- Raw bytes and secret-bearing payloads never cross these boundary
  records.
