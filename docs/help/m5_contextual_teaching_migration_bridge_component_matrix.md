# M5 Contextual-Teaching / Migration-Bridge Component Matrix

This doc describes the frozen M5 **contextual-tip-card**, **migration-bridge-card**,
**sequence-help-strip**, **why-unavailable-explanation-row**, and **source-language-fallback**
component matrix. The matrix is the single source of truth for whether a claimed M5
onboarding, tour, command-help, migration, or localized-help surface may publish a tip, a
migrated-behavior claim, a sequence-help state, a blocked-action explanation, or a
source-language fallback claim.

- **Authoritative validator**: `crates/aureline-learning` module
  `freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`.
- **Schema (shape only)**: `schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json`.
- **Support export (mint-from-truth)**: `artifacts/release/m5-contextual-teaching-proof/support_export.json`.
- **Machine-readable matrix**: `artifacts/release/m5-contextual-teaching-proof/matrix.csv`.
- **Design report**: `artifacts/design/m5-contextual-teaching-migration-bridge-component-matrix.md`.
- **Narrowed fixtures**: `fixtures/ui/m5-contextual-teaching-components/`.

Regenerate every checked-in artifact from truth with the headless emitter:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- csv
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- report
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_teaching_component_matrix -- validate
```

## Component families

| Family | What it names |
| --- | --- |
| `contextual_tip_card` | Why a teaching tip appears (trigger), the stable command that backs it, and how it can be dismissed. |
| `migration_bridge_card` | How an imported behavior maps onto Aureline (`exact`, `native`, `bridge`, `shimmed`, `partial`, `unsupported`) and the source tool it came from. |
| `sequence_help_strip` | The state of a keyboard command sequence, its step kinds, and the stable command that backs it. |
| `why_unavailable_explanation_row` | The owner of a blocked action, the reason it is blocked, and the next safe action. |
| `source_language_fallback` | The localization state of the help shown and how it preserves canonical IDs / citations. |

## Controlled vocabularies

The matrix freezes one controlled vocabulary per governed dimension. The
acceptance-criteria vocabularies are:

- **Migration mapping classes** (imported behavior): `exact`, `native`, `bridge`,
  `shimmed`, `partial`, `unsupported`.
- **Blocked-action owners**: `policy_owner`, `workspace_admin`, `provider_service`,
  `upstream_dependency`, `current_user_scope`, `unknown_owner`.
- **Sequence-help states**: `ready`, `awaiting_next_key`, `partial_match`, `no_binding`,
  `conflicting_binding`, `disabled_in_context`.
- **Source-language / fallback states**: `authored_locale`, `translated_locale`,
  `machine_translated`, `fallback_to_source`, `mixed_locale`, `untranslated_source` and
  `localized_current`, `source_language_shown`, `partial_translation`, `stale_translation`,
  `citation_preserved_fallback`, `no_localization`.

No claimed M5 onboarding/help surface may invent an alternate label for imported behavior,
a blocked action, or a source-language fallback state.

## Hard invariants

Every governed component row asserts, as `false`, that it never:

1. masks its command binding or migration mapping,
2. hides a blocked-action owner or reason,
3. invents an alternate label for a governed state, or
4. severs a source-language citation.

Teaching stays contextual, dismissible, command-backed, and non-authoritative; migrated
behavior discloses its exact/native/bridge/partial state; blocked actions name owner,
reason, and next safe action; sequence help stays keyboard-first; and localized help never
severs canonical IDs / citations.

## Source contracts

The matrix layers on top of, and binds against, these pre-existing contracts:

- `schemas/commands/command_descriptor.schema.json` (stable command IDs).
- `schemas/migration/importer_outcome.schema.json` (importer outcomes).
- `schemas/commands/keybinding_resolver.schema.json` (keybinding / sequence state).
- `schemas/ux/feature_availability_row.schema.json` (enablement / availability reasons).
- `schemas/ux/locale_fallback_state.schema.json` (source-language fallback state).
