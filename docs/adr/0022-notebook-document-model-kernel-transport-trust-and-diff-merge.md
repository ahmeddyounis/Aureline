# ADR 0022 — Notebook document model, kernel transport, trust posture, and diff/merge posture seed

- **Decision id:** D-0027 (see `artifacts/governance/decision_index.yaml#D-0027`)
- **Status:** Proposed — this is an ADR seed. It reserves the canonical
  `.ipynb` preservation rules, the stable-cell-identity rules, the
  Aureline-owned notebook metadata namespace, the four-axis trust
  posture (document trust, kernel trust, output trust, widget trust),
  the kernel-transport and kernel-discovery record shape, the output-
  lineage classes, the diff / merge / review posture, and the paired
  text export rules so the structured-artifact review seed, the
  compatibility archetype rows, the workspace-trust packet, the
  remote-agent service-placement row, and any future notebook viewer
  / kernel host / cell-aware merge driver cannot invent them ad hoc.
  Full freeze lands in a successor ADR once the open questions in
  §Open questions are closed.
- **Decision date:** pending
- **Freeze deadline:** 2027-02-28
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council (co-required with security_trust_review because the widget-trust / rich-output sanitization rules, the kernel-trust transitions, and the output-lineage `replayed_from_captured_output` / `widget_gated_output` rules carry the trust / policy invariants the trust-review remit already owns)
- **Related requirement ids:** `none`

## Context

Aureline's protected paths already assume notebooks are a first-class,
high-risk review surface. ADR-0001 froze the identity-mode envelope
every surface inherits. ADR-0006 froze the VFS canonical path
identity that binds a notebook's on-disk truth. ADR-0007 froze the
credential-handle projection notebooks inherit when an output or an
environment capsule carries a secret. ADR-0008 froze the admin-policy
narrowing ceiling that gates widget rendering, kernel launch, and
rich-output execution. ADR-0009 froze the execution-context object
model that every kernel activation resolves. ADR-0011 projected the
lifecycle markers that carry `notebook_manual_open` and
`notebook_kernel_launch` rows. ADR-0018 froze the workspace-trust
packet that gates notebook-kernel admission in restricted mode.
ADR-0019 reserved the capability-world identity scheme that
`notebook-kernel` and `notebook-observe` worlds project through.
ADR-0020 seeded the remote-agent session contract, including the
service-placement row `near_code_services.notebook_kernel` and the
target-identity-witness rules that govern remote kernels.

The structured-artifact review seed
(`docs/review/structured_artifact_review_seed.md`) and its matrix
(`artifacts/review/structured_artifact_classes.yaml`) already reserve
the `jupyter_notebook` review row with
`metadata_filter_preset = preserve_jupyter_and_aureline_namespaces_only`,
`output_handling_preset = ignore_outputs_by_default_with_opt_in_inclusion`,
`cell_id_stability_required = true`,
`paired_text_representation.pair_state = export_only_no_round_trip`,
`canonical_direction_ref = canonical-direction:notebook->export_script:v1`,
and `round_trip_corpus_ref = round-trip-corpus:jupyter_notebook:v1`.
The compatibility archetype row `notebook_first_data_workflow` reserves
`notebook_kernel_local_only` as the first-stable promise. The
runtime-taxonomy seed at
`docs/runtime/target_discovery_and_install_review_taxonomy.md`
already names notebook-trust, widget-trust, and structured
round-trip risk as controlled-term columns. `docs/ux/entry_restore_truth_audit.md`
reserves notebook restore as `prior_work_preserved_but_not_rerun` so
a reopened notebook never auto-executes outputs.

What none of those rows yet names is **the contract a notebook
document, a notebook cell, a notebook output, a notebook kernel
session, a notebook widget, and a notebook diff / merge record MUST
speak to stay cross-lane coherent**. Without a typed seed, every lane
that touches a notebook — the review surface, the merge driver, the
support-export bundle, the remote-agent kernel placement, the AI
evidence packet, the workspace-trust packet, the portable-profile
capture, and any future notebook viewer — would have to invent its
own cell-identity scheme, its own metadata-survival rule, its own
kernel-trust ladder, its own output-lineage vocabulary, and its own
diff / merge posture. That is exactly the fragmentation this ADR
seed forbids.

This ADR rides alongside ADR-0001 (identity-mode envelope inherited
on every open), ADR-0004 (notebook records cross RPC as typed
payloads; raw notebook JSON bodies, raw cell source bodies, raw
output bytes, raw kernel protocol frames, and raw widget state bytes
never do), ADR-0005 (notebook views ride the shared subscription
envelope with authority class `derived_knowledge` and a declared
freshness hint), ADR-0006 (the notebook path token resolves through
VFS canonical path identity), ADR-0007 (credential-handle classes
caught inside a notebook output resolve through the reveal contract
only; raw secret bytes never appear in a captured output), ADR-0008
(admin-policy narrowing is the orthogonal ceiling over every
notebook capability), ADR-0009 (execution-context resolution
precedes any kernel activation; the environment capsule and
toolchain activator ride the context snapshot), ADR-0011
(`notebook_manual_open` and `notebook_kernel_launch` lifecycle rows
project through the five-axis markers), ADR-0016 (notebook paste,
clipboard, and command routing stay on the command-dispatch
boundary), ADR-0018 (trust-decision packet gates notebook-kernel
launch in restricted mode; repo-owned kernel activators do not auto-
launch), ADR-0019 (the `notebook-observe` and `notebook-kernel`
worlds' budget and world-admission rules stay frozen; this ADR
names the document and kernel those worlds observe), ADR-0020
(remote-agent sessions that carry a `near_code_services.notebook_kernel`
placement row resolve this contract for the local document and this
ADR for the local trust / output ladder; remote-agent authority
rules remain the ceiling on remote kernels), and ADR-0021 (terminal
protocol rules govern any shell / repl backing a kernel; a terminal
spawned by a kernel remains under ADR-0021's pty_owner_class /
shell-integration / clipboard / restore posture, and this ADR does
not widen them).

