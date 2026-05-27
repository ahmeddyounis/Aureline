# Publish launch-language conformance packs, support-class evidence, and downgrade rules for every stable lane — reviewer artifact

This is the human-readable reviewer artifact for the M4 stable
launch-language conformance pack publication truth packet. The
machine-readable contract, checked-in packet, schema, and fixture
corpus are:

- Rust contract: `crates/aureline-language/src/publish_launch_language_conformance_packs_truth_packet/`
- Stable packet: `artifacts/language/m4/publish_launch_language_conformance_packs_truth_packet.json`
- Boundary schema: `schemas/language/publish_launch_language_conformance_packs_truth.schema.json`
- Reviewer doc: `docs/languages/m4/publish-launch-language-conformance-packs-support-class-evidence.md`
- Fixture corpus: `fixtures/language/m4/publish_launch_language_conformance_packs_truth_packet/`

## Lane coverage

The stable packet certifies the following launch-language lanes at the
M4 launch-stable grade:

- `python_launch_language_lane` — Python conformance pack with
  interpreter/venv-aware daily-loop coverage, support-class evidence
  rooted in archetype repos, and downgrade rules covering missing
  fixtures, archetype drift, and unsupported runtime targets.
- `typescript_javascript_launch_language_lane` — TypeScript/JavaScript
  conformance pack with full daily-loop coverage across replacement-grade
  archetype repos, support-class evidence backed by the fixture corpus,
  and downgrade rules covering missing fixtures and dialect drift.
- `rust_launch_language_lane` — Rust conformance pack with Cargo
  workspace daily-loop coverage, support-class evidence backed by
  conformance suite runs, and downgrade rules covering missing fixtures
  and toolchain regressions.
- `go_launch_language_lane` — Go conformance pack with module/workspace
  daily-loop coverage, support-class evidence backed by archetype
  captures, and downgrade rules covering missing fixtures and module
  resolution drift.
- `java_kotlin_launch_language_lane` — Java/Kotlin conformance pack
  with Gradle/Maven daily-loop coverage, support-class evidence backed
  by design-partner captures, and downgrade rules covering missing
  fixtures and build-system drift.
- `c_cpp_launch_language_lane` — C/C++ conformance pack with
  CMake/Bazel daily-loop coverage, support-class evidence backed by
  benchmark captures, and downgrade rules covering missing fixtures and
  cross-compile drift.

## What stable certification means

For each lane the packet asserts the full nine-step daily loop is
covered (open/import, navigate, edit, complete, refactor,
run/test/debug, review, migrate, recover) with bound support class,
support-class-evidence class, downgrade-rule class, evidence class,
known-limit class, downgrade automation, confidence class, evidence
refs, and (where narrowed) a disclosure ref. Every lane also publishes
at least one `support_class_evidence_admission` row proving the support
claim is backed by non-`docs_disclosure_only` evidence and at least one
`downgrade_rule_admission` row binding a real `narrow_on_*` /
`block_on_*` / `manual_only_pending_review` rule consumers can act on
when archetype, fixture, or evidence drift is detected. Every row
excludes raw source bodies, secrets, and ambient authority. Every
required consumer projection (`editor_language_pack`,
`framework_pack_panel`, `language_settings`, `cli_headless`,
`support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) preserves the packet verbatim.

## How to reproduce

Run the unit tests and integration tests:

```
cargo test -p aureline-language publish_launch_language_conformance_packs
```

The integration tests in
`crates/aureline-language/tests/publish_launch_language_conformance_packs_truth_packet.rs`
load every fixture in
`fixtures/language/m4/publish_launch_language_conformance_packs_truth_packet/`
and assert that
`PublishLaunchLanguageConformancePacksTruthPacket::materialize` agrees
with the expected promotion state, finding kinds, and
closed-vocabulary token sets. The integration tests also load the
checked-in stable packet and require that it validates cleanly, covers
every required launch-language lane, and preserves every required
consumer projection.

## Narrowed fixtures

The fixture corpus pins one baseline stable posture plus five
narrowed-below-stable postures:

- `launch_stable_with_unbound_evidence_blocks_stable.json` — a
  `launch_stable` row that lost its evidence binding.
- `missing_daily_loop_step_for_launch_stable_blocks_stable.json` — a
  `launch_stable` lane that lost a `daily_loop_step` row.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — a row
  narrowed below `launch_stable` that lost its disclosure ref.
- `projection_collapses_support_class_evidence_vocabulary_blocks_stable.json`
  — a consumer projection that collapses the support-class-evidence
  vocabulary.
- `raw_source_material_blocks_stable.json` — a row that flips
  `raw_source_material_excluded` to false.

Each fixture asserts the expected validator finding kind so the
reviewer can confirm the validator narrows the right axis.
