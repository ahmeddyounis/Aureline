# Beta debugger and execution-context support matrix

This file is the publishable per-wedge support matrix for the beta debugger,
the launch / attach surfaces, the test runner, and the execution-context
resolver. Migration, partner, and release packets quote this matrix
verbatim instead of restating wedge claims in prose.

## Report metadata

- **Matrix id:** `support_matrix:m3.beta.debug_execution`
- **Schema version:** `1`
- **Milestone:** `m3`
- **Release channel scope:** `beta`
- **As of:** `2026-05-16`
- **Owner:** @ahmeddyounis

## Source contracts

- Canonical runtime manifest —
  [`/crates/aureline-runtime/src/support_matrix_beta/mod.rs`](../../../crates/aureline-runtime/src/support_matrix_beta/mod.rs)
- Cross-tool boundary schema —
  [`/schemas/runtime/support_matrix_beta.schema.json`](../../../schemas/runtime/support_matrix_beta.schema.json)
- Reviewer-facing landing page —
  [`/docs/runtime/m3/language_runtime_support_beta.md`](../../../docs/runtime/m3/language_runtime_support_beta.md)
- Input fixtures —
  [`/fixtures/runtime/m3/support_matrix_inputs/`](../../../fixtures/runtime/m3/support_matrix_inputs/)
- Integration test —
  [`/crates/aureline-runtime/tests/support_matrix_beta.rs`](../../../crates/aureline-runtime/tests/support_matrix_beta.rs)
- Shell consumer —
  [`/crates/aureline-shell/src/support_matrix_beta/mod.rs`](../../../crates/aureline-shell/src/support_matrix_beta/mod.rs)

## Consuming surfaces

- Migration packets quote this matrix to publish the supported wedge claim
  surface during partner intake and rollover review.
- Partner intake packets quote this matrix to fix expectations before the
  reference workspace dry-run.
- Release evidence packets quote this matrix to bind the release-bound
  wedge claim set to the runtime-emitted manifest record.

## Closed support-class vocabulary

| Class | Meaning | Protected dispatch on a fresh, in-sync context |
| --- | --- | --- |
| `supported` | Capability is exercised by the claimed beta surfaces, fixtures, and integration tests. | Allowed |
| `preview` | Capability is wired and inspectable but not yet exercised by claimed beta workflows. | Requires review |
| `limited` | Capability is narrowly available (inspection only, imported metadata only). | Requires review; mutating / privileged work is blocked |
| `unsupported` | Capability is explicitly out of beta scope. | Blocked |

Missing capabilities are represented as `preview` or `limited` rather than
implied parity. `supported` is only used when an integration test exercises
the claim end-to-end.

## Closed execution-context lane vocabulary

| Lane | Meaning |
| --- | --- |
| `local_host` | Local-host execution-context lane (terminal, task, test, debug, ai_tool_call). |
| `remote_attach` | Remote-attach lane (SSH, notebook-kernel-remote). |
| `container` | Devcontainer / container lane. |
| `request_workspace` | Managed-workspace / prebuild-runtime / ai-sandbox lane. |

## Closed downgrade-rule vocabulary

| Rule | Meaning |
| --- | --- |
| `narrow_launch_on_adapter_capability_drop` | Adapter accepted launch but dropped a requested capability; the dropped capability is recorded on the session snapshot. |
| `narrow_attach_to_inspect_only_on_capability_drop` | Adapter accepted attach but dropped a requested capability; the session narrows to inspect-only until the capability returns. |
| `block_on_unclaimed_test_framework` | Test framework outside the claimed coverage manifest; rows render as unclaimed. |
| `block_on_unclaimed_target_class` | Target class outside the claimed lane vocabulary; protected dispatch fails closed. |
| `block_protected_dispatch_on_ticket_drift` | Stored ticket disagrees with the freshly resolved execution context; dispatch must be re-authorised. |
| `block_protected_dispatch_on_capsule_drift` | Environment capsule advanced past the stored hash, or drift state regressed. |
| `block_protected_dispatch_on_trust_state_regression` | Trust regressed from `trusted` to `restricted` or `pending`. |
| `block_protected_dispatch_on_policy_epoch_regression` | Policy epoch on the stored ticket is older than the freshly resolved epoch. |
| `block_on_target_unreachable` | Target became unreachable; the row narrows to evidence-only. |
| `block_on_adapter_negotiation_refused` | Adapter initialisation timed out or negotiation refused a required capability. |

