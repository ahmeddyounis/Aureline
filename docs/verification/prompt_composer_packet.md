# Prompt-composer conformance, tainted-context edge corpus, and omission-explainer audit verification seed

This packet freezes one shared verification story for the AI prompt
composer: which axes a pre-dispatch disclosure card MUST project
verbatim, how omitted / fenced / policy-blocked / redacted context is
audited without disappearing into a generic "context unavailable"
chip, how every edge case in the corpus yields a typed omission
explainer (what was excluded, why, whether the omission is reversible,
what safe next action remains), and how the desktop, CLI / headless,
and export / support surfaces stay parity-locked against the same
disclosure record. It exists so later composer UI, provider
integration, support-export, and public-proof work reuses one
inspectable object model instead of inventing per-surface composer-
test vocabulary, silent flattening of tainted context, ad-hoc
omission copy, or untyped error fall-throughs.

If this packet, the
[`prompt_composer_audit_matrix.yaml`](../../artifacts/ai/prompt_composer_audit_matrix.yaml)
matrix, the
[`prompt_composer_edge_cases/`](../../fixtures/ai/prompt_composer_edge_cases/)
corpus, and the frozen prompt-composer / context-assembly /
request-workspace-ref / tainted-input schemas disagree, the schemas
and the prompt-composer contract win for tooling and this packet
must update in the same change.

Companion artifacts:

- [`/artifacts/ai/prompt_composer_audit_matrix.yaml`](../../artifacts/ai/prompt_composer_audit_matrix.yaml)
  — machine-readable conformance matrix naming the seven typed
  conformance axes, the four typed omission-explainer answers, the
  edge-case roster (oversized attachment, blocked scope, stale
  graph ref, imported-root reference, tainted log, work-item /
  provider link with missing authority, branch-agent dispatch
  preview with omits), the three surface-parity rules, and the
  release-blocking failure conditions.
- [`/fixtures/ai/prompt_composer_edge_cases/`](../../fixtures/ai/prompt_composer_edge_cases/)
  — edge-case fixture corpus. Every fixture is a multi-document
  YAML stream that schema-validates against
  `schemas/ai/prompt_composer_session.schema.json` (and, for the
  embedded `request_workspace_ref` fields,
  `schemas/ai/request_workspace_ref.schema.json`). Each fixture
  carries one composer session, one turn draft, the typed mention
  / attachment / slash-command records that produced the edge
  disposition, one pre-dispatch disclosure record with all four
  count fields rendered verbatim, and a typed
  `omission_explainer` block on the `__fixture__` prelude naming
  the four answers reviewers MUST see.
- [`/fixtures/ai/prompt_composer_cases/`](../../fixtures/ai/prompt_composer_cases/)
  — already-frozen worked-example corpus this packet composes
  over (empty draft, single-file ask, cross-repo ask with omitted
  context, tainted-pasted-content ask, background branch-agent
  dispatch). The new edge corpus extends the same record-kind
  vocabulary; it does not redefine it.
- [`/docs/ai/prompt_composer_contract.md`](../ai/prompt_composer_contract.md)
  — canonical composer-session / turn-draft / mention / attachment
  / slash-command / pre-dispatch disclosure contract this packet
  projects. The packet does not redefine any vocabulary frozen
  there.
- [`/docs/ai/prompt_injection_and_taint_contract.md`](../ai/prompt_injection_and_taint_contract.md)
  — canonical tainted-input source / safeguard / approval-action
  contract that names which trust postures must ride the
  tainted fence and which downstream actions a tainted source
  may not authorise.
- [`/docs/ai/context_assembly_contract.md`](../ai/context_assembly_contract.md)
  — canonical assembly-side mention / attachment / disposition /
  trust-posture / fence-strategy / tainted-usage-constraint
  vocabulary the composer descriptors re-export verbatim.
- [`/schemas/ai/prompt_composer_session.schema.json`](../../schemas/ai/prompt_composer_session.schema.json)
  — boundary schema every edge fixture validates against.
