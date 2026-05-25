# Warm startup, warm restore, and first-useful-work — release evidence

Reviewer-facing evidence packet for the lane that makes warm shell startup,
warm restore, and first-useful-work routing replacement-grade on claimed stable
desktop rows: useful chrome before deep discovery, restore that says exactly /
partially / needs-review without implying full resumption, no silent rerun of
side-effectful sessions, bounded first-useful-work routing, zone-owned truth
that stays in its zone, and responsive fallback that keeps collapsed surfaces
reachable.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful/`](../../../fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful/)
- Schema: [`/schemas/ux/harden_shell_startup_warm_restore_and_first_useful.schema.json`](../../../schemas/ux/harden_shell_startup_warm_restore_and_first_useful.schema.json)
- Companion doc: [`/docs/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md`](../../../docs/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md)
- Typed source: `aureline_shell::warm_continuity` (`model`, `corpus`)
- Headless emitter: `aureline_shell_warm_continuity_corpus`
- Replay + invariant gate: `crates/aureline-shell/tests/warm_continuity_fixtures.rs`

## What this packet proves

1. **Warm startup paints useful chrome before deep discovery on every claimed
   stable row.** Each record's `startup` trace pins `shell_chrome_painted`,
   `command_entry_ready`, and `stable_focus_target` as present, keyboard-
   reachable, and reached before deep discovery completes; the builder rejects
   any record that omits one or reaches it late, and the gate replays it for all
   seven scenarios.

2. **Restore explains exactly / partially / needs-review without implying full
   resumption.** Each restore item carries a `provenance` token, narrowed items
   name a `downgrade_trigger`, and the derived `display_copy.full_resumption_
   implied` and `side_effect_rerun_implied` stay false. The `crash_recovery_
   drafts` drill shows recovered drafts and a schema-translated pane tree as
   honest partials; `expired_remote_session` shows evidence-only remote state.

3. **Side-effectful sessions are never silently rerun.** Every `no_rerun_surface`
   sets `auto_rerun_forbidden` and carries an explicit resume route; provider
   mutations, collaboration controls, and remote actions additionally require
   fresh authorization or review. The drills cover all seven surface classes —
   terminal, task, debug session, notebook cell, provider mutation,
   collaboration control, and remote action.

4. **First-useful-work routing is typed, bounded, inspectable, and
   non-destructive.** Each `landing` decision selects one of the five typed
   routes from its inspectable candidates, records a route reason, stays
   keyboard-reachable and non-destructive, and bounds any remembered preference
   so it can never widen trust or run setup. The `support_export_lines`
   projection surfaces the route and reason for diagnostics/support export.

5. **Regression drills hold without layout collapse or hidden rerun.** The
   corpus covers sleep/resume, display-topology change, missing extension,
   expired remote session, and revoked authorization. None blanks the workspace,
   relocates a zone-owned cue, or reruns a side-effectful surface; collapsed
   surfaces under compact/degraded layouts keep keyboard-reachable reopen routes
   and last-meaningful-state refs.

6. **Responsive fallback keeps zone-owned truth stable.** Across compact,
   standard, and expanded window classes, every `zone_owned_cue` renders in its
   owning zone; only `approved_to_move` surfaces collapse, to sheet, overlay, or
   overflow, each with an explicit reopen route.

## Coverage matrix

| Scenario | Entry cause | Restore class | Window class | Landing route | Honesty marker | exact/partial/review |
| --- | --- | --- | --- | --- | --- | --- |
| `warm_relaunch_exact` | warm_relaunch | exact_restore | standard_desktop | prior_active_editor | false | 5 / 0 / 0 |
| `crash_recovery_drafts` | crash_recovery | recovered_drafts | standard_desktop | changed_files_view | true | 2 / 2 / 0 |
| `sleep_resume` | sleep_resume | compatible_restore | expanded_desktop | prior_active_editor | true | 3 / 0 / 0 |
| `display_topology_change` | display_topology_change | layout_only | compact_desktop | post_entry_handoff_card | true | 1 / 3 / 0 |
| `missing_extension` | missing_extension_fallback | compatible_restore | standard_desktop | readme | true | 2 / 0 / 1 |
| `expired_remote_session` | expired_remote_session | evidence_only | standard_desktop | review_packet | true | 0 / 1 / 1 |
| `revoked_authorization` | revoked_authorization | compatible_restore | compact_desktop | post_entry_handoff_card | true | 2 / 0 / 1 |

## How to reproduce

```sh
cargo test -p aureline-shell --test warm_continuity_fixtures
cargo test -p aureline-shell --lib warm_continuity
cargo run -q -p aureline-shell --bin aureline_shell_warm_continuity_corpus -- index
```

The replay test fails if any checked-in fixture drifts from the in-code corpus;
regenerate with the emitter's `emit-fixtures` subcommand. The builder's
negative-path unit tests prove each honesty invariant rejects a dishonest input.

## Known limits / follow-ups

- The record is the canonical truth; the live native shell, diagnostics packet,
  and support bundle are expected to **ingest** it. Wiring those existing
  consumers to read this record (instead of their current bespoke status text)
  is the natural next consumer step and is intentionally left to the surfaces
  that own those exports.
- `no_restore` with user-authored items, and the `no_prior_context` routing
  reason, are exercised by the builder's unit tests rather than the on-disk
  corpus, since they are honesty edges rather than representative stable rows.
