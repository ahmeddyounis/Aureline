# AI context-assembly, route / spend truth, evidence-packet, and tainted-context contract

This document is the **product-wide contract** for how AI turns are
assembled, how included / omitted / pinned / redacted / policy-blocked
/ tainted context segments are named, how provider / model / route /
spend truth is carried, how evidence packets link back to the
assembly, and how tainted-content fences survive across composer,
inline, background branch-agent, and review-handoff dispatches.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / branch-
agent surface's mint of its own copy, this document wins and the
surface is non-conforming.

The companion artifacts are:

- [`/schemas/ai/context_assembly.schema.json`](../../schemas/ai/context_assembly.schema.json)
  — boundary schema every non-owning surface reads for the
  `ai_context_assembly_record`, `ai_context_segment_record`,
  `prompt_composer_session_record`,
  `prompt_composer_mention_record`,
  `prompt_composer_attachment_record`,
  `prompt_composer_turn_draft_record`,
  `ai_route_plan_record`, `ai_spend_plan_record`,
  `ai_route_receipt_record`, `ai_spend_receipt_record`,
  `ai_tool_call_lineage_record`,
  `ai_branch_agent_dispatch_record`, and
  `ai_context_audit_event_record` shapes.
- [`/schemas/ai/evidence_packet.schema.json`](../../schemas/ai/evidence_packet.schema.json)
  — boundary schema for the `ai_evidence_packet_record`,
  `ai_tainted_content_fence_record`,
  `ai_evidence_source_reference_record`, and
  `ai_evidence_audit_event_record` shapes.
- [`/fixtures/ai/context_assembly_cases/`](../../fixtures/ai/context_assembly_cases/)
  — worked-example corpus covering at least one composer turn with
  tainted retrieved context and one background branch-agent
  dispatch that preserves the originating assembly's trust posture.

This contract rides alongside — it does not re-mint — the
vocabularies already frozen in:

- `docs/adr/0001-identity-modes.md` — workspace-trust state rides
  the `policy_context.trust_state` on every record.
- `docs/adr/0004-rpc-transport-and-schema-toolchain.md` — every
  record in this contract crosses the RPC boundary as a typed
  payload.
