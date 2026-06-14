# Operation-collection and request-list views

## Scope

This document describes the operation-collection tree and request-list view
surfaces that render the frozen API-collection matrix as a real consumer. Each
collection-view and request-list-view row keeps protocol class, environment
identity, contract-source and freshness badge, last-run state, retention mode,
provenance, and open-detail/inspect/export actions inspectable so large API
workspaces stay legible, versionable, and reviewable instead of degenerating
into ad hoc file trees.

The views reuse the canonical matrix vocabulary (`contract_source_class`,
`contract_freshness_state`, `retention_mode`) and the composer export-redaction
vocabulary (`export_redaction`) rather than minting local synonyms, and they add
view-specific vocabularies for protocol class, environment class, last-run
state, provenance, and saved-view visibility.

## Truth sources

- Implementation: `crates/aureline-api/src/implement_operation_collection_and_request_list_views_with_protocol_class_environment_retention_mode_and_contract_or_source_badges/mod.rs`
- Schema: `schemas/data/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.schema.json`
- Checked-in packet: `artifacts/data/m5/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.json`
- Fixtures: `fixtures/data/m5/implement_operation_collection_and_request_list_views_with_protocol_class_environment_retention_mode_and_contract_or_source_badges/`
- Upstream matrix: `artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`

## Locked vocabulary

| Term | Family | Meaning |
|---|---|---|
| `rest`, `graphql`, `grpc`, `websocket` | protocol class | Wire-protocol badge shown on each request-list row. REST and GraphQL are the protocols M5 claims; gRPC and WebSocket are reserved badge classes. |
| `local`, `development`, `staging`, `production`, `managed` | environment class | Named-environment posture shown alongside the friendly name; identity is never reduced to the friendly name alone. |
| `never_run`, `succeeded`, `failed`, `blocked_pending_review`, `stale_needs_resend` | last-run state | Outcome of the last run; stale schema or persisted-operation drift never reads as a green `succeeded`. |
| `local_only_history`, `imported_snapshot`, `provider_linked_contract`, `managed_shared_artifact` | provenance | How the request-list row is sourced; managed/shared rows never inherit desktop-local trust. |
| `private_local`, `workspace_shared` | saved-view visibility | Privacy scope of a saved view; shared views never inherit desktop-local trust. |

## Consumer surfaces

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Request workspace collection tree and request list | stable | stable | The tree and list show protocol, environment, contract-source and freshness, last-run state, retention, and provenance with keyboard navigation and stable ids. |
| Command palette request quick-open | stable | stable | Keyboard-first quick-open ranks by stable id and keeps protocol, environment, and contract identity visible. |
| CLI and headless request list | stable | stable | Headless output prints contract-source, freshness, and last-run state so a blocked row never reads as a green send. |
| Support export collection and request-list view | stable | stable | Support exports carry view truth with redaction-safe export postures. |
| Help and About request-views contract | stable | stable | Help/About describe the view contract, vocabularies, and narrowing rules. |

## Narrowing and honesty rules

- A request-list row whose `contract_freshness_state` is `schema_stale` or
  `contract_unavailable` must not show a `succeeded` last-run state; it narrows
  to `blocked_pending_review` or `stale_needs_resend`.
- A `managed_shared_artifact` row must resolve to a managed environment that
  does not inherit desktop-local trust.
- Saved views store reviewable, text-first filter summaries and never opaque
  binary state; shared saved views never inherit desktop-local trust.
- A stable surface narrows below stable when its proof packet or any required
  guard is missing.
- The views reference the frozen API-collection matrix as a verified upstream
  packet; the matrix remains the source of collection, request, contract,
  origin, and retention truth.
