# Benchmark fixture classes and corpus vocabulary

This document is the **normative** companion to the protected benchmark
corpus manifest at
[`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml).
It defines the closed vocabularies the manifest, the benchmark lab,
the journey harness, the boundary-truth validators, the compatibility
scoreboards, and the support-export lanes resolve against when they
name a corpus entry.

If this document disagrees with the manifest, this document wins and
the manifest must be updated in the same change. Renaming any token
defined here is **breaking** and opens a decision row; adding a value
is additive-minor and lands in this document and the manifest in the
same change.

Companion artifacts:

- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — the machine-readable register.
- [`/fixtures/benchmarks/README.md`](../../fixtures/benchmarks/README.md)
  — the index that pairs with this document.
- [`/fixtures/workspaces/reference/`](../../fixtures/workspaces/reference/)
  — reference-workspace seeds referenced by corpus IDs.
- [`/docs/benchmarks/spike_metric_names.md`](./spike_metric_names.md)
  — the protected-path hook vocabulary the corpus manifest's
  `protected_journeys` values align with.
- [`/docs/governance/benchmark_council_charter.md`](../governance/benchmark_council_charter.md)
  — the council that owns corpus changes, hardware baselines,
  threshold changes, and dispute resolution.
- [`/docs/benchmarks/corpus_governance.md`](./corpus_governance.md)
  — change-control policy for protected benchmark corpora and
  protected-metric changes.
- [`/artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml)
  — the task-success corpus seed whose `fixture.archetype_*`
  reservations are materialised by the reference-workspace seeds
  and cross-referenced from the corpus manifest.

## 1. Why this vocabulary is frozen now

1. Benchmark regressions have to be comparable across runs. If two
   runs measure against "a TS web app" but one was an archetype seed
   and the other was a large-file fixture whose size class happened
   to be the same, the comparison is meaningless. The manifest
   freezes a stable id so the report can say "which corpus revision
   you were measuring" with no ambiguity.
2. The account-free local-first path is a launch-bearing claim. The
   no-account switching scoreboard cites the corpus manifest by id.
   Ids that drift, get renamed, or acquire new meaning silently
   break the scoreboard.
3. The benchmark council's decision scope (charter §3) includes
   *corpus changes* and *hardware-baseline changes*. The council
   needs a vocabulary it can adjudicate against; this document
   makes the vocabulary auditable.
4. Later milestones will add archetype-specific corpora, a
   certified-workspace fixture register, and a hardware-baseline
   register. Each of those extends this vocabulary; reserving the
   shape here prevents a parallel corpus dialect from being minted.

## 2. Scope

### In scope

- Closed vocabularies for: corpus class, protected journey, size
  class, visibility class, retention class, license / redaction
  status, host-platform class, toolchain assumption, archetype
  placeholder tag, support class, evidence-consumer channel.
- Metadata every corpus fixture MUST carry (stable id, revision /
  source lineage, toolchain assumption, platform class, size class,
  protected journeys exercised, support classes exercised, archetype
  tags, evidence-consumer channels, redaction / license status,
  retention class, visibility class, degraded / unsupported notes).
- Rules for `resolution_mode`: when a fixture is a concrete file or
  directory, when it is a live-repo slice, when it is a recipe-only
  entry.
- Segregation-marker rules for fixtures that would need extra
  privacy, export, or retention review before broader CI use.

### Out of scope

- Benchmark hardware baselines (owned by the benchmark council
  charter; named as a decision slot but not enumerated here).
- Statistical confidence and regression thresholds for any individual
  metric.
- Telemetry collection, event naming, and wire formats (owned by the
  onboarding measurement plan and ADR-0005).
- Publication rules for benchmark numbers (owned by
  [`public_comparison_rules.md`](./public_comparison_rules.md); out of
  scope here).

## 3. Corpus classes (frozen)

