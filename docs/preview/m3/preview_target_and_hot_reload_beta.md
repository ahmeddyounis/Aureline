# Preview-origin, preview-target, hot-reload event class, browser-runtime session-origin, and source-mapping truth — beta contract

This document freezes the cross-surface truth model every claimed beta
preview row consults so the chrome can answer four questions on the same
row, in one read, without inferring anything:

1. **Which runtime or renderer produced the view I'm looking at right now?**
   — the `preview_origin_descriptor_record`.
2. **Which target kind does this view represent?** — the
   `preview_target_descriptor_record`. Viewport presets, design renderers,
   simulators, physical devices, browser tabs, embedded webviews, and remote
   preview targets all carry distinct closed labels; degraded targets cite
   an explicit reduced-capability reason instead of inheriting the desktop
   default's capability set.
3. **How fresh and how exact is the source mapping I would jump from, and
   which cross-origin / protocol limits apply when this view comes from a
   browser session?** — the `source_mapping_descriptor` carried inside the
   `browser_runtime_session_origin_record`, plus the hot-reload event class
   on the `hot_reload_state_descriptor_record`.
4. **Whether the next action mutates local runtime state, browser state,
   or a remote target — and what side-effect review and support-export
   summary the surface must keep on screen.** — the
   `runtime_mutation_action_plan_record`.

This contract sits **above** the existing preview-runtime strip / device-
target picker / hot-reload-state record contract
(`/docs/preview/preview_runtime_surface_contract.md`) and **below** the
consumer chrome on every claimed beta preview row. It does not implement
preview runtimes, simulators, or source mappers; it freezes how a beta row
is allowed to talk about them.

Companion artifacts:

- [`/schemas/preview/preview_target_descriptor.schema.json`](../../../schemas/preview/preview_target_descriptor.schema.json)
  — boundary schema for the
  `preview_origin_descriptor_record`,
  `preview_target_descriptor_record`,
  and
  `runtime_mutation_action_plan_record`.
- [`/schemas/preview/hot_reload_state.schema.json`](../../../schemas/preview/hot_reload_state.schema.json)
  — boundary schema for the existing six-class
  `hot_reload_state_record` and the new beta
  `hot_reload_state_descriptor_record` (event-class projection on top of
  the strip state).
- [`/schemas/browser_runtime/session_origin.schema.json`](../../../schemas/browser_runtime/session_origin.schema.json)
  — boundary schema for the
  `browser_runtime_session_origin_record`, including the source-mapping
  quality descriptor every browser-runtime inspector quotes on its
  source-jump button row.
- [`/crates/aureline-preview/src/preview_origin/`](../../../crates/aureline-preview/src/preview_origin/mod.rs)
  — Rust truth model. Every descriptor / plan exposes a `validate()` that
  surfaces the honesty rules below verbatim.
- [`/fixtures/preview/m3/preview_origin_and_browser_runtime/`](../../../fixtures/preview/m3/preview_origin_and_browser_runtime/)
  — worked corpus. Positive fixtures must validate clean; negative
  fixtures cite the named violation token they exercise.

## Why this is a separate beta contract

The existing preview-runtime strip / picker / hot-reload contract already
freezes the snapshot-projected six-class hot-reload state, the three-class
strip mapping projection, and the device-target picker partition. It does
*not* yet make these claims first-class on a row that is **not** a
picker / strip projection — the beta preview row that just renders one
preview-origin / target combination and offers one source-jump and one
mutation-capable action.

Without this contract:

- a beta row can claim a "live preview" without naming whether the runtime
  is a local dev server, a remote workspace runtime, a managed pool slot,
  an embedded renderer, or a static evidence projection;
- a degraded simulator target can silently inherit the desktop default's
  capability badge;
- hot-reload, fast-refresh, reconnect, full-restart, and stale-output
  events all collapse into "live preview";
- source jumps from a browser-runtime view advertise "go to source" without
  disclosing that the cross-origin posture blocks the jump entirely;
- a reload button claims local-only safety when it actually crosses a
  remote workspace or managed-pool boundary;
- support / issue intake exports leak raw URLs / cookies / selectors
  because the surface had no closed-vocabulary fallback to fall back to.

This contract removes all five paths.

## Records

### preview_origin_descriptor_record

Names the runtime / renderer that produced the current view.

Closed vocabularies:

- `preview_origin_class` —
  `local_dev_server` /
  `remote_or_container_runtime` /
  `managed_preview_service` /
  `embedded_renderer` /
  `imported_or_static_evidence`.
- `preview_origin_lifecycle_phase` —
  `not_started` /
  `warming` /
  `running` /
  `reconnecting` /
  `restarting` /
  `suspended` /
  `stopped` /
  `unreachable` /
  `not_applicable_static_evidence`.
- `preview_origin_sharing_posture` —
  `local_only` /
  `same_device_or_lan` /
  `authenticated_org_route` /
  `signed_preview_link` /
  `public_route` /
  `not_applicable_no_network_surface`.