A production notebook editor and a production kernel host do not
land at this milestone. What this seed reserves is the **canonical
notebook-document preservation rules**, the **stable-cell-identity
rules**, the **Aureline-owned metadata namespace rules**, the
**four-axis trust posture**, the **kernel-transport and kernel-
discovery record shape**, the **kernel-session lifecycle states**,
the **output-lineage classes**, the **widget-trust classes**, the
**execution-queue extension point**, the **paired text export
rules**, and the **diff / merge / review posture** so the successor
ADR has concrete records and invariants to compose against rather
than prose.

## Decision

Aureline reserves seven record families — **notebook-document open
record**, **notebook-cell record**, **notebook-kernel session
record**, **notebook-output lineage record**, **notebook-widget
trust record**, **notebook-diff-merge posture record**, and
**notebook-audit event record** — plus a frozen vocabulary for
document trust, kernel trust, output trust, widget trust, output
lineage, kernel transport, kernel-session lifecycle, cell-id
stability, metadata-survival class, paired-text export posture,
diff / merge posture, metadata-filter preset, raw-JSON fallback
class, denial reason, and audit-event id. Every vocabulary below
is opened as an enumerable set whose initial members are frozen by
this seed and whose additions are additive-minor with a
`notebook_protocol_schema_version` bump. Repurposing any named item
is breaking and requires a new decision row.

The intent is deliberately narrower than the successor ADR. This
seed freezes **shape, names, invariants, and refusal posture**, not
the concrete editor, not a renderer-side cell layout, not a specific
kernel protocol choice, and not a default keymap.

### Notebook document model and canonical preservation

A notebook document resolves through the VFS canonical path identity
from ADR-0006 and opens through the command-dispatch boundary from
ADR-0016. A notebook MUST open, diff, merge, export, and save
without silently discarding unknown top-level, metadata, cell-level,
or output-level keys. The on-disk `.ipynb` JSON is the canonical
source; paired text exports are derived, not canonical.

Reserved `notebook_canonical_preservation_class` values:

- `canonical_ipynb_preserved` — the on-disk `.ipynb` format is the
  canonical source; every round-trip (open / save / diff / merge /
  export) preserves top-level `nbformat`, `nbformat_minor`, document
  metadata, cell order, cell source bytes, cell metadata, outputs,
  and unknown vendor keys.
- `export_only_no_round_trip` — a paired text representation exists
  (script export, Markdown export) but is derived; it is never
  promoted to canonical source by any lane.
- `no_paired_text_representation` — the document has no paired text
  form; compare surfaces fall back to `raw_json_fallback_allowed` per
  the diff / merge rules below.

Reserved `notebook_cell_id_stability_class` values:

- `stable_cell_id_required` — every cell carries a stable
  `cell_id_ref` (opaque) whose identity survives save, diff, merge,
  re-order, and re-materialization. This is the baseline posture.
- `stable_cell_id_minted_on_first_save` — a legacy-format document
  without `cell_id`s receives deterministic ids minted on first save
  through the Aureline metadata namespace; subsequent round-trips
  honour `stable_cell_id_required`.
- `cell_id_stability_not_available_raw_json_fallback_only` — the
  document cannot carry stable cell ids (malformed, truncated, or
  non-notebook JSON misfiled as `.ipynb`); the diff surface falls
  back to raw-JSON compare per `raw_json_fallback_allowed`.

Reserved `notebook_metadata_survival_class` values:

- `survival_required_jupyter_and_aureline_namespaces` — top-level
  document metadata and per-cell metadata under the frozen
  `jupyter`, `kernelspec`, `language_info`, and `aureline`
  namespaces MUST survive every round-trip. Filtering them on
  save, diff, or merge is non-conforming.
- `survival_required_vendor_namespaces` — vendor-namespaced
  metadata (for example `collab`, `colab`, `vscode`, `nteract`)
  MUST survive round-trip; filtering is permitted ONLY behind an
  explicit metadata-filter preset and MUST be disclosed on the
  compare surface.
- `survival_recommended_unknown_vendor_namespaces` — unknown vendor
  namespaces SHOULD survive round-trip; a filter that drops them
  MUST declare itself and is non-default.

Reserved `notebook_aureline_metadata_namespace` keys (reserved for
future additive use; carrying an unknown key under `aureline` is
breaking unless declared here):

- `aureline.document_id_ref` — opaque, stable document id bound to
  the VFS canonical path identity. Never cross-document reusable.
- `aureline.cell_id_minting_policy` — which minting policy was used
  when cell ids were first assigned. Values: `preserved_from_source`,
  `minted_on_first_save`, `not_applicable`.
- `aureline.last_kernel_session_id_ref` — opaque ref to the
  `notebook_kernel_session_record` that last produced outputs for
  this document. Evidence only; never an authority to rerun.
- `aureline.last_trust_state_ref` — opaque ref to the
  `notebook_trust_state_record` in force when the document was last
  saved. Evidence only.
- `aureline.paired_text_export_ref` — opaque ref to the paired
  text-export binding when one exists.
- `aureline.widget_trust_summary_refs` — ordered refs to the widget-
  trust summary records for every widget instance rendered on the
  last save.
- `aureline.output_lineage_refs` — ordered refs to output-lineage
  records per output emitted on the last save.
- `aureline.metadata_survival_report_ref` — opaque ref to the
  metadata-survival report produced on the last save (which
  vendor namespaces survived; which were filtered; which were
  unknown). Raw vendor bodies MUST NOT appear in the report.
- `aureline.support_export_scope` — which support-export scope
  this document was admitted to on the last save (if any).

Rules (frozen):

1. **Canonical `.ipynb` is the source of truth.** No lane may
   replace the on-disk `.ipynb` with a paired text form on save,
   diff, merge, or export. A surface that promotes an exported
   script to canonical source is non-conforming.
2. **Unknown keys survive by default.** Top-level, document-
   metadata, cell-metadata, and output-metadata keys outside the
   frozen `jupyter` / `kernelspec` / `language_info` / `aureline`
   namespaces MUST round-trip. Filtering them requires an explicit
   preset and MUST be disclosed per the metadata-filter preset
   vocabulary.
3. **Cell ids are stable.** Every cell carries a stable
   `cell_id_ref`; re-order, edit, merge, and save MUST preserve
   identity. A lane that rewrites cell ids silently is non-
   conforming.
4. **The Aureline namespace is closed.** Keys under `aureline.*`
   are enumerated above. A future key lands through additive-minor
   schema bump; inventing an ad hoc `aureline.*` key on a surface
   is breaking.
