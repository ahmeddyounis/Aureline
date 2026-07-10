# M5 Coverage-Summary-Bar / Coverage-Overlay-Marker Primitive

- Packet: `m5-coverage-summary-overlay-primitive:stable:0001`
- Label: `M5 coverage-summary-bar / coverage-overlay-marker primitive: coverage scope, line-versus-branch-or-combined metric dimension, included run set, freshness, imported/merged/live source note, distinct full-suite/changed-files/single-shard/merged-multi-shard/imported-report/partial-incomplete coverage postures, controlled covered/uncovered/partial/branch-missed/excluded/unknown overlay postures, preserved changed-line emphasis and source run-set identity, durable path back to the evidence object, and bounded reveal/open-uncovered-lines/open-report/rerun and reveal/open-report/open-uncovered-context/export actions`
- Coverage consumers: 5 (5 stable)
- Coverage postures: full_suite_summary, changed_files_summary, single_shard_summary, merged_multi_shard_summary, imported_report_summary, partial_incomplete_summary
- Overlay postures: covered_marker, uncovered_marker, partial_marker, branch_missed_marker, excluded_marker, unknown_marker
- Source notes: live_local_run, imported_report, merged_multi_run, cached_reuse, stale_replay
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Coverage consumers

- **Coverage Report Panel**: `stable`
  - Owner: Coverage report panel owner
  - Scope: The coverage-report panel renders the shared coverage-summary bar so a fresh full-suite line-coverage number reads as a full-suite summary whose uncovered lines can be opened, and a merged multi-shard number reads as a distinct merged-multi-shard summary that always shows its included run set rather than collapsing four shards into one percentage; it renders the shared overlay marker so a stably-covered line and an emphasized newly-uncovered changed line each keep their exact coverage state and a path back to the evidence object
  - Worked summaries: 2 / overlays: 2
    - summary `coverage:report::full-suite-line` (`full_suite`) -> `full_suite_summary` (multi-run `false`, imported `false`, stale `false`)
    - summary `coverage:report::merged-four-shard` (`merged_multi_shard`) -> `merged_multi_shard_summary` (multi-run `true`, imported `false`, stale `false`)
    - overlay `coverage-object:report::covered-line-42` (`covered_line`) -> `covered_marker` (changed `false`, continuity `true`)
    - overlay `coverage-object:report::uncovered-line-88` (`uncovered_line`) -> `uncovered_marker` (changed `true`, continuity `true`)
- **Editor Gutter Overlay**: `stable`
  - Owner: Editor gutter overlay owner
  - Scope: The editor gutter overlay renders the shared coverage-summary bar so a fresh changed-files line-coverage summary reads as a changed-files summary, and it renders the shared overlay marker so a partially-covered regression hotspot and a branch-missed marker on changed lines keep their exact partial / branch-missed meaning, stay emphasized as changed lines, and preserve a durable path back to the coverage evidence — the editor-to-report continuity
  - Worked summaries: 1 / overlays: 2
    - summary `coverage:editor::changed-files-line` (`changed_files_only`) -> `changed_files_summary` (multi-run `false`, imported `false`, stale `false`)
    - overlay `coverage-object:editor::partial-line-17` (`partially_covered`) -> `partial_marker` (changed `true`, continuity `true`)
    - overlay `coverage-object:editor::branch-line-23` (`branch_missed`) -> `branch_missed_marker` (changed `true`, continuity `true`)
- **CI Coverage Summary**: `stable`
  - Owner: CI coverage summary owner
  - Scope: The CI coverage summary renders the shared coverage-summary bar so an imported branch-coverage report from a CI artifact reads as a distinct imported-report summary that names its imported source note and included run set rather than passing as a fresh local number, and it renders the shared overlay marker so an excluded line keeps its excluded meaning
  - Worked summaries: 1 / overlays: 1
    - summary `coverage:ci::imported-branch-report` (`imported_report`) -> `imported_report_summary` (multi-run `true`, imported `true`, stale `false`)
    - overlay `coverage-object:ci::excluded-line-5` (`excluded_line`) -> `excluded_marker` (changed `false`, continuity `true`)
- **Headless / CLI Coverage**: `stable`
  - Owner: Headless CLI coverage owner
  - Scope: The headless / CLI coverage surface renders the shared coverage-summary bar so a cached single-shard region-coverage summary reads as a distinct single-shard summary that names its cached source note and offers a rerun of the non-current number, and it renders the shared overlay marker so a line with no overlay data reads as an unknown marker rather than a covered one — proving the same coverage grammar works without a desktop surface
  - Worked summaries: 1 / overlays: 1
    - summary `coverage:headless::single-shard-region` (`single_shard`) -> `single_shard_summary` (multi-run `false`, imported `false`, stale `false`)
    - overlay `coverage-object:headless::no-data-line-9` (`no_overlay_data`) -> `unknown_marker` (changed `false`, continuity `true`)
- **Coverage Report Export**: `stable`
  - Owner: Coverage report export owner
  - Scope: The coverage report export renders the shared coverage-summary bar so a stale partial-incomplete summary discloses its shard omission and stale provenance instead of presenting a green number, names its included run set, and offers a rerun, and it renders the shared overlay marker so a covered line reads with the same covered vocabulary a reviewer sees in the report and the editor
  - Worked summaries: 1 / overlays: 1
    - summary `coverage:export::partial-incomplete` (`partial_incomplete`) -> `partial_incomplete_summary` (multi-run `true`, imported `false`, stale `true`)
    - overlay `coverage-object:export::covered-line-101` (`covered_line`) -> `covered_marker` (changed `false`, continuity `true`)
