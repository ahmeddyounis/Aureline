# Experiment Provenance, Dataset Summary, and Result Comparison Contract

Experiment evidence is a governed record family. Benchmark claims, design
studies, AI evaluations, performance investigations, and public-proof packets
must cite reproducible experiment records rather than screenshots, slide-only
summaries, or notebooks whose context has to be reconstructed by hand.

Authoritative design anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` Appendix AP,
  "Experiment provenance, dataset summary, and result-comparison matrix."
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` section 17.21 and Appendix AT,
  covering experiment provenance, dataset governance, and honest comparison.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` sections 16.44 and
  Appendix AX, covering run rows, dataset provenance cards, and comparison
  table templates.
- `.t2/docs/Aureline_Milestones_Document.md` sections on corpus, benchmark,
  public-proof, and evidence-lineage governance.

Companion machine-readable artifacts:

- [`/schemas/research/experiment_record.schema.json`](../../schemas/research/experiment_record.schema.json)
- [`/schemas/research/dataset_summary.schema.json`](../../schemas/research/dataset_summary.schema.json)
- [`/schemas/research/result_comparison.schema.json`](../../schemas/research/result_comparison.schema.json)
- [`/fixtures/research/experiment_cases/`](../../fixtures/research/experiment_cases/)

Related upstream contracts:

- [`/docs/benchmarks/corpus_governance.md`](../benchmarks/corpus_governance.md)
  and [`/docs/benchmarks/public_comparison_rules.md`](../benchmarks/public_comparison_rules.md)
  govern benchmark corpus changes and public comparison disclosure.
- [`/docs/benchmarks/benchmark_lab_run_results.md`](../benchmarks/benchmark_lab_run_results.md)
  and [`/schemas/benchmarks/run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json)
  provide the benchmark-lab run-result boundary this family may cite.
- [`/docs/qe/public_proof_scoreboards.md`](../qe/public_proof_scoreboards.md)
  and [`/schemas/qe/public_proof_packet.schema.json`](../../schemas/qe/public_proof_packet.schema.json)
  provide the public-proof packet boundary this family may feed.
- [`/docs/ai/evidence_replayability_contract.md`](../ai/evidence_replayability_contract.md)
  and [`/docs/ai/model_graduation_and_budget_contract.md`](../ai/model_graduation_and_budget_contract.md)
  provide the AI evidence and rollout packet refs this family may cite.
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  binds claim rows to proof refs and downgrade behavior.

## Scope

This family governs metadata for:

- benchmark, performance, and regression experiments;
- AI eval, red-team, model-routing, and prompt-pack evaluation runs;
- design research, usability, compare-truth, and partner-study evidence;
- notebook-adjacent experiments and data-analysis runs;
- public-proof and claim-manifest evidence that needs reproducible lineage.

This family does not run experiments, store raw datasets, implement dashboards,
or replace benchmark/public-proof packet schemas. It defines the shared record
shape those systems cite when they need to prove what was measured, on which
data, under which environment, with which comparison caveats.

Raw prompts, raw notebooks, raw customer data, raw logs, raw screenshots, raw
profile traces, raw repository paths, raw URLs, credentials, and private
participant identifiers do not cross this boundary. Records carry opaque refs,
summary labels, governed classes, counts, dates, digest refs, and safe review
sentences.

## Record Family

The family has three first-class records:

| Record | Schema | Primary job |
|---|---|---|
| Experiment record | `experiment_record.schema.json` | Binds a run to hypothesis, owner, source, build, environment, hardware/profile, corpus or dataset refs, parameters, seed, approvals, waivers, outcome, and publication posture. |
| Dataset summary | `dataset_summary.schema.json` | Describes what data was used without exposing raw data: source class, snapshot, slice size, redaction/sensitivity, license, refresh date, exclusions, and comparability notes. |
| Result comparison | `result_comparison.schema.json` | Joins baseline and candidate runs, records metric deltas, variance/significance notes, disqualifiers, and presentation-safe summaries that keep changed corpus, dataset, environment, hardware, build, parameter, waiver, and disqualifier axes visible. |

Each record carries a frozen `record_kind`, schema version, stable id, owner or
steward refs, and monotonic timestamps. New consumers should cite these records
by id instead of copying their details into lane-specific notes.

## Experiment Records

An experiment record must expose the following without free-form reconstruction:

- `hypothesis` and `experiment_family` naming what the run was intended to
  prove or disprove;
- `owner` with primary and evidence owner refs;
- `source_refs` pointing to notebooks, scripts, task runs, benchmark harness
  rows, design-study packets, or imported artifacts;
- `data_bindings` containing dataset-summary refs and, when applicable,
  benchmark corpus manifest refs and corpus ids;
- `hardware_profile`, `environment_fingerprint`, and `build_refs`, including
  exact-build identity where available;
- `parameter_set` with digest, changed parameter refs, and the required seed
  value or explicit `null` when the run is intentionally unseeded;
