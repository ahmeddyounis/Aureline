# Reference hardware and corpora — object model and identity contract

This document is the **normative** reference for the standalone benchmark
*objects* — corpus manifests, reference-hardware profiles, lab-image revisions,
and protected metrics — that every claimed benchmark and certification row binds
to. It explains the stable-id contract, what fails closed, and how release,
support, and evaluation surfaces cite the same identities.

It sits underneath the
[benchmark-governance matrix](./m5-benchmark-governance.md), which is the
canonical truth source for *which* metric binds *which* corpus, hardware, and
lab image, and for the closed claim-narrowing vocabulary. This page defines the
object shapes those bindings are made of. If this document disagrees with the
machine-readable schemas or matrix, the schemas and matrix win and this document
is updated in the same change.

## Why these objects exist

A benchmark or certification claim is only trustworthy when its corpus
identity, hardware class, lab-image revision, and threshold history are
canonical, reproducible, and freshness-governed. Without one governed object
model, performance-bearing rows drift on tribal knowledge: a metric "ran on the
macOS laptop" against "the usual corpus" with no resolvable revision. This lane
replaces that with stable ids: every protected metric names a corpus-manifest
revision, a reference-hardware profile, and a lab-image revision that resolve in
the canonical registers, or it cannot make a comparable claim.

## The objects and their schemas

Each object is a single self-describing record validated by its own boundary
schema. The schemas carry no credential bodies, raw machine labels, raw customer
or repository names, or raw run logs — only stable ids and reviewable sentences.

| Object | Schema | What it identifies |
| --- | --- | --- |
| Corpus manifest | [`corpus-manifest.schema.json`](../../schemas/benchmarks/corpus-manifest.schema.json) | One governed corpus revision: id, class, bound revision, license / redaction / retention / access posture, permitted workload classes, owner, freshness rule. |
| Reference-hardware profile | [`reference-hardware-profile.schema.json`](../../schemas/benchmarks/reference-hardware-profile.schema.json) | One machine class a claim may cite: id, `reference_lab` vs `self_capture`, display class, default power posture, paired lab-image revision, council status, permitted workload classes. |
| Protected metric | [`protected-metric.schema.json`](../../schemas/benchmarks/protected-metric.schema.json) | One claim-bearing metric bound to its corpus ids, hardware profile, lab-image revision, threshold id and state, owner, waiver, freshness, claim ceiling, and effective claim. |

Lab-image revisions live in the canonical lab-image register and are referenced
by id (`lab_image_ref` + `lab_image_revision`) from hardware profiles and
protected metrics; a recalibration bump resets comparability until a fresh
reference capture lands.

## Canonical registers (source of truth for ids)

Object ids are not minted per lane. They must resolve in these canonical
registers, which the governance validator reads to reject ad hoc identities:

- [`fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — corpus ids and the current `manifest_revision`.
- [`artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  — hardware rows, row classes, and display classes.
- [`artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml)
  — lab-image revisions, environments, power postures, and calibration reset rules.
- [`artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — protected-metric ids and threshold history.

## What fails closed

Benchmark publication fails closed when a required identity is missing. The
[validator](../../ci/check_m5_benchmark_governance.py) and the object schemas
enforce, at minimum:

- **No corpus** — a protected metric with an empty corpus binding is rejected at
  the schema boundary; it cannot claim a result against an un-named corpus.
- **No hardware identity** — a metric or profile with no reference-hardware
  profile or display class is rejected; an anonymous machine carries no claim.
- **No lab-image identity** — a metric with no lab-image revision is rejected;
  comparability has no baseline.
- **Unresolved id** — a corpus, hardware, or lab-image id that is not in the
  canonical register is rejected, so a local developer machine or ad hoc fixture
  set cannot stand in for a protected reference row.
- **Stale corpus revision** — a corpus bound behind the current register
  revision narrows the claim until it is rebaselined.

## The self-capture guardrail

`self_capture` rows exist so local exports stay honest, but they are directional
only. The guardrail keeps them from masquerading as protected reference rows:

- A `self_capture` profile MUST declare
  `council_status: not_reference_eligible_without_promotion`. A self-capture row
  that claims `approved_reference_baseline` is rejected.
- A claim-bearing metric or publication pack (an `aureline_only_claim` or
  `public_head_to_head_comparison`) MUST NOT cite a `self_capture` hardware or
  lab-image identity. Self-capture evidence cannot carry a public claim.

## Claim narrowing and publication holds

The matrix's claim-narrowing engine sets a metric's effective claim to the
lowest of its published ceiling and every fired narrowing rule. The validator
recomputes this and fails when the stored effective claim disagrees, when a
detected narrowing reason is unreported, or when a claim-bearing publication
pack rests on a metric that has narrowed below a claim-bearing level
(publication holds until the metric recovers). See
[the matrix policy](./m5-benchmark-governance.md) for the closed narrowing
vocabulary and the per-rule downgrade targets.

## One identity, many consumers

Release, support/export, docs, and help all consume the **same** stable ids
through the matrix's consumer bindings, never a private copy:

- **Release / shiproom** reads each metric's effective claim and any active
  narrowing reasons, and holds promotion when a claim-bearing pack rests on a
  narrowed metric.
- **Support / export** carries effective claims, narrowing reasons, and the
  bound corpus, hardware, and lab-image ids only — never raw run logs, raw
  machine labels, or raw provider payloads.
- **Docs / help** render the effective claim and downgrade label for each metric
  and pack rather than cloning threshold or comparability prose.

## Fixtures

[`fixtures/benchmarks/reference-hardware/`](../../fixtures/benchmarks/reference-hardware/)
holds object fixtures the validator replays each run: conforming reference
objects plus fail-closed negatives (missing identity, stale revision, no corpus,
self-capture masquerade, and a claim-bearing metric on self-capture hardware).
The matrix's narrowing fixtures live alongside in
[`fixtures/benchmarks/m5-benchmark-governance/`](../../fixtures/benchmarks/m5-benchmark-governance/).

## Changing an object

- Adding a corpus, hardware, or lab-image id is additive; it must land in the
  canonical register first, then be bound from a metric.
- Renaming an id is breaking: it opens a decision record and updates the
  register, the matrix, the schema examples, and this document in the same change.
- Easing a threshold or widening a claim requires the before/after evidence and
  approvals defined in the
  [threshold-change policy](./threshold-change-policy.md) and recorded in the
  [threshold-change ledger](../../artifacts/benchmarks/threshold-change-ledger.json);
  this row does not widen the number of claimed archetypes.
