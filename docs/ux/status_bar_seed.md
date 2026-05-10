# Status-bar seed: target, profile, trust, encoding, and background state

The status bar is the durable, persistent surface for ambient operational
truth. Toasts and banners are transient. OS notifications interrupt. Logs
are not visible without effort. The status bar is the row a user can glance
at to confirm:

- **Target** — what host class are we executing against, and is it reachable?
- **Profile** — which profile, deployment profile, and identity mode are bound?
- **Trust** — is this workspace trusted, restricted, or still evaluating?
- **Encoding** — what encoding, BOM, and line endings does the active buffer carry?
- **Background state** — is there ongoing work, and is its owner degraded?

This seed wires the smallest live surface that makes all five rows
truthful, ordered by the priority ladder, and capable of reporting a
degraded label instead of a stale success label when the upstream truth
flips.

## Truth sources

Every row is a thin projection over upstream truth — never a private cache:

| Row | Upstream truth source |
| --- | --- |
| Target | [`aureline_runtime::ExecutionContext::target_identity`](../../crates/aureline-runtime/src/execution_context/mod.rs) (or a terminal session header). |
| Profile | [`crate::chrome::title_context_bar::ProfileIdentity`](../../crates/aureline-shell/src/chrome/title_context_bar.rs). |
| Trust | [`aureline_workspace::TrustState`](../../crates/aureline-workspace/src/lib.rs) settled by the workspace lifecycle machine. |
| Encoding | [`aureline_workspace::save::SourceFidelityRecord`](../../crates/aureline-workspace/src/save/source_fidelity.rs). |
| Background state | Aggregated owners + degraded posture supplied by the activity center. |

The status-bar projection lives at
[`crates/aureline-shell/src/status_bar/mod.rs`](../../crates/aureline-shell/src/status_bar/mod.rs).
It exposes [`StatusBarSnapshot::project`] over a typed
[`StatusBarInputs`] payload; the live shell, support exports, and proof
fixtures all consume the same projection.

## Stable slots and priority

The seed renders one row per kind, mapped onto the stable slots frozen by
[`docs/ux/status_bar_contract.md`](./status_bar_contract.md):

| Item kind | Default class | Default rank | Stable slot |
| --- | --- | ---: | --- |
| `trust` | `active_context_truth` | 110 | `status.slot.context.workspace` |
| `target` | `active_context_truth` | 120 | `status.slot.context.execution` |
| `profile` | `active_context_truth` | 140 | `status.slot.context.workspace` |
| `background_state` | `ongoing_work` / `ambient_metadata` | 220 / 320 | `status.slot.work.summary` |
| `encoding` | `ambient_metadata` | 360 | `status.slot.metadata.file` |

A row is promoted to `recovery_critical` when its upstream truth becomes
consequence-bearing — for example, a workspace flipping to `Restricted`,
an encoding that decoded as `unknown_binary_like`, or an unreachable /
policy-blocked target. Promotions move the row to the front of the bar
because items are sorted by `priority_rank` ascending.

## Action contract

Each row carries a stable `primary_command_id` that opens the narrowest
useful inspector or settings detail; settings detours are never the
primary action. Activations are keyboard-reachable through the
`keyboard_target_id` carried on every row.

| Row | Primary activation opens |
| --- | --- |
| Target | `inspector.runtime.execution_context` |
| Profile | `inspector.settings.profile` |
| Trust | `inspector.workspace.trust_review` |
| Encoding | `inspector.editor.source_fidelity` |
| Background state | `surface.activity_center.work_summary` |

## Protected walk

Open a workspace and a terminal in the live shell. Confirm that:

1. Five rows appear: target, profile, trust, encoding, and background.
2. Each row reflects the upstream truth (target reachable, trust trusted,
   encoding UTF-8/LF, no background degradation).
3. No row carries a degraded chip; no row is promoted to recovery-critical.

Fixture: [`fixtures/ux/status_bar_cases/protected_walk_local_workspace.json`](../../fixtures/ux/status_bar_cases/protected_walk_local_workspace.json).
Test: `cargo test -p aureline-shell --test status_bar_state_cases protected_walk_local_workspace_case`.

## Failure drill

Put one upstream truth source into a degraded state and confirm the bar
reports the degraded label rather than a stale success state:

- Flip workspace trust to `Restricted`. The trust row promotes to
  `recovery_critical`, surfaces the `PolicyBlocked` chip, and leads the
  bar order. Other rows continue mirroring their upstream truth without
  inventing additional recovery-critical promotions.
  Fixture: [`failure_drill_restricted_trust.json`](../../fixtures/ux/status_bar_cases/failure_drill_restricted_trust.json).
- Lose reachability to a remote SSH target. The target row promotes to
  `recovery_critical` with an `Offline` chip even though trust and
  background state are nominal.
- Decode a binary-like file. The encoding row promotes to
  `recovery_critical` with a `PolicyBlocked` chip; the buffer is not
  silently treated as text.
- Flip the sync transport to offline while the active buffer carries
  mixed line endings. Multiple rows surface degraded chips simultaneously
  without dropping any other ambient truth.
  Fixture: [`multi_degraded_remote_offline.json`](../../fixtures/ux/status_bar_cases/multi_degraded_remote_offline.json).

## Verification

```
cargo test -p aureline-shell --lib status_bar
cargo test -p aureline-shell --test status_bar_state_cases
```

## Out of scope for the seed

- Extension-contributed status items, branding budgets, and the status menu
  are deferred to the broader `status_bar_contract.md` work.
- Per-owner background-job rows; the seed renders one aggregate row.
- Transient toast / banner replacement; routed notifications continue to
  flow through [`crate::notifications`](../../crates/aureline-shell/src/notifications/mod.rs).
