# M5 Badge-Family Consumer Contract

This contract is the adoption lane over Aureline's frozen M5 badge families. It
proves that the six governed badge families — **support class**, **evidence
freshness**, **lifecycle**, **channel**, **deployment scope**, and **compatibility
state** — are reusable cross-product cues rather than release-center-only or
ecosystem-only concepts, by binding every claimed M5 badge consumer to the same
canonical badge schemas and the same label / explanation / downgrade-reason /
filter-key parity vocabulary.

- Boundary schema: `schemas/ui/m5-badge-family-consumer.schema.json`
- Support export: `artifacts/release/m5-badge-family-consumer-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-badge-family-consumer-proof/matrix.csv`
- Markdown report: `artifacts/components/m5-badge-family-consumer.md`
- Protected fixtures: `fixtures/ui/m5-badge-family-consumers/`

## Source of truth

The frozen badge-family matrix
(`schemas/ui/m5-badge-family-matrix.schema.json`) names the six governed badge
families, and four narrowed primitive lanes own their working badge resolvers and
canonical schemas:

| Badge family | Canonical schema |
| --- | --- |
| support class | `schemas/ui/m5-support-class-and-evidence-freshness-badge.schema.json` |
| evidence freshness | `schemas/ui/m5-support-class-and-evidence-freshness-badge.schema.json` |
| lifecycle | `schemas/ui/m5-lifecycle-and-channel-badge.schema.json` |
| channel | `schemas/ui/m5-lifecycle-and-channel-badge.schema.json` |
| deployment scope | `schemas/ui/m5-deployment-scope-badge.schema.json` |
| compatibility state | `schemas/ui/m5-compatibility-state-badge.schema.json` |

Each consumer points at the canonical schema for the families it adopts rather than
re-wording the label, explanation, or downgrade reason in local prose.

## Consumers

The acceptance criteria name eight badge consumers, each bound in the matrix:

1. **Marketplace / Install** — support-class and lifecycle badges.
2. **Help / About** — evidence-freshness and support-class badges; references the
   canonical schema so its prose can never drift.
3. **Settings / Policy** — deployment-scope and channel badges in the policy
   explainers.
4. **Onboarding / Start Center** — lifecycle badge, plus an evidence-freshness
   badge that auto-narrows when its evidence goes stale.
5. **Diagnostics** — evidence-freshness auto-narrowed on stale proof, plus a
   compatibility-state badge auto-narrowed when its scope reduces.
6. **Support Export** — support-class and compatibility-state badges from an export
   snapshot; references the canonical schema so the export never loses badge
   meaning.
7. **Runtime / Deployment** — deployment-scope auto-narrowed on reduced scope, plus
   a channel badge at full claim scope.
8. **Workspace / Archetype** — compatibility-state and deployment-scope badges.

## Parity vocabulary

Every binding preserves the four shared parity facets: **label**, **explanation**,
**downgrade reason**, and **filter key**. A badge stays a compact contract with
stable labels, explanation drawers, downgrade rules, and separately filterable
axes — no axis is collapsed into an overloaded pill, freshness is never implied
from support class, and exported evidence never loses badge meaning.

## Render modes and auto-narrowing

A badge renders either at **full claim scope** or auto-narrowed. Each narrowed mode
maps to one exact downgrade reason and next action, disclosed on a self-contained
narrow banner (never a generic "reduced" note):

| Render mode | Narrow reason | Next action |
| --- | --- | --- |
| `full_claim_scope` | — | — |
| `freshness_narrowed` | `evidence_stale` | `refresh_stale_evidence` |
| `scope_narrowed` | `scope_reduced` | `review_narrowed_scope` |
| `export_projection` | `export_snapshot` | `open_live_badge_surface` |

The banner keeps the parity vocabulary intact and names the preserved facets, so a
narrowed badge is understood from the banner alone.

## Acceptance criteria

- **The same badge family means the same thing across claimed M5 profiles and
  consumers, with no surface-local reinterpretation.** Every family is adopted by
  at least two distinct consumers, and every binding points at the family's
  canonical schema and artifact rather than local prose (`badge_family_reuse_unproven`,
  `canonical_ref_mismatch`).
- **Docs/help/support exports preserve the same label, explanation, and downgrade
  reason the live UI shows.** The Help/About and support-export consumers must
  reference the canonical badge schema for each family they adopt
  (`docs_help_reference_missing`), and the support export reconstructs consumer
  parity from the shared model.

Regenerate all artifacts from the single seed builder with:

```sh
cargo run -q -p aureline-release --bin aureline_release_add_shared_marketplace_help_settings_onboarding_diagnostics_export_runtime_and_workspace_consumers_so_badge_families_keep_label_explanation_and_downgrade_parity_across_claimed_m5_profiles -- support-export
```
