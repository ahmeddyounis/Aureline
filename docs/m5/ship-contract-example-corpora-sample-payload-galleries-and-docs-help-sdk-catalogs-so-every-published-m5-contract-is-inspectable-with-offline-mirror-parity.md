# Ship contract example corpora, sample payload galleries, and docs/help/SDK catalogs for every published M5 contract

This is the narrative companion to the canonical **M5 contract catalog**. The machine-readable catalog is authoritative; if the two disagree, the catalog wins and this document must be updated in the same change.

- Catalog (source of truth): `artifacts/contracts/m5-contract-catalog.json`
- Sample payload galleries: `examples/contracts/m5-gallery/*.json`
- Help-center catalog: `docs/help/m5-public-contract-catalog.md`
- SDK samples doc: `docs/sdk/m5-contract-samples.md`
- Boundary schema: `schemas/public/m5-contracts/m5_contract_catalog.schema.json`
- Validator: `tools/validate_m5_contract_catalog.py`
- Regenerator: `tools/regenerate_m5_contract_catalog.py`
- Typed consumer + protected tests: `aureline-release` (`ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity`)
- Evidence/proof packet: `artifacts/m5/ship-contract-example-corpora-sample-payload-galleries-and-docs-help-sdk-catalogs-so-every-published-m5-contract-is-inspectable-with-offline-mirror-parity.md`

## What the catalog is for

The public-contract publication matrix records *whether* each M5 artifact family has published its contract forms. The per-form catalogs publish the JSON Schema packages, the OpenAPI service routes, and the WIT capability worlds. This catalog is the *consuming* layer on top of all of them: it lets users, admins, support, extension authors, and self-host/mirror operators enumerate every published contract family from one source and inspect a real, checked-in sample payload — offline — for each one.

Every entry points back to the canonical schema/spec identifier and the lifecycle label the matrix publishes after narrowing, so the catalog is never the only source of truth and never advertises a greener label than the matrix.

## What shipped

- A checked-in catalog joining all 16 published M5 contract families to their lifecycle label, canonical schema/spec identity, compatibility note, offline posture, and sample payload gallery.
- One sample payload gallery per family (32 samples total) with nominal and partial/not-provided samples and field-by-field notes.
- A Help-center catalog and an SDK samples doc rendered from the same source, plus the boundary schema, validator, regenerator, and a typed Rust consumer with an in-product CLI inspect surface.

## In-product inspect surface

The typed consumer ships a headless inspect bin that prints the catalog, a per-family inspect view, and the support-export projection. The per-family view links back to the same catalog entry and example payload the Help and SDK docs publish, so one catalog entry backs docs, SDK, support export, and in-product inspection at once:

```sh
cargo run -q -p aureline-release --bin aureline_release_ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity -- inspect command_descriptors
```

## Offline and mirror parity

The catalog, the galleries, the backing schemas, the Help/SDK docs, and the validator bundle into offline/mirror artifact sets and need no live service to inspect (`offline_bundle.requires_runtime_service` is `false`). Support-sensitive families publish copy/export-safe samples only.
