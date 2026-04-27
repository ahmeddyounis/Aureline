# Notebook collaboration, presenter focus, and share contract

This document freezes the notebook-specific collaboration layer for
cell anchors, output anchors, comment anchors, presenter focus, runtime
boundary labels, notebook share scopes, and redaction-before-share
review.

It is normative. A notebook surface that shares a cell, output, comment,
presentation step, runtime status, export packet, browser review, or
support capture MUST emit the machine-readable records linked below.
Those records explain what is local-only, what is shared live, what is
captured for export or presentation, and what was redacted or omitted
before a share leaves the local boundary.

Companion artifacts:

- [`/schemas/notebooks/cell_anchor.schema.json`](../../schemas/notebooks/cell_anchor.schema.json)
  defines `notebook_anchor_record`,
  `notebook_anchor_drift_review_record`,
  `notebook_presenter_focus_record`, and
  `notebook_anchor_audit_event_record`.
- [`/schemas/notebooks/notebook_share_scope.schema.json`](../../schemas/notebooks/notebook_share_scope.schema.json)
  defines `notebook_runtime_boundary_record`,
  `notebook_share_scope_record`, `notebook_redaction_review_record`,
  `notebook_share_continuity_record`, and
  `notebook_share_audit_event_record`.
- [`/fixtures/notebooks/notebook_collab_cases/`](../../fixtures/notebooks/notebook_collab_cases/)
  contains worked YAML cases for anchor drift review, output rebind
  review, presenter focus with captured runtime state, redaction before
  export, offline review continuity, and shared-live scope disclosure.

## Composition, not redefinition

This contract composes with existing notebook and collaboration
contracts:

- [`/docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
  owns canonical `.ipynb` preservation, stable cell identity,
  notebook trust axes, kernel session states, output lineage, widget
  trust, and no-auto-rerun posture.
- [`/docs/notebooks/output_viewer_truth_contract.md`](./output_viewer_truth_contract.md)
  owns output truth states, output include policy, heavy-output
  degradation, live/captured/stale/replayed labels, and widget output
  handling.
- [`/docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md)
  owns session lifecycle, shared-object authority, permission downgrade,
  and cross-surface anchor-drift rules.
- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md)
  owns presenter/follow state and control grants. This notebook
  contract narrows those rows for cell/output focus and never widens
  presenter authority into run, kernel, terminal, debug, or edit
  control.
- [`/docs/collaboration/consent_retention_contract.md`](../collaboration/consent_retention_contract.md)
  owns retention, consent, export, delete, guest, route, residency, and
  audit envelopes for collaboration sessions.

When these sources disagree, the source contract wins and this document
MUST be updated in the same change. This document does not ship live
notebook collaboration, presenter tooling, a replay player, or a share
service. It freezes the row shapes those implementations must read and
write.

## Anchor contract

Notebook anchors MUST be stable, reviewable, and scoped to the notebook
object they name. The primary identity is the notebook document ref plus
stable cell id. Output, comment, metadata, and attachment anchors add
their own stable refs. Raw notebook JSON, raw cell source, raw output
payloads, raw comments, raw paths, raw URLs, and raw credentials never
cross the anchor boundary.

Anchor target classes:

| Class | Meaning | Required stable refs |
|---|---|---|
| `cell_source_anchor` | Source text, rendered markdown, or raw cell body. | notebook document ref, stable cell id, source span ref or source hash ref |
| `cell_output_anchor` | One output or output group. | notebook document ref, stable cell id, output id, output-lineage ref |
| `cell_comment_thread_anchor` | Comment or review thread bound to a cell or output. | notebook document ref, stable cell id, comment-thread ref, optional output id |
| `cell_metadata_anchor` | Metadata key or structured metadata path. | notebook document ref, stable cell id, metadata path ref |
| `cell_attachment_anchor` | Attachment or media object bound to a cell. | notebook document ref, stable cell id, attachment ref |
| `notebook_document_anchor` | Document-level review note, header state, or notebook-level presentation target. | notebook document ref |

Anchor basis classes:

- `stable_cell_id_only`
- `stable_cell_id_plus_source_span`
- `stable_cell_id_plus_output_id`
- `stable_cell_id_plus_comment_thread`
- `stable_cell_id_plus_metadata_path`
- `stable_cell_id_plus_attachment_ref`
- `document_level_notebook_ref`
- `line_only_degraded_fallback`

Line-only fallback is allowed only as an explicit degraded state. It is
never allowed to masquerade as a stable cell, output, or comment anchor.

