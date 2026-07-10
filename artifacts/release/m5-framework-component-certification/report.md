# M5 Framework Component Surface Certification

- Packet: `m5-framework-component-certification:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-framework-component-proof/support_export.json`
- Surfaces: 8 / 8 certified (3 green, 5 yellow, 0 red)
- Families covered: true
- Proving source and recovery preserved on every surface: true
- No surface lets a heuristic route masquerade as exact: true
- No surface implies a no-op write or hides the execution boundary: true
- Auto-narrowed surfaces: 5
- Report clean: true

## Surfaces

- **cert:run-config-center** — surface=run_config_center claimed=exact_framework_truth certified=exact_framework_truth status=green narrowed_axes=0 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
- **cert:support-export** — surface=support_export claimed=exact_framework_truth certified=exact_framework_truth status=green narrowed_axes=0 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
- **cert:cli-headless** — surface=cli_headless claimed=exact_framework_truth certified=exact_framework_truth status=green narrowed_axes=0 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
- **cert:framework-pack-center** — surface=framework_pack_center claimed=exact_framework_truth certified=unverified_pack_projection status=yellow narrowed_axes=1 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
- **cert:route-explorer** — surface=route_explorer claimed=exact_framework_truth certified=heuristic_inference_projection status=yellow narrowed_axes=1 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
- **cert:topology-view** — surface=topology_view claimed=exact_framework_truth certified=unlinked_source_projection status=yellow narrowed_axes=1 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
- **cert:convention-diagnostics** — surface=convention_diagnostics claimed=exact_framework_truth certified=unproven_version_range_projection status=yellow narrowed_axes=1 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
- **cert:generator-review** — surface=generator_review claimed=exact_framework_truth certified=partial_generator_effect_projection status=yellow narrowed_axes=1 proving_source_recovery=true heuristic_as_exact=false no_op_or_hidden_boundary=false
