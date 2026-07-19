# M5 Navigation-Content Component Surface Certification

- Packet: `m5-navigation-content-component-certification:stable:0001`
- As of: `2026-07-12T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-navigation-content-proof/support_export.json`
- Profiles: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:live-active-context-shell** — profile=live_active_context_shell claimed=current_navigation_result certified=current_navigation_result status=green narrowed_axes=0
- **cert:reviewable-explorer-tree** — profile=reviewable_explorer_tree claimed=reviewable_structure_result certified=reviewable_structure_result status=green narrowed_axes=0
- **cert:reviewable-result-grid** — profile=reviewable_result_grid claimed=reviewable_structure_result certified=reviewable_structure_result status=green narrowed_axes=0
- **cert:traced-breadcrumb-trail** — profile=traced_breadcrumb_trail claimed=reviewable_structure_result certified=reviewable_structure_result status=green narrowed_axes=0
- **cert:stale-hierarchy-breadcrumb** — profile=stale_hierarchy_breadcrumb claimed=reviewable_structure_result certified=hierarchy_unverified_projection status=yellow narrowed_axes=1
- **cert:unresolved-count-list** — profile=unresolved_count_list claimed=reviewable_structure_result certified=count_unverified_projection status=yellow narrowed_axes=1
- **cert:stale-provenance-grid** — profile=stale_provenance_grid claimed=reviewable_structure_result certified=sort_filter_unverified_projection status=yellow narrowed_axes=1
- **cert:partial-freshness-panel** — profile=partial_freshness_panel claimed=reviewable_structure_result certified=source_freshness_projection status=yellow narrowed_axes=1
