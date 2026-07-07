# M5 docs-browser component matrix contract

**Row:** M05-868 — Freeze the M5 docs-search-bar, docs-result-row,
symbol-linked-reference-card, docs-source-badge, docs-pack-row, and
handoff-banner component matrix (batch B102).

This contract freezes the reusable **documentation-browser and knowledge-surface
component matrix** so documentation search, docs-pack, and browser-handoff
surfaces stop drifting on corpus, provider, version, freshness, and handoff
language. It is the docs-domain analog of the shell runtime-boundary
(`freeze_the_m5_runtime_boundary_*`) and release-center
(`freeze_the_m5_release_candidate_card_*`) component freezes.

- **Crate / module:** `aureline-docs`,
  `freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`
- **Schema:** `schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json`
- **Support export (canonical truth):**
  `artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/support_export.json`
- **Matrix CSV / Markdown report:** same directory (`matrix.csv`, `.md`)
- **Narrowed fixtures:**
  `fixtures/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/`
- **Headless emitter:** `cargo run -p aureline-docs --bin aureline_docs_browser_component_matrix -- <support-export|report|csv|fixture-*|validate>`

## Governed component families (8)

| Family | Owns (family-specific vocabulary) |
| --- | --- |
| `docs_search_bar` | corpus classes |
| `docs_scope_switcher` | version / package scopes |
| `docs_result_row` | match states, project-doc override reasons |
| `symbol_linked_reference_card` | symbol anchors |
| `docs_source_version_badge` | source providers, freshness states |
| `docs_pack_row` | docs-pack states |
| `stale_example_finding_row` | stale-example statuses |
| `docs_handoff_banner` | browser-handoff reasons |

Every family also declares docs surface families, deployment lines, mandatory
plus truth labels, non-visual accessibility routes, consumer surfaces, and
downgrade triggers.

## Frozen controlled vocabularies

- **Corpus class:** `first_party_docs`, `api_reference`, `guide_tutorial`,
  `codebase_symbol`, `community_contributed`, `vendor_dependency`,
  `release_notes_changelog`
- **Version / package scope:** `exact_version_match`, `nearby_version`,
  `project_specific`, `latest_stable`, `pinned_range`, `unversioned`
- **Result match state:** `exact_match`, `nearby_match`,
  `project_specific_match`, `mirrored_match`, `cached_match`, `stale_match`
- **Project-doc override reason:** `project_pinned_override`,
  `local_freshness_override`, `explicit_user_preference`,
  `vendor_source_unavailable`, `policy_scoped_override`, `no_override`
- **Symbol anchor:** `function_symbol`, `type_symbol`, `module_symbol`,
  `field_or_method`, `macro_symbol`, `unresolved_anchor`
- **Source provider:** `bundled_local`, `mirrored_registry`,
  `first_party_hosted`, `third_party_hosted`, `offline_import`, `ai_derived`
- **Freshness state:** `live_current`, `recently_synced`, `cached_offline`,
  `stale_expired`, `unknown_freshness`
- **Docs-pack state:** `pinned_pack`, `mirrored_pack`, `offline_pack`,
  `quarantined_pack`, `update_available`, `unpinned_tracking`
- **Stale-example status:** `example_current`, `api_signature_drifted`,
  `deprecated_symbol_used`, `broken_link_target`, `version_mismatch_example`,
  `unverified_example`
- **Browser-handoff reason:** `no_local_corpus`, `interactive_content_required`,
  `auth_gated_source`, `dynamic_rendering_required`, `external_canonical_source`,
  `user_requested_browser`

Shared/topology vocabularies: docs surface family (8), deployment line (5),
consumer surface (10), accessibility route (6), required label (6, with mandatory
`identity` / `state` / `keyboard_route`), qualification class (6), downgrade
trigger (12).

## Hard component invariants

Every component row must keep all four `false`:

1. `masks_corpus_or_source_provenance` — never hide which corpus or source a
   component draws from.
2. `shows_stale_or_cached_as_live_current` — never present cached or mirrored
   documentation as live/current.
3. `invents_private_docs_status_grammar` — never invent a second docs-status
   grammar outside this matrix.
4. `hides_handoff_reason_or_override_reason` — never dead-end a browser handoff or
   silently reorder results without stating why.

## Non-visual / CLI / export expectations

Every component declares a non-visual accessibility route set (keyboard focus,
screen-reader announcement, non-hover reachability, pointer-optional,
high-contrast safety, support-exportability). Docs-browser primitives must never
become hover-only or browser-only affordances: the same corpus/source/version/
freshness/handoff truth is reachable via keyboard, screen reader, CLI inspect, and
the support export.

## Auto-narrowing

Qualification narrows below Stable when a downgrade trigger fires (e.g. corpus
class unstated, source masked, mirrored/cached shown as live, quarantined pack
shown as trusted, handoff reason unstated, proof stale). The two checked-in
narrowed fixtures demonstrate the pattern while keeping every family visible:
`stale_example_finding_row` → Beta, `docs_handoff_banner` → Preview.

## Bound source contracts

`stable_docs_source_result_pack_and_citation.schema.json`,
`symbol_linked_reference.schema.json`, `docs_pack_manifest.schema.json`, and
`schemas/integration/browser_handoff_packet.schema.json` — this matrix hardens
shared components layered on top of those already-claimed systems; it does not
re-architect docs retrieval, citation assembly, or docs-pack distribution.

## Consumer rule

Every claimed M5 docs/help/onboarding/AI consumer points at this one canonical
component contract instead of rewording docs truth locally. Future implementation
rows have an agreed field/state baseline and no open ambiguity about
corpus/provider/version/handoff vocabulary.
