# AI context-assembly worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/context_assembly_contract.md`](../../../docs/ai/context_assembly_contract.md)
and the schemas at
[`/schemas/ai/context_assembly.schema.json`](../../../schemas/ai/context_assembly.schema.json)
and
[`/schemas/ai/evidence_packet.schema.json`](../../../schemas/ai/evidence_packet.schema.json).

Every file is a multi-document YAML stream. The first document is
a `__fixture__` prelude summarising the scenario, the contract /
ADR sections it exercises, and the record kinds it produces. The
remaining documents are individual `ai_context_assembly_record`,
`ai_context_segment_record`, `prompt_composer_session_record`,
`prompt_composer_mention_record`,
`prompt_composer_attachment_record`,
`prompt_composer_turn_draft_record`, `ai_route_plan_record`,
`ai_spend_plan_record`, `ai_route_receipt_record`,
`ai_spend_receipt_record`, `ai_tool_call_lineage_record`,
`ai_branch_agent_dispatch_record`,
`ai_context_audit_event_record`, `ai_evidence_packet_record`,
`ai_tainted_content_fence_record`,
`ai_evidence_source_reference_record`, and
`ai_evidence_audit_event_record` instances that conform to the
schemas.

No fixture embeds raw prompt text, raw document bodies, raw
terminal / log bodies, raw generated artifact bytes, raw
request / response payloads, raw user-supplied text, raw URLs,
raw paths, or raw credential material. Every such field is an
opaque ref.

## Cases

- [`composer_turn_with_tainted_retrieved_document.yaml`](./composer_turn_with_tainted_retrieved_document.yaml)
  — inline composer turn where the user attaches a
  retrieved_document pulled through a connected provider. The
  assembly places the document as a tainted segment under the
  `quoted_as_data_only` fence, preserves the full
  tainted-usage-constraint set, carries matching
  `ai_tainted_content_fence_record` entries on the evidence
  packet, and records a `tool_call_lineage` entry whose result
  flowed into the tainted segment. A separate workspace_symbol
  segment is pinned by the composer plan; a docs_pack_excerpt
  segment carries a citation-anchor ref; a policy-blocked
  segment names its typed block_reason; an omitted segment names
  its typed omit_reason; a redacted segment names its typed
  redaction_reason set.
- [`background_branch_agent_inherits_taint.yaml`](./background_branch_agent_inherits_taint.yaml)
  — background branch-agent dispatch originating from the
  same composer session. The turn draft's
  `dispatch_target_class` is `background_branch_agent`; the
  `ai_branch_agent_dispatch_record` inherits every
  tainted-usage-constraint from the originating assembly, the
  originating assembly's `scope_filter_class`, and the
  originating assembly's `redaction_class`. The nested evidence
  packet preserves the tainted fence and records a
  `ai_evidence_tainted_fence_preserved_on_handoff` audit event
  so downstream review / support can confirm the constraint set
  survived the handoff without silent downgrade.
