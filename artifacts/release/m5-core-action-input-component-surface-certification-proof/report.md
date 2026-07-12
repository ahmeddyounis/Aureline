# M5 Core-Action-Input Component Surface Certification

- Packet: `m5-core-action-input-component-surface-certification:stable:0001`
- As of: `2026-07-12T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-core-action-input-proof/support_export.json`
- Profiles: 8 / 8 certified (2 green, 6 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 6
- Report clean: true

## Profiles

- **cert:live-trusted-control-surface** — profile=live_trusted_control_surface claimed=trusted_control certified=trusted_control status=green narrowed_axes=0
- **cert:reviewable-control-structure** — profile=reviewable_control_structure claimed=reviewable_control certified=reviewable_control status=green narrowed_axes=0
- **cert:unbound-command-surface** — profile=unbound_command_surface claimed=reviewable_control certified=command_binding_unverified_projection status=yellow narrowed_axes=1
- **cert:unlabeled-icon-surface** — profile=unlabeled_icon_surface claimed=reviewable_control certified=accessible_name_unverified_projection status=yellow narrowed_axes=1
- **cert:riskier-split-default-surface** — profile=riskier_split_default_surface claimed=reviewable_control certified=default_safety_unverified_projection status=yellow narrowed_axes=1
- **cert:stale-validation-field** — profile=stale_validation_field claimed=reviewable_control certified=validation_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-toggle-control** — profile=unverified_toggle_control claimed=reviewable_control certified=toggle_semantics_unverified_projection status=yellow narrowed_axes=1
- **cert:partial-retention-search-field** — profile=partial_retention_search_field claimed=reviewable_control certified=retention_disclosed_projection status=yellow narrowed_axes=1
