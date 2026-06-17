# Fixtures: M5 remembered-state objects

Fixture corpus for the `m5_remembered_state_objects` packet. The canonical full packet is checked
in at:

`artifacts/workspace/m5/m5-remembered-state-objects.json`

and validated against:

`schemas/workspace/m5-remembered-state.schema.json`

## Scenarios

Each fixture is a standalone `WindowTopologySnapshot` consumed by the crate's
`m5_remembered_state_objects` unit tests via `include_str!`.

- `schema_evolution_forward_migrate.json` — a snapshot whose pane tree is at the **legacy** schema
  version `0`. The tests forward-migrate it to the current version, assert a `compatible_restore`
  outcome, and confirm stable pane ids survive the migration.
- `partial_hydrate.json` — a snapshot mid-hydrate: one `ready` editor, one `needs_hydration`
  notebook, and one `placeholder` AI panel. The tests confirm all three slots and their stable ids
  are preserved while the surfaces hold distinct availability states.
- `missing_dependency_substitution.json` — a snapshot where a custom-extension pane lost its
  extension. The tests confirm the slot and `pane_id` survive behind a placeholder that names the
  original role, preserves the slot (never `silent_delete`), and leaves the sibling editor
  untouched.

The fail-closed rejections — a serialized live authority ticket, a missing live-authority
attestation, a machine-local object marked exportable, a portable object carrying machine-local
state, a silent-delete placeholder, a flattened authority/topology bundle, a dangling reference,
duplicate pane ids, a bundle overstating fidelity over a placeholder, a placeholder without a card,
and a schema-id registry mismatch — are exercised as synthetic gate drills in the crate's
`m5_remembered_state_objects` unit tests against the checked-in packet.
