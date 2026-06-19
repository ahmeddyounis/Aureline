# M5 contract samples

This is the SDK-facing index of the **sample payload galleries** for every published M5 contract family. It is rendered from the catalog at `artifacts/contracts/m5-contract-catalog.json`; the catalog is authoritative.

## How to use the galleries

Each family has a gallery at `examples/contracts/m5-gallery/<family>.json`. A gallery names the family's `contract_identity` (the canonical schema or spec identifier and its lifecycle label) and lists `samples`, each with a `sample_class` (`nominal` or `partial_or_not_provided`), a `payload`, and `field_notes` that annotate every field. For a JSON-Schema-backed family, the `json_schema_validation_ref` names the schema the sample payloads validate against, so you can confirm a sample against the published package.

## Galleries

| Family | Lifecycle | Identity | Validates against | Gallery |
| --- | --- | --- | --- | --- |
| command_descriptors | stable | json_schema | `schemas/public/m5-json/command_descriptors.schema.json` | [`command_descriptors.json`](../../examples/contracts/m5-gallery/command_descriptors.json) |
| cli_headless_structured_output | stable | json_schema | `schemas/public/m5-json/cli_headless_structured_output.schema.json` | [`cli_headless_structured_output.json`](../../examples/contracts/m5-gallery/cli_headless_structured_output.json) |
| task_event_envelope | beta | json_schema | `schemas/public/m5-json/task_event_envelope.schema.json` | [`task_event_envelope.json`](../../examples/contracts/m5-gallery/task_event_envelope.json) |
| execution_context_provenance | stable | json_schema | `schemas/public/m5-json/execution_context_provenance.schema.json` | [`execution_context_provenance.json`](../../examples/contracts/m5-gallery/execution_context_provenance.json) |
| diagnostic_records | beta | json_schema | `schemas/public/m5-json/diagnostic_records.schema.json` | [`diagnostic_records.json`](../../examples/contracts/m5-gallery/diagnostic_records.json) |
| project_doctor_findings | beta | json_schema | `schemas/public/m5-json/project_doctor_findings.schema.json` | [`project_doctor_findings.json`](../../examples/contracts/m5-gallery/project_doctor_findings.json) |
| repair_transactions | beta | json_schema | `schemas/public/m5-json/repair_transactions.schema.json` | [`repair_transactions.json`](../../examples/contracts/m5-gallery/repair_transactions.json) |
| support_bundles_and_handoff | stable | json_schema | `schemas/public/m5-json/support_bundles_and_handoff.schema.json` | [`support_bundles_and_handoff.json`](../../examples/contracts/m5-gallery/support_bundles_and_handoff.json) |
| appearance_sessions_and_theme_assets | beta | json_schema | `schemas/public/m5-json/appearance_sessions_and_theme_assets.schema.json` | [`appearance_sessions_and_theme_assets.json`](../../examples/contracts/m5-gallery/appearance_sessions_and_theme_assets.json) |
| teaching_tour_and_learning_packets | beta | json_schema | `schemas/public/m5-json/teaching_tour_and_learning_packets.schema.json` | [`teaching_tour_and_learning_packets.json`](../../examples/contracts/m5-gallery/teaching_tour_and_learning_packets.json) |
| policy_bundles | beta | json_schema | `schemas/public/m5-json/policy_bundles.schema.json` | [`policy_bundles.json`](../../examples/contracts/m5-gallery/policy_bundles.json) |
| capability_records | stable | json_schema | `schemas/public/m5-json/capability_records.schema.json` | [`capability_records.json`](../../examples/contracts/m5-gallery/capability_records.json) |
| notification_and_chronology_primitives | beta | json_schema | `schemas/public/m5-json/notification_and_chronology_primitives.schema.json` | [`notification_and_chronology_primitives.json`](../../examples/contracts/m5-gallery/notification_and_chronology_primitives.json) |
| replay_and_trace_evidence | beta | json_schema | `schemas/public/m5-json/replay_and_trace_evidence.schema.json` | [`replay_and_trace_evidence.json`](../../examples/contracts/m5-gallery/replay_and_trace_evidence.json) |
| extension_host_wit_world | beta | wit_world | — (WIT world package) | [`extension_host_wit_world.json`](../../examples/contracts/m5-gallery/extension_host_wit_world.json) |
| service_optional_api | stable | openapi_spec | `schemas/public/m5-json/service_optional_api.schema.json` | [`service_optional_api.json`](../../examples/contracts/m5-gallery/service_optional_api.json) |

## Partial and not-provided states

Every gallery includes a `partial_or_not_provided` sample so the SDK shows the contract's stable representation for a partial or not-provided outcome — these are user-facing states, not errors, and the galleries do not omit them.

## Offline use

The galleries, the catalog, and the backing schemas are checked in and bundle into offline/mirror artifact sets; no live service is required to read or validate a sample.