## Drift and rebind review

Anchor drift is reviewable. A surface MUST NOT silently move a comment,
focus target, or output reference across cells or outputs just because a
nearby cell, line, or output looks similar.

Drift states:

| State | Required behavior |
|---|---|
| `anchor_exact` | The anchor still resolves to the same cell and target ref. |
| `anchor_remapped_same_cell_review_optional` | The source span moved within the same cell; the surface shows the remap label. |
| `anchor_cell_reordered_exact` | The cell moved but stable cell id and target ref still match. |
| `anchor_output_reexecuted_review_required` | A rerun produced a new output candidate; output rebind requires review. |
| `anchor_output_deleted_metadata_only` | The output no longer exists; preserve anchor metadata and show deleted-output state. |
| `anchor_cell_deleted_relocation_forbidden` | The cell was deleted; preserve metadata and forbid relocation to another cell. |
| `anchor_cross_cell_rebind_candidate_review_required` | A candidate in another cell exists; rebind requires explicit review. |
| `anchor_line_only_degraded_review_required` | Only line or context fallback remains; show degraded anchor state. |
| `anchor_remote_projection_diverged_local_only` | Local and shared projections differ; keep anchor local until rejoin or review resolves it. |

Legal rebind decisions:

- `no_rebind_exact`
- `preserve_anchor_with_drift_label`
- `user_confirmed_same_cell_rebind`
- `user_confirmed_cross_cell_rebind`
- `user_confirmed_output_rebind`
- `supersede_with_new_anchor`
- `silent_rebind_denied`
- `rebind_denied_by_policy`

Cross-cell and output rebinds require a
`notebook_anchor_drift_review_record` with `review_required = true`, a
reviewer or owner actor ref, the prior target, the candidate target, and
the decision. The original anchor remains auditable. If the user wants
to move the discussion permanently, the product mints a new anchor and
links it through `supersede_with_new_anchor`; it does not overwrite the
old row.

## Presenter and follow focus

Presenter focus for notebooks is a view-authority layer. It may point
participants at a notebook document, cell, output, comment, metadata
path, or runtime-status row. It never grants cell execution, kernel
restart, output clearing, editing, export, terminal, debug, or secret
reveal authority.

Every presenter focus row MUST name:

- the bound collaboration session and presenter state refs;
- the notebook document ref;
- a notebook anchor ref for the subject being focused;
- a focus subject class;
- the follow state (`following_presenter`, `breakaway_local_view`,
  `presenter_focus_active`, `focus_paused`, or
  `follow_unavailable_captured_only`);
- `follow_mutation_forbidden = true`;
- the runtime boundary label visible to followers;
- the restore ref or local-selection ref used when presentation exits.

Breakaway is not drift. A participant who breaks away is in
`breakaway_local_view`; the local view is authoritative until the
participant returns to presenter. A degraded follow MUST show
`follow_unavailable_captured_only` or an equivalent local-only state
instead of injecting input, scrolling hidden panes, or running notebook
cells to catch up.

## Runtime boundary labels

Notebook sharing must name where runtime state came from before a user
trusts it. The runtime boundary label is required on presenter focus,
share-scope, continuity, and runtime-boundary records.

Runtime boundary labels:

| Label | Meaning |
|---|---|
| `no_kernel_document_only` | Notebook is readable and editable, but no runtime is selected. |
| `local_only_no_shared_runtime` | Runtime exists locally and is not shared. |
| `shared_live_local_kernel` | A live local kernel is being shared under the session envelope. |
| `shared_live_remote_kernel` | A live remote kernel is being shared under the session envelope. |
| `shared_live_managed_kernel` | A live managed kernel is being shared under the session envelope. |
| `captured_for_export_or_presentation` | State is captured evidence for export or presentation, not live runtime. |
| `runtime_disconnected_last_known` | Runtime disconnected; prior outputs remain visible as captured or stale evidence. |
| `provider_runtime_unavailable` | Provider-linked runtime state cannot be resolved. |
| `offline_review_no_live_runtime` | Offline review is available, but no live collaboration or runtime state exists. |

A captured output or replayed output MUST NOT become shared live because
the presenter focuses it or because a browser companion renders it.
The output viewer state still controls live/captured/stale/replayed
truth.

## Share scopes

Notebook share scopes are destination-specific. A decision admitted for
presentation is not reusable for support, export, collaboration, or
clipboard copy without recomputing the scope and redaction review.

