# M5 terminal-tab / header-strip primitive contract

Task M05-853 · Batch B100 (runtime-boundary and repair components) · crate
`aureline-shell`.

This document is the human-readable contract for the one reusable **terminal-tab /
header-strip primitive**. It narrows the `terminal_tab` family frozen by the
[M05-852 runtime-boundary component matrix](m5_runtime_boundary_components_contract.md)
into a working primitive with a real resolver, so a user can orient *before typing*
and can tell whether a session is local, remote, containerized, managed, shared,
live, restored, or inspect-only.

The authoritative gate is the Rust validator and resolver in
`crates/aureline-shell/src/implement_the_m5_terminal_tab_and_header_strip_boundary_liveness_and_shared_control_primitive/`.
The export-safe boundary schema is
[`schemas/ui/m5-terminal-tab.schema.json`](../../schemas/ui/m5-terminal-tab.schema.json).
This doc explains intent; the code and schema are the truth.

## The two halves

1. **A resolver** — `resolve_terminal_tab(&M5TerminalTabResolutionInput) ->
   Result<M5ResolvedTerminalTab, M5TerminalTabResolutionError>`. It takes one
   session's title, host boundary, shell-integration quality, liveness, connection
   state, live-or-last-known cwd, and collaboration role / follow / reauthorization
   state, and derives:
   - the **input posture** — the headline verdict a user reads before typing;
   - the **cwd display state** — live cwd, last-known cwd, unavailable, or not
     reported by the shell;
   - the **shared-control posture** — surfaced in the tab chrome, not inferred from
     background collaboration metadata.
2. **A parity matrix** — `M5TerminalTabPrimitivePacket` — binding one row per
   claimed M5 terminal-console consumer to the same anatomy, postures, cwd states,
   shared-control postures, export fields, and non-visual accessibility routes, so
   the boundary and integration cues stay visible everywhere and the support export
   reconstructs boundary and liveness truth from one shared model.

## Terminal-console consumers (matrix rows)

The acceptance criteria require the cues to remain visible across five consumers,
each a row in the matrix:

| Consumer | Token | Shell zone |
| --- | --- | --- |
| Terminal Panel | `terminal_panel` | `bottom_panel` |
| Notebook Console | `notebook_console` | `bottom_panel` |
| Request Console | `request_console` | `main_workspace` |
| Preview Dev-Server Console | `preview_dev_server` | `main_workspace` |
| Incident Shell | `incident_shell` | `main_workspace` |

## Derived input posture (AC1: live vs restored before input)

`resolve_terminal_tab` derives exactly one posture, in priority order:

1. `closed_no_input` — the session has exited.
2. `read_only_restored` — the tab is a transcript-restored session.
3. `read_only_reconnecting` — the session dropped and is reconnecting.
4. `inspect_only_observer` — the participant's role is observer.
5. `reauthorization_blocked` — input is blocked pending reauthorization.
6. `write_capable_live` — a live, non-observer, authorized session.

Only `write_capable_live` sets `is_write_capable = true`. A `restored_from_transcript`
session is therefore **never** write-capable — a restored transcript can never be
confused with a live write-capable shell. The packet-level lint
`restored_write_confusion_unproven` requires at least one worked resolution that
proves a restored transcript resolving read-only and non-write-capable.

## Cwd-or-transcript state

`cwd_display` is derived from shell-integration quality and liveness:

- `cwd_not_reported_by_shell` — the shell integration does not report cwd
  (`command_marks_only`, `basic_pty_no_integration`).
- `live_cwd_reported` — a live session whose integration reports cwd.
- `last_known_cwd_shown` — a restored / reconnecting / closed session's last-known
  cwd.
- `cwd_unavailable` — the shell can report cwd but none is available (never invents
  a stale value).

## Shared-control posture (AC3: explicit, not inferred)

`shared_control_posture` is derived in the tab chrome:

- `solo_session` — no other participants.
- `reauthorization_required` — a shared session whose control is blocked pending
  reauthorization (`requires_reauthorization = true`).
- `shared_observer_only` — an observer.
- `shared_control_held` — the participant holds control (host / control-holder /
  presenter).
- `shared_following_presenter` — the participant follows the presenter.

The lints `shared_control_disclosure_unproven` and
`reauthorization_disclosure_unproven` require worked resolutions that prove a shared
session disclosing a non-solo posture and a reauthorization-blocked session
disclosing it.

## Resolver errors

`empty_session_title`, `remote_host_missing_connection_state`,
`local_host_with_connection_state`, `follow_state_without_role`,
`reauthorization_without_shared_session`, and `forbidden_session_material` (a title
or cwd carrying `://`, `secret`, `password`, `api_key`, or `bearer `).

## Hard invariants (all MUST be false on every row)

- `masks_host_or_runtime_boundary`
- `conflates_live_and_restored_session`
- `invents_private_terminal_grammar`
- `infers_shared_control_from_background_metadata`

## Reused vs minted vocabulary

Reused verbatim from the frozen runtime-boundary matrix (M05-852):
`M5ShellIntegrationQuality`, `M5TerminalSessionLiveness`, `M5HostBoundaryClass`,
`M5RemoteConnectionState`, `M5CollaborationRole`, `M5FollowState`,
`M5RuntimeBoundaryAccessibilityRoute`, `M5RuntimeBoundaryQualificationClass`, and
`M5RuntimeBoundaryDowngradeTrigger`. Reused from the frozen shell-zone matrix:
`M5ResponsiveClass`, `M5ShellConsumerSurface`, `M5ShellZoneSlot`, `M5WindowClass`.

Minted here (what the frozen matrix left implicit about the tab itself):
`M5TerminalConsoleSurface`, `M5TerminalTabAnatomyPart`, `M5TerminalInputPosture`,
`M5CwdDisplayState`, `M5SharedControlPosture`, and `M5TerminalTabExportField`.

## Support / export and fixtures

The bin `aureline_shell_m5_terminal_tab_primitive` is the only mint-from-truth path
for:

- `artifacts/release/m5-terminal-tab-proof/support_export.json` (+ `matrix.csv`)
- `artifacts/components/m5-terminal-tab-primitive.md`
- `fixtures/ui/m5-terminal-tab-primitive/{incident_shell_beta_narrowed,preview_dev_server_preview_narrowed}.json`

Raw URLs, local paths, usernames, hostnames, tokens, and credentials never cross
the boundary; session titles and cwds are carried only as opaque, export-safe
representations.
