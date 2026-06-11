# M5 depth-import migration & compatibility report

Generated from the seeded report in
[`crate::m5_depth_imports`](../../../../crates/aureline-shell/src/m5_depth_imports/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_depth_imports -- markdown > \
  artifacts/compat/m5/migration-reports/m5_depth_import_report.md
```

- Report id: `migration:m5_depth_import:v1:default`
- Rows: 6
- Artifact classes covered: 6/6
- Preview before apply: true
- Every apply reversible: true
- No overclaimed parity: true
- No raw artifact content: true
- Generated at: `2026-06-11T00:00:00Z`

## Outcome summary

| Outcome | Count |
|---|---:|
| imported | 1 |
| mapped | 2 |
| skipped | 0 |
| manual_review | 1 |
| bridge_required | 1 |
| unsupported | 1 |
| **total** | **6** |

## Artifact-class coverage

| Artifact class | Outcome | Fidelity | Continuity scope | Reversible |
|---|---|---|---|:---:|
| Notebook handoff | `imported` | `translated` | `native` | yes |
| Request / schema bundle | `mapped` | `partial` | `partial_mapping` | yes |
| Database query / session export | `mapped` | `partial` | `partial_mapping` | yes |
| Profiler / trace capture | `manual_review` | `partial` | `inspect_only` | yes |
| Template / scaffold manifest | `bridge_required` | `shimmed` | `bridge` | yes |
| Companion / export packet | `unsupported` | `unsupported` | `export_bundle` | yes |

## Notebook handoff import (`notebook_handoff`)

Notebook cells, outputs, and attachment refs translate into a native imported document. The apply is checkpointed and the imported document can be restored if the result is not what the user expected.

- Outcome: `imported`
- Continuity scope: `native`
- Restore path: Restore from the pre-apply notebook checkpoint to revert the imported document.
- Known deviations:
  - `deviation:notebook.kernel_state` — Live kernel state and execution counts are not part of the handoff; the imported document opens without a running kernel. (recoverable: true)

## Request and schema bundle import (`request_schema_bundle`)

Requests and schemas map onto a native request workspace, but the mapping is partial: scripts and secrets are intentionally excluded and disclosed as known deviations rather than implied to come across.

- Outcome: `mapped`
- Continuity scope: `partial_mapping`
- Compatibility note: Saved requests and the API schema map into a request workspace; environment secrets and pre-request scripts are not imported and must be re-entered.
- Restore path: Restore from the pre-apply request-workspace checkpoint to remove the mapped collection.
- Known deviations:
  - `deviation:request.secret_scripts` — Environment secrets and pre-request scripting do not cross the boundary. (recoverable: false)

## Database query and session export (`database_query_session_export`)

Query text and session metadata map into a native query library. Credentials are excluded by design, so the row stays a disclosed partial mapping rather than a parity claim.

- Outcome: `mapped`
- Continuity scope: `partial_mapping`
- Compatibility note: Saved queries and session metadata map into a query library; live connection credentials are never imported and must be reconnected explicitly.
- Restore path: Restore from the pre-apply data-workspace checkpoint to remove the imported query library.
- Known deviations:
  - `deviation:database.live_connection` — Live connection credentials and open result-set state do not cross the boundary. (recoverable: true)

## Profiler or trace capture import (`profiler_trace_capture`)

A captured trace is opened inspect-only for review. Because nothing durable is written, there is no checkpoint to restore; the row is flagged for manual review so the user decides whether to keep the inspected capture.

- Outcome: `manual_review`
- Continuity scope: `inspect_only`
- Compatibility note: Captured traces open in an inspect-only viewer; Aureline does not re-run or re-symbolicate the capture, so the import does not mutate durable state.
- Known deviations:
  - `deviation:profiler.resymbolication` — Re-symbolication and re-capture are not performed; symbols already present in the capture are shown as-is. (recoverable: false)

## Signed template or scaffold manifest import (`template_scaffold_manifest`)

A signed scaffold manifest is honored through a verified compatibility bridge. The bridge posture is disclosed so the row is never presented as native scaffold parity.

- Outcome: `bridge_required`
- Continuity scope: `bridge`
- Compatibility note: Signed scaffold manifests run through a compatibility bridge rather than as a native scaffold; the bridge must be present before the scaffold can execute.
- Bridge: `bridge:scaffold-manifest-runner` (required_before_use) — Scaffold execution runs through the compatibility bridge; the manifest signature is verified before any scaffold action is offered.
- Known deviations:
  - `deviation:template.native_scaffold` — There is no native scaffold engine for this manifest format; continuity depends on the bridge remaining available. (recoverable: true)

## Companion or export packet import (`companion_export_packet`)

Companion export packets have no native import target, so the migration center keeps them as an export bundle and says so plainly instead of implying a live companion import.

- Outcome: `unsupported`
- Continuity scope: `export_bundle`
- Compatibility note: Companion export packets are preserved as a re-exportable bundle for offline review; Aureline does not import them as a live companion session, so the bundle is the supported continuity path.
- Known deviations:
  - `deviation:companion.live_session` — Live companion device sessions are not reconstructed; only the export bundle is retained. (recoverable: false)