## Matrix

| Wedge | Launch | Attach | Test | Execution-context rollup | local_host | remote_attach | container | request_workspace |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `python` (Python service / data app) | `supported` (`dap_helper`) | `preview` (`dap_helper`) | `supported` (claimed: `pytest`) | `preview` | `supported` | `preview` | `supported` | `preview` |
| `typescript_javascript` (TypeScript / JavaScript web app or service) | `preview` (`dap_helper`) | `preview` (`dap_helper`) | `limited` (no claimed framework; previewed: `jest`, `vitest`, `node_test`) | `limited` | `supported` | `limited` | `preview` | `limited` |

## Per-wedge detail

### `python` — Python service / data app

- **Launch:** `supported`. Python debug-launch is wired through the beta
  DAP host. Adapter capability negotiation records both the agreed and
  dropped capability sets on the session snapshot.
- **Attach:** `preview`. Attach is reachable through the same DAP host but
  is not exercised by claimed beta workflows; rows render but protected
  attach dispatch requires review.
- **Test:** `supported`. Pytest discovery, run, rerun, tree, inline marker,
  and artifact identity are part of the claimed coverage manifest. Other
  Python test frameworks fall outside the manifest and render as unclaimed.
- **Execution context:**
  - `local_host`: `supported` — terminal, task, test, debug, and
    ai_tool_call surfaces resolve through the local_host lane.
  - `container`: `supported` — devcontainer / container lane resolves the
    same execution context, with the boundary cue visible.
  - `remote_attach`: `preview` — lane is wired and inspectable but Python
    remote debug attach is not exercised by claimed beta workflows.
  - `request_workspace`: `preview` — request-workspace lane resolves
    managed-workspace seeds; protected dispatch requires review per the
    trust contract.
- **Downgrade rules:** every canonical rule from the closed vocabulary
  applies. Capsule drift, ticket drift, trust regression, and policy-epoch
  regression all block protected dispatch.

### `typescript_javascript` — TypeScript / JavaScript web app or service

- **Launch:** `preview`. TS/JS debug-launch is reachable through the DAP
  host beta but is not exercised by claimed test or debug coverage. Rows
  render but protected launch dispatch requires review.
- **Attach:** `preview`. Like launch, attach is reachable through the same
  DAP host but is not exercised by claimed coverage and requires review
  before protected dispatch.
- **Test:** `limited`. No TS/JS framework is on the claimed beta coverage
  manifest yet. `jest`, `vitest`, and `node_test` render as previewed but
  unclaimed; the rerun-last command reports `unavailable` until a framework
  joins the manifest.
- **Execution context:**
  - `local_host`: `supported` — local host terminal, task, and
    execution-context resolution is the claimed daily-driver lane.
  - `container`: `preview` — devcontainer / container lane resolves the
    same execution context but is not exercised by claimed coverage.
  - `remote_attach`: `limited` — lane is reachable for inspection only;
    protected TS/JS dispatch fails closed.
  - `request_workspace`: `limited` — lane resolves managed-workspace seeds
    but cannot dispatch protected TS/JS work without a claimed framework.
- **Downgrade rules:** every canonical rule from the closed vocabulary
  applies. In addition to the drift / regression rules,
  `block_on_unclaimed_test_framework` is load-bearing for this wedge.

## How to refresh

The matrix is generated from the canonical
[`aureline_runtime::SupportMatrixBetaManifest`](../../../crates/aureline-runtime/src/support_matrix_beta/mod.rs)
record. To extend or narrow it:

1. Update the canonical manifest, the JSON schema, the reviewer doc, and
   the input fixtures together — none of those four are allowed to drift.
2. Re-run
   `cargo test -p aureline-runtime --test support_matrix_beta` and
   `cargo test -p aureline-shell --lib support_matrix_beta` so the
   fixture, integration, and shell-consumer tests all pass.
3. Edit this file to reflect the new closed vocabulary or the new wedge
   row. The matrix is read by partner and release packets; ad-hoc prose
   updates outside the closed vocabulary fail the contract.

## Out of scope

- M5 notebook-kernel launch / attach depth.
- Cross-workspace ticket import.
- Launch-language breadth outside the two claimed beta wedges (Java /
  Kotlin, Rust workspace, and Go service are held-for-later per the alpha
  scope matrix and remain out of this matrix).
- Full collaboration / multi-user productization of any column.
