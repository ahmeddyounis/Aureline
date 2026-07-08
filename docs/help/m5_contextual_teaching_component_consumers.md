# M5 contextual-teaching component consumers

Status: Stable · Schema `schemas/ui/m5-contextual-teaching-component-consumer.schema.json` · Record kind `add_shared_onboarding_help_importer_keybinding_modal_command_doc_consumers_so_contextual_teaching_components_keep_mapping_enablement_source_language_truth_aligned_across_claimed_m5_profiles`

This is the **adoption lane** over the frozen M5 contextual-teaching / migration-bridge
component matrix
(`docs/help/m5_contextual_teaching_migration_bridge_component_matrix.md`). The matrix freezes
five governed component families and four sibling implement lanes narrow them into working
primitives:

| Component family | Narrowed primitive | Canonical schema |
| --- | --- | --- |
| `contextual_tip_card` | contextual-tip card | `schemas/ui/m5-contextual-tip-card.schema.json` |
| `migration_bridge_card` | migration-bridge card | `schemas/ui/m5-migration-bridge-card.schema.json` |
| `sequence_help_strip` | sequence-help strip | `schemas/ui/m5-sequence-help-strip.schema.json` |
| `why_unavailable_explanation_row` | why-unavailable / source-language row | `schemas/ui/m5-why-unavailable-source-language.schema.json` |
| `source_language_fallback` | why-unavailable / source-language row | `schemas/ui/m5-why-unavailable-source-language.schema.json` |

This lane proves those five families are **reusable components** — not one onboarding page
plus a few isolated help objects — by binding every claimed M5 teaching consumer to the same
canonical component schemas and the same descriptor vocabulary.

## Consumers

| Consumer | Token | Role |
| --- | --- | --- |
| First-Run Onboarding | `onboarding_flow` | teaches commands and sequences on first run |
| Migration Importer | `migration_importer` | discloses exact / native / bridge / partial imported behavior |
| Keybinding / Leader Help | `keybinding_leader_help` | maps old keybindings to new commands, keyboard-first |
| Command Docs | `command_docs` | documents the same command / blocked-action / citation truth the product renders |
| Help Pane | `help_pane` | explains blocked actions and localized fallback in place |
| Localized Support Packet | `localized_support_packet` | the authoritative rendering; references the canonical schemas so its prose can never drift |

Every family is adopted by **at least two** distinct consumers, and the localized support
packet references the canonical schema for every family it adopts.

## Shared descriptor vocabulary

The acceptance criterion is one truth for **command binding, migration mapping, blocked-action
explanation, and source-language citation** across every teaching surface. Those four
descriptors (`command_binding`, `migration_mapping`, `blocked_action_explanation`,
`source_language_citation`) are required on every binding, so users no longer see one
explanation in the importer, another in keyboard help, and a third in docs / help for the same
command or limitation.

## Auto-narrowing

A consumer that cannot preserve full parity **auto-narrows** its claim language and always
discloses a self-contained banner naming the exact reason and the recovery action — never a
generic "degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `imported_behavior_partial_narrowed` | `imported_behavior_partial` | `review_migration_mapping_before_trusting` | `imported_behavior_partial_not_exact` |
| `sequence_unsupported_narrowed` | `sequence_unsupported` | `open_full_cheat_sheet_for_supported_sequence` | `sequence_unsupported_no_backing_command` |
| `blocked_owner_changed_narrowed` | `blocked_action_owner_changed` | `contact_current_blocking_owner` | `blocked_action_owner_reassigned` |
| `localized_fallback_stale_narrowed` | `localized_fallback_stale_or_policy_limited` | `view_source_language_or_request_localization` | `localized_fallback_stale_or_policy_limited` |

### Partial / unsupported state is never exact teaching parity

`imported_behavior_partial` and `sequence_unsupported` reflect **partial or unsupported**
behavior. The resolver marks such a binding `reflects_partial_or_unsupported_state = true`,
always narrows it, and always resolves `asserts_exact_teaching_parity = false`. Only a
full-parity binding may assert exact teaching parity. This is the acceptance criterion that
partial or unsupported state no longer masquerades as exact teaching parity on any claimed M5
teaching consumer.

## Resolver

`resolve_teaching_component_binding` takes one consumer's adoption of one component family, the
descriptor set it surfaces, the parity-health mode, and any export caveats, and produces one
`M5TeachingComponentResolvedBinding`. It rejects an empty or incomplete descriptor set and any
forbidden binding material, preserves the descriptor vocabulary at full parity, auto-narrows
under any weakened mode, and — when narrowed — emits a self-contained banner.

## Governance & proof

The checked support export, matrix CSV, and Markdown report live under
`artifacts/release/m5-contextual-teaching-component-consumer-proof/`, and the two narrowed
fixtures (migration-importer → Beta, help-pane → Preview) live under
`fixtures/ui/m5-contextual-teaching-component-consumers/`. All are minted only by the
`aureline_learning_m5_contextual_teaching_component_consumers` headless emitter so the in-code
matrix, the artifact, the worked bindings, and the fixtures never drift. Raw secrets,
endpoints, tokens, and raw provider bodies never cross this boundary.
