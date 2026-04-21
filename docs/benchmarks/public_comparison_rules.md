# Public benchmark comparison rules

This document is the **normative** rule set for any benchmark result
that leaves Aureline's internal dashboard context. It governs
methodology-only publication, Aureline-only claim packets, and public
head-to-head comparisons.

Companion artifacts:

- [`/docs/benchmarks/benchmark_publication_pack_template.md`](./benchmark_publication_pack_template.md)
  — required packet shape for public benchmark evidence.
- [`/docs/benchmarks/corpus_governance.md`](./corpus_governance.md)
  — change-control and protected-metric governance policy.
- [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — revisioned protected-metrics file every claim-bearing packet cites.
- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — revisioned corpus manifest every packet cites.
- [`/docs/governance/benchmark_council_charter.md`](../governance/benchmark_council_charter.md)
  — approval forum for protected-path disputes and threshold changes.

## 1. Allowed publication postures

| Publication posture | What it allows | Minimum evidence posture |
|---|---|---|
| `methodology_only` | Explain how Aureline measures a task without claiming a public win | any run context, but packet must say it is not claim-bearing |
| `aureline_only_claim` | State Aureline's own measured result on a protected task | reference capture or an explicitly narrowed methodology note |
| `public_head_to_head_comparison` | Compare Aureline against another product on a named task | reference capture, comparable methodology, full disclosure fields |
| `quarantined_not_comparable` | Explain why a previously cited comparison is no longer comparable | quarantined or reset evidence only; no performance conclusion |

## 2. Required disclosure for head-to-head comparisons

Every public head-to-head comparison MUST disclose:

- exact build identity refs for Aureline and the compared product or
  toolchain state when exact-build identity does not exist for the other
  side;
- exact command line and material configuration knobs;
- corpus-manifest revision and protected-metrics revision;
- task parity note, exclusions, and declared non-applicable features;
- competitor version, plugin or extension posture, and install or
  launch posture;
- docs/help version-match state for the Aureline build being cited; and
- stable refs to the raw run metadata needed for independent review.

Omitting any of those fields converts the packet to
`methodology_only` or `quarantined_not_comparable`; it does not remain
claim-bearing by implication.

## 3. Prohibited practices

- Do not publish a head-to-head comparison from a `self_capture` or
  `smoke_subset` run.
- Do not compare against a different corpus-manifest or
  protected-metrics revision without saying the comparison is narrowed
  or reset.
- Do not hide a threshold easing, corpus removal, or hardware/image
  recalibration inside the same feature PR that benefits from it.
- Do not use a microbenchmark-only win to overrule a workflow-corpus
  regression without an explicit benchmark-council trade-off record.
- Do not omit competitor plugins, launch flags, or feature disables
  that materially affect the result.
- Do not publish customer-derived or partner-derived raw bytes, names,
  or identifiers in a public packet unless written approval says they
  may be disclosed.

## 4. External, partner-derived, and customer-derived inputs

When an external or customer-derived corpus contributes to a public
comparison:

- licensing, redaction, retention, and access-control review are
  mandatory before the corpus enters CI;
- public packets cite only stable corpus ids or approved redacted
  labels, not private customer or repository names;
- retention posture must be recorded before the first public packet
  cites the corpus; and
- the governance change for the corpus lands separately from the
  feature PR unless the benchmark council records an explicit bundled
  exception.

## 5. Refresh and withdrawal triggers

A public benchmark packet MUST be refreshed or withdrawn when any of the
following change in a way that affects comparability:

- corpus-manifest revision;
- protected-metrics revision;
- reference hardware or lab-image definition;
- Aureline docs/help applicability;
- competitor version, plugin posture, or launch posture;
- active waiver or advisory that narrows the claim; or
- comparability class changes to `quarantined` or `not_yet_comparable`.

## 6. Approval path

- Methodology-only packets require performance-owner review.
- Aureline-only public claim packets require performance-owner review
  and the normal publication-pack review.
- Public head-to-head comparison changes require performance-owner and
  product-owner approval, plus any legal/privacy review triggered by the
  corpus used.

## 7. Relationship to the publication pack

These rules define what is allowed to be claimed. The publication-pack
template defines how the claim is documented. A comparison is not ready
to publish until both documents are satisfied in the same change.
