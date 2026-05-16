# Extension compatibility report for beta ecosystem lanes

This report is the reviewer-facing projection of
[`bridge_matrix.yaml`](./bridge_matrix.yaml). Marketplace rows, SDK
docs, publication packets, and release notes cite the matrix row ids
below instead of restating compatibility claims locally.

## Report metadata

- **Report id:** `extension_compatibility_report:m3.beta`
- **Matrix id:** `extension_bridge_matrix:m3.beta`
- **Matrix revision:** `1`
- **Release channel scope:** `beta`
- **As of:** `2026-05-16`
- **Generated at:** `2026-05-16T20:00:00Z`
- **Source compatibility row:** `compat_row:extension_host.sdk_wit_permission_window`
- **Canonical matrix:** [`artifacts/compat/m3/bridge_matrix.yaml`](./bridge_matrix.yaml)
- **Docs projection:** [`docs/extensions/m3/compatibility_matrix_beta.md`](../../../docs/extensions/m3/compatibility_matrix_beta.md)

## Compatibility rows

| Lane | Bridge state | Compatibility label | Support | Runtime window | SDK window | Manifest window | Downgrade behavior |
|---|---|---|---|---|---|---|---|
| `wasm_component_native` | `native` | `exact` | `experimental` | `runtime_window:extension_host_v1_beta.wasm_component` | `sdk_window:aureline_sdk_beta.wasm_component` | `manifest_window:extension_manifest_beta.v1` | `downgrade_unsupported` / `explicitly_unsupported` |
| `external_host_supervised` | `native` | `exact` | `experimental` | `runtime_window:extension_host_v1_beta.external_host` | `sdk_window:aureline_sdk_beta.external_host` | `manifest_window:extension_manifest_beta.external_host` | `downgrade_unsupported` / `explicitly_unsupported` |
| `vscode_selected_api_bridge` | `bridge` | `translated` | `limited` | `runtime_window:extension_host_v1_beta.compat_bridge` | `sdk_window:aureline_sdk_beta.compat_bridge` | `manifest_window:extension_manifest_beta.compat_bridge` | `downgrade_best_effort` / `degraded` |
| `vscode_theme_snippet_shim` | `shimmed` | `shimmed` | `limited` | `runtime_window:asset_shim_beta.theme_snippet` | `sdk_window:aureline_sdk_beta.asset_shim` | `manifest_window:extension_manifest_beta.asset_shim` | `downgrade_best_effort` / `degraded` |
| `vscode_webview_or_private_workbench_runtime` | `unsupported` | `unsupported` | `unsupported` | `runtime_window:unsupported_foreign_webview_runtime` | `sdk_window:unsupported_foreign_webview_runtime` | `manifest_window:unsupported_foreign_webview_runtime` | `downgrade_unsupported` / `explicitly_unsupported` |

## Per-row detail

### `extension_bridge_row:wasm_component_native_beta`

- **Claimed lane:** `wasm_component_native`
- **Bridge posture:** `native`, `exact` only for the declared WIT worlds and target platforms.
- **Runtime window:** host runtime `1.0.0-beta.1..1.0.x`; runtime, capability worlds, and permission vocabulary must all be in range.
- **SDK window:** `aureline.sdk.beta 1.0.0-beta.1..1.0.x`; requires typed docs, sample-pack coverage, and conformance results.
- **Manifest window:** manifest schema `1` with permission vocabulary `1`; breaking field or permission drift fails closed.
- **Downgrade behavior:** disable or quarantine the extension when host floor, WIT world, SDK floor, or permission vocabulary is outside the window. Installed state remains present but activation is refused.
- **Evidence:** `artifacts/compat/m3/extension_conformance_kit_report.json`, `docs/extensions/m3/runtime_v1_beta.md`, `docs/extensions/m3/sdk_v1/README.md`.

### `extension_bridge_row:external_host_supervised_beta`

