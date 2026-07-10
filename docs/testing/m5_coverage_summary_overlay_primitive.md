# M5 coverage-summary-bar / coverage-overlay-marker primitive

This document is the contract reference for the reusable M5 **coverage-summary bar** and
**coverage-overlay marker** — two governed test-intelligence components implemented as one twin
primitive in the `aureline-runtime` crate
(`implement_coverage_summary_bars_and_coverage_overlay_markers_with_included_run_provenance_line_versus_branch_or_partial_truth_changed_file_emphasis_and_open_report_continuity_across_claimed_m5_test_surfaces`).

It narrows two of the seven families frozen by the
[test-intelligence component matrix](m5_test_intelligence_component_matrix.md) —
`coverage_summary_bar` and `coverage_overlay_marker` — into two resolvers plus a parity matrix,
so a green coverage number stops hiding what run set it measured and an editor gutter glyph
stops losing the exact coverage state and the path back to the evidence that produced it.

## Why this exists

A user should never trust a single coverage percentage without knowing whether it measured the
full suite, only changed files, a single shard, a merged multi-shard run, an imported report,
or a partial scope, nor whether the number is a live local run, an imported report, a merged
multi-run, a cached reuse, or a stale replay. And an editor overlay glyph should never be
ambiguous about whether a line is covered, uncovered, partial, branch-missed, excluded, or
unknown, nor lose the path back to the evidence object that produced it. This primitive makes
each of those states explicit and identical across every claimed coverage consumer.

## Coverage-summary bar

`resolve_coverage_summary_bar` takes one summary's scope class, metric kind, provenance class,
freshness state, source note, included run count, covered / total units, and shard-omission
flag, and derives a **coverage posture** that is one-to-one with the coverage scope class:

| Coverage scope | Coverage posture |
| --- | --- |
| `full_suite` | `full_suite_summary` |
| `changed_files_only` | `changed_files_summary` |
| `single_shard` | `single_shard_summary` |
| `merged_multi_shard` | `merged_multi_shard_summary` |
| `imported_report` | `imported_report_summary` |
| `partial_incomplete` | `partial_incomplete_summary` |

Because the map is one-to-one, no two scopes collapse into one percentage — the
acceptance-criterion axis. A multi-run or imported summary always sets
`requires_included_run_label`, so multi-run and imported evidence never collapse into one
unlabeled number. A shard omission (`discloses_shard_omission`) and a stale provenance
(`is_stale`) always stay visible.

Actions: `reveal_coverage_details`, `open_coverage_report`, and `export_coverage` are always
offered; `open_uncovered_lines` whenever there are uncovered units; `rerun_coverage` whenever
the number is not a fully current run.

## Coverage-overlay marker

`resolve_coverage_overlay_marker` takes one marker's coverage state, emphasis class, provenance
class, changed-line flag, source run-set ref, evidence object ref, and line reference, and
derives an **overlay posture** that is one-to-one with the frozen controlled coverage-overlay
vocabulary:

| Overlay state | Overlay posture | Needs attention? |
| --- | --- | --- |
| `covered_line` | `covered_marker` | no |
| `uncovered_line` | `uncovered_marker` | yes |
| `partially_covered` | `partial_marker` | yes |
| `branch_missed` | `branch_missed_marker` | yes |
| `excluded_line` | `excluded_marker` | no |
| `no_overlay_data` | `unknown_marker` | no |

Because the map is one-to-one, the marker always preserves its exact coverage-state meaning
(`preserves_state_meaning`) and never invents an alternate label. A changed line carrying a
`changed_line_emphasis`, `newly_uncovered`, or `regression_hotspot` emphasis is kept emphasized
(`is_emphasized_change`). The source run-set identity and a durable path back to the evidence
object are always preserved (a missing evidence ref fails resolution), so an editor overlay
never severs its `has_report_continuity` to the report.

Actions: `reveal_marker_details`, `open_coverage_report`, and `export_marker` are always
offered; `open_uncovered_context` whenever the marker flags a coverage gap.

## Parity matrix

`M5CoverageComponentsPacket` binds one row per claimed coverage consumer — the coverage-report
panel, the editor gutter overlay, the CI coverage summary, the headless/CLI coverage surface,
and the coverage report export — to the shared summary and overlay anatomy, vocabulary,
postures, actions, export fields, and non-visual accessibility routes, so the same coverage
grammar holds across the report, the editor, CI, headless/export, and support consumers with
identical vocabulary. Each row carries four hard invariants (all `false`):

- `collapses_multi_run_into_single_percentage`
- `hides_shard_omission_or_stale_provenance`
- `drops_line_versus_branch_dimension`
- `invents_alternate_coverage_state_label`

## Boundary

Raw coverage payloads, pasted paths, credentials, and private endpoints stay outside the export
boundary; every scope label, source run-set ref, evidence object ref, line reference, and
identity is carried only as an opaque, export-safe representation.

## Artifacts

- Canonical packet schema: `schemas/ui/m5-coverage-summary-bar.schema.json`
- Overlay-marker companion schema: `schemas/ui/m5-coverage-overlay-marker.schema.json`
- Support export: `artifacts/release/m5-coverage-summary-overlay-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-coverage-summary-overlay-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-coverage-summary-overlay-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-coverage-summary-overlay-primitive/`

All are minted from the seed builders by the
`aureline_runtime_coverage_summary_overlay_primitive` headless emitter; the checked-in support
export is asserted equal to the seed builder in tests.
