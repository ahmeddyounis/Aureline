# Target / origin badges and host-boundary cues on M1 seed entry points

This is the reviewer-facing landing page for the M1 seed that surfaces
target / origin badges and host-boundary cues on every run-capable entry
point: the bottom-panel terminal pane, the task seed, the debug-prep
seed, and the provider / auth entry point.

The seed exists so a remote SSH host, a managed workspace, a
devcontainer, or a managed sign-in tenant cannot look like an ordinary
local desktop launch merely because the surface had limited room for
copy.

## Truth sources (do not fork)

The seed projects from upstream truth — it never invents its own
identity vocabulary.

- Target class, working directory, reachability, identity mode, and
  trust posture come from
  [`aureline_runtime::ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs).
- Account-boundary class, provider domain label, and pending-session
  state for provider / auth chips come from
  [`aureline_auth::BrowserCallbackPacket`](../../crates/aureline-auth/src/browser_callback/mod.rs).
- The shared shell-side projection lives in
  [`aureline_shell::badges::target_origin`](../../crates/aureline-shell/src/badges/target_origin/mod.rs).

When this page disagrees with those sources or the upstream PRD / TAD
under `.t2/docs/`, the upstream sources win and this page must update
in the same change.

## What the badge carries

Each badge is one record with a stable schema. Surfaces quote every
field verbatim — they never re-derive a `boundary_visible` boolean
locally.

| Field | Source | Notes |
|---|---|---|
| `entry_point` | Local | Names the surrounding chrome (terminal / task seed / debug-prep / provider auth). |
| `target_class` + `target_label` | `ExecutionContext::target_identity::target_class` | Mirrored vocabulary; provider / auth entries use the `provider_entry_point` variant because a sign-in chip does not execute user code on a host. |
| `canonical_target_id` | `ExecutionContext::target_identity` | For provider entries the id is `provider:<packet_id>` so the chip stays addressable. |
| `origin_class` + `origin_label` | `IdentityMode` (execution entries) or `AccountBoundaryClass` (provider entries) | Same vocabulary across both — a managed sign-in chip uses the same `Managed` token the terminal would use. |
| `boundary_cue` | Derived | Typed enum (`hidden`, `local_to_container`, `local_to_remote`, `local_to_managed`, `local_to_provider`, `degraded_trust`, `policy_blocked`, `unknown`) — never a bare boolean. |
| `boundary_cue_visible` | Derived | `true` for every cue except `hidden`. |
| `trust_state` + token | `ExecutionContext::policy_and_trust::trust_state` | Quoted verbatim. |
| `execution_context_ref` | `ExecutionContext::execution_context_id` | Lets a support export correlate the badge back to the resolved context. |
| `auth_packet_ref` | `BrowserCallbackPacket::packet_id` | Present on provider / auth entries; absent on pure-execution entries. |
| `honesty_marker_present` | Derived | `true` when the upstream record carries a degraded field, an unresolved trust posture, an unknown account boundary, or a denied pending session. |

## Boundary-cue precedence

The cue derivation is deterministic. Highest precedence first:

1. `unknown_boundary` on the auth packet → `Unknown` (fail-closed, honesty marker).
2. Reachability `policy_blocked` or a degraded field naming an
   activator block → `PolicyBlocked` (honesty marker).
3. `trust_state == pending_evaluation` → `DegradedTrust` (honesty marker).
4. Target class crosses the local desktop boundary → typed cue:
   - `ssh_remote`, `remote_workspace_vm`, `notebook_kernel_remote` → `LocalToRemote`
   - `container_local`, `devcontainer` → `LocalToContainer`
   - `managed_workspace`, `prebuild_runtime`, `ai_sandbox` → `LocalToManaged`
5. Provider / auth entry whose origin class is not `account_free_local`
   → `LocalToProvider` (the tenant boundary is crossed even when the
   execution target is local).
6. Otherwise → `Hidden`.

## Protected walk

Open the terminal, the task seed, the debug-prep seed, and any
provider / auth entry point against a trusted local-desktop seed and
inspect the badges. The expected truth:

- All four badges quote `Local` for the target.
- All execution-entry badges quote `Local only` for the origin.
- The provider / auth badge quotes the account-boundary class from the
  seed packet and shows the `provider_entry_point` target token.
- Every boundary cue is `Hidden` and no honesty marker is present.

The fixture
[`fixtures/runtime/target_origin_cases/local_terminal_protected_walk.json`](../../fixtures/runtime/target_origin_cases/local_terminal_protected_walk.json)
replays this walk against the same projection the live shell renders.

## Failure drills

Two failure drills exercise the seed:

- [`fixtures/runtime/target_origin_cases/remote_target_failure_drill.json`](../../fixtures/runtime/target_origin_cases/remote_target_failure_drill.json)
  — enter the lane from a context whose target is a remote SSH host.
  Every execution-entry badge MUST agree on `local_to_remote` so the
  user sees the same boundary truth in the terminal pane, the task
  card, and the debug-prep prompt. The `execution_entries_consistent()`
  helper is the assertion the chrome runs before painting the row.
- [`fixtures/runtime/target_origin_cases/pending_trust_honesty_drill.json`](../../fixtures/runtime/target_origin_cases/pending_trust_honesty_drill.json)
  — the resolver settles a local target while the workspace trust
  posture is still pending. Every execution-entry badge MUST surface
  the `degraded_trust` cue and an honesty marker rather than collapsing
  to a stale `Hidden` chip on the local target.

## Out of scope (M1 seed boundary)

This seed deliberately covers the minimum needed to keep boundary
truth visible across the M1 dogfood lanes. Out of scope:

- Full `host_identity_chip_record` fields — the badge cites the
  execution-context id and packet id rather than minting a new
  cross-surface chip.
- Boundary-change banners, reconnect lineage, or wrong-target
  reapproval flows — covered by the upstream
  [`docs/ux/host_identity_contract.md`](./host_identity_contract.md)
  contract for later milestones.
- Full provider sync, hosted-account flows, or remote-attach breadth
  beyond the M1 vocabulary.
