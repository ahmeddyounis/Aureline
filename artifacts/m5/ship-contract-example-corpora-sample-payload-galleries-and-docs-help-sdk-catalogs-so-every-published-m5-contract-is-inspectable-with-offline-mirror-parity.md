# Ship contract example corpora, sample payload galleries, and docs/help/SDK catalogs for every published M5 contract

Evidence record for the canonical M5 contract catalog: the one inspectable index that joins every published M5 contract family to its lifecycle label, canonical schema/spec identifier, compatibility note, offline posture, and a checked-in sample payload gallery.

## What shipped

- A checked-in contract catalog over every published M5 contract family: [`/artifacts/contracts/m5-contract-catalog.json`](../contracts/m5-contract-catalog.json) (16 families, 32 samples).
- Sample payload galleries (nominal plus partial/not-provided, with field-by-field notes): [`/examples/contracts/m5-gallery/`](../../examples/contracts/m5-gallery/).
- The Help-center catalog and the SDK samples doc rendered from the same source: [`/docs/help/m5-public-contract-catalog.md`](../../docs/help/m5-public-contract-catalog.md) and [`/docs/sdk/m5-contract-samples.md`](../../docs/sdk/m5-contract-samples.md).
- The boundary schema: [`/schemas/public/m5-contracts/m5_contract_catalog.schema.json`](../../schemas/public/m5-contracts/m5_contract_catalog.schema.json).
- The typed product object, its protected tests, and the in-product CLI inspect surface: `crates/aureline-release/src/ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity/` and `crates/aureline-release/src/bin/aureline_release_ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity.rs`.
- The single source of truth (regenerator) and the validator: [`/tools/regenerate_m5_contract_catalog.py`](../../tools/regenerate_m5_contract_catalog.py) and [`/tools/validate_m5_contract_catalog.py`](../../tools/validate_m5_contract_catalog.py).
- Negative fixtures and CI capture: [`/fixtures/contracts/m5-contract-catalog/`](../../fixtures/contracts/m5-contract-catalog/) and [`/artifacts/release/captures/ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity_validation_capture.json`](../release/captures/ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity_validation_capture.json).

## Families covered

`command_descriptors`, `cli_headless_structured_output`, `task_event_envelope`, `execution_context_provenance`, `diagnostic_records`, `project_doctor_findings`, `repair_transactions`, `support_bundles_and_handoff`, `appearance_sessions_and_theme_assets`, `teaching_tour_and_learning_packets`, `policy_bundles`, `capability_records`, `notification_and_chronology_primitives`, `replay_and_trace_evidence`, `extension_host_wit_world`, `service_optional_api`.

## How it stays honest

- Each entry's `lifecycle_label` equals the publication matrix's `published_label` for that family, so a narrowed contract family narrows here automatically and the catalog never advertises a greener label.
- Each gallery points back to the canonical schema/spec identifier and lifecycle label; the samples are never the only source of truth.
- Every JSON-Schema-backed gallery sample validates against the published package schema named by `json_schema_validation_ref`.
- Every gallery includes a partial/not-provided sample, so stable user-facing partial outcomes are never omitted.
- The catalog, galleries, backing schemas, and validator bundle into offline/mirror artifact sets and need no live service to inspect.
