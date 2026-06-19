# M5 contract blocker dashboard

Machine-readable contract-health summary for shiproom and partner review. It is rendered from one source — the contract-health register at `artifacts/release/m5-contract-health.json` — by `tools/regenerate_m5_contract_health.py`, so shiproom resolves exact contract package versions and build identity for the candidate under review instead of running an ad hoc spreadsheet check. If this page and the register disagree, the register wins and both are regenerated together.

- Register: `artifacts/release/m5-contract-health.json`
- CI gate manifest: `ci/contracts/m5-contract-gates/manifest.json`
- Build identity (resolved at review time): `artifacts/build/build_identity.json`
- Release candidate: `rc-m5-public-contract-train`
- Current as of: `2026-06-19`

## Promotion decision

**HOLD** — Promotion is held: one or more release-blocking M5 contract families have a failing required contract gate (a missing schema/spec package, example corpus, validator suite, compatibility report, or release-packet linkage). Publishing the missing contract evidence and rerunning the gates clears the hold.

Blocking families: `task_event_envelope`.
Blocking gate kinds: `compatibility_report`.

## Family health

| Family | Blocking | Health | Decision | Package version | Mirror | Failing gates |
| --- | --- | --- | --- | --- | --- | --- |
| `command_descriptors` | yes | healthy | clear | `json_schema` v1 | not_applicable | — |
| `cli_headless_structured_output` | yes | healthy | clear | `json_schema` v1 | not_applicable | — |
| `task_event_envelope` | yes | blocked | hold | `json_schema` v1 | unpublished | `compatibility_report` |
| `execution_context_provenance` | yes | healthy | clear | `json_schema` v1 | not_applicable | — |
| `diagnostic_records` | no | healthy | clear | `json_schema` v1 | not_applicable | — |
| `project_doctor_findings` | no | healthy | clear | `json_schema` v1 | not_applicable | — |
| `repair_transactions` | no | healthy | clear | `json_schema` v1 | not_applicable | — |
| `support_bundles_and_handoff` | yes | healthy | clear | `json_schema` v1 | current | — |
| `appearance_sessions_and_theme_assets` | no | healthy | clear | `json_schema` v1 | current | — |
| `teaching_tour_and_learning_packets` | no | healthy | clear | `json_schema` v1 | current | — |
| `policy_bundles` | no | healthy | clear | `json_schema` v1 | current | — |
| `capability_records` | yes | healthy | clear | `json_schema` v1 | not_applicable | — |
| `notification_and_chronology_primitives` | no | healthy | clear | `json_schema` v1 | not_applicable | — |
| `replay_and_trace_evidence` | no | healthy | clear | `json_schema` v1 | current | — |
| `extension_host_wit_world` | yes | healthy | clear | `wit_world` v1 | not_applicable | — |
| `service_optional_api` | yes | healthy | clear | `openapi_spec` v1 | current | — |

## Counts

- Families: 16 (8 release-blocking)
- Health: 15 healthy, 0 narrowed, 1 blocked
- Gates: 80 evaluated (79 pass, 0 downgrade, 1 fail)
- Mirror-publishable families: 15 / 16

## How it stays honest

- Each family's `lifecycle_label` equals the publication matrix's published label after narrowing, so a narrowed contract family narrows here automatically and the dashboard never advertises a greener label.
- A release-blocking family with a failing required contract gate holds promotion; CI runs the same register, so docs/help can never claim a contract is published if the gates say otherwise.
- Mirror/offline publishability follows the gate outputs, so self-hosted and air-gapped trains see the same blockers.
