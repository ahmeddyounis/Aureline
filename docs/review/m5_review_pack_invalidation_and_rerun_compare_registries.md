# M5 review-pack invalidation-record and rerun-compare registries

Invalidation / rerun / compare lane over the frozen [M5 review-pack evaluator
matrix][matrix] (`m5_review_pack_evaluator_matrix`). It keeps local review, hosted
review, CI, and AI review from letting stale evidence overclaim freshness: when the
base/head identity, worktree scope, review-pack version, or environment capsule changes
materially, the evaluated result is invalidated with the exact reason for the stale
transition, and a rerun-review / compare action lets a user inspect what changed between
the previous evaluator packet and the current base/head or pack revision without losing
draft-only notes or local evidence.

## Registry-A — review-pack invalidation record

One machine-readable invalidation record per stale transition, carrying:

- the exact invalidation cause — base-revision drift, head-revision drift,
  worktree-scope drift, review-pack version drift, review-pack content-digest drift, or
  environment-capsule drift;
- a stable pack identity that survives export packets, support bundles, and reopened
  review workspaces;
- the pack version and content digest — never dropped on export, publish, or reopen;
- the drifted base / head revision and worktree scope the evaluator ran against, and the
  target diff identity;
- the evaluator outcome and its lineage, kept mechanically distinct from
  provider-authoritative mergeability so a stale local parity estimate never reads as
  fresh, authoritative truth;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A record that cannot name its invalidation cause, that is a hand-copied per-entry
assumption instead of tracing to the shared registry, or that publishes an incomplete
object degrades honestly instead of reading as a fresh, authoritative review result. The
registry reuses the matrix `m5-review-pack.schema.json` domain schema.

## Registry-B — review-pack rerun / compare

The rerun-review and compare action a result binds, so a user can inspect what changed
between the previous evaluator packet and the current base/head or pack revision:

- the **previous** evaluator packet binding;
- the **current** base/head-or-pack-revision packet binding;
- the **preserved draft / local-evidence** binding — draft-only notes and local evidence
  carry forward marked stale rather than discarded or pretended current.

The compare carries the divergence labels the evaluator emits wherever it cannot claim
full coverage — partial-scope, slice-omitted, stale-pack, ci-only, and
provider-unavailable. The registry reuses the matrix `m5-review-pack-result.schema.json`
domain schema.

## Acceptance criteria proven by the resolved examples

1. At least one fixture per invalidation cause — base drift, head drift, pack revision
   drift, and environment-capsule drift — forces a visible stale transition with a named
   reason (the clean invalidation records cover all six causes; an incomplete or
   unnamed-cause record degrades).
2. Compare actions can inspect the previous and current evaluator packet without losing
   draft comments or local evidence: the previous-packet, current-packet, and
   preserved-draft-evidence rerun/compare bindings stay distinct across the canonical,
   accessible, and audit resolution forms.
3. No claimed review surface keeps queue eligibility, approval validity, or AI policy
   compliance green after a material pack / base / environment drift; the hard invariants
   and degrade ladder catch any surface that would.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-review-pack-invalidation-and-rerun-compare-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_review_pack_evaluator_matrix/mod.rs