- [`/schemas/ai/request_workspace_ref.schema.json`](../../schemas/ai/request_workspace_ref.schema.json)
  — boundary schema for the typed cross-schema
  `request_workspace_ref_record` re-exported on every composer
  descriptor.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — requirement register and explicit honesty rules: omitted,
  fenced, redacted, and policy-blocked context MUST stay visible
  before dispatch; silent flattening of tainted context to a
  generic error is non-conforming.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — composer descriptors are the schema-of-record for the user-
  facing pre-dispatch surface; CLI / headless and export /
  support surfaces project the same record without re-minting.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — the pre-dispatch disclosure card MUST list the typed required
  fields verbatim; the four count fields render even when the
  count is zero so the audit stream proves the disclosure
  happened.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — composer chip discipline, count rendering rules, and the
  honesty contract for tainted / omitted / policy-blocked /
  redacted dispositions.
- `.t2/docs/Aureline_Milestones_Document.md`
  — composer truth remains an inspectable verification packet
  during the foundations phase rather than a live UI surface.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.prompt_composer.conformance_and_edge_corpus_seed
evidence_id: evidence.verification.prompt_composer.packet
title: Prompt-composer conformance, tainted-context edge corpus, and omission-explainer audit verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
    - GOV-CORPUS-901
    - GOV-DATA-002
    - ARCH-PACK-901
  claim_row_refs:
    - packet_row:prompt_composer.conformance_axes
    - packet_row:prompt_composer.omission_explainer_contract
    - packet_row:prompt_composer.edge_corpus
    - packet_row:prompt_composer.surface_parity
    - packet_row:prompt_composer.release_blocking_failures
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-05-04T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: prompt_composer_conformance_seed@1
  trigger_revision: prompt_composer_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen prompt-composer session / turn-draft
    / mention / attachment / slash-command / pre-dispatch disclosure
    schemas, the typed request-workspace-ref schema, the assembly-
    side mention / attachment / disposition / trust-posture / fence-
    strategy / tainted-usage-constraint vocabulary, and the already-
    seeded prompt-composer worked-example corpus. No live composer
    UI, provider integration, CLI / headless surface, or support /
    export pipeline is wired to this packet yet. Claims are
    structural: every row in the audit matrix and every edge
    fixture re-exports already-frozen vocabulary rather than
    minting per-surface synonyms.
artifact_links:
  supporting_evidence_ids:
    - evidence.verification.prompt_composer.audit_matrix
    - evidence.verification.prompt_composer.edge_corpus
    - evidence.ai.prompt_composer_cases
    - evidence.ai.prompt_composer_contract
    - evidence.ai.prompt_injection_and_taint_contract
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/ai/prompt_composer_edge_cases/oversized_attachment_budget_omit.yaml
    - fixtures/ai/prompt_composer_edge_cases/blocked_scope_outside_named_workset.yaml
    - fixtures/ai/prompt_composer_edge_cases/stale_graph_ref_freshness_floor_unmet.yaml
    - fixtures/ai/prompt_composer_edge_cases/imported_root_reference_untrusted_work_item.yaml
    - fixtures/ai/prompt_composer_edge_cases/tainted_log_broker_redacted_credential.yaml
    - fixtures/ai/prompt_composer_edge_cases/work_item_link_missing_authority.yaml
    - fixtures/ai/prompt_composer_edge_cases/provider_link_missing_authority.yaml
    - fixtures/ai/prompt_composer_edge_cases/branch_agent_dispatch_preview_with_omits.yaml
    - fixtures/ai/prompt_composer_cases/empty_draft.yaml
    - fixtures/ai/prompt_composer_cases/single_file_ask.yaml
    - fixtures/ai/prompt_composer_cases/cross_repo_ask_with_omitted_context.yaml
    - fixtures/ai/prompt_composer_cases/tainted_pasted_content.yaml
    - fixtures/ai/prompt_composer_cases/background_branch_agent_dispatch.yaml
  archetype_refs: []
  source_anchor_refs:
    - docs/ai/prompt_composer_contract.md
    - docs/ai/prompt_injection_and_taint_contract.md
    - docs/ai/context_assembly_contract.md
    - schemas/ai/prompt_composer_session.schema.json
    - schemas/ai/request_workspace_ref.schema.json
    - schemas/ai/context_assembly.schema.json
    - schemas/ai/tainted_input_source.schema.json
    - artifacts/ai/prompt_composer_audit_matrix.yaml
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one closed `conformance_axes` set (attachments, mentions, omitted
  context, tainted pasted content, route / spend preview,
  branch-agent dispatch preview, required disclosure) every
  composer surface answers verbatim;
