# Editor-layer fixtures

Seed corpus for the contract frozen in
[`/docs/ux/editor_anatomy_contract.md`](../../../docs/ux/editor_anatomy_contract.md)
and the boundary schema at
[`/schemas/ux/editor_layer.schema.json`](../../../schemas/ux/editor_layer.schema.json).

Each fixture is a single JSON document that exercises editor layer
ownership, accessory role classification, stable text-column behavior,
state placement, or degraded/large-file downgrades without exposing raw
absolute paths, raw file bodies, raw preview DOM, raw provider payloads,
or credential material. Identity is carried through opaque refs and
bounded labels.

## Cases

| Fixture | Record kind | Main proof |
| --- | --- | --- |
| [`canonical_layer_catalog.json`](./canonical_layer_catalog.json) | `editor_layer_catalog_record` | The canonical seven layers, launch-bearing accessories, and state-placement rules are assigned without overlapping ownership. |
| [`stable_text_column_transient_assists.json`](./stable_text_column_transient_assists.json) | `editor_stack_case_record` | Diagnostics, inlay hints, ghost text, and hover/peek surfaces do not move the source text column. |
| [`state_placement_compare_restore_generated_live.json`](./state_placement_compare_restore_generated_live.json) | `editor_stack_case_record` | Compare, restored, generated, read-only, live-preview, and degraded states surface in the required layers with non-icon cues. |
| [`large_file_degraded_layer_downgrade.json`](./large_file_degraded_layer_downgrade.json) | `editor_stack_case_record` | Large-file and degraded mode narrow, hide, or downgrade layers only with explicit disclosure and replacement routes. |

## Intended usage

- Renderer tests can assert that transient accessories do not alter the
  stable source text column.
- Editor UX review can verify that every accessory maps to one layer and
  one role class.
- Large-file and degraded-mode tests can verify layer-specific
  downgrade disclosure and support-export continuity.
- Accessibility tests can confirm that compact or hidden state has a
  keyboard-reachable detail route and is not icon-only.
