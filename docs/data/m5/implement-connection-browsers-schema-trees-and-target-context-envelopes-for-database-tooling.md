# Connection browsers, schema trees, and target-context envelopes for database tooling

## Scope

This document describes the canonical M5 qualification packet for connection browsers, schema trees, and target-context envelopes in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling/mod.rs`
- Schema: `schemas/data/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.schema.json`
- Checked-in packet: `artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`
- Fixtures: `fixtures/data/m5/implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Connection browser | stable | stable | Shows connection class, auth source, write posture, target identity, schema tree, and export redaction. |
| Schema tree | stable | stable | Shows source kind, depth limits, node count, freshness state, and stale labeling. |
| Target context envelope | stable | stable | Shows endpoint identity, connection class, auth source, write posture, statement safety, transaction posture, result scope, and export redaction. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Connection browsers do not expose raw connection strings or secrets.
- Schema trees do not masquerade stale schema as live truth.
- Target-context envelopes are visible before send and disclose auth source, write posture, and statement safety.
- Export redaction modes are explicit across all surfaces.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
