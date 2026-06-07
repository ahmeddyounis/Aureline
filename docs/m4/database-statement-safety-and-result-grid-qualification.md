# Database Statement-Safety And Result-Grid Qualification

The promoted build treats database tooling as a governed execution and export
surface. A data-tooling row may show stable language only when it has current
packet evidence for connection class, execution origin, auth-source mode,
target identity, write posture, statement safety, transaction posture, result
scope, export redaction, query-history privacy, explain-plan freshness, and
handoff lineage.

Source of truth:

- [`artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json`](../../artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json)
- [`schemas/release/database-statement-safety-and-result-grid-qualification.schema.json`](../../schemas/release/database-statement-safety-and-result-grid-qualification.schema.json)
- [`fixtures/data/m4/database-statement-safety-and-result-grid/cases.json`](../../fixtures/data/m4/database-statement-safety-and-result-grid/cases.json)

## What Is Stable

Stable coverage is limited to the guarded contracts:

- connection picker/run-bar rows that disclose engine, connection class,
  database/schema, execution origin, auth-source mode, transaction/write
  posture, and target identity;
- statement-safety rows that separate read-only, DML, DDL,
  session-affecting, multi-statement, ambiguous, and blocked classes;
- result-grid rows that disclose virtualization, typed columns, large-cell
  expansion, truncation, result scope, and copy/export redaction;
- query-history metadata rows that are local-first, bounded, redactable, and
  metadata-first by default;
- explain-plan rows that distinguish estimated, actual, and stale imported
  plans with engine/version and capture-time truth;
- handoff rows that preserve source refs, row/column scope, type fidelity,
  freshness, destination class, and share/local restrictions.

## What Is Narrowed

Live multi-engine SQL execution and direct row mutation are not promoted by
this packet. They remain below stable until connector-specific evidence proves
write guards, transaction behavior, cancellation, result streaming, export
round trips, staged row mutation, and rollback/recovery behavior.

## Required Downgrade Behavior

If any exposed data-tooling surface cannot prove its connection safety,
statement-safety behavior, result-grid virtualization/export truth,
query-history redaction, explain-plan freshness, and handoff lineage, it must
render as preview, inspect-only, import-only, or labs. It must not inherit
stable wording from a notebook, request workspace, generic table viewer, or
runtime row.
