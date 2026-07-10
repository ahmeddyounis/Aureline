# M5 Flaky-State-Badge / Retry-History-Row Primitive

- Packet: `m5-flaky-retry-primitive:stable:0001`
- Label: `M5 flaky-state-badge / retry-history-row primitive: controlled flaky classification, classifier confidence, classifier source, retry-window visibility, last outcome, mute/quarantine status, distinct stable/suspected/reproduced/stable-again/muted/unknown flaky postures, controlled passed-first-try/passed-on-retry/failed-all-retries/errored/skipped/aborted retry postures, ordered attempt outcomes, environment/build/runtime deltas, local/remote/notebook/imported-CI attempt origins, a required evidence window before a reproduced verdict, and bounded reveal/open-retry-history/rerun/mute-or-quarantine and reveal/rerun/open-logs/export actions`
- Quality consumers: 5 (5 stable)
- Flaky postures: stable_badge, suspected_flaky_badge, reproduced_flaky_badge, stable_again_badge, manually_muted_badge, unknown_flaky_badge
- Retry postures: passed_first_try_row, passed_on_retry_row, failed_all_retries_row, errored_row, skipped_row, aborted_row
- Classifier sources: local_heuristic, statistical_model, imported_ci_classifier, manual_override, unknown_classifier
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Quality consumers

- **Flaky Dashboard Panel**: `stable`
  - Owner: Flaky dashboard panel owner
  - Scope: The flaky dashboard renders the shared flaky-state badge so a reproduced-flaky verdict measured over eight attempts with five observed failures reads as a confirmed flake, while a single-occurrence suspicion stays a suspected-flaky badge rather than borrowing the authority of a reproduced verdict; it renders the shared retry-history row so a divergent pass-on-retry names its ordered outcomes and the environment delta that explains why the same test passed on the second attempt
  - Worked badges: 2 / rows: 1
    - badge `flaky-badge:dashboard::reproduced-checkout` (`reproduced_flaky`) -> `reproduced_flaky_badge` (confirmed `true`, window `true`, muted `false`)
    - badge `flaky-badge:dashboard::suspected-payment` (`suspected_flaky`) -> `suspected_flaky_badge` (confirmed `false`, window `false`, muted `false`)
    - row `test:dashboard::checkout-flow` (`passed_on_retry`) -> `passed_on_retry_row` (divergent `true`, delta `true`, logs `true`)
- **Editor / Test-Tree Badge**: `stable`
  - Owner: Editor / test-tree badge owner
  - Scope: The editor / test-tree flaky badge renders the shared flaky-state badge so a stable test reads as a stable badge with its high-confidence statistical classifier source shown, and it renders the shared retry-history row so a clean first-try pass rerun on a remote attempt keeps its stable test identity and a path back to the raw logs
  - Worked badges: 1 / rows: 1
    - badge `flaky-badge:editor::stable-parser` (`stable`) -> `stable_badge` (confirmed `false`, window `false`, muted `false`)
    - row `test:editor::parser-unit` (`passed_first_try`) -> `passed_first_try_row` (divergent `false`, delta `true`, logs `true`)
- **Retry History Panel**: `stable`
  - Owner: Retry history panel owner
  - Scope: The retry-history panel renders the shared flaky-state badge so a previously flaky test that is stable again reads as a stable-again badge, and it renders the shared retry-history row so a failed-all-retries row on a notebook attempt discloses its widened rerun scope and its runtime delta rather than presenting the rerun as the same selection, and an errored row keeps its errored meaning
  - Worked badges: 1 / rows: 2
    - badge `flaky-badge:retry::stable-again-index` (`stable_again`) -> `stable_again_badge` (confirmed `false`, window `false`, muted `false`)
    - row `test:retry::index-rebuild` (`failed_all_retries`) -> `failed_all_retries_row` (divergent `false`, delta `true`, logs `true`)
    - row `test:retry::notebook-cell` (`errored_attempt`) -> `errored_row` (divergent `false`, delta `true`, logs `true`)
- **Headless / CLI Flaky-Retry**: `stable`
  - Owner: Headless CLI flaky-retry owner
  - Scope: The headless / CLI flaky-retry surface renders the shared flaky-state badge so a manually-quarantined verdict reads as a manually-muted badge that keeps its quarantine status disclosed rather than silently suppressing a failure, and it renders the shared retry-history row so a skipped attempt imported from CI reads as a skipped row that names its imported origin — proving the same grammar works without a desktop surface
  - Worked badges: 1 / rows: 1
    - badge `flaky-badge:headless::quarantined-network` (`manually_muted`) -> `manually_muted_badge` (confirmed `false`, window `true`, muted `true`)
    - row `test:headless::network-timeout` (`skipped_attempt`) -> `skipped_row` (divergent `false`, delta `true`, logs `true`)
- **Flaky-Retry Export**: `stable`
  - Owner: Flaky-retry export owner
  - Scope: The flaky-retry export renders the shared flaky-state badge so a verdict with insufficient data reads as an unknown-flaky badge rather than a settled one, and it renders the shared retry-history row so an aborted attempt reads with the same aborted vocabulary a reviewer sees in the dashboard and the editor, with a path back to the raw logs
  - Worked badges: 1 / rows: 1
    - badge `flaky-badge:export::unknown-migration` (`unknown_flaky`) -> `unknown_flaky_badge` (confirmed `false`, window `false`, muted `false`)
    - row `test:export::migration-smoke` (`aborted_attempt`) -> `aborted_row` (divergent `false`, delta `true`, logs `true`)
