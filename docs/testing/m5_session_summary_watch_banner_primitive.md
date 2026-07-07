# M5 session-summary-bar / watch-mode-banner primitive

This document is the contract reference for the reusable M5 **session-summary bar** and
**watch-mode banner** — two governed test-explorer status components implemented as one twin
primitive in the `aureline-runtime` crate
(`implement_session_summary_bars_and_watch_mode_banners_with_exact_selection_running_backlog_retry_counts_live_reduced_polling_unavailable_state_last_successful_cycle_and_recover_pause_truth_across_claimed_m5_test_lanes`).

It narrows two of the seven families frozen by the
[test-explorer / watch / triage component matrix](m5_test_explorer_watch_triage_component_matrix.md)
— `session_summary_bar` and `watch_mode_banner` — into two resolvers plus a parity matrix,
so a long-running testing session stops collapsing into one ambiguous spinner and a degraded
watch stops hiding why it degraded.

## Why this exists

A user watching a test run should never have to guess whether the spinner means *discovering
tests*, *executing them*, *draining a watch backlog*, or *refreshing imported status*, nor
whether a "watching…" label is still live or has silently dropped to polling. This primitive
makes each of those states explicit and identical across every claimed status consumer.

## Session-summary bar

`resolve_session_summary_bar` takes one session's mode, activity phase, exact selection
scope, outcome, target/environment, attempt lineage (retry state), current watch fidelity,
and running/backlog/retry counts, and derives a **session posture** that is one-to-one with
the activity phase:

| Activity phase | Session posture |
| --- | --- |
| `discovering_tests` | `discovering_session` |
| `executing_tests` | `executing_session` |
| `processing_watch_backlog` | `watch_backlog_session` |
| `refreshing_imported_status` | `imported_refresh_session` |
| `settled_complete` | `settled_session` |

Because the map is one-to-one, discovery, execution, watch-backlog drain, and imported-status
refresh **never share one generic loading treatment** — the acceptance-criterion axis.

Actions: `reveal_session_details` and `export_session` are always offered;
`rerun_exact_selection` only once the session has settled; `cancel_running_session` only
while it is still pending; `open_watch_banner` only when the session watches for changes. The
current watch fidelity is always carried, and its degradation stays visible on the bar
(`watch_is_degraded`), so a degraded watch never hides behind a still-green summary.

## Watch-mode banner

`resolve_watch_mode_banner` takes one watch's fidelity state, optional degrade reason, last
successful cycle, and backlog, and derives a **watch posture** that is one-to-one with the
frozen controlled `live` / `reduced` / `polling` / `unavailable` (plus `paused` /
`reconnecting`) vocabulary:

| Watch fidelity | Watch posture | Degraded? |
| --- | --- | --- |
| `live` | `live_watch` | no |
| `reduced` | `reduced_watch` | yes |
| `polling` | `polling_watch` | yes |
| `reconnecting` | `reconnecting_watch` | yes |
| `paused` | `paused_watch` | no (user-initiated) |
| `unavailable` | `unavailable_watch` | yes |

A **degraded** watch (reduced / polling / reconnecting / unavailable) must carry a degrade
reason, or resolution fails with `missing_degrade_reason` — the banner never hides why
fidelity dropped. The last successful cycle is always preserved. `recover_watch` is offered
whenever the watch is not already live; `pause_watch` whenever there is an active watch to
pause (never for an already-paused or unavailable watch); `reveal_watch_details` and
`export_watch_state` are always offered.

## Parity matrix

`M5SessionWatchStatusPacket` binds one row per claimed status consumer — the test-explorer
status bar, the editor status bar, the run-panel status, the headless/CLI status, and the
session/watch report export — to the shared session and watch anatomy, vocabulary, postures,
actions, export fields, and non-visual accessibility routes, so the same status grammar
holds across the tree, status, headless/export, and triage consumers with identical
vocabulary. Each row carries four hard invariants (all `false`):

- `collapses_activity_into_one_spinner`
- `drops_retry_lineage`
- `invents_alternate_watch_label`
- `hides_watch_degrade_or_last_cycle`

## Boundary

Raw log bodies, pasted paths, credentials, and private endpoints stay outside the export
boundary; every selection label, watch label, and identity is carried only as an opaque,
export-safe representation.

## Artifacts

- Canonical packet schema: `schemas/ui/m5-test-session-summary-bar.schema.json`
- Watch-banner companion schema: `schemas/ui/m5-test-watch-mode-banner.schema.json`
- Support export: `artifacts/release/m5-session-summary-watch-banner-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-session-summary-watch-banner-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-session-summary-watch-banner-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-session-summary-watch-banner-primitive/`

All are minted from the seed builders by the
`aureline_runtime_session_summary_watch_banner_primitive` headless emitter; the checked-in
support export is asserted equal to the seed builder in tests.
