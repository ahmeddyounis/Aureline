# Warm startup, warm restore, and first-useful-work truth (stable)

## Why this lane exists

Every time the shell comes back **warm** — a plain relaunch, a crash recovery, a
sleep/resume, a display-topology change, a missing extension, an expired remote
session, or a revoked authorization — three things must happen, in order, and
each one used to be a place the product could quietly lie:

- **Paint useful chrome first.** The window, canonical zones, a usable command
  entry, and a stable keyboard focus target must appear *before* deep discovery
  (full indexing, remote reconnect, provider hydration) finishes. A
  startup-only spinner that blocks the whole surface is not useful chrome.
- **Restore honestly.** Local editors, layout, pane trees, tabs, status
  surfaces, and non-mutating context come back — but the product must say what
  came back **exactly**, what came back **partially**, and what now **needs
  review**, and it must never imply the live runtime resumed. A restored layout
  is not a re-run terminal.
- **Route to the next useful action.** A typed landing decision picks the prior
  active editor, a changed-files view, the README, a review packet, or a
  post-entry handoff card, and **records why** — without widening trust,
  installing packages, applying a workflow bundle, or suppressing a required
  admission checkpoint.

Before this lane, the workspace could blank behind a full-surface spinner while
a recoverable frame existed, a restored layout could read as a fully resumed
runtime, a terminal/task/debugger could silently rerun, a zone-owned cue (a
breadcrumb, a trust badge, an execution-target cue) could slide into a different
chrome position during hydration or resize, a remembered preference could
quietly widen trust, and a compact layout could drop a side surface with no way
back to it.

This lane closes that gap with **one governed record** every surface reads
verbatim — it does **not** fork a per-surface loading state.

## The governed record

`warm_continuity_record` is minted by
`crates/aureline-shell/src/warm_continuity` and frozen at the boundary by
`schemas/ux/harden_shell_startup_warm_restore_and_first_useful.schema.json`. The
desktop shell, diagnostics, support exports, and Help/About all read this single
record, so they cannot drift on startup order, restore fidelity, rerun safety,
routing, zone-owned truth, or responsive fallback for the same warm cycle.

The record re-projects (rather than forks) the upstream restore vocabularies:
the restore-fidelity classes mirror
`aureline_recovery::session_restore::records::RestoreClass`, the downgrade
triggers mirror its `DowngradeTriggerClass`, the canonical zones mirror
`aureline_shell::layout::ShellZoneId`, and the window classes mirror
`aureline_shell::layout::AdaptiveClass`.

It carries five bound sections:

1. **`startup`** — the skeleton-first → hydrate-second milestone trace.
2. **`restore`** — the restore class, the per-item provenance split, and the
   side-effectful surfaces held with no auto-rerun.
3. **`landing`** — the typed first-useful-work decision, its reason, its
   inspectable candidates, and any bounded remembered preference.
4. **`zone_identity`** — the zone-owned truth cues and where each rendered.
5. **`responsive`** — the current window class and any collapsed side surfaces.

## The honesty invariants the builder enforces

`WarmContinuityRecord::build` refuses to mint a record that would lie. Each of
these is a `BuildError`, not a warning, so a dishonest projection fails the row
instead of shipping:

- **Useful-chrome-first.** The three early milestones — `shell_chrome_painted`,
  `command_entry_ready`, `stable_focus_target` — must all be present, all
  `reached_before_deep_discovery`, and all keyboard-reachable.
- **No implied full resumption.** A restore preserves user-authored state and
  layout but may never imply the live runtime resumed; user-authored items can
  never be present under a `no_restore` class.
- **Narrowed items name why.** Every `restored_partially` / `needs_review` item
  carries a `downgrade_trigger`.
- **No silent rerun.** Every side-effectful surface (terminal, task, debug
  session, notebook cell, provider mutation, collaboration control, remote
  action) sets `auto_rerun_forbidden`, carries an explicit `resume_route_ref`,
  and — when it is remote- or authority-bound — requires fresh authorization or
  review.
- **Bounded routing.** The selected route is one of the inspectable
  `candidate_routes`, is keyboard-reachable and non-destructive, and any
  remembered preference may influence routing only — never widen workspace
  trust, install packages, apply a workflow bundle, or suppress a required
  checkpoint.
- **Zone-owned truth stays put.** A breadcrumb, trust badge, execution-target
  cue, workspace identity, or status summary may update its label/placeholder
  during warm-up but may never render outside its owning zone.
- **Collapsed surfaces stay reachable.** A surface that collapses to a sheet,
  overlay, or overflow keeps an explicit, keyboard-reachable `reopen_route_ref`
  and a `last_meaningful_state_ref`, and only `approved_to_move` surfaces move.

The derived `display_copy` block restates the "no lie" outcome as an inspectable
set of booleans that are all false on any minted record.

## What never crosses this boundary

Raw paths, raw command lines, raw URLs, raw tokens, raw provider payloads, and
raw user content never appear on these records. Every affordance carries an
opaque `aureline://<class>/<id>` ref and a short reviewable sentence. The
`support_export_lines` projection is the redaction-safe, deterministic block the
support bundle and diagnostics packet quote verbatim, so first-useful-work
routing is inspectable in support export without leaking content.

## The drill corpus

`crates/aureline-shell/src/warm_continuity/corpus.rs` mints one scenario per
named drill and pins each rendered record bit-for-bit under
`fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful/`. The corpus
covers the five required regression cases — **sleep/resume**, **display-topology
change**, **missing extension**, **expired remote session**, and **revoked
authorization** — plus the **warm relaunch** and **crash recovery** baselines,
and it exercises every window class, every landing route, every restore class,
every side-effectful surface, every collapse target, and every zone-owned cue.

| Scenario | Entry cause | Restore class | Window class | Landing route |
| --- | --- | --- | --- | --- |
| `warm_relaunch_exact` | `warm_relaunch` | `exact_restore` | `standard_desktop` | `prior_active_editor` |
| `crash_recovery_drafts` | `crash_recovery` | `recovered_drafts` | `standard_desktop` | `changed_files_view` |
| `sleep_resume` | `sleep_resume` | `compatible_restore` | `expanded_desktop` | `prior_active_editor` |
| `display_topology_change` | `display_topology_change` | `layout_only` | `compact_desktop` | `post_entry_handoff_card` |
| `missing_extension` | `missing_extension_fallback` | `compatible_restore` | `standard_desktop` | `readme` |
| `expired_remote_session` | `expired_remote_session` | `evidence_only` | `standard_desktop` | `review_packet` |
| `revoked_authorization` | `revoked_authorization` | `compatible_restore` | `compact_desktop` | `post_entry_handoff_card` |

## Regenerating and inspecting

```sh
# Refresh the on-disk fixtures (the test fails if they drift).
cargo run -q -p aureline-shell \
  --bin aureline_shell_warm_continuity_corpus -- emit-fixtures \
  fixtures/ux/m4/harden_shell_startup_warm_restore_and_first_useful

# Stable corpus index — scenario id, cause, restore class, route, rollups.
cargo run -q -p aureline-shell \
  --bin aureline_shell_warm_continuity_corpus -- index

# Per-scenario support-export truth block.
cargo run -q -p aureline-shell \
  --bin aureline_shell_warm_continuity_corpus -- plaintext

# Replay + invariant gate.
cargo test -p aureline-shell --test warm_continuity_fixtures
```

## Consumers

The shell paints from this record; diagnostics, support exports, and Help/About
**ingest** it rather than cloning its status text. Later dashboards and docs in
this lane should read the checked-in fixtures and schema as the canonical truth
source for warm-startup honesty on claimed stable rows.
