# AI prompt-composer edge-case corpus (oversized attachments, blocked scope, stale graph refs, imported-root references, tainted logs, and missing-authority links)

This directory holds the edge-case companion corpus for the
prompt-composer conformance verification packet at
[`/docs/verification/prompt_composer_packet.md`](../../../docs/verification/prompt_composer_packet.md)
and the audit matrix at
[`/artifacts/ai/prompt_composer_audit_matrix.yaml`](../../../artifacts/ai/prompt_composer_audit_matrix.yaml).

The worked-example corpus that pins the minimal conformant
composer turn shapes lives in
[`/fixtures/ai/prompt_composer_cases/`](../../prompt_composer_cases/).
This directory extends that corpus with edge cases that exercise
the typed omission / fence / policy-block / redaction surfaces
the contract requires every composer surface to project verbatim.

Every file is a multi-document YAML stream. The first document is
a `__fixture__` prelude summarising the scenario, the contract
sections it exercises, the typed classes it hits, and — importantly
— an `omission_explainer` block naming the four typed answers the
matrix audits (what was excluded, why, whether the omission is
reversible, and what safe next action remains). Each subsequent
document is one record conforming to
[`/schemas/ai/prompt_composer_session.schema.json`](../../../schemas/ai/prompt_composer_session.schema.json)
under JSON Schema Draft 2020-12.

No fixture embeds raw prompt text, raw retrieved-document bodies,
raw terminal / log bodies, raw generated artifact bytes, raw
request / response payloads, raw user-supplied text, raw
workspace paths, raw URLs, raw endpoint hostnames, raw API keys,
raw OAuth tokens, raw mTLS material, or raw credential material.
Every such field is an opaque ref or a structured redaction-safe
readout.

## Cases

| Scenario file                                                | Triggering disposition                       | Reversibility                                    | Safe next action                              |
|--------------------------------------------------------------|----------------------------------------------|--------------------------------------------------|-----------------------------------------------|
| `oversized_attachment_budget_omit.yaml`                      | `omitted_under_budget`                       | `reversible_by_user_action`                      | `dispatch_without_excluded_context`           |
| `blocked_scope_outside_named_workset.yaml`                   | `omitted_outside_scope`                      | `reversible_by_user_action`                      | `widen_scope_filter`                          |
| `stale_graph_ref_freshness_floor_unmet.yaml`                 | `omitted_freshness_floor_unmet`              | `reversible_by_freshness_refresh`                | `refresh_freshness_index`                     |
| `imported_root_reference_untrusted_work_item.yaml`           | `fenced_tainted_remote_collaborator`         | `reversible_by_higher_trust_context_sharing`     | `request_higher_trust_context_sharing`        |
| `tainted_log_broker_redacted_credential.yaml`                | `redacted_under_broker_pass`                 | `irreversible_denied_always` (redacted bytes)    | `select_alternative_target`                   |
| `work_item_link_missing_authority.yaml`                      | `policy_blocked_admin_policy`                | `reversible_by_admin_policy_change`              | `request_admin_policy_change`                 |
| `provider_link_missing_authority.yaml`                       | `policy_blocked_connected_provider_policy`   | `reversible_by_approval_request`                 | `request_approval_ticket`                     |
| `branch_agent_dispatch_preview_with_omits.yaml`              | `fenced_tainted_user_pasted`                 | `irreversible_denied_always` (paste authority)   | `dispatch_without_excluded_context`           |

Every fixture pins:

- exactly one `prompt_composer_session_descriptor`,
- exactly one `prompt_composer_turn_draft_descriptor` whose
  `draft_state` reflects the dispatch posture (most rows dispatch
  inline; the branch-agent preview row is `dispatched_branch_agent`),
- the matching mention / attachment / slash-command records that
  produced the edge disposition,
- exactly one `prompt_composer_predispatch_disclosure_record` with
  all four count fields rendered verbatim (`tainted_attachment_count`,
  `omitted_attachment_count`, `policy_blocked_attachment_count`,
  `redacted_attachment_count`) — including zero values where
  applicable — so the audit stream can prove the disclosure
  rendered the count even when nothing fell into a given bucket,
  and
- audit events for the typed outcome (`composer_turn_draft_attachment_omitted`,
  `composer_turn_draft_attachment_fenced_tainted`,
  `composer_turn_draft_attachment_policy_blocked`,
  `composer_turn_draft_attachment_redacted`).

The `__fixture__` prelude also pins a typed `omission_explainer`
block carrying the four answers the matrix audits. The block is
metadata for reviewers; it is not part of the composer-session
schema. Reviewers walk it to confirm each surface (desktop,
CLI / headless, export / support) renders the same four answers
verbatim.

## Acceptance-criteria coverage

The seeded edge cases cover the three acceptance criteria named
in the spec:

- **Composer proof runs fail when omitted or tainted context
  disappears silently from audit artifacts or review surfaces.**
  Every fixture renders the four count fields verbatim on the
  pre-dispatch disclosure record, including a non-zero count for
  the bucket the row exercises (1 for the dropped / fenced /
  blocked / redacted row). The audit stream records the typed
  `composer_turn_draft_attachment_omitted` /
  `…_fenced_tainted` / `…_policy_blocked` / `…_redacted` event
  every time. A composer surface that drops one of those events
  or zeros out the corresponding count is non-conforming — see
  the `tainted_context_disappeared_silently` and
  `omitted_context_disappeared_silently` failure conditions on
  the audit matrix.
- **Every edge case yields a typed omission or downgrade
  explanation rather than a generic error or silent flattening.**
  Every fixture's `__fixture__` prelude pins one
  `triggering_disposition` from the contract's typed
  `attachment_disposition_class` vocabulary plus the four typed
  omission-explainer answers (`what_was_excluded`, `why_excluded`,
  `reversible_class`, `safe_next_action_class`) — none of which
  resolve to a generic "context unavailable" / "retry" string.
- **The packet can be cited by future QE / public-proof work
  without recreating a separate AI-composer test vocabulary.**
  Every typed value used in the fixtures is already part of one
  of the frozen prompt-composer / context-assembly /
  request-workspace-ref / tainted-input vocabularies. No edge
  fixture mints a new mention kind, attachment kind, disposition
  class, fence strategy, tainted usage constraint, account /
  provider path class, approval posture, or disclosure field
  class.

## Validation

Every fixture in this directory validates against
`schemas/ai/prompt_composer_session.schema.json` (and, for the
embedded `request_workspace_ref` fields, against
`schemas/ai/request_workspace_ref.schema.json`) under JSON Schema
Draft 2020-12. The same Python harness referenced in
`fixtures/ai/prompt_composer_cases/README.md` validates this
directory; substitute the directory glob:

```python
for path in (repo / "fixtures/ai/prompt_composer_edge_cases").glob("*.yaml"):
    docs = list(yaml.safe_load_all(path.read_text()))
    for doc in docs:
        if doc is None or "__fixture__" in doc:
            continue
        validator.validate(doc)
```