5. **Paired text exports are derived.** A paired `.py` / `.md`
   export is evidence-only and carries the canonical-direction
   ref `canonical-direction:notebook->export_script:v1`. Save of
   the paired text form MUST NOT mutate the canonical `.ipynb`;
   regeneration of the paired text form is permitted under the
   generated-artifact safe-edit posture.

### Four-axis trust posture

A notebook carries four independent trust axes that MUST NOT be
collapsed into one generic "notebook trust" state. Each axis
projects its own suppression and its own disclosure.

Reserved `notebook_document_trust_state` values:

- `document_trust_inherited_from_workspace` — the document
  inherits the ADR-0018 workspace-trust state. Default.
- `document_trust_elevated_on_explicit_grant` — the user granted
  per-document trust above the workspace default; the grant is
  session-bounded or persistent per the trust-decision packet and
  MUST survive neither workspace downgrade nor identity change.
- `document_trust_restricted_by_policy` — admin policy narrows
  this document to restricted regardless of workspace trust.
- `document_trust_revoked` — the user or policy revoked trust; the
  document remains editable and reviewable, kernel launch and
  widget execution are denied.

Reserved `notebook_kernel_trust_state` values:

- `kernel_trust_inherited_from_document_trust` — default: kernel
  launch admission honours the document-trust state above, the
  workspace-trust state from ADR-0018, and the execution-context
  root from ADR-0009.
- `kernel_trust_elevated_for_signed_repo_recipe` — the kernel-
  activator is a repo-owned recipe signed per the ADR-0018 signed-
  repo allowance; elevation is additive, never widening past the
  admin-policy ceiling.
- `kernel_trust_restricted_by_policy` — admin policy narrows
  kernel launch even under document trust `elevated_on_explicit_grant`.
- `kernel_trust_unavailable_no_kernel` — no kernel is resolvable
  (toolchain missing, remote-agent not connected, managed-workspace
  not ready); the document remains editable and reviewable.

Reserved `notebook_output_trust_state` values:

- `output_trust_live_from_current_session` — an output is freshly
  produced by the current `notebook_kernel_session_record`; its
  provenance is the live session.
- `output_trust_captured_from_prior_session` — an output was
  captured by a prior kernel session and is rendered with an
  evidence-only disclosure; re-execution produces a new output,
  it does not authoritatively update the captured one.
- `output_trust_replayed_from_captured_output` — an output was
  rendered from a captured output without executing the kernel;
  replay is evidence only and is visibly disclosed.
- `output_trust_orphaned_no_kernel_binding` — an output exists in
  the document but no kernel session can be bound to it (kernel
  name changed, toolchain renamed, remote-agent unreachable);
  rendered with an orphaned-output chip.
- `output_trust_widget_gated` — an output is a widget whose live
  state is gated by widget-trust; static render-only fallback
  applies until the widget-trust axis admits execution.

Reserved `notebook_widget_trust_state` values:

- `widget_trust_denied_by_default` — the default for every widget
  instance; live JS / comm-channel binding is suppressed; a static
  render-only preview is shown.
- `widget_trust_admitted_with_preview` — the user admitted the
  widget after a preview chip; live state MAY bind through the
  notebook-kernel host world.
- `widget_trust_suppressed_by_trust` — workspace trust is
  `restricted` or `trust_revoked`; widget live binding is suppressed.
- `widget_trust_suppressed_by_policy` — admin policy narrows
  widget live binding regardless of trust.
- `widget_trust_suppressed_by_content_class` — widget content
  intersects a high-risk preview class (rich active content,
  bidi / invisible formatting reveal, confusable identifier) per
  the safe-preview detector contract.
- `widget_trust_unavailable_no_kernel` — no kernel session exists
  to back the widget's comm channel; live binding is impossible
  regardless of trust.

Rules (frozen):

1. **Four axes, never one state.** Document trust, kernel trust,
   output trust, and widget trust each resolve independently. A
   surface that collapses them into a single "notebook is trusted"
   banner is non-conforming per the runtime-taxonomy seed.
2. **Trust transitions are audited.** Every transition on every
   axis emits a `notebook_audit_event_record` with one of the
   reserved audit-event ids below.
3. **Downgrade cancels live state.** A transition from
   `document_trust_elevated_on_explicit_grant` to
   `document_trust_restricted_by_policy` or `document_trust_revoked`
   MUST:
     - cancel any running kernel session (`session_cancelled_by_trust_downgrade`);
     - transition every widget to `widget_trust_suppressed_by_trust`;
     - transition every live output to
       `output_trust_captured_from_prior_session` or
       `output_trust_replayed_from_captured_output` depending on
       whether a captured output survives.
4. **Elevated grants are not inherited across identity change.**
   A `document_trust_elevated_on_explicit_grant` grant minted under
   one identity mode MUST NOT carry across an identity-mode change
   (ADR-0001), a workspace-trust witness change, or a remote-agent
   target-identity change (ADR-0020).
5. **Widget live binding requires kernel trust.** A widget cannot
   transition to `widget_trust_admitted_with_preview` when the
   kernel-trust axis is `kernel_trust_unavailable_no_kernel`,
   `kernel_trust_restricted_by_policy`, or absent.
6. **Editability survives.** Under every combination except a
   policy-level document lock (which is out of scope for this seed),
   the document remains openable, editable, searchable, and
   reviewable. Trust narrowing gates execution and widgets; it
   never bars notebook review.

### Kernel transport and discovery

A notebook kernel is resolved, launched, observed, and killed
through typed records. Raw kernel-protocol frames, raw Jupyter
wire bytes, raw zmq frames, and raw shell-channel bytes MUST NOT
cross RPC. The host owns the kernel lifecycle; a kernel that
claims to kill the host or widen its admission through a wire
message is non-conforming.

Reserved `notebook_kernel_transport_class` values:

- `local_managed_toolchain_kernel` — a kernel launched by the
  local toolchain activator (Python venv / conda / managed
  toolchain) through ADR-0009 execution-context resolution.
- `local_provisioned_kernel` — a kernel launched through a user-
  approved kernelspec on the local host.
- `remote_agent_primary_kernel` — a kernel launched inside the
  remote-agent session per ADR-0020
  `near_code_services.notebook_kernel` placement row.
