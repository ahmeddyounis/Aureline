# AI copy guardrail cases

Worked examples for
[`/docs/ai/ai_copy_guardrails_contract.md`](../../../docs/ai/ai_copy_guardrails_contract.md)
and the term registries in
[`/artifacts/ai/approved_ai_terms.yaml`](../../../artifacts/ai/approved_ai_terms.yaml)
and
[`/artifacts/ai/forbidden_ai_terms.yaml`](../../../artifacts/ai/forbidden_ai_terms.yaml).

These fixtures are intentionally schema-light at this revision. Each file
declares a single copy review case with the proposed copy, evidence and
context posture, allowed controls, expected decision, and assertions a
future linter or product review should enforce.

## Cases

- [`educational_answer_with_citations.yaml`](./educational_answer_with_citations.yaml)
  - accepted educational AI answer with citations, provider route
  disclosure, context limits, and only read-only controls.
- [`low_confidence_action_proposal.yaml`](./low_confidence_action_proposal.yaml)
  - accepted low-confidence action proposal that removes direct apply
  and offers preview, diff, and sandbox validation.
- [`stale_doc_explanation.yaml`](./stale_doc_explanation.yaml)
  - accepted stale-doc explanation that names source freshness and
  refresh/open-source routes instead of claiming current docs.
- [`rejected_overclaiming_phrase.yaml`](./rejected_overclaiming_phrase.yaml)
  - rejected copy that combines guaranteed success, review-free
  mutation, false validation, false exhaustiveness, and collapsed
  explain/action wording.

No fixture embeds raw prompt text, raw file bodies, raw logs, raw URLs,
raw provider payloads, credentials, or secrets. Opaque refs stand in for
source, provider, route, validation, and evidence objects.
