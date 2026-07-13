# M5 Visual-Foundation Shared Consumers: One Vocabulary Across Surfaces

This lane proves that the eight B136 visual-foundation families are actually **reused** by the
major claimed M5 surface families instead of remaining a detached design-system packet. It is the
closing consumer-adoption capstone over the visual-foundation matrix
([`schemas/design-system/m5-visual-foundation-matrix.schema.json`](../../schemas/design-system/m5-visual-foundation-matrix.schema.json))
and its four implement lanes (color/theme, syntax/diff/chart, typography, and
spacing/sizing/radii/elevation + hit-target).

- Rust module: `crates/aureline-ui/src/m5_visual_foundations_shared_consumers_one_vocabulary_across_surfaces/`
- Boundary schema: [`schemas/design-system/m5-visual-foundations-shared-consumers.schema.json`](../../schemas/design-system/m5-visual-foundations-shared-consumers.schema.json)
- Support export: `artifacts/release/m5-visual-foundations-shared-consumers-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-visual-foundations-shared-consumers-proof/matrix.csv`
- Summary: `artifacts/release/m5-visual-foundations-shared-consumers-proof/summary.md`
- Fixtures: `fixtures/ui/m5-visual-foundations-shared-consumers/`
- Emitter: `cargo run -p aureline-ui --example dump_m5_visual_foundations_shared_consumers -- <subcommand>`

## The three honesty axes

1. **Reuse.** Each of the eight shared foundation families — `color_system`,
   `semantic_theme_token`, `syntax_token`, `diff_token`, `chart_token`, `typography`,
   `spacing_sizing_radii_elevation`, and `hit_target` — is adopted by **two or more distinct
   consumers**, so a family is proven to be shared visual infrastructure rather than a one-surface,
   feature-local fork of color or geometry meaning.
2. **One vocabulary / no drift.** For a given foundation object every consumer surface presents an
   identical `state_facets` block: the same `semantic_role_word`, `family_word`,
   `token_reference_word`, `theme_variant_word`, `density_context_word`, and `non_color_cue_word`.
   The semantic-role word must be a token from the frozen `M5VisualSemanticRole` vocabulary
   (`brand`, `interactive`, `neutral`, `status`, `syntax`, `diff`, `chart`), and a role that
   carries status or data meaning (`status`, `syntax`, `diff`, `chart`) may never fall back to a
   hue-alone sentinel — it must always pair color with a real non-color cue.
3. **Map back to one family.** Support (`support_export`) and CLI/export (`cli_export`) consumers
   point at the canonical per-domain schema **and** the frozen matrix by id, so an exported packet
   can always map a shell / editor / review / data / docs visual surface back to one shared
   contract family. The eight families map to three canonical domain schemas: the color system, the
   syntax/diff/chart tokens, and typography-and-geometry.

## Narrowing is disclosed, never hidden

A surface may narrow *how much* it shows across the desktop-full, compact-narrowed,
remote-projected, and exported-redacted representations, but it may never reword the underlying
vocabulary per surface. Every narrowed binding carries an explicit `narrow_note` naming the reason,
the preserved vocabulary, and the next action; a remote-projected binding additionally names its
remote source; and an exported binding names its export-safe detail boundary rather than collapsing
the object out of view.

## Consumer inventory

The support export is itself the consumer inventory: one binding per (foundation object, consumer
surface, representation). Twenty bindings across eight objects cover all nine consumer surfaces
(`shell_ui`, `editor_ui`, `review_ui`, `data_ui`, `docs_ui`, `settings_ui`, `cli_export`,
`support_export`, `product_ui`) and all four representations, with every family adopted by at least
two consumers. Any partial or narrowed adoption is made explicit by the binding's `representation`,
`parity_state`, and `narrow_note` fields — a narrowed binding is never silently equated with a full
one.

| Foundation family | Object | First consumers |
| --- | --- | --- |
| `color_system` | Status color-system palette | shell, settings, support-export |
| `semantic_theme_token` | Surface-role token | shell, editor |
| `syntax_token` | Keyword-scope token | editor, review, cli-export |
| `diff_token` | Addition-region token | review, editor |
| `chart_token` | Categorical-series token | data, docs |
| `typography` | Body-scale token | docs, editor, product |
| `spacing_sizing_radii_elevation` | Spacing-scale step | shell, data |
| `hit_target` | Minimum hit-target control | settings, product, support-export |

## Guardrails

Every binding carries five hard invariants that must be `false`, mirroring the B136 track
invariants: `relies_on_hue_alone_for_meaning`,
`lets_syntax_or_diff_palette_collide_with_diagnostics`,
`shrinks_hit_target_below_supported_minimum`, `lets_chart_meaning_depend_on_color_alone`, and
`forks_local_spacing_or_elevation_from_shared_geometry`. Any true value, any vocabulary drift, any
role word outside the frozen set, any missing canonical reference on an export consumer, or any
stale proof narrows the lane through the recorded downgrade triggers rather than hiding the family.
