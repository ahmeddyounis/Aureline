# Preview Diagnostics, Device/Viewport States, and Return-to-Source Actions Across Framework Packs

- Packet: `preview-diagnostics:stable:0001`
- Schema: `schemas/review/implement-preview-diagnostics-device-or-viewport-states-and-return-to-source-actions-across-framework-packs.schema.json`
- Support export: `artifacts/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/support_export.json`
- Contract doc: `docs/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs.md`
- Fixtures: `fixtures/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/`
- Producer: `aureline_review::current_preview_diagnostics_export`

## Coverage

- **Framework packs** name each convenience framework integration (`web_react`,
  `web_vue`, `web_svelte`, `web_angular`, `web_static_html`, `native_runtime`,
  `generic_framework_pack`, `unknown_pack_provider_owned`) and disclose whether
  the pack supports source mapping, viewport emulation, and hot reload, so a
  diagnostic can never claim a capability the pack does not have.
  `unknown_pack_provider_owned` is never flattened into a known pack.
- **Preview diagnostics** carry, per diagnostic, the durable review anchor, the
  framework pack id, a typed severity (`info`, `warning`, `error`, `fatal`,
  `unknown_severity_provider_owned`), a typed kind (`build_error`,
  `compile_error`, `runtime_error`, `hot_reload_failure`, `type_error`,
  `lint_warning`, `console_error`, `unknown_diagnostic_provider_owned`), a
  redaction-aware message label, an attention block, and an actor attribution with
  an audit row, so a diagnostic is always anchored, attributable, and honest about
  its severity. An `error`/`fatal`/unknown severity, an unknown kind, an unknown
  viewport, a missing/unknown source map, or a blocked action each require an
  explicit attention reason.
- **Device / viewport states** record, per diagnostic, the viewport class
  (`desktop`, `tablet`, `mobile`, `responsive_fluid`, `custom_viewport`,
  `device_emulation`, `unknown_viewport_provider_owned`), a device label, a
  dimensions label, an emulation flag, and a disclosure flag, so a preview can
  never hide which device or viewport produced the view.
- **Return-to-source actions** record, per diagnostic, the source mapping
  exactness (`exact_line_column`, `approximate_line`, `file_only`,
  `generated_no_source_map`, `unknown_mapping_provider_owned`), the mapping
  freshness (`fresh_current_build`, `stale_prior_build`,
  `unknown_freshness_provider_owned`), and a typed action kind
  (`jump_to_source_local`, `reveal_in_editor`, `copy_source_location`,
  `open_in_browser_handoff`, `unsupported_no_source_map`). A jump is read-only
  navigation unless an attributable `open_in_browser_handoff` cites a
  `handoff_ref`, and a stale or missing source map narrows the action
  (`blocked_source_map_stale_review_required`, `blocked_no_source_map`,
  `blocked_generated_content_no_origin`) rather than jumping blind.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: the diagnostic severity is explicit; the device/viewport
state, source mapping, source-mapping freshness, and return-to-source action are
all disclosed; a return-to-source action is read-only unless an attributable
handoff is cited; the framework pack identity is explicit; every diagnostic is
anchored and attributable; no action creates hidden write scope; a stale source
map narrows the action; downgrade narrows the claim instead of hiding the lane;
and stale or underqualified rows block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`diagnostic_attribution_missing`, `source_map_stale`, `source_map_undisclosed`,
`viewport_state_undisclosed`, `framework_pack_unknown`,
`return_to_source_unsupported`, `trust_narrowing`, and
`upstream_dependency_narrowed`.

## Boundary

Raw preview URLs, raw host names, raw diagnostic stacks, raw source bodies, raw
provider payloads, raw absolute paths, raw author email addresses, credentials,
and live provider responses never cross this boundary. The packet carries only
metadata, framework pack capabilities, severities, diagnostic kinds, viewport
classes, source-mapping classes, freshness classes, action kinds, blocked
classes, reviewable labels, and contract references. Every diagnostic, viewport,
source mapping, and return-to-source action stays attributable and reviewable
before any handoff or upstream effect fires.
