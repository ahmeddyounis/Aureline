# Browser-runtime, preview-route, and cross-origin / storage review contract

This document freezes the cross-surface contract Aureline uses to
represent **browser-runtime identity**, **preview-route bindings**, and
**risky runtime mutations** before preview and full-stack workflows
land. The goal is simple: a browser-runtime-aware surface (live
preview lane, browser companion handoff, inspection add-on, console /
network / storage viewer, request-replay packet, live-style-edit
packet, service-worker-change packet, support / export reader, hosted-
review reader, AI tool review surface) MUST cite **one** browser-
runtime-session record and (when the session is bound to a service)
**one** preview-route record, instead of inventing per-surface labels
for runtime identity, route binding, cross-origin posture, source-map
freshness, or inspection lifecycle.

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document MUST be updated in the same change.

## Companion artifacts

- [`/schemas/runtime/browser_runtime_session.schema.json`](../../schemas/runtime/browser_runtime_session.schema.json)
  — boundary schema for the `browser_runtime_session_record`.
- [`/schemas/runtime/preview_route.schema.json`](../../schemas/runtime/preview_route.schema.json)
  — boundary schema for the `preview_route_record`.
- [`/fixtures/runtime/browser_runtime_cases/`](../../fixtures/runtime/browser_runtime_cases/)
  — concrete browser-runtime-session and preview-route fixtures
  covering live local, live remote managed, companion handoff,
  imported handoff, replayed capture, inspect-only uncertain, external
  handoff only, and blocked-runtime-identity cases.
- [`/docs/runtime/browser_inspection_contract.md`](./browser_inspection_contract.md)
  plus [`/schemas/runtime/console_event.schema.json`](../../schemas/runtime/console_event.schema.json),
  [`/schemas/runtime/network_event_ref.schema.json`](../../schemas/runtime/network_event_ref.schema.json),
  and [`/schemas/runtime/storage_object_state.schema.json`](../../schemas/runtime/storage_object_state.schema.json)
  — browser inspection packet bodies for console, network, and storage
  evidence that attach to this session / route / source-map truth.

Composes by reference (no payload restated):

