# M5 Terminal-Tab, Remote-Target-Pill, Environment-Status-Strip, Toolchain-Pin-Row, Presence-Avatar-Stack, and Repair-Action-Card Component Matrix

- Packet: `m5-runtime-boundary-components:stable:0001`
- Label: `M5 terminal-tab, remote-target-pill, environment-status-strip, toolchain-pin-row, presence-avatar-stack, and repair-action-card component matrix`
- Component families: 6 (6 stable)
- Host-boundary classes: local_host, remote_ssh_host, container_host, managed_workspace_host, virtual_machine_host, wasm_sandbox_host
- Reversibility classes: fully_reversible_checkpoint, reversible_with_backup, partially_reversible, irreversible_confirmed, reversal_requires_manual_steps
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Component families

- **terminal_tab**: `stable`
  - Owner: Terminal/session component owner
  - Scope: One terminal-tab model carrying the session title, the host boundary it runs against, and the true shell-integration quality; it never implies richer integration than the live session provides and never conflates a live session with a restored transcript
  - Shell zone: `bottom_panel`
  - Required labels: identity, state, keyboard_route, boundary, resolved_source
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **remote_target_pill**: `stable`
  - Owner: Remote/transport component owner
  - Scope: One remote-target-pill model naming the host boundary — local, remote, container, managed workspace, virtual machine, or sandbox — and the live connection state, so a remote or offline target is never masked as a healthy local one
  - Shell zone: `title_context_bar`
  - Required labels: identity, state, keyboard_route, boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **environment_status_strip**: `stable`
  - Owner: Environment/runtime component owner
  - Scope: One environment-status-strip model naming the winning runtime source — project pin, workspace, tool manager, system default, container, or session override — so a user never has to infer which runtime is active or why it won
  - Shell zone: `status_bar`
  - Required labels: identity, state, keyboard_route, resolved_source, boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **toolchain_pin_row**: `stable`
  - Owner: Toolchain/tooling component owner
  - Scope: One toolchain-pin-row model explaining why a toolchain won — the source that selected it and its pin state — so a missing, conflicting, or overridden pin is disclosed rather than shown as a clean resolution
  - Shell zone: `right_inspector`
  - Required labels: identity, state, keyboard_route, resolved_source
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **presence_avatar_stack**: `stable`
  - Owner: Collaboration/presence component owner
  - Scope: One presence-avatar-stack model showing each participant's collaboration role and follow state, so an observer is never conflated with a controller and who-follows-whom is always explicit
  - Shell zone: `title_context_bar`
  - Required labels: identity, state, keyboard_route, boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **repair_action_card**: `stable`
  - Owner: Repair/diagnostics component owner
  - Scope: One repair-action-card model showing a repair's blast radius and reversibility class before approval, so a user always knows what a repair will change and whether it can be undone; it never understates blast radius or overstates reversibility
  - Shell zone: `transient_overlay`
  - Required labels: identity, state, keyboard_route, reversibility, boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
