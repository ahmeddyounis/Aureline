# AI Copy Guardrails, Confidence Vocabulary, and Forbidden High-Trust Phrasing

- Catalog: `m5-ai-copy-guardrail-catalog:stable:0001`
- Label: `AI Copy Guardrails, Confidence Vocabulary, and Forbidden High-Trust Phrasing`
- Reference locale: `en`
- Controlled terms: 9 | Forbidden phrases: 14
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Controlled AI terms

- `term.proposal.suggested` — Suggested [suggested / proposal_state]: Model-proposed guidance or a next step that has not been accepted, applied, run through validation, or made authoritative. Requires: Evidence basis or cited refs; Confidence label and reason; Next safe action such as Open source or Open diff.
  - Surfaces: prompt_composer, patch_review, notebook_help | Consumers: product_ui, support_export, narrated_announcement
- `term.proposal.proposed` — Proposed [proposed / proposal_state]: A concrete change put forward for review; it carries a target and scope but is not yet accepted or applied. Requires: Target identity and scope; Confidence label; Review or open-diff route.
  - Surfaces: patch_review, prompt_composer | Consumers: product_ui, support_export, release_demo
- `term.proposal.draft` — Draft [draft / proposal_state]: Generated content or a generated patch that exists outside canonical source truth and still needs review before apply or publication. Requires: Draft or request-workspace ref; Source or diff route; Review or discard path.
  - Surfaces: prompt_composer, patch_review, docs_help | Consumers: product_ui, docs_help, support_export
- `term.context.context_used` — Context used [context_used / context_disclosure]: Names the context segments an answer actually used and the scope it left out, so the surface never implies it read more than it did. Requires: Covered scope; Excluded or omitted scope and reason; Expand or open-detail route when available.
  - Surfaces: prompt_composer, notebook_help, provider_account | Consumers: product_ui, support_export, narrated_announcement
- `term.validation.not_run` — Validation not run [validation / validation]: No validation plan has produced an outcome for this AI result; the surface must not imply checks, lint, build, or policy review passed. Requires: Reason validation is missing, skipped, or not applicable; Safe validation route when available.
  - Surfaces: patch_review, prompt_composer | Consumers: product_ui, support_export, release_demo
- `term.validation.passed` — Validation passed [validation / validation]: A named validation plan produced a passed outcome for the declared scope; the surface names the validation plan and the exact scope it covered. Requires: Validation plan ref; Declared validation scope; Any excluded checks or stale evidence.
  - Surfaces: patch_review | Consumers: product_ui, support_export, docs_help
- `term.confidence.low_confidence` — Low confidence [low_confidence / confidence]: The AI result is below the surface confidence floor because evidence is missing, conflicting, stale, partial, or omitted by policy. Requires: The specific limiting reason; A safe next action such as Open source or Prepare preview; Direct mutation controls removed. Direct mutation controls are suppressed.
  - Surfaces: prompt_composer, patch_review, notebook_help, docs_help | Consumers: product_ui, support_export, narrated_announcement
- `term.review.review_required` — Review required [review_required / review_posture]: Human, policy, ownership, or write-scope review remains required before the change is applied, published, or sent to a provider. Requires: Review owner or review surface; The blocked action or scope. Direct mutation controls are suppressed.
  - Surfaces: patch_review, provider_account | Consumers: product_ui, support_export, release_demo
- `term.reversibility.revert_undo_available` — Revert available [revert_undo_available / reversibility]: A prior, known-good state is retained, so an applied or proposed change can be reverted or undone through a named checkpoint. Requires: Checkpoint or revert-class ref; The revert or undo route.
  - Surfaces: patch_review, provider_account | Consumers: product_ui, support_export, narrated_announcement

## Forbidden high-trust phrases

- `forbidden.perfection.guaranteed` (perfection_guarantee): "guaranteed" — AI-inferred output is provisional and cannot promise guaranteed success or absence of risk. → term.confidence.low_confidence, term.validation.not_run
- `forbidden.perfection.perfect` (perfection_guarantee): "perfect" — AI-inferred output is provisional and cannot be described as perfect. → term.confidence.low_confidence, term.proposal.suggested
- `forbidden.review_free.no_review_needed` (review_free_mutation): "no review needed" — AI copy cannot waive the review and approval a mutation requires. → term.review.review_required, term.validation.not_run
- `forbidden.review_free.auto_apply` (review_free_mutation): "auto-apply" — AI copy cannot imply a change applies without review or approval. → term.review.review_required, term.proposal.draft
- `forbidden.autonomy.done_for_you` (false_autonomy): "done for you" — AI copy cannot imply the assistant autonomously finished the work. → term.proposal.suggested, term.review.review_required
- `forbidden.autonomy.fully_autonomous` (false_autonomy): "fully autonomous" — AI copy cannot claim autonomous completion on a trust-sensitive surface. → term.proposal.suggested, term.proposal.proposed
- `forbidden.validation.validated` (false_validation): "validated" — Validation language is reserved for a named validation state and outcome. → term.validation.not_run, term.validation.passed
- `forbidden.validation.safe_to_apply` (false_validation): "safe to apply" — Safety-to-apply requires a named validation outcome, not AI prose. → term.validation.not_run, term.review.review_required
- `forbidden.confidence.definitely` (confidence_overstatement): "definitely" — AI copy must state evidence and confidence class instead of pretending certainty. → term.confidence.low_confidence, term.proposal.suggested
- `forbidden.confidence.knows_the_codebase` (confidence_overstatement): "knows the codebase" — AI copy must not pretend inference is direct knowledge of the codebase. → term.context.context_used, term.confidence.low_confidence
- `forbidden.exhaustiveness.all_files` (false_exhaustiveness): "all files" — Scope breadth must come from a scope object, not AI prose. → term.context.context_used, term.confidence.low_confidence
- `forbidden.exhaustiveness.nothing_else_affected` (false_exhaustiveness): "nothing else affected" — Impact breadth must come from a scope object, not AI prose. → term.context.context_used, term.review.review_required
- `forbidden.freshness.up_to_date` (false_freshness): "up to date" — Freshness language must match the governing freshness state. → term.confidence.low_confidence, term.context.context_used
- `forbidden.freshness.latest_docs` (false_freshness): "latest docs" — Docs freshness must match the governing freshness state, not AI prose. → term.context.context_used, term.confidence.low_confidence

## Cross-consumer term reuse

- `term.confidence.low_confidence`: narrated_announcement, product_ui, support_export
- `term.context.context_used`: narrated_announcement, product_ui, support_export
- `term.proposal.draft`: docs_help, product_ui, support_export
- `term.proposal.proposed`: product_ui, release_demo, support_export
- `term.proposal.suggested`: narrated_announcement, product_ui, support_export
- `term.reversibility.revert_undo_available`: narrated_announcement, product_ui, support_export
- `term.review.review_required`: product_ui, release_demo, support_export
- `term.validation.not_run`: product_ui, release_demo, support_export
- `term.validation.passed`: docs_help, product_ui, support_export
