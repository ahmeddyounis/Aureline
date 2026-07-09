# M5 learning component consumers

Status: Stable · Schema `schemas/ui/m5-learning-component-consumer.schema.json` · Record kind `add_shared_onboarding_migration_contextual_help_docs_browser_feature_family_tour_companion_handoff_and_support_export_consumers_so_learning_components_keep_citation_privacy_and_progress_language_aligned_across_claimed_m5_profiles`

This is the **adoption lane** over the frozen M5 learning-component matrix
(`docs/help/m5_learning_component_matrix.md`). The matrix freezes six governed component families
and three sibling implement lanes narrow them into working primitives:

| Component family | Narrowed primitive | Canonical schema |
| --- | --- | --- |
| `learning_mode_toggle` | learning-mode toggle / tip card | `schemas/ui/m5-learning-mode-toggle-tip-card-controls.schema.json` |
| `tip_card` | learning-mode toggle / tip card | `schemas/ui/m5-learning-mode-toggle-tip-card-controls.schema.json` |
| `guided_exercise_step` | guided-exercise step / progress marker | `schemas/ui/m5-guided-exercise-step-progress-marker-controls.schema.json` |
| `progress_marker` | guided-exercise step / progress marker | `schemas/ui/m5-guided-exercise-step-progress-marker-controls.schema.json` |
| `glossary_chip_or_card` | glossary chip-card / safe-explanation banner | `schemas/ui/m5-glossary-chip-card-safe-explanation-banner-controls.schema.json` |
| `safe_explanation_banner` | glossary chip-card / safe-explanation banner | `schemas/ui/m5-glossary-chip-card-safe-explanation-banner-controls.schema.json` |

This lane proves those six families are **reusable components** — not one onboarding page plus a
few isolated help objects — by binding every claimed M5 learning consumer to the same canonical
component schemas and the same descriptor vocabulary.

## Consumers

| Consumer | Token | Role |
| --- | --- | --- |
| First-Run Onboarding | `onboarding` | teaches learning mode and first tips on first run |
| Migration Onboarding | `migration` | teaches while a user migrates into M5 workflows |
| Contextual Help | `contextual_help` | explains in place with cited, explain-versus-do-bounded help |
| Docs / Browser | `docs_browser` | surfaces glossary and explanation content with cited source truth |
| Feature-Family Tour | `feature_family_tour` | walks a feature family with user-owned, default-local progress |
| Companion Handoff | `companion_handoff` | carries learning truth to a companion surface honestly |
| Support / Export Packet | `support_export` | the authoritative rendering; references the canonical schemas so its prose can never drift |

Every family is adopted by **at least two** distinct consumers, and the support / export packet
references the canonical schema for every family it adopts.

## Shared descriptor vocabulary

The acceptance criterion is one truth for **citation, source-class / freshness, progress ownership
/ privacy, and explain-versus-do** across every teaching surface. Those four descriptors
(`citation_source`, `source_class_freshness`, `progress_ownership_privacy`, `explain_versus_do`)
are required on every binding, so users no longer see one citation in onboarding, another in docs,
and a third in a companion handoff for the same learning object, and progress stays user-owned and
default-local everywhere.

## Auto-narrowing

A consumer that cannot preserve full parity **auto-narrows** its claim language and always
discloses a self-contained banner naming the exact reason and the recovery action — never a generic
"degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `cached_pack_narrowed` | `cached_pack_served` | `refresh_pack_when_online` | `content_served_from_cached_pack` |
| `stale_source_narrowed` | `source_content_stale` | `review_source_freshness_before_trusting` | `source_content_stale` |
| `citation_unavailable_narrowed` | `cited_source_unavailable_or_not_installed` | `open_cited_source_or_request_access` | `cited_source_unavailable_or_not_installed` |
| `progress_local_only_narrowed` | `progress_local_only` | `export_progress_or_enable_supported_sync` | `progress_local_only_not_synced` |

### Uncited / unavailable source is never live cited parity

`cited_source_unavailable_or_not_installed` reflects an **uncited or unavailable** source. The
resolver marks such a binding `reflects_uncited_or_unavailable_source = true`, always narrows it,
and always resolves `asserts_live_cited_parity = false`. Only a full-parity binding may assert a
live, cited source. This is the acceptance criterion that an uncited or unavailable source no
longer masquerades as a live, cited one on any claimed M5 learning consumer.

## Resolver

`resolve_learning_component_binding` takes one consumer's adoption of one component family, the
descriptor set it surfaces, the parity-health mode, and any export caveats, and produces one
`M5LearningComponentResolvedBinding`. It rejects an empty or incomplete descriptor set and any
forbidden binding material, keeps the descriptor vocabulary aligned at full parity, auto-narrows
under any weakened mode, and — when narrowed — emits a self-contained banner.

## Governance & proof

The checked support export, matrix CSV, and Markdown report live under
`artifacts/release/m5-learning-component-consumer-proof/`, and the two narrowed fixtures
(docs / browser → Beta, companion handoff → Preview) live under
`fixtures/ui/m5-learning-component-consumers/`. All are minted only by the
`aureline_learning_m5_learning_component_consumers` headless emitter so the in-code matrix, the
artifact, the worked bindings, and the fixtures never drift. Raw secrets, endpoints, tokens, and
raw provider bodies never cross this boundary.
