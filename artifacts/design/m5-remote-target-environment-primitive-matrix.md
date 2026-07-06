# M5 remote-target-pill / environment-status-strip primitive — design matrix

Task M05-854 · Batch B100 · crate `aureline-shell`.

This is the hand-authored design companion to the machine-generated
`artifacts/components/m5-remote-target-environment-primitive.md` (minted by the
`aureline_shell_m5_remote_target_environment_primitive` bin). It shows how the shared
remote-target pill and environment status strip project the same source / scope /
readiness truth across all nine run-capable surfaces so a user can tell — from the
same place they launch work — which target and runtime won, where the value came
from, and whether the current state is ready, degraded, or blocked.

## Surface × truth-axis matrix

Every surface carries the **full** parity: all five pill parts, all seven strip
parts, all six target postures, all six readiness states, all five provenance states,
all five scopes, all eleven export fields, and all six accessibility routes. Parity
*is* the guarantee.

| Surface | Zone | Headline worked resolutions |
| --- | --- | --- |
| Run Console | `status_bar` | local project-pinned → `local_inline` + `ready`; connected remote, offline cache → `connected_healthy` + `degraded_cached` |
| Test Runner | `status_bar` | container connecting → `establishing` + `ready`; reconnecting remote → `reconnecting` + `degraded_unreachable_target` |
| Debug Session | `status_bar` | managed offline cache, narrowed value → `offline_cached` + `degraded_narrowed`; disconnected VM, policy-blocked → `disconnected` + `blocked_by_policy` |
| Notebook Runtime | `status_bar` | wasm sandbox unresolved → `connected_healthy` + `blocked_unresolved`; local default → `local_inline` + `ready` |
| Request Runner | `main_workspace` | connected remote resolved → `connected_healthy` + `ready`; managed offline cache → `offline_cached` + `degraded_cached` |
| Database Session | `main_workspace` | connected remote, policy-blocked → `connected_healthy` + `blocked_by_policy`; local project pin → `local_inline` + `ready` |
| Preview Server | `main_workspace` | connected container → `connected_healthy` + `ready`; reconnecting VM → `reconnecting` + `degraded_unreachable_target` |
| Pipeline Run | `main_workspace` | connected managed, narrowed → `connected_healthy` + `degraded_narrowed`; disconnected remote unresolved → `disconnected` + `blocked_unresolved` |
| Incident Surface | `main_workspace` | connected remote session override → `connected_healthy` + `ready`; container offline cache → `offline_cached` + `degraded_cached` |

## Acceptance-criterion coverage

- **AC1 — no inferring the active host/runtime from unrelated logs or settings.** The
  pill's mandatory parts (`target_identity`, `host_or_environment_class`,
  `connection_state`) and the strip's mandatory parts (`runtime_kind`,
  `resolved_label_version`, `winning_source`, `readiness_state`,
  `why_this_context_entrypoint`) surface the boundary and winning source inline. The
  `masks_host_or_environment_boundary` invariant must be `false` on every row.
- **AC2 — same source/scope/readiness truth when cached, narrowed, or policy-blocked.**
  The readiness ladder maps `cached_offline → degraded_cached`,
  `narrowed_approximate → degraded_narrowed`, `policy_blocked → blocked_by_policy`,
  and `unresolved → blocked_unresolved`. Lints `cached_or_narrowed_readiness_unproven`
  and `policy_blocked_readiness_unproven` fail the packet if no worked resolution
  proves those degraded / blocked outcomes; the vocabulary set is frozen identically
  for all nine surfaces.
- **AC3 — target identity and runtime resolution inspectable from the launch place.**
  Every resolved context exposes the one-step `why_context_entrypoint`, the
  `why_this_context_entrypoint` strip part is mandatory, and
  `hides_why_this_context_entrypoint` must be `false`.

## Degraded remote-target disclosure

The `remote_degraded_disclosure_unproven` lint requires at least one worked
resolution where a remote target is degraded (`reconnecting` / `offline_cached` /
`disconnected`) and its posture is not `connected_healthy`, so the pill always shows
degraded / reconnect state rather than masking it as connected.
