# M5 migration-bridge-card primitive

The migration bridge card is one of the five governed contextual-teaching / migration-bridge
component families frozen by the
[M5 contextual-teaching / migration-bridge component matrix](../help/m5_contextual_teaching_migration_bridge_component_matrix.md).
This primitive narrows that family into a single reusable resolver,
[`resolve_migration_bridge_card`](../../crates/aureline-learning/src/ship_migration_bridge_cards_with_old_path_new_command_mapping_native_bridge_shimmed_partial_states_and_undo_import_parity_across_claimed_m5_importer_and_migration_surfaces/mod.rs),
so an imported user can understand — from the card alone — exactly how one imported behavior
maps onto Aureline, **without** detached docs or tribal knowledge, and can never mistake an
approximated or partial mapping for exact parity.

## What the resolver decides

Given one imported behavior's migration mapping class, source tool, opaque old-path reference,
optional opaque new-command reference, affected-scope summary, unsupported edge cases, whether
the import created a durable user-facing change, the optional opaque import rollback /
checkpoint reference, and its opaque stable bridge identity, the resolver derives:

- **Bridge posture** — derived one-to-one from the frozen migration mapping class so an
  approximated, shimmed, partial, or unsupported behavior can never be presented as exact
  parity:
  1. `exact_parity` — an exact one-to-one mapping (`exact`).
  2. `native_equivalent` — a native Aureline equivalent (`native`).
  3. `bridged_approximation` — a bridge that only approximates (`bridge`).
  4. `shimmed_compatibility` — supported through a compatibility shim (`shimmed`).
  5. `partial_coverage` — some of the imported behavior is missing (`partial`).
  6. `unsupported_no_mapping` — no mapping exists (`unsupported`).
- **Bounded actions** — every card offers `view_mapping_details` so the mapping can always be
  inspected. A mapped behavior also offers `open_native_command`; a durable import change offers
  `undo_import_changes` (always backed by a rollback checkpoint); an import that created a
  checkpoint offers `review_import_checkpoint`; and an unsupported mapping or any named edge case
  offers `report_unsupported_edge_case`.

Every resolved card also asserts the acceptance-criterion invariants: it
`discloses_old_path_and_new_command`, `discloses_mapping_state_honestly`,
`never_overstates_as_exact_parity`, `preserves_affected_scope`,
`preserves_unsupported_edge_cases`, `preserves_import_rollback_linkage`, and
`keeps_undo_review_available_for_durable_changes`.

A durable import change with **no** rollback linkage is rejected outright
(`durable_change_without_rollback`), so undo / review always stays available wherever an import
changed durable user-facing behavior (settings, keybindings, snippets, or other durable
behavior). An unsupported behavior may not declare a native command
(`native_command_on_unsupported_state`), and a partial or unsupported mapping must name its
uncovered edge cases (`missing_unsupported_edge_cases`).

## Reused vs minted vocabulary

The migration mapping class, imported source tool, surface family, deployment line, teaching
consumer surface, accessibility route, qualification class, and downgrade triggers are reused
verbatim from the frozen component matrix. This primitive mints new vocabulary only for what
that matrix left implicit about the bridge card itself: its importer / migration consumers, its
anatomy parts, its derived bridge posture, its bounded actions, and its export fields. No M5
migration surface invents a second bridge-card grammar.

## Importer / migration consumers

One parity row is bound per claimed M5 importer / migration consumer so the old-path /
new-command / mapping-state / undo-import vocabulary stays identical across desktop,
headless/export, and support consumers:

- Migration Report Panel
- Import Diff Row
- First-Run Switch Summary
- Keybinding Migration Notice
- Support Migration Export

## Source contracts

- `schemas/ui/m5-migration-bridge-card.schema.json` — this primitive's boundary schema.
- `docs/migration/m5_migration_bridge_card_primitive.md` — this contract doc.
- `schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json` — the frozen
  component matrix this primitive narrows from.
- `schemas/migration/importer_outcome.schema.json` — the importer-outcome contract the mapping
  state binds against.
- `schemas/migration/import_rollback_checkpoint.schema.json` — the import rollback / checkpoint
  contract the undo / review action binds against.

## Checked-in evidence

- Support export: `artifacts/release/m5-migration-bridge-card-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-migration-bridge-card-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-migration-bridge-card-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-migration-bridge-card-primitive/`

All evidence is minted from one source of truth by the headless emitter:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- csv
cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- report
cargo run -q -p aureline-learning --bin aureline_learning_m5_migration_bridge_card_primitive -- validate
```