Every fixture in the corpus manifest carries exactly one
`corpus_class` from the closed set below. Adding a class is
additive-minor; repurposing is breaking.

| `corpus_class`                 | What it is                                                                                                          | Canonical consumer                                                    |
|--------------------------------|---------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------|
| `microbenchmark_scenario`      | A narrow case exercising one or a small set of protected-path hooks, typically with deterministic synthetic input   | Benchmark lab (text stack, shaping, save pipeline)                    |
| `workflow_scenario`            | An end-to-end user-level flow composed of harness steps and reference-workspace inputs                              | Benchmark lab (Bootstrap / entry-parity), UX evidence                 |
| `archetype_seed`               | A minimal workspace shape that the archetype detector MUST classify to a named support class                        | Benchmark lab (certified-archetype workflows), compatibility report   |
| `large_file_trigger`           | A fixture that exercises exactly one trigger of the ADR-0003 large-file-mode evaluator                              | Benchmark lab (large file)                                            |
| `recovery_or_restore_scenario` | A fixture that exercises entry-restore / recovery-journal / session-checkpoint restore                              | Benchmark lab (recovery ladder), support bundle (crash recovery)      |
| `reference_workspace`          | A stable workspace shape used by multiple scenarios; names *what was opened*, not *what happened*                   | Referenced by other corpus entries via their inputs list              |
| `boundary_truth_case`          | A fixture that validates a frozen boundary schema (save-target token, mutation journal, entry / restore result)     | Boundary-truth validators; support-export families                    |

## 4. Protected journeys (aligned, not invented)

The manifest's `protected_journeys` vocabulary is the union of:

1. the ADR-0002 protected-path bucket set documented in
   [`spike_metric_names.md`](./spike_metric_names.md)
   (`startup`, `first_useful_chrome`, `first_paint`, `input_to_paint`,
   `render_submission`, `frame_budget`, `placeholder_open`,
   `placeholder_edit`, `placeholder_save`, `fallback_resolution`,
   `observability`);
2. the four corpus-specific journey tokens the benchmark lab needs
   to attach fixtures to lanes that the spike vocabulary does not
   cover: `vfs_root_enumeration`, `save_pipeline`,
   `recovery_journal_write`, `recovery_journal_restore`;
3. one boundary-truth journey for fixtures that exercise a frozen
   boundary schema and no protected hot path:
   `boundary_truth_contract`.

A fixture MUST NOT invent a journey token outside this set. Reserved
buckets (`first_useful_chrome`, `frame_budget`) are admissible on
fixture entries today but the coverage map records zero scenarios
until a corpus lands against them; this is intentional and matches
the spike-metric-names doc's "reserved slot, admissible in schema,
zero firings today" pattern.

## 5. Size classes (frozen)

| Class       | Range                     | Notes                                                                                                |
|-------------|---------------------------|------------------------------------------------------------------------------------------------------|
| `micro`     | `< 1 KiB`                 | Smallest tier. Hand-authorable; ideal for boundary cases and reference workspaces that are "shape only". |
| `tiny`      | `1 KiB – 10 KiB`          | Typical envelope for per-trigger large-file fixtures and archetype seeds.                            |
| `small`     | `10 KiB – 100 KiB`        | Live-repo slices and microbenchmark corpora that need some span.                                      |
| `medium`    | `100 KiB – 1 MiB`         | Not used at this manifest revision; reserved for workflow scenarios that need a realistic span.       |
| `large`     | `1 MiB – 10 MiB`          | Not used at this manifest revision; reserved.                                                         |
| `oversize`  | `> 10 MiB`                | **Never checked in as bytes.** Fixture declares `resolution_mode: recipe_only` and names a generator; size is the expected regeneration size. |

A fixture above `medium` MUST have a recipe; no fixture above `small`
is checked in today.

## 6. Visibility, retention, license

### Visibility

- `public` — fixture ships with the repository and is admissible in
  public CI.
