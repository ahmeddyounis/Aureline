# Benchmark-publication pack template

<!--
Copy this template when publishing benchmark evidence outside the raw
dashboard or nightly-lab context.

Related control artifacts:
- docs/benchmarks/benchmark_lab_run_results.md
- artifacts/perf/reference_hardware_manifest.yaml
- artifacts/perf/lab_image_manifest.yaml
- docs/perf/self_capture_parity.md
- artifacts/bench/protected_metrics.yaml
- artifacts/bench/corpus_change_control.yaml
- docs/benchmarks/public_comparison_rules.md
- artifacts/bench/fitness_function_catalog.yaml
- fixtures/benchmarks/corpus_manifest.yaml
- artifacts/evidence/evidence_metadata_fields.yaml
- artifacts/governance/evidence_id_conventions.md
- docs/governance/evidence_freshness_policy.md
- artifacts/governance/evidence_freshness_slos.yaml
- artifacts/governance/evidence_rerun_triggers.yaml
- schemas/governance/evidence_packet_header.schema.json
- docs/build/exact_build_identity_model.md
- docs/docs/docs_pack_manifest_contract.md
- docs/release/release_artifact_graph.md

This template freezes what a public benchmark packet must carry so
methodology, caveats, docs applicability, and exclusions are recorded in
one packet instead of scattered across dashboards, slides, and memory.
-->

## Shared packet header

Every benchmark-publication pack SHOULD embed a header that conforms to
`schemas/governance/evidence_packet_header.schema.json` and SHOULD
follow `artifacts/governance/evidence_id_conventions.md` when it links
design captures, benchmark runs, verification packets, support drills,
known-limit notes, or migration packets.

- **Packet id:** `<benchmark-publication-pack-id>`
- **Packet state:** `draft` | `in_review` | `publishable` | `quarantined` | `superseded` | `withdrawn`
- **Publication posture:** `methodology_only` | `aureline_only_claim` | `public_head_to_head_comparison` | `quarantined_not_comparable`
- **Claim summary:** one sentence naming the claim or the methodology snapshot
- **Published on:** `YYYY-MM-DD`
- **Measured on:** `YYYY-MM-DDTHH:MM:SSZ`
- **Owner:** `@handle`
- **Evidence owner:** `@handle`
- **Review forums:** `performance_council`, `release_council`, `<other forum if required>`
- **Primary run refs:** list of `run_id`, `dashboard_ref`, or both
- **Primary exact-build identity refs:** list of `exact_build_identity_ref`
- **Release channel / workspace version:** `<channel>` / `<version>`
- **Protected metrics revision:** `<metrics-file-id>@<revision>`
- **Fitness-function catalog revision:** `<catalog-id>@<revision>`
- **Corpus manifest revision:** `<manifest-id>@<revision>`
- **Active waiver packet refs:** waiver packet ids or `none`
- **Active advisory refs:** advisory ids or `none`

## Public statement

Two or three sentences that say what the packet is claiming, whether it
is methodology-only or claim-bearing, and what caveat a reader must know
before reading numbers.

## Build and docs/help applicability

- **Exact-build identity set:** list the coordinated build identities
  this packet is about.
- **Docs/help pack revision:** `<pack_revision_ref>` or `not_applicable`
- **Docs/help version-match state:** `exact_build_match` | `compatible_minor_drift` | `incompatible_drift_detected` | `pre_release_unverified` | `unknown_target_build`
- **Docs/help caveat or repair hook:** `<one sentence>` or `none`
- **Source anchor refs:**
  - `<path>#<anchor>` — short note on why this anchor governs the claim.
  - ...

## Invocation and config

- **Exact command line:** quote the complete command line used to emit
  the run.
- **Config refs or digests:** list config files, digests, feature-flag
  settings, and non-default knobs that materially affect the result.
- **Environment variables or launch overrides:** list only the values
  required to reproduce the result; if none, say `none`.
- **Install-topology or publication posture refs:** install-profile
  card refs, portable/offline/mirror posture, or `not_applicable`.

```text
<exact command line here>
```

## Environment notes

- **Hardware definition ref:** `<hardware-definition-ref>`
- **Benchmark environment ref:** `<environment-definition-ref>`
- **Display class id:** `<display-class-id>`
- **Lab image revision:** `<lab-image-id>@<revision>`
- **Host OS / architecture:** `<os>` / `<arch>`
- **Power / thermal posture:** `<battery/ac, governor, thermal note>`
- **Display / font / locale note:** `<display scale, fonts, locale, input method>`
- **Network or remote posture:** `<local-only, remote, offline, air-gapped, latency profile>`
- **Lab-specific note:** one sentence on anything else the reader must
  know for comparability.

