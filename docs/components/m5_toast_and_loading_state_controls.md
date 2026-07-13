# M5 toast and loading-state controls

This is the fourth **implement lane** over the frozen
[M5 decision-feedback component matrix](../../schemas/ui/m5-decision-feedback-component-matrix.schema.json)
(see the [component contract](m5_decision_feedback_components_contract.md)). It turns the two
transient-acknowledgement and loading-honesty primitives — the **toast** and the **loading state** —
into resolvers that produce export-safe, honest projections across the claimed M5 shell, review,
settings, help, support, and support-export surfaces.

- Rust source: `crates/aureline-ui/src/m5_toast_and_loading_state_acknowledgement_and_loading_fidelity/`
- Combined schema: [`schemas/ui/m5-toast-and-loading-state-controls.schema.json`](../../schemas/ui/m5-toast-and-loading-state-controls.schema.json)
- Per-component schemas: [`m5-toast.schema.json`](../../schemas/ui/m5-toast.schema.json),
  [`m5-loading-state.schema.json`](../../schemas/ui/m5-loading-state.schema.json)
- Proof packet: `artifacts/release/m5-toast-and-loading-state-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-toast-and-loading-state-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_toast`

A toast reads as a clean, short-lived, durably-linked acknowledgement only when it names:

- its **label** / identity, never unstated;
- its **toast durability** — transient acknowledgment, mirrored to the activity center, dismissible by
  the user, auto-dismiss timed, or action retained elsewhere — never the disallowed toast-only-truth
  token;
- its **state disposition** — info, success, warning, blocked, pending, degraded, acknowledged, or
  dismissed — from the one frozen taxonomy;
- its **acknowledgement scope** — transient confirmation, background handoff, reversible-action ack,
  non-blocking notice, or durable-outcome ack — from the one shared vocabulary;
- a **durable-object backlink** (activity center, review queue, settings record, support record, or
  notification center) whenever the outcome still matters after the toast is dismissed;
- at most **one bounded action**, where present;
- copy that **acknowledges transiently** and **never becomes the only durable truth**;
- an explanation that is **reconstructable from the support export**, never screenshot-only.

It degrades — never silently passes — when the label is unstated, the surface context is unresolved,
the durability is toast-only truth, the acknowledgement scope is unresolved, the acknowledgement is not
short-lived, the outcome matters but the durable backlink is missing, the backlink target is
unresolved, a present action is not bounded, the toast is used as the only durable truth, the
explanation cannot be reconstructed from the export, or the proof packet is stale.

### `resolve_loading_state`

A loading state reads as a clean, partial-preserving, readiness-honest pane only when it names:

- its **label** / identity, never unstated;
- its **loading fidelity** — skeleton preserves layout, partial data retained, inline progress scoped,
  determinate progress, or indeterminate spinner scoped — never the disallowed full-screen-spinner
  token;
- its **state disposition** and **surface context**;
- its **loading treatment** — skeleton, retained previous content, stable placeholder, partial results
  streaming, or blocked waiting — from the one shared vocabulary rather than one spinner treatment;
- its **readiness posture** — warming not ready, partially ready, blocked needs action, ready complete,
  or stalled retryable;
- **useful partial content preserved** without blanking a useful pane;
- copy that **never overclaims readiness** while data is warming or blocked;
- what the pane is **loading and why**;
- an explanation that is **reconstructable from the support export**.

It degrades when the label is unstated, the surface context is unresolved, the fidelity is the
full-screen-spinner token, the loading treatment is unresolved, the readiness posture is unresolved, a
useful pane is blanked, partial content is not preserved, readiness is overclaimed, the purpose is
unstated, or the explanation cannot be reconstructed from the export.

## Hard invariants (row-level, MUST be `false`)

- `toast_represents_durable_work_as_toast_only`
- `toast_lacks_durable_backlink_when_outcome_matters`
- `loading_blanks_useful_pane`
- `loading_uses_full_screen_spinner_when_partial_capable`

## Acceptance criteria (proven by resolved examples, not asserted)

1. **Toasts point back to a durable object when the result matters** — every clean toast whose outcome
   matters after dismissal carries a durable backlink, no clean toast is the only durable truth, at
   least one clean toast proves the durable backlink, a backlink-missing toast degrades, and a
   toast-only-truth toast degrades.
2. **Loading states preserve partial content and never overclaim readiness** — every clean loading state
   with partial content preserves it without blanking a useful pane, no clean loading state overclaims
   readiness, clean loading states cover the skeleton / retained-previous-content / stable-placeholder /
   partial-results-streaming / blocked-waiting treatment grammar, a blanked-pane example degrades, a
   full-screen-spinner example degrades, and a readiness-overclaim example degrades.
3. **Reconstructable from the export** — at least one clean toast stays reconstructable off-screenshot
   with a durable backlink, at least one clean loading state stays reconstructable off-screenshot, a
   not-reconstructable toast degrades, and a not-reconstructable loading state degrades.

## Regenerating the proof and fixtures

```text
cargo run -p aureline-ui --example dump_m5_toast_loading_controls -- support-export
cargo run -p aureline-ui --example dump_m5_toast_loading_controls -- csv
cargo run -p aureline-ui --example dump_m5_toast_loading_controls -- report
cargo run -p aureline-ui --example dump_m5_toast_loading_controls -- fixture-review-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_toast_loading_controls -- fixture-support-ui-preview-narrowed
cargo run -p aureline-ui --example dump_m5_toast_loading_controls -- validate
```
