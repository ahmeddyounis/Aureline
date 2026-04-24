# Learnability / guided-surface worked fixtures

Worked-example fixtures for the learnability, guided-surface, and
citation-backed explainer contract frozen in
[`/docs/ux/learnability_contract.md`](../../../docs/ux/learnability_contract.md)
and validated by
[`/schemas/ux/guided_surface_state.schema.json`](../../../schemas/ux/guided_surface_state.schema.json).

Each fixture validates against the boundary schema as one of three
record kinds: `guided_surface_state_record`,
`guided_surface_rule_record`, or `learning_mode_profile_record`. The
fixtures carry only opaque ids, typed vocabulary, short privacy-safe
labels, monotonic placeholder timestamps, and opaque policy-bundle
refs — no raw URLs, no raw absolute paths, no raw prompt / completion
text, no raw cryptographic material.

## Index

| Fixture                                                                    | Record kind                          | Exercises                                                                                      |
|----------------------------------------------------------------------------|--------------------------------------|------------------------------------------------------------------------------------------------|
| `onboarding_card_open_folder_command_anchored.json`                        | `guided_surface_state_record`        | Onboarding card anchored to a canonical `command_id`, portable dismissal, live authoritative. |
| `glossary_card_docs_anchor_backed.json`                                    | `guided_surface_state_record`        | Glossary card backed by a `glossary_term_anchor`; per-session dismissal.                      |
| `guided_tour_step_command_and_docs_anchor.json`                            | `guided_surface_state_record`        | Guided-tour step citing a canonical command plus docs anchor; measurement linkage.            |
| `architecture_explainer_symbol_reference.json`                             | `guided_surface_state_record`        | Architecture explainer backed by a symbol-linked reference with derived upstream anchors.     |
| `contextual_tip_keymap_bridge.json`                                        | `guided_surface_state_record`        | Contextual tip backed by a keymap bridge resolver; per-device dismissal.                      |
| `exercise_step_blocked_by_trust.json`                                      | `guided_surface_state_record`        | Exercise step suppressed until workspace trust is acknowledged.                                |
| `policy_disabled_learning_surface_suppressed.json`                         | `guided_surface_state_record`        | Learning surface suppressed under an active policy bundle that disables learning surfaces.    |
| `speaker_note_adjunct_preserves_command_identity.json`                     | `guided_surface_state_record`        | Presentation speaker-note adjunct that preserves the command id + docs anchor.                |
| `guided_surface_rule_onboarding_card.json`                                 | `guided_surface_rule_record`         | Rule row for onboarding-card surface kind.                                                     |
| `guided_surface_rule_glossary_card.json`                                   | `guided_surface_rule_record`         | Rule row for glossary-card surface kind.                                                       |
| `guided_surface_rule_exercise_step.json`                                   | `guided_surface_rule_record`         | Rule row for exercise-step surface kind with trust requirement.                                |
| `learning_mode_profile_default_individual_learner.json`                    | `learning_mode_profile_record`       | Default individual-learner profile declaring every const-true invariant.                      |

Every fixture cites the contract sections it exercises and binds each
axis by reference rather than redefinition.