## Corpus and task definition

- **Protected journeys or fitness rows cited:** list `ff.*` ids.
- **Corpus refs:** list `corpus.*` ids or repo slices.
- **Task script or success criterion:** quote the task the benchmark
  measures. If it is not a script, say exactly what counted as success.
- **Exclusions declared up front:** list any intentionally omitted
  scenario, platform, plugin, locale, or workload.

## Comparability and quarantine

- **Run context class:** `reference_capture` | `provisional_capture` | `self_capture` | `smoke_subset`
- **Comparability class:** `comparable_to_baseline` | `comparable_to_prior_run_same_host` | `not_yet_comparable` | `quarantined`
- **Quarantine reasons:** list the typed reasons or `none`
- **Comparable baseline ref:** `<run_id or baseline ref>` or `none`
- **Comparability note:** one sentence that says what the reader may and
  may not compare this packet against.
- **Drift fields, if any:** `lab image` | `display class` | `power policy` | `thermal posture` | `calibration rules` | `none`
- **Comparison-methodology change ref:** `<change_id from corpus_change_control.yaml>` or `none`

## Results cited

| `evidence_id` | Row or task ref | Result | Captured at | `stale_after` | Source |
|---|---|---|---|---|---|
| `<evidence-id>` | `<ff.* or task id>` | `pass` | `YYYY-MM-DDTHH:MM:SSZ` | `<duration or null>` | `<repo-relative path or stable ref>` |
| ... | ... | ... | ... | ... | ... |

## Competitor settings

If `Publication posture` is not `public_head_to_head_comparison`, write
`not_applicable` and leave no ambiguity.

- **Competitor product / version:** `<product>` / `<version>` or `not_applicable`
- **Plugin / extension / feature set:** `<short list>` or `not_applicable`
- **Install / launch posture:** `<portable, installed, plugin-disabled, etc.>` or `not_applicable`
- **Task parity note:** one sentence stating how task parity was
  enforced or why the comparison is intentionally narrow.

## Known limits and exclusions

- Every caveat that would otherwise end up in a slide footnote or oral
  explanation belongs here.
- Name whether the packet is benchmark-lab truth, release-candidate
  truth, or methodology-only truth.
- If a waiver, stale corpus revision, docs drift, or missing platform
  blocks a wider claim, say so directly.

## Publication envelope

These items MUST be public in the packet:

- exact command line,
- config refs/digests,
- exact-build identity refs,
- channel/version context,
- protected-metrics revision,
- run-context and comparability/quarantine posture,
- corpus revision and task definition,
- docs/help version-match state,
- known limits and exclusions, and
- competitor settings when a public comparison is made.

These items MAY remain internal, but only by stable ref or explicit
omission note:

- raw trace bundles and raw log bodies,
- restricted fixture bytes or private partner repo names,
- machine serial numbers and other host-only identifiers,
- competitor license keys or paid configuration material, and
- internal review discussion.

## Evidence index

Every evidence item listed here should use the shared field names from
`artifacts/evidence/evidence_metadata_fields.yaml`.

- **`evidence_id:`** `<evidence-id>`
  - **Artifact family:** `benchmark_run_result` | `benchmark_dashboard` | `fitness_catalog_snapshot` | `<other catalog family>`
  - **Packet id:** `<benchmark-publication-pack-id>`
  - **Evidence ref:** `<repo-relative path or stable ref>`
  - **Captured at:** `YYYY-MM-DDTHH:MM:SSZ`
  - **Stale after:** `<ISO-8601 duration or null>`
  - **Source revision:** `<commit, schema revision, or manifest revision>`
  - **Trigger revision:** `<catalog/schema/anchor revision or null>`
  - **Channel context:** `<channel>`
  - **Deployment context:** `<profile ids>`
  - **Comparability note:** `<one sentence>`
  - **Exact-build identity ref:** `<exact_build_identity_ref or null>`
  - **Source anchor refs:** `<anchor refs or none>`
  - **Qualification row refs:** `<compat-row ids or none>`
  - **Continuity drill refs:** `<drill ids or none>`
  - **Waiver packet refs:** `<waiver ids or none>`

## Signoff and refresh trigger

- **Decision:** `publish packet` | `keep methodology_only` | `quarantine packet` | `withdraw packet`
- **Named refresh trigger:** one sentence saying what new run, corpus
  revision, docs-pack revision, or comparability reset invalidates this
  packet. Prefer a trigger id from
  `artifacts/governance/evidence_rerun_triggers.yaml` and keep
  `stale_after` within the ceiling from
  `artifacts/governance/evidence_freshness_slos.yaml`.
