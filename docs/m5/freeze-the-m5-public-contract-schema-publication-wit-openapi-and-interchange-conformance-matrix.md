# Freeze the M5 public-contract, schema-publication, WIT/OpenAPI, and interchange-conformance matrix

This is the narrative companion to the canonical **M5 public-contract publication
matrix**. The machine-readable matrix is authoritative; if the two disagree, the
matrix wins and this document must be updated in the same change.

- Matrix (source of truth): `artifacts/contracts/m5-stability-lifecycle-map.json`
- Flat inventory (CSV): `artifacts/contracts/m5-public-contract-inventory.csv`
- Human matrix (Markdown): `artifacts/contracts/m5-public-contract-matrix.md`
- Schema: `schemas/public/m5-contracts/m5_public_contract_matrix.schema.json`
- Validator: `tools/validate_m5_public_contract_matrix.py`
- Regenerator: `tools/regenerate_m5_public_contract_matrix.py`
- Typed consumer + protected tests: `aureline-release`
  (`freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix`)
- Evidence/proof packet: `artifacts/m5/freeze-the-m5-public-contract-schema-publication-wit-openapi-and-interchange-conformance-matrix.md`

## What the matrix is for

M5 ships many versioned artifact families and stable-facing machine-readable
outputs. This matrix is the one canonical inventory of every artifact family the
source docs treat as a **published contract** — machine-readable, exportable, and
observable through the CLI, SDK, support export, or a mirror — so later batches
stop inventing local schema, export, or versioning behavior by implication.

Each row classifies one family and freezes the publication bar it must clear
before it can carry a Stable contract claim. The matrix does not restate field
semantics; it reuses the existing contract-family registry, compatibility-surface
inventory, qualification matrix, stability/lifecycle vocabulary, and claim
manifest, and points at the published schemas, WIT worlds, OpenAPI specs, example
corpora, validators, and release packets that already exist.

## What each row classifies

Every row records, for one M5 public-contract family:

- **Contract form** — drawn from the compatibility-surface inventory's
  `contract_form_values` (JSON-Schema-backed doc, JSON-Schema registry, record
  registry, event-envelope schema, WIT world package, OpenAPI family, field set,
  CLI structured output, textual interchange contract, asset-package manifest, or
  teaching content pack).
- **Stability lane** — the contract-family registry maturity lane (`stable`,
  `beta`, `experimental`, `internal`).
- **Reader/writer posture** — `reader_only`, `writer_only`, `read_write`, or
  `bidirectional_interchange`.
- **Packaging need** — `local_only`, `mirrored`, `managed`, or `browser_handoff`.
- **Claim label / published label** — the lifecycle label the contract is put
  forward at (`lts`/`stable`/`beta`/`preview`/`withdrawn`) and the label it
  effectively publishes after any narrowing.
- **Publication requirements** — per contract form (JSON Schema, WIT, OpenAPI,
  Markdown summary, example payloads, migration notes) plus the validator suite,
  whether the form is **required** before promotion and its **state**
  (`published`, `partial`, `missing`, `not_applicable`), with refs to the
  published artifacts.
- **Release-packet dependency** — the claim manifest entry, qualification row, or
  evidence index the family rides.
- **Compatibility links** — the compatibility-surface row and qualification row.

## The cutline and auto-narrowing

The launch cutline sits at `stable`. A family carries a Stable (or LTS) contract
claim only when **every required** contract form, its validator suite, and its
release-packet linkage are `published`. A family that is missing any required
publication evidence raises the matching **gap reason** and **narrows below the
cutline** automatically — it never inherits an adjacent published family's claim.

Gap reasons are closed: `json_schema_unpublished`, `wit_world_unpublished`,
`openapi_spec_unpublished`, `markdown_summary_unpublished`,
`example_payloads_unpublished`, `migration_notes_unpublished`,
`validator_suite_unpublished`, and `release_packet_unlinked`. Each has a stop rule
that, when it fires on a family put forward at the cutline, holds promotion and
prescribes the remediation action (publish the contract form, publish examples,
wire the validator suite, or link the release packet).

This is the same narrow-don't-inherit posture used by the M5 claim matrix,
qualification matrix, and certification-train evidence index, expressed for the
*publication* of contract forms.

## Worked narrowing examples (auto-downgrade)

The current matrix holds promotion (`hold`) because two release-blocking families
put forward at the cutline are missing required publication evidence:

- `task_event_envelope` is put forward for a Stable contract claim but publishes
  no migration/deprecation notes yet, so it narrows to Beta
  (`migration_notes_unpublished`).
- `service_optional_api` is put forward for a Stable contract claim but its
  OpenAPI family is still a seed (`partial`), so it narrows to Beta
  (`openapi_spec_unpublished`).

Both narrow until the missing contract form is published, which is exactly the
release-control behavior this row freezes.

## How downstream surfaces consume it

The matrix is meant to drive, not duplicate:

- **Claim manifests and shiproom dashboards** read the per-family publication
  state and promotion verdict rather than maintaining a shadow inventory.
- **Help/About, SDK/docs, and support export** render the published label and the
  published contract refs, never re-deriving field semantics.
- **Claim-publication automation** narrows any marketed/support-class family whose
  required contract, validator, migration, or publication evidence is missing,
  stale, or downgraded.

## Maintenance

Edit the row set in `tools/regenerate_m5_public_contract_matrix.py`, then run:

```bash
python3 tools/regenerate_m5_public_contract_matrix.py
python3 tools/validate_m5_public_contract_matrix.py
cargo test -p aureline-release --test \
  freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix
```

The regenerator is the single source of truth for the JSON, the CSV and Markdown
projections, the CI validation capture, and the negative fixtures. The validator
and the typed Rust consumer both recompute the derived state from the publication
requirements, so a hand-edited matrix fails CI.
