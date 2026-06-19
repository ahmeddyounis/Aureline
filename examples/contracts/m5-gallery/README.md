# M5 contract sample payload galleries

This directory holds one **sample payload gallery** per published M5 contract family. Each gallery carries a nominal sample and a partial/not-provided sample, each with field-by-field notes, and points back to the canonical schema/spec identifier and lifecycle label so the samples are never the only source of truth.

The catalog at `artifacts/contracts/m5-contract-catalog.json` is authoritative; if a gallery and the catalog disagree, the catalog wins and both are regenerated together by `tools/regenerate_m5_contract_catalog.py`.

| Family | Lifecycle | Identity | Gallery |
| --- | --- | --- | --- |
| command_descriptors | stable | json_schema | `command_descriptors.json` |
| cli_headless_structured_output | stable | json_schema | `cli_headless_structured_output.json` |
| task_event_envelope | beta | json_schema | `task_event_envelope.json` |
| execution_context_provenance | stable | json_schema | `execution_context_provenance.json` |
| diagnostic_records | beta | json_schema | `diagnostic_records.json` |
| project_doctor_findings | beta | json_schema | `project_doctor_findings.json` |
| repair_transactions | beta | json_schema | `repair_transactions.json` |
| support_bundles_and_handoff | stable | json_schema | `support_bundles_and_handoff.json` |
| appearance_sessions_and_theme_assets | beta | json_schema | `appearance_sessions_and_theme_assets.json` |
| teaching_tour_and_learning_packets | beta | json_schema | `teaching_tour_and_learning_packets.json` |
| policy_bundles | beta | json_schema | `policy_bundles.json` |
| capability_records | stable | json_schema | `capability_records.json` |
| notification_and_chronology_primitives | beta | json_schema | `notification_and_chronology_primitives.json` |
| replay_and_trace_evidence | beta | json_schema | `replay_and_trace_evidence.json` |
| extension_host_wit_world | beta | wit_world | `extension_host_wit_world.json` |
| service_optional_api | stable | openapi_spec | `service_optional_api.json` |
