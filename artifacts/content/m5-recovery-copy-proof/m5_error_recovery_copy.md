# Error/Recovery Copy Objects and Degraded-State Reason Chips

- Catalog: `m5-error-recovery-copy-catalog:stable:0001`
- Label: `Error/Recovery Copy Objects and Degraded-State Reason Chips`
- Reference locale: `en`
- Reason chips: 8 | Recovery blocks: 6
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Degraded-state reason chips

- `chip.restricted` (restricted, token `restricted`, caution): The capability is permitted only within a narrower, disclosed scope until the restriction is lifted.
- `chip.partial_index` (partial_index, token `partial_index`, caution): The index is still building, so results cover only the part indexed so far.
- `chip.remote_host` (remote_host, token `remote_host`, notice): The work depends on a remote host whose reachability is not guaranteed.
- `chip.policy_blocked` (policy_blocked, token `policy_blocked`, blocking): An active policy blocks the action; it cannot run as requested on this deployment.
- `chip.cached` (cached, token `cached`, notice): Data is shown from a local cache with a disclosed cache posture, not proven current.
- `chip.stale` (stale, token `stale`, warning): Prior data is shown after its freshness floor was passed; it may no longer be current.
- `chip.reconnecting` (reconnecting, token `reconnecting`, notice): A connection is being re-established; the state clears on its own when the link returns.
- `chip.rollback_available` (rollback_available, token `rollback_available`, notice): A prior, known-good state is retained and can be restored.

## Recovery blocks

- `recovery.network.remote_host_unreachable` [network / warning]
  - Failed: The Remote host connection to {host_name} dropped. Why: The network path to {host_name} is likely unavailable right now. Still works: Local edits, search, and history stay available; data shows as Stale while Reconnecting. Next: Reconnect to {host_name} (Open reconnect status)
  - Chips: chip.reconnecting, chip.remote_host, chip.stale
- `recovery.runtime.policy_blocked_action` [runtime / blocking]
  - Failed: Policy blocked: {action_name} cannot run on this deployment. Why: An administrator policy named {policy_name} disallows this action. Still works: Read-only work continues; allowed actions still run and recent results stay Cached. Next: Request access for {action_name} (Open policy settings)
  - Chips: chip.cached, chip.policy_blocked
- `recovery.repair.partial_index` [repair / caution]
  - Failed: Search is running on a Partial index; some results are missing. Why: Indexing of {scope_name} has not finished after the last change. Still works: Indexed files search normally; results so far are correct and shown as Cached. Next: Rebuild the project index (Open repair)
  - Chips: chip.cached, chip.partial_index
- `recovery.install.partial_install_rollback` [install / critical]
  - Failed: Installing {package_name} version {version_code} failed partway. Why: The install step likely could not write to {install_path}. Still works: The previous version keeps running; a Rollback available restores it cleanly. Next: Roll back the install (Open rollback)
  - Chips: chip.rollback_available
- `recovery.review.restricted_scope` [review / caution]
  - Failed: Applying the change is Restricted to {scope_name}. Why: The change touches paths outside the reviewed scope {scope_name}. Still works: In-scope edits apply now; out-of-scope changes stay Cached for a later review. Next: Request review to widen scope (Open review help)
  - Chips: chip.cached, chip.restricted
- `recovery.docs_help.stale_offline` [docs_help / notice]
  - Failed: Help content is Stale while offline. Why: The docs pack has not refreshed since {since_time} without a connection. Still works: Bundled help opens normally and shows the last Cached copy. Next: Refresh the docs pack (Open offline docs)
  - Chips: chip.cached, chip.stale

## Cross-surface chip reuse

- `chip.cached`: cli_help_summary, dynamic_banner, inline_blocker, project_doctor, screen_reader, screenshot_caption, support_export
- `chip.partial_index`: cli_help_summary, project_doctor, screenshot_caption, support_export
- `chip.policy_blocked`: cli_help_summary, inline_blocker, project_doctor, support_export
- `chip.reconnecting`: dynamic_banner, inline_blocker, screen_reader, support_export
- `chip.remote_host`: dynamic_banner, inline_blocker, screen_reader, support_export
- `chip.restricted`: inline_blocker, project_doctor, support_export
- `chip.rollback_available`: dynamic_banner, screenshot_caption, support_export
- `chip.stale`: cli_help_summary, dynamic_banner, inline_blocker, screen_reader, support_export