- `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
  — projected composer / assembly views ride the shared
  subscription envelope; AI output is `derived_knowledge`
  authority.
- `docs/adr/0006-vfs-save-cache-identity.md` — workspace file
  slices included as context segments quote VFS identity refs
  only.
- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
  — the broker-owned redaction pass runs before any segment's
  bytes reach the provider; raw credential material never reaches
  the model input and never appears in evidence packets.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
  — admin policy MAY narrow which providers, routes, spend
  ceilings, and context sources the composer is permitted to use;
  policy MAY NOT silently widen.
- `docs/adr/0009-execution-context-and-scope.md` —
  `scope_filter_class` / `execution_context_id` re-exported; this
  contract never invents a parallel scope vocabulary.
- `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`
  — connected-provider payloads ride the connected-provider
  vocabulary; browser-handoff-return values are treated as
  `untrusted_external` by default and carry the tainted fence.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  — `freshness_class`, `client_scope`, `redaction_class`
  re-exported without modification.
- `docs/adr/0012-extension-manifest-permission-publisher-policy.md`
  — extension-proposed context rides the extension effective-
  permission surface; the composer never promotes
  extension-proposed bytes to trusted instruction authority.
- `docs/adr/0013-docs-help-service-health-truth.md` —
  `citation_anchor_record` is the only anchor vocabulary; every
  authoritative docs / generated-reference / runbook / release-
  note quote on an AI turn MUST carry a citation-anchor ref.
- `docs/adr/0014-search-readiness-ranking-result-truth.md` —
  `search_result_packet_record` is the only search-result
  vocabulary; AI turns that retrieve workspace facts quote that
  packet by id rather than re-deriving its truth.
- `docs/ux/shell_interaction_safety_contract.md` —
  `interaction_safety_packet_record` is the only
  preview / apply / revert vocabulary; AI apply rides the
  `ai_apply_canvas` surface and quotes the packet by id.

## Who reads this document

- **AI / prompt-composer / context-resolver authors** minting the
  assembly, placing segments, resolving routes, computing spend,
  and preserving taint across dispatch boundaries.
- **Review / diff / support / evidence surface authors** reading
  the assembly, the route / spend receipts, and the tainted-fence
  set without re-deriving trust.
- **Branch-agent / background dispatch authors** inheriting
  assembly identity, tainted-usage constraints, scope, and
  redaction class across the dispatch boundary.
- **Extension / connected-provider / collaboration surface
  authors** whose context contributions are labelled with a
  trust posture rather than promoted to authority by default.
- **Support, parity-audit, and claim-manifest tooling** reading
  every axis (segment class, trust posture, fence strategy, route
  class, spend class, citation-anchor coverage) mechanically.

## One contract, five turn classes, one record shape

The contract applies uniformly to the five turn classes below. A
surface that mints a private segment vocabulary, its own
trust-posture set, its own fence strategy, its own route / spend
labels, or its own evidence-packet shape is non-conforming.

| Turn class                                 | Typical origin                                                                                          |
|--------------------------------------------|---------------------------------------------------------------------------------------------------------|
| `inline_composer`                          | Foreground composer-driven turn; applies or proposes against the active workspace / editor canvas.      |
| `background_branch_agent`                  | Long-running background branch-agent dispatch; owns its own branch / review handoff.                    |
| `review_handoff`                           | Composer-driven turn whose output is handed off to a review / diff canvas instead of applied inline.    |
| `tool_follow_up`                           | Composer-driven turn emitted after a tool call returned; the return value is fenced if untrusted.       |
| `support_replay_only`                      | Assembly reconstructed for support / parity audit; never dispatched; spend / route receipts are absent. |

Every AI turn on the list above — and every future dispatch
target that inherits a protected posture — emits exactly one
`ai_context_assembly_record` and (on dispatch) exactly one
`ai_evidence_packet_record`. Per-surface chips, sheets, and
summary badges are UI freedoms that MUST collapse back into these
packets' addressable axes on every boundary (mutation journal,
support export, claim manifest, review handoff, nested evidence
packet).

## Core axes (frozen)

Every `ai_context_assembly_record` names the following axes, and
every `ai_context_segment_record` names exactly one value from
each segment-level axis. Adding a value is additive-minor and
bumps `ai_context_assembly_schema_version`; repurposing a value is
breaking and requires a new decision row.

### Segment class

How the segment is represented on the assembly.

- `included`
- `omitted`
- `pinned`
- `redacted`
- `policy_blocked`
- `tainted`

Rules (frozen):

1. The segment class names the **posture of inclusion**, not the
   segment's origin. A user-pinned workspace symbol is
   `pinned`; a policy-blocked user-supplied file is
   `policy_blocked` regardless of the user's intent; a
   retrieved-document excerpt placed under a tainted fence is
   `tainted` even when the user explicitly added it.
2. Every segment whose class is `omitted` MUST carry a typed
   `omit_reason`; a surface that silently drops a segment (no
   segment record, no omit reason) is non-conforming.
3. Every segment whose class is `redacted` MUST carry at least
   one `redaction_reason`; the broker-owned redaction pass
   (ADR-0007) runs before the provider sees the bytes.
4. Every segment whose class is `policy_blocked` MUST carry a
   typed `block_reason`; free-form block strings are
   non-conforming.
5. Every segment whose class is `tainted` MUST carry exactly one
   `tainted_fence_strategy`, at least one
   `tainted_usage_constraint`, and the instructional role
   `fenced_tainted_data`. A tainted segment placed in any other
   role is non-conforming.
6. Every segment whose class is `pinned` MUST carry a
   `pinned_by_actor_ref`. Pinned segments survive budget
   pressure; a resolver that silently drops a pinned segment is
   non-conforming.

### Source class

Where the segment's bytes came from. The set is closed.

- `workspace_symbol`
- `workspace_file_slice`
- `workspace_buffer_slice`
- `workspace_graph_summary`
- `workspace_search_result`
- `workspace_diagnostics`
- `workspace_mutation_journal`
- `docs_pack_excerpt`
- `generated_reference_excerpt`
- `runbook_step`
- `release_note_excerpt`
- `citation_anchor_quote`
- `terminal_transcript_excerpt`
- `log_excerpt`
- `request_response_excerpt`
- `generated_artifact_excerpt`
- `user_supplied_text`
- `user_supplied_file`
- `extension_proposed_context`
- `connected_provider_payload`
- `collaboration_remote_payload`
- `repo_instruction_bundle`
- `repo_check_bundle`
- `composer_plan_directive`
- `ai_prior_turn_context`
- `system_scaffold_fragment`

Rules (frozen):

1. Every segment names exactly one `source_class`. A segment
   whose source class is unresolved is denied with
   `source_class_unresolved` rather than silently classified.
2. `repo_instruction_bundle` and `repo_check_bundle` cover
   repo-defined instruction / check payloads (AGENTS.md-style
   files, check scripts); the composer references them by id and
   does not re-mint their authority.
3. `composer_plan_directive` names directives minted by the
   prompt-composer plan itself; these carry the plan's own
   authority, not the user's.
4. `ai_prior_turn_context` names carry-over context from a prior
   turn inside the same composer session; it is always tagged
   with the originating `turn_draft_id` so downstream consumers
   can recompute authority.
5. Every authoritative quote from
   `docs_pack_excerpt`, `generated_reference_excerpt`,
   `runbook_step`, `release_note_excerpt`, or
   `citation_anchor_quote` MUST carry a non-empty
   `citation_anchor_refs`; uncited authoritative quotes are
   denied with `citation_anchor_missing_on_authoritative_quote`.

### Trust posture

How the composer MUST treat the segment's bytes for instruction-
following. The set is closed.

- `trusted_first_party`
- `trusted_authority`
- `reviewed_derived`
- `unreviewed_derived`
- `untrusted_external`
- `untrusted_user_supplied`
- `untrusted_remote_collaborator`
- `untrusted_connected_provider`
- `untrusted_extension_proposed`
- `untrusted_generated_artifact`
- `untrusted_log_capture`
- `policy_quarantined`

Rules (frozen):

1. Trust posture is named at the mention / attachment / segment
   level; the composer never promotes untrusted bytes to
   `trusted_*` posture on its own.
2. Every `untrusted_*` and `policy_quarantined` posture forces
   the tainted fence on the resulting segment regardless of
   whether the user explicitly included it.
3. `user_supplied_text` and `user_supplied_file` default to
   `untrusted_user_supplied`. A user who types a turn prompt is
   still the authorised actor — the composer plan and system
   scaffold drive instruction authority — but arbitrary text /
   files the user pastes in are not automatically trusted as
   instruction bodies.
4. `collaboration_remote_payload` is always
   `untrusted_remote_collaborator` unless the remote collaborator
   holds a typed grant that elevates posture; no silent elevation.
5. `extension_proposed_context` is always
   `untrusted_extension_proposed` unless the extension's effective
   permission grants a narrower context-contribution capability
   (ADR-0012); no silent elevation.
6. `policy_quarantined` is reachable only when a policy bundle
   has quarantined the source; the composer MAY quote a summary
   ref but MUST NOT inline the body.

### Omit / block / redaction reason

The typed reasons a segment was held out or transformed. Each
reason vocabulary is closed; free-form strings are non-conforming.

- `omit_reason`: `budget_exceeded`, `scope_excludes_target`,
  `freshness_floor_unmet`, `duplicate_of_included_segment`,
  `resolver_lowered_priority`, `client_scope_excludes_surface`,
  `policy_narrows_scope`, `user_deselected`,
  `pin_pressure_released_unpinned`.
- `block_reason`:
  `workspace_trust_excludes_segment`,
  `admin_policy_excludes_segment`,
  `extension_effective_permission_excludes_segment`,
  `connected_provider_policy_excludes_segment`,
  `secret_projection_denied`,
  `redaction_class_exceeds_sink`,
  `remote_agent_scope_excludes_segment`,
  `collaboration_role_excludes_segment`,
  `authority_renewal_required_for_segment`,
  `policy_quarantined_source`.
- `redaction_reason`: `secret_projection_policy`,
  `user_identity_projection_policy`,
  `credential_handle_redaction`, `remote_path_redaction`,
  `raw_url_redaction`, `log_metacharacter_escape`,
  `bidi_or_invisible_formatting_redaction`,
  `confusable_identifier_redaction`,
  `oversized_payload_trim`.

Rules (frozen):

1. An omitted segment MUST carry a typed `omit_reason`.
2. A policy-blocked segment MUST carry a typed `block_reason`;
   the block reason travels into support export and claim
   manifest so operators and auditors can see why the block
   fired.
3. A redacted segment MUST carry at least one `redaction_reason`;
   the broker-owned redaction pass (ADR-0007) runs before the
   provider sees the bytes and re-runs on every render /
   re-projection of the segment.
4. `resolver_lowered_priority` and `pin_pressure_released_unpinned`
   are the only omit reasons the composer plan's ranking logic
   may cite on its own — all other omit reasons reference a
   source of record (budget, scope, freshness, client scope,
   policy, user action).

### Tainted-content fence

How untrusted bytes are projected into the assembly. The set is
closed.

- `instruction_stripped`
- `quoted_as_data_only`
- `sandboxed_excerpt`
- `summary_only_no_body`
- `metadata_only_no_content`
- `citation_reference_only`

Rules (frozen):

1. Every tainted segment carries exactly one `fence_strategy`.
2. `quoted_as_data_only` is the default fence for retrieved
   documents, terminal / log snippets, request / response
   payloads, and user-supplied text; the composer plan MAY
   escalate to `summary_only_no_body` or
   `metadata_only_no_content` under policy pressure but MAY NOT
   silently downgrade.
3. `instruction_stripped` is permitted only on inputs whose
   structure admits a clean instruction strip (e.g. a recognised
   document type with a known header / body split); it MAY NOT
   be applied to free-form text.
4. `citation_reference_only` is permitted only when the segment
   has an authoritative citation anchor; it is the fence strategy
   that survives policy quarantine on otherwise-authoritative
   sources.

### Tainted-usage constraints

Downstream constraints the composer plan applied at assembly
time. The set is closed.

- `must_not_gain_tool_permission`
- `must_not_escalate_scope`
- `must_not_mint_citations`
- `must_not_override_instruction_bundle`
- `must_not_publish_externally`
- `must_not_commit_to_repo`
- `must_not_dispatch_branch_agent`
- `must_not_route_to_higher_cost_tier`
- `must_preserve_fence_in_downstream_packet`

Rules (frozen):

1. Every tainted segment carries at least one usage constraint;
   a segment without constraints is denied with
   `tainted_usage_constraint_missing`.
2. Constraints survive into downstream branch-agent dispatches,
   review handoffs, and nested evidence packets without
   re-inference. A handoff that drops any constraint is denied
   with `tainted_fence_dropped_on_handoff`.
3. `must_preserve_fence_in_downstream_packet` is applied by
   default on every tainted segment; explicit opt-out requires a
   decision row.
4. A tainted segment may never escalate `trust_posture` later in
   the session; re-deriving the same bytes under a different
   posture requires a fresh segment id and a fresh provenance
   chain.

### Instructional role

Why the segment was placed and at what authority. The set is
closed.

- `system_scaffold`
- `composer_plan_directive`
- `repo_instruction_bundle`
- `repo_check_bundle`
- `user_turn_prompt`
- `user_prior_turn_prompt`
- `ai_prior_turn_response`
- `tool_call_request`
- `tool_call_response`
- `retrieved_reference`
- `attached_data`
- `fenced_tainted_data`

Rules (frozen):

1. Every included, pinned, or redacted segment names exactly one
   instructional role.
2. `fenced_tainted_data` is the only role permitted for tainted
   segments.
3. The ordering of authority is (highest to lowest):
   `system_scaffold` > `composer_plan_directive` >
   `repo_instruction_bundle` > `user_turn_prompt` /
   `user_prior_turn_prompt` > `ai_prior_turn_response` /
   `tool_call_response` / `retrieved_reference` /
   `attached_data` > `fenced_tainted_data`. `fenced_tainted_data`
   never gains instruction authority regardless of its source.

### Provider / route / cost / spend axes

Route and spend truth the composer plan names before dispatch.

- `provider_class`: `first_party_self_hosted`,
  `first_party_managed`, `connected_provider_vendor`,
  `connected_provider_self_hosted`,
  `extension_provided_provider`, `mocked_test_provider`,
  `disabled_no_provider`.
- `route_path_class`: `direct_first_party`,
  `vendor_cloud_direct`, `vendor_cloud_via_broker`,
  `self_hosted_direct`, `self_hosted_via_broker`,
  `extension_mediated`, `offline_cached_only`,
  `denied_by_policy`.
- `cost_visibility_class`: `metered_per_request`,
  `metered_per_token`, `flat_fee_subscription`,
  `bundled_no_incremental_cost`, `estimated_unverified`,
  `undisclosed_by_provider`.
- Receipts: `ai_route_receipt_record` and
  `ai_spend_receipt_record` record the actual route / spend for
  every dispatched turn.

Rules (frozen):

1. Every assembly names exactly one `provider_class`,
   `route_path_class`, and `cost_visibility_class`; surfaces that
   fold these into a single chip still emit each axis as a
   separately addressable field.
2. `disabled_no_provider` and `denied_by_policy` are dispatch-
   denying states; an assembly in either state MUST NOT be
   dispatched and denies with `provider_disabled_by_policy` or
   `route_denied_by_policy`.
3. `undisclosed_by_provider` is permitted only when the provider
   does not expose cost metadata; surfaces render this state
   verbatim rather than fabricating a per-request cost.
4. Every dispatched turn produces an `ai_route_receipt_record`
   and an `ai_spend_receipt_record`. A dispatched turn without
   either receipt is denied with `route_receipt_missing` or
   `spend_receipt_missing` at audit time.
5. `estimated_cost_units` on the route / spend plan is an
   estimate; the receipt may differ. Both are preserved so
   downstream review can compare planned-vs-actual.
6. Spend ceilings from the plan are enforced before dispatch;
   exceeding the ceiling denies with `spend_ceiling_exceeded`
   and reopens the plan rather than silently rerouting.

### Composer session, mention, attachment, turn-draft slots

Reserved shapes the composer owns and every downstream surface
reads by id.

- `prompt_composer_session_record` — one per composer session;
  names the `composer_plan_ref` and the `request_workspace_ref`
  (the isolated staging scope where composer drafts accumulate
  before dispatch) and the `scope_filter_class` the session
  operates under.
- `prompt_composer_mention_record` — typed mentions the user
  places inside a turn draft (`symbol_mention`, `file_mention`,
  `workset_mention`, `execution_context_mention`,
  `search_result_mention`, `docs_anchor_mention`,
  `citation_anchor_mention`, `runbook_step_mention`,
  `diagnostic_mention`, `terminal_transcript_mention`,
  `log_span_mention`, `request_response_mention`,
  `generated_artifact_mention`, `review_thread_mention`,
  `branch_agent_dispatch_mention`,
  `connected_provider_resource_mention`,
  `collaboration_participant_mention`,
  `extension_resource_mention`). Raw strings that happen to look
  like a path or URL are not mentions and are never promoted to
  one silently.
- `prompt_composer_attachment_record` — typed attachments
  (`retrieved_document`, `terminal_log_capture`,
  `generated_artifact_excerpt`, `request_response_payload`,
  `user_supplied_text`, `user_supplied_file`,
  `citation_anchor_bundle`, `diagnostic_bundle`,
  `workspace_slice_bundle`, `branch_agent_evidence_bundle`).
  Every attachment names its own `trust_posture`; the composer
  never assumes attached content is trusted unless the
  attachment kind and source establish that posture.
- `prompt_composer_turn_draft_record` — the unit of work
  dispatched. `draft_state` moves through `draft` /
  `queued_for_dispatch` / `dispatched_inline` /
  `dispatched_branch_agent` / `dispatched_review_handoff` /
  `cancelled` / `superseded`. `dispatch_target_class`,
  `route_plan_ref`, `spend_plan_ref`,
  `branch_agent_dispatch_ref`, and `review_handoff_ref` are
  named on the draft; the assembly quotes them by ref.

Rules (frozen):

1. Every turn draft references exactly one composer session and
   one composer plan. A draft without a session is denied with
   `composer_session_ref_missing`.
2. A terminal turn draft (`cancelled`, `superseded`) MAY NOT
   mint a new assembly; attempts deny with
   `turn_draft_terminal_cannot_dispatch`.
3. Drafts dispatched as `background_branch_agent` carry a
   `branch_agent_dispatch_ref`; drafts dispatched as
   `review_handoff` carry a `review_handoff_ref`.
4. Supersession is explicit. A draft that replaces a prior draft
   names its `supersedes_turn_draft_ref`; the superseded draft
   is not deleted, so the audit trail of "this draft existed and
   was replaced" survives.

## Record shapes

The schemas at
`schemas/ai/context_assembly.schema.json` and
`schemas/ai/evidence_packet.schema.json` freeze the following
record shapes. This section summarises; the schemas are
authoritative.

### `ai_context_assembly_record`

Envelope for one dispatched (or replay-only) AI turn. Required
fields (all separately addressable):

- `assembly_id` (opaque)
- `composer_session_ref`
- `composer_plan_ref`
- `request_workspace_ref`
- `turn_draft_ref`
- `scope_filter_class` (re-exported from ADR-0009)
- `segment_refs` (non-empty; every included / omitted / pinned /
  redacted / policy-blocked / tainted segment appears here by
  id)
- `mention_refs`
- `attachment_refs`
- `instruction_bundle_refs`
- `check_bundle_refs`
- `tool_call_lineage_refs`
- `route_plan_ref`
- `spend_plan_ref`
- `route_receipt_ref` (nullable — non-null after dispatch)
- `spend_receipt_ref` (nullable — non-null once provider
  reports)
- `branch_agent_dispatch_ref` (non-null when
  `dispatch_target_class = background_branch_agent`)
- `review_handoff_ref` (non-null when
  `dispatch_target_class = review_handoff`)
- `provider_class`
- `route_path_class`
- `cost_visibility_class`
- `dispatch_target_class`
- `tainted_segment_count`
- `pinned_segment_count`
- `omitted_segment_count`
- `policy_blocked_segment_count`
- `redacted_segment_count`
- `freshness_class` (re-exported from ADR-0011)
- `client_scopes` (non-empty; re-exported from ADR-0011)
- `running_build_identity_ref`
- `policy_context` (re-exported from ADR-0008 / ADR-0009 /
  ADR-0001)
- `redaction_class` (re-exported from ADR-0011)
- `minted_at`

Non-conforming when:

- the segment set is empty while `dispatch_target_class` is not
  `support_replay_only`;
- `tainted_segment_count > 0` and any tainted segment lacks a
  `fence_strategy`, `usage_constraints`, or
  `instructional_role = fenced_tainted_data`;
- `dispatch_target_class = background_branch_agent` and
  `branch_agent_dispatch_ref` is null;
- `dispatch_target_class = review_handoff` and
  `review_handoff_ref` is null;
- `provider_class = disabled_no_provider` or
  `route_path_class = denied_by_policy` and the turn was
  dispatched anyway.

### `ai_context_segment_record`

One record per segment on the assembly. Required fields:

- `segment_id` (opaque)
- `assembly_id_ref`
- `segment_class`
- `source_class`
- `trust_posture`
- `provenance_ref` (opaque reference to the owner that produced
  the segment — symbol id, file slice id, docs anchor id,
  terminal transcript span id, log span id, request-response id,
  generated artifact id, user-input id, extension-proposed-
  context id, connected-provider payload id, repo instruction /
  check bundle id, prior-turn id, composer-plan-directive id.
  Raw bodies never appear)
- `origin_mention_ref` (nullable)
- `origin_attachment_ref` (nullable)
- `instructional_role`
- `omit_reason` (required when `segment_class = omitted`)
- `block_reason` (required when
  `segment_class = policy_blocked`)
- `redaction_reasons` (required non-empty when
  `segment_class = redacted`)
- `tainted_fence_strategy` (required when
  `segment_class = tainted`)
- `tainted_usage_constraints` (required non-empty when
  `segment_class = tainted`)
- `citation_anchor_refs` (required non-empty for authoritative
  source classes listed above)
- `byte_budget_hint` (nullable)
- `pinned_by_actor_ref` (required when
  `segment_class = pinned`)
- `freshness_class`
- `client_scopes`
- `policy_context`
- `redaction_class`
- `minted_at`

### `ai_evidence_packet_record`

Per-turn evidence packet. Required fields:

- `evidence_packet_id`
- `assembly_id_ref`
- `composer_session_ref`
- `turn_draft_ref`
- `dispatch_target_class`
- `provider_class`
- `route_path_class`
- `route_receipt_ref` (required non-null when
  `dispatch_target_class != support_replay_only`)
- `spend_receipt_ref` (required non-null when
  `dispatch_target_class != support_replay_only`)
- `branch_agent_dispatch_ref` (required when
  `dispatch_target_class = background_branch_agent`)
- `review_handoff_ref` (required when
  `dispatch_target_class = review_handoff`)
- `tool_call_lineage_refs`
- `source_reference_refs`
- `tainted_fence_refs`
- `tainted_fence_count`
- `citation_anchor_coverage` (`required_count`,
  `provided_count`)
- `downstream_handoff_refs` (every packet the evidence was
  handed off to — review / diff canvas, mutation-journal entry,
  support bundle, claim manifest, nested evidence packet on a
  dependent branch-agent dispatch)
- `freshness_class`
- `client_scopes`
- `running_build_identity_ref`
- `policy_context`
- `redaction_class`
- `minted_at`

Non-conforming when:

- `tainted_fence_count` < the originating assembly's
  `tainted_segment_count` (the packet is missing fences);
- `citation_anchor_coverage.required_count >
  citation_anchor_coverage.provided_count` (authoritative claims
  are uncited);
- a downstream handoff dropped any tainted usage constraint the
  packet carried;
- `dispatch_target_class != support_replay_only` and either
  route or spend receipt ref is null.

### `ai_tainted_content_fence_record`

One record per tainted segment covered by the packet. Required
fields:

- `tainted_fence_id`
- `evidence_packet_ref`
- `assembly_id_ref`
- `segment_id_ref`
- `tainted_source_class` (closed set:
  `retrieved_document`, `terminal_snippet`, `log_snippet`,
  `generated_artifact_excerpt`, `request_payload`,
  `response_payload`, `user_supplied_text`,
  `user_supplied_file`, `extension_proposed_payload`,
  `connected_provider_payload`,
  `collaboration_remote_payload`, `tool_call_return_value`,
  `browser_handoff_return_value`, `ai_prior_turn_output`)
- `trust_posture` (subset of the context-assembly trust posture
  — `untrusted_*` / `policy_quarantined` / `unreviewed_derived`)
- `fence_strategy`
- `usage_constraints` (non-empty)
- `source_reference_refs` (may be empty when the fence covers a
  segment that carries no user-visible citation)
- `preserved_markings` (opaque label ids preserved on the fenced
  content for downstream render)
- `policy_context`
- `minted_at`

### `ai_evidence_source_reference_record`

One record per reference that backed a user-visible claim on the
turn. Required fields:

- `source_reference_id`
- `evidence_packet_ref`
- `claim_class`
- `target_ref` (opaque reference to the typed target — raw
  paths, raw URLs, raw bodies never appear)
- `segment_id_ref` (nullable)
- `citation_anchor_refs` (required non-empty on authoritative
  docs / generated-reference / runbook / release-note claims
  with a trusted source posture)
- `source_posture`
- `tainted_fence_ref` (required when
  `source_posture = tainted_external`)
- `freshness_class`
- `policy_context`
- `minted_at`

### Route / spend receipt shapes

`ai_route_receipt_record` and `ai_spend_receipt_record` record
the actual route / spend for every dispatched turn. The assembly
quotes the receipts by id; the evidence packet quotes the
assembly. Raw provider URLs, raw tenant tokens, and raw
credential material never appear on receipts.

### Tool-call lineage and branch-agent dispatch

- `ai_tool_call_lineage_record` — one per tool call the turn
  invoked, with `tool_outcome` in
  `not_invoked`, `invoked_success`, `invoked_partial`,
  `invoked_failure`, `invoked_denied_by_policy`,
  `invoked_requires_renewal`, `invoked_tainted_return`. Every
  `invoked_tainted_return` outcome MUST have every result
  segment recorded with `segment_class = tainted`.
- `ai_branch_agent_dispatch_record` — one per background
  branch-agent dispatch. Preserves the originating assembly's
  `inherited_tainted_usage_constraints`,
  `inherited_scope_filter_class`, and
  `inherited_redaction_class`. A branch-agent dispatch that
  drops any constraint is non-conforming.

### `ai_context_audit_event_record` and `ai_evidence_audit_event_record`

Structured events on the `ai_context` and `ai_evidence` audit
streams. Events MUST NOT carry raw prompt text, raw document
bodies, raw terminal / log bodies, raw generated artifact bytes,
raw request / response payloads, raw user-supplied text, or raw
credential material.

Frozen event ids on the `ai_context` stream:

- `ai_context_assembly_minted`
- `ai_context_segment_included`
- `ai_context_segment_omitted`
- `ai_context_segment_pinned`
- `ai_context_segment_redacted`
- `ai_context_segment_policy_blocked`
- `ai_context_segment_tainted_fenced`
- `ai_context_tainted_return_observed`
- `ai_context_route_plan_minted`
- `ai_context_spend_plan_minted`
- `ai_context_route_receipt_recorded`
- `ai_context_spend_receipt_recorded`
- `ai_context_turn_draft_opened`
- `ai_context_turn_draft_dispatched`
- `ai_context_turn_draft_cancelled`
- `ai_context_turn_draft_superseded`
- `ai_context_branch_agent_dispatched`
- `ai_context_review_handoff_dispatched`
- `ai_context_tool_call_invoked`
- `ai_context_tool_call_denied`
- `ai_context_instruction_bundle_loaded`
- `ai_context_check_bundle_attached`
- `ai_context_denial_emitted`
- `ai_context_assembly_schema_version_bumped`

Frozen event ids on the `ai_evidence` stream:

- `ai_evidence_packet_minted`
- `ai_evidence_source_reference_recorded`
- `ai_evidence_tainted_fence_recorded`
- `ai_evidence_tainted_fence_preserved_on_handoff`
- `ai_evidence_tainted_fence_dropped_denial`
- `ai_evidence_citation_anchor_missing_denial`
- `ai_evidence_receipt_missing_denial`
- `ai_evidence_schema_version_bumped`

### Denial reasons

Typed denials on the two audit streams. Denials fail closed;
silent downgrade to a best-effort assembly or evidence packet is
forbidden.

`ai_context` denials:
`segment_class_unresolved`, `source_class_unresolved`,
`trust_posture_unresolved`, `omit_reason_missing`,
`block_reason_missing`, `redaction_reason_missing`,
`tainted_fence_strategy_missing`,
`tainted_usage_constraint_missing`,
`instructional_role_missing`, `provider_class_unresolved`,
`route_path_class_unresolved`, `cost_visibility_unresolved`,
`provider_disabled_by_policy`, `route_denied_by_policy`,
`spend_ceiling_exceeded`, `spend_receipt_missing`,
`route_receipt_missing`,
`citation_anchor_missing_on_authoritative_quote`,
`instruction_bundle_ref_missing`, `check_bundle_ref_missing`,
`composer_session_ref_missing`,
`turn_draft_terminal_cannot_dispatch`,
`tainted_segment_instructional_role_forbidden`,
`tainted_segment_tool_permission_forbidden`,
`raw_body_forbidden_on_boundary`,
`raw_url_forbidden_on_boundary`,
`raw_prompt_text_forbidden_on_boundary`,
`ai_context_assembly_schema_version_lagging`.

`ai_evidence` denials:
`assembly_ref_missing`, `route_receipt_ref_missing`,
`spend_receipt_ref_missing`, `tainted_source_class_unresolved`,
`tainted_fence_strategy_missing`,
`tainted_usage_constraint_missing`,
`tainted_fence_dropped_on_handoff`,
`citation_anchor_missing_on_authoritative_claim`,
`raw_body_forbidden_on_boundary`,
`raw_url_forbidden_on_boundary`,
`raw_prompt_text_forbidden_on_boundary`,
`ai_evidence_packet_schema_version_lagging`.

## Example lifecycle

### Composer turn with tainted retrieved context

```
 ┌───────────────────────────────┐
 │ prompt_composer_session opens │
 └───────────────┬───────────────┘
                 │ user opens turn draft
                 ▼
        ┌──────────────────┐
        │ turn_draft draft │
        └────────┬─────────┘
                 │ user mentions symbol + attaches retrieved doc
                 ▼
 ┌──────────────────────────────────────────┐
 │ composer_plan resolves mentions /        │
 │ attachments; runs broker redaction pass; │
 │ assigns trust postures                   │
 └───────────────────┬──────────────────────┘
                     │
         ┌───────────┴───────────┐
         ▼                       ▼
  ┌──────────────┐        ┌──────────────────┐
  │ included     │        │ tainted          │
  │ workspace    │        │ retrieved doc    │
  │ symbol seg   │        │ (quoted_as_data) │
  └──────┬───────┘        └────────┬─────────┘
         │                         │
         └───────────┬─────────────┘
                     ▼
        ┌──────────────────────────┐
        │ ai_context_assembly      │
        │ (segment_refs, route &   │
        │ spend plan, receipts)    │
        └──────────┬───────────────┘
                   │ dispatched inline
                   ▼
        ┌──────────────────────────┐
        │ ai_evidence_packet       │
        │ (tainted_fence_refs,     │
        │ source_reference_refs,   │
        │ citation anchor coverage)│
        └──────────────────────────┘
