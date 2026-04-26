# Onboarding, learning-mode, guided-tour, and presentation-mode worked fixtures

Worked-example fixtures for the onboarding, learning-mode, guided-tour,
architecture-map, and presentation-mode contract frozen in
[`/docs/ux/learning_and_presentation_contract.md`](../../../docs/ux/learning_and_presentation_contract.md)
and validated by the three companion schemas:

- [`/schemas/ux/guided_tour.schema.json`](../../../schemas/ux/guided_tour.schema.json)
- [`/schemas/ux/learning_progress.schema.json`](../../../schemas/ux/learning_progress.schema.json)
- [`/schemas/ux/presentation_mode_state.schema.json`](../../../schemas/ux/presentation_mode_state.schema.json)

Each YAML file is a scenario manifest: a `__fixture__` envelope plus
a `records:` array whose items each validate as one of the record
kinds declared by the relevant schema's `oneOf`. The fixtures carry
only opaque ids, typed vocabulary, short privacy-safe labels,
monotonic placeholder timestamps, and opaque policy-bundle refs —
no raw URLs, no raw absolute paths, no raw prompt / completion
text, no raw audience identifiers, and no raw cryptographic
material.

## Index

| Fixture                                                            | Schema                                  | Exercises                                                                                                          |
|--------------------------------------------------------------------|-----------------------------------------|--------------------------------------------------------------------------------------------------------------------|
| `first_run_no_account_tour.yaml`                                   | `guided_tour.schema.json`               | First-run no-account guided tour over `individual_local` + `self_hosted`; two waypoints citing canonical commands.|
| `tour_over_partial_indexing.yaml`                                  | `guided_tour.schema.json`               | Learning-mode tour over a partial index, disclosed in the freshness chip.                                          |
| `glossary_card_source_language_fallback.yaml`                      | `guided_tour.schema.json`               | Glossary-card waypoint falling back from `de-DE` to source language with disclosure.                              |
| `exercise_step_reversible_scope.yaml`                              | `guided_tour.schema.json` + `learning_progress.schema.json` | Exercise-step waypoint with portable per-profile progress.                                                         |
| `presentation_session_shared_follow_disabled.yaml`                 | `presentation_mode_state.schema.json`   | Local-only walkthrough; no follow target; audience-visibility = local view only.                                   |
| `presentation_session_shared_follow_enabled.yaml`                  | `presentation_mode_state.schema.json`   | Shared-to-named-audience session with active follow grant; three canonical layout anchors.                         |
| `presentation_breakaway_and_waypoint_navigation.yaml`              | `presentation_mode_state.schema.json`   | Audience breakaway with manual-rejoin recovery; pinned follow target after recovery.                               |
| `learning_progress_manifest_individual_learner.yaml`               | `learning_progress.schema.json`         | Default individual-learner progress manifest with all eight const-true invariants.                                |
| `presentation_invariant_manifest.yaml`                             | `presentation_mode_state.schema.json`   | Presentation-mode invariant manifest with all twelve const-true invariants.                                        |

Every fixture cites the contract sections it exercises and binds
each axis by reference rather than redefinition.
