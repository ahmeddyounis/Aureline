# M5 Shell-Metric / Density / Geometry Shared Consumers — One Geometry Across Surfaces

**Status:** stable · **Batch:** B138 · **Row:** M05-1161

This lane is the consumer-adoption capstone for the five B138 shell-geometry families frozen in the
[shell-metric / density matrix](./m5_shell_metric_density_contract.md) and implemented by the three
registry lanes:

| Family | Domain schema |
| --- | --- |
| `shell_metric`, `minimum_size` | `schemas/shell/m5-shell-metrics.schema.json` |
| `density_mode`, `responsive_geometry`, `collapse_priority` | `schemas/shell/m5-density-mode.schema.json` |

It proves — by fixtures, not screenshots — that shell metrics, minimum sizes, density modes,
responsive window classes, and collapse priority are actually **reused** by the shell, editor,
review, notebook, data, settings, and CLI/export/support surfaces users hit most often, instead of
remaining an isolated design packet each surface silently re-forks with private widths and row
heights.

## What the packet asserts

The packet
(`artifacts/release/m5-shell-metric-density-shared-consumers-proof/support_export.json`, schema
`schemas/shell/m5-shell-metric-density-shared-consumers.schema.json`) records one `consumer_binding`
per (geometry object × consumer surface × representation). The three honesty axes mirror the batch
acceptance criteria.

1. **Reuse.** Every one of the five shell-geometry families is adopted by at least two distinct
   consumers, so a family is proven shared infrastructure rather than a one-surface fork of metrics,
   density, or adaptive sizing.
2. **One geometry / no drift.** For a given geometry object every consumer surface presents an
   identical `state_facets` block — the same `geometry_role_word` (a frozen `zone` / `metric` /
   `hit_target` / `density` / `responsive` / `collapse` / `workspace_dominance` token), the same
   `family_word`, `registry_reference_word`, `width_or_density_class_word`, `surface_context_word`,
   and `minimum_guarantee_word`. A surface may narrow *how much* it shows across the desktop,
   compact, remote, and exported representations, but it may never reword the grammar per surface,
   and a role that carries density, responsive, collapse, or workspace-dominance meaning may never
   drop task identity, shrink a hit target below the supported minimum, invent a private fracturing
   width, or hide a primary workflow behind an overlay-only fallback.
3. **Map back to one family.** Support and CLI/export bindings point at both the canonical per-domain
   schema and the frozen matrix schema by id, so an exported packet always maps a surface back to one
   shared contract family.

## The minimum-guarantee gate

An adaptive role (`density`, `responsive`, `collapse`, `workspace_dominance` — every role for which
`must_preserve_task_identity_under_collapse()` is true) must always carry a real
`minimum_guarantee_word`. The gate rejects the sentinel fallbacks `none`, `overlay_only`,
`private_width`, `shrunk_below_minimum`, and `hidden_workflow`, so a density switch, width-class
change, or collapse can never quietly satisfy geometry constraints by starving the workspace,
shrinking a hit target, inventing a private width, or hiding a workflow behind an overlay.

## Guardrail row-invariants

Every binding declares five guardrails, each of which must be `false` — these are the B138 hard
invariants:

- `density_or_collapse_changes_command_focus_or_trust`
- `extension_or_embedded_sets_private_fracturing_width`
- `shrinks_hit_target_below_supported_minimum`
- `hides_primary_workflow_behind_overlay_only_fallback`
- `lets_zone_starve_main_workspace_below_minimum`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason,
the preserved grammar, and the next action; remote projections additionally name a
`remote_source_note` and exports an `export_detail_note`. Narrowing is a disclosed change in
*depth*, never a change in the underlying grammar.

## Consumer inventory

| Family | Adopting consumers |
| --- | --- |
| `shell_metric` | shell, editor, support-export |
| `minimum_size` | editor, review, cli-export |
| `density_mode` | shell, notebook, data (compact) |
| `responsive_geometry` | shell, data, product (remote) |
| `collapse_priority` | shell, settings, review |

Any partial or narrowed adoption is explicit in each binding's `representation` and `narrow_note`.

## Regenerating the artifacts

The seed builders in the module are the only mint-from-truth path. Re-emit with:

```text
cargo run -p aureline-ui --example dump_m5_shell_metric_density_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_shell_metric_density_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_shell_metric_density_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_shell_metric_density_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_shell_metric_density_shared_consumers -- fixture-exported-redaction-narrowed
```

The checked-in artifacts are byte-locked against these builders by the module's tests.
