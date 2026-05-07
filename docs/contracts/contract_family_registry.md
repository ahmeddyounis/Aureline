# Contract-Family Registry

The contract-family registry is the shared “family map” for Aureline’s
contract-bearing packets, envelopes, and registries. It exists so new work does
not invent local naming, versioning, and minimum-envelope conventions for
machine-readable payloads.

Authoritative artifacts:

- `artifacts/contracts/contract_families.yaml` — machine-readable registry.
- `schemas/contracts/contract_family_registry.schema.json` — boundary schema for
  the registry and for worked examples.
- `fixtures/contracts/contract_family_examples/` — worked examples showing how
  a family participates in CLI/headless output, desktop export, support bundles,
  release packets, and provider/browser handoff evidence.

## How to use this registry

When adding a new machine-readable packet or record:

1. Find the most appropriate `family_id` in `artifacts/contracts/contract_families.yaml`.
2. Reuse the family’s version-field rule and minimum-envelope requirements
   before minting new field names.
3. If no family fits, add a new `contract_family_row` (and at least one worked
   example fixture) before introducing a new envelope or registry in a
   workstream-local format.

## Minimum-envelope ledger (stability floor)

The registry publishes a minimum-envelope ledger. Before a family claims
machine-readable stability (for example: export-safe, replay-safe, or
release-evidence-safe), its payloads must carry:

- Stable object identity fields (packet/event/bundle/record ids).
- Schema identity fields (record kind + schema version fields).
- Target/context binding (workspace/scope/execution context/provider scope).
- Actor/route identity where the payload is consequence-bearing.
- Build/bundle linkage where reproducibility or audit requires it.
- Lifecycle/support/confidence posture where consumers might overclaim.
- Source-language fallback and message-key bindings where user-facing text is
  present.
- Explicit redaction/omission/export posture (raw payloads never inlined).

## Adding a new family

Add a new row to `artifacts/contracts/contract_families.yaml` and include:

- `schema_homes[]` plus `version_field_names[]`
- minimum-envelope field sets under `minimum_envelope`
- `primary_doc_refs[]` and at least one `example_source_refs[]`
- compatibility linkages (`compatibility_surface_ids`, `compat_row:*` refs) when
  the family participates in compatibility or release review

