# Rail/sidebar section-slot fixtures

Seed corpus for the contract frozen in
[`/docs/ux/rail_sidebar_contract.md`](../../../docs/ux/rail_sidebar_contract.md)
and the schema at
[`/schemas/ux/section_slot.schema.json`](../../../schemas/ux/section_slot.schema.json).

Each file is a single JSON document validating against one of the four
record kinds in the schema:

- `section_slot_catalog_record`
- `section_slot_record`
- `sidebar_visibility_state_record`
- `section_surface_placement_decision_record`

Every fixture:

- uses only the closed section ids, rail slot classes, placement
  lanes, visibility classes, and remembered-state classes from the
  contract;
- records command-backed equivalents for hide/show, direct section
  jumps, overflow, or critical actions where applicable;
- keeps raw paths, raw URLs, raw credentials, raw provider payloads,
  and raw user content out of the record;
- names the contract sections it exercises under
  `__fixture__.contract_sections`.

## Cases

| Fixture | Record kind | Scenario axis | Contract anchor |
|---|---|---|---|
| [`canonical_top_level_sections.json`](./canonical_top_level_sections.json) | `section_slot_catalog_record` | Seven primary ranks, conditional Collaboration and Support/Admin ranks, future rows routed to overflow, hide/show/direct-jump command refs. | 4, 5, 8, 10 |
| [`explorer_section_slot.json`](./explorer_section_slot.json) | `section_slot_record` | Explorer as the default structural browsing owner with file operations routed through commands/review rather than private sidebar-only actions. | 5, 6, 9, 11.1 |
| [`run_test_section_slot.json`](./run_test_section_slot.json) | `section_slot_record` | Run/Test owns target browsing while output, debug console, and evidence views route to bottom panel or main workspace. | 7, 11.4 |
| [`hidden_search_reopen_last_state.json`](./hidden_search_reopen_last_state.json) | `sidebar_visibility_state_record` | Hidden Search sidebar preserves query/scope/scroll/focus state and has rail, palette, and menu return routes. | 8, 9 |
| [`future_extension_overflow_decision.json`](./future_extension_overflow_decision.json) | `section_surface_placement_decision_record` | Extension-contributed dependency view denied primary rail placement and routed to overflow/existing section instead of creating an eighth primary section. | 4, 10, 11.8 |
| [`source_control_critical_action_parity.json`](./source_control_critical_action_parity.json) | `section_surface_placement_decision_record` | Push/publish-style Source Control action cannot exist only in a sidebar row; command-backed parity and review routing are required. | 6, 8, 11.3, 12 |

## Schema reference

- Section-slot schema:
  [`/schemas/ux/section_slot.schema.json`](../../../schemas/ux/section_slot.schema.json).

