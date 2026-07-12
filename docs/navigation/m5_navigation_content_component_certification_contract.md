# M5 navigation-content component surface certification contract (M05-1115)

This is the closing B132 capstone over the frozen M5 navigation-content component matrix
(`schemas/ui/m5-navigation-content-component-matrix.schema.json`). Where the freeze matrix
defines the six reusable **tab strip**, **breadcrumbs**, **tree view**, **list view**,
**table/grid**, and **panel header** components, the M05-1109..1112 implement lanes narrow
each one, the shared consumer lane aligns their vocabulary, and the M05-1114 accessibility
lane proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity plus
per-family auto-narrowing, this capstone **certifies** that the shared component truth holds
on every claimed M5 navigation-content operating profile — and auto-narrows any profile that
cannot sustain it.

- Boundary schema:
  [`schemas/ui/m5-navigation-content-component-certification.schema.json`](../../schemas/ui/m5-navigation-content-component-certification.schema.json)
- Canonical proof bundle (release):
  `artifacts/release/m5-navigation-content-component-certification-proof/`
- Fixtures mirror:
  `fixtures/ui/m5-navigation-content-component-certification/`
- Implementing module (aureline-shell):
  `certify_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_truth_on_every_claimed_m5_navigation_content_profile`

## What it certifies

The packet is keyed on the claimed **profile** a user, operator, or support engineer reads
navigation and content truth through — not on component family or implement lane. Eight
profiles are certified:

| Profile | Claim | Verdict | Families |
| --- | --- | --- | --- |
| `live_active_context_shell` | `current_navigation_result` | green | tab strip, panel header |
| `reviewable_explorer_tree` | `reviewable_structure_result` | green | tree view, list view |
| `reviewable_result_grid` | `reviewable_structure_result` | green | table/grid, list view |
| `traced_breadcrumb_trail` | `reviewable_structure_result` | green | breadcrumbs, tab strip |
| `stale_hierarchy_breadcrumb` | narrows to `hierarchy_unverified_projection` | yellow | breadcrumbs, tree view |
| `unresolved_count_list` | narrows to `count_unverified_projection` | yellow | list view, table/grid |
| `stale_provenance_grid` | narrows to `sort_filter_unverified_projection` | yellow | table/grid, list view |
| `partial_freshness_panel` | narrows to `source_freshness_projection` | yellow | panel header, tab strip |

Every one of the six frozen component families is certified on at least one profile, so the
shell, explorer, search, review, request/data, and help/support lanes all trace back to the
one B132 component family.

## Truth axes

Each row is scored on eight truth axes, each appearing exactly once:

1. **visual** — active context, hierarchy / path, disclosure, selection-versus-current,
   item state, counts, sort/filter provenance, and source-freshness on the primary surface.
2. **keyboard** — the same truth and its bounded local actions reachable without a pointer,
   never hover-only.
3. **screen_reader** — the same truth announced non-visually, never color/motion/glyph-only.
4. **high_zoom_reflow** — the same truth reflows legibly at high zoom.
5. **reduced_motion** — the same truth legible and usable with reduced motion.
6. **cli_export** *(always-on)* — the profile state reconstructable as text / JSON / Markdown.
7. **degraded_state** — a stale hierarchy, unresolved count scope, stale provenance, or
   partial freshness honestly downgrades the claim rather than reading as fresh navigation.
8. **navigation_content_truth** — active context, hierarchy, disclosure, selection, counts,
   provenance, and freshness stay explicit and never collapse into generic chrome.

## Invariants

- **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
  `current_navigation_result` / `reviewable_structure_result` claim while a truth axis is not
  current is over-claiming and **blocks (red)**. A profile that discloses the reduction by
  narrowing its claim (with a bound, non-generic reason and a frozen downgrade trigger) is
  honestly **yellow**.
- **Only a live first-party current-navigation profile may certify `current_navigation_result`.**
  Any reviewable, stale, unresolved, or partial profile that keeps a current-navigation claim
  blocks.
- **CLI/export parity is always-on** and must stay certified so support and automation can
  reconstruct the active context, hierarchy, disclosure, selection, exact / loaded /
  all-matching counts, sort/filter provenance, and source-freshness from the same component
  identity the user saw.
- **Certification may only narrow a claim, never strengthen it.**
- **All five B132 guardrails must hold** on every row (a breach blocks):
  1. tabs must not masquerade as top-level workflow navigation;
  2. counts or blocked rows must not hide behind ambiguous ellipses;
  3. tree / list / table local actions must not be hover-only;
  4. the panel header must not become a cluttered secondary toolbar;
  5. exact / loaded / all-matching count scopes must not collapse into one vague total.

## Metadata-only boundary

Raw tree bodies, row payloads, query internals, credentials, secrets, and endpoint refs
never cross this boundary. The packet carries only typed class tokens, opaque component refs,
booleans, and controlled labels so support, release, and diagnostics exports can reconstruct
exactly what an accessible fallback would have shown without leaking sensitive material.

## Regenerating the proof

The seed builder is the single source of truth shared by the tests and the on-disk export.
Regenerate the checked-in artifacts and fixtures with:

```sh
GEN_NAVIGATION_CONTENT_CERT_ARTIFACTS=1 cargo test -p aureline-shell \
  certify_tab_strip_breadcrumbs...::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the checked-in export drifts
from the seeded builder.
