# M5 CLI/headless structured output and result codes

This is the human-readable index of the **M5 CLI/headless structured-output and result-code catalog**. The machine-readable catalog at `artifacts/contracts/m5-cli-output-catalog.json` is authoritative; if the two disagree, the catalog wins and this document must be updated in the same change.

## What the catalog publishes

For every new M5 CLI/headless inspect, export, report, and health surface, the catalog publishes one surface row that binds:

- a **structured-output schema reference** resolved from the canonical M5 JSON Schema catalog (`schemas/public/m5-json/<family>.schema.json`),
- a **result-code catalog** — stable enums drawn from the CLI/headless machine-output stability contract, each with a pinned numeric code and a partial-result flag,
- a **lifecycle label** equal to the publication matrix's effective published label for the family,
- the **partial-result** and **freshness** vocabularies the surface can emit (`complete` / `partial` / `degraded` / `unavailable` / `stale_retest_needed`, and `fresh` / `stale` / `retest_needed` / `unknown`), and
- a **UI/CLI parity** declaration with a CLI fixture and a UI inspect fixture proving the lifecycle/degraded-state vocabulary is identical on both surfaces.

## Surfaces

| Surface | Kind | Family | Lifecycle | Envelope | Schema |
| --- | --- | --- | --- | --- | --- |
| `command_inspect` | inspect | command_descriptors | stable | `json_document_single` | `schemas/public/m5-json/command_descriptors.schema.json` |
| `support_bundle_export` | export | support_bundles_and_handoff | stable | `json_document_single` | `schemas/public/m5-json/support_bundles_and_handoff.schema.json` |
| `diagnostics_report` | report | diagnostic_records | beta | `sarif_2_1_0_document` | `schemas/public/m5-json/diagnostic_records.schema.json` |
| `project_doctor_health` | health | project_doctor_findings | beta | `json_document_single` | `schemas/public/m5-json/project_doctor_findings.schema.json` |
| `restore_provenance_inspect` | inspect | replay_and_trace_evidence | beta | `json_document_single` | `schemas/public/m5-json/replay_and_trace_evidence.schema.json` |
| `ai_evidence_export` | export | execution_context_provenance | stable | `ndjson_event_stream` | `schemas/public/m5-json/execution_context_provenance.schema.json` |
| `capability_qualification_inspect` | inspect | capability_records | stable | `json_document_single` | `schemas/public/m5-json/capability_records.schema.json` |
| `repair_transaction_report` | report | repair_transactions | beta | `json_document_single` | `schemas/public/m5-json/repair_transactions.schema.json` |
| `policy_config_inspect` | inspect | policy_bundles | beta | `json_document_single` | `schemas/public/m5-json/policy_bundles.schema.json` |

## Result-code catalog

Every result code is a member of the closed `exit_code_class` vocabulary frozen in `schemas/automation/cli_output_registry_entry.schema.json`, so a machine consumer keys off the stable enum, not the human text. The numeric code is pinned for shell-level consumers; `success` and `success_no_action_taken` are always `0`.

| Result code | Numeric | Partial-result carrier | Meaning |
| --- | --- | --- | --- |
| `success` | 0 | no | The surface completed and emitted a full structured result. |
| `success_no_action_taken` | 0 | no | The surface completed; nothing matched, so no rows were emitted. |
| `partial_success_with_warnings` | 10 | yes | Some rows resolved; a partial-result block lists what could not. |
| `usage_error` | 64 | no | The invocation was malformed; no structured result was produced. |
| `input_validation_error` | 65 | no | An argument failed validation; no structured result was produced. |
| `policy_or_trust_denied` | 77 | no | Admin policy or workspace trust denied the surface. |
| `credential_broker_denied` | 78 | no | A required credential handle was denied by the broker. |
| `preview_required_not_shown` | 73 | no | A required preview was not shown, so the surface refused to act. |
| `approval_required_not_granted` | 74 | no | A required approval was not granted, so the surface refused to act. |
| `dry_run_would_have_applied` | 75 | no | A dry run reported the change it would have applied. |
| `timeout_or_deadline_exceeded` | 124 | no | The surface hit a deadline; a partial or stale-retest result may be emitted. |
| `network_or_remote_unavailable` | 69 | no | A remote dependency was unavailable; local-only output is degraded. |
| `kill_switch_active` | 76 | no | A kill switch is active; the surface is disabled. |
| `dependency_missing_or_stale` | 72 | no | A required input was missing or stale; retest is needed. |
| `unsupported_on_headless` | 71 | no | The surface has no machine projection in this headless context. |
| `cancelled_by_user` | 130 | no | The invocation was cancelled before completion. |
| `unrecoverable_internal_error` | 70 | no | An internal error prevented a structured result. |

## Partial results and staleness

A surface that cannot fully resolve emits `partial_success_with_warnings` with a `partial_result_state` of `partial` or `degraded`; a surface whose inputs are stale emits a `freshness_state` of `stale` or `retest_needed` so automation never mistakes a stale cache for a fresh result. These two vocabularies are closed and stable and are shared field-for-field with the matching UI inspect surface.

## UI/CLI parity

Each surface ships a CLI fixture and a UI inspect fixture under `fixtures/contracts/m5-cli-json/`. The validator proves the two carry an identical `partial_result_state`, `freshness_state`, and `lifecycle_label`, so the desktop inspect surface and the CLI/headless output never diverge on the lifecycle or degraded-state vocabulary.

| Surface | UI inspect surface | Match mode |
| --- | --- | --- |
| `command_inspect` | `command_palette.command_inspector` | exact_match_required |
| `support_bundle_export` | `support_center.bundle_inspector` | exact_match_required |
| `diagnostics_report` | `editor.problems_inspector` | projection_match_required |
| `project_doctor_health` | `support_center.doctor_panel` | exact_match_required |
| `restore_provenance_inspect` | `recovery.restore_provenance_view` | exact_match_required |
| `ai_evidence_export` | `ai.evidence_inspector` | exact_match_required |
| `capability_qualification_inspect` | `about.capability_inventory_panel` | exact_match_required |
| `repair_transaction_report` | `support_center.repair_ledger` | exact_match_required |
| `policy_config_inspect` | `admin.policy_inspector` | exact_match_required |

## Offline and mirror use

The catalog, its boundary schema, the per-surface parity fixtures, and the validator bundle into offline/mirror artifact sets and validate without runtime service access (`offline_bundle.requires_runtime_service` is `false`).

## Freshness

The catalog is current as of `2026-06-19`. CI regenerates it from `tools/regenerate_m5_cli_output_catalog.py`, runs `tools/validate_m5_cli_output_catalog.py`, and runs the typed Rust consumer's tests, so the published surfaces cannot drift from the catalog.
