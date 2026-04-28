# Output, log, and result-viewer contract

This document is the shared UX contract for output viewers, log viewers,
result grids, and artifact previews. It exists so every surface that
renders task output, shell output, logs, structured result rows,
captured artifacts, diagnostic streams, notebook cell outputs, test
output, or build output adopts one honesty model for size, truncation,
autoscroll/freeze, active-content trust, source identity, freshness, and
export path instead of inventing local size badges, local freeze
wording, or local "truncated" labels that hide provenance, buffering,
or sandbox state.

The contract is normative. Where this document disagrees with the
source UI / UX spec or a frozen upstream contract it quotes, the upstream
wins and this document MUST be updated in the same change. Where this
document disagrees with a downstream surface's private output / log /
result wording, this document wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ux/output_viewer_object.schema.json`](../../schemas/ux/output_viewer_object.schema.json)
  - boundary schema every non-owning surface reads.
- [`/fixtures/ux/output_viewer_cases/`](../../fixtures/ux/output_viewer_cases/)
  - worked fixtures covering live autoscroll-frozen tails, snapshot
  replay, provider-owned retention truncation, blocked active content,
  large-artifact open-in-detail, and imported-artifact read-only review.

## Composition, not redefinition

This contract rides alongside - it does not re-mint - the vocabularies
already frozen in:

- [`/docs/ux/live_update_review_contract.md`](./live_update_review_contract.md)
  and [`/schemas/ux/live_set_state.schema.json`](../../schemas/ux/live_set_state.schema.json)
  - review-control state, delivery state, authority-limit state,
  buffered-change indicator, anchor status, batch-membership honesty,
  follow/autoscroll posture, provider-owned limitations, schema drift,
  truncation, and copy/export scope. Every output viewer whose origin
  is a live stream MUST point at one emitted `live_set_state_record`
  via `live_set_state_ref` and MUST NOT redefine any of that state.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  and [`/schemas/ux/view_freshness.schema.json`](../../schemas/ux/view_freshness.schema.json)
  - cross-surface freshness badge, materialized-view disclosure,
  captured-versus-live scope truth, and export-safe rules. Output
  viewers keep origin, size, trust, and truncation here, but any
  cross-surface "live exact", "snapshot", "partial", "stale", or
  "approximate" badge resolves through `view_freshness_record`.
- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  - safe-preview trust-class, connectivity-state, and the
  safe-preview downgrade-trigger ladder. Every output viewer MUST
  project its trust posture from that schema.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  - execution-context record. Every output viewer whose source is a
  runnable (shell command, task, kernel cell, test run, build job,
  debug session) MUST cite its `execution_context_record_ref` rather
  than mint a local run-identity field.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  - representation-labeled copy / export and deny-closed behavior when
  scope honesty is lost. Every output viewer MUST follow the same
  representation-labeled copy / export posture.
- [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  and [`/schemas/preview/preview_snapshot.schema.json`](../../schemas/preview/preview_snapshot.schema.json)
  - preview-runtime mapping confidence, stale-editability, source-sync
  state, and downgrade triggers. Artifact-preview viewers that carry
  a preview snapshot MUST cite the preview snapshot by reference and
  MUST NOT restate its vocabulary.
- [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  - `artifact_origin_class`, `provenance_state`, and
  `default_edit_posture` for imported or generated artifacts rendered
  in the viewer.
- [`/docs/ux/transient_surface_contract.md`](./transient_surface_contract.md)
  - tooltip / hovercard / peek rules; detail-panel promotion from an
  output viewer promotes into a transient or durable surface that
  contract already governs.

## Who reads this document

- **Surface authors** implementing task output panes, shell output
  scrollback, log viewers, structured result grids, notebook cell
  output areas, test runners, build dashboards, diagnostic streams,
  and artifact preview surfaces.
- **Product writers** choosing copy for size disclosure, "Showing last
  N lines", freeze / autoscroll controls, textual fallback labels,
  and export scope labels.
- **Support and parity-audit tooling** that needs one machine-readable
  packet explaining what output the user was reviewing, what was
  excluded, what was blocked for trust reasons, and what left the
  product on copy / export.

## One contract, five viewer families, one object

The contract applies uniformly to the viewer families below. A viewer
that mints a private size vocabulary, private "truncated" label, private
autoscroll-freeze control, private active-content block reason, or
private export-scope story is non-conforming.

| Viewer family | Typical examples | Review risk to control |
|---|---|---|
| `output_viewer` | task output pane, shell output scrollback, debug console, extension output channel | hidden buffering, silent shell freeze on burst, missing size disclosure |
| `log_viewer` | container logs, pipeline logs, remote log tail, persistent service log | autoscroll jumping underneath the reader, provider retention boundaries, middle elision |
| `result_grid_viewer` | SQL results, variable explorer, structured query rows, data-tool grids | row sampling, approximate totals, schema drift, typed export scope |
| `artifact_preview_viewer` | binary artifact preview, image or media preview, captured-output preview, notebook cell output, test artifact | size overflow, blocked active content, unknown or stale lineage, representation-labeled copy |
| `diagnostic_viewer` | compiler diagnostics, linter streams, type-check results, analysis streams | rate-limited producer, stale-after-re-edit, per-run provenance mixed with prior runs |

Every viewer above - and every future viewer that inherits the same
review risk - emits one `output_viewer_object_record` whenever it needs
to explain its output posture across RPC, support export, companion
surfaces, or durable evidence.

## Core model

The contract freezes seven orthogonal posture axes plus the composition
reference that makes live streams honest:

1. **Output class**: what the viewer is rendering
   (`stdout_stream`, `stderr_stream`, `combined_stdio_stream`,
   `structured_result_rows`, `structured_log_segments`,
   `diagnostic_message_stream`, `test_result_stream`,
   `build_output_stream`, `cell_output_stream`,
   `binary_artifact_preview`, `media_artifact_preview`).
2. **Size bucket**: the viewer's disclosed size class
   (`tiny`, `small`, `medium`, `large`, `very_large`,
   `unbounded_stream`).
3. **Viewer mode**: how the output is currently presented
   (`inline`, `virtualized`, `open_detail`,
   `blocked_active_content`, `textual_fallback`,
   `snapshot_review`).
4. **Origin class**: where the viewer is reading from
   (`live_stream`, `cached_playback`, `imported_artifact`,
   `snapshot_capture`).
5. **Trust posture**: trust class re-exported from the safe-preview
   contract plus an active-content policy class the viewer MUST emit
   whenever it renders markup or executable-capable content.
6. **Freshness**: re-exported freshness class plus a typed
   disclosure label.
7. **Export path**: representation class, copy scope class, export scope
   class, and buffered-change visibility re-exported from the live-set
   contract.

The important rule is separation:

- an **unbounded stream** is not silently rendered as a **large**
  buffer;
- a **blocked active-content** viewer is not silently renamed to
  "cached";
- a **cached** or **imported** viewer is not allowed to imply live
  continuity;
- **live stream** posture is delegated to `live_set_state_record` and
  MUST NOT be restated locally;
- **shell-freeze on burst** is never acceptable; a viewer that cannot
  keep up MUST switch to `virtualized`, `open_detail`, or
  `textual_fallback` and disclose the switch.

## Frozen axes

### Output class

Closed vocabulary:

- `stdout_stream`
- `stderr_stream`
- `combined_stdio_stream`
- `structured_result_rows`
- `structured_log_segments`
- `diagnostic_message_stream`
- `test_result_stream`
- `build_output_stream`
- `cell_output_stream`
- `binary_artifact_preview`
- `media_artifact_preview`

Rules:

1. The output class labels what the producer emits, not how the viewer
   chose to render it. A JSON-lines stream rendered as a grid remains
   `structured_log_segments` at the producer; the viewer renames
   representation via `representation_class` on the copy/export
   posture, not by reclassifying the output.
2. Binary and media artifact previews MUST NOT render as `inline`
   unless the viewer has a bounded, trust-cleared renderer for that
   content type.

### Size bucket

Closed vocabulary:

- `tiny`
- `small`
- `medium`
- `large`
- `very_large`
- `unbounded_stream`

Rules:

1. The size bucket is a UX-visible class, not a byte count. Surfaces
   MUST also carry byte-count and line/row-count truths in the size
   disclosure block.
2. `unbounded_stream` is reserved for producers without a declared
   end-of-stream (tailed logs, long-running pipelines, interactive
   shells). An unbounded stream MUST default to `virtualized` and
   MUST never materialize an `inline` full-body layout.
3. `very_large` MAY be materialized inline only when virtualization is
   active; otherwise the viewer MUST offer `open_detail` or a
   `textual_fallback` before full-body rendering.

### Viewer mode

Closed vocabulary:

- `inline`
- `virtualized`
- `open_detail`
- `blocked_active_content`
- `textual_fallback`
- `snapshot_review`

Rules:

1. `inline` means the full current content is rendered directly in the
   host surface. A viewer MUST NOT select `inline` for sizes that would
   force re-layout on the main thread or block the host shell.
2. `virtualized` means only the visible window materializes; the
   viewer is reviewable via scroll and search without holding the
   full body in view.
3. `open_detail` means the current content is summarized in-surface and
   promoted to a dedicated panel, editor, or transient surface for
   full review. The promotion MUST preserve source identity and
   freshness cues (see `transient_surface_contract.md`).
4. `blocked_active_content` means the viewer is withholding
   executable-capable content (HTML with scripts, SVG with active
   handlers, embedded iframes, autoplaying media, or inert but
   untrusted widgets) and is rendering a placeholder. Blocked active
   content MUST offer a typed textual fallback.
5. `textual_fallback` means the viewer is rendering a plain-text or
   sanitized representation instead of the rich rendering (for
   accessibility, blocked active content, policy, or representation).
6. `snapshot_review` means the viewer is bound to an immutable basis
   (captured output, imported artifact, exported snapshot, replayed
   log slice). Snapshot review MUST NOT silently resume live updates.

### Origin class

Closed vocabulary:

- `live_stream`
- `cached_playback`
- `imported_artifact`
- `snapshot_capture`

Rules:

1. `live_stream` is the only origin allowed to claim current
   continuity. A live-stream viewer MUST carry a non-null
   `live_set_state_ref` and delegate delivery / buffering / anchor /
   batch / follow / truncation / provider-limit honesty to the
   referenced `live_set_state_record`.
2. `cached_playback` is reserved for output that was live but is now
   being replayed from a local cache or captured tail. Cached
   playback MUST NOT present itself as live. The cache's capture
   time MUST travel on the static basis.
3. `imported_artifact` is reserved for output brought in from a
   prior session, a support bundle, a share sheet, or an external
   download. Imported artifacts MUST cite
   `artifact_origin_class` and `provenance_state` from the
   generated-artifact edit-posture schema.
4. `snapshot_capture` is reserved for an immutable capture pinned at a
   named basis (run completion capture, incident export, signed
   evidence capture). Snapshot captures MUST carry a captured-at
   timestamp and MUST be read-only on copy / export.

### Trust posture

- `trust_class` is re-exported from
  `/schemas/security/trust_class.schema.json`: `RawText`,
  `SanitizedRich`, `TrustedLocalActive`, `IsolatedRemoteActive`.
- `sandbox_posture_class` is a closed vocabulary:
  `not_applicable_no_active_content`, `sandbox_isolated`,
  `sandbox_restricted`, `sandbox_unavailable`,
  `sandbox_not_required_trusted_local`.
- `active_content_policy_class` is a closed vocabulary:
  `no_active_content_present`, `active_content_allowed`,
  `active_content_blocked_trust`,
  `active_content_blocked_policy`,
  `active_content_blocked_sandbox`,
  `active_content_blocked_representation`.

Rules:

1. Active-content blocking is always an explicit, typed block. A viewer
   MUST NOT drop a `<script>`, an autoplaying video, or an active
   SVG handler silently; the block reason MUST appear via
   `active_content_policy_class` and a reviewer-visible label.
2. `trust_class = IsolatedRemoteActive` requires
   `sandbox_posture_class` in `{sandbox_isolated, sandbox_restricted,
   sandbox_unavailable}`.
3. `trust_class = RawText` requires `sandbox_posture_class =
   not_applicable_no_active_content` and
   `active_content_policy_class in {no_active_content_present,
   active_content_blocked_representation}`.
4. Trust posture MUST be consistent with the notebook cell output
   posture, the preview runtime trust posture, and the terminal output
   trust posture. A viewer that renders the same artifact in two places
   MUST NOT present two different trust classes for the same
   `source_identity_ref + execution_context_record_ref` tuple.

### Source identity and run or kernel ref

Every viewer object names:

- `source_identity` (`source_identity_class` + `source_ref` +
  `source_label`) drawn from the closed vocabulary:
  - `shell_command_invocation`
  - `task_run`
  - `kernel_cell`
  - `test_run`
  - `build_pipeline`
  - `debug_session`
  - `container_log`
  - `remote_process`
  - `diagnostic_producer`
  - `imported_artifact`
  - `snapshot_source`
- an optional `run_or_kernel_ref` (`kind` +
  `ref` + `label`) with `kind` drawn from:
  - `task_run`, `kernel_session`, `test_run`, `build_run`,
  `debug_session`, `shell_session`, `remote_process_session`,
  `not_applicable`.

Rules:

1. A runnable output MUST cite both `source_identity` and
   `run_or_kernel_ref`. Imported or snapshot origins MAY leave
   `run_or_kernel_ref.kind = not_applicable` but MUST cite the
   original run identity, if known, on the static basis.
2. Source identity MUST be stable across viewer refreshes within the
   same run. Re-binding to a new run requires a new
   `output_viewer_object_record`.

### Freshness

- `freshness_class` is re-exported from the capability-lifecycle and
  safe-preview contracts: `authoritative_live`, `warm_cached`,
  `degraded_cached`, `stale`, `unverified`.
- Viewers that are `cached_playback`, `imported_artifact`, or
  `snapshot_capture` MUST NOT claim `authoritative_live`.
- Viewers with origin `live_stream` AND live delivery (via the
  referenced `live_set_state_record`) MAY claim `authoritative_live`;
  otherwise the freshness MUST drop to a non-live class.

### Size disclosure

Every record carries a `size_disclosure` block:

- `size_bucket` (above);
- `byte_count` (`exact` / `approximate` / `unknown` + value);
- `line_or_row_count` (`exact` / `approximate` / `unknown` + value);
- `disclosed_label` (reviewer-visible summary);
- `large_output_action_class`: closed vocabulary
  `no_action_required`, `materialize_virtualized`,
  `open_in_detail_panel`, `fetch_on_demand`,
  `export_only_no_inline_rendering`,
  `deny_inline_size_over_budget`.

Rules:

1. A viewer whose size bucket is `large`, `very_large`, or
   `unbounded_stream` MUST NOT select `no_action_required`.
2. `deny_inline_size_over_budget` is the correct posture when inline
   rendering would freeze the host shell. It MUST be accompanied by a
   typed `open_in_detail_panel` or `export_only_no_inline_rendering`
   follow-up.
3. Size disclosure MUST remain visible regardless of the current
   viewer_mode. A user who opens the detail panel still sees the same
   size bucket, byte count, and line / row count.

### Truncation

Every record carries a `truncation` block:

- `owner`: `none`, `client`, `provider`;
- `state`: `none`, `head_truncated`, `tail_truncated`,
  `middle_elided`, `range_trimmed`;
- `summary_label`;
- `truncation_reason`: `not_applicable`, `size_limit_client`,
  `size_limit_provider`, `retention_window_provider`,
  `rate_limit_provider`, `redaction_applied`, `policy_block`,
  `representation_forced_textual`.

Rules:

1. Truncation MUST say whether it was imposed by the client or the
   upstream provider. Silent truncation is non-conforming.
2. Truncation that removes content before the user's current anchor
   (`head_truncated` or `middle_elided`) MUST, for `live_stream`
   origins, align with the `windowing_or_truncation` block on the
   referenced `live_set_state_record`.
3. `redaction_applied` is the correct reason when the omitted content
   was dropped by a redaction policy; it MUST NOT be flattened into
   `size_limit_client`.

### Control contract

Every record carries a `control_contract` block with typed sub-blocks:

- `search`:
  - `available` (bool);
  - `search_scope_class`:
    `visible_only`, `loaded_window`, `full_source`,
    `not_applicable`.
- `bookmark`:
  - `available` (bool);
  - `bookmark_scope_class`:
    `viewer_line`, `source_line`, `anchor_ref_based`,
    `not_applicable`.
- `copy`:
  - `available` (bool);
  - `copy_scope_class` (re-exported from the live-set contract).
- `export`:
  - `available` (bool);
  - `export_scope_class` (re-exported from the live-set contract).
- `freeze_autoscroll`:
  - `available` (bool);
  - `current_follow_mode`: `follow_latest`, `manual_scroll`,
    `frozen`, `not_applicable` (aligned with the live-set follow
    mode).
- `jump_to_latest`:
  - `available` (bool);
  - `jump_action_class`: `show_new_results`, `jump_to_latest`,
    `resume_live`, `not_applicable` (aligned with the live-set
    jump-action vocabulary).
- `textual_fallback`:
  - `available` (bool);
  - `textual_fallback_class`: `not_offered`,
    `offered_for_accessibility`,
    `offered_for_blocked_active_content`,
    `offered_for_size_over_budget`,
    `offered_for_policy`,
    `required_by_policy`.
- `large_output_disclosure`:
  - `representation_class` (re-exported);
  - `disclosed_label`;
  - `buffered_change_visibility` (re-exported).

Rules:

1. `search` and `bookmark` MUST declare their scope class; a search
   that only scans the visible window MUST NOT be labeled "search
   everything".
2. `freeze_autoscroll` and `jump_to_latest` MUST agree with the
   referenced `live_set_state_record` follow / jump posture. A local
   freeze control that does not set `current_follow_mode = frozen`
   on the referenced live-set record is non-conforming.
3. A viewer in `viewer_mode = blocked_active_content` MUST carry
   `textual_fallback.available = true` and
   `textual_fallback_class != not_offered`.
4. A viewer in `viewer_mode = textual_fallback` MUST carry
   `textual_fallback.available = true`.
5. Copy and export MUST remain representation-labeled per the
   shell-interaction-safety contract. Buffered-change visibility on a
   live-stream origin MUST mirror the referenced
   `live_set_state_record` value.

### Policy context and redaction

- `policy_context`: `identity_mode`, `policy_epoch`, `trust_state`,
  optional `execution_context_id` (aligned with the live-set
  schema).
- `redaction_class` is re-exported and MUST match the redaction class
  of any companion live-set record for the same source identity.

## Cross-surface review rules

1. **No shell freeze.** A viewer that cannot render inline within the
   host-shell time budget MUST switch mode (`virtualized`,
   `open_detail`, `textual_fallback`, or
   `deny_inline_size_over_budget`) and disclose the switch. Silent
   spinners, blocking re-layout, or frozen shells are non-conforming.
2. **No hidden truncation.** Client- and provider-owned truncation
   MUST be labeled with a typed reason. A viewer that drops bytes
   silently to stay inside a budget is non-conforming.
3. **No silent active-content drop.** Any active content that is not
   rendered MUST surface a typed `active_content_policy_class` plus a
   reviewer-visible label. A viewer that strips `<script>` or muted a
   media element without disclosure is non-conforming.
4. **Live stream honesty is delegated.** Output viewers MUST NOT
   re-mint `delivery_state`, `authority_limit_state`,
   `update_buffer`, `anchor`, `batch_membership`, `follow_control`,
   `provider_limitations`, `schema_drift`, or
   `windowing_or_truncation`. A live-stream viewer points at one
   `live_set_state_record` and projects its freshness cues from that
   record.
5. **Cached, imported, and snapshot origins are first-class.** Captured
   output, support-bundle imports, and named snapshots MAY be fully
   reviewable while not live. Their static basis, capture time, and
   export posture MUST remain explicit.
6. **Trust consistency with preview, notebook, and terminal.** The
   trust class the viewer renders MUST match the trust class the
   notebook cell output, preview runtime, or terminal scrollback
   renders for the same source. Trust divergence between surfaces
   for the same source is non-conforming.

## Surface guidance

| Viewer family | Default mode | Size policy | Required honesty cues |
|---|---|---|---|
| `output_viewer` / shell output | `virtualized` for unbounded, `inline` for tiny / small | switch to `open_detail` at `very_large`; deny inline above budget | source identity, run/kernel ref, size bucket, byte and line counts, truncation owner, follow mode from live-set ref |
| `log_viewer` | `virtualized` with `follow_latest` while in the live tail | provider retention and rate-limit surfaces are provider-owned truncation | live-set ref, truncation owner, export scope, copy representation class |
| `result_grid_viewer` | `virtualized` with anchored selection | schema drift and approximate totals route through the live-set ref | live-set ref, anchored selection, export scope, schema-drift summary |
| `artifact_preview_viewer` | `inline` only when bounded and trust-cleared; otherwise `open_detail`, `blocked_active_content`, or `textual_fallback` | `very_large` opens in detail; binary previews require a trust-cleared renderer or fall back to textual | trust class, active-content policy class, sandbox posture, artifact-origin class, provenance state |
| `diagnostic_viewer` | `virtualized` with `manual_scroll` when rate-limited | rate-limited producers route through the live-set provider-limit axis | source identity (diagnostic producer ref), execution context ref, freshness class |

## Source anchors

- TAD output / log virtualization rules.
- UI / UX Spec task-output, shell-output, logs, results-grid, and
  notebook-cell-output rules.
- UI / UX Spec artifact preview and open-in-detail rules.
- Security / Safe Preview trust-class and active-content policy rules.
- Live-update review contract (freeze / buffer / truncate / anchor
  vocabulary).
- Shell interaction safety contract (representation-labeled copy /
  export, focus-return on detail promotion).
- Preview runtime contract (source-mapping confidence and
  stale-editability for artifact previews that carry a preview
  snapshot).

## Change discipline

- Adding a new `output_class`, `size_bucket`, `viewer_mode`,
  `origin_class`, `sandbox_posture_class`,
  `active_content_policy_class`, `source_identity_class`,
  `run_or_kernel_ref.kind`, `large_output_action_class`,
  `truncation` state or reason, `search_scope_class`,
  `bookmark_scope_class`, or `textual_fallback_class` value is
  additive-minor and bumps `output_viewer_object_schema_version`.
- Repurposing an existing value is breaking and requires a new
  decision row.
- Re-exported vocabularies (trust_class, freshness_class,
  redaction_class, copy_scope_class, export_scope_class,
  buffered_change_visibility, representation_class, authority_class,
  execution_context_record_ref) are owned by their home schemas and
  MUST NOT be narrowed, widened, or renamed in this contract.

Building the actual output / log / result / artifact viewer surfaces,
per-surface virtualization implementations, and the rendering pipelines
that translate producer bytes into pixels are explicitly out of scope at
this revision. This contract freezes the object model, the state
vocabulary, and the composition rule; surface implementations land in
later milestones.