```

Rules honoured by the diagram:

- The retrieved doc rides `trust_posture = untrusted_external`
  and `segment_class = tainted`; the composer plan applied
  `quoted_as_data_only` and the constraint
  `must_not_gain_tool_permission`.
- The assembly's `tainted_segment_count = 1`, and the evidence
  packet's `tainted_fence_count = 1` — a missing fence would
  deny the packet.
- The workspace symbol ride `trust_posture =
  trusted_first_party` and `segment_class = included`; it may
  carry instruction authority, the tainted segment may not.

### Background branch-agent dispatch inheriting taint

```
 ┌──────────────────────────┐
 │ inline composer turn     │
 │ (tainted segment set T0) │
 └──────────┬───────────────┘
            │ user dispatches branch-agent follow-up
            ▼
 ┌────────────────────────────────────────────┐
 │ ai_branch_agent_dispatch                   │
 │ inherits T0's tainted_usage_constraints,   │
 │ scope_filter_class, redaction_class        │
 └───────────────────┬────────────────────────┘
                     │
                     ▼
 ┌──────────────────────────┐
 │ new ai_context_assembly  │
 │ T0 segments carried over │
 │ with segment_class =     │
 │ tainted; new segments    │
 │ assessed afresh          │
 └──────────┬───────────────┘
            │ dispatched background_branch_agent
            ▼
 ┌──────────────────────────┐
 │ ai_evidence_packet with  │
 │ downstream_handoff_refs  │
 │ back to the originating  │
 │ packet                   │
 └──────────────────────────┘
