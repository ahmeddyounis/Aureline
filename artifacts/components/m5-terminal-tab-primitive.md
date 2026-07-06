# M5 Terminal-Tab and Header-Strip Primitive: Boundary, Liveness, and Shared Control

- Packet: `m5-terminal-tab-primitive:stable:0001`
- Label: `M5 terminal-tab and header-strip primitive: session title, host boundary, shell-integration quality, cwd-or-transcript state, and shared control`
- Terminal-console consumers: 5 (5 stable)
- Input postures: write_capable_live, read_only_restored, read_only_reconnecting, inspect_only_observer, reauthorization_blocked, closed_no_input
- Cwd display states: live_cwd_reported, last_known_cwd_shown, cwd_unavailable, cwd_not_reported_by_shell
- Shared-control postures: solo_session, shared_control_held, shared_observer_only, shared_following_presenter, reauthorization_required
- Host-boundary classes: local_host, remote_ssh_host, container_host, managed_workspace_host, virtual_machine_host, wasm_sandbox_host
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Terminal-console consumers

- **Terminal Panel**: `stable`
  - Owner: Terminal panel owner
  - Scope: The terminal panel renders the shared terminal tab so a live local PTY reads as write-capable with its live cwd, while a remote session restored from a transcript reads as read-only with its last-known cwd — never confused with a live write-capable shell
  - Shell zone: `bottom_panel`
  - Worked resolutions: 2
    - `app-server` on `local_host` → `write_capable_live` (cwd `live_cwd_reported`, `solo_session`)
    - `api-server` on `remote_ssh_host` → `read_only_restored` (cwd `last_known_cwd_shown`, `solo_session`)
- **Notebook Console**: `stable`
  - Owner: Notebook console owner
  - Scope: The notebook kernel console renders the shared terminal tab so a container-hosted detached-running kernel with a held control token reads as shared-control-held and write-capable, while a dropped session reads as read-only-reconnecting with its last-known cwd
  - Shell zone: `bottom_panel`
  - Worked resolutions: 2
    - `kernel-py` on `container_host` → `write_capable_live` (cwd `live_cwd_reported`, `shared_control_held`)
    - `kernel-r` on `remote_ssh_host` → `read_only_reconnecting` (cwd `last_known_cwd_shown`, `solo_session`)
- **Request Console**: `stable`
  - Owner: Request console owner
  - Scope: The request/REPL console renders the shared terminal tab so an observer on a managed workspace host reads as inspect-only with cwd-not-reported by the shell, while a collaborator following the presenter reads as shared-following-presenter with cwd-unavailable rather than a stale value
  - Shell zone: `main_workspace`
  - Worked resolutions: 2
    - `repl-managed` on `managed_workspace_host` → `inspect_only_observer` (cwd `cwd_not_reported_by_shell`, `shared_observer_only`)
    - `repl-local` on `local_host` → `write_capable_live` (cwd `cwd_unavailable`, `shared_following_presenter`)
- **Preview Dev-Server Console**: `stable`
  - Owner: Preview dev-server owner
  - Scope: The preview dev-server console renders the shared terminal tab so a closed dev-server reads as closed-no-input with its last-known cwd, while a shared wasm-sandbox session pending reauthorization reads as reauthorization-blocked and reauthorization-required rather than silently allowing input
  - Shell zone: `main_workspace`
  - Worked resolutions: 2
    - `vite-preview` on `virtual_machine_host` → `closed_no_input` (cwd `last_known_cwd_shown`, `solo_session`)
    - `wasm-preview` on `wasm_sandbox_host` → `reauthorization_blocked` (cwd `cwd_not_reported_by_shell`, `reauthorization_required`)
- **Incident Shell**: `stable`
  - Owner: Incident shell owner
  - Scope: The incident/break-glass shell renders the shared terminal tab so a presenter driving a live remote session reads as shared-control-held and write-capable with its live cwd, while a restored incident transcript reads as read-only with its last-known cwd — boundary and liveness legible before any keystroke
  - Shell zone: `main_workspace`
  - Worked resolutions: 2
    - `incident-triage` on `remote_ssh_host` → `write_capable_live` (cwd `live_cwd_reported`, `shared_control_held`)
    - `incident-log` on `container_host` → `read_only_restored` (cwd `last_known_cwd_shown`, `solo_session`)
