# Status-bar seed fixtures

Worked snapshots emitted by `aureline_shell::status_bar::StatusBarSnapshot`
when it projects upstream truth (target, profile, trust, encoding, and
background state) onto the protected M1 status-bar surface.

These fixtures are the truth a reviewer reads to confirm:

1. **Protected walk** — opening a workspace and a terminal yields a row per
   item kind synchronized with the upstream truth. No row is missing, no
   row carries a stale success state, and the rows are ordered by the
   contract priority ladder.
2. **Failure drill** — putting one upstream truth source into a degraded
   state surfaces a degraded label and (where appropriate) a
   recovery-critical promotion on the matching row instead of leaving a
   stale "Trusted"/"Reachable"/"UTF-8" label visible.
3. **Persistence over toasts** — every degraded posture is durable in the
   bar; the bar does not ask the user to read a transient toast or hidden
   debug log to learn the same truth.

## Cases

| Fixture | Scenario |
| --- | --- |
| [`protected_walk_local_workspace.json`](./protected_walk_local_workspace.json) | Local workspace, trusted, UTF-8/LF buffer, idle background work — every row reports the upstream truth without a degraded chip. |
| [`failure_drill_restricted_trust.json`](./failure_drill_restricted_trust.json) | Trust flips to `Restricted`. Trust row promotes to recovery-critical with a `PolicyBlocked` chip; no other row is invented as recovery-critical. |
| [`multi_degraded_remote_offline.json`](./multi_degraded_remote_offline.json) | Remote SSH target is unreachable, sync transport is offline, and the active buffer has mixed line endings. Multiple rows surface degraded chips simultaneously without dropping any other ambient truth. |

## Layout

Each case carries a frozen expected `StatusBarSnapshot` shape derived from
inputs declared in the same fixture, so a regression in the projection
function or the upstream vocabulary will surface as a fixture diff. The
integration test at
`crates/aureline-shell/tests/status_bar_state_cases.rs` replays the inputs
and asserts on the projected snapshot.