- `managed_workspace_agent_kernel` — a kernel launched by a
  managed-workspace agent per ADR-0020
  `managed_workspace_agent_only` placement row.
- `provider_side_remote_kernel` — a kernel launched inside a
  provider-side session; every capability world denies mutation
  by default per ADR-0010 unless an approval ticket is present.
- `compatibility_bridge_remote_kernel` — a kernel launched through
  a compatibility-bridge remote per ADR-0019 bridge profiles.

Reserved `notebook_kernel_session_state` values:

- `kernel_session_requested` — command-dispatch route admitted,
  trust-decision packet evaluated, execution-context root resolved;
  kernel allocation pending.
- `kernel_session_starting` — kernel process / remote session
  starting; shell / control channels negotiated (host-owned).
- `kernel_session_active` — kernel active; execution-queue
  admits new cells.
- `kernel_session_idle` — no cell execution for longer than the
  declared idle threshold; no state change.
- `kernel_session_busy` — kernel executing a cell from the
  execution queue.
- `kernel_session_interrupt_requested` — user invoked interrupt;
  SIGINT / control-channel interrupt sent.
- `kernel_session_restart_requested` — user invoked restart;
  prior outputs become `output_trust_captured_from_prior_session`;
  a fresh kernel session is allocated only on user confirmation.
- `kernel_session_cancelled_by_trust_downgrade` — document-trust
  / workspace-trust / policy narrowing revoked admission;
  in-flight executions are cancelled.
- `kernel_session_closed` — orderly close (user, policy, or
  document close).
- `kernel_session_lost_transport` — remote kernel's transport
  dropped; the reconnect window opens per ADR-0020 reconnect-
  decision record.
- `kernel_session_reconnected_same_identity` — reconnect admitted
  with `target_identity_witness_match = matched`; UI state re-
  bound; prior cells NOT replayed.
- `kernel_session_reconnected_identity_changed` — reconnect
  admitted but witness changed; fresh kernel required; prior
  outputs become `output_trust_captured_from_prior_session`.
- `kernel_session_unavailable_no_kernel` — no kernel is
  resolvable; the document remains editable and reviewable, and
  every cell carries the typed `execution_unavailable_without_kernel`
  chip per the degraded-state disclosure contract.
- `kernel_session_quarantined` — supervisor projected quarantine
  (protocol violation, resource budget exceeded, credential leak
  detected, widget-trust violation).

Reserved fields on every `notebook_kernel_session_record`:

| Field | Notes |
|---|---|
| `kernel_session_id` | Opaque, stable id (safe to log). |
| `document_id_ref` | Ref to the notebook document this kernel backs. |
| `transport_class` | One of the transport classes above. |
| `remote_agent_session_id_ref` | Non-null iff transport is a remote class. |
| `target_identity_witness_ref` | Non-null iff transport is a remote class; resolves through ADR-0020. |
| `execution_context_root_ref` | Ref to the ADR-0009 context-snapshot record. |
| `kernelspec_ref` | Ref to the kernelspec row (name, display name, language, argv); raw argv bodies do not cross. |
| `kernel_trust_state_at_launch` | One of `notebook_kernel_trust_state` values. |
| `advertised_protocol_profile_ref` | Opaque ref to the declared protocol profile. |
| `session_state` | One of `notebook_kernel_session_state` values. |
| `auto_rerun_forbidden` | Boolean; MUST be true on every transition into a reconnected / restored state. |
| `captured_at` | Monotonic timestamp. |
| `redaction_class` | Redaction posture on any payload. |

Rules (frozen):

1. **Host controls lifecycle.** The command-dispatch routes
   `notebook.kernel.launch`, `notebook.kernel.interrupt`,
   `notebook.kernel.restart`, and `notebook.kernel.kill` are the
   only paths that allocate, interrupt, restart, or terminate a
   kernel. A kernel-initiated message MUST NOT be interpreted as
   an authoritative lifecycle transition.
2. **Execution-context resolution precedes launch.** Every
   kernel launch cites an execution-context root; a kernel that
   cannot resolve its root refuses to start with
   `notebook_execution_context_root_unresolved`.
3. **No kernel, still editable.** When no kernel is resolvable,
   the document opens, renders, diffs, merges, searches, and
   exports. Only execution-dependent surfaces (run cell, restart,
   live widget) render the typed `execution_unavailable_without_kernel`
   chip. Silent disappearance is non-conforming.
4. **Remote kernels remain remote-owned.** A kernel whose
   transport is `remote_agent_primary_kernel`,
   `managed_workspace_agent_kernel`,
   `provider_side_remote_kernel`, or
   `compatibility_bridge_remote_kernel` does not downgrade to a
   local kernel on reconnect. If the remote session cannot be
   restored, the prior outputs remain as
   `output_trust_captured_from_prior_session` and the user
   chooses a fresh local session explicitly.
5. **Local-vs-remote boundary cues are visible.** A notebook
   backed by a non-local kernel MUST render a visible
   local-vs-remote boundary cue on the kernel-target indicator,
   on every run affordance, and on every widget preview.
6. **Raw wire frames never cross RPC.** Shell / iopub / control
   / stdin channel bytes, Jupyter wire-protocol frames, raw
   kernel argv bodies, and raw kernelspec JSON bodies MUST NOT
   cross RPC. Records ride as refs with redaction envelope.

### Execution-queue extension point

The execution queue is the host-owned ordered projection of cells
pending execution. Every cell admission is a typed record; the
seed reserves the extension point without fixing scheduler
internals.

Reserved `notebook_execution_queue_admission_class` values:

- `admitted_in_order` — default; the cell enters the queue at
  the tail and is dispatched in FIFO order.
- `admitted_skipped_by_policy` — admin policy denies execution
  of this cell (cell-metadata tag, language mismatch, forbidden
  magics); the cell admission emits an audit event and renders
  a typed chip.
- `admitted_deferred_no_kernel` — kernel is
  `kernel_session_unavailable_no_kernel`; the admission is
  deferred; a fresh kernel resolves the deferral only on user
  confirmation.
- `denied_by_trust` — document trust, kernel trust, or policy
  denies execution; the cell runs nothing.

Rules (frozen):

1. **Queue is host-owned.** Cell admission, ordering, and
   dispatch stay on the command-dispatch boundary.
