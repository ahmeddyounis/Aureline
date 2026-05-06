# Surface-slot Matrix Cases

These fixtures exercise the UI composition surface-slot matrix frozen in
[`/docs/ux/ui_composition_surface_slot_matrix.md`](../../../docs/ux/ui_composition_surface_slot_matrix.md)
and the machine-readable artifact at
[`/artifacts/ux/surface_slot_matrix.yaml`](../../../artifacts/ux/surface_slot_matrix.yaml).

The boundary schema is:

- [`/schemas/ux/surface_slot_row.schema.json`](../../../schemas/ux/surface_slot_row.schema.json)

Coverage:

- `surface_slot_definitions_seed.yaml` exports a seed set of slot
  definition rows.
- `surface_slot_surface_mappings_seed.yaml` exports canonical
  surface-to-slot mapping rows (allowed + banned slots).
- `surface_slot_admission_guardrails.yaml` contains admission-decision
  cases proving protected-slot denial and safe reroute/placeholder rules.

