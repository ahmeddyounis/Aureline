# SDK conformance packet for `aureline.sdk.beta` @ `1.0.0-beta.1`

- **Packet id:** `sdk_conformance_packet:aureline.sdk.beta-1.0.0-beta.1`
- **Release channel scope:** `beta`
- **As of:** `2026-05-19`
- **Generated at:** `2026-05-19T20:00:00Z`
- **Decision class:** `ready_for_authors`
- **Reason class:** `all_claimed_surfaces_available_in_beta`
- **Validator report:** [`artifacts/compat/m3/extension_conformance_kit_report.json`](../../../artifacts/compat/m3/extension_conformance_kit_report.json)
- **Sample-pack record:** [`fixtures/extensions/m3/sample_pack/ready_for_authors_wasm_and_external_host.json`](../../../fixtures/extensions/m3/sample_pack/ready_for_authors_wasm_and_external_host.json)
- **Lifecycle metadata packet:** [`artifacts/extensions/m3/lifecycle_metadata_packet.json`](../../../artifacts/extensions/m3/lifecycle_metadata_packet.json)
- **Bridge matrix:** [`artifacts/compat/m3/bridge_matrix.yaml`](../../../artifacts/compat/m3/bridge_matrix.yaml)
- **Bridge-compatibility scorecard:** [`artifacts/extensions/m3/bridge_compatibility_scorecard.json`](../../../artifacts/extensions/m3/bridge_compatibility_scorecard.json)

## Validator suite summary

- Result: `pass` (5/5 cases matched expectations, 3 aggregate blockers)
- Required scenarios: `activation`, `degraded_path`, `disable_rollback`, `install`, `permission_prompt`
- Observed scenarios: `activation`, `degraded_path`, `disable_rollback`, `install`, `permission_prompt`

## Sample-pack summary

- Starter pack: `sdk_v1_starter_pack:aureline.sdk.beta-1.0.0`
- Decision: `ready_for_authors` / reason `all_claimed_surfaces_available_in_beta`
- Surfaces: 2 claimed, 2 available, 0 preview
- Samples: 1 wasm runnable, 1 external-host runnable
- Authoring guides: 4

## Lifecycle metadata summary

- Packet: `extension_lifecycle_metadata_packet:aureline.sdk.beta`
- Policy: [`docs/extensions/m3/sdk_versioning_and_deprecation.md`](../../../docs/extensions/m3/sdk_versioning_and_deprecation.md)
- Rows: 7 total, 7 governed beta/stable, 1 deprecated
- Decision: `ready_for_authors` / reason `all_rows_governed`

## Bridge-compatibility scorecard

| Lane | State | Scorecard | Native | Bridge |
|---|---|---|---|---|
| `wasm_component_native` | `native` | `native_green` | `native_supported` | `bridge_not_applicable` |
| `external_host_supervised` | `native` | `native_green` | `native_supported` | `bridge_not_applicable` |
| `vscode_selected_api_bridge` | `bridge` | `bridge_amber` | `native_not_applicable` | `bridge_translated_with_caveats` |
| `vscode_theme_snippet_shim` | `shimmed` | `shimmed_amber` | `native_not_applicable` | `bridge_shimmed_static_only` |
| `vscode_webview_or_private_workbench_runtime` | `unsupported` | `unsupported_red` | `native_unsupported` | `bridge_unsupported` |

### Lane caveats and non-green reasons

- `extension_bridge_row:vscode_api_bridge_beta`
  - Non-green reasons: `bridge_translated_subset_not_native_parity`
  - Known limits:
    - No workbench DOM injection.
    - No undocumented private API compatibility.
    - No ambient Node.js privilege inside the shell.
- `extension_bridge_row:vscode_theme_snippet_shim_beta`
  - Non-green reasons: `static_asset_shim_no_runtime_parity`
  - Known limits:
    - Token families outside the Aureline design system are approximated.
    - Extension-contributed commands are not carried by the asset shim.
    - Runtime hooks and webviews are not executed.
- `extension_bridge_row:unsupported_webview_runtime`
  - Non-green reasons: `foreign_runtime_unsupported_in_beta_lane`
  - Known limits:
    - Full VS Code webview API parity is not claimed.
    - Workbench DOM injection is unsupported.
    - Undocumented private APIs are unsupported.

## Docs freshness findings

| Doc | Token | Check | Status |
|---|---|---|---|
| `docs/extensions/m3/sdk_v1/README.md` | `artifacts/extensions/m3/lifecycle_metadata_packet.json` | `lifecycle_metadata_packet_ref` | `cite_present` |
| `docs/extensions/m3/sdk_v1/README.md` | `sdk_versioning_and_deprecation.md` | `versioning_policy_ref` | `cite_present` |
| `docs/extensions/m3/conformance_kit_beta.md` | `artifacts/compat/m3/extension_conformance_kit_report.json` | `conformance_kit_report_ref` | `cite_present` |
| `docs/extensions/m3/conformance_kit_beta.md` | `artifacts/extensions/m3/lifecycle_metadata_packet.json` | `lifecycle_metadata_packet_ref` | `cite_present` |
| `docs/extensions/m3/compatibility_matrix_beta.md` | `artifacts/compat/m3/bridge_matrix.yaml` | `bridge_matrix_path` | `cite_present` |
| `docs/extensions/m3/compatibility_matrix_beta.md` | `artifacts/extensions/m3/lifecycle_metadata_packet.json` | `lifecycle_metadata_packet_ref` | `cite_present` |
| `docs/extensions/m3/sdk_versioning_and_deprecation.md` | `lifecycle_metadata_packet.json` | `versioning_policy_ref` | `cite_present` |
| `docs/extensions/m3/sdk_conformance_beta.md` | `sdk_conformance_packet:aureline.sdk.beta-1.0.0-beta.1` | `sample_pack_starter_pack_ref` | `cite_present` |
| `docs/extensions/m3/sdk_conformance_beta.md` | `extension_bridge_matrix:m3.beta` | `bridge_matrix_id` | `cite_present` |
| `docs/extensions/m3/sdk_conformance_beta.md` | `artifacts/extensions/m3/sdk_conformance_packet.json` | `conformance_kit_report_ref` | `cite_present` |

## Known caveats

- Bridge and shimmed lanes are not parity claims; downgrade is governed by the bridge matrix.
- Compatibility-bridge surfaces are preview-only on this line and never inherit ambient Node.js privilege.
- Sample-pack rows cite manifest baseline, permission manifest, runtime contract, and SDK release-bundle refs; raw bytes never appear in the packet.

## Consuming surfaces

- `docs/extensions/m3/sdk_conformance_beta.md`
- `docs/extensions/m3/sdk_v1/README.md`
- `docs/extensions/m3/compatibility_matrix_beta.md`
- `artifacts/release/m3/release_notes_draft.md`
- `artifacts/extensions/m3/publication_pipeline/publication_pipeline_record.json`
- `artifacts/extensions/m3/publication_pipeline/publication_support_export.json`