- **Claimed lane:** `external_host_supervised`
- **Bridge posture:** `native`, but only through the supervised external-host envelope.
- **Runtime window:** host runtime `1.0.0-beta.1..1.0.x` with supervisor envelope `1`; executable identity, platform support, restart budget, and capability worlds must match.
- **SDK window:** `aureline.sdk.beta 1.0.0-beta.1..1.0.x`; external-host contract refs and supervision evidence are required.
- **Manifest window:** manifest schema `1` with executable disclosure and permission vocabulary `1`.
- **Downgrade behavior:** stop, restart, disable, or quarantine the external host before widening shell authority. User-authored state stays visible.
- **Evidence:** `docs/extensions/m3/host_isolation_beta.md`, `fixtures/extensions/m3/conformance_kit/external_host_degraded_disable_rollback_pass.json`.

### `extension_bridge_row:vscode_api_bridge_beta`

- **Claimed lane:** `vscode_selected_api_bridge`
- **Bridge posture:** `bridge`, `translated`, `bridge_no_exact_parity`.
- **Runtime window:** bridge profile `2026-05` for host runtime `1.0.0-beta.1`; only the declared capability-world subset is negotiated.
- **SDK window:** compatibility analyzer profile `2026-05` with SDK `1.0.0-beta.1`; translated, skipped, and unsupported APIs must be named before publication.
- **Manifest window:** manifest schema `1` plus bridge profile ref, caveat labels, permission delta, performance delta, and rollback path.
- **Downgrade behavior:** bridge activation narrows or pauses when the analyzer profile is stale. Imported settings and user data remain readable.
- **Known limits:** no workbench DOM injection, no undocumented private API compatibility, and no ambient Node.js privilege inside the shell.
- **Evidence:** `fixtures/extensions/marketplace_discovery_cases/bridge_state_compatibility_bridge_required.yaml`, `docs/extensions/marketplace_ranking_and_trust_contract.md`.

### `extension_bridge_row:vscode_theme_snippet_shim_beta`

- **Claimed lane:** `vscode_theme_snippet_shim`
- **Bridge posture:** `shimmed`, `shimmed`, `shimmed_no_exact_parity`.
- **Runtime window:** asset shim profile `2026-05` with host runtime `1.0.0-beta.1`; static assets can remain readable without runtime activation.
- **SDK window:** compatibility analyzer profile `2026-05` with SDK `1.0.0-beta.1`.
- **Manifest window:** manifest schema `1` with imported-source provenance; lossy transforms must be reviewable.
- **Downgrade behavior:** imported assets remain inspectable and reversible; unsupported runtime behavior stays disabled.
- **Known limits:** token families outside the Aureline design system are approximated, extension-contributed commands are not carried, and runtime hooks or webviews are not executed.
- **Evidence:** `fixtures/design/token_export_cases/extension_partial_high_contrast_inheritance_pass.json`, `docs/migration/source_ecosystem_coverage_matrix.md`.

### `extension_bridge_row:unsupported_webview_runtime`

- **Claimed lane:** `vscode_webview_or_private_workbench_runtime`
- **Bridge posture:** `unsupported`, `unsupported`, `unsupported_no_parity`.
- **Runtime window:** no supported beta runtime window.
- **SDK window:** no SDK line; authors must port to native Aureline APIs.
- **Manifest window:** no admissible manifest window for private workbench runtime.
- **Downgrade behavior:** registry ingest, install, publication, and activation are refused. Imported metadata may remain visible for review.
- **Known limits:** full VS Code webview API parity, workbench DOM injection, and undocumented private APIs are not claimed.
- **Evidence:** `docs/extensions/marketplace_ranking_and_trust_contract.md`, `fixtures/contracts/reference_examples/migration/import_preview_partial_missing_extension.yaml`.

## Consumer contract

- Marketplace rows must cite `extension_bridge_matrix_id`,
  `extension_bridge_matrix_row_id`, bridge state, window ids, and bridge
  known limits before opening install review.
- SDK docs must link the same row ids when describing native, bridge,
  shimmed, or unsupported paths.
- Publication and support packets must carry both the extension
  compatibility report ref and the bridge-matrix row ref.
- Release notes must cite this matrix whenever they mention extension
  runtime, SDK, manifest, or bridge support.
