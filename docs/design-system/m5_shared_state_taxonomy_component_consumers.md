# M5 shared-state-taxonomy component consumers

Status: Stable · Schema `schemas/ui/m5-shared-state-taxonomy-component-consumer.schema.json` · Record kind `add_shared_shell_command_search_review_settings_provider_test_and_support_consumers_so_state_taxonomy_components_keep_label_recovery_and_accessibility_parity_across_claimed_m5_profiles`

This is the **adoption lane** over the frozen M5 shared-component-state-taxonomy component
matrix (`docs/design-system/m5-shared-component-state-taxonomy-component-matrix.md`). The matrix
freezes four governed component families, and three sibling implement lanes narrow the last
three families into working primitives while the shared taxonomy itself is the frozen matrix:

| Component family | Owning contract | Canonical schema |
| --- | --- | --- |
| `shared_component_state_taxonomy` | shared component-state taxonomy (frozen matrix) | `schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json` |
| `interactive_state` | interactive-state contract | `schemas/ui/m5-interactive-state-contract.schema.json` |
| `selection_or_lock_state` | selection-or-lock-state contract | `schemas/ui/m5-selection-lock-state-contract.schema.json` |
| `degraded_state_application` | loading / pending / degraded state-application contract | `schemas/ui/m5-loading-pending-degraded-state-contract.schema.json` |

This lane proves those four families are **reusable state contracts** — not a design-system
island — by binding every claimed M5 consumer to the same canonical contract schemas and the
same descriptor vocabulary.

## Consumers

| Consumer | Token | Role |
| --- | --- | --- |
| Shell Chrome | `shell_chrome` | status bar, panes, progress affordances |
| Command / Help | `command_help` | command palette and Help pane |
| Search / Dense Collection | `search_dense_collection` | lists, trees, grids |
| Review / Work-Item | `review_work_item` | review and work-item flows |
| Settings / Capability | `settings_capability` | settings and capability prompts |
| Provider / Offline-Capture | `provider_offline_capture` | provider and offline-capture rows |
| Test / Watch | `test_watch` | test-run and watch surfaces |
| Support / Recovery | `support_recovery` | the authoritative rendering; references the canonical schemas so its exported prose can never drift |

Every family is adopted by **at least two** distinct consumers, and the support / recovery lane
references the canonical schema for every family it adopts.

## Shared descriptor vocabulary

The acceptance criterion is one truth for **state semantics, state cause, consequence /
recovery, and the accessibility label** across every claimed M5 profile, surface, and export
path. Those four descriptors (`state_semantics`, `state_cause`, `consequence_and_recovery`,
`accessibility_label`) are required on every binding, so the same state family reads and behaves
consistently across shell chrome, commands, dense collections, reviews, settings, provider rows,
test surfaces, and the support / recovery lane — and support and docs / help explain the same
state cause / recovery truth the live surface shows without cloning divergent copy.

## Auto-narrowing

A consumer that cannot preserve full parity **auto-narrows** its claim language and always
discloses a self-contained banner naming the exact reason and the recovery action — never a
generic "degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `state_cause_unresolved_narrowed` | `state_cause_unresolved` | `resolve_state_cause_before_trusting` | `state_cause_unresolved_not_explained` |
| `recovery_unavailable_narrowed` | `recovery_unavailable` | `follow_disclosed_recovery_path` | `recovery_unavailable_degraded` |
| `lock_owner_unresolved_narrowed` | `lock_owner_unresolved` | `contact_current_lock_owner` | `lock_owner_reassigned` |
| `accessibility_route_reduced_narrowed` | `accessibility_route_reduced` | `open_full_accessible_state_description` | `accessibility_route_reduced_fallback` |

### Incomplete / degraded state is never an exact, healthy state

`state_cause_unresolved` and `recovery_unavailable` reflect an **incomplete or degraded** state.
The resolver marks such a binding `reflects_incomplete_or_degraded_state = true`, always narrows
it, and always resolves `asserts_exact_state_parity = false`. Only a full-parity binding may
assert exact state parity. This is the acceptance criterion that an incomplete or degraded state
no longer masquerades as an exact, healthy state on any claimed M5 consumer. A re-resolved lock
owner and a reduced accessibility route still narrow visibly, but stay explainable rather than
incomplete.

## Resolver

`resolve_state_component_binding` takes one consumer's adoption of one component family, the
descriptor set it surfaces, the parity-health mode, and any export caveats, and produces one
`M5StateComponentResolvedBinding`. It rejects an empty or incomplete descriptor set and any
forbidden binding material, preserves the descriptor vocabulary at full parity, auto-narrows
under any weakened mode, and — when narrowed — emits a self-contained banner.

## Governance & proof

The checked support export, matrix CSV, and Markdown report live under
`artifacts/release/m5-shared-state-taxonomy-component-consumer-proof/`, and the two narrowed
fixtures (provider / offline-capture → Beta, test / watch → Preview) live under
`fixtures/ui/m5-shared-state-taxonomy-component-consumers/`. All are minted only by the
`aureline_design_system_m5_shared_state_taxonomy_component_consumers` headless emitter so the
in-code matrix, the artifact, the worked bindings, and the fixtures never drift. Raw secrets,
endpoints, tokens, and raw provider bodies never cross this boundary.
