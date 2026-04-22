# Benchmark corpus governance and protected-metric change control

This document is the **normative** policy for benchmark-corpus
governance, protected-metric change control, and the benchmark-lab
change workflow. It is the human-readable companion to
[`/artifacts/bench/corpus_change_control.yaml`](../../artifacts/bench/corpus_change_control.yaml)
and
[`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml).

If this document disagrees with either machine-readable artifact, this
document wins and the YAML must be updated in the same change.

Companion artifacts:

- [`/artifacts/bench/corpus_change_control.yaml`](../../artifacts/bench/corpus_change_control.yaml)
  — machine-readable governance asset matrix, approval rules, CI gates,
  PR-separation rules, and changelog template.
- [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — revisioned protected-metrics file. Every row carries threshold
  snapshot, rationale, comparability note, calibration state, change
  authority, and public-reporting posture.
- [`/artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  — canonical benchmark hardware-row and display-class register.
- [`/artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml)
  — lab-image revision, benchmark-environment row, calibration
  checklist, and comparability-reset register.
- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — revisioned corpus manifest for every benchmark fixture.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — protected fitness-function catalog. Owns row identity and the wider
  measurement vocabulary; the protected-metrics file owns the governed
  threshold snapshot and change-control posture.
- [`/docs/benchmarks/public_comparison_rules.md`](./public_comparison_rules.md)
  — external publication and head-to-head comparison rules.
- [`/docs/benchmarks/benchmark_publication_pack_template.md`](./benchmark_publication_pack_template.md)
  — required packet shape when benchmark evidence leaves the raw
  dashboard or shiproom context.
- [`/docs/governance/benchmark_council_charter.md`](../governance/benchmark_council_charter.md)
  — forum that approves protected-path corpus and threshold changes.

## 1. Why this policy exists

Benchmark numbers are governance artifacts, not just engineering output.
If a corpus, threshold, hardware image, or public-comparison method can
drift casually, the benchmark lab stops being an audit trail and becomes
an optimization game. This policy exists so that:

1. protected workflow claims remain tied to one revisioned corpus
   manifest and one revisioned protected-metrics file;
2. threshold easing, corpus removal, and hardware/image recalibration
   carry structured rationale and evidence instead of hiding inside a
   feature change;
3. partner- or customer-derived inputs cannot silently enter CI or
   public packets without licensing, redaction, retention, and
   access-control review; and
4. public benchmark claims inherit one disclosure discipline rather than
   being reconstructed from dashboards, slides, and memory.

## 2. Scope

### In scope

- Governance rules for the protected benchmark corpus classes used on
  protected paths, including microbenchmark, workflow/archetype,
  remote/collaboration, and accessibility corpora.
- The revisioned protected-metrics file and its relationship to the
  fitness-function catalog.
- Change-control workflow for corpus addition/removal, threshold
  tightening/easing, hardware or lab-image recalibration, and public
  comparison changes.
- Changelog requirements for protected-path changes.
- Rules for customer-derived or third-party corpora entering CI or
  public benchmark packets.

### Out of scope

- Populating every future release-family or competitor-harness
  environment row. The seeded hardware and lab-image manifests exist in
  this milestone, but later milestones widen them.
- Marketing copy or a public benchmark website. This policy governs the
  evidence packets those surfaces would have to quote.
- Replacing the fitness-function catalog. The catalog remains the
  canonical row registry; this document governs how its protected
  thresholds and comparability posture are changed.

## 3. Governance asset matrix

Every governance asset below MUST name required metadata, change
authority, and a CI or review rule. The machine-readable form lives in
[`corpus_change_control.yaml`](../../artifacts/bench/corpus_change_control.yaml).

| Asset | Required metadata | Change authority | CI / review rule |
|---|---|---|---|
| **Microbenchmark corpus** | scenario description, hardware assumptions, hot-path focus, toolchain posture, license/redaction status | subsystem lead + performance owner | may not replace workflow corpora as sole proof |
| **Workflow / archetype corpus** | repo revision, toolchain versions, platform matrix, license/redaction status, protected journeys | performance owner + architecture board for protected sets | threshold and corpus changes reviewed on protected paths |
| **Remote / collaboration corpus** | network profile, region assumptions, remote image versions, reconnect expectations | remote lead + performance owner | outage and reconnect scenarios included |
| **Accessibility corpus** | assistive-tech setup, locale/input method, task script, task-fidelity note | accessibility owner + QA | no perf-only optimization may invalidate task fidelity |
| **Protected metrics file** | threshold value, rationale, comparability note, calibration state/date, change authority | performance owner + architecture board for threshold easing | easing thresholds requires before/after evidence |
| **Reference hardware manifest** | hardware row id, CPU/GPU/memory/storage summary, display-class ids, default power posture, council status | performance owner + release engineering | recalibration requires comparability note |
| **Lab-image / environment manifest** | lab-image revision, environment rows, calibration rule set, comparability-reset rules | performance owner + release engineering | recalibration requires comparability note |
| **Public benchmark publication pack** | exact command line, config, version, environment, competitor settings, protected-metrics revision | product + performance owner | raw run metadata retained for audit |

## 4. Single-source rule

Every reported benchmark run and every public benchmark packet MUST be
traceable to:

- exactly one corpus-manifest revision;
- exactly one protected-metrics-file revision; and
- exactly one fitness-function-catalog revision;
- exactly one hardware-definition row; and
- exactly one benchmark-environment row.

The corpus manifest answers "which bytes and scenarios were measured".
The protected-metrics file answers "which governed thresholds and
comparability notes were in force". The fitness-function catalog answers
"which row identities, data-source kinds, and waiver authorities those
measurements resolved against".

Any change that would break this linkage is a governance failure.

## 5. Change classes

The change classes below are frozen at this milestone. Their structured
rules live in
[`corpus_change_control.yaml`](../../artifacts/bench/corpus_change_control.yaml).

| Change class | Example | Minimum approvals | Required follow-through |
|---|---|---|---|
| **Corpus addition** | add a design-partner repo slice or a new workflow fixture | owner + performance owner + legal/privacy when external | changelog entry; note if CI admission is still restricted |
| **Corpus removal** | retire a stale protected fixture | owner + architecture board when protected | comparability note and release-evidence linkage |
| **Threshold easing** | widen startup or save latency bar | performance owner + architecture board | structured rationale, before/after evidence, release-evidence linkage |
| **Threshold tightening** | harden an existing bar | performance owner | calibration note and protected-metrics refresh |
| **Hardware/image recalibration** | new reference laptop SKU or OS image | performance owner + release engineering | comparability reset note and release-family boundary note |
| **Public comparison change** | change competitor version, plugin set, launch config, or task-parity method | performance owner + product owner | publication-pack refresh and comparison-rules review |

## 6. Workflow

1. Open the change against the canonical benchmark-governance assets,
   not in a quiet feature PR.
2. Record the affected corpus ids, fitness rows, protected-metrics rows,
   and publication-pack or release-evidence refs up front.
3. Capture before/after evidence for any threshold easing, corpus
   removal on protected paths, or hardware/image recalibration.
4. Record a comparability note explaining what remains comparable and
   what must reset.
5. Obtain the approvals required by the change class before merge.
6. Refresh the publication pack or release-evidence packet when the
   change narrows, widens, or re-baselines a public claim.

## 7. Protected-path rules

- A corpus, threshold, hardware-image, or public-comparison change MUST
  NOT ride hidden inside the same PR as the feature whose regression it
  could hide unless the benchmark council explicitly approves the bundle
  and the changelog records that exception.
- Threshold easing on a protected path MUST include a structured
  rationale, before/after evidence refs, approvals, a comparability
  note, and a release-evidence ref in the same change.
- Corpus removal on a protected path MUST include a comparability note,
  approvals, and release-evidence linkage in the same change.
- Microbenchmark improvements do not justify workflow-corpus regressions
  unless a benchmark-council trade-off record says so explicitly.
- Hardware or lab-image recalibration resets comparability until a new
  reference capture lands against the new baseline.

## 8. External and customer-derived corpora

External, partner-derived, or customer-derived corpora require extra
discipline before entering CI or public packets.

- Licensing review is mandatory. The provenance class, allowed use, and
  redistribution posture must be recorded before the corpus is admitted.
- Redaction review is mandatory. Raw customer identifiers, repository
  names, secrets, or otherwise non-public bytes MUST be replaced with
  redacted or synthetic equivalents before any public packet cites the
  corpus.
- Retention review is mandatory. The corpus must name a retention class
  and, when required, a segregated-storage or delete schedule.
- Access-control review is mandatory. A restricted corpus MUST NOT land
  in public CI or the public repository merely because its derived
  metrics are useful.
- Admission to CI is separate from feature delivery. A feature PR may
  reference the need for a new external corpus, but the corpus itself
  and its governance approvals land through the benchmark-governance
  path.

## 9. Protected-path changelog requirements

The machine-readable changelog template lives in
[`corpus_change_control.yaml`](../../artifacts/bench/corpus_change_control.yaml).
At minimum, any threshold easing or protected-path corpus removal MUST
record:

- change class and summary;
- affected corpus ids and/or protected-metric row ids;
- rationale;
- comparability note;
- approvals;
- before/after evidence refs; and
- linked release-evidence or benchmark-publication-pack ref.

A change that cannot fill those fields is not ready to merge.

## 10. Relationship to public comparison rules

This document governs how benchmark assets change. Public publication is
governed separately by
[`public_comparison_rules.md`](./public_comparison_rules.md), which
defines when a result may leave the internal dashboard context, what a
head-to-head comparison must disclose, and when a published comparison
must be refreshed or withdrawn.

## 11. What this document is not

- It is **not** the benchmark-council charter. The charter owns forum
  structure, cadence, and escalation.
- It is **not** the run-result schema. That boundary stays in
  [`benchmark_lab_run_results.md`](./benchmark_lab_run_results.md) and
  [`/schemas/benchmarks/run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json).
- It is **not** a public benchmark packet. That shape lives in
  [`benchmark_publication_pack_template.md`](./benchmark_publication_pack_template.md).
