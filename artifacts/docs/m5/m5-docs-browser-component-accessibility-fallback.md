# M5 Docs-Browser-Component Accessibility & Auto-Narrowing

- Packet: `m5-docs-browser-component-accessibility-fallback:stable:0001`
- As of: `2026-07-06T00:00:00Z`
- Families: 8 certified across 8 / 8 frozen families
- Status: 2 green / 6 yellow / 0 red

## Rows

- **a11y:docs-search-bar** (docs_search_bar) — family=docs_search_bar keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=current_authoritative effective_claim=current_authoritative status=parity
- **a11y:docs-scope-switcher** (docs_scope_switcher) — family=docs_scope_switcher keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=supported_reference effective_claim=supported_reference status=parity
- **a11y:docs-result-row** (docs_result_row) — family=docs_result_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=current_authoritative effective_claim=cached_reference status=narrowed_disclosed
  - Auto-narrow: current_authoritative → cached_reference (dimension=result_freshness, trigger=freshness_hidden) — Result shown from a cached / mirrored copy — not a live provider read until the corpus refreshes
- **a11y:symbol-linked-reference-card** (symbol_linked_reference_card) — family=symbol_linked_reference_card keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=current_authoritative effective_claim=unverified_reference status=narrowed_disclosed
  - Auto-narrow: current_authoritative → unverified_reference (dimension=symbol_linkage, trigger=symbol_anchor_unresolved_hidden) — Symbol anchor resolved by keyword fallback only — reference shown unverified, not linked to the exact symbol
- **a11y:docs-source-version-badge** (docs_source_version_badge) — family=docs_source_version_badge keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=current_authoritative effective_claim=version_adjacent_reference status=narrowed_disclosed
  - Auto-narrow: current_authoritative → version_adjacent_reference (dimension=source_provenance, trigger=source_provider_masked) — Source resolvable only at a nearby version — badge shown version-adjacent, not the exact-version provider
- **a11y:docs-pack-row** (docs_pack_row) — family=docs_pack_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=current_authoritative effective_claim=policy_blocked_reference status=narrowed_disclosed
  - Auto-narrow: current_authoritative → policy_blocked_reference (dimension=pack_verification, trigger=pack_state_misrepresented) — Pack quarantined pending re-verification — shown policy-blocked, not a trusted pinned pack
- **a11y:stale-example-finding-row** (stale_example_finding_row) — family=stale_example_finding_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=current_authoritative effective_claim=cached_reference status=narrowed_disclosed
  - Auto-narrow: current_authoritative → cached_reference (dimension=example_drift, trigger=stale_example_shown_as_current) — Example drifted from its source — shown from a cached snapshot anchored to an older version, not current
- **a11y:docs-handoff-banner** (docs_handoff_banner) — family=docs_handoff_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=current_authoritative effective_claim=unverified_reference status=narrowed_disclosed
  - Auto-narrow: current_authoritative → unverified_reference (dimension=handoff_state, trigger=handoff_reason_unstated) — Handoff return-path source unverified — shown unverified until the destination reachability is re-confirmed
