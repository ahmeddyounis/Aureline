# Interface-Freeze Register

This document is the reviewer-facing companion for the gated interface-freeze
register:

- [`/artifacts/governance/interface_freeze_register_beta.json`](../../artifacts/governance/interface_freeze_register_beta.json)
- schema: [`/schemas/governance/interface_freeze_register_beta.schema.json`](../../schemas/governance/interface_freeze_register_beta.schema.json)

The register makes the Beta API/ops freeze **explicit** instead of leaving it
implicit in scattered version constants. It records, for every governed
interface surface, whether the contract is open, soft-frozen, or hard-frozen,
the version it was frozen at, the exception classes permitted for it, and any
recorded exceptions that authorized a change.

It builds on the existing governed schema-family registry
([`/schemas/registry/schema_registry.json`](../../schemas/registry/schema_registry.json));
it is not a parallel system. Every governed schema family must declare a freeze
state through a row whose `version_source` is `governed_schema_registry`, and
those rows are cross-checked against the registry for version and schema
reference.

## Freeze states

- `open` — the surface may still change freely; it is not part of the Beta
  freeze. No exception is required to change it.
- `soft_frozen` — additive, backward-compatible change is expected and does not
  need an exception; breaking change is discouraged and reviewed.
- `hard_frozen` — the surface is locked at its frozen version. It may not move
  past `frozen_at_version` without a recorded exception whose class is one of
  the row's `allowed_exception_classes`.

## Surfaces governed

The register spans CLI/headless output, settings and portable-state
serialization, extension SDK and manifest schemas, and the governed
export/telemetry/diagnostic packet families. Each row names a
`surface_class`:

- `cli_headless`
- `settings_portable_state`
- `extension_sdk_manifest`
- `governed_export_packet`

## Exception classes

A recorded exception names one class of authorized change:

- `additive_backward_compatible` — a purely additive, backward-compatible
  change.
- `security_fix` — a change required to correct a security defect.
- `defect_correction` — a change required to correct an incorrect contract.
- `coordinated_breaking_change` — an approved, coordinated breaking change with a
  migration path.

A `hard_frozen` row whose `allowed_exception_classes` omits a class can never use
that class to justify a change. A row may allow an empty set, which means no
change is permitted at all.

## Version handling

Each row carries `frozen_at_version` (the version that was frozen) and
`current_version` (the version in force today). For `governed_schema_registry`
rows the `current_version` must equal the governed registry's `schema_version`
for that family, so the register cannot claim a version the registry disagrees
with. For `declared` rows — settings, portable state, and extension manifest
surfaces that are not governed-registry families — the `current_version` is
declared in the register itself.

When a `hard_frozen` row's `current_version` differs from its
`frozen_at_version`, there must be a recorded exception landing at the current
version with an allowed class. Otherwise the surface changed silently and the
gate fails.

## CI gate

Run:

```sh
python3 ci/check_beta_interface_freeze.py --repo-root .
```

The gate fails when a governed schema family lacks a freeze state, when a
hard-frozen surface changed version without a recorded exception, when a
governed-source row disagrees with the governed registry, when a recorded
exception uses a class the row does not allow, or when the summary counts or
closed vocabularies drift. It also runs negative drills proving the coverage,
hard-freeze, and exception-class rejections all fire, and writes a validation
capture to
[`/artifacts/governance/captures/interface_freeze_register_beta_validation_capture.json`](../../artifacts/governance/captures/interface_freeze_register_beta_validation_capture.json).

The typed Rust consumer
(`aureline_governance::interface_freeze::current_interface_freeze_register`)
reads the same register and runs the same cross-check, so `cargo test -p
aureline-governance` enforces these invariants without a cargo build in CI.
