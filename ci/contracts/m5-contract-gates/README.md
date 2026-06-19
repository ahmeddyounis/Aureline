# M5 contract CI gates

These are the **CI gates** that make a missing, stale, downgraded, or incompatible M5 contract package block the same release and claim-publication paths as missing evidence or stale qualification rows. They are rendered from one source — the machine-readable contract-health register at `artifacts/release/m5-contract-health.json` — by `tools/regenerate_m5_contract_health.py` and checked by `tools/validate_m5_contract_health.py`. The shiproom blocker dashboard at `shiproom/m5-contract-blocker-dashboard.md` renders from the same register, so shiproom never relies on a one-off spreadsheet check.

## Gates

| Gate | Guards | Raises | Descriptor |
| --- | --- | --- | --- |
| `schema_spec_package` | json_schema, wit_world, openapi_spec | json_schema_unpublished, wit_world_unpublished, openapi_spec_unpublished | [`schema_spec_package.json`](schema_spec_package.json) |
| `example_corpus` | example_payloads | example_payloads_unpublished | [`example_corpus.json`](example_corpus.json) |
| `validator_coverage` | validator_suite | validator_suite_unpublished | [`validator_coverage.json`](validator_coverage.json) |
| `compatibility_report` | markdown_summary, migration_notes | markdown_summary_unpublished, migration_notes_unpublished | [`compatibility_report.json`](compatibility_report.json) |
| `release_packet_linkage` | release-packet linkage | release_packet_unlinked | [`release_packet_linkage.json`](release_packet_linkage.json) |

## How a gate fails or downgrades a candidate

Each gate reads the publication-requirement states the M5 public-contract publication matrix records for a family. A required artifact that is `published` passes; one that is `partial` downgrades the family (it inherits the matrix's narrowed label); one that is `missing` fails the gate. A failing gate on a **release-blocking** family holds promotion. The mirror/offline publishability of a family follows the same gate outputs, so sovereign and self-hosted trains are not second-class citizens.

## Current decision

- Decision: **hold**
- Blocking families: `task_event_envelope`
- Blocking gate kinds: `compatibility_report`
