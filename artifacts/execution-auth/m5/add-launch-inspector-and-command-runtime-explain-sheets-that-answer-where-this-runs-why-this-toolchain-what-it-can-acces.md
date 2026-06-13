# M5 Launch-Inspector Explain Sheets

- Packet: `m5-launch-inspector-explain-sheets:stable:0001`
- Label: `M5 Launch-Inspector Explain Sheets`
- Sheets: 7 (0 degraded or unsupported)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-10T00:00:00Z)

## Explain sheets by route

- **desktop** (launch-explain:desktop:0001) — status: complete
  - Where it runs: notebook_kernel on macos_desktop (subprocess_isolated_local) via local_isolated
  - Why this toolchain: Project notebook kernel (pinned) (project_pinned_toolchain)
  - What it can access: read_workspace, write_workspace, process_spawn, network_egress · Secret scope: no_secret_access
  - Who approved it: issuer:approval-broker:local (`ticket:notebook-kernel:0001`) · Posture: ticket_required_per_session · Epoch: policy-epoch:m5:0007
- **cli** (launch-explain:cli:0001) — status: complete
  - Where it runs: scaffold_hook on headless_ci (subprocess_isolated_local) via local_isolated
  - Why this toolchain: Project scaffold generator (pinned) (project_pinned_toolchain)
  - What it can access: read_workspace, write_workspace, process_spawn · Secret scope: no_secret_access
  - Who approved it: issuer:approval-broker:local (`ticket:scaffold-hook:0001`) · Posture: ticket_required_per_action · Epoch: policy-epoch:m5:0007
- **ai** (launch-explain:ai:0001) — status: complete
  - Where it runs: ai_tool on macos_desktop (subprocess_isolated_local) via local_isolated
  - Why this toolchain: Isolated AI tool runtime (policy_mandated_runtime)
  - What it can access: read_workspace, write_workspace, network_egress, secret_handle_projection · Secret scope: handle_only_delegated
  - Who approved it: issuer:approval-broker:local (`ticket:ai-tool:0001`) · Posture: ticket_required_per_action · Epoch: policy-epoch:m5:0007
- **recipe** (launch-explain:recipe:0001) — status: complete
  - Where it runs: recipe on linux_desktop (subprocess_isolated_local) via local_isolated
  - Why this toolchain: Saved recipe runner (pinned) (project_pinned_toolchain)
  - What it can access: read_workspace, write_workspace, process_spawn · Secret scope: no_secret_access
  - Who approved it: issuer:approval-broker:local (`ticket:recipe:0001`) · Posture: ticket_required_per_scope · Epoch: policy-epoch:m5:0007
- **extension** (launch-explain:extension:0001) — status: complete
  - Where it runs: request_api_send on macos_desktop (brokered_network_only) via brokered_network
  - Why this toolchain: Brokered HTTPS transport (remote_brokered_runtime)
  - What it can access: network_egress, secret_handle_projection · Secret scope: handle_only_delegated
  - Who approved it: issuer:approval-broker:local (`ticket:request-api-send:0001`) · Posture: ticket_required_per_scope · Epoch: policy-epoch:m5:0007
- **remote** (launch-explain:remote:0001) — status: complete
  - Where it runs: remote_mutation on managed_remote_runtime (isolated_remote_runtime) via remote_isolated, off-device
  - Why this toolchain: Managed remote runtime (remote_brokered_runtime)
  - What it can access: remote_mutation, network_egress, secret_handle_projection · Secret scope: scoped_brokered_secret
  - Who approved it: issuer:remote-broker:managed (`ticket:remote-mutation:0001`) · Posture: ticket_required_per_action · Epoch: policy-epoch:m5:0007
- **companion** (launch-explain:companion:0001) — status: complete
  - Where it runs: browser_routed_action on managed_remote_runtime (isolated_remote_runtime) via remote_isolated, off-device
  - Why this toolchain: Companion-paired remote browser runtime (remote_brokered_runtime)
  - What it can access: browser_navigation, network_egress · Secret scope: no_secret_access
  - Who approved it: issuer:approval-broker:local (`ticket:browser-routed-action:0001`) · Posture: ticket_required_per_action · Epoch: policy-epoch:m5:0007
