# Implement contract CI gates, release-artifact-graph linkage, and shiproom blockers for stale, missing, or incompatible M5 schema/spec packages

Evidence record for the canonical M5 contract-health register: the machine-readable join that ties every published M5 contract family to the CI gates guarding its contract packages, the exact release artifact graph and build identity the candidate ships, and the shiproom blocker decision those signals produce.

## What shipped

- A checked-in contract-health register: [`/artifacts/release/m5-contract-health.json`](../release/m5-contract-health.json) (16 families, 80 gate evaluations).
- The CI gates and their manifest: [`/ci/contracts/m5-contract-gates/`](../../ci/contracts/m5-contract-gates/).
- The shiproom blocker dashboard: [`/shiproom/m5-contract-blocker-dashboard.md`](../../shiproom/m5-contract-blocker-dashboard.md).
- The Help-center page: [`/docs/help/m5-contract-health.md`](../../docs/help/m5-contract-health.md).
- The boundary schema: [`/schemas/public/m5-contracts/m5_contract_health.schema.json`](../../schemas/public/m5-contracts/m5_contract_health.schema.json).
- The typed product object, its protected tests, and the in-product CLI inspect surface: `crates/aureline-release/src/implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages/` and `crates/aureline-release/src/bin/aureline_release_implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages.rs`.
- The single source of truth (regenerator) and the validator: [`/tools/regenerate_m5_contract_health.py`](../../tools/regenerate_m5_contract_health.py) and [`/tools/validate_m5_contract_health.py`](../../tools/validate_m5_contract_health.py).
- Negative fixtures and CI capture: [`/fixtures/contracts/m5-contract-health/`](../../fixtures/contracts/m5-contract-health/) and [`/artifacts/release/captures/implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages_validation_capture.json`](../release/captures/implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages_validation_capture.json).

## Families covered

`command_descriptors`, `cli_headless_structured_output`, `task_event_envelope`, `execution_context_provenance`, `diagnostic_records`, `project_doctor_findings`, `repair_transactions`, `support_bundles_and_handoff`, `appearance_sessions_and_theme_assets`, `teaching_tour_and_learning_packets`, `policy_bundles`, `capability_records`, `notification_and_chronology_primitives`, `replay_and_trace_evidence`, `extension_host_wit_world`, `service_optional_api`.

## How it stays honest

- Each family's `lifecycle_label` equals the publication matrix's published label after narrowing, so a narrowed contract family narrows here automatically and the register never advertises a greener label.
- A release-blocking family with a failing required contract gate sets the register's promotion decision to `hold`; the same register backs CI, the shiproom dashboard, and the Help page, so docs/help can never claim a contract is published when the build artifact graph for that train does not contain the matching package.
- The register reuses the matrix gap-reason and remediation vocabulary and the release-candidate freshness states, so contract health is not a new red/yellow vocabulary.
- Mirror/offline publishability follows the gate outputs, so sovereign and self-hosted trains see the same blockers.

## Current decision

Promotion decision: **hold**. Promotion is held: one or more release-blocking M5 contract families have a failing required contract gate (a missing schema/spec package, example corpus, validator suite, compatibility report, or release-packet linkage). Publishing the missing contract evidence and rerunning the gates clears the hold.
