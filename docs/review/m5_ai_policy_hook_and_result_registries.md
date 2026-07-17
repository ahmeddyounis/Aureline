# M5 AI-policy-hook and AI-policy-result registries

AI-review-policy-hook implement lane over the frozen [M5 review-pack evaluator matrix][matrix]
(`m5_review_pack_evaluator_matrix`). It makes the matrix's `ai_policy_hook` object class operable
by carrying resolved, honest projections of two registries so review detail, the AI review panel,
the review-pack summary, the local-CI parity strip, provider handoff, and support / export
surfaces inherit one canonical AI-review-policy model — allowed analyzers, severity thresholds,
suppression classes, and mandatory citation requirements all bound to the active review-pack
version / digest — rather than a hand-authored parallel prose that has to be kept consistent.

## Registry-A — AI-policy-hook

One machine-readable policy hook per AI review run, carrying the **policy-hook facet** it binds so
each facet resolves through the same review-pack version and evaluator lineage as human, local,
and CI review:

- `allowed_analyzer` — the analyzer set the active pack allows the AI run to use;
- `severity_threshold` — the severity threshold the pack applies to AI findings;
- `suppression_class` — the suppression class the pack authorises;
- `mandatory_citation` — the mandatory citation requirement the pack imposes;
- `experimental_analyzer` — the AI run used an experimental / narrower analyzer capability than the
  declared pack (a divergent-capability state that must be disclosed);
- `policy_downgraded` — the AI run operated under a downgraded policy relative to the declared pack
  (a divergent-capability state that must be disclosed).

The `experimental_analyzer` and `policy_downgraded` facets are provider-truth-sensitive: they
surface directly in the user-facing disclosure language a hook publishes, so their claim must stay
matched to the pack version the run actually resolved through. A hook that cannot name the pack
version it resolved through, that is a hand-copied per-run assumption instead of tracing to the
shared registry, or that would apply a suppression class, severity threshold, or citation
expectation from a different or stale pack revision degrades honestly instead of running AI review
under an undisclosed pack version. The registry reuses the matrix `m5-ai-policy-hook.schema.json`
domain schema.

## Registry-B — AI-policy-result

The typed outcome an AI review run inherits from the governing pack, naming which **result
binding** it carries so a rerun or an outdated finding stays attributable to the pack that governs
it:

- `analyzer_result_class_binding` — the analyzer result class the run holds: full, experimental,
  or policy-downgraded;
- `pack_version_digest_binding` — the active review-pack version and content digest the run
  resolved through;
- `rerun_staleness_binding` — the rerun / staleness relationship a prior finding holds after a pack
  change: current, rerun-required-after-pack-change, or stale-after-pack-change.

The result keeps the analyzer result class and the pack version / digest so an exported review /
support packet can be read without the live UI having to re-interpret whether the run was full,
experimental, or policy-downgraded. The registry uses the minted `m5-ai-policy-result.schema.json`
domain schema.

## Acceptance criteria proven by the resolved examples

1. At least one AI review run shows the active review-pack version / digest, the analyzer class,
   and whether the result is full, experimental, or policy-downgraded: the
   `pack_version_digest_binding` and `analyzer_result_class_binding` results and the
   `experimental_analyzer` / `policy_downgraded` hook facets stay mechanically distinct across the
   rows, and a hook that hides a divergent run behind a full reading degrades.
2. AI review cannot silently apply a suppression class, severity threshold, or citation
   expectation from a different or stale review-pack revision: the `suppression_class`,
   `severity_threshold`, and `mandatory_citation` facets are bound to the pack version / digest,
   and a hook whose pack-version / evaluator-lineage join is not preserved degrades.
3. Reruns after pack changes mark prior AI findings stale or rerun-required rather than preserving
   them as current pack-compliant evidence: the `rerun_staleness_binding` result carries
   `rerun_required_after_pack_change` and `stale_after_pack_change` states, and the support export
   and the narrowed fixtures carry the result class and staleness relationship end to end.

Raw prompts, secret values, and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-ai-policy-hook-and-result-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_review_pack_evaluator_matrix/mod.rs
