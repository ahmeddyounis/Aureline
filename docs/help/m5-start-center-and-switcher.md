# Start Center and project switching for M5 surfaces

Aureline resumes and switches work from one canonical recent-work list. The
Start Center (shown when no workspace is open) and the in-workspace project
switcher both project the **same** rows, so the target kind, trust state, and
what can be safely restored read identically whether you are cold-starting or
switching from an active session. The new M5 entry surfaces — notebooks,
request/data workspaces, profiler captures, framework packs, companion
handoff, and managed sync — reuse this list instead of shipping their own
launchers.

## Quick actions are command-backed cards

The first Start Center controls are five governed quick-action cards, not hero
tiles or marketing prompts:

| Card | Target truth | Disclosure |
|---|---|---|
| Open folder | Local folder or repository root | `cmd:workspace.open_folder`, folder icon, local-path account optional |
| Open workspace | Workspace or workset manifest | `cmd:workspace.open_folder` scoped to workspace files |
| Clone repository | Remote repository | `cmd:workspace.clone_repository`, review-before-trust badge |
| Restore last session | Recovery checkpoint or prior root | `cmd:workspace.restore_from_checkpoint`, restore-fidelity badge |
| Import from… | Portable state, handoff packet, or imported config | `cmd:workspace.import_profile`, compare-before-apply badge |

Each card carries an icon, verb-first label, short helper text, optional badge,
command id, and shortcut-state disclosure. The cards remain visible before
sign-in and before network readiness so local open and restore paths are not
hidden behind setup.

## Target kinds stay distinct

Every recent row keeps its real target kind instead of collapsing into a
generic "recent project" row:

- **Local folder** — a folder, file, or repository opened in place.
- **Workspace file** — a saved single-root workspace.
- **Multi-root workspace** — a multi-root (workset) manifest.
- **SSH target** — an SSH or remote-repository-backed workspace.
- **Container / dev container** — a container or dev-container workspace.
- **Managed workspace** — a managed cloud workspace.
- **Import packet** — an imported state package, handoff packet, or imported
  config root.
- **Bundle-backed entry** — a template, prebuild, or launch-bundle entry.

Import packets and managed workspaces are never silently treated as ordinary
local folders.

## Trust and restore are shown before you open

Each row shows its trust state (trusted, restricted, or pending evaluation) and
how much of the prior session can be restored (exact, compatible, layout-only,
evidence-only, or none) **before** activation. A probable, relocated, or
unreachable target never silently widens trust — the displayed trust always
matches the stored trust on both surfaces.

The restore vocabulary is canonical everywhere:

| Token | Label | Meaning |
|---|---|---|
| `exact_restore` | Exact restore | Same object identity and session state can return without translation. |
| `compatible_restore` | Compatible restore | Same object identity can return after a declared compatible translation or rebind. |
| `layout_only` | Layout only | Window, pane, or editor layout can return, but live session state cannot. |
| `recovered_drafts` | Recovered drafts | Dirty buffers or drafts can return without claiming a full session restore. |
| `evidence_only` | Evidence only | Evidence can be exported or inspected, but not replayed as active state. |
| `no_restore` | No restore | No restorable state is available for the entry. |

Start Center, crash recovery, manual switchers, support diagnostics, and
headless exports use those exact labels and tokens.

## Workspace-switcher entries preserve window and boundary truth

The workspace switcher renders a richer entry record over the same recent-work
object. Each entry carries:

- The canonical object identity ref (`filesystem_identity_ref`,
  `remote_target_descriptor_ref`, `artifact_descriptor_ref`, or recent-work
  fallback).
- Open-window state: current window, open in another window, reopen available,
  or blocked/unavailable.
- Selected profile and keymap refs when crossing local, remote, managed,
  imported, or starter-template boundaries.
- Local/remote/managed/imported/cached badges.
- Dirty-session state, dirty-buffer count, and the canonical restore badge.
- Close window, reopen previous workspace, move to new window, open/transfer,
  reconnect, reauthorize, and cancel actions as applicable.

Switching to a different project cannot destroy the current context silently:
the action row says whether Aureline will reuse the current window, transfer an
already-open window, open another window, or keep the previous workspace
available for reopen.

## Restore-prompt cards explain the safe path

Restore prompts are also projections of the same object identity and restore
vocabulary. A prompt shows a redaction-safe session summary, dirty-buffer
count, canonical restore class, partial/unsafe reasons, and the safest next
action. Safe mode, open without restore, clear journal, and export evidence are
visible affordances on every prompt, including crash-recovery and support
diagnostics projections.

## Missing, moved, and partial targets

When a target cannot open as an ordinary live workspace, the row says so and
offers keyboard-complete recovery actions instead of failing silently:

| State | What you see | Recovery |
|---|---|---|
| Missing root | The local path or mount is gone | Locate, open anyway, remove from list |
| Relocated workspace | The root moved from its stored identity | Locate, open anyway, remove from list |
| Stale target | Only cached metadata is available | Open read-only cached view, remove |
| Remote host unreachable | An SSH, container, or managed host is down | Reconnect / reauthorize, retry later |
| Partial restore | The target is reachable but only layout/evidence can be restored | Open with the available restore |

Pin/unpin and remove-from-list are always available, and every action is
reachable from the keyboard.

## Support and release evidence

The same truth is published as an export-safe packet so support and release
review can cite a row's state without a private dashboard lookup. Diagnostics
for missing-root, relocated-workspace, stale-target, remote-host, and
partial-restore states are redacted to the target-kind label and carry no raw
path, host, or credential body. The published audit lives at
`artifacts/ux/m5/start-center-and-switcher-audit.md`.

Reusable M5 project-entry cards, rows, and sheets are frozen in
`artifacts/design/m5-project-entry-component-matrix.md`. Start Center quick
actions, recent rows, workspace-switcher entries, restore cards, entry review
sheets, destination-collision sheets, post-entry handoff cards, admission
checkpoint cards, and archetype-readiness rows must use the schema refs under
`schemas/ui/m5-*-*.schema.json` instead of inventing local target-kind,
restore, trust, or setup-urgency wording.

## Inspecting the packet

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- packet
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- quick-actions
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- switcher-entries
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- restore-prompts
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- restore-vocabulary
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- diagnostics
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- validate
```