- one closed `omission_explainer_required_answers` set (what was
  excluded, why excluded, reversible class, safe next action
  class) every edge row renders verbatim before the user
  acknowledges dispatch;
- one closed `reversibility_class_vocabulary`
  (`reversible_by_user_action`,
  `reversible_by_admin_policy_change`,
  `reversible_by_freshness_refresh`,
  `reversible_by_workspace_trust_grant`,
  `reversible_by_approval_request`,
  `reversible_by_higher_trust_context_sharing`,
  `irreversible_denied_always`);
- one closed `safe_next_action_class_vocabulary`
  (`widen_scope_filter`, `request_admin_policy_change`,
  `refresh_freshness_index`, `grant_workspace_trust`,
  `request_approval_ticket`, `request_higher_trust_context_sharing`,
  `select_alternative_target`, `dispatch_without_excluded_context`,
  `cancel_turn_draft`, `contact_support`,
  `no_safe_next_action_denied_always`);
- one edge-case roster covering oversized attachments, blocked
  scope, stale graph refs, imported-root references, tainted logs,
  work-item links with missing authority, provider links with
  missing authority, and a branch-agent dispatch preview with
  omitted context;
- one closed `failure_conditions` set so a composer proof run fails
  when omitted or tainted context disappears silently from the
  audit artifacts or when any review surface drops one of the
  required typed fields; and
- one closed `surface_parity_rules` set so the desktop, CLI /
  headless, and export / support surfaces project the same four
  count fields and the same four typed omission-explainer answers
  from the same record.