2. **In-flight executions do not auto-replay on restore.**
   A reopened notebook MUST NOT re-enqueue or re-execute a cell
   that was mid-execution at close, at kernel crash, or at
   transport loss.

### Output-lineage classes and relationships

Every rendered output on a reopened, reconnected, or restored
notebook is one of the five classes below. The seed pins their
relationships to kernel-session identity, cell-execution
identity, and widget identity so the successor ADR, the
support-export lane, and the AI evidence-packet lane inherit
one vocabulary instead of three.

Reserved `notebook_output_lineage_class` values:

- `live_output_from_current_session` — the output was produced
  by the current kernel session's current cell-execution run;
  trust posture is `output_trust_live_from_current_session`.
- `captured_output_from_prior_session` — the output was saved
  into the notebook from a prior kernel session; trust posture
  is `output_trust_captured_from_prior_session`.
- `replayed_from_captured_output` — the surface rendered the
  captured output without re-executing the cell; trust posture
  is `output_trust_replayed_from_captured_output`; the chip
  cites `replay_from_capture_never_authoritative`.
- `orphaned_no_kernel_binding` — the output exists in the
  document but cannot be bound to a kernel session (kernelspec
  renamed, toolchain missing, remote-agent unreachable); trust
  posture is `output_trust_orphaned_no_kernel_binding`.
- `widget_gated_output` — the output is a widget; live binding
  is gated by the widget-trust axis; the static render-only
  fallback applies until widget-trust admits execution.

Rules (frozen):

1. **Every output cites a lineage class.** A notebook surface
   that renders an output without citing the lineage class is
   non-conforming.
2. **Kernel-session identity is preserved.** `live_output_*`
   and `captured_output_*` outputs carry the
   `kernel_session_id_ref` that produced them. A surface that
   elides the ref is non-conforming.
3. **Cell-execution identity is preserved.** Every output
   carries a `cell_execution_id_ref` resolving through the
   execution-queue admission that produced it.
4. **Replay never promotes captured to live.** Re-rendering a
   captured output never transitions it to `live_output_*`.
   Only a fresh kernel execution with a fresh cell_execution_id
   produces a live output.
5. **Orphaned outputs stay evidence.** An orphaned output is
   never re-executed by automation. The user chooses whether
   to execute (after picking a new kernel) or to drop the
   captured output.
6. **Widget-gated outputs degrade safely.** A widget-gated
   output renders its static preview (HTML/MIME bundle minus
   live JS / comm bindings) and cites
   `widget_trust_denied_by_default` until the user admits the
   widget through the preview chip.

### Diff, merge, and review posture

The structured-artifact review seed already reserves the cell-
aware compare viewer and the `semantic_merge_where_safe` posture
with reason `cell_and_output_identity_at_risk`. This ADR
reserves the notebook-specific output-ignore modes, metadata
filters, cell-aware merge defaults, and raw-JSON fallback
admission rules.

Reserved `notebook_output_handling_mode` values (re-exporting
and extending the review-seed preset vocabulary):

- `ignore_outputs_by_default_with_opt_in_inclusion` — default;
  diff and merge ignore every output cell; a user-opt-in
  surfaces them as evidence-only compare.
- `collapse_outputs_by_default` — outputs are collapsed under
  a typed chip; clicking expands an evidence-only preview.
- `include_outputs_with_trust_gate` — outputs are compared
  inline only when document trust is
  `document_trust_elevated_on_explicit_grant` AND workspace
  trust is `trusted`; otherwise the surface falls back to
  collapse-by-default.
- `outputs_never_merged_only_compared` — merge never mixes
  outputs from two sides; one side's outputs win verbatim or
  both sides' outputs are dropped in favour of a fresh
  execution.

Reserved `notebook_metadata_filter_preset` values (re-exported):

- `preserve_jupyter_and_aureline_namespaces_only` — default for
  compare / merge; vendor-namespaced metadata is preserved on
  save (per `survival_required_vendor_namespaces`) but hidden
  from the compare surface by default.
- `preserve_all_vendor_metadata` — the compare / merge surface
  shows every vendor namespace; required for round-trip audits.
- `preserve_none_forbidden` — reserved; NOT admitted at this
  milestone. A filter that drops all metadata breaks unknown-
  key survival and is non-conforming.

Reserved `notebook_diff_merge_posture_class` values:

- `cell_aware_semantic_compare_default` — the compare viewer
  renders cell-aware structural diffs. Default.
- `cell_aware_three_way_merge_permitted_behind_corpus` —
  structured three-way merge is permitted only behind
  `conformance-corpus:review.jupyter_notebook:v1` and the
  `round-trip-corpus:jupyter_notebook:v1`.
- `regenerate_from_canonical_source_on_conflict` — reserved
  for cases where the document is paired with a derived text
  form that is the authoritative intent; not the default for
  hand-authored notebooks.
- `raw_json_fallback_allowed` — the compare surface falls back
  to raw JSON compare ONLY when the cell-id stability class is
  `cell_id_stability_not_available_raw_json_fallback_only` OR
  the document is malformed / truncated. Raw JSON fallback is
  always visibly disclosed.

Reserved `notebook_raw_json_fallback_reason_class` values:

- `fallback_not_applicable` — default; cell-aware semantic
  compare is in force.
- `fallback_cell_id_stability_not_available` — cell ids cannot
  be resolved; raw JSON compare is permitted with a chip.
- `fallback_malformed_json_or_truncated_document` — the document
  fails JSON validation or nbformat validation; raw compare is
  permitted with a chip.
- `fallback_unsupported_nbformat_version` — the nbformat major
  version is outside the supported window; raw compare is
  permitted with a chip and a migration hint.

Rules (frozen):

1. **Cell-id stability is the merge gate.** Three-way merge is
   admitted only when every cell on both sides resolves a
   stable cell id. A merge whose identity lineage cannot be
   established falls back to compare-only.
2. **Outputs never silently merge.** The default merge posture
   NEVER mixes live outputs, captured outputs, or widget-gated
   outputs from two sides. On conflict the surface routes to
   either `outputs_never_merged_only_compared` or
   `include_outputs_with_trust_gate`; silent interleave is
   non-conforming.
