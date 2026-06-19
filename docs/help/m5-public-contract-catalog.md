# M5 public contract catalog

This is the Help-center index of every **published M5 contract family**. It is rendered from one source — the machine-readable catalog at `artifacts/contracts/m5-contract-catalog.json` — so Help/About, the SDK docs, the docs center, support export, and the in-product CLI inspect surface all show the same lifecycle labels, schema/spec identifiers, example payloads, and compatibility notes. If this page and the catalog disagree, the catalog wins and both are regenerated together.

## What this catalog gives you

- One enumerable list of every published M5 contract family, its lifecycle label, and its canonical schema/spec identifier.
- A checked-in **sample payload gallery** per family (nominal plus partial/not-provided) you can inspect offline, with field-by-field notes.
- Offline/mirror parity: the catalog, the galleries, the backing schemas, and the validator bundle into mirror artifact sets and need no live service to inspect.

## Published contract families

| Family | Lifecycle | Form | Identity | Schema / spec | Samples |
| --- | --- | --- | --- | --- | --- |
| command_descriptors | stable | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/command_descriptors.schema.json` | [`command_descriptors.json`](../../examples/contracts/m5-gallery/command_descriptors.json) |
| cli_headless_structured_output | stable | cli_structured_output | json_schema | `schemas/public/m5-json/cli_headless_structured_output.schema.json` | [`cli_headless_structured_output.json`](../../examples/contracts/m5-gallery/cli_headless_structured_output.json) |
| task_event_envelope | beta (narrowed) | event_envelope_schema | json_schema | `schemas/public/m5-json/task_event_envelope.schema.json` | [`task_event_envelope.json`](../../examples/contracts/m5-gallery/task_event_envelope.json) |
| execution_context_provenance | stable | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/execution_context_provenance.schema.json` | [`execution_context_provenance.json`](../../examples/contracts/m5-gallery/execution_context_provenance.json) |
| diagnostic_records | beta | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/diagnostic_records.schema.json` | [`diagnostic_records.json`](../../examples/contracts/m5-gallery/diagnostic_records.json) |
| project_doctor_findings | beta | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/project_doctor_findings.schema.json` | [`project_doctor_findings.json`](../../examples/contracts/m5-gallery/project_doctor_findings.json) |
| repair_transactions | beta | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/repair_transactions.schema.json` | [`repair_transactions.json`](../../examples/contracts/m5-gallery/repair_transactions.json) |
| support_bundles_and_handoff | stable | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/support_bundles_and_handoff.schema.json` | [`support_bundles_and_handoff.json`](../../examples/contracts/m5-gallery/support_bundles_and_handoff.json) |
| appearance_sessions_and_theme_assets | beta | asset_package_manifest | json_schema | `schemas/public/m5-json/appearance_sessions_and_theme_assets.schema.json` | [`appearance_sessions_and_theme_assets.json`](../../examples/contracts/m5-gallery/appearance_sessions_and_theme_assets.json) |
| teaching_tour_and_learning_packets | beta | teaching_content_pack | json_schema | `schemas/public/m5-json/teaching_tour_and_learning_packets.schema.json` | [`teaching_tour_and_learning_packets.json`](../../examples/contracts/m5-gallery/teaching_tour_and_learning_packets.json) |
| policy_bundles | beta | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/policy_bundles.schema.json` | [`policy_bundles.json`](../../examples/contracts/m5-gallery/policy_bundles.json) |
| capability_records | stable | record_registry | json_schema | `schemas/public/m5-json/capability_records.schema.json` | [`capability_records.json`](../../examples/contracts/m5-gallery/capability_records.json) |
| notification_and_chronology_primitives | beta | event_envelope_schema | json_schema | `schemas/public/m5-json/notification_and_chronology_primitives.schema.json` | [`notification_and_chronology_primitives.json`](../../examples/contracts/m5-gallery/notification_and_chronology_primitives.json) |
| replay_and_trace_evidence | beta | json_schema_backed_contract_doc | json_schema | `schemas/public/m5-json/replay_and_trace_evidence.schema.json` | [`replay_and_trace_evidence.json`](../../examples/contracts/m5-gallery/replay_and_trace_evidence.json) |
| extension_host_wit_world | beta | wit_world_package | wit_world | `wit/aureline/aureline.wit` | [`extension_host_wit_world.json`](../../examples/contracts/m5-gallery/extension_host_wit_world.json) |
| service_optional_api | stable | openapi_family | openapi_spec | `openapi/service_api_seed.yaml` | [`service_optional_api.json`](../../examples/contracts/m5-gallery/service_optional_api.json) |

## Narrowing

A family's `lifecycle_label` is the label the publication matrix effectively publishes after narrowing. A family whose required contract, validator, migration, or publication evidence is missing or stale narrows below the launch cutline in the matrix, and this catalog inherits that narrowed label automatically — it never advertises a greener label than the matrix. Any narrowed family is marked `(narrowed)` above and carries its active gap reasons in the catalog entry.

## Offline and mirror use

Support and enterprise evaluation can inspect the full contract set from a build without live network access: `offline_bundle.requires_runtime_service` is `false`, and every gallery sample and backing schema is checked in. Support-sensitive families publish copy/export-safe samples only and never widen disclosure beyond their declared redaction class.

## Freshness

The catalog is current as of `2026-06-19`. CI regenerates it from the publication matrix and the per-form catalogs via `tools/regenerate_m5_contract_catalog.py`, runs `tools/validate_m5_contract_catalog.py`, and runs the typed Rust consumer's tests, so the catalog, galleries, and docs cannot drift from the upstream contract truth.
