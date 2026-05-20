# Crash-Loop Recovery Center Report

This packet reviews the crash-loop recovery center: the product-owned surface
that repeated startup failures route into instead of an invisible restart
loop. The center keeps crash and build identity visible, narrows the blast
radius to a suspected fault domain and recent changes, and offers bounded,
command-backed recovery choices that prefer preserve/inspect/disable over
destructive cleanup.

## Evidence

| Evidence | Path |
| --- | --- |
| Boundary schema | `schemas/support/crash_loop_recovery.schema.json` |
| Rust evaluator | `crates/aureline-support/src/crash_loop_center/mod.rs` |
| Canonical fixtures | `fixtures/recovery/m3/crash_loop_center/` |
| Beta contract | `docs/support/m3/crash_loop_recovery_beta.md` |
| Tests | `crates/aureline-support/tests/crash_loop_center_beta.rs` |

## Review Findings

| Area | Result |
| --- | --- |
| No invisible loop | A center is only synthesized for a proven restart-budget breach (`strike_count >= strike_budget > 0`) or an explicit user request, and it pins `silent_restart_suppressed = true`. Unproven breaches are rejected. |
| Identity stays visible | The crash id, build id, restore class, suspected fault domain, fault-domain ref, and last reopen mode are carried verbatim into the center and the support packet; empty crash/build ids are rejected. |
| Bounded choices | Every center offers Enter safe mode, Open without restore, Open logs, Export crash manifest, and Report issue. There is no generic "try again" or "reset" choice class. |
| Narrowed blast radius | Each recent extension change yields a targeted, reversible Disable-extension choice; each recent profile/layout/setting change yields a targeted Disable-profile/layout choice that names the concrete suspect. |
| No-silent-rerun | Safe mode and Open without restore always pin `no_silent_rerun = true` and require explicit confirmation; for `local_mutating` and `privileged_or_remote` sessions they additionally require review. |
| Evidence-first | Recovered drafts, checkpoint diffs, rollbackable state, and the local-history timeline stay distinct evidence entry points rather than collapsing into the recovery choices. |
| Preserve over destroy | No synthesized choice deletes user-owned state, every choice is narrower than a full reset, and the center never suggests destructive cleanup. |
| Accessibility | Every choice and entry point is keyboard-complete with a screen-reader label and a unique focus order; the center carries a screen-reader summary and a focus-trap-safe posture. |

## Support Export Posture

`CrashLoopRecoverySupportPacket` compiles a metadata-safe envelope with:

- the visible crash id, build id, restore class, suspected fault domain,
  fault-domain ref, last reopen mode, and trigger class;
- one row per recovery choice (command id, review/confirmation/no-silent-rerun
  flags, disposition summary, preserved state classes, targeted suspect);
- one row per evidence entry point and per recent change;
- evidence refs carried by reference only.

`render_support_summary` renders a support-safe, screen-reader-legible view of
the same truth. The envelope carries opaque ids and closed-vocabulary tokens
only — raw paths, hosts, credentials, command lines, stack bodies, and live
authority handles never cross the boundary; `raw_private_material_excluded`,
`ambient_authority_excluded`, and `destructive_cleanup_suggested = false` are
pinned.

## Drill Coverage

The protected corpus exercises five postures:

- startup budget breach with an extension suspect and a privileged session
  (no-silent-rerun gating);
- reopen budget breach with profile and layout suspects and a mutating
  session;
- repeated restore-replay failure with an evidence-only restore class and
  draft + rollbackable-state entries;
- runtime-host budget breach with an unknown fault domain and no suspects;
- an explicit user request that opens the center without a budget breach.

## Follow-Ups

- Wire `CrashLoopRecoveryCenter` into the shell recovery surface and bind each
  `command_id` to its executor so the cards are live.
- Bridge the Disable-recently-changed-extension choice into the extension
  bisect flow when a single suspect is not obvious.
- Extend the corpus with a remote-attach session drill once a persistent
  remote-session transport lands.
