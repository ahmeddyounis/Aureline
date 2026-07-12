# M5 Navigation / Content-Component Accessibility & Auto-Narrowing (M05-1114)

This contract governs the **keyboard / screen-reader / high-zoom / reduced-motion / CLI / export
parity and honest automatic claim narrowing** capstone over the frozen M5 navigation-content
component matrix
(`schemas/ui/m5-navigation-content-component-matrix.schema.json`, workstream **B132**). It is the
accessibility / export / narrowing sibling of the implementation lanes (M05-1109 … M05-1112) that
resolve per-surface truth for the tab strip, breadcrumbs, tree view, list view, table / grid, and
panel header primitives.

- **Schema:** `schemas/ui/m5-navigation-content-component-accessibility-parity.schema.json`
- **Support export (canonical):**
  `artifacts/release/m5-navigation-content-component-accessibility-parity/support_export.json`
- **Matrix CSV:**
  `artifacts/release/m5-navigation-content-component-accessibility-parity/matrix.csv`
- **Report:** `artifacts/release/m5-navigation-content-component-accessibility-parity.md`
- **Mirror fixtures:**
  `fixtures/ui/m5-navigation-content-component-accessibility-parity/`
- **Rust module:** `aureline-shell` →
  `add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_hierarchy_selection_count_sort_filter_or_freshness_truth_is_missing_or_stale_across_claimed_m5_navigation_content_components`
- **Regeneration:** `GEN_NAVIGATION_CONTENT_A11Y_ARTIFACTS=1 cargo test -p aureline-shell --lib add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_hierarchy`

## What each row certifies

Each row keys on one frozen `M5NavigationContentComponentFamily` and reuses the frozen required
labels, downgrade triggers, and consumer surfaces from the matrix (no parallel synonyms are minted).
A row certifies that the family:

1. **Reaches canonical truth via assistive tech.** A keyboard-complete, screen-reader-reachable,
   high-zoom-legible, reduced-motion-safe, and CLI/headless-reachable path exposes the same active
   context, hierarchy / path, disclosure state, selection-versus-current, item state, count scope,
   sort / filter provenance, and source-freshness the rich component shows — never a hover-only badge.
   Hierarchy-heavy families (the tree view's nested disclosure structure) additionally bind their
   nested structure to a flat list / textual path.
2. **Exports without a raw payload.** The support / release / CLI export reconstructs the component's
   meaning from typed tokens and opaque refs, copyable as text / JSON / Markdown; a raw tree body, row
   payload, or query internal is never the only export.
3. **Auto-narrows honestly.** When a navigation / content dimension weakens, the component's claim
   auto-narrows from `current_navigation_result` / `reviewable_structure_result` to the exact
   permitted projection, names the binding dimension and the frozen downgrade trigger, and preserves
   the canonical component identity / active context / last-known state. The underlying navigation /
   content truth is never dropped opaquely.

## The claim ladder and honest narrowing

`M5NavContentComponentClaim` ranks the postures a surface may present, strongest first:

| Claim | Meaning |
| --- | --- |
| `current_navigation_result` | Fully current, authoritative, count-honest, provenance-clear surface. |
| `reviewable_structure_result` | A reviewable read-only dense structure (tree / grid), not a live-current surface. |
| `hierarchy_unverified_projection` | Hierarchy / path stale or partial — last-known ancestry preserved. |
| `count_unverified_projection` | Exact / loaded / all-matching count scope unresolved — loaded scope preserved. |
| `sort_filter_unverified_projection` | Sort / filter provenance stale — last-known ordering named. |
| `source_freshness_projection` | Source-freshness cue only partial / cached — cached cue disclosed. |

`M5NavContentComponentConditionState::cannot_be_shown_current` flags the three overclaim-risk states —
`hierarchy_path_stale`, `count_scope_unresolved`, and `sort_filter_provenance_stale` — that must never
keep a `current_navigation_result` claim. `source_freshness_partial` is deliberately **excluded**: a
cached / partial freshness cue shown honestly is a disclosed-absence operation, not a truth
overstatement, so it still auto-narrows to `source_freshness_projection` but does not trip the
`weak_state_shown_as_current` guardrail.

## Guardrails (mirroring the frozen matrix)

- A tab strip never masquerades as top-level workflow navigation.
- Counts and blocked rows are never hidden behind ambiguous ellipses; exact, loaded, and
  all-matching scopes stay distinct.
- Tree / list / table local actions are never hover-only.
- A panel header never becomes a cluttered secondary toolbar.
- Every narrowed rendering surface discloses its reduction and preserves its labels, so shell,
  explorer, search, review, request/data, help, AI-context, support-export, and product consumers stay
  aligned on the same narrowed state.

## Acceptance-criteria coverage

- **Every B132 component has non-visual and exported representations that preserve hierarchy and count
  truth.** Six rows cover the six frozen families one-to-one; each offers keyboard / screen-reader /
  high-zoom / reduced-motion / CLI reach and an export-safe summary with no raw payload.
- **Stale or partial navigation / content evidence causes visible narrowing rather than silent
  optimistic copy.** Four families auto-narrow to their permitted projection with a precise, non-generic
  label and the frozen trigger; the tab strip stays green and the tree view stays a disclosed-reduced
  reviewable structure.
- **Accessibility, export, and narrowing behaviors are proven in the first claimed B132 consumers.**
  Each row ships to at least the support export and product UI plus its two most relevant surfaces, and
  the full nine-surface consumer set is exercised across the packet.
