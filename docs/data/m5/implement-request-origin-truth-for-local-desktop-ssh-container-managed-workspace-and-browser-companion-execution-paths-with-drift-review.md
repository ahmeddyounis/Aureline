# Request-origin truth and rerun drift review

## Scope

This document describes the request-origin truth and rerun drift-review records
that make execution origin a first-class fact everywhere a request can run.
Localhost, container service names, and private DNS mean different things across
the local-desktop, SSH, container, managed-workspace, and browser-companion
execution paths, so each resolved-origin row keeps its execution path, canonical
origin lane, opaque target identity, trust boundary, and drift state inspectable.
The companion rerun-review sheets distinguish *rerun exactly* from *rerun with
current context* and enumerate every origin change before dispatch so a saved
request or rerun never silently retargets through a different host, lane, or
trust boundary than before.

The records reuse the canonical matrix vocabulary (`request_origin_kind`,
`request_origin_drift_state`, `retention_mode`) and reference the frozen
API-collection matrix as a verified upstream packet rather than minting a local
synonym set. The finer `origin_execution_path` adds the explicit SSH and
local-desktop distinction this lane requires while mapping one-to-one onto the
frozen origin lanes.

## Truth sources

- Implementation: `crates/aureline-api/src/implement_request_origin_truth_for_local_desktop_ssh_container_managed_workspace_and_browser_companion_execution_paths_with_drift_review/mod.rs`
- Schema: `schemas/data/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.schema.json`
- Checked-in packet: `artifacts/data/m5/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.json`
- Fixtures: `fixtures/data/m5/implement_request_origin_truth_for_local_desktop_ssh_container_managed_workspace_and_browser_companion_execution_paths_with_drift_review/`
- Upstream matrix: `artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`

## Locked vocabulary

| Term | Family | Meaning |
|---|---|---|
| `local_desktop`, `ssh`, `container`, `managed_workspace`, `browser_companion` | execution path | The five lanes a request can resolve through; each maps one-to-one onto a canonical origin lane. |
| `local_host`, `remote`, `container`, `managed`, `browser_companion` | origin kind | The frozen matrix origin lanes the execution paths resolve under. |
| `desktop_local_trust`, `remote_host_trust`, `container_scoped_trust`, `managed_tenant_trust`, `browser_companion_trust` | trust boundary | Trust scope per lane; only the local-desktop path may claim desktop-local trust. |
| `origin_stable`, `origin_changed` | drift state | Whether the origin resolves to the same target as the last run. |
| `rerun_exactly`, `rerun_with_current_context` | rerun mode | Re-dispatch the exact recorded origin, or re-resolve through the current environment (which can drift). |
| `host_identity_changed`, `origin_lane_changed`, `trust_boundary_changed`, `port_or_service_changed`, `private_dns_rebound` | origin change | The enumerated changes a rerun review lists before dispatch. |

## Consumer surfaces

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Request composer origin chip | stable | stable | The composer shows origin class and target identity before send, so localhost never silently resolves to a remote or container target. |
| Rerun review sheet | stable | stable | The sheet distinguishes rerun-exactly from rerun-with-current-context and enumerates every origin change before dispatch. |
| Request list origin column | stable | stable | Each saved request shows its origin class and an origin-changed warning when the resolved target drifted. |
| Browser companion origin banner | stable | stable | Companion requests show origin and trust boundary and never inherit desktop-local trust or naming. |
| CLI and headless origin line | stable | stable | Headless runs print origin truth and refuse to dispatch a drifted rerun-with-current-context without acknowledgement. |
| Support export origin truth | stable | stable | Exports carry origin class, target identity, trust boundary, and enumerated changes with metadata-only retention. |
| Help and About origin contract | stable | stable | Help/About describe the five paths, trust boundaries, rerun modes, and the origin-changed review contract. |

## Origin and drift-review rules

- An origin's `execution_path` always resolves under its canonical `origin_kind`
  lane and `trust_boundary`; the finer path never diverges from the frozen
  matrix lanes.
- Only the local-desktop path may inherit desktop-local trust. SSH, container,
  managed-workspace, and browser-companion origins never inherit desktop-local
  trust or naming.
- Every origin keeps an explicit, named target identity and blocks silent
  retargeting on reopen or rerun behind an acknowledgement.
- A `origin_changed` origin always surfaces an origin-changed warning and is
  reviewed through a rerun-with-current-context sheet that blocks dispatch until
  the enumerated changes are acknowledged.
- `rerun_exactly` re-dispatches the exact recorded origin and snapshot, so its
  resolved origin equals its prior origin and never drifts.
- `rerun_with_current_context` re-resolves the origin through the current
  environment; when it drifts, the sheet enumerates each origin change before
  dispatch.
- Rerun and compare UX never widens request-history retention toward unsafe body
  or header capture; history stays metadata-only or redacted-replayable.
- The origin truth references the frozen API-collection matrix as a verified
  upstream packet; the matrix remains the source of origin-lane, drift, and
  trust-isolation truth.