- `review_refs`, with approval and waiver refs kept separate;
- `outcome`, `reproducibility_label`, and `publication_posture`;
- `governance_links` to benchmark run results, AI evidence packets, design
  research packets, public-proof packets, claim rows, known-limit notes, record
  classes, and support/export packets as applicable.

An experiment with `publication_posture = publishable_public_proof` must cite a
public-proof packet or benchmark-publication pack. An experiment that is
waived must keep waiver refs in `review_refs.waiver_refs`; waivers are never
hidden in notes.

## Dataset Summaries

Dataset summaries are metadata-first. They preserve comparability and privacy
without exposing raw rows or source names.

Required fields include:

- `source_profile` with a closed `source_class`, source snapshot ref, location
  class, provenance class, and source steward;
- `version_snapshot` with snapshot/version labels, schema refs, transform refs,
  and refresh date;
- `slice_profile` with scope class, estimated rows/files, size class,
  partition refs, sampling/truncation state, and slice-size notes;
- `sensitivity_profile` with sensitivity, redaction, retention, and export
  posture classes;
- `license_posture` with license class, review status, allowed use, and
  redistribution posture;
- `exclusions` and `comparability_notes` as typed rows, not prose footnotes;
- `linkage` back to corpus manifests, benchmark governance, AI eval packets,
  design-study packets, public-proof packets, and claim rows where relevant.

If a dataset changes snapshot, partition, sampling method, redaction posture,
or license posture, the change must be visible to comparison records through
typed `comparability_notes` and the comparison's dataset-delta axis.

## Result Comparisons

Result comparisons must separate metric change from context drift. A valid
comparison names:

- baseline and candidate experiment refs;
- metric schema/version and comparison purpose;
- comparison label (`comparable`, `data_skewed`, `hardware_skewed`,
  `environment_skewed`, `lineage_missing`, `not_comparable`, and related
  states);
- code, dataset, corpus, hardware, environment, build, and parameter deltas;
- metric rows with baseline, candidate, delta, threshold state, and confidence
  or significance refs;
- variance/significance notes;
- disqualifiers with severity and whether they must be visible in presentation;
- a presentation-safe summary view that declares which changed axes are shown.

A presentation view may be compact, but it may not hide material corpus,
dataset, hardware, environment, build, parameter, waiver, or disqualifier
changes. Public or public-proof candidate summaries must expose changed corpus
and hardware axes whenever the comparison label says they changed.

## Linkage Rules

This family is a common spine, not a replacement for downstream packets:

- Benchmark run results cite experiment records when a performance number needs
  hypothesis, owner, parameter, seed, approval, or dataset-summary context.
- Public-proof packets cite experiment records and result comparisons when a
  claim needs reproducibility and a presentation-safe summary.
- AI evaluation and graduation packets cite experiment records for protected
  eval, red-team, latency, cost, prompt-pack, or route-comparison evidence.
- Design research and usability-study packets cite experiment records for
  study hypothesis, source, participant/data summary, approval posture, and
  share scope.
- Claim-manifest rows cite experiment records, comparisons, or public-proof
  packets instead of embedding benchmark or design-study evidence directly.

If a downstream packet restates a value from this family, it must preserve the
same id and downgrade posture. The downstream packet may narrow a claim but may
not widen it beyond the experiment or comparison record.

## Export and Share Posture

Exports are explicit:

- `metadata_only` exports may include ids, classes, dates, counts, digest refs,
  metric summaries, redaction posture, and comparison labels.
- `summary_first` exports may include presentation-safe summaries and derived
  metric rows.
- Raw payload exports are outside this schema family and must cite a separate
  artifact manifest, redaction report, approval ticket, and destination/share
  class.

When lineage is incomplete, the family stays useful but downgrades: local
comparison can proceed under `context_incomplete`, `lineage_missing`, or
`not_comparable`; public-proof or claim-bearing surfaces must narrow, refresh,
or withdraw.

## Fixture Coverage

The seed fixtures cover:

- a benchmark-public-proof experiment that cites a protected corpus, exact
  build, reference hardware/profile, parameters, seed, and public-proof packet;
- a redacted customer-derived dataset summary with license, sensitivity,
  exclusion, and comparability rows;
- a comparison where data and hardware changed, forcing visible presentation
  axes and a narrowed claim;
- an AI evaluation run that cites protected-eval and graduation packet refs
  while carrying approval and waiver refs separately.

## Invariants

- Every claim-bearing experiment has a reproducible record with owner, source,
  data, build, hardware/profile, environment, parameters, seed, approvals, and
  waiver posture.
- Dataset summaries make snapshot, slice, sensitivity, redaction, license,
  refresh, exclusion, and comparability state visible without exposing raw
  data by default.
- Result comparisons keep changed corpora, datasets, hardware, environment,
  build, parameters, waivers, and disqualifiers out of free-form notes.
- Benchmark, AI, design-research, performance, and public-proof lanes share
  this contract family by reference instead of minting parallel provenance
  dialects.
