# M5 AI-Review Shared Consumers: One Vocabulary Across Surfaces

This lane is the B151 consumer-adoption capstone. It binds the four governed AI-review-assist objects frozen by
the [`m5_ai_review_assist_matrix`](m5-ai-review-assist-ops.md) — the **AI review finding row**, the **review
scope selector**, the **publish-to-review sheet**, and the **resolution memory row** — to the concrete
consumers that render them, and proves, by fixtures rather than screenshots, that the same seeded finding
carries one identical vocabulary wherever Aureline inspects, reruns, publishes, exports, or reopens AI review
output.

- Rust module: `aureline_ui::m5_ai_review_shared_consumers_one_vocabulary_across_surfaces`
- Boundary schema: [`schemas/review/m5-ai-review-shared-consumers.schema.json`](../../schemas/review/m5-ai-review-shared-consumers.schema.json)
- Proof bundle: `artifacts/review/m5-ai-review-shared-consumers-proof/` (`support_export.json`, `matrix.csv`, `summary.md`)
- Fixtures: `fixtures/review/m5-ai-review-shared-consumers/` (`compact_remote_narrowed.json`, `exported_redaction_narrowed.json`)
- Emitter: `cargo run -p aureline-ui --example dump_m5_ai_review_shared_consumers -- <support-export|report|csv|fixture-compact-remote-narrowed|fixture-exported-redaction-narrowed|validate>`

## Consumers

Nine shared consumer surfaces adopt the AI-review vocabulary: review detail, the AI review panel, the finding
row, the review scope selector, the publish-to-review sheet, the pending-review tray, the provider
publish-review surface, the resolution-memory ledger, and the support / export packet. Each of the four objects
is adopted by at least two distinct consumers, so an object is proven to be shared review infrastructure rather
than a one-surface fork.

## One vocabulary, no drift

For a given seeded finding, every consumer surface must present identical `AiReviewStateFacetValues`: the same
AI-review-role word, object word, registry-reference word, publish-state word, surface-context word, and
finding-lifecycle word. The AI-review-role word must be a token from the frozen `M5AiReviewAssistRole`
vocabulary (`finding_classification`, `analyzed_scope_disclosure`, `publish_destination_disclosure`,
`local_versus_provider_state`, `lifecycle_state_tracking`, `publish_export_fallback`,
`resolution_memory_disclosure`), so no surface invents an alternate label for publish state, analyzed scope, or
finding lifecycle.

A role that carries finding-classification, analyzed-scope, publish-destination, or local-versus-provider
meaning is a **gate role**: it must pair its surface presentation with a real
`finding_current_scope_bound_and_destination_disclosed` continuity and never collapse to a stale sentinel
(`outdated_finding_shown_as_current`, `suppressed_finding_shown_as_active`,
`local_draft_lost_when_publish_failed`, `provider_destination_hidden`).

## Narrowing is disclosed

A surface may narrow *how much* it renders across the desktop-full, compact, remote-projected, and
exported-redacted representations, but never reword the vocabulary. Every narrowed representation carries an
explicit `AiReviewNarrowNote` naming the reason, the preserved vocabulary, and the next action; remote and
exported forms additionally name their remote-source and export-safe-detail boundaries.

## Map back to one object

Support / export consumers point at the canonical per-domain schema and the frozen matrix by id, so an exported
packet — and every copy / export / open-in-provider action — maps back to one shared contract object rather
than diverging into a surface-local payload.

## Guardrails

Each binding re-asserts the matrix's five hard invariants (all MUST be `false`): it never lets AI review
results publish or merge implicitly, never hides whether output stays local or becomes a provider comment /
suggested patch / check annotation, never keeps a stale finding looking current after diff or instruction
drift, never loses local drafts or evidence when provider write scope is missing or a publish fails, and never
presents an AI review finding without its analyzed scope, publish destination, or lifecycle state.

## Acceptance criteria mapping

1. **The same seeded finding shows the same state vocabulary and stable ID across surfaces** — enforced by the
   per-finding facet identity and the `ai_review_vocabulary_drift_across_surfaces` violation.
2. **No claimed consumer surface invents alternate labels** — enforced by the frozen-role-token gate
   (`ai_review_role_word_outside_vocabulary`) over the shared publish-state / scope / lifecycle words.
3. **Copy / export / open-provider actions preserve one canonical payload** — enforced by
   `points_at_canonical_contracts` and the `support_export_reference_missing` violation.