```

Rules honoured by the diagram:

- Tainted-usage constraints inherit verbatim; the dispatch may
  not drop
  `must_not_commit_to_repo`,
  `must_not_publish_externally`, or
  `must_preserve_fence_in_downstream_packet` on handoff.
- The originating packet's `downstream_handoff_refs` names the
  branch-agent packet; the branch-agent packet's
  `downstream_handoff_refs` names any further handoff (review /
  diff canvas, mutation journal, support bundle, claim
  manifest).
- If any constraint is dropped, the handoff denies with
  `tainted_fence_dropped_on_handoff` and the branch-agent
  dispatch does not mint.

## Per-turn-class projection requirements (frozen)

| Turn class                 | Required axes                                                                                                                                                                                                                                                                      | Required disclosure                                                                                                                                                                      |
|----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `inline_composer`          | full assembly; `route_plan_ref` and `spend_plan_ref`; `route_receipt_ref` / `spend_receipt_ref` after dispatch; `interaction_safety_packet_record` on any apply (shell interaction-safety contract).                                                                                | AI apply on `ai_apply_canvas` MUST carry `representation_class = generated` with a non-empty `citation_anchor_refs` when quoting authoritative material, or denies under ADR-0013.       |
| `background_branch_agent`  | same plus `branch_agent_dispatch_ref`, `inherited_tainted_usage_constraints`, `inherited_scope_filter_class`, `inherited_redaction_class`.                                                                                                                                          | Branch-agent dispatch rides the `ai_initiated` authority class for any mutation (shell contract); tainted usage constraints survive downstream without re-inference.                     |
| `review_handoff`           | same plus `review_handoff_ref` into the review / diff canvas.                                                                                                                                                                                                                      | Review / diff canvas reads the assembly by id; it does not re-derive trust or fence state.                                                                                               |
| `tool_follow_up`           | same plus non-empty `tool_call_lineage_refs`; every `invoked_tainted_return` outcome MUST have every result segment recorded with `segment_class = tainted`.                                                                                                                       | The next turn's composer plan treats the fenced result as `fenced_tainted_data`; a surface that promotes a tainted return to instruction authority is non-conforming.                    |
| `support_replay_only`      | assembly reconstructed for support / parity audit; `route_receipt_ref` / `spend_receipt_ref` absent by construction.                                                                                                                                                                | Support tooling renders the assembly as a read-only artifact; no dispatch occurs; the evidence packet shape is reused with `dispatch_target_class = support_replay_only`.                |

### Chip collapsing is a UI freedom; record addressability is mandatory

A surface MAY fold `segment_class` /
`trust_posture` / `fence_strategy` /
`provider_class` / `route_path_class` /
`cost_visibility_class` into one chip for dense rendering,
provided the underlying records retain each axis as a separately
addressable field. Support exports, claim manifests, and parity
audits read each axis independently.

## Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `ai_context_assembly_record`,
   `ai_context_segment_record`,
   `prompt_composer_session_record`,
   `prompt_composer_mention_record`,
   `prompt_composer_attachment_record`,
   `prompt_composer_turn_draft_record`,
   `ai_route_plan_record`, `ai_spend_plan_record`,
   `ai_route_receipt_record`, `ai_spend_receipt_record`,
   `ai_tool_call_lineage_record`,
   `ai_branch_agent_dispatch_record`,
   `ai_context_audit_event_record`,
   `ai_evidence_packet_record`,
   `ai_tainted_content_fence_record`,
   `ai_evidence_source_reference_record`, and
   `ai_evidence_audit_event_record` cross the RPC boundary as
   typed payloads (ADR-0004). Raw prompt text, raw document
   bodies, raw terminal / log bodies, raw generated artifact
   bytes, raw request / response payloads, raw user-supplied
   text, and raw credential material never cross.
2. Mutation-journal entries, save manifests, support bundles,
   and claim manifests name `assembly_id`, `segment_id`,
   `turn_draft_id`, `route_receipt_id`, `spend_receipt_id`,
   `branch_agent_dispatch_id`, `evidence_packet_id`,
   `tainted_fence_id`, `source_reference_id`, and
   `running_build_identity_ref` only.
3. Crash dumps and core files MUST NOT inherit unresolved
   assemblies or evidence packets; a crash that lands mid-
   dispatch discards the assembly rather than persisting a
   partial axis set.
4. Any downstream surface that handoffs an evidence packet MUST
   preserve every tainted usage constraint the packet carried;
   a dropped constraint denies with
   `tainted_fence_dropped_on_handoff` and records the denial on
   the `ai_evidence` audit stream.

Redaction defaults (frozen):

| Sink                                 | Default inclusion                                                                                                                                                                                                                                                                                |
|--------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | Assembly / segment / turn-draft / route-plan / spend-plan / route-receipt / spend-receipt / branch-agent / tool-call / evidence-packet / tainted-fence / source-reference ids, segment class, trust posture, fence strategy, provider class, route path class, cost visibility class, audit-event ids. No raw bodies, paths, URLs, or prompt text. |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw bodies or paths.                                                                                                                                                                                                                           |
| `support_bundle`                     | Full per-axis values, full tainted-fence enumeration (ids + `tainted_source_class` + `fence_strategy` + `usage_constraints`), full citation-anchor enumeration, full route / spend receipt summary. Raw bodies excluded.                                                                           |
| `evidence_packet`                    | Release-relevant fields: `running_build_identity_ref`, `dispatch_target_class`, `provider_class`, `route_path_class`, `cost_visibility_class`, full tainted-fence set, full source-reference set, full `citation_anchor_coverage`. Raw bodies never included.                                        |
| `claim_manifest`                     | Full per-axis values. Raw bodies never included.                                                                                                                                                                                                                                                  |
| `mutation_journal_entry`             | Ids, `dispatch_target_class`, `segment_class` summary, `provider_class`, `route_path_class`, `cost_visibility_class`. No raw bodies or URLs.                                                                                                                                                     |
| `save_manifest` (ADR-0006)           | Same as `mutation_journal_entry`.                                                                                                                                                                                                                                                                 |
| `ai_context_capture`                 | Assembly / segment / evidence / fence ids only. Raw bodies and prompt text never captured; this sink exists specifically to avoid re-capturing the model input into another store.                                                                                                                |
| `recipe_manifest`                    | `assembly_id`, `turn_draft_id`, `running_build_identity_ref`. Raw bodies forbidden.                                                                                                                                                                                                              |
| `profile_export` / `sync`            | Same as `recipe_manifest`.                                                                                                                                                                                                                                                                       |
| `crash_dump`                         | Opt-in only; redaction scan precedes packaging; denied by default for packets whose `policy_context` references a managed policy bundle.                                                                                                                                                         |
| `terminal_transcript`                | `assembly_id` and `turn_draft_id` only; raw tool-call bodies and raw URLs require boundary-labelled confirmation before capture.                                                                                                                                                                |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

## Schema-of-record posture

The eventual AI / prompt-composer crate's Rust types are the
source of truth. The JSON Schema exports at
`schemas/ai/context_assembly.schema.json` and
`schemas/ai/evidence_packet.schema.json` are the cross-tool
boundary every non-owning surface reads. Adding a new segment
class, source class, trust posture, omit / block / redaction
reason, fence strategy, usage constraint, instructional role,
mention kind, attachment kind, turn-draft state, dispatch-target
class, provider class, route path class, cost visibility class,
instruction-bundle kind, check-bundle kind, tool-call outcome,
audit-event id, or denial reason is additive-minor and bumps the
relevant `*_schema_version`; repurposing any existing value is
breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014 and the shell
interaction-safety contract.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- Working AI inference. This contract reserves the assembly /
  route / spend / evidence shape; the AI crate wires the model
  provider later.
- Provider routing, connected-provider auth flow bodies
  (ADR-0010 reserves those), and extension-effective-permission
  resolution bodies (ADR-0012 reserves those).
- The prompt-composer UX. This contract reserves the composer
  session, mention, attachment, and turn-draft slots; the
  composer UX wires them later.
- The background branch-agent runtime. This contract reserves
  the dispatch record and the inheritance rules; the runtime
  wires the actual background execution later.
- The eventual AI / prompt-composer crate's Rust types; the JSON
  Schema exports reserve the boundary shape until the crate
  lands.

These lines move only by opening a new decision row, not by
editing this contract.

## Reuse guarantee

This contract is reusable by composer, inline-apply, branch-
agent, review-handoff, and support-replay flows without
redefining core assembly or evidence semantics. A new AI surface
MUST:

1. Quote the segment / trust / fence / usage-constraint /
   instructional-role / provider / route / cost vocabularies
   above verbatim.
2. Emit `ai_context_assembly_record` on every turn (dispatched
   or replay-only); emit `ai_context_segment_record` for every
   segment regardless of class (including `omitted` and
   `policy_blocked`); emit `ai_route_plan_record` /
   `ai_spend_plan_record` before dispatch; emit
   `ai_route_receipt_record` / `ai_spend_receipt_record` after
   dispatch; emit `ai_evidence_packet_record` on dispatch; emit
   `ai_tainted_content_fence_record` for every tainted segment
   on the packet; emit `ai_evidence_source_reference_record`
   for every user-visible claim; emit
   `ai_branch_agent_dispatch_record` on every background
   dispatch; emit `ai_tool_call_lineage_record` on every tool
   call.
3. Preserve each axis as a separately addressable field on the
   packet even when the UI folds them into one chip.
4. Honour the taint-preservation posture above; dropping a
   tainted-usage constraint on handoff is non-conforming.
