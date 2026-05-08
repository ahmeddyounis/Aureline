# Contract example drift rules

This document defines the contract for keeping:

- boundary schemas under `schemas/**`,
- canonical reference example payloads under `fixtures/**`, and
- docs snippets that embed those payloads

in sync.

Authoritative source map: `artifacts/ci/contract_example_sources.yaml`.

## Terminology

- **Protected family**: a contract family listed in the source map. The gate is
  intentionally scoped; the protected set grows as more families become
  launch-critical and widely referenced.
- **Canonical example**: a reference example payload indexed in
  `artifacts/contracts/example_pack_index.yaml` and listed in the source map.
- **Docs snippet**: a Markdown snippet block that starts with
  `<!-- aureline-snippet: ... -->` and ends with `<!-- /aureline-snippet -->`.

## Source-map rules

The source map is a small “review surface” that binds together:

- schema refs (the files whose semantics are protected),
- canonical example ids (the payloads that must remain schema-valid), and
- docs snippet ids (the snippet blocks that must not drift from the canonical payloads).

The source map carries SHA-256 digests for protected schema and payload files.
Updating a protected schema or payload therefore requires a source-map edit in
the same change, which makes the review explicit and mechanically checkable.

## Validation rules

### Example validation

- Every protected `example_id` MUST resolve to an example row in
  `artifacts/contracts/example_pack_index.yaml`.
- The example payload MUST validate against the example’s `schema_ref` after
  stripping fixture-only metadata keys (`$schema`, `__fixture__`).

### Docs snippet validation

- Every `docs_snippet` row in the source map MUST resolve to exactly one
  snippet block with the same `id=...` in the referenced Markdown document.
- The snippet’s embedded payload MUST match the canonical example payload after
  applying the configured normalization (by default, strip fixture-only metadata).

### Explicit review rules (CI gate)

For protected families:

- If a protected schema file changes, the source map MUST change in the same
  diff (and the recorded digest MUST match).
- If a protected canonical payload changes (including redaction posture), the
  source map MUST change in the same diff (and the recorded digest MUST match).

## How to represent evolution without false failures

The drift gate is intentionally strict about **traceability** and intentionally
flexible about **forward evolution**:

- **Additive fields**: adding an optional field to a schema does not force
  every existing example to include it. Examples remain valid if the schema
  allows omission.
- **Deprecated fields**: keep deprecated fields schema-valid during their
  overlap window. Examples may remain as-is (or be marked deprecated in the
  example index) until the removal window closes.
- **Redacted examples**: redaction changes are treated as meaning changes for
  docs/support consumers. Update the canonical payload and the source map in
  the same change; the gate prevents “silent” redaction drift.
- **Version-branch examples**: when a legacy shape must remain documented,
  keep it as a separate example row (often `fidelity_class: deprecated`) with a
  schema-ref that still validates that legacy shape, or with a dedicated legacy
  schema snapshot. Do not “repurpose” an example id to mean a different shape.

