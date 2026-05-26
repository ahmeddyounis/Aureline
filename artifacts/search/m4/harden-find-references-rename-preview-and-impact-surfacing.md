# Navigation-target truth packet — stable proof artifact

This artifact certifies the M4 stable lane for find-references,
rename-preview, and impact-surfacing rows across the launch languages.
The reviewer-facing contract lives at
[`docs/search/m4/harden-find-references-rename-preview-and-impact-surfacing.md`](../../../docs/search/m4/harden-find-references-rename-preview-and-impact-surfacing.md);
the boundary schema lives at
[`schemas/search/navigation_target_truth_packet.schema.json`](../../../schemas/search/navigation_target_truth_packet.schema.json);
the protected fixture corpus lives at
[`fixtures/search/m4/navigation_target_truth_packet/`](../../../fixtures/search/m4/navigation_target_truth_packet/).

## Proof summary

The checked-in stable packet at
[`navigation_target_truth_packet.json`](navigation_target_truth_packet.json)
covers every governed navigation row class across the launch
languages and pins how relation kind, access kind, provider class,
freshness, ambiguity, scope completeness, downgrade state, and
confidence are projected to every consumer surface.

Row classes asserted by the packet:

| Row class | Language lane example | Provider class | Downgrade state |
| --- | --- | --- | --- |
| `definition` | Rust | `language_server` | `canonical` |
| `declaration` | TypeScript | `language_server` | `canonical` |
| `implementation` | Python | `project_graph` | `canonical` |
| `reference` (`read`) | Rust | `language_server` | `canonical` |
| `reference` (`write`) | Rust | `language_server` | `canonical` |
| `reference` (`call`) | TypeScript | `language_server` | `canonical` |
| `reference` (`inherit`) | Python | `project_graph` | `canonical` |
| `reference` (`import`) | Rust | `language_server` | `canonical` |
| `reference` (`export`) | TypeScript | `language_server` | `canonical` |
| `reference` (`route_binding`) | TypeScript | `framework_pack` | `runtime_or_framework_only_disclosed` |
| `reference` (`test_only`) | Rust | `language_server` | `canonical` |
| `reference` (`generated`) | Rust | `generated_source_bridge` | `generated_boundary_disclosed` |
| `reference` (`runtime_observed`) | Python | `runtime_observer` | `runtime_or_framework_only_disclosed` |
| `call_hierarchy_edge` | Rust | `language_server` | `canonical` |
| `type_hierarchy_edge` | TypeScript | `language_server` | `canonical` |
| `related_object` (`documented_by`) | Rust | `project_graph` | `canonical` |
| `rename_preview` | Rust | `language_server` | `partial_index_disclosed` |

Every reference row preserves a closed access kind from
`{read, write, call, inherit, import, export, route_binding,
test_only, generated, runtime_observed}`.

Every required consumer projection is present and preserves the
packet verbatim:

- `editor_navigation_pane`
- `graph_topology`
- `ai_context`
- `review_workspace`
- `support_export`
- `cli_headless`
- `release_proof_index`

## Below-stable refusal proof

The fixture corpus pairs the baseline stable case with five
narrowed-below-stable cases that the validator must refuse:

- `silent_relation_alias_blocks_stable.json` — a definition row that
  reports `relation_kind=declaration` while staying `canonical`.
- `reference_missing_access_context_blocks_stable.json` — a reference
  row that drops its `reference_context` (and therefore the
  access-kind label).
- `aliased_due_to_shallow_provider_missing_context_blocks_stable.json`
  — an `aliased_due_to_shallow_provider` row that drops its
  aliasing context.
- `consumer_projection_drops_access_kind_blocks_stable.json` — an
  `ai_context` consumer projection that drops the access-kind
  vocabulary.
- `rename_preview_missing_context_blocks_stable.json` — a rename
  preview row that drops its rename_preview_context.

Each narrowed case carries an `expect.expected_finding_kinds` list
the integration tests assert against, so a regression that re-enables
silent aliasing, drops an access kind, or hides rename blocked
candidates from a consumer surface immediately fails the protected
fixture suite.

## Cross-checked references

- Schema: [`schemas/search/navigation_target_truth_packet.schema.json`](../../../schemas/search/navigation_target_truth_packet.schema.json)
- Contract doc: [`docs/search/m4/harden-find-references-rename-preview-and-impact-surfacing.md`](../../../docs/search/m4/harden-find-references-rename-preview-and-impact-surfacing.md)
- Fixture corpus: [`fixtures/search/m4/navigation_target_truth_packet/`](../../../fixtures/search/m4/navigation_target_truth_packet/)
- Rust module: [`crates/aureline-graph/src/navigation_target_truth_packet/`](../../../crates/aureline-graph/src/navigation_target_truth_packet/)
- Integration test: [`crates/aureline-graph/tests/navigation_target_truth_packet.rs`](../../../crates/aureline-graph/tests/navigation_target_truth_packet.rs)
