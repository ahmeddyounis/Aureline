# Ecosystem compatibility scorecards and reference-workspace linkage

## Scope

This packet defines one canonical compatibility scorecard row for stable-facing
ecosystem claims and proves that marketplace cards, migration reports,
bridge-detail views, bundle-detail views, support exports, and release claim
manifests can project directly from that row.

## Canonical fields

- parity band: `stable`, `limited`, `preview`, `retest_pending`, `unsupported`
- freshness: `current`, `aging`, `stale`, `expired`, `missing`
- evidence source: `reference_workspace_report`, `archetype_certification`, `bridge_matrix`, `conformance_suite`, `migration_fixture`
- linkage: deployment-profile refs, runtime-profile refs, reference-workspace ids, and reference-workspace lineage refs
- downgrade inputs: bridge parity, evidence freshness, reference-workspace certification state, known-gap state

## Guardrails

- Stable claimed rows must cite reference-workspace ids and lineage refs.
- Consumer projections must preserve the exact parity band, freshness state,
  evidence source, and reference-workspace linkage of the canonical row.
- Stale evidence narrows to `retest_pending`.
- Narrowed bridge parity narrows to `limited`.
- Expired reference-workspace certification narrows to `retest_pending`.

## Canonical fixtures

- `stable-current.json`: all three rows remain `stable`.
- `stale-evidence.json`: imported Python workflow narrows to `retest_pending`.
- `bridge-narrowed.json`: the VS Code webview bridge narrows to `limited`.
- `certification-expired.json`: the Python launch bundle narrows to `retest_pending`.

## Reference row anchors

- Python import path: `ecosystem_scorecard_row:ms_python_import`
- Bridge detail: `ecosystem_scorecard_row:vscode_webview_bridge`
- Bundle inheritance: `ecosystem_scorecard_row:python_launch_bundle`
