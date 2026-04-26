# Debug session, mapping quality, symbol / source-map strip, and dump-analysis truth contract

This document freezes one user-visible debug-session truth model so
launch, attach, live-shared, captured-dump, restored, and imported
debug surfaces never silently collapse into one ambiguous "Debug
session" word. Every reviewer, AI assist, support reviewer, and
release-evidence consumer reads the same record family and answers
the same five questions before trusting any frame, variable, or
source jump:

> 1. Is this a live session under our control, an inspect-only live
>    overlay, a captured dump, a restored session, or an imported
>    evidence-only session?
> 2. Are values, frames, and source-jumps based on exact-build
>    identity, an approximate (line-table-only) mapping, an
>    imported-symbol surface, or no symbols at all?
> 3. Is expression evaluation a safe read, an admitted unsafe write,
>    or held pending purity confirmation?
> 4. Does restart / rerun / reattach / reopen-dump map to a typed
>    authority, or is it silently disabled because the posture is
>    not live?
> 5. Did this session publish an irreversible side effect that locks
>    further auto-rerun?

Machine-readable companions:

- [`/schemas/execution/debug_session.schema.json`](../../schemas/execution/debug_session.schema.json)
  — the `debug_session_record` (header, posture, process / thread
  tree, frame stack, variables / watch list, evaluate / REPL
  history, symbol / source-map strip annotations) and the typed
  `debug_session_action_authority_record` projecting restart / rerun
  / reattach / reopen-dump authority.
- [`/schemas/execution/mapping_quality.schema.json`](../../schemas/execution/mapping_quality.schema.json)
  — the `mapping_quality_record` carrying the closed
  `mapping_quality_class` vocabulary (exact / approximate /
  symbol-only / unresolved / build-mismatch / imported-symbol /
  no-symbol) and the typed fallback-action class (open-source /
  open-disasm / open-imported-symbol-summary / open-no-navigable-
  target / open-redacted) every navigation surface narrows to.
- [`/schemas/execution/crash_dump_card.schema.json`](../../schemas/execution/crash_dump_card.schema.json)
  — the `crash_dump_card_record` for the captured / restored /
  imported dump artifact card (dump kind, capture posture,
  provenance, redaction posture, exception summary, and the symbol /
  source-map strip annotations frozen here).
- [`/fixtures/execution/debug_cases/`](../../fixtures/execution/debug_cases/)
  — worked YAML fixtures covering the required scenarios
  (exact-build local attach, source-map mismatch, imported symbols,
  captured minidump, shared-debug overlay with preserved core truth).

This contract composes with — and never re-mints — the surfaces it
joins to:

- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  and
  [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  — every debug session cites an execution-context root.
- [`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json)
  and
  [`/schemas/execution/attempt.schema.json`](../../schemas/execution/attempt.schema.json)
  — `debug_session_run` is the matching `run_kind_class`. A debug
  session record cites the parent run / attempt and inherits its
  queue-admission summary and host-boundary class.
- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  and
  [`/docs/debug/artifact_resolution_seed.md`](../debug/artifact_resolution_seed.md)
  — symbol / source-map / crash artifacts every frame, watch, and
  dump card resolves to. The truth contract names the *user-visible*
  posture; the artifact-resolution seed names the *resolution*
  posture; both share the same `debug_artifact_ref` per row.
- [`/schemas/execution/terminal_session.schema.json`](../../schemas/execution/terminal_session.schema.json)
  and
  [`/docs/execution/terminal_truth_contract.md`](./terminal_truth_contract.md)
  — the live-vs-restored / inspect-only language re-projects the
  same shape used for terminal sessions; renderers MUST NOT mint
  a parallel vocabulary.
- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — the build-identity axes every "exact" mapping must verify
  field-for-field.
- [`/docs/security/secret_broker_contract.md`](../security/secret_broker_contract.md)
  — credential-handle classes any evaluation / REPL / watch row
  may intersect.
- [`/docs/commands/command_dispatch_contract.md`](../commands/command_dispatch_contract.md)
  — the descriptor every restart / rerun / reattach / reopen-dump
  authority cites.

If `.t2/docs/Aureline_PRD.md`,
`.t2/docs/Aureline_Technical_Design_Document.md`, or
`.t2/docs/Aureline_UI_UX_Spec_Document.md` disagree with this
contract, those upstream documents win and this contract plus the
companion schemas update in the same change.

## Why freeze this now

The Debug surface is among the densest truth surfaces in an IDE:
one tab can bind a process tree, a multi-thread frame stack, a
variables view, a watch list, an evaluate / REPL log, a source
strip, and a dump card all at once. Without one shared truth
contract the failure modes are familiar:

- a launch session and a captured-minidump session render the same
  chrome, and the user taps "Restart" on the dump (silently no-op,
  silently restart a different process, or — worse — silently
  launch a new process from imported state);
- an imported-symbol frame from a third-party dependency renders
  exactly like a frame with full source, and a "Go to source" jump
  silently opens the wrong file or a generic decompiled stub
  without a typed posture;
- a watch expression with unknown purity is silently evaluated
  (a getter call mutates state) or silently shows "<error>" with
  no way to admit unsafe;
- a build-mismatched debug symbol (right module name, wrong build
  id) is silently used for line numbers, and the user steps to the
  "wrong line" without any indication;
- a shared-debug session lets a follower silently issue a "step"
  while the presenter believed they had inspect-only;
- a restored session paints "running" because its lifecycle state
  was preserved in the snapshot, and a rerun re-publishes a deploy
  that already shipped from the original session.

This contract makes those distinctions explicit before any DAP
adapter, breakpoint resolver, or frame-render path ships.

## Scope

Frozen at this revision:

- the `debug_session_record` header (debug-session id, parent run /
  attempt refs, host boundary, target-identity witness ref,
  command-dispatch descriptor, execution-context root, trust state,
  identity mode, admin-policy epoch);
- the `debug_posture_class` ten-value vocabulary covering launch
  (local-owns-lifetime / remote-owns-lifetime), attach (local /
  remote with witness), live-shared (inspect-only / temporary-step-
  grant), captured (dump no live control), restored (no live
  control), imported (evidence only), and the
  `debug_posture_unknown_requires_review` honesty row;
- the `stopped_state_class` vocabulary (running / stopped at
  breakpoint / step / exception / entry / pause-request / unknown);
- the process-tree / thread-tree shape (process role class, thread
  role class, frame stack);
- the closed `frame_kind_class` vocabulary (user source / generated
  source / third-party dependency / runtime or kernel / inlined /
  disasm-only / unknown);
- the `mapping_quality_class` seven-value vocabulary (exact /
  approximate / symbol-only / unresolved / build-mismatch /
  imported-symbol / no-symbol) plus the `mapping_quality_unknown_
  requires_review` honesty row, and the typed `fallback_action_
  class` vocabulary every navigation surface narrows to;
- the watch / evaluate / REPL evaluation-state vocabulary, the
  typed `evaluation_side_effect_class` (pure / local-writes / external
  calls / state mutation / purity-unknown / admitted-unsafe /
  blocked-under-purity-unknown), and the typed evaluation-purity
  rules (purity-unknown holds the row pending typed user
  confirmation; an admitted unsafe row MUST cite a command-dispatch
  descriptor);
- the symbol-strip vocabulary (full / split-resolved / split-missing
  / minimal-line-tables / fully-stripped / unknown) and the source-
  map-strip vocabulary (full / partial-names / digest-mismatch /
  absent / unknown) every frame and dump card narrows to;
- the `restart_authority_class` vocabulary (admitted under
  command-dispatch descriptor / denied no-live-control / denied
  irreversible-side-effect-published / denied dump-or-restored /
  reattach-admitted / reattach-denied target-unreachable /
  reopen-dump-admitted-inspect-only / unknown);
- the `crash_dump_card_record` shape (dump kind, capture posture,
  provenance, redaction posture, exception summary, primary-thread
  ref, symbol-strip / source-map-strip annotations,
  `debug_artifact_ref` join);
- the closed denial-reason and audit-event vocabularies on each
  record;
- `additionalProperties = false` on every record. Raw register
  state, raw memory bytes, raw stack-frame argv, raw evaluation
  values, raw watch expressions, raw REPL histories, raw command
  lines, raw absolute paths, raw URLs, raw secrets, and raw dump
  bytes MUST NOT cross any of these boundaries — records carry refs,
  digests, counts, and class labels only.

Out of scope at this revision:

- implementing concrete DAP / GDB / LLDB / V8 / Chrome / Edge /
  Node / .NET / JVM adapters or any debugger-protocol layer;
- the full debugger UI (breakpoint gutter rendering, step-over /
  step-in keybindings, exception-breakpoint UX, hover-evaluate UX,
  inline values UX);
- the conditional-breakpoint and logpoint contracts;
- the source-link / source-fetch network protocols;
- the symbol-server and source-map-server protocols (named via the
  artifact-resolution seed but the resolver implementation rides
  separate decision rows);
- the live-share / shared-debug presence and follow contracts
  (composed via the shared-control vocabulary already frozen on
  the terminal-session record).

## 1. The debug-session record

The `debug_session_record` is the per-session truth row. One debug
session opens against one execution-context root, one host
boundary, and one parent run / attempt; its lifecycle is bounded
by the parent attempt's lifecycle. A captured / restored / imported
session has no parent attempt of its own — its `parent_attempt_ref`
is null and its `debug_posture_class` MUST be one of
`debug_posture_captured_dump_no_live_control`,
`debug_posture_restored_session_no_live_control`, or
`debug_posture_imported_session_evidence_only`.

### 1.1 Header and identity

Every record carries:

- `debug_session_record_id` — opaque, stable id for one debug
  session. Debugger UI, support bundle, and release-evidence
  surfaces resolve this id field-for-field instead of minting a
  per-surface session id.
- `parent_run_ref` / `parent_attempt_ref` — refs to the parent
  `run_record` and `attempt_record`. Captured / restored / imported
  sessions null `parent_attempt_ref` and cite a
  `originating_run_ref` if the originating run is known.
- `command_dispatch_descriptor_ref` — ref to the descriptor that
  opened the session. Restart / rerun / reattach / reopen-dump
  authority resolves through this descriptor; the session record
  never mints authority by itself.
- `execution_context_root_ref`, `trust_state_ref`,
  `identity_mode_ref`, `admin_policy_epoch_ref` — context anchors.

### 1.2 Debug posture (closed vocabulary)

`debug_posture_class` is the load-bearing field. The ten-value
vocabulary is:

| Posture | When it applies |
|---|---|
| `debug_posture_launch_local_owns_lifetime` | We launched the inferior on the local host; we own its lifetime (kill / restart / detach allowed). |
| `debug_posture_launch_remote_owns_lifetime_with_witness` | We launched the inferior on a remote host / agent / container / managed runtime under a typed target-identity witness. |
| `debug_posture_attach_to_local_process` | We attached to a pre-existing local process. Detach allowed; restart MAY require user confirmation. |
| `debug_posture_attach_to_remote_process_with_witness` | We attached to a pre-existing remote process under a typed target-identity witness. |
| `debug_posture_live_shared_inspect_only` | Live shared-debug overlay; the local user can inspect frames / variables / watches but cannot step / continue / restart. |
| `debug_posture_live_shared_with_temporary_step_grant` | Live shared-debug overlay with an explicit temporary step / continue grant from the presenter. |
| `debug_posture_captured_dump_no_live_control` | Inspect-only on a captured dump (minidump / core / lightweight snapshot / kernel panic). No process is alive. |
| `debug_posture_restored_session_no_live_control` | Restored from a session snapshot; evidence-only, no live PTY or inferior. |
| `debug_posture_imported_session_evidence_only` | Imported from a support bundle or external capture. Inspect-only; never inherits live authority. |
| `debug_posture_unknown_requires_review` | Honesty row. Fails closed and forbids any mutating action. |

Every posture outside the launch / attach / live-shared families
forbids `step`, `continue`, `restart`, `pause`, and `set
breakpoint` actions — those resolve to a typed denial under the
matching `restart_authority_class` value.

### 1.3 Stopped state

`stopped_state_class` is the seven-value vocabulary
(`running_no_stopped_thread`, `stopped_at_breakpoint`,
`stopped_at_step`, `stopped_at_exception_or_signal`,
`stopped_at_entry`, `stopped_at_pause_request`,
`stopped_state_unknown_requires_review`). The contract rules:

1. A `running_no_stopped_thread` session MUST have an empty
   `frame_stack_refs` per thread (frames are not navigable while
   the thread is running).
2. A `stopped_at_exception_or_signal` session MUST cite an
   `exception_summary_ref` on the stopped thread.
3. `stopped_state_unknown_requires_review` fails closed and forbids
   step / continue / restart.
4. Captured / restored / imported sessions MUST report
   `stopped_at_*` (the snapshot's frozen state); they MUST NOT
   report `running_no_stopped_thread`.

### 1.4 Process and thread tree

The session carries `process_refs[]`. Each `debug_process_record`
carries:

- `process_role_class` (root inferior / child / attached external
  / managed worker / unknown);
- `thread_refs[]` to `debug_thread_record` rows;
- `module_summary_refs[]` to `debug_module_summary_record` rows
  pinning the per-module symbol-strip / source-map-strip class and
  build-identity linkage.

Each `debug_thread_record` carries:

- `thread_role_class` (main / worker / io / gpu-or-compute /
  unknown);
- `frame_stack_refs[]` to `debug_frame_record` rows in
  outermost-first order;
- `stopped_state_class` (per-thread); the session-level state is
  the highest-precedence non-running thread state.

Each `debug_frame_record` carries:

- `frame_kind_class` (user source / user generated source /
  third-party dependency / runtime or kernel / inlined into caller
  / disasm only / unknown);
- `mapping_quality_record_ref` — the closed mapping-quality row
  the frame resolves through;
- `module_identity_ref` — opaque ref to the
  `debug_artifact_entry_record` describing the module's symbols /
  source map;
- `source_strip_class` — the per-frame symbol-strip class
  (re-exporting the module-level value);
- `source_map_strip_class` — the per-frame source-map-strip class
  (when the frame is a JS / TS / CSS / generated frame);
- `is_inlined_into_caller_ref` — opaque ref to the parent frame
  when this frame was inlined into the caller; the renderer MUST
  preserve the inlined relationship rather than collapsing it.

### 1.5 Variables and watch list

The session carries `variables_view_ref` (refs to
`debug_variable_record` rows for the currently focused frame) and
`watch_refs` (refs to `debug_watch_record` rows that survive frame
changes).

Each variable / watch row carries:

- `evaluation_state_class` (evaluated / pending / held-pending-
  purity / blocked-under-purity-unknown / failed / redacted /
  unknown);
- `evaluation_side_effect_class` (pure / local-writes / external-
  calls / state-mutation / purity-unknown / admitted-unsafe /
  blocked / unknown);
- `value_summary_class` (literal-summary / structured-summary /
  truncated / no-value-yet / no-summary-secret-or-redacted /
  unknown). Raw values never cross this boundary; the renderer
  resolves the typed summary class.
- `credential_handle_class_refs[]` — non-empty when the value
  intersects a credential-handle class. The summary MUST resolve
  to `no_summary_secret_or_redacted` when this list is non-empty.

The contract rules:

1. A row with `evaluation_side_effect_class =
   evaluation_purity_unknown_requires_explicit_confirmation` MUST
   resolve `evaluation_state_class` to
   `watch_evaluation_held_pending_purity_confirmation`. The
   debugger UI surfaces a typed "Confirm unsafe evaluation" prompt
   (separate from the evaluate / REPL surface) before re-resolving.
2. A row with `evaluation_side_effect_class =
   evaluation_admitted_unsafe_irreversible_under_explicit_consent`
   MUST cite a `command_dispatch_descriptor_ref` on the row's
   audit log. The session record's `auto_rerun_admitted_on_this_record`
   flips to `false` once any such row resolves.
3. A captured / restored / imported session MUST NOT admit
   `evaluation_admitted_unsafe_*`; the dump has no live state to
   mutate. The typed denial is
   `debug_session_record_must_forbid_unsafe_evaluation_on_dump_or_restored_session`.

### 1.6 Evaluate / REPL log

`repl_history_refs[]` is the append-only ordered list of REPL /
Evaluate entries. Each `debug_repl_entry_record` carries:

- `repl_history_entry_class` (pure-evaluation-admitted / held-
  pending-purity / admitted-with-unsafe-consent / blocked-under-
  purity-unknown / redacted / unknown);
- `evaluation_side_effect_class` (re-export);
- `value_summary_class` (re-export);
- `command_dispatch_descriptor_ref` — required when
  `repl_history_entry_class` is
  `repl_entry_admitted_with_explicit_unsafe_consent`.

The contract rules:

1. The REPL log is append-only — replacing or rewriting a prior
   entry is a typed denial under
   `debug_session_record_must_not_admit_silent_overwrite_of_repl_history`.
2. A blocked entry MUST stay in the log with its typed class; the
   debugger UI MUST NOT silently elide it.
3. Captured / restored / imported sessions MUST NOT admit unsafe
   REPL entries (see 1.5 rule 3).

### 1.7 Symbol / source-map strip annotations

The session-level `symbol_strip_summary` block re-projects the
per-module / per-frame strip vocabulary so a reviewer can read
"this session has full symbols on three modules, split-missing on
one, fully stripped on one" without resolving every frame. The
block carries:

- `module_count_full_present`,
  `module_count_split_external_resolved`,
  `module_count_split_external_missing`,
  `module_count_minimal_line_tables`,
  `module_count_fully_stripped`,
  `module_count_unknown_requires_review`;
- `source_map_count_full_present`,
  `source_map_count_partial_names_missing`,
  `source_map_count_digest_mismatch`,
  `source_map_count_absent_no_mapping`,
  `source_map_count_unknown_requires_review`.

The strip-class vocabularies are closed (see § 2 below). Adding a
new value is additive-minor and bumps the schema version;
repurposing one is breaking.

### 1.8 Restart / rerun / reattach / reopen-dump distinctions

`action_authority_summary` is the typed projection of the
descriptor's authority for the four debug-specific actions:

- `restart_authority_class` for "Restart" (relaunch the same
  configuration in the same session);
- `rerun_authority_class` for "Rerun" (mint a new run and a fresh
  debug session under a fresh command-dispatch descriptor);
- `reattach_authority_class` for "Reattach" (re-resolve the target
  process and re-attach);
- `reopen_dump_authority_class` for "Reopen dump" (open a different
  dump under inspect-only posture).

The contract rules:

1. `debug_posture_captured_dump_no_live_control`,
   `debug_posture_restored_session_no_live_control`, and
   `debug_posture_imported_session_evidence_only` MUST resolve
   `restart_authority_class` and `reattach_authority_class` to
   `restart_denied_dump_or_restored_session_inspect_only`. The
   "Restart" affordance is hidden or rendered as a typed denial.
2. A run whose latest outcome carries
   `side_effects_irreversible_action_published` MUST resolve
   `restart_authority_class` and `rerun_authority_class` to the
   matching `*_denied_irreversible_side_effect_published` value.
3. `live_shared_inspect_only` MUST deny restart / reattach /
   step / continue. `live_shared_with_temporary_step_grant` admits
   step / continue under the temporary grant; restart / reattach
   stay denied.
4. Reopen-dump is admissible on every posture under inspect-only;
   a fresh dump opens its own debug session record.

### 1.9 Auto-rerun gate

`auto_rerun_admitted_on_this_record` MUST be `false` whenever any
of the following hold:

- `debug_posture_class` is captured / restored / imported / unknown;
- the parent run's latest outcome carries
  `side_effects_irreversible_action_published`;
- any REPL / watch row resolved
  `evaluation_admitted_unsafe_irreversible_under_explicit_consent`.

A typed user-initiated fresh run under a fresh command-dispatch
descriptor is the only path back to a new debug session.

## 2. Mapping quality and fallback actions

`mapping_quality_class` is the closed seven-value vocabulary every
frame, every "Go to source", every variable / watch source-jump,
and every dump-card frame narrows to before navigating:

| Class | Meaning | Default fallback action |
|---|---|---|
| `mapping_exact_full_source_and_symbols` | Symbols verified field-for-field against the build-identity axes; source bytes hash-match the symbol's recorded source identity. | `open_authoritative_source` |
| `mapping_approximate_lines_only_no_columns` | Line-table-only mapping (no column / no inline). Source bytes hash-match the recorded source identity but column / inline detail is absent. | `open_authoritative_source` (with the line-only chip) |
| `mapping_symbol_only_no_source_lines` | Symbol name resolved (function, file basename) but no line-table; source bytes are unknown or absent. | `open_pinned_decompiled_source` or `open_disasm_view` |
| `mapping_unresolved_no_source_no_symbols` | Neither symbols nor source resolved at this address. | `open_disasm_view` |
| `mapping_build_mismatch_resolved_to_different_build` | A candidate was found but the build-id / module-uuid / source-map-digest / commit-hash / toolchain-pin disagrees. The mapping MUST NOT be used for navigation; the resolver records the mismatch token from the artifact-resolution seed. | `open_no_navigable_target_show_address_only` |
| `mapping_imported_symbol_external_dependency_only` | Resolved through an imported / external dependency surface (third-party crate, npm module, OS framework). Source MAY be a pinned decompiled stub; the row carries an explicit "external dependency" chip. | `open_imported_symbol_summary` |
| `mapping_no_symbol_unmapped_address_disasm_fallback` | No symbol record found; only an address. | `open_disasm_view` |
| `mapping_quality_unknown_requires_review` | Honesty row. Fails closed and forbids navigation. | `open_unknown_requires_review` |

`fallback_action_class` is closed:

- `open_authoritative_source` — open the file the symbol record
  pinned, at the resolved line / column;
- `open_pinned_decompiled_source` — open a pinned decompiled stub
  (e.g. a shipped `.d.ts` shim, a generated header, or a
  decompiler artifact);
- `open_imported_symbol_summary` — open an inspect-only summary
  card naming the dependency, the pinned version, and the symbol
  identity (without source bytes);
- `open_disasm_view` — open a disassembly / instruction-stream
  view at the address;
- `open_no_navigable_target_show_address_only` — render the
  address inline with a typed "no navigable target" chip; no jump;
- `open_redacted_under_redaction_class` — render the row under a
  redaction class that hides any source location;
- `open_unknown_requires_review` — fail closed; show only the
  honesty row.

The contract rules:

1. A frame whose `mapping_quality_class` is
   `mapping_build_mismatch_resolved_to_different_build` MUST NOT
   silently navigate. The renderer surfaces the mismatch token
   from the joined `debug_artifact_entry_record` (e.g.
   `mismatch_module_uuid`, `mismatch_source_map_digest`,
   `mismatch_commit_hash`) verbatim.
2. The mapping-quality row MUST cite the same
   `debug_artifact_ref` the artifact-resolution seed resolves; the
   debugger UI, support-bundle anchor, and release artifact-graph
   node read the same row token-for-token.
3. A mapping-quality row whose
   `evidence_state_class = evidence_pending_resolver_walk` is
   admissible only while a resolver walk is in flight. The
   resolution result becomes the terminal class within one
   resolver-walk SLO (named in the artifact-resolution seed).
4. The renderer MUST NOT collapse `mapping_imported_symbol_*` into
   `mapping_exact_*` even when the imported symbol's pinned
   decompiled source happens to be available.

## 3. The crash-dump card

`crash_dump_card_record` is the inspect-only artifact card a
captured / restored / imported dump renders alongside its debug
session record. It pins the user-visible truth a reviewer needs
before reading any frame on a dump:

- `dump_kind_class` (minidump / core / lightweight snapshot /
  kernel panic / unknown);
- `dump_capture_posture_class` (captured-local-no-upload /
  captured-local-with-consent-upload / imported-from-support-
  bundle / imported-from-remote-capture / unknown);
- `dump_provenance_class` (workspace-local / managed-workspace /
  remote-target / user-supplied side-loaded / unknown);
- `dump_redaction_posture_class` (metadata-only / summary-only-no-
  body / body-internal-support-only / body-evidence-packet-only /
  unknown);
- `exception_summary` block — `exception_class_label`,
  `signal_class_label`, `fault_address_kind_class`,
  `primary_thread_ref`, `frame_count_observed_summary`. Raw
  exception messages, raw signal payloads, raw fault addresses,
  and raw register state never appear here;
- `module_strip_summary` — re-export of the symbol-strip /
  source-map-strip vocabulary for every module observed on the
  dump;
- `debug_artifact_ref` — same id the resolver uses for this dump's
  row in the `debug_artifact_manifest_record`;
- `surface_linkage` — `debug_session_record_ref`,
  `support_bundle_anchor_ref`,
  `release_artifact_graph_node_ref`,
  `crash_envelope_ref`,
  `symbolication_report_ref`.

The contract rules:

1. A dump card MUST NOT carry `auto_rerun_admitted_on_this_record =
   true`; the parent debug session inherits the captured-dump
   posture and its own auto-rerun flag is locked false.
2. A dump card whose `dump_capture_posture_class` is
   `dump_captured_local_no_upload` MUST resolve
   `dump_redaction_posture_class` to one of `metadata_only` or
   `summary_only_no_body`. Raw bodies never travel under
   `metadata_only`.
3. A dump card whose `dump_provenance_class` is
   `provenance_user_supplied_side_loaded` MUST cite an explicit
   consent ref on the parent debug session's audit log and MUST
   render under the `imported` posture.
4. A dump card MUST resolve `module_strip_summary` against the
   joined `debug_artifact_entry_record`; the strip-class counts
   are not re-derived on the card.

## 4. Parity rules

The debug-session record, the mapping-quality record, the crash-
dump card record, the joined `debug_artifact_entry_record`, the
support-bundle manifest row, and the release artifact-graph node
all carry the same `debug_artifact_ref` for one logical artifact
and the same `debug_session_record_id` for one logical session.
Surface chrome may reorder copy but MAY NOT mint a parallel id /
badge / label / status string.

| Surface | What it resolves |
|---|---|
| Debugger UI | `debug_session_record` (header + posture), each frame's `mapping_quality_record_ref`, the dump card if present, the variables / watch / REPL rails. |
| Support bundle | `debug_session_record` summary, all `debug_artifact_ref` rows (under `included_metadata_only` by default), the dump card under its declared posture. |
| Release evidence | The `debug_artifact_ref` rows that resolve through the release artifact-graph node (built / split debug symbols, source-map bundle, crash-symbols archive, support runbook bundle, reproducibility pack); the debug-session record itself does not appear on release-public surfaces. |
| Replay / import probes | `debug_posture_class = debug_posture_imported_session_evidence_only`; every action authority resolves to its `*_denied_*` class. |

## 5. Invariants the contract guarantees

The eight invariants the contract guarantees on the debug-session
rail:

| Invariant | Where it lives | Failure mode when missing |
|---|---|---|
| Captured / restored / imported sessions never paint as live | `debug_posture_class` MUST be one of the captured / restored / imported values; `restart_authority_class` and `reattach_authority_class` MUST be `*_denied_dump_or_restored_session_inspect_only`; the typed denial is `debug_session_record_must_not_paint_dump_or_restored_session_as_live` | A user taps "Restart" on a dump, expecting to relaunch the dumped process |
| Build-mismatched mappings never silently navigate | `mapping_quality_class = mapping_build_mismatch_resolved_to_different_build` MUST resolve `fallback_action_class = open_no_navigable_target_show_address_only`; the typed denial is `mapping_quality_record_must_not_navigate_under_build_mismatch` | A user steps to "the wrong line" because line-table addresses happen to overlap |
| Imported-symbol frames never collapse into exact-source frames | `mapping_quality_class = mapping_imported_symbol_external_dependency_only` cannot be elided into `mapping_exact_*`; the typed denial is `mapping_quality_record_must_not_collapse_imported_symbol_into_exact` | A reviewer assumes a third-party frame is mapped to local source bytes |
| Purity-unknown evaluation cannot silently mutate state | `evaluation_side_effect_class = evaluation_purity_unknown_requires_explicit_confirmation` holds the row; `evaluation_admitted_unsafe_*` requires a command-dispatch descriptor; the typed denial is `debug_session_record_must_hold_evaluation_pending_purity_confirmation_when_unknown` | A watch evaluation calls a getter that mutates state without the user knowing |
| Restart / rerun / reattach / reopen-dump are typed authorities, not implied | `restart_authority_class`, `rerun_authority_class`, `reattach_authority_class`, `reopen_dump_authority_class` are closed enums; every value outside `*_admitted_*` MUST cite a denial reason; the typed denial is `debug_session_record_must_cite_action_authority_class_per_action` | A reviewer cannot tell why a "Restart" affordance is greyed out |
| Auto-rerun is forbidden after irreversible side effects or unsafe evaluations | `auto_rerun_admitted_on_this_record = false` whenever any unsafe REPL / watch row resolved or the parent run published an irreversible side effect; the typed denial is `debug_session_record_must_narrow_auto_rerun_when_irreversible_or_unsafe_evaluated` | An auto-rerun re-publishes a deploy or re-issues an irreversible mutation |
| Shared-debug never silently grants step authority | `debug_posture_live_shared_inspect_only` MUST deny step / continue / restart; `debug_posture_live_shared_with_temporary_step_grant` MUST cite a temporary-grant ref on the parent shared-control metadata; the typed denial is `debug_session_record_must_cite_temporary_step_grant_ref_when_shared` | A follower silently steps a session the presenter believed they had inspect-only on |
| The REPL / watch / variables history is append-only | Every row carries its own opaque id; replacing a prior row is a typed denial under `debug_session_record_must_not_admit_silent_overwrite_of_repl_history` | A "clear" action silently destroys evidence of a prior unsafe evaluation |

## 6. Out-of-band attestations and audit events

Every record family carries a closed `audit_event_id` vocabulary
so a reviewer can read the chronology without raw bodies.

The debug-session family covers debug-session published / revoked
/ posture-transitioned / stopped-state-transitioned / process-tree-
attached / variables-view-resolved / watch-row-attached / watch-
row-evaluated / watch-row-held-pending-purity / watch-row-admitted-
unsafe / repl-entry-attached / repl-entry-blocked-under-purity /
repl-entry-admitted-unsafe / restart-admitted / restart-denied /
rerun-admitted / rerun-denied / reattach-admitted / reattach-denied
/ reopen-dump-admitted / silent-paint-as-live-forbidden-denial /
silent-overwrite-of-repl-history-forbidden-denial / silent-collapse-
of-imported-symbol-forbidden-denial / silent-navigate-under-build-
mismatch-forbidden-denial / silent-step-without-grant-forbidden-
denial / audit-denial-emitted.

The mapping-quality family covers mapping-quality-row published /
quality-class-narrowed / fallback-action-narrowed /
silent-collapse-of-imported-symbol-forbidden-denial /
silent-navigate-under-build-mismatch-forbidden-denial /
audit-denial-emitted.

The crash-dump-card family covers crash-dump-card published /
posture-transitioned / redaction-posture-narrowed /
redaction-posture-broadened-with-approval-ticket /
silent-paint-dump-as-live-forbidden-denial /
silent-admit-restart-on-dump-forbidden-denial /
audit-denial-emitted.

Raw register state, raw memory bytes, raw stack-frame argv, raw
evaluation values, raw watch expressions, raw REPL histories, raw
command lines, raw env bodies, raw API request / response bodies,
raw absolute paths, raw URLs, raw secret values, raw exception
messages, and raw stack traces MUST NOT appear on any audit event.

## 7. Versioning

Each schema declares its own `*_schema_version` const at value
`1`:

- `debug_session.schema.json` — `debug_session_schema_version = 1`;
- `mapping_quality.schema.json` — `mapping_quality_schema_version = 1`;
- `crash_dump_card.schema.json` — `crash_dump_card_schema_version = 1`.

Adding a new enum member, a new optional field, or a new sub-
record kind is additive-minor and bumps the matching version
const. Repurposing an existing enum member or removing one is
breaking and requires a new decision row plus companion updates
to the artifact-resolution seed and the run / attempt / artifact-
event schemas. Adding a new schema file under
`schemas/execution/` is additive-minor under the execution-family
rules in
[`/artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml).

## 8. Source anchors

- `.t2/docs/Aureline_PRD.md` §5.41, §5.43, §5.44 — Support Center
  scope, debugger artifact identity, and crash artifact / source-
  map mismatch disclosure;
- `.t2/docs/Aureline_PRD.md` §10.13, §10.15, §10.22 —
  supportability, redaction, and field-diagnostics posture every
  debug-session export inherits;
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §24.2
  and §24.4 — supportability plane and evidence composition rules;
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §23.66 and §23.67 —
  debugger UI must name build-id mismatches, stale source maps,
  and unavailable debug data with stable labels, not generic
  failures.

If a future revision of those documents disagrees with this
contract, the upstream documents win and the contract plus the
companion schemas update in the same change.
