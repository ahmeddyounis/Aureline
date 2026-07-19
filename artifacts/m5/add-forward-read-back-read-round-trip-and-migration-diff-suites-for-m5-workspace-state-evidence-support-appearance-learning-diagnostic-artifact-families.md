# Evidence: M5 reader/writer compatibility suite

This evidence packet records the reader/writer compatibility proof for the durable M5 artifact families. It is generated alongside the suite catalog and is referenced by the canonical M5 evidence index.

## Proof corpus

- Suite catalog: `artifacts/contracts/m5-reader-writer-compat-suite.json` (current as of `2026-06-19`)
- Suites: 15 (5 write-back, 10 compare-only)
- Compatibility cases: 105
- Migration-diff reports: 15 (all additive)
- Checked-in fixtures: 45

## Verification

```bash
python3 tools/regenerate_m5_reader_writer_compat_suite.py
python3 tools/validate_m5_reader_writer_compat_suite.py
cargo test -p aureline-release --test rel_it_04_add_forward_read_back
```

## Per-family coverage

| Family | Posture | Cases | Migration diff |
| --- | --- | --- | --- |
| command_descriptors | read_write | 7 | v1→v2 additive |
| cli_headless_structured_output | reader_only | 7 | v1→v2 additive |
| task_event_envelope | bidirectional_interchange | 7 | v1→v2 additive |
| execution_context_provenance | reader_only | 7 | v1→v2 additive |
| diagnostic_records | reader_only | 7 | v1→v2 additive |
| project_doctor_findings | reader_only | 7 | v1→v2 additive |
| repair_transactions | read_write | 7 | v1→v2 additive |
| support_bundles_and_handoff | reader_only | 7 | v1→v2 additive |
| appearance_sessions_and_theme_assets | bidirectional_interchange | 7 | v1→v2 additive |
| teaching_tour_and_learning_packets | reader_only | 7 | v1→v2 additive |
| policy_bundles | reader_only | 7 | v1→v2 additive |
| capability_records | reader_only | 7 | v1→v2 additive |
| notification_and_chronology_primitives | reader_only | 7 | v1→v2 additive |
| replay_and_trace_evidence | reader_only | 7 | v1→v2 additive |
| service_optional_api | bidirectional_interchange | 7 | v1→v2 additive |
