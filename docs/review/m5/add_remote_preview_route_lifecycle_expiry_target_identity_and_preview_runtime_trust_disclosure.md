# Remote Preview Route Lifecycle, Expiry, Target Identity, and Preview/Runtime Trust Disclosure

Status: canonical M5 review-lane contract. The checked-in implementation,
fixtures, schema, and proof packet produced by this lane are canonical; later
product, help, and support surfaces consume them rather than re-describing the
state manually.

- Crate module: `aureline-review` →
  `add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure`
- Producer: `aureline_review::current_remote_preview_route_export`
- Packet type: `RemotePreviewRoutePacket` (`record_kind =
  add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure`,
  `schema_version = 1`)
- Boundary schema:
  `schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json`
- Support export:
  `artifacts/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/support_export.json`
- Fixtures:
  `fixtures/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/`

## Purpose

This lane publishes a remote preview route for a reviewed change from inside the
product without ever surfacing a route that is unattributable, that never
expires, that hides which target it points at, or that conceals the trust posture
of the runtime serving it. It binds four pillars into one export-safe truth
packet that the preview panel, remote preview route card, route lifecycle sheet,
review workspace header, command palette, CLI / headless output, support exports,
diagnostics, and Help / About all project identically.

It builds on, and references by id, the frozen preview-route contract
(`schemas/runtime/preview_route.schema.json`), the browser-runtime-session
contract (`schemas/runtime/browser_runtime_session.schema.json`), the
pipeline-run-row contract (`schemas/ci/pipeline_run_row.schema.json`), and the
trust-class vocabulary (`schemas/security/trust_class.schema.json`).

## Records

### Remote preview route row

Each route row names its durable review anchor (`durable_anchor_id`) and a
redaction-aware target identity (`target_identity_label`, `target_run_id`,
`target_commit_label`). It carries:

- `lifecycle_phase` — one of `requested`, `provisioning`, `live`,
  `expiring_soon`, `expired`, `revoked`, `failed`, or
  `unknown_phase_provider_owned`. The unknown phase is never flattened into a
  known one and must carry at least one attention reason.
- `expiry` — a disclosure block (`expiry_state`, `ttl_seconds`,
  `expires_at_label`, `auto_revoke_on_expiry`). A time-bounded state
  (`active_time_bounded`, `expiring_soon`, `expired`, `revoked_before_expiry`)
  must carry a positive TTL, a non-empty expiry label, and `auto_revoke_on_expiry`
  set to true. The `no_expiry_unbounded` state means the route has no expiry
  bound; such a route must be `blocked_no_expiry_not_time_bounded` rather than
  served.
- `host_identity` — the host/provider class (`aureline_managed_host`,
  `provider_hosted`, `self_hosted_tunnel`, `local_loopback`,
  `unknown_host_provider_owned`), a redaction-aware host label, and
  `origin_disclosed` set to true. An unknown host carries an attention reason.
- `preview_trust` — the runtime trust class (`sandboxed_isolated`,
  `sandboxed_network_limited`, `runtime_trusted_workspace`,
  `untrusted_remote_content`, `unknown_trust_provider_owned`), the network egress
  class (`no_egress`, `egress_to_named_targets`, `unrestricted_egress`,
  `unknown_egress_provider_owned`), an `executes_untrusted_code` flag, a
  `runtime_writes_disclosed` flag set to true, and a trust disclosure label.
  Untrusted content, unrestricted/unknown egress, executing untrusted code, or an
  unknown trust class each require an attention reason.
- `mutation_mode` — one of `publish_now`, `open_in_provider`, or
  `deferred_publish`. The local-only `local_draft` mode is intentionally absent:
  publishing a remote preview route reaches upstream host state. `publish_now`
  must cite an `approval_ticket_ref`, `open_in_provider` must cite a
  `browser_handoff_ref`, and `deferred_publish` must cite a `deferred_queue_ref`.
- `blocked_class` — `not_blocked` or one of the blocked reasons, including
  `blocked_route_expired`, `blocked_no_expiry_not_time_bounded`, and
  `blocked_untrusted_runtime_review_required`. A blocked route carries at least
  one attention reason.
- `actor_attribution_label` and `audit_row_ref` — both required and non-empty, so
  every published or revoked route is attributable and lands an audit row.

### Lifecycle events

Each lifecycle event row binds to a route by `route_id` and records a typed
`event_kind` (`provisioned`, `went_live`, `extended_expiry`, `expiry_warning`,
`expired`, `revoked`, `failed`, `unknown_event_provider_owned`), the `from_phase`
and `to_phase` it transitioned between, and a disclosure label. Every route
carries at least one lifecycle event row, and every event row references an
existing route.

## Invariants

`RemotePreviewRoutePacket::validate` returns a stable list of
`RemotePreviewRouteViolation` tokens. The packet is canonical only when the list
is empty. The enforced invariants are:

- `wrong_record_kind` / `wrong_schema_version` / `missing_identity` — record kind,
  schema version, and identity fields are correct and present.
- `missing_source_contracts` — the schema, doc, preview-route, browser-runtime,
  pipeline-run, and trust-class refs are all present.
- `route_rows_missing` / `route_row_incomplete` — at least one route row, each
  with its required fields.
- `target_identity_missing` — every route names its target identity, run, and
  commit.
- `expiry_not_time_bounded` — an unbounded route is blocked for not being
  time-bounded.
- `expiry_disclosure_incomplete` — a time-bounded route carries a positive TTL, an
  expiry label, and auto-revoke.
- `host_identity_undisclosed` — every route discloses its host label and origin.
- `runtime_trust_undisclosed` — every route discloses its runtime trust and write
  scope.
- `attribution_missing` — every route carries an actor attribution and audit row.
- `mutation_grant_ref_missing` — each mutation mode cites the grant it requires.
- `attention_reason_missing` — an unknown phase/host/trust/egress, untrusted
  runtime, or blocked route carries at least one attention reason.
- `route_missing_lifecycle_event` — every route has at least one lifecycle event
  row.
- `orphan_event_reference` — a lifecycle event row references an existing route.
- `lifecycle_event_rows_missing` / `lifecycle_event_row_incomplete` — at least one
  lifecycle event row, each with its required fields.
- `downgrade_triggers_missing` / `consumer_surfaces_missing` — both lists are
  non-empty.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — the review, projection, and proof blocks hold.
- `raw_boundary_material_in_export` — the export carries no forbidden boundary
  material.

## Downgrade behavior

The `downgrade_triggers` list names the conditions that narrow this lane below
its claimed qualification: `proof_stale`, `policy_blocked`,
`route_attribution_missing`, `route_expiry_unbounded`, `route_expired_or_revoked`,
`runtime_trust_undisclosed`, `host_identity_undisclosed`, `trust_narrowing`, and
`upstream_dependency_narrowed`. Proof freshness carries an SLO (168 hours) and an
automatic-narrow flag, so stale or underqualified rows narrow the claim before
publication rather than overstating it.

## Boundary

Raw preview URLs, raw host names, raw run / log bodies, raw provider payloads, raw
absolute paths, raw author email addresses, credentials, and live provider
responses never cross this boundary. The packet is metadata-only: lifecycle
phases, expiry states, host classes, trust classes, network egress classes,
mutation modes, blocked classes, reviewable labels, and contract references. Every
route, expiry, host, and trust disclosure stays attributable and reviewable before
any upstream effect fires.
