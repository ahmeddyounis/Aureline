# Publish launch-language conformance packs, support-class evidence, and downgrade rules for every stable lane — stable contract

Status: Stable lane proof for the Python, TypeScript/JavaScript, Rust,
Go, Java/Kotlin, and C/C++ launch-language lanes at the M4
launch-stable grade.

This document is the reviewer-facing contract for the stable
launch-language conformance pack publication truth packet. The packet
is the single source of truth that the editor language pack, framework
pack panel, language settings/help, CLI/headless inspector, support
export, release proof index, Help/About proof card, and the
conformance dashboard all read; surfaces MUST NOT mint local copies or
paraphrase conformance posture.

The packet publishes one conformance pack per certified launch-language
lane. A lane that claims `launch_stable` MUST cover the full daily loop
on a certified archetype repo, MUST bind at least one
`support_class_evidence_admission` row binding a non-`docs_disclosure_only`
support-class-evidence class, and MUST publish at least one
`downgrade_rule_admission` row binding a downgrade rule (`narrow_on_*`
or `block_on_*`) the conformance dashboard can apply when archetype,
fixture, or evidence drift is detected.

## What the packet asserts

For each governed *launch-language lane × row* the packet asserts:

1. The **launch-language lane class** — one of
   `python_launch_language_lane`,
   `typescript_javascript_launch_language_lane`,
   `rust_launch_language_lane`, `go_launch_language_lane`,
   `java_kotlin_launch_language_lane`, or `c_cpp_launch_language_lane`.
   Every certified packet MUST carry at least one row for each of the
   six required lanes.
2. The **conformance-pack row class** — one of
   `conformance_pack_quality`, `support_class_evidence_admission`,
   `downgrade_rule_admission`, `daily_loop_step`, `unsupported_gap`,
   `known_limit`, or `downgrade_automation`. A `daily_loop_step` row
   MUST bind a real daily-loop step; a `support_class_evidence_admission`
   row MUST bind a non-`not_applicable` support-class-evidence class; a
   `downgrade_rule_admission` row MUST bind a non-`not_applicable`
   downgrade-rule class; no other row class is permitted to bind one.
3. The **support class** — one of `launch_stable`,
   `launch_stable_below`, `beta_grade_only`, `preview_only`,
   `unsupported`, or `support_unbound`. The validator refuses to certify
   a row that claims `launch_stable` while any binding is unbound
   (support, known limit, downgrade automation, evidence, support-class
   evidence, or downgrade rule).
4. The **support-class-evidence class** — one of
   `archetype_repo_backed`, `fixture_corpus_backed`,
   `conformance_suite_backed`, `benchmark_backed`,
   `design_partner_backed`, `framework_migration_backed`,
   `docs_disclosure_only`, `not_applicable`, or `evidence_unbound`. A
   lane that claims `launch_stable` MUST bind at least one
   `support_class_evidence_admission` row backed by a non-`docs_disclosure_only`
   class so the packet can prove its claimed support class is rooted in
   current fixture, archetype, conformance, benchmark, design-partner,
   or framework-migration evidence.
5. The **downgrade-rule class** — one of `narrow_on_missing_fixture`,
   `narrow_on_missing_archetype`, `narrow_on_failed_migration`,
   `narrow_on_low_confidence`, `block_on_missing_evidence`,
   `block_on_unsupported_gap`, `manual_only_pending_review`,
   `not_applicable`, or `rule_unbound`. A lane that claims
   `launch_stable` MUST surface at least one
   `downgrade_rule_admission` row binding a real rule the conformance
   dashboard publishes alongside the pack so consumers know exactly how
   the lane will narrow when archetype, fixture, or evidence drift is
   detected.
6. The **daily-loop step class** — one of `open_or_import`, `navigate`,
   `edit`, `complete`, `refactor`, `run_test_debug`, `review`,
   `migrate`, `recover`, or `not_applicable`. A lane that claims
   `launch_stable` MUST cover every certified daily-loop step.
7. The **evidence class** — one of `archetype_repo_evidence`,
   `framework_migration_evidence`, `design_partner_evidence`,
   `fixture_repo_evidence`, `conformance_suite_evidence`,
   `benchmark_evidence`, `docs_disclosure_evidence`, or
   `evidence_unbound`. A row whose evidence class is `evidence_unbound`
   is refused.
8. The **known-limit class** — one of `none_declared`,
   `language_subset_only`, `framework_subset_only`,
   `archetype_subset_only`, `daily_loop_step_subset_only`,
   `migration_subset_only`, `unsupported_runtime_target`,
   `beta_capability_sample_only`, or `limit_unbound`. A row whose known
   limit is `limit_unbound` is refused.
9. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`,
   `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`,
   `auto_demote_on_low_confidence`, `auto_block_on_missing_evidence`,
   `auto_block_on_unsupported_gap`, `manual_only_pending_review`, or
   `automation_unbound`. A row whose automation is
   `automation_unbound` is refused.
10. The **conformance-pack confidence class** — `high_confidence`,
    `medium_confidence`, or `low_confidence`. A row that claims
    `launch_stable` at `low_confidence` is narrowed below stable until
    evidence grows.
11. The **evidence refs** — every row preserves at least one
    repo-relative evidence ref proving the conformance claim.
12. The **disclosure ref** — every row that is not `launch_stable`,
    that declares a non-`none_declared` known limit, or that binds a
    non-`none` downgrade automation MUST carry a repo-relative
    disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one of
those booleans to false. The packet never admits raw source bodies,
raw fixture payloads, raw archetype-repo bodies, package manifests,
build-system credentials, signing keys, ambient
`HOME`/`PATH`/`CARGO_HOME`/`GOPATH` values, or any other private
material past the boundary.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `launch_stable` while its support, support-class
  evidence, downgrade rule, evidence, known-limit, or downgrade
  automation class is unbound,
- a lane that claims `launch_stable` is missing a certified
  `daily_loop_step` row for any of the nine required steps
  (open/import, navigate, edit, complete, refactor, run/test/debug,
  review, migrate, recover),
- a lane that claims `launch_stable` has no
  `support_class_evidence_admission` row binding a non-`not_applicable`
  support-class-evidence class,
- a lane that claims `launch_stable` has no
  `downgrade_rule_admission` row binding a non-`not_applicable`
  downgrade-rule class,
- a `daily_loop_step`, `support_class_evidence_admission`, or
  `downgrade_rule_admission` row drops its bound class,
- a non-`daily_loop_step` row binds a daily-loop step, a
  non-`support_class_evidence_admission` row binds a
  support-class-evidence class, or a non-`downgrade_rule_admission` row
  binds a downgrade rule,
- a row narrowed below `launch_stable` drops its disclosure ref,
- a row declares a non-`none_declared` known limit and drops its
  disclosure ref,
- a row binds a non-`none` downgrade automation and drops its
  disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies (lane, row class, support
  class, support-class evidence, downgrade rule, daily-loop step, known
  limit, downgrade automation, or evidence class),
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## Lane scope

Each launch-language lane is certified at the launch-stable wedge with
the following understood scope:

- **`python_launch_language_lane`** — conformance pack for the Python
  daily-driver lane (see
  `stabilize-python-daily-driver-quality-with-interpreter-venv.md`)
  including interpreter/venv resolution, navigation, refactor,
  run/test/debug, and migration coverage on certified Python archetype
  repos.
- **`typescript_javascript_launch_language_lane`** — conformance pack
  for the replacement-grade TypeScript/JavaScript/HTML/CSS lane (see
  `stabilize-typescript-javascript-html-and-css-replacement-grade.md`)
  including ECMAScript/TypeScript navigation, refactor, run/test/debug,
  and migration coverage on certified TS/JS archetype repos.
- **`rust_launch_language_lane`** — conformance pack for the Rust
  daily-driver lane (see
  `stabilize-rust-daily-driver-quality-with-cargo-workspaces.md`)
  including Cargo workspace navigation, refactor, run/test/debug, and
  migration coverage on certified Rust archetype repos.
- **`go_launch_language_lane`** — conformance pack for the Go
  daily-driver lane (see
  `stabilize-go-daily-driver-quality-with-modules-workspaces.md`)
  including module/workspace navigation, refactor, run/test/debug, and
  migration coverage on certified Go archetype repos.
- **`java_kotlin_launch_language_lane`** — conformance pack for the
  Java/Kotlin daily-driver lane (see
  `stabilize-java-and-kotlin-daily-driver-quality-with.md`) including
  Gradle/Maven navigation, refactor, run/test/debug, and migration
  coverage on certified Java/Kotlin archetype repos.
- **`c_cpp_launch_language_lane`** — conformance pack for the C/C++
  daily-driver lane (see `stabilize-c-and-cpp-daily-driver-quality-with.md`)
  including CMake/Bazel navigation, refactor, run/test/debug, and
  migration coverage on certified C/C++ archetype repos.

## Consumer projections

The packet declares eight required consumer projections — one per
surface in `ConsumerSurface::REQUIRED`. Each projection preserves the
packet id, the closed vocabularies (lane, row class, support class,
support-class evidence, downgrade rule, daily-loop step, known limit,
downgrade automation, evidence class), supports JSON export, and
excludes raw private material and ambient authority. The validator
emits `missing_consumer_projection`, `consumer_projection_drift`,
`lane_vocabulary_collapsed`, `row_class_vocabulary_collapsed`,
`support_class_vocabulary_collapsed`,
`support_class_evidence_vocabulary_collapsed`,
`downgrade_rule_vocabulary_collapsed`,
`daily_loop_step_vocabulary_collapsed`,
`known_limit_vocabulary_collapsed`,
`downgrade_automation_vocabulary_collapsed`, or
`evidence_class_vocabulary_collapsed` as blockers if a surface drops or
remints the packet.

## Reviewer checklist

- The checked-in stable packet at
  `artifacts/language/m4/publish_launch_language_conformance_packs_truth_packet.json`
  parses, validates, and covers all six required lanes plus all eight
  required consumer projections.
- The Rust contract at
  `crates/aureline-language/src/publish_launch_language_conformance_packs_truth_packet/`
  materializes a stable baseline, narrows the five recorded
  non-baseline postures, and refuses raw source material, secrets, or
  ambient authority past the boundary.
- The fixture corpus at
  `fixtures/language/m4/publish_launch_language_conformance_packs_truth_packet/`
  pins the baseline and the five narrowed-below-stable postures and is
  exercised by the integration tests at
  `crates/aureline-language/tests/publish_launch_language_conformance_packs_truth_packet.rs`.
