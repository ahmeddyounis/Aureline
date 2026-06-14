# M5 Session Plans And Attempt-Record Ledger

- Packet: `session-attempt-ledger:stable:0001`
- Label: `M5 Session Plans And Attempt-Record Ledger`
- Sessions: 4 across 4 / 4 flows
- Attempts: 6 (1 imported, 2 derive from a predecessor)

## Sessions

- **session:local:checkout** (local_workspace / rerun_failed): retry `retry_failed_up_to_limit`, watch `watch_disabled`
  - Local rerun-failed of the checkout suite
  - selection `selection:cli:rerun-failed` snapshot `snapshot:framework-pack:checkout` (4 targets)
  - lineage: runtime `runtime:cpython-3.12` toolchain `toolchain:pytest-8` env `env:workspace-venv` (local_authoritative)
  - attempt #1 `attempt:local:checkout:1` [initial] → `failed`
  - attempt #2 `attempt:local:checkout:2` [rerun_failed] → `passed` (from `attempt:local:checkout:1`)
- **session:remote:integration** (remote_target / run_selected): retry `no_retry`, watch `watch_debounced`
  - Remote integration run on the shared builder
  - selection `selection:ui:integration` snapshot `snapshot:framework-pack:integration` (1 targets)
  - lineage: runtime `runtime:cpython-3.12` toolchain `toolchain:pytest-8` env `env:remote-builder` (remote_authoritative)
  - attempt #1 `attempt:remote:integration:1` [initial] → `passed`
- **session:notebook:analysis** (notebook_kernel / run_selected): retry `no_retry`, watch `watch_disabled`
  - Notebook-linked analysis test run
  - selection `selection:ui:notebook` snapshot `snapshot:notebook:analysis` (1 targets)
  - lineage: runtime `runtime:ipykernel-6` toolchain `toolchain:nbval` env `env:notebook-kernel` (notebook_authoritative)
  - attempt #1 `attempt:notebook:analysis:1` [initial] → `passed`
- **session:imported:smoke** (imported_provider / import_provider_join): retry `imported_no_retry`, watch `imported_not_watchable`
  - Imported CI smoke evidence joined for triage
  - selection `selection:support:imported-ci` snapshot `snapshot:imported-ci:smoke` (1 targets)
  - lineage: runtime `runtime:provider-reported` toolchain `toolchain:provider-reported` env `env:provider-reported` (imported_read_only)
  - attempt #1 `attempt:imported:smoke:1` [imported_join] → `imported`
  - attempt #2 `attempt:imported:smoke:2` [local_parity_rerun] → `failed` (from `attempt:imported:smoke:1`)
