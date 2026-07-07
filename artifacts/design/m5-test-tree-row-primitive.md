# M5 Test-Tree-Row Primitive

- Packet: `m5-test-tree-row-primitive:stable:0001`
- Label: `M5 test-tree-row primitive: suite/template/concrete-case/notebook-backed/imported-result/partial-discovery item classes, stable identity, current state, last-result freshness, imported/live origin, target/environment shorthand, parameterized-case count, mute/quarantine and release impact, derived row posture, exact rerun scope, and bounded reveal/rerun/debug/review-quarantine/export actions`
- Test-surface consumers: 5 (5 stable)
- Row postures: quarantined_row, partial_discovery_row, imported_evidence_row, stale_result_row, suite_aggregate_row, live_concrete_row
- Rerun scopes: whole_suite, parameterized_group, single_case, notebook_cells, imported_replay_only, nothing_concrete_yet
- Item classes: suite, template, concrete_case, notebook_backed_item, imported_result, partial_discovery_placeholder
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Test-surface consumers

- **Test Explorer Tree**: `stable`
  - Owner: Test explorer tree owner
  - Scope: The test-explorer tree renders the shared test-tree row so a durable-keyed unit suite names its class, identity, and live-local origin with a whole-suite rerun scope, and a durable-keyed concrete case with a fresh live-local pass reads as the highest-certainty live-concrete row exposing single-case rerun and debug
  - Worked rows: 2
    - `tree:auth-unit-suite` (`suite` / `live_local`) → `suite_aggregate_row` (rerun `whole_suite`, live-certainty `false`, muted `false`)
    - `tree:auth-unit-suite::token-refresh` (`concrete_case` / `live_local`) → `live_concrete_row` (rerun `single_case`, live-certainty `true`, muted `false`)
- **Editor Gutter Tree**: `stable`
  - Owner: Editor gutter tree owner
  - Scope: The editor-gutter tree renders the shared test-tree row so a parameterized template names its 12-variant parameterized-group rerun scope without collapsing the count, and a notebook-backed item names its notebook-cells rerun scope and stays debuggable — neither widening its rerun scope
  - Worked rows: 2
    - `tree:integration::matrix-parse` (`template` / `live_local`) → `suite_aggregate_row` (rerun `parameterized_group`, live-certainty `false`, muted `false`)
    - `tree:notebook::data-load-smoke` (`notebook_backed_item` / `live_local`) → `live_concrete_row` (rerun `notebook_cells`, live-certainty `true`, muted `false`)
- **Run Panel Tree**: `stable`
  - Owner: Run panel tree owner
  - Scope: The run-panel tree renders the shared test-tree row so an imported CI result reads as an imported-evidence row that is replay-only and withholds the local rerun/debug it cannot honestly offer, and a stale live-local failure reads as a stale-result row that still exposes single-case rerun and debug — so imported evidence never inherits live certainty
  - Worked rows: 2
    - `tree:e2e::checkout-flow@ci` (`imported_result` / `imported_ci`) → `imported_evidence_row` (rerun `imported_replay_only`, live-certainty `false`, muted `false`)
    - `tree:pricing::round-half-even` (`concrete_case` / `live_local`) → `stale_result_row` (rerun `single_case`, live-certainty `false`, muted `false`)
- **Headless / CLI Tree**: `stable`
  - Owner: Headless CLI tree owner
  - Scope: The headless / CLI tree renders the shared test-tree row so a partial-discovery placeholder reads as a partial-discovery row with a nothing-concrete-yet rerun scope and no faked rerun, and a durable-keyed concrete case flagged flaky-suspected on a fresh live-local run reads as a live-concrete row exposing single-case rerun and debug — proving the same tree grammar works headless
  - Worked rows: 2
    - `tree:discovery::pending-spec-module` (`partial_discovery_placeholder` / `unknown_origin`) → `partial_discovery_row` (rerun `nothing_concrete_yet`, live-certainty `false`, muted `false`)
    - `tree:contract::schema-back-compat` (`concrete_case` / `live_local`) → `live_concrete_row` (rerun `single_case`, live-certainty `true`, muted `false`)
- **Test Report Export**: `stable`
  - Owner: Test report export owner
  - Scope: The test-report export renders the shared test-tree row so a team-owned quarantined concrete case reads as a quarantined row whose hidden-from-release impact heads it while still exposing rerun, debug, and review-quarantine, and a durable-keyed benchmark suite names its whole-suite rerun scope — the same row a reviewer reads elsewhere
  - Worked rows: 2
    - `tree:auth::login-redirect-quarantined` (`concrete_case` / `live_local`) → `quarantined_row` (rerun `single_case`, live-certainty `false`, muted `true`)
    - `tree:bench::render-suite` (`suite` / `live_local`) → `suite_aggregate_row` (rerun `whole_suite`, live-certainty `false`, muted `false`)
