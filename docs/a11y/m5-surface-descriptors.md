# M5 Accessibility-Surface Descriptors and Bridge Mappings

This document is the contract for the M5 accessibility-surface descriptor catalog
that materializes one machine-readable descriptor per claimed custom-rendered M5
surface. Where the frozen dynamic-surface matrix governs *which* accessibility
objects a custom surface may publish and *which* controlled vocabularies they
carry, this catalog supplies the *concrete* per-surface truth that shell, editor,
terminal, notebook, data, and review surfaces map into the OS accessibility bridge.

- Record kind: `m5_accessibility_surface_descriptor_catalog`
- Schema: [`schemas/a11y/m5-surface-descriptors.schema.json`](../../schemas/a11y/m5-surface-descriptors.schema.json)
- Canonical support export: [`artifacts/a11y/m5-bridge-descriptor-proof/support_export.json`](../../artifacts/a11y/m5-bridge-descriptor-proof/support_export.json)
- Governance summary artifact: [`artifacts/a11y/m5-bridge-descriptor-proof/bridge-descriptor-proof.md`](../../artifacts/a11y/m5-bridge-descriptor-proof/bridge-descriptor-proof.md)
- Fixtures: [`fixtures/a11y/m5-surface-descriptors/`](../../fixtures/a11y/m5-surface-descriptors/)
- Producer: `aureline_shell::accessibility::current_stable_m5_surface_descriptor_export`
- Headless emitter: `aureline_shell_m5_surface_descriptors`
- Frozen governing matrix: [`schemas/a11y/m5-dynamic-surface-a11y.schema.json`](../../schemas/a11y/m5-dynamic-surface-a11y.schema.json)

## Why this catalog exists

The custom shell is supportable only when every claimed dynamic surface can explain
its semantic model to OS accessibility APIs and assistive tooling. Before this
catalog the OS accessibility bridge depended on per-surface hand wiring that drifted
from docs, diagnostics, and assistive-tech proof artifacts. The descriptor catalog
makes the bridge mapping a single governed packet: one descriptor per surface,
reused by shell, editor, terminal, notebook, data, and review surfaces, by
diagnostics, by support exports, by docs/help, and by assistive-tech conformance
packets. Pixel-only rendering and pointer-only affordances are never the source of
truth once a surface has a descriptor.

## Claimed surfaces

The catalog carries a descriptor for each claimed custom-rendered surface family:

| Surface family | Descriptor | Primary role | OS bridge |
| --- | --- | --- | --- |
| `shell_region` | `surface:shell.zone-frame` | `landmark_region` | `ui_automation` |
| `editor_canvas` | `surface:editor.content-canvas` | `text_document` | `ui_automation` |
| `terminal_canvas` | `surface:terminal.log-canvas` | `live_log_region` | `ui_automation` |
| `dense_collection` | `surface:data.dense-collection` | `data_grid_cell` | `ui_automation` |
| `notebook_cell` | `surface:notebook.cell` | `notebook_cell` | `ui_automation` |
| `data_cell` | `surface:data.cell` | `data_grid_cell` | `ui_automation` |
| `review_diff` | `surface:review.diff-hunk` | `structure_group` | `ui_automation` |
| `overlay_sheet` | `surface:shell.overlay-sheet` | `structure_group` | `ui_automation` |

## What each descriptor binds

Each `surface_descriptor` binds a stable `surface_id` to:

- **Roles and regions** — a `primary_role_class` plus one or more semantic
  `regions`, each with its own role class, label, and landmark flag, so the
  surface's structure is exposed rather than left visual-only.
- **A screen-reader label model** — the `name_source`, the dynamic
  `state_label_classes` spoken alongside the name, the announcement
  `fallback_durability`, and the `non_visual_fidelity` the model exposes.
- **Focus-order metadata** — an ordered `stops` list (indices contiguous from
  zero, each naming a real region) plus the `async_return_disposition` and
  `return_fallback_durability` that keep focus from teleporting or vanishing on an
  asynchronous update or overlay teardown.
- **Reduced-motion and high-zoom postures** — explicit `reduced_motion` and
  `high_zoom` declarations, gated by `behavior_changes_under_reduced_motion` and
  `behavior_changes_under_high_zoom`. A surface whose behavior changes under a mode
  must declare a concrete adaptation posture; a surface that does not must declare a
  no-change posture. The declaration cannot be left implicit.
- **An OS accessibility-bridge mapping and health** — the `bridge_kind`, the
  current `bridge_state` (health), the delivered `non_visual_fidelity`, the
  `native_role_hint` (UI Automation / NSAccessibility / AT-SPI), and the disclosed
  `degradation_reason`.

## Controlled vocabulary reuse

The controlled state vocabularies — `semantic_role_class`, `non_visual_fidelity`,
`bridge_state`, `focus_return_disposition`, `announcement_politeness`,
`coalescing_strategy`, and `fallback_durability` — are reused verbatim from the
frozen dynamic-surface matrix through the `shared_vocabulary_set` block, which must
match the matrix's canonical token lists. The descriptor-shaped vocabularies a
concrete surface adds — `surface_family`, `bridge_kind`, `name_source`,
`state_label_class`, `reduced_motion_posture`, `high_zoom_posture`, and
`bridge_degradation_reason` — are frozen in the `descriptor_vocabulary_set` block.
No surface mints a parallel synonym for a governed state.

## Auto-narrowing on degraded bridge or stale proof

A descriptor whose bridge is not `bridged_active` must disclose its degradation
(`degradation_reason` set), must not claim `full_accessible` non-visual fidelity,
must not stay `stable`, and must carry a `bridge_partial_or_stale` or
`bridge_unavailable` downgrade trigger. A descriptor whose assistive-tech proof has
gone stale narrows its qualification while keeping the surface visible. The
`bridge_degraded.json` and `proof_stale_narrowed.json` fixtures exercise both paths:
the narrowing is always a disclosed claim change, never a hidden surface.

## Consumers

`shell`, `editor`, `terminal`, `notebook`, `data_grid`, and `review` surfaces map
their accessibility bridge nodes from the descriptors; diagnostics, support exports,
docs/help, and assistive-tech conformance packets reuse the same descriptors. The
`consumer_projection` block records that every one of those consumers is wired to
the descriptors rather than re-deriving bridge state.

## Regenerating the catalog

The seed builders in `aureline_shell::accessibility` are the single producer of the
checked-in support export and fixtures. Regenerate with the headless emitter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- support-export \
  > artifacts/a11y/m5-bridge-descriptor-proof/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- markdown \
  > artifacts/a11y/m5-bridge-descriptor-proof/bridge-descriptor-proof.md
cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- fixture-bridge-degraded \
  > fixtures/a11y/m5-surface-descriptors/bridge_degraded.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_surface_descriptors -- fixture-proof-stale-narrowed \
  > fixtures/a11y/m5-surface-descriptors/proof_stale_narrowed.json
```

The `checked_support_export_matches_seed` test fails if the checked-in export drifts
from the seed builder, so the artifact, the fixtures, and the in-code catalog stay
in lockstep.
