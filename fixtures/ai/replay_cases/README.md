# AI evidence-replayability worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/evidence_replayability_contract.md`](../../../docs/ai/evidence_replayability_contract.md)
and the schemas at
[`/schemas/ai/evidence_replay_packet.schema.json`](../../../schemas/ai/evidence_replay_packet.schema.json)
and
[`/schemas/ai/audit_storage_manifest.schema.json`](../../../schemas/ai/audit_storage_manifest.schema.json).

Every file is a JSON document carrying a `__fixture__` prelude
summarising the scenario, the contract sections it exercises,
and the record kinds it produces, plus a `records` array
containing individual `ai_replay_packet_record`,
`ai_replay_capture_coverage_record`,
`ai_replay_provider_availability_record`,
`ai_replay_mutation_lineage_record`,
`ai_replay_audit_event_record`,
`ai_audit_storage_manifest_record`,
`ai_audit_storage_artifact_record`, and
`ai_audit_storage_audit_event_record` instances that conform to
the schemas.

No fixture embeds raw prompt text, raw document bodies, raw
terminal / log bodies, raw generated artifact bytes, raw
request / response payloads, raw user-supplied text, raw URLs,
raw paths, or raw credential material. Every such field is an
opaque ref.

## Cases

- [`full_replay_inline_composer.json`](./full_replay_inline_composer.json)
  — inline composer turn whose assembly, evidence packet, route /
  spend receipts, tainted-content fences, mutation-journal
  entries, interaction-safety packets, and running-build identity
  are all retained and whose provider / model identity remains
  reachable at `reachable_same_identity_same_version`. Every
  required capture class is either `required_captured`,
  `required_captured_reference_only`, or
  `required_omitted_with_disclosure` (for raw bodies the contract
  never stores). Replay grade = `full_replay`.
- [`partial_replay_provider_unavailable.json`](./partial_replay_provider_unavailable.json)
  — composer turn from an older release whose model identity has
  since been retired by the vendor. The assembly, evidence packet,
  receipts, tainted fences, mutation lineage, and context set all
  remain retained; `provider_identity` and `model_identity` are
  marked `required_captured_reference_only` but the provider
  availability record grades them `unreachable_retired`. Replay
  grade = `partial_replay_provider_unavailable` with
  `degraded_capture_classes` = `[provider_identity,
  model_identity]`.
- [`non_replayable_raw_byte_dependent.json`](./non_replayable_raw_byte_dependent.json)
  — composer turn whose behavior depended on a user-supplied
  file pasted inline that the user did not re-offer and whose raw
  bytes the contract never retained. The attachment set is
  captured reference-only, but the user-supplied-text capture class
  is marked `required_omitted_with_disclosure` with
  `raw_user_supplied_text_forbidden_unless_separately_retained`.
  Replay grade = `non_replayable_raw_byte_dependent`; the packet
  preserves evidence honesty — what is known, what was omitted,
  and why.
