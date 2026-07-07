# M5 docs-browser-component accessibility & auto-narrowing (M05-874)

This lane is the accessibility-and-auto-narrowing **capstone** over the frozen M5
docs-browser component matrix
(`freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`).
Where the freeze matrix defines the reusable docs search bar, scope switcher, docs
result row, symbol-linked reference card, docs source / version badge, docs-pack row,
stale-example finding row, and browser-handoff banner primitives — and the 869–873
implementation lanes resolve their per-surface truth — this lane certifies, per component
family, that documentation claims stay **keyboard-complete, assistive-tech-reachable,
CLI/export-safe, and self-narrowing** rather than presenting cached, version-adjacent,
mirrored, quarantined, or policy-blocked docs as a still-current authoritative reference.

- Rust module:
  `crates/aureline-docs/src/implement_keyboard_screen_reader_cli_export_parity_and_docs_browser_component_claim_auto_narrowing/`
- Boundary schema:
  `schemas/docs/implement-keyboard-screen-reader-cli-export-parity-and-docs-browser-component-claim-auto-narrowing.schema.json`
- Support export (canonical):
  `artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback/support_export.json`
- Matrix CSV:
  `artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback/matrix.csv`
- Markdown report:
  `artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback.md`
- Fixtures:
  `fixtures/docs/m5/m5-docs-browser-component-accessibility-fallback/`

## What it certifies

Each `DocsBrowserAccessibilityRow` keys on one frozen
`M5DocsBrowserComponentFamily` and reuses the frozen `M5DocsRequiredLabel`,
`M5DocsDowngradeTrigger`, and shared `M5DocsConsumerSurface` vocabulary rather than
minting parallel synonyms, so certified labels stay byte-identical to the matrix and the
sibling primitive packets. Four properties are proven per family:

1. **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
   screen-reader-reachable, and CLI/headless-reachable path into the same corpus class,
   provider / source, version / package scope, symbol anchor, project-doc override
   reason, freshness reading, pack pin / mirror / offline / quarantine state,
   stale-example status, and browser-handoff reason the rich surface shows — never a
   hover-only card that strands assistive-tech or headless users. Hierarchy-heavy
   families (the symbol-linked reference card's symbol-anchor tree with its nested
   member / signature sub-rows) additionally bind their tree to a flat list / textual
   path.
2. **Export parity.** The support / docs / evaluation export reconstructs each
   component's meaning from typed tokens and opaque refs without a screenshot.
3. **Honest auto-narrowing.** When docs freshness, version match, pack verification, or
   source / handoff reachability weakens, the component's docs-support claim auto-narrows
   from `current_authoritative` / `supported_reference` down the ladder, discloses the
   narrowing with a precise trigger and binding dimension, and preserves the canonical
   corpus / source / version / symbol / pack / handoff identity. A component with every
   dimension intact must **not** carry a spurious narrowing.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the docs browser,
   help center, AI evidence, onboarding exports, headless CLI, and support / admin
   exports so docs / help / onboarding / AI publication stays aligned rather than
   drifting in copy.

## Claim ladder

`M5DocsSupportClaim` (strongest first):

| Claim | Rank | Meaning |
| --- | --- | --- |
| `current_authoritative` | 5 | Fresh, version-matched, provider-verified reference. |
| `supported_reference` | 4 | Resolved, self-sufficient object (scope switcher, resolved result). |
| `version_adjacent_reference` | 3 | Drawn from a nearby version / package scope. |
| `cached_reference` | 2 | Cached / mirrored last-known copy, not a live read. |
| `unverified_reference` | 1 | Symbol linkage / source could not be verified. |
| `policy_blocked_reference` | 0 | Pack quarantined or a policy dependency is unmet. |

Each observed `M5DocsConditionState` imposes a permitted ceiling — `current` →
`current_authoritative`, `adjacent` → `version_adjacent_reference`, `cached` →
`cached_reference`, `unverified` → `unverified_reference`, `quarantined` →
`policy_blocked_reference`. The effective claim may never exceed the strongest permitted
ceiling across a row's modeled dimensions.

## Dimensions and frozen triggers

Every family models its primary weakening dimension. The four spec-required
auto-narrowing axes — docs freshness, version match, pack verification, and handoff
state — are `result_freshness`, `version_match`, `pack_verification`, and
`handoff_state`. Each dimension names an on-topic frozen `M5DocsDowngradeTrigger`:

| Dimension | Family | Frozen trigger |
| --- | --- | --- |
| `corpus_reachability` | docs_search_bar | `corpus_class_unstated` |
| `version_match` | docs_scope_switcher | `version_scope_unstated` |
| `result_freshness` | docs_result_row | `freshness_hidden` |
| `symbol_linkage` | symbol_linked_reference_card | `symbol_anchor_unresolved_hidden` |
| `source_provenance` | docs_source_version_badge | `source_provider_masked` |
| `pack_verification` | docs_pack_row | `pack_state_misrepresented` |
| `example_drift` | stale_example_finding_row | `stale_example_shown_as_current` |
| `handoff_state` | docs_handoff_banner | `handoff_reason_unstated` |

## Seed status

Eight rows, one per frozen family: **2 green** (docs_search_bar current-authoritative;
docs_scope_switcher supported-reference) and **6 yellow** honest auto-narrows covering
`cached_reference`, `unverified_reference`, `version_adjacent_reference`, and
`policy_blocked_reference` — so all six claim tiers appear as effective claims, all eight
dimensions are exercised, and all ten consumer surfaces ingest a row. No red rows may
ship.

## Regenerating artifacts

The support export, CSV, report, and fixtures are generated from the single seed builder:

```
GEN_DOCS_BROWSER_A11Y_ARTIFACTS=1 cargo test -p aureline-docs generate_artifacts
```

Verify with:

```
cargo test -p aureline-docs --lib implement_keyboard_screen_reader_cli_export_parity_and_docs_browser_component_claim_auto_narrowing
```
