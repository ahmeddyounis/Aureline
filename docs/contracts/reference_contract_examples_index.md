# Reference Contract Examples

Aureline publishes reference example payloads so implementers can study real,
schema-shaped packets instead of reverse-engineering object structure from prose.

Canonical sources:

- Machine index: `artifacts/contracts/example_pack_index.yaml`
- Index schema: `schemas/contracts/example_pack_index.schema.json`
- Redaction and fidelity rules: `docs/contracts/example_redaction_rules.md`
- Curated payload home: `fixtures/contracts/reference_examples/`

## How to use

1. Open `artifacts/contracts/example_pack_index.yaml` and pick a `families[]`
   row that matches the contract surface you are implementing.
2. Prefer examples whose `fidelity_class` is `canonical`.
3. Use `schema_ref` to find the boundary schema that defines the payload.
4. Treat `intentionally_partial` and `illustrative_only` examples as shape
   guides, not full-fidelity guarantees.
5. When adding a new example, update the YAML index in the same change so docs,
   CI, and support tooling share one source of truth.

## Validation

`tools/validate_contract_example_pack.py` validates the index and schema-checks
the `ci_required` example payloads referenced by it (after stripping fixture-only
metadata keys per `docs/contracts/example_redaction_rules.md`).