It does not claim a composer UI, provider integration, CLI /
headless surface, or support-export pipeline is wired up. It
claims only that the packet, the audit matrix, and the edge
fixture corpus now exist in one reviewable form and reuse the
frozen prompt-composer / context-assembly / request-workspace-ref
/ tainted-input vocabulary already landed elsewhere.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:prompt_composer.conformance_axes` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.prompt_composer.audit_matrix` | Freezes the seven typed conformance axes (attachments, mentions, omitted context, tainted pasted content, route / spend preview, branch-agent dispatch preview, required disclosure) every composer surface answers verbatim. |
| `packet_row:prompt_composer.omission_explainer_contract` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.prompt_composer.audit_matrix`, `evidence.verification.prompt_composer.edge_corpus` | Closed answer-set (what was excluded, why, reversible class, safe next action class) plus closed reversibility and safe-next-action vocabularies; replaces generic-error fall-through. |
| `packet_row:prompt_composer.edge_corpus` | `GOV-CORPUS-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.prompt_composer.edge_corpus` | Edge fixtures cover oversized attachments, blocked scope, stale graph refs, imported-root references, tainted logs, work-item / provider links with missing authority, and branch-agent dispatch preview with omits. Each fixture schema-validates against `prompt_composer_session.schema.json`. |
| `packet_row:prompt_composer.surface_parity` | `GOV-TRUTH-901`, `GOV-DATA-002` | `seed_only` | `internal` | `evidence.verification.prompt_composer.audit_matrix` | Desktop, CLI / headless, and export / support surfaces project the same four count fields and four typed omission-explainer answers from the same disclosure record. |
| `packet_row:prompt_composer.release_blocking_failures` | `GOV-TRUTH-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.prompt_composer.audit_matrix` | Closed `failure_conditions` set names when a composer proof run MUST fail (silent disappearance of tainted / omitted context, missing required disclosure field, parity drift, generic-error replacement, branch-agent dispatch intent undisclosed, tainted paste promoted to instruction authority). |

## Conformance axes

Every prompt-composer surface answers seven questions for every
turn. The audit matrix freezes one row per axis.

| Axis | Required record kinds | Required disclosure fields / count fields |
|---|---|---|
| `attachments_axis` | `prompt_composer_attachment_descriptor` | `tainted_attachment_count_disclosed`, `omitted_attachment_count_disclosed`, `policy_blocked_attachment_count_disclosed`, `redacted_attachment_count_disclosed` |
| `mentions_axis` | `prompt_composer_mention_descriptor` | (mention disposition projected onto the same four count fields above) |
| `omitted_context_axis` | mention / attachment descriptors with one of the `omitted_*` dispositions | `omitted_attachment_count_disclosed`; typed omission explainer renders verbatim |
| `tainted_pasted_content_axis` | mention / attachment descriptors with one of the `fenced_tainted_*` dispositions | `tainted_attachment_count_disclosed`; `tainted_fence_strategy` and non-empty `tainted_usage_constraints` MUST render |
| `route_spend_preview_axis` | `prompt_composer_predispatch_disclosure_record` | `route_path_disclosed`, `cost_visibility_disclosed`, `estimated_spend_disclosed`, `spend_ceiling_disclosed`; `route_plan_placeholder_ref` / `spend_plan_placeholder_ref` non-null on `disclosure_ready` |
| `branch_agent_dispatch_preview_axis` | turn-draft descriptor with `dispatch_target_class = background_branch_agent` | `branch_agent_dispatch_intent_disclosed`; `branch_agent_dispatch_placeholder_ref` non-null on the disclosure |
| `required_disclosure_axis` | `prompt_composer_predispatch_disclosure_record` | `scope_filter_disclosed`, `target_context_disclosed`, `active_account_disclosed`, `active_provider_disclosed`, `active_model_disclosed`, `route_path_disclosed`, `cost_visibility_disclosed`, `approval_posture_disclosed`, `request_workspace_ref_disclosed` |

Rule: a composer surface that drops one of the seven axes is non-
conforming. Chip collapsing in the UI is a freedom; record
addressability is mandatory across desktop, CLI / headless, and
export / support surfaces.

## Omission-explainer contract

Every edge row whose disposition is one of `omitted_*`,
`policy_blocked_*`, `fenced_tainted_*`, or
`redacted_under_broker_pass` MUST render the four typed answers
below verbatim. A row that renders without one of the four
answers is non-conforming and trips the
`omission_explainer_required_answer_missing` failure condition.

| Answer class | Contract |
|---|---|
| `what_was_excluded` | Names the typed `mention_kind` / `attachment_kind` / `source_ref` the composer dropped, fenced, redacted, or denied. May not collapse to a generic "context unavailable" or "search returned nothing" string. |
| `why_excluded` | Names the typed `disposition_class` (one of `omitted_outside_scope`, `omitted_under_budget`, `omitted_freshness_floor_unmet`, `omitted_dedup_against_pinned`, `omitted_user_deselected`, `omitted_policy_narrows_scope`, `fenced_tainted_*`, `policy_blocked_*`, `redacted_under_broker_pass`, `denied_data_class_denied_always`) verbatim. |
| `reversible_class` | One of the closed `reversibility_class_vocabulary` tokens (see below). |
| `safe_next_action_class` | One of the closed `safe_next_action_class_vocabulary` tokens (see below). May not collapse to a generic "retry" or silent fall-through. |

### `reversibility_class_vocabulary` (frozen)

| Token | Meaning |
|---|---|
| `reversible_by_user_action` | User can pin / un-omit / reattach the excluded context without policy or admin involvement. |
| `reversible_by_admin_policy_change` | Reversal requires an admin policy change (typically for `policy_blocked_admin_policy` / `omitted_policy_narrows_scope`). |
| `reversible_by_freshness_refresh` | Reversal requires refreshing the underlying graph / docs / pack index past the freshness floor. |
| `reversible_by_workspace_trust_grant` | Reversal requires the workspace-trust grant (ADR-0001) on a folder / workspace currently in the `restricted` posture. |
| `reversible_by_approval_request` | Reversal requires a fresh approval ticket (typically the ADR-0010 connected-provider browser-handoff ticket). |
| `reversible_by_higher_trust_context_sharing` | Reversal requires the higher-trust context-sharing approval surface (broader payload than baseline composer flows admit). |
| `irreversible_denied_always` | Reversal is denied by the contract; e.g. `credential_handle_denied_always` / `secret_projection_denied_always` data classes. |

### `safe_next_action_class_vocabulary` (frozen)

| Token | Meaning |
|---|---|
| `widen_scope_filter` | User narrows or replaces the active `scope_filter_class` to admit the excluded target. |
| `request_admin_policy_change` | User opens the admin-policy change route to admit the blocked authority / project / provider. |
| `refresh_freshness_index` | User refreshes the graph / docs / pack index so the row's freshness floor is met. |
| `grant_workspace_trust` | User grants workspace-trust on the folder / workspace whose trust posture caused the block. |
| `request_approval_ticket` | User opens the approval-request flow (typically the ADR-0010 connected-provider browser-handoff ticket). |
| `request_higher_trust_context_sharing` | User opens the higher-trust context-sharing approval surface. |
| `select_alternative_target` | User picks a different mention / attachment that resolves cleanly. |
| `dispatch_without_excluded_context` | User acknowledges the disclosure and dispatches without the excluded context. |
| `cancel_turn_draft` | User cancels the turn draft; no dispatch fires. |
| `contact_support` | User opens the support / replay channel; the row's authority cannot be self-served. |
| `no_safe_next_action_denied_always` | The contract denies reversal; the row reports the denial verbatim and offers no next-action lane. |

Rules (frozen):

1. The four answers MUST render on the same surface as the
   pre-dispatch disclosure card. A surface that emits the
   `tainted_attachment_count` / `omitted_attachment_count` /
   `policy_blocked_attachment_count` /
   `redacted_attachment_count` chip but hides the four typed
   answers behind tooltips is non-conforming.
2. The answer set MUST be projected verbatim into export /
   support bundles. The export bundle MAY redact `target_ref`
   strings but MAY NOT drop the typed `what_was_excluded` /
   `why_excluded` / `reversible_class` / `safe_next_action_class`
   classes.
3. A row whose disposition is `denied_data_class_denied_always`
   MUST set `reversible_class = irreversible_denied_always` and
   `safe_next_action_class = no_safe_next_action_denied_always`
   (or `select_alternative_target`); it MAY NOT advertise a
   reversal path the contract forbids.
4. A row whose disposition is `redacted_under_broker_pass` MAY
   render `reversible_class = irreversible_denied_always` for
   the redacted bytes themselves while pointing the user at
   `select_alternative_target` for the parent attachment.

## Edge case roster

Every edge row binds one fixture under
`fixtures/ai/prompt_composer_edge_cases/` to one or more
conformance axes, one triggering disposition, and the four typed
answers the omission explainer renders. The matrix
(`artifacts/ai/prompt_composer_audit_matrix.yaml`) carries the
full machine-readable roster; the table below names the human-
reviewer rows.

| Row id | Fixture | Disposition | Reversible class | Safe next action |
|---|---|---|---|---|
| `prompt_composer.edge.oversized_attachment.budget_omit` | `oversized_attachment_budget_omit.yaml` | `omitted_under_budget` | `reversible_by_user_action` | `dispatch_without_excluded_context` |
| `prompt_composer.edge.blocked_scope.outside_named_workset` | `blocked_scope_outside_named_workset.yaml` | `omitted_outside_scope` | `reversible_by_user_action` | `widen_scope_filter` |
| `prompt_composer.edge.stale_graph_ref.freshness_floor_unmet` | `stale_graph_ref_freshness_floor_unmet.yaml` | `omitted_freshness_floor_unmet` | `reversible_by_freshness_refresh` | `refresh_freshness_index` |
| `prompt_composer.edge.imported_root_reference.untrusted_work_item` | `imported_root_reference_untrusted_work_item.yaml` | `fenced_tainted_remote_collaborator` | `reversible_by_higher_trust_context_sharing` | `request_higher_trust_context_sharing` |
| `prompt_composer.edge.tainted_log.broker_redacted_credential` | `tainted_log_broker_redacted_credential.yaml` | `redacted_under_broker_pass` | `irreversible_denied_always` | `select_alternative_target` |
| `prompt_composer.edge.work_item_link_missing_authority` | `work_item_link_missing_authority.yaml` | `policy_blocked_admin_policy` | `reversible_by_admin_policy_change` | `request_admin_policy_change` |
| `prompt_composer.edge.provider_link_missing_authority` | `provider_link_missing_authority.yaml` | `policy_blocked_connected_provider_policy` | `reversible_by_approval_request` | `request_approval_ticket` |
| `prompt_composer.edge.branch_agent_dispatch_preview_with_omits` | `branch_agent_dispatch_preview_with_omits.yaml` | `fenced_tainted_user_pasted` | `irreversible_denied_always` | `dispatch_without_excluded_context` |

Rule: every fixture MUST schema-validate against
`schemas/ai/prompt_composer_session.schema.json` and MUST carry
a `__fixture__` prelude with an `omission_explainer` block naming
the four typed answers. A fixture whose typed disposition does
not match the matrix row's `triggering_disposition` is non-
conforming.

## Surface parity rules

The composer packet truth MUST render identically across three
surfaces. Chip collapsing in the UI is a freedom; record-level
field addressability is mandatory.

| Surface | May mint records? | May collapse chips? | MUST project |
|---|---|---|---|
| `desktop_composer_pane` | yes | yes | four count fields + four typed omission-explainer answers |
| `cli_or_headless_composer` | yes | no | four count fields + four typed omission-explainer answers |
| `export_or_support_bundle` | no (quotes minted records) | no | four count fields + four typed omission-explainer answers |

Rule: a surface that drops one of the eight fields (the four
count fields plus the four typed omission-explainer answers) is
non-conforming even when its UI chip collapses. The export /
support bundle MAY apply the surface's normal redaction class
(`internal_support_restricted` / `operator_only_restricted`) but
MAY NOT drop the typed classes.

## Release-blocking failure conditions

A composer proof run MUST fail when any of the following
conditions are observed across the edge fixture corpus or across
the three parity surfaces:

1. **`tainted_context_disappeared_silently`** — A turn-draft
   descriptor lists a `fenced_tainted_*` attachment or mention
   but the resulting pre-dispatch disclosure record's
   `tainted_attachment_count` is zero, OR the `disclosure_fields`
   list omits `tainted_attachment_count_disclosed`.
2. **`omitted_context_disappeared_silently`** — A turn-draft
   descriptor lists an `omitted_*` / `policy_blocked_*` /
   `redacted_*` mention or attachment but the resulting
   disclosure record reports zero on the matching count field
   OR omits the matching `disclosure_field_class`.
3. **`omission_explainer_required_answer_missing`** — An edge
   row whose disposition is one of the `omitted_*` /
   `policy_blocked_*` / `fenced_tainted_*` /
   `redacted_under_broker_pass` values renders without one of
   the four typed answers.
4. **`generic_error_replaced_typed_disposition`** — An edge row
   whose contract disposition is typed collapses to a generic
   "context unavailable" or "search returned nothing" string on
   any surface.
5. **`required_disclosure_field_missing`** — A pre-dispatch
   disclosure record lacks one of the contract-required
   `disclosure_field_class` entries (scope, target, account,
   provider, model, route, cost visibility, approval posture,
   request_workspace_ref). Schema enforcement denies with
   `required_disclosure_field_missing`.
6. **`parity_drift_between_surfaces`** — Desktop, CLI / headless,
   and export / support do not project the same four count
   fields and four typed omission-explainer answers from the
   same disclosure record.
7. **`branch_agent_dispatch_intent_undisclosed`** — A turn-draft
   descriptor whose `dispatch_target_class` is
   `background_branch_agent` dispatches without
   `branch_agent_dispatch_intent_disclosed` on the disclosure
   OR without a non-null `branch_agent_dispatch_placeholder_ref`.
8. **`tainted_paste_promoted_to_instruction_authority`** — A
   turn-draft descriptor admits an attachment with
   `trust_posture` in the untrusted set but the descriptor's
   `tainted_usage_constraints` does not include
   `must_not_override_instruction_bundle`, OR a downstream
   slash-command invocation cites the tainted attachment as an
   `instruction_bundle_ref`.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.verification.prompt_composer.audit_matrix` | `verification_corpus` | Defines the conformance axes, omission-explainer answers, edge-case roster, and surface-parity rules every composer surface cites. | current | `artifacts/ai/prompt_composer_audit_matrix.yaml` |
