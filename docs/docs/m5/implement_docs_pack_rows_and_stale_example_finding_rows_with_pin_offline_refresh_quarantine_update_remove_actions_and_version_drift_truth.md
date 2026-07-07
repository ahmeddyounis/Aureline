# M5 docs-pack row & stale-example finding row primitive (M05-872)

Two reusable M5 docs-lifecycle primitives — the **docs-pack row** and the
**stale-example finding row** — projected the same way across every claimed M5
docs-manager, help-pack, onboarding, AI-context, and support surface a user reaches when
they manage documentation packs or inspect example drift.

This module *implements* two families that the frozen docs-browser component matrix
(`freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`,
M05-868) named and froze: `docs_pack_row` and `stale_example_finding_row`. It narrows
their frozen vocabulary into working primitives with real resolvers, so pack lifecycle
and example drift become first-class inspectable decisions rather than a generic warning.

## What it provides

1. **A pack resolver — [`resolve_docs_pack_row`]** — takes one pack's identity, selected
   scope, size/count, signer/source, pin/mirror/offline/quarantine state, refresh time,
   and verification state, and derives:
   - a **pack trust posture** (`pinned_current`, `tracking_current`,
     `mirror_served_not_live`, `offline_only`, `update_overdue`, `stale_needs_refresh`,
     `quarantined_untrusted`, `verification_unverified`) via a fixed, honesty-first ladder
     — quarantine and verification failure come first, so a quarantined, stale, mirrored,
     or offline pack keeps a **distinct** posture and never collapses into one generic
     warning or reads as live;
   - the **pin / offline / refresh / quarantine-review / update / remove** actions the
     pack allows, plus an always-available `export_pack_manifest` so pack actions keep
     export parity.

2. **A finding resolver — [`resolve_stale_example_row`]** — takes one stale-example
   finding's title, the affected snippet/command/config anchor, its stale-example status,
   the documented and current versions, and derives:
   - an **example drift posture** (`example_verified_current`,
     `example_current_pending_reverify`, `signature_drift_actionable`,
     `deprecated_symbol_actionable`, `broken_reference_actionable`,
     `version_mismatch_actionable`, `unverified_needs_check`) — a drifted or unverified
     example is never shown as current, and an example claiming current with stale/unknown
     freshness is held for reverification;
   - the **compare / open-current-source / suppress / export** actions, so a drift becomes
     a concrete, anchored, actionable row instead of a vague "docs may be old" hint;
   - a `has_version_drift` flag when the documented and current versions differ.

3. **A parity matrix — `M5DocsPackFindingPrimitivePacket`** — binds one row per claimed M5
   pack/finding consumer (`docs_pack_manager`, `help_pack_panel`, `onboarding_pack_step`,
   `ai_pack_context`, `support_pack_evidence`) to the shared pack-row and stale-example
   anatomy, the same trust postures, drift postures, pack states, stale-example statuses,
   actions, export fields, and non-visual accessibility routes.

## Acceptance-criteria coverage

- **Users can tell whether a docs pack is pinned, stale, mirrored, quarantined, or current
  before trusting it** — the trust-posture ladder keeps these distinct, and
  `PackStateDistinctnessUnproven` fails the packet unless every distinct state is proven by
  a worked pack row.
- **Example drift becomes an actionable row with concrete anchors** — every finding is
  anchored to a snippet/command/config, and `ExampleDriftActionableUnproven` fails the
  packet unless at least one actionable drift with `compare_drift` + `open_current_source`
  is proven.
- **Pack/update/remove actions keep mirror/offline/export parity across docs/help/support
  consumers** — `ActionParityUnproven` fails the packet unless the pack
  update/remove/export-manifest actions and the example compare/open/export actions all
  appear across the worked cases.
- **Honesty** — `TrustHonestyUnproven` fails the packet if any worked case shows a
  quarantined/stale/mirrored/offline pack as live or a drifted example as current, or if the
  live-vs-not-live / current-vs-drift contrast is not proven.

## Boundaries

Raw URLs, raw tokens, credentials, private endpoints, pack payloads, and example bodies
stay outside the support boundary. Every pack name, signer, refresh time, affected anchor,
cited version, and action target is carried only as an opaque, export-safe representation.
The reused pack state, stale-example status, corpus class, version scope, source provider,
freshness state, docs surface family, deployment line, consumer surface, accessibility
route, qualification class, and downgrade trigger vocabularies come verbatim from the
frozen M05-868 matrix — this module invents no parallel pack-row or stale-example grammar.

## Artifacts

- Schema: `schemas/docs/m5-docs-pack-row-and-stale-example-finding-row-primitive.schema.json`
- Support export: `artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/support_export.json`
- Matrix CSV: `artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/matrix.csv`
- Report: `artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive.md`
- Narrowed fixtures: `fixtures/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/`

## Regenerate

```sh
BIN=target/debug/aureline_docs_pack_row_stale_example_finding_primitive
$BIN support-export > artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/support_export.json
$BIN csv            > artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/matrix.csv
$BIN report         > artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive.md
$BIN fixture-onboarding-pack-beta-narrowed  > fixtures/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/onboarding_pack_beta_narrowed.json
$BIN fixture-ai-pack-context-preview-narrowed > fixtures/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/ai_pack_context_preview_narrowed.json
```
