# M5 required-evidence-check and local-CI-parity registries

Third implement lane over the frozen [M5 review-pack evaluator matrix][matrix]
(`m5_review_pack_evaluator_matrix`). It makes the matrix's `required_evidence_check_row`
and `local_ci_parity_strip` object classes operable by carrying resolved, honest
projections of two registries so review, AI review, provider-backed review, and
support / export surfaces inherit one canonical model of required checks and local/provider
check parity rather than a hand-authored parallel prose that has to be kept consistent.

## Registry-A — required-evidence-check row

One machine-readable required-evidence-check row per required check, carrying:

- the required check identity — a must-run test, scanner, docs / migration note, incident
  link, or rollout note;
- the evidence-check state, kept mechanically distinct so the eight states never collapse
  into one success / failure bucket: `required`, `optional`, `skipped`, `suppressed`,
  `timed_out`, `ci_only`, `not_evaluated_here`, and `provider_unavailable`;
- whether Aureline evaluated the check locally, imported it, or could not evaluate it here;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A row that would collapse an unevaluated (`skipped` / `not_evaluated_here`), muted
(`suppressed`), interrupted (`timed_out`), provider-only (`ci_only`), or
`provider_unavailable` check into a green pass, or that is a hand-copied per-check assumption
instead of tracing to the shared registry, degrades honestly instead of reading as satisfied.
The registry reuses the matrix `m5-review-pack-result.schema.json` domain schema.

## Registry-B — local-CI-parity strip

The typed strip that compares the local parity estimate against the provider-authoritative
state and names the capability difference between them, so a local estimate never widens into
provider-authoritative or queue-eligible mergeability from one green summary. It classifies
each strip as a `local_parity_estimate_binding`, a `provider_authoritative_binding`, or a
`capability_difference_binding`, and the capability difference names environment, secrets,
runner class, service dependencies, branch protections, or provider-only merge simulation.
The registry reuses the matrix `m5-local-ci-parity.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Checks and evidence surfaces distinguish `required`, `optional`, `skipped`, `suppressed`,
   `timed_out`, `ci_only`, `not_evaluated_here`, and `provider_unavailable` without collapsing
   them into one success / failure bucket — all eight canonical states are exercised by clean
   rows, and a row that publishes an incomplete object degrades.
2. A local parity estimate can never render as provider-authoritative or queue-eligible without
   an explicit state change backed by provider evidence; the strip keeps the local estimate and
   the provider-authoritative state mechanically distinct.
3. At least one fixture exercises capability-difference compare for environment, secrets, and
   provider-only merge-simulation deltas; the binding registry keeps each parity dimension
   distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-required-evidence-check-and-local-ci-parity-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_review_pack_evaluator_matrix/mod.rs
