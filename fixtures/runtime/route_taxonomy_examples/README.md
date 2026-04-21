# Action route-truth fixtures

Worked fixtures for the origin / target / route / exposure /
route-change / authority-linkage taxonomy frozen in
[`/docs/runtime/origin_target_route_taxonomy.md`](../../../docs/runtime/origin_target_route_taxonomy.md).
Every fixture here binds to a row in the
[`action_origin_target_labels.yaml`](../../../artifacts/runtime/action_origin_target_labels.yaml)
matrix and carries the exact frozen tokens from those six
vocabularies.

The fixtures exist so the command router, shell adapter, CLI
surface, AI tool-call plane, connected-provider adapters,
browser-handoff launcher and callback receiver, publish pipeline,
tunnel exposer, managed-workspace control plane, support-export
lane, mutation-journal, replay, and evidence-packet lanes can
write against a shared corpus without inventing per-surface route
fields. Each fixture carries a `__fixture__` section summarising
the scenario, the axes it exercises, and the contract sections it
illustrates.

## Intended usage

- **Contract conformance.** Every fixture MUST carry the frozen
  tokens from the six vocabularies in the taxonomy doc. Surface
  authors compare their emitted route-truth packet against the
  matching fixture field-for-field.
- **Projection-parity corpus.** A later parity audit between UI
  disclosure, CLI transcript, support export, mutation-journal
  entry, and evidence packet compares emitted rows for the same
  `invocation_session_id` against the fixture's declared token
  set.
- **Replay, audit, rollback.** Replay and audit tooling uses the
  fixtures as the reference envelopes every call site is
  expected to mint, so wrong-target and route-changed cases are
  detectable without re-deriving route truth.

## Required axes per fixture

A fixture MUST carry:

- `action_origin_class`, `action_target_class`,
  `action_route_class`, `action_exposure_class`,
  `route_change_reason_code`, `authority_linkage_class`,
- `invocation_session_id`, `command_id`, `command_revision_ref`,
- `action_origin_ref` (with `origin_actor_class` non-null when
  the target is `connected_provider_target` or the origin is a
  provider-callback or webhook),
- `action_target_ref` (with the right non-null slot for the
  target class: `execution_context_id` for ADR-0009 execution
  targets, `connected_provider_record_id` for providers,
  `browser_handoff_packet_ref` for system-browser /
  native-callback / embedded-webview, `publish_evidence_packet_ref`
  for publish, `tunnel_session_ref` for tunnel,
  `managed_workspace_instance_ref` for managed,
  `bridged_helper_ref` for bridged helpers),
- `host_boundary_cue_stack` (outermost-to-innermost, matching
  the target class),
- `exposure_disclosure_summary` (non-empty human-legible
  paragraph; tooltip-only disclosure is non-conforming),
- `authority_linkage_ref` (with the right non-null slot for the
  linkage class, or null everywhere when the class is
  `no_authority_required_read_only`, `local_user_keystroke_authority`,
  or `authority_missing_denied`),
- `prior_target_ref` / `prior_route_class` (non-null on every
  `route_changed_*` reason that the matrix flags as
  `requires_prior_target_ref: true`),
- `policy_context`, `redaction_class`,
  `export_inclusion_posture`, `freshness_class`, `minted_at`,
- `audit_event_refs` and `evidence_refs` arrays.

## Fixtures

### Read-only local

- [`local_read_search_in_workspace.yaml`](./local_read_search_in_workspace.yaml)
  — CLI-invoked `search.find_in_workspace` against the local
  index. Exercises `no_side_effect_local_read` exposure with
  `no_authority_required_read_only` linkage.

### Local mutating

- [`local_mutating_save_dirty_buffer.yaml`](./local_mutating_save_dirty_buffer.yaml)
  — user-keystroke `editor.save` crossing a local RPC route to
  the workspace VFS. Exercises `local_only_mutation` exposure
  with `local_user_keystroke_authority` linkage.

### Approval-gated provider publish

- [`approval_gated_git_push_to_provider.yaml`](./approval_gated_git_push_to_provider.yaml)
  — user-keystroke `git.push_branch` whose proposed
  `local_rpc_route` escalated to `approval_gated_route` against a
  `connected_provider_target`. Exercises
  `route_escalated_to_approval_required` reason and the
  `approval_ticket_linked` linkage tying back to the ADR-0010
  approval-ticket record.

### Browser-mediated OAuth callback return

- [`browser_mediated_oauth_callback_return.yaml`](./browser_mediated_oauth_callback_return.yaml)
  — inbound `provider_callback_inbound` origin arriving through
  a `provider_webhook_return_route` to an
  `embedded_webview_target`. Exercises
  `browser_session_visible` exposure and the
  `browser_handoff_packet_linked` linkage that closes out a
  previously-minted `browser_handoff_packet`.

### Wrong-target detected

- [`wrong_target_detected_managed_workspace_drift.yaml`](./wrong_target_detected_managed_workspace_drift.yaml)
  — user-keystroke `workspace.resume_from_idle` that resolved
  against one managed-workspace instance and detected drift
  mid-invocation. Exercises
  `route_changed_wrong_target_detected` reason, carries both
  `prior_target_ref` and the corrected
  `action_target_ref.managed_workspace_instance_ref`, and
  preserves `prior_route_class`.

### Route changed to browser handoff

- [`route_changed_to_browser_handoff_publish.yaml`](./route_changed_to_browser_handoff_publish.yaml)
  — user-keystroke `provider.publish_release_notes` whose
  proposed route was `approval_gated_route` to a
  `connected_provider_target`; the mutation is not reachable
  locally so the route escalated to `browser_handoff_route` to a
  `system_browser_target`. Exercises
  `route_escalated_to_browser_handoff` reason and the
  `browser_handoff_packet_linked` linkage.

## Related schemas and artifacts

- [`/schemas/runtime/execution_context.schema.json`](../../../schemas/runtime/execution_context.schema.json)
  — re-exports `target_class`, the scope / authority envelope,
  and the execution-context id every route packet quotes.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json)
  — re-exports `destination_class`, `provider_actor_class`, and
  the browser-handoff packet the browser-mediated fixtures
  reference.
- [`/schemas/integration/approval_ticket.schema.json`](../../../schemas/integration/approval_ticket.schema.json)
  — re-exports the approval-ticket record the approval-gated
  fixture references.
- [`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json)
  — the command-descriptor and invocation-session envelope the
  route-truth packet cross-walks against.
- [`/artifacts/runtime/action_origin_target_labels.yaml`](../../../artifacts/runtime/action_origin_target_labels.yaml)
  — the machine-readable matrix binding every frozen token to
  its minimum fields, admissible companions, required
  redaction, and conformance tests.
