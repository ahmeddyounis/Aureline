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
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- diagnostics
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- validate
```
