# Trust-gated lifecycle-hook review

This document describes the lifecycle-hook review engine: how a repo-defined
lifecycle action becomes an explicit, reviewable, repairable object instead of
a silent side effect. The canonical implementation is
[`crates/aureline-env/src/hook_review/mod.rs`](../../crates/aureline-env/src/hook_review/mod.rs);
the corpus and expected outcomes are checked in under
[`fixtures/env/hook-review/`](../../fixtures/env/hook-review/) and the
human-readable proof is
[`artifacts/env/hook-review-and-repair.md`](../../artifacts/env/hook-review-and-repair.md).

It builds directly on the typed environment capsule described in
[`docs/env/environment-capsule.md`](environment-capsule.md): the capsule lane
carries the declared, trust-gated hooks an environment defines, and this lane
decides — in the current trust and policy context — which of them may run and
turns the rest into visible review objects.

## Why this exists

A starter, devcontainer, Compose project, Nix shell, direnv `.envrc`, or
bootstrap script declares lifecycle actions: preflight validators, build and
setup commands, and post-create hooks. Run silently, they are hidden execution
assumptions. Skipped silently — because the workspace is restricted, the policy
denies them, a secret is missing, the activator is unsupported, or an upstream
action failed — they are confusing no-ops that disappear without explanation.

This lane closes that gap. Every repo-defined action is surfaced as a
`LifecycleHook`, and the review engine gives each one an explicit disposition,
a named reason when it is held, a statement of what did or did not run, and a
repair — so nothing runs that should not, and nothing is silently dropped.

## The declared hook

A `LifecycleHook` is the review-safe statement of one repo-defined action:

- **`activator`** — the definition mechanism: `devcontainer`, `compose`, `nix`,
  `direnv`, `bootstrap`, or `post_create`.
- **`kind`** — what it is: a `lifecycle_hook`, a `preflight_validator`, or a
  `build_setup_command`.
- **`phase`** — the lifecycle phase it runs in (reused from the capsule).
- **`gate_state`** — whether its trust gate is `gated`, `pending_review`, or
  `ungated`.
- **`authority_ref`**, **`command_digest`** — the authority contract the gate
  binds to and a digest of the command. **The command body is never carried.**
- **`required_secrets`**, **`depends_on`** — the secrets it needs and the hooks
  it depends on.

## One engine, one object

`review_hooks` is the single engine. It folds each hook through the
`HookReviewContext` — `trusted`, `restricted_mode`, the supported and denied
activators, the denied hook kinds, the projected secrets, and the failed
actions — and returns one `HookReviewPacket` carrying, per hook, a
`HookReviewEntry` with:

- a `HookDisposition` — `allowed`, `review_required`, `restricted`, `denied`, or
  `blocked`,
- the `HookHoldReason` behind it (none when allowed),
- a review-safe `what_happens` line ("Ran: …" / "Did not run: …"),
- a `safe_next_step`, and
- a `HookRepair` that preserves the exact hook identity, reason, and next step.

The reason precedence (first match wins, in `direct_hold_reason`) is: a policy
denial, then an unsupported activator, then a failed action, then a missing
secret, then an ungated authority, then an uncleared gate, then restricted
mode. After the direct pass, a fixpoint pass propagates `upstream_blocked` to
any otherwise-clear hook that depends on a hook which will not run, so a failed
bootstrap action visibly blocks the post-create hook that needs it.

`desktop_hook_review`, `headless_hook_review`, `ai_hook_review`, and
`support_hook_review` all delegate to `review_hooks`, so desktop, CLI / headless,
AI, and support read the **same** object. A restricted-mode or policy-denied
user sees exactly what did not run and what still works, identically on every
surface.

## Restricted mode and policy denial

The packet's `what_did_not_run` and `what_still_works` lines are the heart of
the restricted-mode and policy-denied experience: they name the held hooks and
the safe subset that still runs. The rolled-up `HookReviewPosture` is
`all_cleared`, `review_pending`, `partially_blocked`, or `fully_blocked`, and
`HookReviewPosture::trust_hooks_evidence_state` maps it back onto the governance
trust-hooks `EvidenceState` so the review lane narrows the capsule's trust-hooks
dimension in lockstep rather than forking a parallel model.

## Repair and recovery

Every non-allowed hook carries a `HookRepair`: the `RepairKind`
(`request_approval`, `run_manually_after_review`, `request_policy_exception`,
`provide_missing_secret`, `enable_activator_support`, `retry_after_bootstrap_fix`,
or `repair_upstream_first`), the reason, a safe next step, the approval-lineage
reference the repair routes through, and a review-safe preview. The
`HookReviewDrill` corpus walks five failures — a blocked post-create hook, a
failed bootstrap action, a missing secret, an unsupported activator, and a
policy-denied lifecycle step — from injection through a visible held state and a
repair back to a cleared review.

## Export

`export_hook_review` projects a metadata-first `HookReviewExport` (posture, what
did not run, what still works, and the repairs) wrapping the same packet for
support and release surfaces, so a hook failure stays attributable and
recoverable through the support bundle.

## Guardrails

- **Never silently run a trust-gated action.** An ungated hook, a hook awaiting
  review, and a hook in an untrusted or restricted workspace are held and
  surfaced as suggestions — never run merely because a template or capsule
  references them.
- **A no-op is never silent.** A policy-denied step, a missing secret, an
  unsupported activator, a failed bootstrap action, and a hook blocked behind
  one of those each become a visible entry with a named reason and a repair.
- **No command bodies or secret values.** Hook commands are digests and secrets
  are named, never carried; the review is metadata-only.
