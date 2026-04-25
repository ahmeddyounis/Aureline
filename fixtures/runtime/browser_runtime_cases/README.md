# Browser-runtime, preview-route, and cross-origin / storage review fixtures

Worked-example fixtures for the browser-runtime contract frozen in
[`/docs/runtime/browser_runtime_contract.md`](../../../docs/runtime/browser_runtime_contract.md)
and the boundary schemas at
[`/schemas/runtime/browser_runtime_session.schema.json`](../../../schemas/runtime/browser_runtime_session.schema.json)
and
[`/schemas/runtime/preview_route.schema.json`](../../../schemas/runtime/preview_route.schema.json).

Every fixture carries only opaque service / port / route / device /
origin / source-map / browser-handoff / replay / approval-ticket /
policy-bundle / policy-epoch / actor handles plus monotonic
placeholder timestamps and redaction-aware labels. No raw URLs, raw
schemes, raw hostnames, raw IPs, raw ports, raw paths, raw query
strings, raw cookie values, raw bearer tokens, raw API keys, raw
service-worker script bodies, raw rendered DOM bytes, raw stack
frames, raw absolute filesystem paths, or raw author identity strings
appear in any fixture.

## Preview-route fixtures

| Fixture                                                 | Class                                              | Acceptance bullet covered                                                              |
|---------------------------------------------------------|----------------------------------------------------|----------------------------------------------------------------------------------------|
| `route_user_authored_local_active.yaml`                 | `user_authored_local_preview_route`                | Time-bounded, auditable, preserves source workspace identity.                          |
| `route_managed_workspace_pinned.yaml`                   | `managed_workspace_preview_route`                  | Managed-locked policy scope; managed-admin revoke; org-only share.                     |
| `route_imported_from_browser_handoff.yaml`              | `imported_from_browser_handoff_preview_route`      | Imported route preserves captured target context; no live revoke path.                 |
| `route_ai_tool_proposed_pending_review.yaml`            | `ai_tool_proposed_preview_route_pending_review`    | AI-tool-proposed route forbidden from leaving pending-review state.                    |

## Browser-runtime-session fixtures

| Fixture                                                       | Session class                                          | Acceptance bullet covered                                                                                  |
|---------------------------------------------------------------|--------------------------------------------------------|------------------------------------------------------------------------------------------------------------|
| `session_live_local_browser_workspace_trusted.yaml`           | `live_local_browser_runtime_session`                   | Live mutation admissible only when runtime identity AND source mapping are explicit.                       |
| `session_live_remote_managed_pool.yaml`                       | `live_remote_managed_browser_runtime_session`          | Managed-locked workspace trust; mutation admissible only via approval ticket.                              |
| `session_imported_from_browser_handoff.yaml`                  | `imported_from_browser_handoff_packet_session`         | Inspection lifecycle pinned to `inspection_imported_from_browser_handoff_packet`; no live mutation.        |
| `session_replayed_from_capture.yaml`                          | `replayed_from_capture_session`                        | Inspection lifecycle pinned to `inspection_replayed_from_capture`; mutation actions not applicable.        |
| `session_inspect_only_source_map_stale.yaml`                  | `inspect_only_uncertain_runtime_session`               | Source-map staleness degrades live-style edits and modified-replay to inspect-only honestly.               |
| `session_external_handoff_remote_target_unverifiable.yaml`    | `external_handoff_only_session`                        | Remote target identity unverifiable forces every mutation to external handoff.                             |
| `session_blocked_runtime_identity_unverifiable.yaml`           | `blocked_runtime_identity_unverifiable_session`        | Runtime identity unverifiable forces every mutation entry to a blocked class.                              |

## Cross-cutting acceptance coverage

- **Live mutation only when runtime identity and source mapping are
  explicit.** The live local and live remote managed sessions cite an
  explicit `runtime_identity_class`, a non-stale `source_map_freshness_class`,
  and a non-`runtime_identity_unverifiable_user_review_required` posture.
  The stale-source-map session degrades live-style edits and modified-
  replay to `inspect_only_source_mapping_uncertain`. The unverifiable
  remote target and unverifiable runtime sessions degrade to external
  handoff and blocked, respectively.
- **Preview / route fixtures are time-bounded, auditable, and preserve
  source-workspace or remote-target identity.** Every route fixture
  carries `route_duration_class`, `route_expiry_state_class`,
  `route_revoke_posture_class`, `target_context_class`, and
  `source_workspace_identity_class`. Imported and AI-tool-proposed
  routes pin the appropriate captured / blocked posture.
- **Browser-runtime labels reusable by desktop, CLI, evidence, and
  companion handoff paths.** Every fixture uses the same
  `browser_runtime_session_class`, `runtime_identity_class`,
  `mutation_action_class`, `mutation_admissibility_class`,
  `inspection_lifecycle_state`, `evidence_origin_class`, and
  `downgrade_trigger` vocabularies.
- **Inspection add-ons declare live, imported, replayed, or blocked
  state through the same browser-runtime contract.** The
  `inspection_lifecycle_state` field on every session fixture pins the
  add-on's lifecycle to a value drawn from the closed nine-value
  vocabulary. No fixture invents a parallel label.