Honesty rules (cited by `validate()` and the schema's `allOf`):

- `managed_preview_service` requires a non-null `managed_workspace_approval_ref`.
- `imported_or_static_evidence` requires `lifecycle_phase = not_applicable_static_evidence`.
- `lifecycle_phase = not_applicable_static_evidence` is reserved for
  `imported_or_static_evidence` origins.
- Non-runtime origins (`embedded_renderer` / `imported_or_static_evidence`)
  cannot declare a live (`running` / `reconnecting`) lifecycle phase.
- Remote-audience sharing postures
  (`authenticated_org_route` / `signed_preview_link` / `public_route`)
  require a non-null `exposure_record_ref`.
- Remote / managed origins
  (`remote_or_container_runtime` / `managed_preview_service`)
  cannot advertise `local_only` sharing posture (the local-only-safety
  overclaim).

### preview_target_descriptor_record

Names the target kind on screen right now, and the capability profile the
target row can honestly claim.

Closed vocabularies:

- `preview_target_class` —
  `viewport_preset_only` /
  `design_renderer_target` /
  `simulator_target` /
  `physical_device_target` /
  `browser_tab_target` /
  `embedded_webview_target` /
  `remote_preview_target`.
- `device_capability_class` —
  `desktop_default` /
  `touch` /
  `mobile` /
  `tablet` /
  `wearable` /
  `television` /
  `high_dpi` /
  `low_resource` /
  `reduced_capability` /
  `not_applicable`.
- `reduced_capability_reason` —
  `none` /
  `no_input_device` /
  `no_camera` /
  `no_gpu_acceleration` /
  `low_resource_mode` /
  `network_restricted` /
  `protocol_downgraded` /
  `policy_narrowed` /
  `unsupported_runtime_feature` /
  `emulator_fallback`.

Honesty rules:

- `device_capability_class = reduced_capability` requires a
  `reduced_capability_reason` other than `none`.
- Any other `device_capability_class` requires
  `reduced_capability_reason = none`.
- Browser-runtime targets
  (`browser_tab_target` / `embedded_webview_target` / `remote_preview_target`)
  require a non-null `browser_runtime_session_origin_ref`.
- Non-browser-runtime targets
  (`viewport_preset_only` / `design_renderer_target` / `simulator_target` /
  `physical_device_target`) must not publish a
  `browser_runtime_session_origin_ref`.
- `viewport_preset_only` with `desktop_default` capability must publish a
  positive `viewport_pixel_width` and `viewport_pixel_height`.

### hot_reload_state_descriptor_record

Beta event-class projection layered on top of the existing six-class
strip-projection record.

Closed vocabularies:

- `hot_reload_event_class` —
  `hot_reload` /
  `fast_refresh` /
  `reconnect` /
  `full_restart` /
  `stale_output` /
  `unavailable`.
- `hot_reload_descriptor_underlying_state_class` —
  `applied` /
  `partial` /
  `restart_required` /
  `rebuild_required` /
  `failed` /
  `unavailable`.

Honesty rules:

- Projection map: `hot_reload` requires
  `underlying_state_class ∈ {applied, partial}`; `fast_refresh` requires
  `applied`; `reconnect` requires `{applied, partial}`; `full_restart`
  requires `{restart_required, applied}`; `stale_output` requires
  `{partial, failed, rebuild_required}`; `unavailable` requires
  `unavailable`.
- `fast_refresh` implies `component_state_preserved = true`. A surface that
  loses component state must downgrade to `hot_reload`.
- `full_restart` and `unavailable` forbid `component_state_preserved = true`.
- Recovery routes are gated per event class. The chrome never collapses
  failed, rebuild-required, or full-restart events into a generic
  "live preview" or "stale" label.

### browser_runtime_session_origin_record

The cross-surface answer to "which browser session is producing this view,
and what cross-origin / protocol limits apply when we inspect it."

Closed vocabularies:

- `browser_session_origin_class` —
  `attached_local_browser` /
  `embedded_webview` /
  `remote_devtools_bridge` /
  `external_handoff_browser` /
  `no_session_attached`.
- `browser_session_scope_class` —
  `per_tab` /
  `per_window` /
  `per_profile` /
  `per_webview` /
  `per_extension_host` /
  `handoff_only` /
  `not_applicable`.
- `cross_origin_posture_class` —
  `same_origin` /
  `cross_origin_allowed` /
  `cross_origin_blocked` /
  `cross_origin_partially_blocked` /
  `mixed_content_blocked` /
  `not_applicable`.
- `protocol_posture_class` —
  `secure_context` /
  `insecure_context_local` /
  `insecure_context_remote_downgraded` /
  `devtools_protocol_only` /
  `bridge_only_no_protocol` /
  `not_applicable`.
- `source_mapping_quality_class` —
  `exact` /
  `heuristic` /
  `stale` /
  `partial` /
  `unavailable`.

Honesty rules:

- `external_handoff_browser` forbids a live `session_handle_ref` and
  requires a non-null `handoff_record_ref` with
  `session_scope_class = handoff_only`.
- `no_session_attached` forbids a `session_handle_ref` and requires
  `cross_origin_posture = protocol_posture = not_applicable`.
- Inspectable sessions
  (`attached_local_browser` / `embedded_webview` / `remote_devtools_bridge`)
  require a `session_handle_ref`.
- Cross-origin postures that imply blocked targets
  (`cross_origin_blocked` / `cross_origin_partially_blocked` /
  `mixed_content_blocked`) cannot claim
  `source_mapping.source_mapping_quality_class = exact` (the explicit
  "cross-origin limits preserved" invariant).
- `source_mapping_descriptor` sub-invariants: `exact` forbids unmappable
  nodes and requires full coverage; `partial` requires at least one
  unmappable and one mapped node; `unavailable` forbids mapped nodes.

### runtime_mutation_action_plan_record

Every mutation-capable preview / browser-runtime action publishes one plan
*before* it is admissible. Inspect-only actions also publish a plan so the
chrome can keep the disclosure shape uniform.

Closed vocabularies:

- `mutation_action_kind` —
  `inspect_only` /
  `reload_preview` /
  `clear_browser_storage` /
  `replay_network_request` /
  `live_style_edit` /
  `restart_runtime` /
  `navigate_browser_tab` /
  `trigger_hot_reload` /
  `toggle_device_condition` /
  `export_inspector_state`.
- `mutation_blast_class` —
  `no_mutation_inspect_only` /
  `local_runtime_only` /
  `local_browser_state_only` /
  `local_runtime_and_browser_state` /
  `remote_runtime_reachable` /
  `managed_preview_service_state` /
  `shared_route_audience` /
  `not_applicable_static_evidence`.
- `mutation_review_requirement` —
  `no_review_required_inspect_only` /
  `explicit_confirm_before_apply` /
  `managed_approval_required_before_apply` /
  `blocked_not_admissible`.

Honesty rules:

- `inspect_only` / `export_inspector_state` cannot declare a mutation
  blast class.
- Mutation-capable actions cannot declare
  `blast_class = no_mutation_inspect_only`.
- Remote / governed blast classes
  (`remote_runtime_reachable` / `managed_preview_service_state` /
  `shared_route_audience`) require
  `managed_approval_required_before_apply` (or `blocked_not_admissible`).
- `managed_approval_required_before_apply` requires a non-null
  `managed_workspace_approval_ref`.
- `blocked_not_admissible` requires a non-null `block_reason_summary`.
- `block_reason_summary` is admissible only when
  `review_requirement = blocked_not_admissible`.
- Browser-state-mutating actions
  (`clear_browser_storage` / `navigate_browser_tab` /
  `replay_network_request` / `live_style_edit`) require a non-null
  `browser_runtime_session_origin_ref`.
- Cross-validate (against the referenced origin / target / session):
  - `imported_or_static_evidence` origin cannot host a mutation-capable
    plan.
  - Remote / managed origin cannot host a local-only blast claim.
  - Remote-audience sharing posture requires
    `shared_route_audience` blast class for mutations.
  - `managed_preview_service` origin requires
    `managed_approval_required_before_apply` for mutations.
  - A browser-runtime session that does not admit mutation
    (`external_handoff_browser` / `no_session_attached`) cannot host a
    mutation plan with any mutation blast class.

## Support / export safety

Every record is closed-vocabulary or opaque-handle by construction:

- Reviewer-facing labels (`redacted_runtime_label`, `redacted_label`,
  `redacted_session_label`) never embed raw URLs, raw hostnames, raw IPs,
  raw device serials, raw account handles, raw cookies, or raw selectors.
- Opaque refs (`runtime_handle_ref`, `exposure_record_ref`,
  `device_target_descriptor_ref`, `browser_runtime_session_origin_ref`,
  `session_handle_ref`, `handoff_record_ref`,
  `managed_workspace_approval_ref`, `preview_snapshot_record_ref`)
  are stable, support-export-safe identifiers — never raw application
  state.
- Side-effect and support-export summaries on
  `runtime_mutation_action_plan_record` are explicit closed-vocabulary
  sentences. The chrome always renders them on the action confirm row,
  and a support export quotes the summary verbatim.
- Issue intake / support exports inherit the same redaction floor:
  raw URLs, raw cookies, raw selectors, raw private app state never
  appear; only the closed enum tokens, opaque handles, and the redacted
  labels / summaries above do.

## Degraded targets degrade honestly

Reduced-capability / unsupported / unavailable preview targets MUST cite
`device_capability_class = reduced_capability` and a non-`none`
`reduced_capability_reason`. The chrome never silently inherits the
desktop default's capability badge. When a preview surface cannot satisfy
the browser-runtime inspection lane at all (e.g. the runtime adapter does
not support the inspector), the surface publishes
`browser_runtime_session_origin_record` with
`session_origin_class = no_session_attached` and
`source_mapping_quality_class = unavailable`, and any mutation plan
against it must be `blocked_not_admissible` with a `block_reason_summary`.
