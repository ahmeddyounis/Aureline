# Extension compatibility matrix beta

This page is the author-facing projection of
[`artifacts/compat/m3/bridge_matrix.yaml`](../../../artifacts/compat/m3/bridge_matrix.yaml).
Use the row ids here in issue reports, marketplace reviews, publication
packets, and SDK support questions.

Support windows and deprecation posture come from
[`artifacts/extensions/m3/lifecycle_metadata_packet.json`](../../../artifacts/extensions/m3/lifecycle_metadata_packet.json)
under the policy in
[`sdk_versioning_and_deprecation.md`](./sdk_versioning_and_deprecation.md).
Each compatibility row cites both an `extension_bridge_row:*` id and
the `lifecycle_row:*` ids that govern its SDK, manifest, WIT,
permission, or bridge surface.

## Summary

| Lane | Row id | State | What is supported |
|---|---|---|---|
| Wasm component native | `extension_bridge_row:wasm_component_native_beta` | `native` / `exact` | Declared WIT worlds and target platforms inside the beta SDK and manifest window. |
| External host supervised | `extension_bridge_row:external_host_supervised_beta` | `native` / `exact` | Supervised external processes with declared executable identity, platform support, restart budget, and capability worlds. |
| VS Code selected API bridge | `extension_bridge_row:vscode_api_bridge_beta` | `bridge` / `translated` | Selected settings, command mappings, and read-only editor API subsets through a named bridge profile. |
| VS Code theme and snippet shim | `extension_bridge_row:vscode_theme_snippet_shim_beta` | `shimmed` / `shimmed` | Static theme, icon-theme, and snippet assets after compatibility-analyzer mapping. |
| VS Code webview and private runtime | `extension_bridge_row:unsupported_webview_runtime` | `unsupported` / `unsupported` | No beta support. Port to native Aureline APIs. |

## Runtime, SDK, And Manifest Windows

Each row names four windows:

| Window | Required meaning |
|---|---|
| Runtime window | Host runtime or supervisor version, capability-world set, isolation profile, and out-of-window activation posture. |
| SDK window | SDK line, typed API availability, WIT or external-host contract coverage, and conformance requirement. |
| Manifest window | Manifest schema, permission vocabulary, required disclosures, and unknown-field preservation rule. |
| Bridge window | Native, bridge, shimmed, partial, or unsupported state plus parity posture, caveats, permission delta, and performance delta. |
| Lifecycle rows | Support window, lifecycle state, deprecation posture, replacement path, and removal target for the governed surface. |

The beta extension row uses the canonical compatibility row
`compat_row:extension_host.sdk_wit_permission_window`. If a package is
outside the named runtime, SDK, manifest, or bridge window, the host
disables, quarantines, narrows, or refuses the row according to the
matrix. It does not silently load ambiguous extension behavior.

## Native versus compatibility-bridge scorecard

Release notes, marketplace surfaces, and support packets cite the
per-lane native-runtime versus compatibility-bridge scorecard at
[`artifacts/extensions/m3/bridge_compatibility_scorecard.json`](../../../artifacts/extensions/m3/bridge_compatibility_scorecard.json)
so native and bridge rows cannot share one undifferentiated green
label. The scorecard is projected from this matrix by the SDK
conformance packet generator described in
[`sdk_conformance_beta.md`](./sdk_conformance_beta.md).

## Bridge Honesty

Bridge and shimmed rows are not parity claims.

- `bridge` means a governed compatibility profile translates a declared
  subset and must show caveats before install or update.
- `shimmed` means an asset or behavior is approximated with known
  limits and reversible provenance.
- `partial` is reserved for rows whose supported subset is smaller than
  the declared source artifact.
- `unsupported` is an explicit row, not an omitted future promise.

The matrix forbids `exact` labels on bridge, shimmed, partial, or
unsupported rows. Marketplace, install review, support exports, SDK
docs, and release packets must carry the matrix row id wherever they
surface a bridge or shim.

## Downgrade Behavior

| Row id | Downgrade support | Out-of-window posture | User-state behavior |
|---|---|---|---|
| `extension_bridge_row:wasm_component_native_beta` | `downgrade_unsupported` | `explicitly_unsupported` | Installed state remains; activation is disabled or quarantined. |
| `extension_bridge_row:external_host_supervised_beta` | `downgrade_unsupported` | `explicitly_unsupported` | External host stops or quarantines before shell authority widens. |
| `extension_bridge_row:vscode_api_bridge_beta` | `downgrade_best_effort` | `degraded` | Imported settings and data remain readable; bridge activation narrows or pauses. |
| `extension_bridge_row:vscode_theme_snippet_shim_beta` | `downgrade_best_effort` | `degraded` | Imported assets remain inspectable and reversible. |
| `extension_bridge_row:unsupported_webview_runtime` | `downgrade_unsupported` | `explicitly_unsupported` | Metadata may remain visible for review; install, publication, and activation are refused. |

## Consumer Checklist

- Marketplace rows cite `extension_bridge_matrix:m3.beta` and a concrete
  `extension_bridge_row:*` value plus the lifecycle row refs from the
  matrix.
- SDK docs link the same row ids when explaining native, bridge,
  shimmed, or unsupported author paths.
- Publication packets carry `bridge_matrix_ref` and
  `bridge_matrix_row_ref` in compatibility metadata together with
  `lifecycle_metadata_ref` and `deprecation_packet_template_ref`.
- Release notes cite this matrix before mentioning beta extension
  runtime, SDK, manifest, or bridge support.
