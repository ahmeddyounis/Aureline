# AI memory, delete / export, and deferred-intent reconciliation worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/memory_and_reconciliation_contract.md`](../../../docs/ai/memory_and_reconciliation_contract.md)
and the schemas at
[`/schemas/ai/memory_object.schema.json`](../../../schemas/ai/memory_object.schema.json)
and
[`/schemas/ai/deferred_intent_outbox.schema.json`](../../../schemas/ai/deferred_intent_outbox.schema.json).

Every file is a JSON document carrying a `__fixture__` prelude
summarising the scenario, the contract sections it exercises, and
the record kinds it produces, plus a `records` array containing
individual `ai_memory_object_record`,
`ai_memory_invalidation_event_record`,
`ai_memory_delete_request_record`,
`ai_memory_export_assembly_record`,
`ai_memory_audit_event_record`, `deferred_intent_record`,
`deferred_intent_state_transition_record`,
`deferred_intent_reconciliation_record`, and
`deferred_intent_audit_event_record` instances that conform to the
schemas.

No fixture embeds raw conversation text, raw prompt bodies, raw
retrieved-document bodies, raw tool return bytes, raw embedding
vectors, raw URLs, raw absolute paths, raw user identifiers, raw
billing-account ids, raw API keys, raw OAuth tokens, raw mTLS
material, raw model weights, raw pack bytes, or raw provider
payloads. Every such field is an opaque ref.

## Cases

- [`delete_thread_per_class_outcomes.json`](./delete_thread_per_class_outcomes.json)
  — user-initiated 'Delete this thread' against one
  `conversation_history_user_visible` row, two
  `derived_cache_regeneratable` rows, three
  `embedding_row_regeneratable` rows, and one
  `retained_evidence_packet_copy` row. Conversation history and
  derived rows are cleared; the evidence-packet copy is preserved
  under `evidence_packet_overrides`. Demonstrates per-class
  delete outcomes and the partial-completion path.
- [`embedder_change_invalidation.json`](./embedder_change_invalidation.json)
  — embedder identity bump fires `embedder_identity_changed`
  against an `embedding_row_regeneratable` row. Outcome is
  `row_invalidated_and_marked_for_regeneration`; the row is
  re-derived on the next read.
- [`clear_workspace_ai_state_with_legal_hold.json`](./clear_workspace_ai_state_with_legal_hold.json)
  — admin-initiated 'Clear AI state for this workspace' against a
  workspace whose conversation history is under
  `administrative_legal` hold. The request is queued
  (`blocked_on_hold_request_queued`); reusable repo facts are
  cleared; the user-pinned saved memory and the evidence-packet
  copy are preserved with their typed reasons.
- [`per_class_user_self_service_export.json`](./per_class_user_self_service_export.json)
  — 'Export my AI data' user-self-service flow. Includes
  conversation history, user-pinned saved memory, and the
  retained evidence-packet copy. Excludes derived caches /
  embeddings (`regeneratable_excluded_by_design`) and
  workspace-scoped reusable facts
  (`local_authority_only_not_in_managed_export`).
- [`deferred_intent_revalidate_before_send.json`](./deferred_intent_revalidate_before_send.json)
  — read-only `provider_call_invocation` queued under one
  policy_epoch, then transitioned to
  `queued_revalidate_before_send_required` after
  `policy_epoch_rolled`. Demonstrates revalidation-posture
  attachment (`revalidate_policy_epoch_only`).
- [`deferred_intent_replay_forbidden_mutating.json`](./deferred_intent_replay_forbidden_mutating.json)
  — mutating `agent_step_invocation` forbidden to replay after
  the rollout state demoted (`rollout_state_demoted_or_withdrawn_non_replayable`).
  Reconciliation outcome = `dropped_replay_forbidden`; paired
  audit denial event names
  `mutating_intent_replay_against_changed_binding_forbidden`.
- [`deferred_intent_sent_after_revalidation.json`](./deferred_intent_sent_after_revalidation.json)
  — companion to `deferred_intent_revalidate_before_send.json`.
  The read-only intent successfully revalidates against the new
  policy_epoch and transitions to `fully_reconciled_completed`.
  Reconciliation outcome = `sent_against_revalidated_environment`
  with `environment_axes_changed = [policy_epoch_rolled]` and
  paired `route_receipt_ref` / `evidence_packet_ref`.
