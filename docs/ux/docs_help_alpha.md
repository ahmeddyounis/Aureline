# Docs/Help Alpha Contract

The docs/help alpha surface is a bounded search and truth projection over existing product contracts. It does not create a second command vocabulary: command-backed help rows quote the command registry command id, command revision, and `docs_help_anchor_ref` anchor.

## Source Model

Every result card and About/service-health row carries the same descriptor shape:

- Source class and source revision.
- Display source version and running build identity when build-bound.
- Version match, freshness, support class, client scope, destination trust class, and contract state.
- Locale availability and source-language fallback.
- Offline posture and local-only continuity limit.
- Citation availability, citation anchors, and an open-details target.
- Exact reopen target for commands, docs nodes, product surfaces, browser handoffs, citation drawers, and evidence cards.

The controlled contract states are `ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`, `policy_blocked`, and `unavailable`. These states are shared by help search, About, service health, and support export rows.

## Content Classes

The alpha lane names the content class on every row:

- `canonical_help`
- `generated_suggestion`
- `stale_example_warning`
- `publish_boundary_handoff`
- `migration_hint`
- `service_health_note`
- `about_service_health_truth`

Generated suggestions and stale-example warnings remain explicitly publish-boundary aware. They can be inspected and exported, but they do not imply publication until validation and owner review clear.

## Reopen And Export

Search results preserve stable docs-node ids, help anchors, owning command ids, destination descriptor refs, and citation anchors. Support exports reduce the same objects to copy-safe rows that still reconstruct source, version, freshness, locale fallback, offline posture, citation state, and exact reopen identity without requiring sign-in.

The seed fixtures live under `fixtures/docs/help_search_alpha/` and `fixtures/docs/docs_help_alpha/`. The proof artifact is `artifacts/docs/help_search_alpha.yaml`.
