# M5 review-pack-record and review-pack-result registries

First implement lane over the frozen [M5 review-pack evaluator matrix][matrix]
(`m5_review_pack_evaluator_matrix`). It makes the matrix's `review_pack_record` and
`review_pack_result` object classes operable by carrying resolved, honest projections of
two registries so review, AI review, provider-backed review, and support / export surfaces
inherit one canonical set of declarative review packs and their evaluator results rather than
a hand-authored parallel prose that has to be kept consistent.

## Registry-A — review-pack record

One machine-readable review-pack record per repo-defined review pack, carrying:

- a stable pack identity that survives export packets, support bundles, and reopened review
  workspaces;
- the pack version and content digest — never dropped on export, publish, or reopen;
- the scope selector (changed files, pull / merge request, base..head range, worktree
  uncommitted, full tree, saved pack snapshot);
- the target diff identity and the worktree / base revision the evaluator ran against;
- the evaluator outcome and its lineage, kept mechanically distinct from provider-authoritative
  mergeability so a local parity estimate never reads as authoritative;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A record that cannot bind its evaluator outcome to a classified scope selector, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that publishes
an incomplete object degrades honestly instead of reading as a fresh, authoritative review
result. The registry reuses the matrix `m5-review-pack.schema.json` domain schema.

## Registry-B — review-pack result

The typed evaluator event that names which evaluated scope, pack version / digest, or
divergence label a result binds, so a changed evaluated scope, pack digest, or divergence
label stays a visible, typed event rather than a silent mutation that leaves a stale pack
looking fresh. The result carries the divergence labels the evaluator emits wherever it cannot
claim full coverage — partial-scope, slice-omitted, stale-pack, ci-only, and provider-unavailable.
The registry reuses the matrix `m5-review-pack-result.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. At least one human-readable packet (the Markdown summary) and one machine-readable packet
   (the support export) preserve pack version / digest, scope selectors, divergence labels, and
   evaluator lineage end to end; a record with an incomplete pack object degrades.
2. Pack freshness and scope-omission states (stale-pack, partial-scope, slice-omitted, ci-only,
   provider-unavailable) stay visible in the review UI projection, the CSV / export, and the
   support packet instead of being flattened into a generic stale review state.
3. No evaluated review result survives a pack / base / worktree drift without an explicit
   stale-pack or stale-relative-to-base/head transition; the binding registry keeps each
   evaluator-result dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-review-pack-record-and-result-registries.schema.json`) documents the
shape.

[matrix]: ../../crates/aureline-ui/src/m5_review_pack_evaluator_matrix/mod.rs
