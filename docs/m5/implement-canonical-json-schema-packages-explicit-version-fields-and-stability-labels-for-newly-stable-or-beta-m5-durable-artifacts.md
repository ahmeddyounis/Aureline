# Implement canonical JSON Schema packages, explicit version fields, and stability labels for M5 durable artifacts

This is the narrative companion to the canonical **M5 JSON Schema catalog**. The
machine-readable catalog is authoritative; if the two disagree, the catalog wins
and this document must be updated in the same change.

- Catalog (source of truth): `artifacts/contracts/m5-json-schema-catalog.json`
- Published packages: `schemas/public/m5-json/*.schema.json`
- Example payloads: `examples/contracts/m5/json/*.json`
- Round-trip fixtures: `fixtures/contracts/m5-json-roundtrip/*.json`
- SDK catalog doc: `docs/sdk/m5-json-schema-catalog.md`
- Boundary schema: `schemas/public/m5-contracts/m5_json_schema_catalog.schema.json`
- Validator: `tools/validate_m5_json_schema_catalog.py`
- Regenerator: `tools/regenerate_m5_json_schema_catalog.py`
- Typed consumer + protected tests: `aureline-release`
  (`implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts`)
- Evidence/proof packet: `artifacts/m5/implement-canonical-json-schema-packages-explicit-version-fields-and-stability-labels-for-newly-stable-or-beta-m5-durable-artifacts.md`

## What the catalog is for

The public-contract publication matrix records *whether* each M5 artifact family
has published the contract forms it needs. This catalog publishes the **JSON
Schema package itself** for every family the matrix puts forward as a
JSON-Schema-backed contract: a checked-in schema under `schemas/public/m5-json/`
with an explicit in-band version field, a lifecycle/stability label, a
field-level compatibility contract, an example payload, and a round-trip fixture.

Downstream surfaces resolve one schema identifier and one lifecycle label per
family from the catalog instead of restating field semantics. The package set is
the canonical contract form for the durable M5 artifact families the latest docs
treat as delivery-grade product truth.

## What each package publishes

Every package records, for one durable M5 artifact family:

- **Schema identifier** — the package's `$id` (a stable
  `https://aureline.dev/schemas/public/m5-json/<family>.schema.json` URI) and its
  checked-in `schema_path`.
- **Version field(s)** — the in-band schema version field(s) (for example
  `command_descriptor_schema_version`); the primary version field is required by
  the schema, and a reader rejects an unknown major without parsing prose.
- **Lifecycle label** — the label the family publishes (`lts`/`stable`/`beta`/
  `preview`/`withdrawn`), equal to the publication matrix's effective
  `published_label` after narrowing.
- **Field contract** — the additive-field rule
  (`additive_minor_optional_only`), required-field policy
  (`frozen_required_set`), unknown-field policy (`preserve`), downgrade behavior
  (`narrow_below_cutline`), and the migration-note hooks a reader consults.
- **Compatibility note** — a human-readable note plus a ref to the family's
  contract doc.
- **Example payload and round-trip fixture** — a minimal, version-stamped example
  and a fixture carrying unknown fields that must survive validation.

The package schema is self-describing: its `x-aureline-contract` annotation
carries the family id, lifecycle label, and version fields, so a reader resolves
the schema identifier and lifecycle label from the schema file alone.

## Producer and reader expectations

The field contract makes the producer/reader rules explicit:

- **Additive-field rule** — new fields land only as optional members in additive
  minor bumps; a new required field is a major bump.
- **Required-field policy** — the required set (record-kind tag, the primary
  version field, and the primary stable object identity) is frozen until a major
  bump.
- **Unknown-field preservation** — every package sets `additionalProperties:
  true`, so a reader that does not recognize a field keeps it. The guardrail is
  enforced: the validator loads each round-trip fixture, validates it against the
  package schema, and asserts an undeclared field survives a parse/serialize
  round-trip. Schema publication never strips fields the docs promise to
  round-trip.
- **Downgrade behavior** — a family that loses required publication evidence
  narrows below the launch cutline (it never inherits an adjacent published
  family's label), matching the matrix's narrow-don't-inherit posture.
- **Migration-note hooks** — each package points at the canonical interface
  lifecycle policy, and durable-state families also point at the migration and
  restore playbook.

## Offline and mirror use

The catalog, the package schemas, the example payloads, the round-trip fixtures,
and the validator bundle into offline/mirror artifact sets and validate without
runtime service access (`offline_bundle.requires_runtime_service` is `false`).
Exported artifacts can therefore be validated offline and mirrored.

## How downstream surfaces consume it

- **Export/import, support export, and docs/help** resolve a family's `schema_id`
  and `lifecycle_label` from the catalog (or from the package schema's `$id` and
  `x-aureline-contract.lifecycle_label`) and quote one identity and one label.
- **The publication matrix** continues to record per-form publication state; this
  catalog supplies the JSON Schema package it points at, and each package's
  lifecycle label is cross-checked against the matrix `published_label`.
- **Claim-publication automation** narrows any marketed/support-class family whose
  required contract, validator, migration, or publication evidence is missing,
  stale, or downgraded.

## Maintenance

Edit the package set in `tools/regenerate_m5_json_schema_catalog.py`, then run:

```bash
python3 tools/regenerate_m5_json_schema_catalog.py
python3 tools/validate_m5_json_schema_catalog.py
cargo test -p aureline-release --test \
  implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts
```

The regenerator is the single source of truth for the catalog, the package
schemas, the examples, the round-trip fixtures, the SDK doc, the CI validation
capture, and the negative fixtures. The validator and the typed Rust consumer
both recompute the derived state, so a hand-edited catalog or package fails CI.
