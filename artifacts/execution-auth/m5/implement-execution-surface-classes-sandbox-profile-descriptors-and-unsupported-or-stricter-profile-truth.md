# M5 Execution-Surface Resolution

- Packet: `m5-execution-surface-resolution:stable:0001`
- Label: `M5 Execution-Surface Classes, Sandbox-Profile Descriptors, and Unsupported-or-Stricter-Profile Resolution`
- Derived from matrix: `m5-runtime-authority-matrix:stable:0001`
- Profiles: 6 · Launch paths: 10 · Platforms: 5

## Launch-path classes

- **task_execution** → surface `scaffold_hook` (local_isolated)
- **terminal_session** → surface `notebook_kernel` (local_isolated)
- **notebook_cell** → surface `notebook_kernel` (local_isolated)
- **request_send** → surface `request_api_send` (brokered_network)
- **database_query** → surface `database_action` (brokered_network)
- **debug_session** → surface `notebook_kernel` (local_isolated)
- **connector_action** → surface `request_api_send` (brokered_network)
- **ai_tool_call** → surface `ai_tool` (local_isolated)
- **browser_routed_action** → surface `browser_routed_action` (remote_isolated)
- **remote_mutation** → surface `remote_mutation` (remote_isolated)

## Sandbox-profile descriptors

- `sandbox.inert_no_execution` v1 (inert) — No code execution; read-only inert surface.
- `sandbox.in_process_trusted_local` v1 (in_process) — Runs in-process under the host policy epoch with no isolation boundary.
- `sandbox.brokered_network_only` v1 (brokered_network) — No local process; network egress brokered through the transport plane.
- `sandbox.subprocess_isolated_local` v1 (local_isolated) — Runs in an isolated local subprocess with a scoped capability envelope.
- `sandbox.container_isolated_local` v1 (local_isolated) — Runs in a container-isolated local runtime.
- `sandbox.isolated_remote_runtime` v1 (remote_isolated) — Runs in an isolated remote runtime confined to a managed sandbox.

## Platform resolution

### linux_desktop

- **request_api_send**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `stable`
- **database_action**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **notebook_kernel**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **scaffold_hook**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **preview_server**: `sandbox.container_isolated_local` → `sandbox.container_isolated_local` [supported] · qual `beta`
- **ai_tool**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **recipe**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **browser_routed_action**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`
- **incident_flow**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **remote_mutation**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`

### macos_desktop

- **request_api_send**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `stable`
- **database_action**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **notebook_kernel**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **scaffold_hook**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **preview_server**: `sandbox.container_isolated_local` → `sandbox.isolated_remote_runtime` [narrowed_to_stricter_profile] · qual `preview`
- **ai_tool**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **recipe**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **browser_routed_action**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`
- **incident_flow**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **remote_mutation**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`

### windows_desktop

- **request_api_send**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `stable`
- **database_action**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **notebook_kernel**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **scaffold_hook**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **preview_server**: `sandbox.container_isolated_local` → `sandbox.isolated_remote_runtime` [narrowed_to_stricter_profile] · qual `preview`
- **ai_tool**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **recipe**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **browser_routed_action**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`
- **incident_flow**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **remote_mutation**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`

### managed_remote_runtime

- **request_api_send**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `stable`
- **database_action**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **notebook_kernel**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **scaffold_hook**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **preview_server**: `sandbox.container_isolated_local` → `sandbox.container_isolated_local` [supported] · qual `beta`
- **ai_tool**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **recipe**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **browser_routed_action**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`
- **incident_flow**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **remote_mutation**: `sandbox.isolated_remote_runtime` → `sandbox.isolated_remote_runtime` [supported] · qual `preview`

### headless_ci

- **request_api_send**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `stable`
- **database_action**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **notebook_kernel**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **scaffold_hook**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **preview_server**: `sandbox.container_isolated_local` → `sandbox.inert_no_execution` [narrowed_to_stricter_profile] · qual `preview`
- **ai_tool**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **recipe**: `sandbox.subprocess_isolated_local` → `sandbox.subprocess_isolated_local` [supported] · qual `beta`
- **browser_routed_action**: `sandbox.isolated_remote_runtime` → `(none — fails closed)` [unsupported_fail_closed] · qual `unavailable`
- **incident_flow**: `sandbox.brokered_network_only` → `sandbox.brokered_network_only` [supported] · qual `beta`
- **remote_mutation**: `sandbox.isolated_remote_runtime` → `(none — fails closed)` [unsupported_fail_closed] · qual `unavailable`

