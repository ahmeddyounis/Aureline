# M5 dialog / sheet and consequence-block controls

This is the second **implement lane** over the frozen
[M5 decision-feedback component matrix](../../schemas/ui/m5-decision-feedback-component-matrix.schema.json)
(see the [component contract](m5_decision_feedback_components_contract.md)). It turns the two
highest-risk confirmation primitives — the **dialog / sheet** and the **consequence block** — into
resolvers that produce export-safe, honest projections across the claimed M5 review, settings,
update / install, repair, shell, and support-export surfaces.

- Rust source: `crates/aureline-ui/src/m5_dialog_sheet_and_consequence_block_rationale_scope_and_rollback_continuity/`
- Combined schema: [`schemas/ui/m5-dialog-sheet-and-consequence-block-controls.schema.json`](../../schemas/ui/m5-dialog-sheet-and-consequence-block-controls.schema.json)
- Per-component schemas: [`m5-dialog-sheet.schema.json`](../../schemas/ui/m5-dialog-sheet.schema.json),
  [`m5-consequence-block.schema.json`](../../schemas/ui/m5-consequence-block.schema.json)
- Proof packet: `artifacts/release/m5-dialog-sheet-and-consequence-block-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-dialog-sheet-and-consequence-block-controls/`

The Rust validator in `crates/aureline-ui` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_dialog`

A dialog / sheet reads as a clean, trustworthy confirmation only when it names:

- its **title** / identity, never unstated;
- its **action model** — named specific actions, primary-and-cancel, destructive-confirm-named,
  rationale-and-scope-stated, or dismissible-safe — never the disallowed generic-yes-no token;
- its **state disposition** — info, success, warning, blocked, pending, degraded, acknowledged, or
  dismissed — from the one frozen taxonomy;
- its **rationale** and its **named scope**;
- **explicit action labels** (never generic Yes/No);
- a **safe initial focus** target that never auto-fires a destructive action;
- a **cancel / escape path**;
- **focus return** to its invoker when reopened from status, the activity center, support, or a deep
  link (the reopen origin is carried so a broken reopen continuity degrades honestly);
- a **help / docs hook**.

It degrades — never silently passes — when the title is unstated, the surface context is unresolved,
the action model is generic-yes-no, the rationale or scope is unstated, the actions are not explicitly
named, the initial focus is unsafe or unresolved, the cancel path is missing, focus does not return on
reopen, the reopen origin is unresolved, or the help / docs hook is missing.

### `resolve_consequence`

A consequence block reads as a clean, blast-radius-named, rollback-honest primitive only when it names:

- its **affected-object / scope label**, never unstated;
- its **disclosure** — named blast radius, rollback available, rollback unavailable stated, help path
  present, or explicit named actions — never the disallowed generic-yes-no token;
- its **state disposition** and **surface context**;
- its **named blast radius** — single object, multiple objects, workspace-wide, deployment-wide, or
  irreversible-external;
- its **reversibility posture** and, where relevant, a **partial-success or irreversible note**;
- a stated **rollback / help posture**;
- an explanation **reachable by keyboard, screen reader, and export** — never screenshot-only.

It degrades when the label is unstated, the surface context is unresolved, the disclosure is
generic-yes-no, the affected object is unnamed, the blast radius or reversibility is unresolved, the
rollback posture is unstated, a required partial / irreversible note is missing, the block reduces to
generic Yes/No ambiguity, or the explanation is reachable only via a screenshot.

## Hard invariants (row-level, MUST be `false`)

- `dialog_uses_generic_yes_no_in_high_risk`
- `dialog_focus_fails_to_return_on_reopen`
- `consequence_omits_named_blast_radius`
- `consequence_reduces_to_generic_yes_no`

## Acceptance criteria (proven by resolved examples, not asserted)

1. **Rationale, scope, and explicit actions exposed consistently** — clean dialogs cover the
   named-specific-actions / primary-and-cancel / destructive-confirm-named action-model grammar and
   always state rationale and scope, a generic-yes-no dialog degrades, a rationale-missing dialog
   degrades, a scope-missing dialog degrades, and no clean dialog is generic-yes-no or missing
   rationale / scope.
2. **Focus, escape / cancel, and focus-return stability** — at least one clean dialog keeps a safe
   initial focus, a cancel path, and focus return on reopen, a safe-focus-missing dialog degrades, a
   cancel-missing dialog degrades, a focus-return-broken dialog degrades, and no clean dialog loses safe
   focus, cancel, or focus return.
3. **Explainable without screenshots** — at least one clean consequence names its blast radius and
   rollback posture reachable off-screenshot, a blast-radius-unresolved consequence degrades, a
   screenshot-only consequence degrades, and at least one clean dialog and one clean consequence both
   keep a canonical trace.

## Regenerating the proof and fixtures

```text
cargo run -p aureline-ui --example dump_m5_dialog_consequence_controls -- support-export
cargo run -p aureline-ui --example dump_m5_dialog_consequence_controls -- csv
cargo run -p aureline-ui --example dump_m5_dialog_consequence_controls -- report
cargo run -p aureline-ui --example dump_m5_dialog_consequence_controls -- fixture-review-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_dialog_consequence_controls -- fixture-updates-ui-preview-narrowed
cargo run -p aureline-ui --example dump_m5_dialog_consequence_controls -- validate
```
