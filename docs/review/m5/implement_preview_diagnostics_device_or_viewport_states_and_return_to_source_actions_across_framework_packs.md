# Preview Diagnostics, Device/Viewport States, and Return-to-Source Actions Across Framework Packs

Status: canonical M5 review-lane contract. The checked-in implementation,
fixtures, schema, and proof packet produced by this lane are canonical; later
product, help, and support surfaces consume them rather than re-describing the
state manually.

- Crate module: `aureline-review` →
  `implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs`
- Producer: `aureline_review::current_preview_diagnostics_export`
- Packet type: `PreviewDiagnosticsPacket` (`record_kind =
  implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs`,
  `schema_version = 1`)
- Boundary schema:
  `schemas/review/implement-preview-diagnostics-device-or-viewport-states-and-return-to-source-actions-across-framework-packs.schema.json`
- Support export:
  `artifacts/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/support_export.json`
- Fixtures:
  `fixtures/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/`

## Purpose

This lane surfaces a build, compile, runtime, or hot-reload diagnostic raised
while previewing a reviewed change from inside the product without ever hiding
which framework pack produced it, which device / viewport state the view was
rendered in, or how fresh and how exact the source mapping behind a
return-to-source jump is. It binds four pillars into one export-safe truth packet
that the preview diagnostics panel, preview panel, diagnostic card,
return-to-source action, review workspace header, command palette, CLI / headless
output, support exports, diagnostics, and Help / About all project identically.

It builds on, and references by id, the preview-target descriptor contract
(`schemas/preview/preview_target_descriptor.schema.json`), the device-target
descriptor contract (`schemas/preview/device_target_descriptor.schema.json`), the
hot-reload state contract (`schemas/preview/hot_reload_state.schema.json`), and
the trust-class vocabulary (`schemas/security/trust_class.schema.json`).

## Records

### Framework pack row

Each framework pack row names a stable `pack_id` (which diagnostics reference), a
`pack_class` (`web_react`, `web_vue`, `web_svelte`, `web_angular`,
`web_static_html`, `native_runtime`, `generic_framework_pack`, or
`unknown_pack_provider_owned`), a `pack_label`, three capability flags
(`supports_source_mapping`, `supports_viewport_emulation`,
`supports_hot_reload`), a coverage label, and a disclosure label. The unknown
pack class is never flattened into a known pack. Every diagnostic references an
existing pack row.

### Preview diagnostic row

Each diagnostic row names its durable review anchor (`durable_anchor_id`), the
`pack_id` it came from, and a redaction-aware preview target label. It carries:

- `severity` — one of `info`, `warning`, `error`, `fatal`, or
  `unknown_severity_provider_owned`. `error`, `fatal`, and the unknown severity
  each require at least one attention reason.
- `diagnostic_kind` — one of `build_error`, `compile_error`, `runtime_error`,
  `hot_reload_failure`, `type_error`, `lint_warning`, `console_error`, or
  `unknown_diagnostic_provider_owned`. The unknown kind requires an attention
  reason.
- `message_label` — a redaction-aware diagnostic message (no raw stack or source
  body).
- `viewport_state` — a `DeviceViewportState` (`viewport_class`, `device_label`,
  `dimensions_label`, `emulated`, `state_disclosed`). The state must be disclosed:
  a non-empty device label, a non-empty dimensions label, and `state_disclosed`
  set true. An `unknown_viewport_provider_owned` class requires an attention
  reason.
- `source_mapping` — a `SourceMappingDisclosure` (`mapping_class`,
  `freshness_class`, `mapping_disclosed`, `mapping_label`). The mapping must be
  disclosed: a non-empty mapping label and `mapping_disclosed` set true. A
  `generated_no_source_map`/unknown mapping class, or a `stale_prior_build`/unknown
  freshness, each require an attention reason.
- `return_to_source` — a `ReturnToSourceAction` (`action_kind`,
  `action_disclosed`, `read_only`, `action_label`, `handoff_ref`). The action must
  be disclosed: a non-empty action label and `action_disclosed` set true. The
  `read_only` flag must match the action kind — `jump_to_source_local`,
  `reveal_in_editor`, `copy_source_location`, and `unsupported_no_source_map` are
  read-only; `open_in_browser_handoff` is not read-only and must cite a
  `handoff_ref`. An `unsupported_no_source_map` action must be blocked.
- `blocked_class` — `not_blocked` or one of the blocked reasons, including
  `blocked_no_source_map`, `blocked_source_map_stale_review_required`, and
  `blocked_generated_content_no_origin`. A blocked action carries at least one
  attention reason.
- `actor_attribution_label` and `audit_row_ref` — both required and non-empty, so
  every diagnostic and its action is attributable and lands an audit row.

## Invariants

`PreviewDiagnosticsPacket::validate` returns a stable list of
`PreviewDiagnosticsViolation` tokens. The packet is canonical only when the list
is empty. The enforced invariants are:

- `wrong_record_kind` / `wrong_schema_version` / `missing_identity` — record kind,
  schema version, and identity fields are correct and present.
- `missing_source_contracts` — the schema, doc, preview-target, device-target,
  hot-reload, and trust-class refs are all present.
- `framework_pack_rows_missing` / `framework_pack_row_incomplete` — at least one
  framework pack row, each with its required fields.
- `diagnostic_rows_missing` / `diagnostic_row_incomplete` — at least one
  diagnostic row, each with its required fields.
- `orphan_pack_reference` — every diagnostic references an existing framework
  pack row.
- `viewport_state_undisclosed` — every diagnostic discloses its device / viewport
  state.
- `source_mapping_undisclosed` — every diagnostic discloses its source mapping.
- `return_to_source_undisclosed` — every diagnostic discloses its
  return-to-source action.
- `return_to_source_read_only_mismatch` — the read-only flag matches the action
  kind.
- `return_to_source_handoff_ref_missing` — an `open_in_browser_handoff` action
  cites a handoff ref.
- `return_to_source_unsupported_not_blocked` — an `unsupported_no_source_map`
  action is blocked.
- `attribution_missing` — every diagnostic carries an actor attribution and audit
  row.
- `attention_reason_missing` — an `error`/`fatal`/unknown severity, an unknown
  kind, an unknown viewport, a missing/unknown source map, or a blocked action
  carries at least one attention reason.
- `downgrade_triggers_missing` / `consumer_surfaces_missing` — both lists are
  non-empty.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — the review, projection, and proof blocks hold.
- `raw_boundary_material_in_export` — the export carries no forbidden boundary
  material.

## Downgrade behavior

The `downgrade_triggers` list names the conditions that narrow this lane below
its claimed qualification: `proof_stale`, `policy_blocked`,
`diagnostic_attribution_missing`, `source_map_stale`, `source_map_undisclosed`,
`viewport_state_undisclosed`, `framework_pack_unknown`,
`return_to_source_unsupported`, `trust_narrowing`, and
`upstream_dependency_narrowed`. Proof freshness carries an SLO (168 hours) and an
automatic-narrow flag, so stale or underqualified rows narrow the claim before
publication rather than overstating it.

## Boundary

Raw preview URLs, raw host names, raw diagnostic stacks, raw source bodies, raw
provider payloads, raw absolute paths, raw author email addresses, credentials,
and live provider responses never cross this boundary. The packet is
metadata-only: framework pack capabilities, severities, diagnostic kinds,
viewport classes, source-mapping classes, freshness classes, action kinds,
blocked classes, reviewable labels, and contract references. Every diagnostic,
viewport, source mapping, and return-to-source action stays attributable and
reviewable before any handoff or upstream effect fires.