- `internal` — fixture is a repository-internal control record; the
  repository may still be public, but the fixture is not a published
  surface.
- `restricted` — fixture requires extra privacy, export, or retention
  review before broader CI use. **No fixture in this repository is
  `restricted`**; the token is reserved so later corpora that need
  segregated storage have a defined class.

### Retention

- `permanent_seed` — kept across milestones; retiring requires a
  decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- `milestone_scoped` — may be retired between milestones with an
  audit entry; not used at this manifest revision.
- `derived_regenerable` — the fixture is defined by a recipe; the
  bytes are regenerated and retirement is bookkeeping only.

### License / redaction status

- `original_author_aureline_project_mit_0bsd_choice` — contributed
  under the project license per
  [`/CONTRIBUTING.md`](../../CONTRIBUTING.md).
- `synthetic_no_real_content` — synthetic bytes; no real code,
  customer data, or third-party content.
- `vendored_with_attribution_pending_notice` — vendored third-party
  content that requires an import-register row and a release-notice
  seed entry; the
  [`third_party_import_register.yaml`](../../artifacts/governance/third_party_import_register.yaml)
  and
  [`release_notice_seed.yaml`](../../artifacts/governance/release_notice_seed.yaml)
  gain rows in the same change.
- `requires_extra_privacy_review_before_ci` — the fixture carries
  privacy, export, or retention concerns beyond the public /
  internal classes; the fixture MUST also be marked with a
  `segregation_markers` entry on the manifest and MUST NOT ship in
  this repository unless a segregation bundle has been approved.

## 7. Host-platform and toolchain posture

### Host-platform class

- `host_independent` — byte-stable across host OS and arch.
- `host_dependent` — bytes or measurement inputs differ across hosts;
  the benchmark report MUST record `host_os` and
  `rustc_target_triple` in the run metadata.
- `host_os_pinned` — fixture is pinned to one host OS and declares
  it. Not used at this manifest revision; reserved.

### Toolchain assumption

- `none` — no toolchain dependency.
- `synthetic_bytes_no_toolchain` — generated bytes with no compiler
  or interpreter dependency.
- `rust_toolchain_pinned_v1` — requires the repo-pinned toolchain
  from
  [`/rust-toolchain.toml`](../../rust-toolchain.toml).
- `node_js_declared_only` — the fixture's package manifest declares
  an engine; no `node_modules` install is required by the fixture
  itself. Benchmarks that require an installed tree populate it
  out of band.
- `python_interpreter_declared_only` — the fixture's package
  manifest declares an interpreter; no virtual environment is
  required by the fixture itself.
- `jvm_toolchain_declared_only` — the fixture's JDK/build manifest
  declares exact versions; no wrapper, dependency cache, or build
  output is required by the fixture itself.
- `go_toolchain_declared_only` — the fixture's module or workspace
  manifest declares the Go version; no module cache is required by the
  fixture itself.
- `c_cpp_toolchain_declared_only` — the fixture's CMake or preset
  manifest declares the native tools; generated host paths and build
  output are populated out of band.

## 8. Archetype placeholder tags and support classes

### Archetype tags

Reserved archetype identities so later certified-workspace and
compatibility scoreboards compose against one vocabulary:

- `ts_web_app`
- `python_data_app`
- `java_or_kotlin_service`
- `rust_workspace_self_host`
- `go_service_or_monorepo_slice`
- `c_or_cpp_native_project`
- `misc_folder`
- `not_archetype_bound` — the fixture is not archetype-sensitive
  (microbenchmarks, boundary cases).

Adding an archetype tag is additive-minor. Renaming is breaking.

### Support classes

The closed set aligns with the task-success corpus's
`archetype_detection_outcome` vocabulary:

- `certified_archetype_match`
- `supported_archetype_match`
- `community_archetype_match`
- `experimental_archetype_match`
- `probable_archetype_partial`
- `unrecognised_archetype`
- `archetype_detection_unavailable`
- `not_support_bound` — the fixture does not qualify an archetype
  support tier.

