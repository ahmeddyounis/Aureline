# Geometry token and hit-target contract fixtures

Worked fixtures for the geometry token ledger and hit-target / overflow floors
contract frozen in:

- [`/docs/design/geometry_and_hit_target_contract.md`](../../../docs/design/geometry_and_hit_target_contract.md)

and validated by:

- [`/schemas/design/geometry_token.schema.json`](../../../schemas/design/geometry_token.schema.json)

These cases exist to keep launch-critical component families aligned on:

- which density-derived height token is used (`size.row.*`, `size.control.*`);
- which hit-target floors are asserted (`size.hit.min`, resize-handle floors); and
- which overflow guardrails are claimed when density/zoom/text scale makes a
  layout cramped.

Cases are intentionally small and do not embed screenshots, raw assets, or large
payloads.

## Fixtures

- [`button_standard_pointer.yaml`](./button_standard_pointer.yaml)
  — Standard-density pointer case for button hit targets and control height.
- [`input_compact_keyboard.yaml`](./input_compact_keyboard.yaml)
  — Compact-density keyboard case for input control height and focus clearance.
- [`tab_standard_pointer.yaml`](./tab_standard_pointer.yaml)
  — Tab overflow guardrails with stable hit targets.
- [`tree_compact_pointer.yaml`](./tree_compact_pointer.yaml)
  — Tree indentation and disclosure metrics with stable disclosure hit targets.
- [`table_standard_keyboard.yaml`](./table_standard_keyboard.yaml)
  — Table-row density token binding and collapse-before-truncate guardrails.
- [`dialog_comfortable_pointer.yaml`](./dialog_comfortable_pointer.yaml)
  — Dialog radius and scrim opacity expectations plus hit-target floors.
- [`banner_standard_pointer.yaml`](./banner_standard_pointer.yaml)
  — Banner spacing and overflow guardrails.
- [`status_item_compact_keyboard.yaml`](./status_item_compact_keyboard.yaml)
  — Status-item density + quiet-opacity token expectations.
