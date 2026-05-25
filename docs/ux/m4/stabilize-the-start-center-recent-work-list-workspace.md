# Start Center, recent-work, and workspace-switcher target-kind disclosure (stable)

## Why this lane exists

The no-workspace Start Center, the in-workspace workspace switcher, and the
recent-work list all answer the same question before the user commits: *what
will open, can I trust it, how much will restore, and what can I do if it is not
reachable?* Each surface used to be a place the product could quietly mislead a
switching user:

- A switcher row could claim a remote was reachable while the Start Center row
  already showed it as disconnected.
- A missing folder or a moved root could render as an ordinary local open.
- A managed cloud workspace could imply a restore fidelity it could not prove.
- A failed open could silently drop the recent-work entry, losing the only
  affordance the user had to recover it.
- A local-open path could be buried beneath account or marketplace content, and
  a no-account user could lose their recent work entirely when identity or
  managed services were absent.

This lane closes that gap with **one governed record per entry target** that
every surface reads verbatim — it does **not** fork a per-surface row model.

## The governed record

`entry_target_disclosure_record` is minted by
`crates/aureline-shell/src/start_center_stable` and frozen at the boundary by
`schemas/ux/stabilize-the-start-center-recent-work-list-workspace.schema.json`.
The Start Center, recent-work list, workspace switcher, command palette, menus,
diagnostics, support exports, Help/About, and docs all read this single record,
so they cannot drift on target kind, trust, restore, recovery, route
reachability, or accessibility for the same target.

The record **re-projects rather than forks** the upstream recent-work
vocabulary. The target kind, target state, failure-state taxonomy, trust state,
restore availability, and safe recovery actions are the canonical
`aureline_workspace` recent-work types, and the corpus projects them through the
live `aureline_shell::start_center` and `aureline_shell::workspace_switcher`
builders before minting a record. The on-disk fixtures are therefore a genuine
projection of the shell's own row code.

It binds, for one canonical recent-work identity:

1. **Target-kind disclosure** — `target_kind`, `target_kind_label`,
   `target_class`, `location_subtitle` + `subtitle_kind`, `last_opened_at`,
   `trust_state`, and `restore_availability`, plus the `disclosure` block that
   records each fact is actually disclosed before commit.
2. **A public claim ceiling** — `claim_ceiling` names exactly what the row may
   assert: a live open, remote availability, full restore fidelity, or trust.
   The builder rejects any ceiling that over-claims for the row's real state.
3. **Recovery before commit** — `recovery_routes` (Locate, Reconnect /
   Reauthorize, an open-minimal route where safe, and a metadata-only Remove),
   with `discards_stale_entry_on_failure` fixed false.
4. **Cross-surface parity** — `surfaces` pairs the Start Center and switcher row
   ids, shares the recovery action ids, and keeps the switcher's cancel /
   reopen-previous return path.
5. **Route parity** — `routes` reaches the target from the Start Center,
   switcher, command palette, and a menu command, each keyboard reachable.
6. **Accessibility** — `accessibility` carries the tab order, row narration,
   action labels, and per-mode reachability for normal, high-contrast, and
   zoomed layouts.
7. **Availability** — `available_without_account` and
   `available_without_managed_services` are fixed true; absent identity or
   services degrade a row's state, they never hide it.

## The honesty invariants

The builder refuses to mint a record that would lie. Each is a `BuildError`, not
a warning:

- **No claim the product cannot prove.** `asserts_live_open` requires a ready
  target; `asserts_remote_available` requires a remote-backed target that is not
  reconnect-blocked; `asserts_full_restore_fidelity` requires exact restore; and
  `asserts_trusted_without_evaluation` requires a trusted row.
- **Recovery before commit.** A non-ready failure state must expose the recovery
  routes it requires; a `Remove from list` route must be metadata-only; and every
  route must preserve unrelated durable state.
- **Never silently discard a stale entry.** `discards_stale_entry_on_failure`
  must be false.
- **One model across surfaces.** `parity_holds` must be true, the surface
  recovery ids must equal the recovery routes, and the switcher must keep
  `cancel_switch` and `reopen_previous_workspace`.
- **Same routes everywhere.** All four route surfaces must be present, distinct,
  keyboard-reachable, and pointed at the same target.
- **Accessible in every layout.** Normal, high-contrast, and zoomed layouts must
  each keep narration and reachable affordances; the narration must disclose the
  target kind; and the action labels must match the recovery routes in order.
- **No buried local-open path.** A row may not be hidden when an account or
  managed services are absent.

## The claimed stable matrix

The corpus pins eight records spanning the required target kinds, target
classes, and recovery states:

| Record | Target kind | Class | Failure state | Trust | Restore |
| --- | --- | --- | --- | --- | --- |
| `local-folder-docs` | Folder | local | ready | trusted | exact |
| `local-file-manifest` | File | local | ready | trusted | compatible |
| `multi-root-platform` | Workspace | local | ready | trusted | exact |
| `payments-missing-path` | Repository | local | missing_path | trusted | compatible |
| `api-workspace-moved` | Workspace | local | moved_root | trusted | layout_only |
| `infra-ssh-unreachable` | SSH | remote_backed | reconnect_required | pending_evaluation | evidence_only |
| `web-devcontainer-offline` | Dev container | remote_backed | reconnect_required | restricted | compatible |
| `managed-data-expired` | Cloud workspace | managed | reconnect_required | pending_evaluation | layout_only |

## How to reproduce

```sh
cargo test -p aureline-shell --test start_center_stable_fixtures
cargo test -p aureline-shell --lib start_center_stable
cargo run -q -p aureline-shell --bin aureline_shell_start_center_stable -- index
```

The replay test fails if any checked-in fixture drifts from the in-code corpus;
regenerate with the emitter's `emit-fixtures` subcommand. The builder's
negative-path unit tests prove each honesty invariant rejects a dishonest input.

## What ingests this record

The record is the canonical truth source for this lane. The live Start Center,
recent-work list, workspace switcher, command palette, menu commands,
diagnostics packet, support bundle, and Help/About are expected to **ingest** it
(and its `target_kind_label` vocabulary) instead of cloning status text. Wiring
those existing consumers to read this record is the natural next consumer step
and is intentionally left to the surfaces that own those exports.