3. **Unknown metadata survives merge.** A structured three-way
   merge MUST round-trip every unknown top-level, document-
   metadata, cell-metadata, and output-metadata key. Losing an
   unknown key through merge is breaking and rolls the posture
   back to `raw_json_fallback_allowed`.
4. **Raw JSON fallback is disclosed.** Any `raw_json_fallback_allowed`
   case names its `notebook_raw_json_fallback_reason_class`
   verbatim on the compare chip. Silent fallback is non-
   conforming.
5. **Paired text export is never the merge target.** A notebook
   whose paired text form conflicts with its canonical `.ipynb`
   side resolves to regeneration of the paired text form from
   the canonical source, not the other way.

### Denial-reason vocabulary

Notebook denials fail closed; a silent downgrade to a best-effort
open, a best-effort merge, or a best-effort execution is forbidden.
Every denial cites exactly one of the following.

- `notebook_document_trust_state_denies_execution`
- `notebook_kernel_trust_state_denies_launch`
- `notebook_output_trust_state_denies_render`
- `notebook_widget_trust_state_denies_live_binding`
- `notebook_execution_context_root_unresolved`
- `notebook_kernelspec_unresolved_no_kernel_available`
- `notebook_policy_pack_denies_kernel_launch`
- `notebook_policy_pack_denies_widget_rendering`
- `notebook_policy_pack_denies_output_execution`
- `notebook_remote_agent_session_unavailable`
- `notebook_remote_agent_target_identity_witness_unverifiable`
- `notebook_protocol_violation_budget_exceeded`
- `notebook_metadata_filter_preset_not_admitted`
- `notebook_output_handling_mode_not_admitted`
- `notebook_cell_id_stability_required_but_not_available`
- `notebook_merge_requires_round_trip_corpus_not_present`
- `notebook_merge_unknown_metadata_survival_failed`
- `notebook_raw_json_fallback_reason_class_required_not_present`
- `notebook_paired_text_export_promoted_to_canonical_forbidden`
- `notebook_restore_auto_rerun_forbidden`
- `notebook_raw_body_forbidden_on_boundary`

### Audit-event vocabulary

Raw notebook JSON bodies, raw cell source bytes, raw output
bytes, raw kernel-protocol frames, raw kernel argv bodies, raw
widget state bytes, and raw URLs MUST NOT appear on any audit
event. Every event is an opaque, typed id.

- `notebook_document_opened`
- `notebook_document_open_denied`
- `notebook_document_closed`
- `notebook_document_trust_state_transitioned`
- `notebook_kernel_session_requested`
- `notebook_kernel_session_launched`
- `notebook_kernel_session_launch_denied`
- `notebook_kernel_session_interrupt_requested`
- `notebook_kernel_session_restart_requested`
- `notebook_kernel_session_cancelled_by_trust_downgrade`
- `notebook_kernel_session_reconnect_same_identity`
- `notebook_kernel_session_reconnect_identity_changed`
- `notebook_kernel_session_closed`
- `notebook_kernel_session_quarantined`
- `notebook_kernel_session_unavailable_no_kernel_observed`
- `notebook_execution_queue_admission_recorded`
- `notebook_execution_queue_admission_denied`
- `notebook_output_lineage_recorded`
- `notebook_widget_trust_state_transitioned`
- `notebook_widget_live_binding_admitted_with_preview`
- `notebook_widget_live_binding_suppressed`
- `notebook_diff_surface_rendered`
- `notebook_merge_admitted_with_round_trip_corpus`
- `notebook_merge_denied_by_unknown_metadata_survival_failure`
- `notebook_raw_json_fallback_engaged`
- `notebook_paired_text_export_rendered`
- `notebook_metadata_survival_report_emitted`
- `notebook_protocol_violation_observed`

### Schema of record

- Boundary schema: `schemas/notebook/notebook_metadata_aureline.schema.json`
  (with `$defs` that also seat the four-axis trust vocabularies, the
  kernel-transport and kernel-session lifecycle, the output-lineage
  classes, the widget-trust classes, the execution-queue admission
  classes, the metadata-filter presets, the diff / merge posture
  classes, the raw-JSON fallback reason classes, the denial-reason
  set, and the audit-event id set, so one schema is the cross-tool
  boundary). Conformance fixtures live under
  `fixtures/notebook/roundtrip_cases/`.
- Kernel and trust matrix:
  `artifacts/notebook/kernels_and_trust_matrix.yaml` binds each
  kernel transport class to supported kernel trust states, to
  placement-row admission under ADR-0020, to the degraded-state
  disclosure under `kernel_session_unavailable_no_kernel`, and to
  the local-vs-remote boundary-cue posture.
- Round-trip, trust, kernel, and diff / merge worked cases:
  `fixtures/notebook/roundtrip_cases/*.ipynb` pin the canonical-
  ipynb-preserved, stable-cell-id, unknown-metadata-survival,
  paired-text-export, output-lineage, widget-gated, no-kernel-
  available, and raw-JSON-fallback scenarios the successor ADR
  MUST pass.

## Consequences

- **Frozen:** the notebook-document canonical-preservation class
  vocabulary, the cell-id stability class vocabulary, the
  metadata-survival class vocabulary, the Aureline-owned metadata
  namespace key set, the four-axis trust vocabularies (document,
  kernel, output, widget), the kernel-transport class vocabulary,
  the kernel-session lifecycle-state vocabulary, the execution-
  queue admission class vocabulary, the output-lineage class
  vocabulary, the output-handling mode vocabulary, the metadata-
  filter preset vocabulary, the diff / merge posture class
  vocabulary, the raw-JSON fallback reason class vocabulary, the
  denial-reason set, and the audit-event id set.
- **Reserved:** the authority-boundary invariants. Kernel
  lifecycle authority, document-trust authority, admin-policy
  narrowing ceiling, execution-context resolution, remote-agent
  target-identity witness, and widget-trust admission stay host-
  owned. A notebook observes and projects these; it MUST NOT
  mint them. A kernel wire message is never an authority.
- **Reserved:** the process-boundary constraints. Raw notebook
  JSON bodies, raw cell source bytes, raw output bytes, raw
  kernel-protocol frames, raw kernel argv bodies, raw widget
  state bytes, and raw URLs never cross RPC. Notebook records
  cross as typed payloads quoted by ref.
