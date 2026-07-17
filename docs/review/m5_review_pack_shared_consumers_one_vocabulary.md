# M5 Review-Pack Shared Consumers: One Vocabulary Across Surfaces

This lane is the B152 consumer-adoption capstone. It binds the six governed review-pack evaluator objects
frozen by the [`m5_review_pack_evaluator_matrix`](m5-review-pack-evaluator-ops.md) — the **review-pack
record**, the **ownership signal**, the **required-evidence / required-check row**, the **local-CI parity
strip**, the **AI review policy hook**, and the **review-template packet** — to the concrete consumers that
render them, and proves, by fixtures rather than screenshots, that the same seeded review-pack subject carries
one identical vocabulary wherever Aureline reviews, checks merge-readiness, runs AI review, hands off to a
provider, exports, or reopens review-pack evidence.

- Rust module: `aureline_ui::m5_review_pack_shared_consumers_one_vocabulary_across_surfaces`
- Boundary schema: [`schemas/review/m5-review-pack-shared-consumers.schema.json`](../../schemas/review/m5-review-pack-shared-consumers.schema.json)
- Proof bundle: `artifacts/review/m5-review-pack-shared-consumers-proof/` (`support_export.json`, `matrix.csv`, `summary.md`)
- Fixtures: `fixtures/review/m5-review-pack-shared-consumers/` (`compact_remote_narrowed.json`, `exported_redaction_narrowed.json`)
- Emitter: `cargo run -p aureline-ui --example dump_m5_review_pack_shared_consumers -- <support-export|report|csv|fixture-compact-remote-narrowed|fixture-exported-redaction-narrowed|validate>`

## Consumers

Nine shared consumer surfaces adopt the review-pack vocabulary: review detail, the merge-readiness component,
the AI review panel, the provider-handoff surface, the review-pack summary, the ownership overlay, the local-CI
parity strip, the support / export packet, and the help / docs surface. Each of the six objects is adopted by
at least two distinct consumers, so an object is proven to be shared review infrastructure rather than a
one-surface fork.

## One vocabulary, no drift

For a given seeded review-pack subject, every consumer surface must present identical
`ReviewPackSharedStateFacetValues`: the same review-pack-role word, object word, registry-reference word,
parity-state word, surface-context word, and pack-freshness word. The review-pack-role word must be a token
from the frozen `M5ReviewPackRole` vocabulary (`pack_version_and_digest_disclosure`,
`owner_provenance_disclosure`, `evaluator_result_class_disclosure`,
`local_versus_provider_parity_disclosure`, `required_evidence_and_check_disclosure`,
`template_attribution_disclosure`, `pack_freshness_and_invalidation_disclosure`), so no surface invents an
alternate label for local parity, ownership source, or stale-pack state.

A role that carries pack-version / digest, owner-provenance, evaluator-result-class, or
local-versus-provider meaning is a **gate role**: it must pair its surface presentation with a real
`pack_fresh_scope_bound_and_parity_disclosed` continuity and never collapse to a stale sentinel
(`stale_pack_shown_as_fresh`, `partial_scope_shown_as_full`,
`local_estimate_shown_as_provider_authoritative`, `advisory_owner_shown_as_enforced`).

## Narrowing is disclosed

A surface may narrow *how much* it renders across the desktop-full, compact, remote-projected, and
exported-redacted representations, but never reword the vocabulary. Every narrowed representation carries an
explicit `ReviewPackSharedNarrowNote` naming the reason, the preserved vocabulary, and the next action; remote
and exported forms additionally name their remote-source and export-safe-detail boundaries.

## Map back to one object

Support / export consumers point at the canonical per-domain schema and the frozen matrix by id, so an exported
packet — and every copy / export / open-in-provider action — maps back to one shared contract object rather
than diverging into a surface-local payload.

## Guardrails

Each binding re-asserts the matrix's five hard invariants (all MUST be `false`): it never lets a local parity
estimate masquerade as provider-authoritative, never hides a ci-only / not-evaluated-here /
provider-unavailable state behind a green summary, never flattens advisory-owner and enforced-owner into one
owner pill, never lets AI review run under a different pack version without disclosure, and never loses the
review-pack version / digest or template attribution when exporting, publishing, or reopening review evidence.

## Acceptance criteria mapping

1. **The first shared consumers render the same state names, field ordering, and recovery affordances for
   pack / owner / parity / template truth** — enforced by the per-subject facet identity and the
   `review_pack_vocabulary_drift_across_surfaces` violation.
2. **Help / docs and support / export packets no longer invent alternate wording for local parity, ownership
   source, or stale-pack state** — enforced by the frozen-role-token gate
   (`review_pack_role_word_outside_vocabulary`) over the shared parity-state / owner / pack-freshness words and
   the reuse of the same field names in this schema and the live UI.
3. **No first consumer silently drops pack version/digest, owner provenance, or template attribution while
   another consumer still shows it** — enforced by `points_at_canonical_contracts`, the
   `support_export_reference_missing` violation, and the `pack_freshness_missing_for_gate_role` gate.