| `evidence.verification.prompt_composer.edge_corpus` | `verification_corpus` | Defines the edge-case fixture roster. Every fixture validates against `schemas/ai/prompt_composer_session.schema.json`. | current | `fixtures/ai/prompt_composer_edge_cases/` |
| `evidence.ai.prompt_composer_cases` | `source_anchor` | Already-frozen worked-example corpus (empty draft, single-file ask, cross-repo ask, tainted paste, branch-agent dispatch) the edge corpus extends. | current | `fixtures/ai/prompt_composer_cases/` |
| `evidence.ai.prompt_composer_contract` | `source_anchor` | Canonical composer-session / turn-draft / mention / attachment / slash-command / pre-dispatch disclosure contract. | current | `docs/ai/prompt_composer_contract.md` |
| `evidence.ai.prompt_injection_and_taint_contract` | `source_anchor` | Canonical tainted-input source / safeguard / approval-action contract. | current | `docs/ai/prompt_injection_and_taint_contract.md` |

## Verification method

- **Verification classes used:** design review, vocabulary-reuse
  review, fixture review, schema-alignment review.
- **Procedure summary:** verified that the packet, the audit
  matrix, and the edge fixture corpus reuse the prompt-composer
  contract's mention-kind / attachment-kind / disposition-class /
  trust-posture / fence-strategy / tainted-usage-constraint /
  approval-posture / account-provider-path / disclosure-field /
  audit-event vocabularies without minting parallel tokens.
  Verified that each edge fixture carries one composer session,
  one turn draft, the matching mention / attachment / slash-
  command records, one pre-dispatch disclosure record with all
  four count fields rendered verbatim (including zero values),
  and a `__fixture__` prelude `omission_explainer` block with
  the four typed answers. Verified that the conformance axes,
  reversibility class vocabulary, safe-next-action class
  vocabulary, and failure-condition set are closed and that
  every edge row exercises at least one axis and one
  disposition.
