# M5 reader/writer compatibility suite

This is the SDK index of the canonical **M5 reader/writer compatibility suite**. The machine-readable suite at `artifacts/contracts/m5-reader-writer-compat-suite.json` is authoritative.

## How to consume the suite

Look up a family in the suite's `suites` array to resolve its reader/writer posture, its prior/current/unsupported fixtures, its per-case expectations, and its migration-diff report. Each case names the reader version, the writer version, the input fixture, the expected outcome, whether unknown fields are preserved, and whether the case writes back (and, if so, that it is backup/compare-first).

## Case kinds

- `forward_read`
- `back_read`
- `round_trip`
- `migration_diff`
- `unknown_field_preservation`
- `additive_field`
- `downgrade`
- `compare_only`

## Reader/writer posture and write-back

Posture is reused verbatim from the public-contract publication matrix. A `reader_only` family maps to a `compare_only` write-back posture: it is read and diffed but never written back, which is a passing documented state. Every other posture maps to `backup_then_write`: write-back is permitted only with backup/compare-first behavior.

## Published suites

| Family | Posture | Write-back | Cases | Report |
| --- | --- | --- | --- | --- |
| command_descriptors | read_write | backup_then_write | 7 | `artifacts/contracts/m5-migration-diff-reports/command_descriptors.json` |
| cli_headless_structured_output | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/cli_headless_structured_output.json` |
| task_event_envelope | bidirectional_interchange | backup_then_write | 7 | `artifacts/contracts/m5-migration-diff-reports/task_event_envelope.json` |
| execution_context_provenance | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/execution_context_provenance.json` |
| diagnostic_records | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/diagnostic_records.json` |
| project_doctor_findings | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/project_doctor_findings.json` |
| repair_transactions | read_write | backup_then_write | 7 | `artifacts/contracts/m5-migration-diff-reports/repair_transactions.json` |
| support_bundles_and_handoff | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/support_bundles_and_handoff.json` |
| appearance_sessions_and_theme_assets | bidirectional_interchange | backup_then_write | 7 | `artifacts/contracts/m5-migration-diff-reports/appearance_sessions_and_theme_assets.json` |
| teaching_tour_and_learning_packets | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/teaching_tour_and_learning_packets.json` |
| policy_bundles | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/policy_bundles.json` |
| capability_records | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/capability_records.json` |
| notification_and_chronology_primitives | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/notification_and_chronology_primitives.json` |
| replay_and_trace_evidence | reader_only | compare_only | 7 | `artifacts/contracts/m5-migration-diff-reports/replay_and_trace_evidence.json` |
| service_optional_api | bidirectional_interchange | backup_then_write | 7 | `artifacts/contracts/m5-migration-diff-reports/service_optional_api.json` |
