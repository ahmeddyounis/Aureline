# M5 public-contract certification report

Closeout certification for the full M5 public-contract publication lane. For every claimed M5 public artifact family it binds the published contract form, lifecycle metadata, example corpus, validator coverage, compatibility report, and release-graph linkage into one certification verdict. It is rendered from one source — the certification register at `artifacts/certification/m5-public-contract-certification.json` — by `tools/regenerate_m5_public_contract_certification.py`, which joins the contract-health register, the publication matrix, and the per-form contract catalogs, so shiproom, support, docs, SDK, and partner review resolve one certification truth per family instead of restating field semantics. If this report and the register disagree, the register wins and both are regenerated together.

- Register: `artifacts/certification/m5-public-contract-certification.json`
- Shiproom dashboard: `shiproom/m5-public-contract-certification-dashboard.md`
- Contract-health register: `artifacts/release/m5-contract-health.json`
- Publication matrix: `artifacts/contracts/m5-stability-lifecycle-map.json`
- Canonical M5 evidence index: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`
- Current as of: `2026-06-19`

## Certification decision

**HOLD** — Certification is held: one or more release-blocking M5 public artifact families have a missing required contract pillar (published contract form, lifecycle metadata, example corpus, validator coverage, compatibility report, or release-graph linkage). Publishing the missing contract evidence and rerunning the gate clears the hold.

Withheld release-blocking families: `task_event_envelope`.

## Family certification

| Family | Form | Claim | Certified | Ver | State | Pillars (cur/stale/missing) | Decision |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `command_descriptors` | `json_schema_backed_contract_doc` | stable | stable | v1 | certified | 6/0/0 | clear |
| `cli_headless_structured_output` | `cli_structured_output` | stable | stable | v1 | certified | 6/0/0 | clear |
| `task_event_envelope` | `event_envelope_schema` | stable | beta | v1 | withheld | 5/0/1 | hold |
| `execution_context_provenance` | `json_schema_backed_contract_doc` | stable | stable | v1 | certified | 6/0/0 | clear |
| `diagnostic_records` | `json_schema_backed_contract_doc` | beta | beta | v1 | certified | 6/0/0 | clear |
| `project_doctor_findings` | `json_schema_backed_contract_doc` | beta | beta | v1 | certified | 6/0/0 | clear |
| `repair_transactions` | `json_schema_backed_contract_doc` | beta | beta | v1 | certified | 6/0/0 | clear |
| `support_bundles_and_handoff` | `json_schema_backed_contract_doc` | stable | stable | v1 | certified | 6/0/0 | clear |
| `appearance_sessions_and_theme_assets` | `asset_package_manifest` | beta | beta | v1 | certified | 6/0/0 | clear |
| `teaching_tour_and_learning_packets` | `teaching_content_pack` | beta | beta | v1 | certified | 6/0/0 | clear |
| `policy_bundles` | `json_schema_backed_contract_doc` | beta | beta | v1 | certified | 6/0/0 | clear |
| `capability_records` | `record_registry` | stable | stable | v1 | certified | 6/0/0 | clear |
| `notification_and_chronology_primitives` | `event_envelope_schema` | beta | beta | v1 | certified | 6/0/0 | clear |
| `replay_and_trace_evidence` | `json_schema_backed_contract_doc` | beta | beta | v1 | certified | 6/0/0 | clear |
| `extension_host_wit_world` | `wit_world_package` | beta | beta | v1 | certified | 6/0/0 | clear |
| `service_optional_api` | `openapi_family` | stable | stable | v1 | certified | 6/0/0 | clear |

## Contract pillars

Each family is certified on one cell per pillar. Every pillar is required; a missing required pillar on a release-blocking family withholds certification and holds promotion, while a stale pillar narrows the family below the cutline without inheriting an adjacent family's claim.

| Pillar | What it certifies | Certifying artifact |
| --- | --- | --- |
| `published_contract_form` | Published contract form (JSON Schema / WIT / OpenAPI) | `artifacts/contracts/m5-json-schema-catalog.json` |
| `lifecycle_metadata` | Lifecycle metadata (explicit version field + lifecycle label) | `artifacts/contracts/m5-stability-lifecycle-map.json` |
| `example_corpus` | Example payload corpus | `artifacts/contracts/m5-contract-catalog.json` |
| `validator_coverage` | Validator coverage wired into CI | `ci/contract_validation.sh` |
| `compatibility_report` | Compatibility / migration report | `artifacts/contracts/m5-reader-writer-compat-suite.json` |
| `release_graph_linkage` | Release-graph linkage (release packet + build identity) | `artifacts/build/build_identity.json` |

## Counts

- Families: 16 (8 release-blocking)
- Certification: 15 certified, 0 narrowed, 1 withheld (1 narrowed below the marketed claim)
- Pillars: 96 evaluated (95 current, 0 stale, 1 missing)
- Mirror/offline publishable families: 15