- **Automation refs:** the existing JSON Schema validation harness
  (Draft 2020-12, `jsonschema.Draft202012Validator` plus
  `PyYAML` plus a `referencing.Registry` populated with the
  composer-session schema and the request-workspace-ref schema)
  is sufficient to validate every fixture; see
  `fixtures/ai/prompt_composer_cases/README.md` for the
  reference harness.

## Known gaps and waivers

- **Waiver refs:** `none`.
- **Known limit refs:** the packet describes a structural
  contract; a live composer UI, CLI / headless surface, and
  support-export pipeline that emit these records have not
  landed yet. The four count fields and four typed answers are
  available for projection but no surface is producing them in
  product yet.
- **Migration packet refs:** none; the packet does not migrate
  any prior composer-test vocabulary because no separate
  composer-test vocabulary existed before this packet.

## How to cite this packet

Future QE / public-proof / support-export work cites this
packet by `(packet_id, packet_row)` and the audit matrix by
`(registry_id, edge_case_rows[].row_id)` instead of redefining
its own composer-test vocabulary. Examples:

- Release-blocking checks: cite
  `verification.prompt_composer.conformance_and_edge_corpus_seed`
  and the `failure_conditions` set on the audit matrix.
- Public-proof packets: cite the
  `packet_row:prompt_composer.edge_corpus` and the matching
  `edge_case_rows[].row_id` to anchor the typed disposition
  claim without re-minting count or explainer fields.
- Support / replay parity: cite the
  `surface_parity_rules` block to prove the export bundle
  preserved the four count fields and four typed answers from
  the original desktop disclosure.