Destination classes:

- `collaboration_session`
- `presentation_session`
- `notebook_export`
- `report_export`
- `artifact_share_bundle`
- `support_bundle`
- `clipboard_copy`
- `browser_companion_view`
- `offline_review_packet`

Shared notebook state classes:

| Class | Required disclosure |
|---|---|
| `local_only_not_shared` | State remains local; remote viewers see nothing unless exported later. |
| `shared_live` | Live shared state is bound to a session, runtime boundary, and retention envelope. |
| `captured_for_export` | Snapshot is export evidence, not a live notebook or live kernel. |
| `captured_for_presentation` | Snapshot is used for presentation or follow fallback, not live execution. |
| `captured_for_support` | Snapshot is support evidence under support-bundle scope. |
| `metadata_only_shared` | Only metadata and refs leave; payloads are omitted or redacted. |
| `stale_overlay_only` | Collaboration overlay is stale or archived; local notebook state remains separate. |

The visible share label MUST identify at least one of those classes.
Generic labels such as "shared notebook" or "notebook available" are
non-conforming.

## Redaction before share

Redaction is resolved before share, not after upload, not after export,
and not when a recipient opens the packet. A share scope is not admitted
until it cites a redaction review row.

The redaction review MUST classify:

- outputs: full payload, metadata only, summary, truncated payload,
  detached reference, omitted sensitive payload, blocked active content,
  or unavailable;
- secrets: no intersection, secret refs only, values redacted, brokered
  handles only, blocked secret-bearing payload;
- transient runtime state: omitted by default, metadata-only kernel
  status, captured snapshot declared, live shared declared,
  disconnected prior state declared, or no runtime state;
- provider-linked resources: metadata refs only, links removed,
  signed refs only, cross-tenant block, or no provider resources.

Raw output bodies, raw cell source, raw widget state, raw kernel
protocol frames, raw environment variables, raw credential values, raw
provider URLs, raw account ids, raw hostnames, raw paths, and raw user
identifiers are forbidden on both schemas. Records carry opaque refs,
hashes, coarse classes, and reviewable labels.

## Downgrade and continuity

Offline review, stale collaboration overlays, runtime disconnects, and
captured-versus-live transitions are continuity states, not failure
shortcuts.

Required continuity behavior:

- offline notebook review keeps open, search, diff, comment, and export
  paths available where local policy allows;
- stale collaboration overlays remain visible as stale or archived
  overlays, never as current live comments or current presenter focus;
- runtime disconnect preserves prior outputs as captured, stale,
  orphaned, or unavailable evidence according to the output viewer
  contract;
- reconnect, rejoin, rerun, export, and continue-local actions are
  distinct recovery paths;
- no downgrade may replay notebook cells, runtime state, presenter
  focus, comment publishing, or export upload without a fresh user or
  policy admission.

## Required invariants

- Shared notebook state never hides whether content is local-only,
  shared live, captured for export, captured for presentation, captured
  for support, metadata-only, or stale overlay.
- Cell, output, comment, metadata, and attachment anchors bind through
  stable notebook identity first; line-only fallback is visibly
  degraded.
- Anchor drift and cross-cell or output rebind are reviewable and
  auditable; silent rebind is forbidden.
- Presenter focus and follow state preserve explicit view authority
  and never imply edit, run, kernel, debug, terminal, export, or secret
  authority.
- Runtime boundary labels survive export, presentation, browser
  companion, support, and offline review surfaces.
- Redaction review completes before share admission; blocked or pending
  redaction blocks share.
- Outputs, secrets, transient runtime state, provider-linked resources,
  comments, and attachments use destination-specific share scopes.
- Captured, stale, replayed, orphaned, omitted, unavailable, or widget
  gated output never becomes live by presentation, export, or browser
  rendering.

## Additive change discipline

Adding a new anchor target class, drift state, rebind decision, focus
subject, runtime boundary label, share destination, shared state class,
redaction posture, continuity state, audit event id, or denial reason is
additive-minor and bumps the relevant schema version. Repurposing an
existing value, removing a value, or weakening a denial gate is breaking
and requires the normal decision workflow.

## Out of scope

- Implementing notebook real-time collaboration transport.
- Implementing presenter UI, follow UI, or replay UI.
- Implementing notebook kernel sharing, remote execution, or provider
  resource resolution.
- Implementing byte-level redaction. The secret broker and support
  bundle contracts own byte-level redaction; this contract names the
  boundary rows and admission rules.
