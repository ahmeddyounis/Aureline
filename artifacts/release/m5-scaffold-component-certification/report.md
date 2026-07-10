# M5 Scaffold Component Surface Certification

- Packet: `m5-scaffold-component-certification:stable:0001`
- As of: `2026-07-09T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-scaffold-component-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Source and recovery preserved on every surface: true
- No surface hides a side effect behind a generic Create: true
- No surface exposes a secret-bound raw value by default: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:start-center** — surface=start_center claimed=qualified_starter certified=qualified_starter status=green narrowed_axes=0 source_recovery=true hides_side_effect=false raw_value=false
- **cert:template-gallery** — surface=template_gallery claimed=qualified_starter certified=qualified_starter status=green narrowed_axes=0 source_recovery=true hides_side_effect=false raw_value=false
- **cert:workspace-handoff** — surface=workspace_handoff claimed=qualified_starter certified=qualified_starter status=green narrowed_axes=0 source_recovery=true hides_side_effect=false raw_value=false
- **cert:support-export** — surface=support_export claimed=qualified_starter certified=qualified_starter status=green narrowed_axes=0 source_recovery=true hides_side_effect=false raw_value=false
- **cert:scaffold-preflight** — surface=scaffold_preflight claimed=qualified_starter certified=blocked_prerequisite_projection status=yellow narrowed_axes=1 source_recovery=true hides_side_effect=false raw_value=false
- **cert:template-health** — surface=template_health claimed=qualified_starter certified=drifted_template_projection status=yellow narrowed_axes=1 source_recovery=true hides_side_effect=false raw_value=false
- **cert:generation-diff-review** — surface=generation_diff_review claimed=qualified_starter certified=partial_generation_projection status=yellow narrowed_axes=1 source_recovery=true hides_side_effect=false raw_value=false
- **cert:cli-headless** — surface=cli_headless claimed=qualified_starter certified=secret_bound_parameter_projection status=yellow narrowed_axes=1 source_recovery=true hides_side_effect=false raw_value=false