- **Reserved:** the schema-of-record posture. The JSON Schema at
  `schemas/notebook/notebook_metadata_aureline.schema.json` is
  the cross-tool boundary; the kernels-and-trust matrix at
  `artifacts/notebook/kernels_and_trust_matrix.yaml` binds
  transport classes to trust and placement; the worked fixtures
  at `fixtures/notebook/roundtrip_cases/` pin the scenarios. No
  external IDL at this milestone.
- **Permitted:** later additive-minor additions to any enumerated
  set (new output-lineage classes, new widget-trust classes, new
  kernel-transport classes, new metadata filter presets, new
  denial reasons, new audit events) with a schema / vocabulary
  bump.
- **Permitted:** admin policy packs, trust-state narrowing,
  capability-lifecycle markers, compatibility-bridge translation,
  and remote-agent placement rules MAY each narrow a notebook
  document, kernel, output, or widget further. None MAY widen.
- **Follow-up:** the successor ADR closes the open questions
  below (concrete kernel-protocol profile, widget runtime binding,
  paired-text export binding per language, supported nbformat
  window, kernelspec discovery story per platform, execution-queue
  scheduler internals, output-capture retention budget, and
  metadata-survival parity corpus) and promotes this seed's
  `Proposed` status to `Accepted`.
- **Follow-up:** the structured-artifact review seed, the
  compatibility archetype rows, the support-export bundle, the
  AI evidence-packet lane, the workspace-trust packet, the
  remote-agent service-placement row, and the mutation-journal
  restore claim each cite this ADR as the governing notebook
  contract. A lane that hides the document-trust state, the
  kernel-trust state, the output-trust state, the widget-trust
  state, the kernel-session lifecycle state, the output-lineage
  class, or the raw-JSON fallback reason on a notebook-touching
  action denies with the appropriate denial reason.
- **Ratifies:** ADR-0001 identity-mode envelope inherited,
  ADR-0004 typed RPC payload rules, ADR-0005 subscription authority
  `derived_knowledge`, ADR-0006 VFS path identity for the
  notebook path token, ADR-0007 credential-handle projection for
  secret-bearing outputs, ADR-0008 admin-policy narrowing
  ceiling, ADR-0009 execution-context resolution, ADR-0011
  capability-lifecycle markers for
  `notebook_manual_open` / `notebook_kernel_launch`, ADR-0016
  command-dispatch boundary, ADR-0018 trust-decision packet,
  ADR-0019 `notebook-observe` and `notebook-kernel` host worlds
  and budgets, ADR-0020 remote-agent session contract and
  placement-row `near_code_services.notebook_kernel`, and
  ADR-0021 terminal protocol for any shell backing a kernel.
- **Ratifies:** the structured-artifact review seed's
  `jupyter_notebook` row (`cell_aware_compare_viewer`,
  `semantic_merge_where_safe`, `cell_and_output_identity_at_risk`,
  `preserve_jupyter_and_aureline_namespaces_only`,
  `ignore_outputs_by_default_with_opt_in_inclusion`,
  `cell_id_stability_required = true`,
  `permitted_with_round_trip_corpus`,
  `conformance-corpus:review.jupyter_notebook:v1`,
  `round-trip-corpus:jupyter_notebook:v1`). This ADR does not
  re-mint those tokens; it binds them to the notebook-specific
  trust, kernel, output-lineage, and widget-trust axes.

## Alternatives considered

- **Defer notebook vocabulary until the editor lands.** Rejected:
  the structured-artifact review seed, the compatibility
  archetype rows, the runtime-taxonomy seed, the remote-agent
  placement rows, the support-export lane, and the entry-restore
  audit already reserve notebook-shaped fields. Without a typed
  seed, each would either stay free-form or be minted per-surface.
- **Collapse document trust, kernel trust, output trust, and
  widget trust into one "notebook trust" state.** Rejected: the
  runtime-taxonomy seed already names these as distinct columns;
  collapsing them would hide kernel unavailability behind
  document trust and would hide widget rendering behind output
  trust, defeating the review promise that a document stays
  editable when its kernel is unavailable.
- **Let the paired text export become canonical source when
  present.** Rejected: the `.ipynb` carries document metadata,
  per-cell metadata, output trust, widget trust, and kernel-
  session lineage that no paired text form captures. Promoting
  the paired text form to canonical source would drop unknown
  vendor metadata, drop output lineage, and drop widget-trust
  state by construction.
- **Allow widget live binding by default under
  `document_trust_inherited_from_workspace`.** Rejected: widget
  live binding composes arbitrary JavaScript, arbitrary comm-
  channel payloads, and arbitrary DOM injection. The default is
  `widget_trust_denied_by_default` regardless of document trust;
  live binding requires a preview chip admission.
- **Allow silent auto-rerun of prior cells on notebook reopen.**
  Rejected: prior outputs may reference a rotated credential, a
  narrowed policy, a replaced remote target, or a different
  execution-context root. The restore posture
  `prior_work_preserved_but_not_rerun` continues to apply; a
  reopened notebook renders captured outputs as evidence, never
  as live state.
- **Let each surface invent its own cell-id scheme.** Rejected:
  review, merge, search, AI evidence, support export, and the
  entry-restore sheet already cross-reference cells by id. A
  per-surface cell-id scheme would break every cross-reference
  and would hide unknown-metadata survival bugs behind silent
  id rewrites.
