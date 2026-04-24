# Capability-badge axis combination fixtures

Worked fixtures for the seven-axis capability badge matrix frozen in
[`/docs/governance/capability_axis_matrix.md`](../../../docs/governance/capability_axis_matrix.md)
and its machine-readable companion
[`/artifacts/governance/capability_badge_axes.yaml`](../../../artifacts/governance/capability_badge_axes.yaml).

Each fixture names one concrete badge rendering: the axes active, the
effective per-axis values after narrowing, the worst-supporting-axis
downgrade per propagation channel, and (for forbidden fixtures) the
expected denial reason. The corpus exists so that later conformance
checkers, docs surfaces, support exports, claim manifests,
compatibility reports, and About/service-health panels can be
validated against one shared set of rows rather than inventing
local fixtures.

## Intended usage

- **Axis-presence conformance.** Every fixture emits every required
  axis; any missing axis on a propagation channel is a non-
  conformance.
- **Narrowing conformance.** Fixtures carrying live dependency
  markers render the narrowed effective value, not the declared
  value, on every channel that MUST propagate the narrowing.
- **Forbidden-combination conformance.** Fixtures prefixed
  `forbidden_` name the exact ambiguous label pair the matrix
  exists to prevent and carry the `expected_denial_reason` the
  conformance checker should emit.

## Fixtures

- [`stable_capability_authoritative_live.yaml`](./stable_capability_authoritative_live.yaml)
  — allowed: stable, standard-support, stable-channel capability
  on desktop + CLI with `no_live_markers`.
- [`preview_capability_warm_cached_with_marker.yaml`](./preview_capability_warm_cached_with_marker.yaml)
  — allowed: preview capability with one
  `non_stable_capability_dependency` marker narrowing support to
  `best_effort` and freshness to `warm_cached`.
- [`deprecated_capability_with_replacement.yaml`](./deprecated_capability_with_replacement.yaml)
  — allowed: deprecated row with a populated `deprecation_window`
  and `replacement_ref`; support narrows through
  `narrows_effective_support_class`.
- [`disabled_by_policy_kill_switch_tripped.yaml`](./disabled_by_policy_kill_switch_tripped.yaml)
  — allowed: kill-switch-tripped absorption into
  `disabled_by_policy` with a typed reason and repair hook.
- [`managed_only_channel_on_managed_admin_surface.yaml`](./managed_only_channel_on_managed_admin_surface.yaml)
  — allowed: `managed_only_channel` rendered on the
  `managed_admin_surface` client scope.
- [`client_scope_excluded_on_cli.yaml`](./client_scope_excluded_on_cli.yaml)
  — allowed: desktop-only capability rendering a typed tombstone
  on the CLI surface with `client_scope_excludes_surface`.
- [`certified_archetype_supported_row.yaml`](./certified_archetype_supported_row.yaml)
  — allowed: certified archetype row with fresh hardware,
  toolchain, reference-workspace, workflow, and platform-profile
  backing rows.
- [`certified_archetype_backing_row_stale.yaml`](./certified_archetype_backing_row_stale.yaml)
  — downgrade: stale reference-workspace row narrows the
  archetype to `best_effort` and forces the claim posture to
  `limited`.
- [`forbidden_stable_with_no_support.yaml`](./forbidden_stable_with_no_support.yaml)
  — forbidden: stable capability rendered with `no_support` and
  no narrowing marker.
- [`forbidden_ambiguous_available_chip.yaml`](./forbidden_ambiguous_available_chip.yaml)
  — forbidden: single "available" chip collapsing multiple axes.
