# Hook-review and repair proof

This report is the human-readable proof for the trust-gated lifecycle-hook
review engine, its restricted-mode and policy-denied guidance, and its repair
flows. The canonical implementation is
[`crates/aureline-env/src/hook_review/mod.rs`](../../crates/aureline-env/src/hook_review/mod.rs);
the corpus and its expected outcomes are checked in under
[`fixtures/env/hook-review/`](../../fixtures/env/hook-review/) and validated by
`crates/aureline-env/tests/hook_review.rs`.

## What the review is

A `LifecycleHook` is the review-safe statement of one repo-defined lifecycle
action — a devcontainer, Compose, Nix, direnv, bootstrap, or post-create
activator carrying a preflight validator, a build/setup command, or a lifecycle
hook. The command is reduced to a digest and the secrets it needs are named, so
no command body, secret value, or provider payload crosses the boundary.

`review_hooks` folds each hook through the current trust and policy context and
returns one `HookReviewPacket`: a per-hook `HookDisposition`, the
`HookHoldReason` behind it, a statement of what did or did not run, a safe next
step, and a `HookRepair`. Desktop, CLI / headless, AI, and support read the
**same** object, so a restricted-mode or policy-denied user sees the same
review on every surface. The rolled-up `HookReviewPosture` maps back onto the
governance trust-hooks evidence state, so the review lane narrows the capsule's
trust-hooks dimension in lockstep.

## Scenario corpus

The seeded hook set spans all six activators: a bootstrap preflight validator
and setup command, a devcontainer post-create hook that depends on the
bootstrap action, a Compose start command, a Nix activation, a direnv load, and
a post-create seed that needs the `DATABASE_URL` secret.

| Fixture | Profile | Target | Posture | Runnable | Reason tokens |
| --- | --- | --- | --- | --- | --- |
| `hook_review_all_cleared` | `starter` | `local` | `all_cleared` | 7 / 7 | — |
| `hook_review_review_required` | `workspace_template` | `container` | `review_pending` | 0 / 7 | `awaiting_approval` |
| `hook_review_ungated_hook` | `devcontainer` | `devcontainer` | `review_pending` | 6 / 7 | `ungated_authority` |
| `hook_review_restricted_mode` | `remote_container` | `ssh` | `review_pending` | 0 / 7 | `restricted_mode` |
| `hook_review_policy_denied` | `managed_workspace` | `managed_workspace` | `partially_blocked` | 6 / 7 | `policy_denied` |
| `hook_review_missing_secret` | `prebuild` | `container` | `partially_blocked` | 6 / 7 | `missing_secret` |
| `hook_review_unsupported_activator` | `devcontainer` | `vm` | `partially_blocked` | 6 / 7 | `unsupported_activator` |
| `hook_review_bootstrap_failed` | `starter` | `container` | `partially_blocked` | 5 / 7 | `bootstrap_failed`, `upstream_blocked` |
| `hook_review_fully_blocked` | `remote_container` | `container` | `fully_blocked` | 0 / 7 | `policy_denied` |

The corpus covers all six profiles and six target classes, every posture, and
every disposition (`allowed`, `review_required`, `restricted`, `denied`,
`blocked`).

## Failure / recovery drills

| Drill | Injected reason | Disposition | Degraded posture | Repair | Recovers to |
| --- | --- | --- | --- | --- | --- |
| `drill.blocked_post_create` | `upstream_blocked` | `blocked` | `partially_blocked` | `repair_upstream_first` | `all_cleared` |
| `drill.failed_bootstrap` | `bootstrap_failed` | `blocked` | `partially_blocked` | `retry_after_bootstrap_fix` | `all_cleared` |
| `drill.missing_secret` | `missing_secret` | `blocked` | `partially_blocked` | `provide_missing_secret` | `all_cleared` |
| `drill.unsupported_activator` | `unsupported_activator` | `blocked` | `partially_blocked` | `enable_activator_support` | `all_cleared` |
| `drill.policy_denied` | `policy_denied` | `denied` | `partially_blocked` | `request_policy_exception` | `all_cleared` |

Each drill walks the hook from injection through a visible held state and a
repair back to a cleared review, asserting no silent execution, that the reason
and next step are preserved, and that the hook recovers after repair.

## Guardrails proven end-to-end

- **No silent execution.** The `ungated` fixture proves an ungated hook is held
  for review and never run, even on a trusted, unrestricted path. The
  `review_required` and `restricted_mode` fixtures hold every hook rather than
  running it.
- **No silent no-ops.** The `policy_denied`, `missing_secret`,
  `unsupported_activator`, and `bootstrap_failed` fixtures each surface the held
  hooks with a named reason and a repair while the safe subset still runs, and
  `what_still_works` names that subset.
- **Cascades stay visible.** In `hook_review_bootstrap_failed` the failed
  bootstrap action blocks both itself and the post-create hook that depends on
  it (`upstream_blocked`), rather than the post-create hook disappearing.
- **Attributable and recoverable.** Every repair preserves the exact hook id,
  the reason, the next step, and the approval-lineage reference, and the
  metadata-first export carries them into the support bundle.

## How to verify

```
cargo test -p aureline-env
cargo run -p aureline-env --example dump_hook_review fixtures
```
