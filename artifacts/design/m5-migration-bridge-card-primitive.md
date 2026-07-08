# M5 Migration-Bridge-Card Primitive

- Packet: `m5-migration-bridge-card-primitive:stable:0001`
- Label: `M5 migration-bridge-card primitive: migration mapping class, imported source tool, old-path reference, new-command reference, affected scope, unsupported edge cases, import rollback linkage, derived bridge posture (exact-parity/native-equivalent/bridged-approximation/shimmed-compatibility/partial-coverage/unsupported-no-mapping), and bounded view-mapping-details/open-native-command/undo-import-changes/review-import-checkpoint/report-unsupported-edge-case actions`
- Importer / migration consumers: 5 (5 stable)
- Bridge postures: exact_parity, native_equivalent, bridged_approximation, shimmed_compatibility, partial_coverage, unsupported_no_mapping
- Bridge actions: view_mapping_details, open_native_command, undo_import_changes, review_import_checkpoint, report_unsupported_edge_case
- Mapping classes: exact, native, bridge, shimmed, partial, unsupported
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Importer / migration consumers

- **Migration Report Panel**: `stable`
  - Owner: Migration report panel owner
  - Scope: The migration report panel renders the shared migration bridge card so an exact one-to-one mapping the import applied durably is shown as exact parity with the old shortcut, the new command, the affected scope, and available undo / review actions, and an unsupported behavior with no native command is shown honestly as unsupported-no-mapping — never implied to be parity — with its uncovered edge cases named and a report action
  - Worked bridges: 2
    - `bridge:migration-report:command-palette` (`exact` / `rival_ide`) → `exact_parity` (exact-parity `true`, durable `true`, undo `true`)
    - `bridge:migration-report:vimscript-source` (`unsupported` / `modal_editor`) → `unsupported_no_mapping` (exact-parity `false`, durable `false`, undo `false`)
- **Import Diff Row**: `stable`
  - Owner: Import diff row owner
  - Scope: The import diff row renders the shared migration bridge card so a native Aureline equivalent applied durably is shown as native-equivalent with undo / review actions, and a partial mapping applied durably is shown honestly as partial-coverage — never as exact parity — naming the imported keys it does cover, the affected scope, the edge cases it does not cover, and undo / review / report actions
  - Worked bridges: 2
    - `bridge:import-diff:format-document` (`native` / `legacy_editor`) → `native_equivalent` (exact-parity `false`, durable `true`, undo `true`)
    - `bridge:import-diff:repeat-count-motion` (`partial` / `imported_keymap`) → `partial_coverage` (exact-parity `false`, durable `true`, undo `true`)
- **First-Run Switch Summary**: `stable`
  - Owner: First-run switch summary owner
  - Scope: The first-run switch summary renders the shared migration bridge card so a bridge that only approximates the imported behavior and changed nothing durable is shown honestly as bridged-approximation — offering inspect and open-native actions but no undo it does not need — and a shimmed compatibility applied durably is shown as shimmed-compatibility with its shim edge cases named and undo / review / report actions available
  - Worked bridges: 2
    - `bridge:first-run:multi-cursor-gesture` (`bridge` / `rival_ide`) → `bridged_approximation` (exact-parity `false`, durable `false`, undo `false`)
    - `bridge:first-run:build-task-shim` (`shimmed` / `migrated_workflow_config`) → `shimmed_compatibility` (exact-parity `false`, durable `true`, undo `true`)
- **Keybinding Migration Notice**: `stable`
  - Owner: Keybinding migration notice owner
  - Scope: The keybinding migration notice renders the shared migration bridge card so a partial keybinding mapping applied durably is shown honestly as partial-coverage with the edge cases it does not cover named and undo / review / report actions available, and a native keybinding equivalent applied durably is shown as native-equivalent with undo / review actions — every durable keybinding import keeps its undo path
  - Worked bridges: 2
    - `bridge:keybinding:visual-block` (`partial` / `modal_editor`) → `partial_coverage` (exact-parity `false`, durable `true`, undo `true`)
    - `bridge:keybinding:rename-symbol` (`native` / `rival_ide`) → `native_equivalent` (exact-parity `false`, durable `true`, undo `true`)
- **Support Migration Export**: `stable`
  - Owner: Support migration export owner
  - Scope: The support migration export renders the shared migration bridge card so a bridge approximation that changed a durable snippet is shown as bridged-approximation with undo / review actions still available — proving undo survives export for an approximated durable change — and an unsupported behavior with no native command is exported honestly as unsupported-no-mapping with its uncovered edge cases intact and no raw imported config leaking
  - Worked bridges: 2
    - `bridge:support:snippet-expansion` (`bridge` / `legacy_editor`) → `bridged_approximation` (exact-parity `false`, durable `true`, undo `true`)
    - `bridge:support:proprietary-plugin-hook` (`unsupported` / `unknown_source`) → `unsupported_no_mapping` (exact-parity `false`, durable `false`, undo `false`)