- **External IDL + codegen (Protobuf, Cap'n Proto, Smithy).**
  Rejected: same reasoning as ADR 0004 through ADR 0021 — no
  second-language consumer yet beyond the JSON Schema boundary.
  The schema export reserves a clean integration point for the
  document, cell, kernel-session, output-lineage, widget-trust,
  diff / merge, and audit records.

The `D-0027` `freeze_lane` default-if-unresolved posture would block
the notebook review lane, the notebook support-export lane, the
notebook AI-evidence lane, the notebook merge-driver lane, the
remote-agent notebook-kernel placement lane, and the compatibility
archetype `notebook_first_data_workflow` graduation from closing
the notebook contract at the first-beta milestone until a successor
ADR lands. Accepting the seed's `Proposed` status now — with its
reserved vocabulary, records, kernel / trust matrix, and worked
fixtures — avoids that freeze by giving the successor ADR concrete
records to compose against.

## Open questions

These MUST be answered by the successor ADR before this seed is
promoted to `Accepted`.

1. **Concrete kernel-protocol profile.** Which kernel protocol
   does Aureline speak at the first beta (Jupyter's ZeroMQ
   protocol, a WebSocket bridge, or a bundled crate wrapping
   both)? How does the bridge compose with the execution-context
   root and the remote-agent near-code-services placement row?
2. **Widget runtime binding.** Which widget runtime(s) are
   admitted (ipywidgets, anywidget, none), and how does their
   comm-channel binding compose with ADR-0019's `notebook-kernel`
   world budget and ADR-0016's command-dispatch boundary?
3. **Paired-text export binding per language.** Which languages
   ship a paired `.ipynb` ↔ `.py` (or `.md`) export at the first
   beta (Python first; Julia, R, Scala reserved), and what is the
   export-script form (percent-format, jupytext-format, Markdown-
   with-code-fences)? The canonical-direction ref pins
   `notebook->export_script:v1`; the successor ADR names the
   concrete form.
4. **Supported nbformat window.** Which nbformat major.minor
   versions does Aureline read / write / round-trip at the first
   beta, and which versions degrade to `fallback_unsupported_nbformat_version`?
5. **Kernelspec discovery story per platform.** How does Aureline
   discover kernelspecs on macOS, Windows, Linux, WSL, and
   managed-workspace images? Does it read `~/Library/Jupyter`,
   `%APPDATA%\jupyter`, `$XDG_DATA_HOME/jupyter`, and
   `JUPYTER_PATH` with an explicit precedence, or does it ship
   a curated allow-list?
6. **Execution-queue scheduler internals.** What are the per-
   cell timeout, memory, and cooperative-cancellation rules on
   the execution queue? How does Aureline degrade under a
   runaway cell without violating the trust ceiling?
7. **Output-capture retention budget.** What is the per-document,
   per-session, and per-workspace captured-output retention
   budget, and how does it interact with the support-bundle
   redaction class and the portable-profile capture?
8. **Metadata-survival parity corpus.** Which vendor namespaces
   ride the round-trip parity corpus (VS Code, Colab, nteract,
   JupyterLab collab), which land only on the "survival
   recommended" tier, and how is drift reported?
9. **Widget-trust preview UI copy.** What is the verbatim
   preview chip copy for admitting a widget, and how does it
   quote the widget runtime, the comm channel, and the
   kernel-session target identity?
10. **Remote-clipboard and secret-handle posture from inside a
    kernel.** A kernel may emit OSC-52 writes or may surface
    credential-handle reveals through widget output. How does
    this ADR compose with ADR-0007 (credential-handle) and
    ADR-0021 (clipboard trust) for outputs that cross both
    boundaries?

Each question blocks the `Proposed` -> `Accepted` transition and is
tracked in the `decision_history` of `D-0027`.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — ".ipynb canonical open document
  format rules" and "cell-aware diff / metadata filtering / output
  collapse" (quoted in
  `artifacts/review/structured_artifact_classes.yaml#review_rows`).
- `.t2/docs/Aureline_Technical_Design_Document.md` — "notebook
  open never autoexecutes stored output, widgets, or active
  content, and workspace trust, notebook trust, kernel
  availability, and output trust remain visibly distinct" (quoted
  in `artifacts/governance/decision_index.yaml#D-0023`).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — §20
  notebook and interactive computing architecture, Appendix AX
  notebook kernel / trust / reproducibility matrix (quoted in
  `docs/review/structured_artifact_review_seed.md#source-anchors`).
- `.t2/docs/Aureline_Milestones_Document.md` — "notebook row
  first-stable promise is a local kernel only; remote kernels and
  managed compute land on separate future rows" (quoted in
  `fixtures/compat/archetype_seed_notes/notebook_first_data_workflow.md`).

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0027`
- RFC: none (the open-question option space runs down in the
  successor ADR).
- Notebook boundary schema:
  `schemas/notebook/notebook_metadata_aureline.schema.json`
- Kernels-and-trust matrix:
  `artifacts/notebook/kernels_and_trust_matrix.yaml`
- Worked notebook round-trip fixtures:
  `fixtures/notebook/roundtrip_cases/`
- Structured-artifact review seed this contract binds to:
  `docs/review/structured_artifact_review_seed.md`
- Structured-artifact matrix this contract inherits from:
  `artifacts/review/structured_artifact_classes.yaml`
- RPC envelope this contract rides:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`
- Subscription envelope notebook views ride:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
- VFS path identity notebook-path tokens bind to:
  `docs/adr/0006-vfs-save-cache-identity.md`
- Secret-broker handle classes output-capture denies on:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
- Admin-policy narrowing ceiling this contract honours:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
- Execution-context model kernel launches bind to:
  `docs/adr/0009-execution-context-and-scope.md`
- Capability-lifecycle markers notebook rows project through:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
- Shell / command-dispatch boundary notebook entry points route
  through:
  `docs/adr/0016-shell-windowing-input-accessibility-boundary.md`
- Workspace-trust packet every open resolves:
  `docs/adr/0018-workspace-trust-and-restricted-mode.md`
- Capability-world identity scheme the `notebook-observe` /
  `notebook-kernel` worlds project through:
  `docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md`
- Remote-agent session contract remote-owned kernels bind to:
  `docs/adr/0020-remote-agent-contract.md`
- Terminal protocol any shell backing a kernel inherits:
  `docs/adr/0021-terminal-protocol-and-clipboard.md`
- Entry / restore truth audit notebook restore copy quotes:
  `docs/ux/entry_restore_truth_audit.md`
- Runtime-taxonomy seed that names notebook-trust / widget-trust
  / structured-round-trip columns:
  `docs/runtime/target_discovery_and_install_review_taxonomy.md`
- Compatibility archetype row notebook-first-data-workflow:
  `fixtures/compat/archetype_seed_notes/notebook_first_data_workflow.md`
- Affected lanes:
  `governance_lane:architecture_council`,
  `governance_lane:security_trust_review`,
  `governance_lane:compatibility_ecosystem_review`,
  `governance_lane:support_export`,
  `governance_lane:docs_public_truth`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance (as a seed at `Status: Proposed`). A successor ADR
promotes this seed to `Accepted` once the open questions are closed
and records the supersession in this section without rewriting the
body above.