- [`/schemas/preview/preview_snapshot.schema.json`](../../schemas/preview/preview_snapshot.schema.json)
  and [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  — preview-snapshot record, mapping_confidence_class, source_sync_state,
  and stale-editability ladder. The browser-runtime session cites a
  preview-snapshot ref when the session backs a preview surface; the
  session record never restates mapping confidence or stale-editability.
- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  and [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  — safe-preview trust-class, connectivity-state, and downgrade-
  trigger ladder. The session re-exports `TrustedLocalActive` /
  `IsolatedRemoteActive`, the connectivity ladder, and the first twelve
  downgrade triggers verbatim.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  and [`/docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md)
  — execution-context record. The session carries
  `execution_context_record_ref` and never restates target identity,
  sandbox posture, or policy epoch.
- [`/schemas/runtime/environment_capsule.schema.json`](../../schemas/runtime/environment_capsule.schema.json)
  and [`/docs/runtime/environment_capsule_contract.md`](./environment_capsule_contract.md)
  — environment-capsule record. Browser runtimes resolve their
  toolchain identity through the capsule the execution context cites.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — browser-handoff packet and approval-ticket envelope referenced
  whenever a session is imported from a companion handoff.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state vocabulary the route's `policy_scope_block`
  and the session's `workspace_trust_state_class` re-export.

If this document disagrees with those sources, those sources win and
this contract plus the companion schemas update in the same change.

## Why freeze this now

Browser-runtime surfaces are easy places for hidden truth to leak. A
"live" preview can lose its runtime identity and keep claiming live
mutation. An inspector can paint a stale source map as fresh and offer
"jump to source" on nodes that no longer exist. A request-replay
button can quietly replay a captured payload against a different
remote target. A storage-clear button can clear cookies for an origin
the runtime no longer maps to. A service-worker registration can
silently install against a different scope. A "share preview" link can
escape its workspace boundary without an expiry or a revoke path.

Without one contract:

- a session can claim live mutation when runtime identity is
  unverifiable;
- an inspector can offer source-jump on an outdated source map;
- a request-replay panel can replay against a remote target that the
  current session is not bound to;
- a cross-origin allowlist can drift between session capture and
  replay;
- a preview share-link can outlive its workspace policy boundary;
- a captured browser handoff can re-execute against a freshened remote
  target without admitting the drift;
- an inspection add-on can declare its own "imported / replayed /
  blocked" labels that do not align with the session's lifecycle.

This contract closes those gaps by freezing one
`browser_runtime_session_record`, one `preview_route_record`, one
mutation-admissibility ladder, one inspection-lifecycle vocabulary,
and one evidence-provenance shape every browser-runtime-aware surface
MUST project the same way.

## Scope

Frozen at this revision:

- one shared `browser_runtime_session_record` shape covering live
  local browser, live remote managed browser, live companion handoff,
  imported handoff, replayed capture, inspect-only uncertain, external-
  handoff-only, and blocked-runtime-identity sessions;
- the runtime-identity / device-profile / theme / source-revision /
  protocol-scope / cross-origin-boundary / source-map-freshness fields
  every session MUST disclose;
- the per-action mutation-admissibility ladder that gates cookie /
  storage clear, request replay, live style edit, service-worker
  registration / unregistration / skip-waiting / clients-claim /
  message-post, and external-browser handoff actions;
- the inspection-lifecycle vocabulary every inspection add-on declares
  through this contract instead of inventing parallel labels (live,
  imported, replayed, paused, blocked);
- the reserved provenance fields for console, network, and storage
  evidence so later browser-inspection surfaces can attach to the same
  runtime session, route, and source-map truth without redefining core
  identity;
- one shared `preview_route_record` shape covering user-authored,
  workspace-shared, managed-workspace, ad-hoc-session, imported-from-
  handoff, imported-from-export-bundle, and AI-tool-proposed routes;
- the service-identity / mapped-port / target-context / actor /
  duration / expiry / revoke-posture / policy-scope fields every route
  MUST disclose so routes are time-bounded, auditable, and preserve
  source-workspace and remote-target identity.

Out of scope (named explicitly so the schema does not creep):

- building any browser debugging tool, browser companion, inspection
  add-on, dev server, request replayer, live-style-edit pipeline,
  service-worker manager, or preview runtime at this milestone;
- minting browser-engine-specific protocol implementations (CDP, WDP,
  WebDriver BiDi);
- the visual-transform manifest or mapped-edit transform pipeline
  itself (governed by the preview-runtime contract);
- the share-sheet UX (this contract only declares the typed share
  block a later share-sheet flow MUST emit);
- any browser-runtime-specific telemetry; payloads narrow through the
  telemetry / support registry like every other surface.

## Shared session record

Every browser-runtime-aware surface emits the same record from
[`browser_runtime_session.schema.json`](../../schemas/runtime/browser_runtime_session.schema.json).
The record does not replace the preview-snapshot record (mapping
confidence and stale-editability live there) or the execution-context
record (target identity, sandbox posture, policy epoch live there).
It is the compact projection consumers read when they need one stable
answer to four questions:

1. What is the **browser-runtime identity** (target, device, theme,
   source revision, protocol scope, cross-origin boundary set, source-
   map freshness) this surface is bound to?
2. What **cross-origin / storage / live-style / service-worker /
   request-replay** mutations are admissible on this surface in this
   state?
3. What **inspection-lifecycle state** (live, imported, replayed,
   paused, blocked) is the add-on in?
4. Which **preview-route record** bound this runtime session to a
   service identity?

### 1. Browser-runtime session class

`browser_runtime_session_class` is the eight-value lane:

- `live_local_browser_runtime_session`
  Live runtime attached to a local browser tab / window / embedded
  webview. Admissible only when runtime identity AND source-map
  identity are explicit.
- `live_remote_managed_browser_runtime_session`
  Live runtime attached to a managed-workspace browser pool. Same
  identity discipline; additionally requires a non-null
  `preview_route_record_ref`.
- `live_companion_handoff_browser_session`
  Live session paired with a browser-handoff packet. Requires a non-
  null `preview_route_record_ref`.
- `imported_from_browser_handoff_packet_session`
  Non-live session hydrated from a captured browser-handoff packet;
  no live mutation, inspection lifecycle is forced to
  `inspection_imported_from_browser_handoff_packet`.
- `replayed_from_capture_session`
  Non-live session rehydrated from a replay capture; no live mutation.
- `inspect_only_uncertain_runtime_session`
  Runtime identity or source mapping is uncertain; mutation is denied.
- `external_handoff_only_session`
  Surface routes the user to an external browser handoff; no in-app
  mutation.
- `blocked_runtime_identity_unverifiable_session`
  Strictest stop. Inspection lifecycle is forced to
  `inspection_blocked_runtime_identity_unverifiable`.

The schema's `allOf` gates enforce that the live classes carry an
explicit `runtime_identity_class` and an in-sync source map, and that
imported / replayed sessions carry the matching capture / handoff ref.

### 2. Runtime identity, engine, and device profile

`runtime_identity_class` names what kind of browser runtime the
session is bound to:

- `tab_in_local_browser`
- `window_in_local_browser`
- `embedded_webview_in_extension_host`
- `embedded_webview_in_native_shell`
- `remote_managed_browser_pool_tab`
- `remote_managed_device_browser`
- `captured_browser_handoff_runtime`
- `replayed_browser_runtime_capture`
- `runtime_identity_unverifiable_user_review_required`

`runtime_identity_unverifiable_user_review_required` forces the
session into a non-live class and forbids `admissible_under_*` on
every mutation entry.

`browser_engine_family_class` and `engine_build_identity_ref` carry
the engine family (Chromium / WebKit / Gecko / embedded variants) and
an opaque build-identity handle. Raw user-agent strings never project
through this record.

`device_profile` carries `device_profile_class`, `theme_class`,
`color_scheme_state_class`, and an optional opaque `device_handle_ref`
plus redaction-safe label and viewport pixel dimensions. A
`viewport_preset_only_no_real_device` profile MUST set
`device_handle_ref` to null and MUST pair with the
`viewport_preset_only` device-target on the paired preview snapshot.

### 3. Source revision, protocol scope, and cross-origin boundary set

`source_revision_anchor` re-uses the source-revision class vocabulary
already frozen on the preview-snapshot record (`vcs_commit_pinned`,
`working_tree_with_pending_edits`, `release_tag_pinned`,
`build_artifact_digest_pinned`, `remote_ref_pinned`,
`captured_at_handoff_pinned`, `unknown_revision`).

`protocol_scope_class` names which URL schemes the runtime is
permitted to load (`http_only_scope`, `https_only_scope`,
`http_and_https_scope`, `file_local_loader_scope`,
`data_url_scope_no_origin`, `blob_url_scope_no_origin`,
`extension_scheme_scope`, `custom_app_scheme_scope`,
`protocol_scope_unknown_requires_review`).

`data_url_scope_no_origin`, `blob_url_scope_no_origin`, and
`protocol_scope_unknown_requires_review` force inspect-only fallback
for cookie / storage / service-worker mutations because there is no
origin to bound the rule against.

`cross_origin_boundary_set` is a non-empty list of boundary entries.
Each entry carries one of the nine `cross_origin_boundary_class`
values (`same_origin_only`, `same_site_cross_origin_with_explicit_allowlist`,
`cross_site_cors_with_explicit_allowlist`,
`cross_origin_iframe_sandboxed_no_credentials`,
`cross_origin_postmessage_only`, `cross_origin_opener_isolated`,
`cross_origin_embedder_isolated`,
`cross_origin_unrestricted_user_review_required`,
`cross_origin_boundary_unknown_requires_review`) plus an opaque origin
label and an optional CORS allowlist label.

The strictest two classes
(`cross_origin_unrestricted_user_review_required`,
`cross_origin_boundary_unknown_requires_review`) force inspect-only
fallback for cross-origin / storage mutation actions through the
mutation-admissibility ladder.

### 4. Source-map freshness

`source_map.source_map_freshness_class` is the seven-value vocabulary:

- `source_map_in_sync_with_runtime`
- `source_map_in_sync_with_session_capture`
- `source_map_stale_runtime_advanced_since_capture`
- `source_map_inline_only_no_external_map`
- `source_map_missing_runtime_only`
- `source_map_redacted_for_export`
- `source_map_freshness_unknown_requires_review`

The schema enforces that any class outside `source_map_in_sync_*` /
`source_map_inline_only_no_external_map` forbids `admissible_under_*`
on every live-style-edit / request-replay-with-modified-payload
mutation entry.

### 5. Mutation-admissibility ladder

The session enumerates every mutation action it intends to offer or
grey out through the `effective_admissible_mutation_actions` array,
where each entry carries one of eighteen `mutation_action_class`
values (cookie / storage clear, request replay, live style edit,
service-worker register / unregister / skip-waiting / clients-claim /
message-post, external browser handoff, share via share sheet) and
one of thirteen `mutation_admissibility_class` values:

- `admissible_under_workspace_trust`
- `admissible_under_session_only_override`
- `admissible_under_approval_ticket` (requires non-null
  `approval_ticket_ref` on the entry)
- `inspect_only_runtime_identity_uncertain`
- `inspect_only_source_mapping_uncertain`
- `inspect_only_cross_origin_boundary_uncertain`
- `inspect_only_protocol_scope_no_origin`
- `external_handoff_only_remote_target_unverifiable`
- `blocked_pending_user_admit`
- `blocked_pending_policy`
- `blocked_pending_workspace_trust`
- `blocked_runtime_identity_unverifiable`
- `not_applicable_imported_or_replayed_session`

Live mutation (any `admissible_under_*` class) is admissible **only**
when:

- `runtime_identity_class` is not `runtime_identity_unverifiable_user_review_required`,
- `source_map.source_map_freshness_class` is `source_map_in_sync_with_runtime` /
  `source_map_in_sync_with_session_capture` / `source_map_inline_only_no_external_map`,
- `protocol_scope_class` is not `data_url_scope_no_origin` /
  `blob_url_scope_no_origin` / `protocol_scope_unknown_requires_review`,
- `workspace_trust_state_class` is `workspace_trust_session_only_temporary`
  or `workspace_trust_trusted` (or the entry is `admissible_under_approval_ticket`
  with a cited approval ticket),
- the session class is one of the live classes (live local / remote /
  companion handoff).

Otherwise the session degrades honestly to inspect-only or external
handoff and the corresponding `downgrade_trigger_observations` cite
the typed reason.

### 6. Inspection lifecycle

Inspection add-ons (console viewer, network viewer, storage viewer,
service-worker inspector, request-replay panel, live-style-edit
panel) declare their state through the session's
`inspection_lifecycle_state`:

- `inspection_live_attached`
- `inspection_live_attached_workspace_trust_session_only`
- `inspection_imported_from_browser_handoff_packet`
- `inspection_replayed_from_capture`
- `inspection_paused_pending_user_action`
- `inspection_blocked_runtime_identity_unverifiable`
- `inspection_blocked_pending_policy`
- `inspection_blocked_pending_workspace_trust`
- `inspection_blocked_external_handoff_only`

An add-on that needs a new state lands an additive-minor enum value
on this schema, not a parallel record. This satisfies the acceptance
rule that **inspection add-ons declare live, imported, replayed, or
blocked state through the same browser-runtime contract instead of
inventing parallel labels.**

### 7. Evidence-provenance reservation

`evidence_provenance` reserves three opaque packet refs
(`console_evidence_packet_ref`, `network_evidence_packet_ref`,
`storage_evidence_packet_ref`) plus a per-stream
`evidence_origin_class` (`live_capture_from_attached_runtime`,
`imported_from_browser_handoff_packet`,
`imported_from_support_export_bundle`,
`imported_from_replay_capture`,
`imported_from_hosted_review_reader`,
`evidence_origin_unverifiable_user_review_required`).

The packet bodies are governed by
[`browser_inspection_contract.md`](./browser_inspection_contract.md)
and its console, network, and storage schemas. This contract only
carries the refs and the origin class so console / network / storage
viewers attach to the **same** runtime session, route, and source-map
truth.

### 8. Downgrade triggers

The first twelve `downgrade_trigger` values re-export the safe-preview
ladder verbatim (`workspace_trust_revoked`, `policy_narrowed`,
`origin_changed`, `origin_missing`, `connectivity_lost`,
`sanitizer_failed`, `sandbox_unavailable`, `runtime_unhealthy`,
`support_export_boundary`, `source_representation_missing`,
`approval_scope_changed`, `delete_review_preserve_last_evidence`).

The remaining eleven values are browser-runtime-specific extensions:
`runtime_identity_unverifiable`, `remote_target_identity_unverifiable`,
`source_map_stale_runtime_advanced`, `source_map_missing`,
`source_map_redacted`, `cross_origin_boundary_unknown`,
`protocol_scope_no_origin`, `service_worker_state_unverifiable`,
`preview_route_expired`, `preview_route_revoked`,
`preview_route_target_context_changed`.

A session that needs to narrow MUST cite a typed trigger from this
list rather than describing the cause in prose.

## Preview-route record

A preview-route binds a service identity to a mapped port / route
inside a workspace or remote-target boundary. Every
`preview_route_record` is **time-bounded, auditable, and preserves
source-workspace or remote-target identity**, satisfying the
acceptance rule.

### 1. Route class

`preview_route_record_class` names where the route came from:

- `user_authored_local_preview_route` — default for individual workspaces.
- `workspace_shared_preview_route` — committed and shared inside a workspace.
- `managed_workspace_preview_route` — provisioned by managed-workspace policy.
- `ad_hoc_session_preview_route` — session-only; share visibility is
  forced into `not_shareable` or `workspace_only`.
- `imported_from_browser_handoff_preview_route` — hydrated from a
  browser-handoff packet; requires a non-null `browser_handoff_packet_ref`
  and forces `mapped_port_route_class = captured_handoff_route_no_tunnel`,
  `route_duration_class = captured_route_no_duration_imported`,
  `route_expiry_state_class = imported_no_expiry_pinned_to_capture`,
  and `route_revoke_posture_class = captured_route_no_live_revoke_path`.
- `imported_from_export_bundle_preview_route` — hydrated from an
  export bundle; same pinning discipline as imported handoff.
- `ai_tool_proposed_preview_route_pending_review` — forbids leaving
  pending review state; forces `actor_class = ai_tool_proposed_actor_pending_review`.

### 2. Service identity, mapped port / route

`service_identity` carries a `service_identity_class` (eleven-value
vocabulary mirroring `preview_runtime_kind_class`) plus an opaque
`service_handle_ref` and an optional service version ref.

`mapped_port_route` carries a `mapped_port_route_class` (nine-value
vocabulary distinguishing loopback, tunneled-to-managed-workspace,
tunneled-to-org, tunneled-to-remote-managed pool, captured-handoff-no-
tunnel, imported-replay-no-tunnel) plus an opaque `port_handle_ref`,
opaque `route_handle_ref`, and an optional `transport_security_class`.

`captured_handoff_route_no_tunnel` and `imported_replay_route_no_tunnel`
are admissible only on `imported_from_*` route classes.

### 3. Target context and source-workspace identity

`target_context_class` preserves where the route was opened against
(individual local / self-hosted org / managed workspace / remote
managed browser pool / remote managed device pool / captured handoff
/ imported export bundle / unknown).

`source_workspace_identity_class` re-exports the same vocabulary the
browser-runtime-session record carries so a route hydrated from a
support export, a companion handoff, or a hosted-review reader cannot
erase which workspace it was opened against.

### 4. Actor

`actor` carries an `actor_class` (nine-value vocabulary covering human
local / workspace admin / managed admin, AI tool proposed pending
review, automation run, CI pipeline, support-export reader, hosted-
review reader, unknown) plus an opaque `actor_handle_ref`. Raw author
identity strings (email, name, account name) never project through
this record.

### 5. Duration, expiry, and revoke posture

`duration` is the time-bounding block:

- `route_duration_class` ∈ {`session_only_no_persistence`,
  `fixed_duration_minutes`, `fixed_duration_hours`,
  `fixed_duration_until_workspace_close`,
  `fixed_duration_until_policy_review`,
  `captured_route_no_duration_imported`, `route_duration_class_unknown_requires_review`}.
  Fixed-duration classes require a non-null `expires_at`.
- `route_expiry_state_class` ∈ {`active_within_duration_window`,
  `expired_past_duration_window`, `revoked_pending_replacement`,
  `revoked_terminal_no_replacement`, `blocked_pending_user_admit`,
  `blocked_pending_policy_review`, `blocked_pending_workspace_trust`,
  `imported_no_expiry_pinned_to_capture`,
  `route_expiry_state_class_unknown_requires_review`}.
  Only `active_within_duration_window` admits live execution.
- `route_revoke_posture_class` ∈ {`user_self_revoke_only`,
  `workspace_admin_revoke`, `managed_admin_revoke`,
  `policy_revocation_only`, `automatic_expiry_only`,
  `captured_route_no_live_revoke_path`,
  `route_revoke_posture_class_unknown_requires_review`}.

### 6. Policy scope

`policy_scope` carries one `policy_scope_class` plus optional opaque
`policy_bundle_ref` / `policy_epoch_ref`. The `workspace_trust_*`
classes mirror the workspace-trust state vocabulary the session
record re-exports; `policy_bundle_pinned`,
`policy_bundle_pending_re_evaluation`,
`policy_bundle_expired_user_review_required`, and
`policy_scope_class_unknown_requires_review` cover the policy-bundle
ladder. `managed_workspace_preview_route` requires
`policy_scope_class ∈ {workspace_trust_managed_locked, policy_bundle_pinned}`.

### 7. Share posture (extension point)

`share` is reserved as a typed extension point. `share_visibility_class`
∈ {`not_shareable`, `workspace_only`, `organization_only`,
`tenant_only`, `public_link`,
`share_visibility_class_unknown_requires_review`}. The schema enforces
that `public_link` / `tenant_only` / `organization_only` forbid
`automatic_expiry_only` revoke posture and forbid `workspace_trust_unset_no_admit`
/ `workspace_trust_restricted_no_admit` policy scope.

`ad_hoc_session_preview_route` is forbidden from sharing outside
`not_shareable` / `workspace_only`.

## Cross-origin / storage review rules

The mutation-admissibility ladder is the rule set every browser-
runtime-aware surface MUST honor when offering risky runtime
mutations. The contract spells out the rules per action family:

### Cookie / storage clear

- `cookie_clear_for_origin`,
  `storage_clear_local_storage_for_origin`,
  `storage_clear_session_storage_for_origin`,
  `storage_clear_indexeddb_for_origin`,
  `storage_clear_cache_storage_for_origin`,
  `storage_clear_service_worker_registrations_for_origin` are
  admissible only when:
  - the session is a live class (live local / remote / companion handoff),
  - `protocol_scope_class` admits an origin (not `data_url_scope_no_origin` /
    `blob_url_scope_no_origin` / `protocol_scope_unknown_requires_review`),
  - the matching `cross_origin_boundary_entry` for the target origin is
    not `cross_origin_unrestricted_user_review_required` /
    `cross_origin_boundary_unknown_requires_review`,
  - `workspace_trust_state_class` is `workspace_trust_session_only_temporary`
    or `workspace_trust_trusted` (or an approval ticket is cited).

When any of those conditions fail, the session degrades to
`inspect_only_*` for the action and cites the typed downgrade trigger.

### Request replay

- `request_replay_with_modified_payload` requires source-map freshness
  to be in-sync (otherwise the modification cannot be reasoned about
  against canonical source).
- `request_replay_unchanged` MAY remain admissible under in-sync /
  inline-only source-map freshness if the session is live and the
  remote-target identity is verified.

When the remote-target identity is `remote_target_identity_unverifiable_user_review_required`,
both replay actions degrade to
`external_handoff_only_remote_target_unverifiable`.

### Live style edit

- `live_style_edit_inline_stylesheet` and
  `live_style_edit_constructable_stylesheet` require source-map
  freshness to be in-sync. Otherwise the session degrades to
  `inspect_only_source_mapping_uncertain` and the matching
  `live_style_edit_inspector_overlay_only` action MAY remain
  admissible (the overlay does not write through to canonical source).
- `live_style_edit_inspector_overlay_only` is admissible whenever the
  session is a live class and runtime identity is explicit, regardless
  of source-map freshness, because it is inspector-only.

### Service-worker change

- `service_worker_register_or_update`, `service_worker_unregister`,
  `service_worker_skip_waiting`, `service_worker_clients_claim`,
  `service_worker_message_post` require an origin
  (`protocol_scope_class` admits one) and a verified cross-origin
  boundary. They additionally require workspace trust trusted /
  session-only-temporary or an approval ticket. When the service-
  worker state is unverifiable, the matching
  `service_worker_state_unverifiable` downgrade trigger MUST appear.

### Inspect-only fallback

When **runtime identity** or **source mapping** is uncertain, the
session MUST set `browser_runtime_session_class` to one of
`inspect_only_uncertain_runtime_session`, `external_handoff_only_session`,
or `blocked_runtime_identity_unverifiable_session` and MUST resolve
**every** mutation entry to a non-`admissible_under_*` class.

This satisfies the acceptance rule that **live mutation is allowed
only when runtime identity and source mapping remain explicit;
otherwise fixtures degrade honestly to inspect-only or external
handoff.**

## Re-use across desktop, CLI, evidence, and companion handoff paths

The browser-runtime labels in this contract are deliberately surface-
agnostic. Desktop UX, CLI runners, evidence readers, hosted-review
readers, support / export readers, automation run review, AI tool
review, and companion handoff packets all read **the same**
`browser_runtime_session_class`, `runtime_identity_class`,
`mutation_action_class`, `mutation_admissibility_class`,
`inspection_lifecycle_state`, `evidence_origin_class`, and
`downgrade_trigger` vocabularies.

Adding a label that a CLI surface needs but a desktop surface doesn't
is **not** admissible at the surface boundary; it lands additive-minor
on this schema and every consumer surface picks it up automatically.

This satisfies the acceptance rule that **browser-runtime labels can
be reused by desktop, CLI, evidence, and companion handoff paths
without alias drift.**

## Composition with the preview-runtime, safe-preview, and execution-context contracts

- The preview-runtime contract stays the source of truth for preview
  mode, mapping confidence, stale-editability, hot-reload state,
  device-target row, and source-sync chip. This session record cites
  `preview_snapshot_record_ref` and never restates those fields.
- The safe-preview contract stays the source of truth for trust class,
  connectivity state, and the first twelve downgrade triggers. This
  session record re-exports those vocabularies and adds browser-
  runtime-specific triggers.
- The execution-context contract stays the source of truth for target
  identity, sandbox posture, and policy epoch. This session record
  cites `execution_context_record_ref` and never restates those
  fields.

If a future change widens the browser-runtime vocabulary, it MUST
land additive-minor on the relevant schema (browser-runtime-specific
values on this schema; safe-preview / preview / execution-context
values on their owning schemas) and bump the corresponding
`*_schema_version` const.

## Change discipline

Adding a new `browser_runtime_session_class`, `runtime_identity_class`,
`browser_engine_family_class`, `device_profile_class`, `theme_class`,
`color_scheme_state_class`, `protocol_scope_class`,
`cross_origin_boundary_class`, `source_map_freshness_class`,
`mutation_action_class`, `mutation_admissibility_class`,
`inspection_lifecycle_state`, `evidence_origin_class`, or
`downgrade_trigger` value on the session schema, or a new
`preview_route_record_class`, `service_identity_class`,
`mapped_port_route_class`, `target_context_class`,
`source_workspace_identity_class`, `actor_class`,
`route_duration_class`, `route_expiry_state_class`,
`route_revoke_posture_class`, `policy_scope_class`, or
`share_visibility_class` value on the route schema, is additive-minor
and bumps the relevant `*_schema_version` const. Repurposing an
existing value is breaking and requires a new decision row.

Re-exporting a vocabulary from another schema is preferred over
minting a parallel one. Where this contract narrows or extends a
re-export, the gate is documented above; if a future contributor
needs to narrow further, that change lands on the owning schema, not
through a private fork in this directory.
