# M5 ownership-signal-row and owner-conflict registries

Ownership-signal implement lane over the frozen [M5 review-pack evaluator matrix][matrix]
(`m5_review_pack_evaluator_matrix`). It makes the matrix's `ownership_signal` object class
operable by carrying resolved, honest projections of two registries so review lists, review
detail, merge-readiness, AI review, browser handoff, and support / export surfaces inherit one
canonical ownership model — advisory versus enforced owner, repo-rule versus graph-overlay
versus provider-metadata provenance, and reviewer rationale — rather than a hand-authored
parallel prose that has to be kept consistent.

## Registry-A — ownership-signal row

One machine-readable ownership-signal row per owned slice, carrying an owner **source class**
that is never flattened:

- `codeowners_rule_owner` — the owner came from a repo rule (CODEOWNERS);
- `graph_overlay_maintainer` — the owner came from a graph-derived maintainer overlay;
- `provider_suggested_reviewer` — the owner came from provider metadata / suggested reviewers;
- `enforced_review_gate_owner` — an enforced-owner review gate;
- `advisory_area_owner` — an advisory area owner (non-blocking);
- `fallback_default_owner` — the fallback / default owner when no more specific owner applies.

Each row keeps advisory-owner and enforced-owner mechanically distinct (matrix
`OwnerAuthority::is_enforced`) so an advisory owner is never promoted into an enforced merge
gate, and keeps the owner **provenance** — repo rule, graph overlay, or provider metadata —
attributable to the source. A row that cannot bind its owner to a classified source class, that
is a hand-copied per-entry assumption instead of tracing to the shared registry, or that
publishes an incomplete object degrades honestly instead of reading as an authoritative owner
signal. The registry reuses the matrix `m5-ownership-signal.schema.json` domain schema.

## Registry-B — owner-conflict reconciliation

The typed reconciliation event that names which reconciliation binding a result carries so a
disagreement between a CODEOWNERS repo rule, a graph-derived maintainer, and a provider
suggestion stays a visible, explained event rather than a silent last-writer-wins collapse:

- `owner_authority_binding` — the advisory-versus-enforced authority decision;
- `owner_source_provenance_binding` — the repo-rule / graph-overlay / provider-metadata provenance;
- `owner_conflict_rationale_binding` — the explicit winning-versus-advisory relationship and the
  reviewer rationale that explains it.

The reconciliation preserves the owner source class and rationale so an exported review / support
packet can be read without the live UI having to re-interpret the result. The registry uses the
minted `m5-owner-conflict.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Ownership surfaces never collapse advisory-owner and enforced-owner into one ambiguous owner
   label: the `advisory_area_owner` and `enforced_review_gate_owner` source classes and the
   `owner_authority_binding` reconciliation stay mechanically distinct across every row, and a
   record that would flatten them degrades.
2. At least one conflicting-owner packet shows repo rule (`codeowners_rule_owner`), graph overlay
   (`graph_overlay_maintainer`), and provider suggestion (`provider_suggested_reviewer`)
   simultaneously with an explicit winning or advisory relationship carried by the
   `owner_conflict_rationale_binding` reconciliation.
3. Exported review / support packets preserve the owner source class and reviewer rationale
   without requiring the live UI to interpret the result — the support export and the narrowed
   fixtures carry the source classes and the reconciliation rationale end to end.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-ownership-signal-and-conflict-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_review_pack_evaluator_matrix/mod.rs