## 9. Evidence-consumer channels

Every fixture MUST name at least one channel. A fixture that no
channel reads is non-conforming. The closed set is defined in the
manifest's `evidence_consumer_channels` block; at this manifest
revision it covers the benchmark lab lanes (text stack, large file,
Bootstrap / entry parity, migration, certified archetype, VFS and
save pipeline, recovery ladder), the UX-evidence families (first-run
audit, migration dry-run diff), the support-bundle summaries (first
run, performance, crash recovery, recent work, managed workspace,
semantic-readiness packet), release-evidence claim manifests, and
the boundary-truth validators (save-target token, semantic readiness,
mutation journal, entry and restore result, generated artifact
lineage).

## 10. `resolution_mode` rules

A fixture entry resolves one of three ways:

1. **`concrete_file`** (default when `resolution_mode` is omitted).
   `path` points at a repo-relative file or directory. The bytes at
   that path are the authoritative identity of the fixture.
2. **`live_repo_slice`**. `slice.include_globs` and
   `slice.exclude_globs` are resolved at run time against the live
   repository. The benchmark report MUST record the repository
   commit and the resolved file list alongside metrics so the
   "which bytes were measured" question is answerable after the
   fact.
3. **`recipe_only`**. A named `recipe` block generates the bytes at
   run time. This is the only admissible mode for `size_class:
   oversize` fixtures and for fixtures whose identity is a process
   (seeded pseudo-random ASCII, structured synthetic JSON, and so
   on). The recipe name is stable; the bytes it emits are the same
   at every invocation.

A fixture MUST NOT mix modes. Switching a fixture from one mode to
another is **breaking** (the identity changes) even if the id stays
the same; open a decision row.

## 11. Segregation markers

A fixture that would require extra privacy, export, or retention
review before broader CI use is segregated on two axes:

1. it carries `license_status:
   requires_extra_privacy_review_before_ci` **or**
   `visibility_class: restricted`; and
2. it appears in the manifest's top-level `segregation_markers` block
   with a typed reason from the closed set:

   - `contains_real_user_code`
   - `contains_real_customer_data`
   - `bound_to_specific_license_restrictions`
   - `subject_to_export_control_review`
   - `requires_retention_schedule_review`

At this manifest revision no fixture in this repository is
segregated; the block exists so later corpora that need it have a
defined home.

## 12. Change policy

- **Additive-minor** — new fixture id, new corpus class value, new
  archetype tag, new support class, new protected-journey token, new
  evidence-consumer channel, new recipe kind — lands in this
  document and the manifest in the same change. The change cites the
  scenario or evidence family that motivates it.
- **Breaking** — repurposing an id, renaming a vocabulary token,
  switching a fixture between `resolution_mode` values, or removing a
  `permanent_seed` fixture — opens a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- **Benchmark council authority** — corpus changes that are material
  (moving between corpus classes; changing a `protected_journeys`
  list in a way that retargets a fixture across a lane boundary;
  promoting a fixture to `certified_archetype_match`) require a
  benchmark-council decision per the charter §3 and the benchmark
  change-control policy in
  [`corpus_governance.md`](./corpus_governance.md). Additive scenario
  rows that do not cross those lines follow the additive-minor path.

## 13. What this document is not

- It is **not** a benchmark-report specification. The report shape
  is owned by the benchmark council charter and the
  `governance_packet_template.yaml` template.
- It is **not** a hardware-baseline register. Hardware baselines are
  named as a decision slot in the benchmark council charter §3;
  enumerating them is deferred.
- It is **not** a certification-grade archetype catalog. Archetype
  *tags* are reserved here; the certified-archetype catalogue and
  its compatibility promises land in a later milestone under the
  `compatibility_ecosystem_review` lane.
