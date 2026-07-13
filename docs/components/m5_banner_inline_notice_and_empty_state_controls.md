# M5 banner / inline-notice and empty-state controls

This is the third **implement lane** over the frozen
[M5 decision-feedback component matrix](../../schemas/ui/m5-decision-feedback-component-matrix.schema.json)
(see the [component contract](m5_decision_feedback_components_contract.md)). It turns the two
scoped-state explanation primitives — the **banner / inline notice** and the **empty state** — into
resolvers that produce export-safe, honest projections across the claimed M5 review, settings,
update / install, support, shell, and support-export surfaces.

- Rust source: `crates/aureline-ui/src/m5_banner_inline_notice_and_empty_state_scoped_cause_and_next_action/`
- Combined schema: [`schemas/ui/m5-banner-inline-notice-and-empty-state-controls.schema.json`](../../schemas/ui/m5-banner-inline-notice-and-empty-state-controls.schema.json)
- Per-component schemas: [`m5-banner-inline-notice.schema.json`](../../schemas/ui/m5-banner-inline-notice.schema.json),
  [`m5-empty-state.schema.json`](../../schemas/ui/m5-empty-state.schema.json)
- Proof packet: `artifacts/release/m5-banner-inline-notice-and-empty-state-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-banner-inline-notice-and-empty-state-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_banner`

A banner / inline notice reads as a clean, scoped, actionable notice only when it names:

- its **label** / identity, never unstated;
- its **notice scope** — page-scoped, section-scoped, field-inline, global-system, or
  actionable-with-next-step — never the disallowed unscoped / color-only token;
- its **state disposition** — info, success, warning, blocked, pending, degraded, acknowledged, or
  dismissed — from the one frozen taxonomy;
- its **degraded-state variant** — blocked by policy, partial capability, stale data, offline, or
  restricted access — from the one shared vocabulary;
- its **cause** and **what still works** despite the limitation;
- a **primary next action** and a **support / help back-link**;
- copy that **avoids generic failure language** ("something went wrong");
- an explanation that is **reconstructable from the support export**, never screenshot-only.

It degrades — never silently passes — when the label is unstated, the surface context is unresolved,
the scope is unscoped / color-only, the degraded-state variant is unresolved, the cause is unstated,
what-still-works is unstated, the primary next action is missing, the support / help back-link is
missing, generic failure language is used, the explanation cannot be reconstructed from the export, or
the proof packet is stale.

### `resolve_empty_state`

An empty state reads as a clean, purpose-named, next-action-honest card only when it names:

- its **label** / identity, never unstated;
- its **empty-state purpose** — explains purpose, explains current emptiness, offers next action,
  first-run guidance, or filtered no-results — never the disallowed blank-no-explanation token;
- its **state disposition**, **surface context**, and shared **degraded-state variant**;
- **what the area is for** and **why it is empty now** (the emptiness reason);
- the **best next action**;
- copy that **avoids decorative marketing filler** and **generic failure language**;
- an explanation that is **reconstructable from the support export**.

It degrades when the label is unstated, the surface context is unresolved, the purpose is the
blank-no-explanation token, the degraded-state variant is unresolved, the purpose is unstated, the
emptiness reason is unresolved or unexplained, the best next action is missing, decorative filler is
used, generic failure language is used, or the explanation cannot be reconstructed from the export.

## Hard invariants (row-level, MUST be `false`)

- `banner_relies_on_color_alone_for_meaning`
- `banner_uses_generic_failure_language`
- `empty_state_blanks_pane_without_next_action`
- `empty_state_uses_decorative_marketing_filler`

## Acceptance criteria (proven by resolved examples, not asserted)

1. **Generic failure language avoided and next safe action exposed** — every clean banner and empty
   state avoids generic failure language and exposes a next action, a generic-language banner degrades,
   a generic-language empty state degrades, a next-action-missing banner degrades, a next-action-missing
   empty state degrades, and no clean primitive uses generic language or omits its next action.
2. **Scope and degraded-state vocabulary stay consistent** — clean banners cover the page-scoped /
   section-scoped / actionable-with-next-step scope grammar, the blocked-by-policy / partial-capability /
   stale-data / offline / restricted-access degraded-state variants are all covered by clean examples, an
   unscoped / color-only banner degrades, and a variant-unresolved example degrades.
3. **Reconstructable from the export** — at least one clean banner stays reconstructable off-screenshot
   with a support / help back-link, at least one clean empty state stays reconstructable off-screenshot,
   a not-reconstructable banner degrades, and a not-reconstructable empty state degrades.

## Regenerating the proof and fixtures

```text
cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- support-export
cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- csv
cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- report
cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- fixture-review-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- fixture-updates-ui-preview-narrowed
cargo run -p aureline-ui --example dump_m5_banner_empty_state_controls -- validate
```
