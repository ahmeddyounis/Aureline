# TypeScript, JavaScript, HTML, and CSS replacement-grade daily-driver quality — reviewer artifact

This is the reviewer-facing artifact for the M4 stable daily-driver
quality truth packet covering TypeScript, JavaScript, HTML, and CSS
launch-language lanes with replacement-grade support, daily-loop
coverage, known limits, downgrade automation, and evidence binding.
The contract lives at
`docs/languages/m4/stabilize-typescript-javascript-html-and-css-replacement-grade.md`
and is replayed by
`crates/aureline-language/tests/daily_driver_quality_truth_packet.rs`.

## Stable claim

For every governed language lane class (`typescript_daily_driver_lane`,
`javascript_daily_driver_lane`, `html_daily_driver_lane`,
`css_daily_driver_lane`) the packet binds:

- at least one `daily_driver_quality` row per lane (the lane's
  headline daily-driver qualification),
- a `daily_loop_step` row per certified step (open/import, navigate,
  edit, complete, refactor, run/test/debug, review, migrate, recover)
  for every lane that claims `replacement_grade`,
- a closed `support_class` (no surface pretends `replacement_grade`
  while a binding is unbound),
- a closed `daily_loop_step_class` (every replacement-grade lane
  covers the full daily loop; non-loop rows bind `not_applicable`),
- a closed `evidence_class` (archetype-repo, framework-migration,
  design-partner, fixture-repo, conformance-suite, benchmark, or
  docs-disclosure),
- a closed `known_limit_class` (framework subset, language subset,
  archetype subset, migration subset, unsupported runtime target,
  beta capability sample, or `none_declared`),
- a closed `downgrade_automation_class` (auto-narrow on missing
  fixture/archetype, auto-narrow on failed migration / framework gap,
  auto-demote on low confidence, auto-block on missing evidence,
  manual-only, or `none`),
- a closed `daily_driver_confidence_class`, and
- at least one `evidence_refs` entry plus a `disclosure_ref` whenever
  the row is not `replacement_grade`, declares a non-`none_declared`
  known limit, or binds a non-`none` downgrade automation.

## Companion artifacts

- Schema: `schemas/language/daily_driver_quality_truth.schema.json`
- Checked-in packet:
  `artifacts/language/m4/daily_driver_quality_truth_packet.json`
- Fixture corpus:
  `fixtures/language/m4/daily_driver_quality_truth_packet/`
- Rust contract:
  `crates/aureline-language/src/daily_driver_quality_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-language/tests/daily_driver_quality_truth_packet.rs`
- Reviewer doc:
  `docs/languages/m4/stabilize-typescript-javascript-html-and-css-replacement-grade.md`

## Required consumer projections

The packet is preserved verbatim across eight consumer projections:

| Projection              | Surface                                          |
| ----------------------- | ------------------------------------------------ |
| `editor_language_pack`  | Editor language-pack badge/hover                 |
| `framework_pack_panel`  | Framework pack panel                             |
| `language_settings`     | Language settings / help surface                 |
| `cli_headless`          | CLI/headless inspector                           |
| `support_export`        | Support export bundle                            |
| `release_proof_index`   | Release proof index entry                        |
| `help_about`            | Help/About proof card                            |
| `conformance_dashboard` | Conformance dashboard row                        |

A projection that collapses any closed vocabulary, drops the packet id,
drops the row class, support class, daily-loop step, known limit,
downgrade-automation, or evidence-class vocabulary, or leaks raw
private material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `replacement_grade` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a lane that claims `replacement_grade` daily-driver quality is
  missing a certified `daily_loop_step` row for any of the nine
  required steps,
- a `daily_loop_step` row drops its daily-loop step binding,
- a non-`daily_loop_step` row binds a daily-loop step it cannot
  certify,
- a row narrowed below replacement grade or with a non-default known
  limit / non-`none` downgrade automation drops its disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies,
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`DailyDriverQualityTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only and
suitable for inclusion in any support export or release proof bundle.

## Where the packet lives

- Schema: [`schemas/language/daily_driver_quality_truth.schema.json`](../../../schemas/language/daily_driver_quality_truth.schema.json)
- Reviewer doc: [`docs/languages/m4/stabilize-typescript-javascript-html-and-css-replacement-grade.md`](../../../docs/languages/m4/stabilize-typescript-javascript-html-and-css-replacement-grade.md)
- Fixture corpus: [`fixtures/language/m4/daily_driver_quality_truth_packet/`](../../../fixtures/language/m4/daily_driver_quality_truth_packet/)
- Rust module: [`crates/aureline-language/src/daily_driver_quality_truth_packet/mod.rs`](../../../crates/aureline-language/src/daily_driver_quality_truth_packet/mod.rs)
