# Start Center, recent-work, and workspace-switcher disclosure — release evidence

Reviewer-facing evidence packet for the lane that makes the no-workspace Start
Center, recent-work list, and workspace switcher replacement-grade on claimed
stable rows: one canonical target-kind disclosure per entry target, a public
claim ceiling that never over-claims, recovery routes that keep a failed entry,
one model shared across surfaces, route parity across the Start Center,
switcher, command palette, and menus, accessibility across normal / high-contrast
/ zoomed layouts, and rows that stay available without an account or managed
services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace/`](../../../fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace/)
- Schema: [`/schemas/ux/stabilize-the-start-center-recent-work-list-workspace.schema.json`](../../../schemas/ux/stabilize-the-start-center-recent-work-list-workspace.schema.json)
- Companion doc: [`/docs/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md`](../../../docs/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md)
- Typed source: `aureline_shell::start_center_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_start_center_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/start_center_stable_fixtures.rs`

## What this packet proves

1. **Target-kind truth is consistent across local, remote, and managed
   examples.** Each record's `target_kind` / `target_kind_label` /
   `target_class` is the canonical `aureline_workspace` vocabulary, and
   `target_kind_label` is asserted equal to the canonical surface label so docs
   and Help/About ingest one vocabulary instead of hand-authored variants. The
   matrix spans File, Folder, multi-root Workspace, Repository, SSH, Dev
   container, and Cloud workspace.

2. **No row claims trust, restore fidelity, or remote availability it cannot
   prove.** Every record's `claim_ceiling` is bound to its real state: an
   unavailable target cannot assert a live open, a reconnect-blocked or local
   target cannot assert remote availability, a less-than-exact restore cannot
   assert full fidelity, and a non-trusted row cannot assert trust. The builder
   rejects each over-claim, and the gate replays it for all eight records.

3. **A failed open keeps the entry and its recovery routes.**
   `discards_stale_entry_on_failure` is fixed false. Each non-ready record
   exposes the recovery routes its failure state requires — Locate for
   missing/moved roots, Reconnect / Reauthorize and Retry for remote/managed
   targets, an open-minimal route where safe, and a `Remove from list` route
   that is metadata-only and preserves unrelated durable state.

4. **The Start Center and switcher share one model with a return path.** Each
   record's `surfaces` block proves `parity_holds`, shares the recovery action
   ids between both surfaces, and keeps the switcher's `cancel_switch` and
   `reopen_previous_workspace` return paths.

5. **The same target opens from every surface, keyboard-first.** Each record's
   `routes` reaches the target from the Start Center, switcher, command palette,
   and a menu command, each keyboard reachable, each activating the same
   canonical target, each a canonical durable-object ref.

6. **Accessibility holds in every layout.** Each record's `accessibility` block
   carries the tab order, row narration (which discloses the target kind),
   action labels matching the recovery routes, and per-mode reachability for
   normal, high-contrast, and zoomed layouts.

7. **No-account / no-managed-services local entry stays available.**
   `available_without_account` and `available_without_managed_services` are
   fixed true on every record; absent identity or services degrade a row's state
   (reconnect / reauthorize), they never hide it.

## Coverage matrix

| Record | Target kind | Class | Failure state | Trust | Restore | Honesty |
| --- | --- | --- | --- | --- | --- | --- |
| `local-folder-docs` | Folder | local | ready | trusted | exact | false |
| `local-file-manifest` | File | local | ready | trusted | compatible | true |
| `multi-root-platform` | Workspace | local | ready | trusted | exact | false |
| `payments-missing-path` | Repository | local | missing_path | trusted | compatible | true |
| `api-workspace-moved` | Workspace | local | moved_root | trusted | layout_only | true |
| `infra-ssh-unreachable` | SSH | remote_backed | reconnect_required | pending_evaluation | evidence_only | true |
| `web-devcontainer-offline` | Dev container | remote_backed | reconnect_required | restricted | compatible | true |
| `managed-data-expired` | Cloud workspace | managed | reconnect_required | pending_evaluation | layout_only | true |

## How to reproduce

```sh
cargo test -p aureline-shell --test start_center_stable_fixtures
cargo test -p aureline-shell --lib start_center_stable
cargo run -q -p aureline-shell --bin aureline_shell_start_center_stable -- index
```

The replay test fails if any checked-in fixture drifts from the in-code corpus;
regenerate with the emitter's `emit-fixtures` subcommand. The builder's
negative-path unit tests prove each honesty invariant rejects a dishonest input.

## Known limits / follow-ups

- The record is the canonical truth; the live native Start Center, recent-work
  list, workspace switcher, command palette, menus, diagnostics packet, and
  support bundle are expected to **ingest** it (and its `target_kind_label`
  vocabulary). Wiring those existing consumers to read this record instead of
  their current bespoke row text is the natural next consumer step and is
  intentionally left to the surfaces that own those exports.
- The matrix pins representative claimed-stable rows. Container workspaces,
  worksets, portable-state packages, and policy-blocked / quarantined targets
  share the same record shape and recovery taxonomy and are exercised by the
  builder's unit tests and the upstream recent-work tests rather than the
  on-disk corpus.
