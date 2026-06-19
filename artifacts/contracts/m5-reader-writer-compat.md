# M5 reader/writer compatibility report

This is the operator-facing summary of the **M5 reader/writer compatibility suite**. The machine-readable suite at `artifacts/contracts/m5-reader-writer-compat-suite.json` is authoritative; if the two disagree, the suite wins and this report must be regenerated in the same change. Release and support packets link directly to this report and to the per-family migration-diff reports under `artifacts/contracts/m5-migration-diff-reports/`.

## What the suite proves

For every durable M5 artifact family the JSON Schema catalog publishes, the suite carries checked-in fixtures and a migration-diff report proving:

- **forward-read** — a prior-version reader reads a current-version artifact and preserves the new fields,
- **back-read** — a current-version reader reads a prior-version artifact and tolerates the absent additive field,
- **round-trip** — a parse/serialize round-trip preserves every field, including unknown ones (write-back families),
- **migration-diff** — the prior-to-current change is additive-only,
- **unknown-field preservation** — vendor and future fields survive the read,
- **additive-field tolerance** — the field added at the current version is optional,
- **downgrade narrowing** — an artifact at an unsupported newer version narrows below the launch cutline instead of being silently upgraded, and
- **compare-only fallback** — a compare-only family is read and diffed but never written back, which is a passing documented state.

## Per-family suites

| Family | Lifecycle | Reader/writer posture | Write-back | Cases | Migration diff | Report |
| --- | --- | --- | --- | --- | --- | --- |
| command_descriptors | stable | read_write | backup_then_write | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/command_descriptors.json` |
| cli_headless_structured_output | stable | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/cli_headless_structured_output.json` |
| task_event_envelope | beta | bidirectional_interchange | backup_then_write | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/task_event_envelope.json` |
| execution_context_provenance | stable | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/execution_context_provenance.json` |
| diagnostic_records | beta | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/diagnostic_records.json` |
| project_doctor_findings | beta | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/project_doctor_findings.json` |
| repair_transactions | beta | read_write | backup_then_write | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/repair_transactions.json` |
| support_bundles_and_handoff | stable | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/support_bundles_and_handoff.json` |
| appearance_sessions_and_theme_assets | beta | bidirectional_interchange | backup_then_write | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/appearance_sessions_and_theme_assets.json` |
| teaching_tour_and_learning_packets | beta | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/teaching_tour_and_learning_packets.json` |
| policy_bundles | beta | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/policy_bundles.json` |
| capability_records | stable | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/capability_records.json` |
| notification_and_chronology_primitives | beta | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/notification_and_chronology_primitives.json` |
| replay_and_trace_evidence | beta | reader_only | compare_only | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/replay_and_trace_evidence.json` |
| service_optional_api | stable | bidirectional_interchange | backup_then_write | 7 | v1→v2 additive | `artifacts/contracts/m5-migration-diff-reports/service_optional_api.json` |

## Totals

- Suites: **15** (5 write-back, 10 compare-only)
- Cases: **105** across all families
- Migration-diff reports: **15** (all additive)
- Downgrade-narrowing cases: **15**
- Checked-in fixtures: **45** (prior/current/unsupported per family)

## Offline and mirror use

The suite catalog, the fixtures, the migration-diff reports, this report, and the validator bundle into offline/mirror artifact sets and validate without runtime service access (`offline_bundle.requires_runtime_service` is `false`).

## Freshness

The suite is current as of `2026-06-19`. CI regenerates it from `tools/regenerate_m5_reader_writer_compat_suite.py`, runs `tools/validate_m5_reader_writer_compat_suite.py`, and runs the typed Rust consumer's tests, so the published fixtures and reports cannot drift from the suite.
