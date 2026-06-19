# M5 JSON Schema catalog

This is the human-readable index of the canonical **M5 JSON Schema catalog**. The machine-readable catalog at `artifacts/contracts/m5-json-schema-catalog.json` is authoritative; if the two disagree, the catalog wins and this document must be updated in the same change.

## What the catalog publishes

For every durable M5 artifact family the public-contract publication matrix puts forward as a JSON-Schema-backed contract, the catalog publishes one checked-in JSON Schema **package** under `schemas/public/m5-json/` with:

- an explicit in-band **schema version field**,
- a **lifecycle/stability label** (the label the matrix publishes after narrowing),
- a field-level **compatibility contract** (additive-field rule, required-field policy, unknown-field preservation, downgrade behavior, and migration-note hooks),
- an **example payload** and a **round-trip fixture**, and
- a stable **schema identifier** (`$id`) that support, export, and docs/help surfaces resolve.

Each package schema preserves unknown fields (`additionalProperties: true`) so durable artifacts round-trip through export, support, and offline-mirror flows without stripping fields. The `x-aureline-contract` annotation in every package schema carries the family id, lifecycle label, and version fields, so a reader resolves the schema identifier and lifecycle label from the schema file alone.

## Resolving a schema identifier and lifecycle label

Given a durable artifact's `record_kind` and family, look up the family in the catalog's `packages` array to resolve its `schema_id`, `schema_path`, `version_field_names`, and `lifecycle_label`. The same schema identifier and lifecycle label are carried in the package schema file's `$id` and `x-aureline-contract.lifecycle_label`, and the package's `lifecycle_label` agrees with the publication matrix `published_label` for that family.

## Published packages

| Family | Package | Lifecycle | Version field | Schema |
| --- | --- | --- | --- | --- |
| command_descriptors | `m5.command_descriptors` | stable | `command_descriptor_schema_version` | `schemas/public/m5-json/command_descriptors.schema.json` |
| cli_headless_structured_output | `m5.cli_headless_structured_output` | stable | `command_descriptor_schema_version` | `schemas/public/m5-json/cli_headless_structured_output.schema.json` |
| task_event_envelope | `m5.task_event_envelope` | beta | `task_event_envelope_schema_version` | `schemas/public/m5-json/task_event_envelope.schema.json` |
| execution_context_provenance | `m5.execution_context_provenance` | stable | `execution_context_schema_version` | `schemas/public/m5-json/execution_context_provenance.schema.json` |
| diagnostic_records | `m5.diagnostic_records` | beta | `problem_evidence_chain_schema_version` | `schemas/public/m5-json/diagnostic_records.schema.json` |
| project_doctor_findings | `m5.project_doctor_findings` | beta | `doctor_finding_schema_version` | `schemas/public/m5-json/project_doctor_findings.schema.json` |
| repair_transactions | `m5.repair_transactions` | beta | `repair_transaction_schema_version` | `schemas/public/m5-json/repair_transactions.schema.json` |
| support_bundles_and_handoff | `m5.support_bundles_and_handoff` | stable | `support_bundle_schema_version` | `schemas/public/m5-json/support_bundles_and_handoff.schema.json` |
| appearance_sessions_and_theme_assets | `m5.appearance_sessions_and_theme_assets` | beta | `theme_asset_schema_version` | `schemas/public/m5-json/appearance_sessions_and_theme_assets.schema.json` |
| teaching_tour_and_learning_packets | `m5.teaching_tour_and_learning_packets` | beta | `learning_presentation_packet_schema_version` | `schemas/public/m5-json/teaching_tour_and_learning_packets.schema.json` |
| policy_bundles | `m5.policy_bundles` | beta | `admin_policy_schema_version` | `schemas/public/m5-json/policy_bundles.schema.json` |
| capability_records | `m5.capability_records` | stable | `capability_inventory_entry_schema_version` | `schemas/public/m5-json/capability_records.schema.json` |
| notification_and_chronology_primitives | `m5.notification_and_chronology_primitives` | beta | `activity_event_envelope_schema_version` | `schemas/public/m5-json/notification_and_chronology_primitives.schema.json` |
| replay_and_trace_evidence | `m5.replay_and_trace_evidence` | beta | `capture_session_schema_version` | `schemas/public/m5-json/replay_and_trace_evidence.schema.json` |
| service_optional_api | `m5.service_optional_api` | stable | `provider_handoff_schema_version` | `schemas/public/m5-json/service_optional_api.schema.json` |

## Compatibility contract

Every package carries the same field-level compatibility posture: Fields are added only as optional members in additive minor bumps; the required-field set is frozen until a major bump; unknown fields are preserved on round-trip; a family missing required publication evidence narrows below the launch cutline rather than inheriting an adjacent published family's label.

## Offline and mirror use

The catalog, the package schemas, the example payloads, the round-trip fixtures, and the validator bundle into offline/mirror artifact sets and validate without runtime service access (`offline_bundle.requires_runtime_service` is `false`).

## Freshness

The catalog is current as of `2026-06-19`. CI regenerates the catalog and its packages from `tools/regenerate_m5_json_schema_catalog.py`, runs `tools/validate_m5_json_schema_catalog.py`, and runs the typed Rust consumer's tests, so the published packages cannot drift from the catalog.
