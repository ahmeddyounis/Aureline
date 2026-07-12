# M5 badge / chip / pill and popover controls

This is the first **implement lane** over the frozen
[M5 decision-feedback component matrix](../../schemas/ui/m5-decision-feedback-component-matrix.schema.json)
(see the [component contract](m5_decision_feedback_components_contract.md)). It turns the two most
ubiquitous compact decision/feedback primitives — the **badge / chip / pill** and the **popover** —
into resolvers that produce export-safe, honest projections across the claimed M5 help, settings,
review, marketplace, repair, and support-export surfaces.

- Rust source: `crates/aureline-ui/src/m5_badge_chip_pill_and_popover_expansion_and_anchored_focus_return/`
- Combined schema: [`schemas/ui/m5-badge-chip-pill-and-popover-controls.schema.json`](../../schemas/ui/m5-badge-chip-pill-and-popover-controls.schema.json)
- Per-component schemas: [`m5-badge-chip-pill.schema.json`](../../schemas/ui/m5-badge-chip-pill.schema.json),
  [`m5-popover.schema.json`](../../schemas/ui/m5-popover.schema.json)
- Proof packet: `artifacts/release/m5-badge-chip-pill-and-popover-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-badge-chip-pill-and-popover-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_badge`

A badge / chip / pill reads as a clean, legible primitive only when it names:

- the **concise label** (what the badge means), never unstated;
- the **badge expression** — text label, icon-with-text, count-with-label, status word, or removable
  chip — with **no-color-only** semantics, never color-only shorthand;
- the **state disposition** — info, success, warning, blocked, pending, degraded, acknowledged, or
  dismissed — from the one frozen taxonomy;
- the preserved **meaning taxonomy** — lifecycle state, support class, provider origin, policy source,
  or source freshness — that stays stable across surfaces, never unclassified;
- the **overflow behavior** that keeps the badge concise while always keeping an expansion path;
- an **expansion route** back to a **plain-language explanation** reachable by keyboard, screen reader,
  and export — never hover-only.

It degrades — never silently passes — when the label is unstated, the surface context, meaning
taxonomy, or overflow behavior is unresolved, meaning is encoded by color alone, no expansion route is
reachable, the plain-language explanation is missing, the explanation is reachable only on hover, or
the meaning taxonomy drifts across surfaces.

### `resolve_popover`

A popover reads as a clean, lightweight secondary control only when it names:

- its **accessible name** / identity, never unstated;
- its **dismissal behavior** — outside click, Escape, explicit close, focus-returns-to-trigger, or
  non-modal secondary — never the disallowed carries-only-instruction token;
- its **state disposition** and **surface context**;
- **dismissibility** and **keyboard operability**;
- **anchored focus return** to its trigger when closed.

It degrades when the popover is not dismissible, is not keyboard operable, does not return focus to its
trigger, carries the only critical workflow instruction, traps critical steps solely inside itself,
stops being a lightweight non-modal secondary surface, or hides its content behind hover only.

## Hard invariants (row-level, MUST be `false`)

- `badge_meaning_relies_on_color_alone`
- `badge_meaning_hidden_behind_hover_only`
- `popover_carries_only_critical_instruction`
- `popover_fails_to_return_focus_to_trigger`

## Acceptance criteria (proven by resolved examples, not asserted)

1. **Badge taxonomy and popover focus without hover-only truth gaps** — clean badges cover the
   expression and lifecycle/support/policy taxonomy grammar, a color-only badge degrades, a hover-only
   badge degrades, a popover that loses focus return degrades, and no clean badge is color-only or
   hover-only and no clean popover loses focus return.
2. **Plain-language reachability** — at least one clean badge reaches a plain-language explanation by
   keyboard/screen-reader/export, a hover-only badge and a plain-language-missing badge degrade, and no
   clean badge is hover-only.
3. **Badge / popover drift caught before release evidence turns green** — at least one clean badge and
   one clean popover both keep a reachable canonical trace, and a taxonomy-drift badge degrades.

## Regenerating the proof and fixtures

```text
cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- support-export
cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- csv
cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- report
cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- fixture-help-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- fixture-review-ui-preview-narrowed
cargo run -p aureline-ui --example dump_m5_badge_popover_controls -- validate
```
