# M5 trust-component accessibility parity contract

This lane is the **accessibility parity certification capstone** on top of the frozen
[M5 settings-row, capability-sheet, evidence-chronology, and chronology-export component
matrix](../components/m5_trust_chronology_components_contract.md). Where the matrix *freezes* the
six governed high-trust reusable components — the settings row, the permission/capability sheet, the
event/history row, the timeline group, the narrative summary card, and the chronology export
preview, with their settings-row states and source pills, capability consequence classes and scope
states, chronology verbs and provenance badges, chronology detail states and export fields,
non-visual accessibility routes, and mandatory labels — this lane *certifies* that, in every claimed
non-default accessibility condition, the settings rows, capability sheets, event rows, timeline
groups, and export previews stay keyboard- and screen-reader-reachable with focus that lands and
returns in order; stay legible and stable under high-zoom, high-contrast, and compact/comfortable
density with no truth left hover-only, color-only, or dropped when the surface compacts; keep a
durable static text alternative for anything conveyed by motion; and reconstruct the same source,
state, and chronology language from reusable fixtures and a support export rather than ad hoc visual
or screenshot checks.

The lane exists so that M5 can honestly claim mature trust/config/history quality: users never have
to guess where a setting's effective value came from, never mistake a color badge for a permission
consequence, never lose a chronology detail when a dense enterprise layout compacts, and never
mistake a hover-only reveal for a reachable truth — under keyboard, screen reader, high-zoom,
high-contrast, reduced-motion, or compact density — and a support reviewer reconstructs the same
component truth from a saved packet.

## Governed accessibility conditions

The certification proof covers exactly seven claimed non-default accessibility conditions, and
refuses to ship if any is missing. These are exactly the fixture-coverage cases the acceptance
criteria require: keyboard reach, focus order, narration, high-zoom, high-contrast, reduced-motion,
and compact/comfortable density:

- `keyboard_reach` — Keyboard-only reach
- `focus_order` — Focus order after open / dismiss
- `screen_reader_narration` — Screen-reader narration & live announcements
- `high_zoom` — High-zoom / large-text rendering
- `high_contrast` — High-contrast rendering
- `reduced_motion` — Reduced-motion rendering
- `density_compaction` — Compact / comfortable density

## Per-condition certification row

Each row certifies all six frozen trust components together and — pulled straight from the union
across the matrix's six component rows — the settings-row states, source pills, consequence classes,
capability scope states, chronology verbs, provenance badges, chronology detail states, chronology
export fields, accessibility routes, required labels, shell zones, responsive classes, window
classes, surface families, consumer surfaces, and downgrade triggers. The union covers the full six
accessibility routes (`keyboard_focusable`, `screen_reader_announced`, `non_hover_reachable`,
`pointer_optional`, `high_contrast_safe`, `support_exportable`) and the full six required labels
(`identity`, `state`, `keyboard_route`, `provenance`, `effective_value`, `audit_reopen_path`). It is
certified across four posture axes:

- **non-visual reach** — `keyboard_focus_and_narration_reachable` (green),
  `disclosed_reduced_reach_detail` (yellow: a long narration abbreviates or a focus-return lands one
  level up while every component stays keyboard- and screen-reader-reachable), or
  `truth_reachable_by_pointer_or_hover_only` (red: a component's truth is reachable only by pointer
  or hover, or focus does not return in order after dismiss).
- **zoom / contrast / density** — `legible_stable_under_zoom_contrast_density` (green),
  `disclosed_reduced_zoom_contrast_density_detail` (yellow: a label wraps to a shorter form or a
  decorative accent drops in compact density while every component stays legible and keeps a
  non-color-only affordance), or `truncated_color_only_or_lost_on_compaction` (red: a truth-bearing
  item is truncated, conveyed by color only, or lost when the surface compacts).
- **motion alternative** — `durable_text_alternative_present` (green),
  `disclosed_reduced_alternative_detail` (yellow, waiver-backed: a summarized text alternative for a
  small set of high-frequency live updates while a durable text path stays present), or
  `motion_only_affordance` (red: critical state or a live update is conveyed by motion only with no
  durable static text alternative).
- **support-export parity** — `component_truth_reconstructable` (green),
  `disclosed_partial_capture` (yellow: a low-priority component detail is trimmed while the reduction
  is disclosed), or `component_state_absent_from_capture` (red: the component state is absent from the
  support-export capture).

A per-row hard invariant, `never_hover_color_only_or_compaction_lost`, must hold: no critical
setting, capability, or chronology truth may be kept hover-only, color-only, or dropped on
compaction. `false` is a blocker regardless of the four axes.

## Derived status and auto-narrowing

Each row's green/yellow/red status is **derived**, never asserted. Any hard blocker (a blocked axis,
a broken invariant, or an incomplete route/label set) forces `red`; any disclosed narrowing forces
`yellow`; otherwise `green`. A disclosed reduced motion alternative must be backed by an active
waiver to stay yellow rather than red. Because this lane spans every component family, its
accessibility causes are recorded against the two family-agnostic frozen downgrade triggers the
matrix declares: `audit_truth_lost_off_primary_surface` (a reachable, legible, or motion-safe truth
is lost off the primary surface) and `proof_stale` (the support export can no longer reconstruct the
truth).

## Boundary and artifacts

The records carry no raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials
— only stable ids, closed vocabulary, counts, refs, and short labels. The Rust validator in
`crates/aureline-shell` is the authoritative gate; the boundary schema is
[`schemas/shell/m5-trust-component-accessibility-parity.schema.json`](../../schemas/shell/m5-trust-component-accessibility-parity.schema.json)
and documents the shape.

The headless emitter `aureline_shell_m5_trust_component_accessibility_parity` is the only
mint-from-truth path for the published artifacts:

- Packet: [`artifacts/release/m5-trust-component-accessibility-parity-proof/packet.json`](../../artifacts/release/m5-trust-component-accessibility-parity-proof/packet.json)
- Dashboard: [`artifacts/release/m5-trust-component-accessibility-parity-proof/dashboard.json`](../../artifacts/release/m5-trust-component-accessibility-parity-proof/dashboard.json)
- Support export: [`artifacts/release/m5-trust-component-accessibility-parity-proof/support_export.json`](../../artifacts/release/m5-trust-component-accessibility-parity-proof/support_export.json)
- CSV: [`artifacts/release/m5-trust-component-accessibility-parity-proof/matrix.csv`](../../artifacts/release/m5-trust-component-accessibility-parity-proof/matrix.csv)
- Markdown report: [`artifacts/shell/m5-trust-component-accessibility-parity.md`](../../artifacts/shell/m5-trust-component-accessibility-parity.md)

The protected fixtures under
[`fixtures/ui/m5-trust-component-accessibility-parity/packet.json`](../../fixtures/ui/m5-trust-component-accessibility-parity/packet.json)
(plus `dashboard.json`, `support_export.json`, and `compact.txt`) are asserted bit-for-bit equal to
the seed by the integration test
`crates/aureline-shell/tests/m5_trust_component_accessibility_parity_fixtures.rs`.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_accessibility_parity -- validate
cargo test -p aureline-shell --lib m5_trust_component_accessibility_parity::
cargo test -p aureline-shell --test m5_trust_component_accessibility_parity_fixtures
```
