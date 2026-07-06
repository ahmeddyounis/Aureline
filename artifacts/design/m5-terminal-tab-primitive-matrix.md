# M5 terminal-tab / header-strip primitive — design matrix

Task M05-853 · Batch B100 · crate `aureline-shell`.

This is the hand-authored design companion to the machine-generated
`artifacts/components/m5-terminal-tab-primitive.md` (minted by the
`aureline_shell_m5_terminal_tab_primitive` bin). It shows how the shared terminal
tab projects the same boundary / liveness / shared-control truth across all five
terminal-console consumers so a user can orient before typing.

## Consumer × truth-axis matrix

Every consumer carries the **full** parity: all eight anatomy parts, all six input
postures, all four cwd display states, all five shared-control postures, all seven
export fields, and all six accessibility routes. Parity *is* the guarantee.

| Consumer | Zone | Headline worked resolutions |
| --- | --- | --- |
| Terminal Panel | `bottom_panel` | live local → `write_capable_live` + `live_cwd_reported`; remote restored → `read_only_restored` + `last_known_cwd_shown` |
| Notebook Console | `bottom_panel` | container detached kernel (control held) → `write_capable_live` + `shared_control_held`; dropped → `read_only_reconnecting` |
| Request Console | `main_workspace` | managed-host observer → `inspect_only_observer` + `cwd_not_reported_by_shell`; collaborator following → `shared_following_presenter` + `cwd_unavailable` |
| Preview Dev-Server | `main_workspace` | closed dev-server → `closed_no_input` + `last_known_cwd_shown`; wasm-sandbox pending reauth → `reauthorization_blocked` + `reauthorization_required` |
| Incident Shell | `main_workspace` | presenter driving live remote → `write_capable_live` + `shared_control_held`; restored incident log → `read_only_restored` |

## Acceptance-criterion coverage

- **AC1 — distinguish live PTY from restored transcript before input.** The derived
  `input_posture` puts `closed_no_input` / `read_only_restored` /
  `read_only_reconnecting` ahead of `write_capable_live`; `is_write_capable` is true
  only for `write_capable_live`. Lint `restored_write_confusion_unproven` fails the
  packet unless a restored transcript is proven read-only and non-write-capable.
- **AC2 — boundary and integration cues visible across all consumers.** The matrix
  requires all five consumer families (`required_console_missing`), each carrying
  the mandatory anatomy (`session_title`, `host_boundary_badge`, `liveness_state`)
  and mandatory export fields (`session_title`, `host_boundary`, `liveness`,
  `input_posture`).
- **AC3 — shared-control and reauthorization explicit, not inferred.**
  `shared_control_posture` is derived in the tab chrome and carried as an export
  field; the hard invariant `infers_shared_control_from_background_metadata` must be
  false. Lints `shared_control_disclosure_unproven` and
  `reauthorization_disclosure_unproven` require worked proofs.

## Narrowed lanes (qualification honesty)

Two checked-in fixtures show narrowing without hiding any consumer:

- `incident_shell_beta_narrowed` — the incident shell held at **Beta** (a slice of
  break-glass sessions do not yet render the reauthorization cue on every profile).
- `preview_dev_server_preview_narrowed` — the preview dev-server console narrowed to
  **Preview** pending last-known-cwd parity proof across every export path.

Both keep all five consumers present and validating; only the one row's
qualification changes.
