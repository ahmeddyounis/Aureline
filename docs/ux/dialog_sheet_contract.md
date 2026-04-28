# Dialog, Sheet, and Review-Surface Contract

This document freezes how Aureline chooses between a modal dialog, a
sheet, and a dedicated review surface. It exists so destructive,
permission-bearing, setup, and evidence-heavy flows stay reviewable
without product-owned nested overlays or vague confirmations.

The contract is normative. Where this document disagrees with the UI /
UX Spec or design-system source language it projects, the source spec
wins and this document plus its schema and fixtures update in the same
change. Where this document disagrees with a downstream flow's private
dialog, sheet, or takeover rule, this document wins.

Companion artifacts:

- [`/schemas/ux/review_surface.schema.json`](../../schemas/ux/review_surface.schema.json)
  - boundary schema for one `review_surface_record`.
- [`/fixtures/ux/dialog_sheet_cases/`](../../fixtures/ux/dialog_sheet_cases/)
  - worked cases for binary destructive confirmation, structured
  permission review, dense evidence takeover, multi-step setup handoff,
  product-owned nested overlay denial, and platform-auth exception.

This contract composes with, and does not replace:

- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for consequence class, required visible fields, safe default focus,
  focus return, representation, and responsive fallback.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  for durable job rows, attention items, history linkage, notification
  routing, dismissal verbs, and reopen semantics.
- [`/docs/ux/transient_surface_contract.md`](./transient_surface_contract.md)
  for tooltip, hovercard, popover, peek, pinning, and promotion.
- [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md)
  for staged form review, validation, probes, stale apply gates, and
  support handoff.
- [`/docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  for trust, policy, permission, side-effect, denial, and revocation
  prompt anatomy.

Normative source sections projected here include
`.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 9.8, 9.14, 16.18, and
Appendix EU; `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
sections 16.8 and 16.18; and the PRD / architecture rule that feedback is
inline first, status or task surface second, modal last.

## Scope

This contract applies when an Aureline surface needs to interrupt,
review, or take over the user's current flow for one of these reasons:

- binary destructive confirmation;
- structured permission, trust, policy, auth, or entitlement review;
- dense evidence, diff, batch, governance, or recovery review;
- multi-step setup that preserves workspace context;
- conflict choice or other short structured decision; or
- a platform-mandated auth or file-picker overlay.

Transient previews, menus, tooltips, hovercards, and command-palette
rows are out of scope except when they promote into one of these
surfaces.

## Surface Classes

| Surface class | Use for | Required behavior |
|---|---|---|
| `modal_dialog` | Short binary or short structured decision that gates the current workflow. | Origin, target, consequence block, specific actions, safe default focus, and focus return all visible or recorded. |
| `window_attached_sheet` / `side_sheet` / `full_sheet` | Structured review, setup, compare, permission, or inspectable detail that benefits from keeping the workspace visible. | Persistent title, back-to-source context, explicit close path, keyboard-complete scan space, and durable handoff when work continues. |
| `dedicated_review_surface` / `full_surface_takeover` | Diff, evidence, recovery, or governance review that needs more room than a sheet. | Origin cue, breadcrumb or title context, compare-friendly layout, restore behavior, and a route back to the invoking object. |
| `platform_auth_dialog` / `platform_file_picker_dialog` | Host-owned auth, sign-in, or file choice that the product cannot render inside its own surface. | Product explanation remains adjacent before launch and after return; the host overlay is the only allowed stacked overlay exception. |

No product-owned flow may stack a second dialog or sheet over a product
dialog or sheet. It must update the current surface, replace it with a
larger surface, or hand off to a durable target. Platform auth and
platform file pickers are the only allowed nested overlays.

## Selection Matrix

| Need | Preferred surface | Escalate to dedicated review surface when |
|---|---|---|
| Binary destructive confirm | `modal_dialog` | The decision is not truly binary, affects a batch, needs a diff/table/log, has partial-failure handling, or needs durable post-apply inspection. |
| Structured permission review | `window_attached_sheet`, `side_sheet`, or `full_sheet` | Capability groups, policy deltas, revocation routes, target identity, and deny-path behavior cannot fit with the primary action visible. |
| Dense evidence or diff review | `full_sheet`, `dedicated_review_surface`, or `full_surface_takeover` | The user must compare large diffs, evidence tables, logs, generated artifacts, governance records, or batch membership side by side. |
| Multi-step setup with preserved workspace context | `window_attached_sheet` or `full_sheet` | The setup becomes a workspace entry flow, recovery workflow, or long review that must survive navigation or restart. |
| Conflict choice | `modal_dialog` for a short choice; sheet for structured conflict review. | More than one target, policy source, or recovery path must be compared. |
| Platform auth or file picker | `platform_auth_dialog` or `platform_file_picker_dialog`. | Never escalates by product nesting; the product resumes into the invoking sheet or a durable return target. |

A modal is acceptable only when the user can read the whole decision,
its consequence, and all available actions without losing the invoking
object. Dense inspection in a cramped modal is non-conforming.

## Dedicated Surface Requirements

A dedicated review surface is required when any of these are true:

- the review includes diffs, logs, tables, evidence packets, large
  batch membership, or generated artifacts that need scanning or
  comparison;
- the user must preserve comments, expanded rows, scroll position,
  selected hunks, or staged choices across navigation or restart;
- the action crosses provider, remote, policy, identity, shared, or
  high-blast boundaries and the consequence block cannot remain visible
  with the primary action inside a sheet;
- partial failure, rollback, recovery, or verification is meaningful
  enough to inspect after apply; or
- multiple actors, owners, tenants, workspaces, roots, or review objects
  must remain visible at the same time.

Dedicated surfaces are still review surfaces. They must preserve origin,
focus return or a truthful re-entry target, action specificity, and
durable job handoff.

## Origin and Context

Every dialog, sheet, and takeover opens from a named invoking object or
command. The surface record must preserve:

- invoking command or control;
- owning window;
- workspace or workset scope;
- canonical target object;
- source surface;
- return target; and
- title, subtitle, breadcrumb, or provenance copy that makes the origin
  visible.

A surface that appears as a random new screen without a relationship to
the invoking object is non-conforming. If the invoking object disappears
while the surface is open, the surface returns focus to the nearest safe
ancestor or renders a placeholder that explains why exact return is no
longer possible.

## Consequence and Action Rules

For destructive, cross-boundary, permission-bearing, or high-blast
flows:

1. The consequence block remains visible at the same time as the
   primary action. Scroll-only warnings are not enough.
2. Required visible fields from the interaction-safety contract remain
   visible under responsive fallback: target, actor or requester,
   authority, consequence, scope, recovery, policy source, hidden or
   blocked counts, and basis freshness as applicable.
3. The initial focus lands on the safest sensible action. A destructive
   primary action does not take the default Enter path unless the user
   is already inside an explicit destructive review.
4. Labels name the verb and target scope. `Yes`, `No`, `OK`,
   `Continue`, `Apply`, `Accept`, and `Submit` alone are
   non-conforming for product-native consequential flows.
5. Partial failure, skipped members, blocked members, and hidden
   members are named before commit when they can occur.

Platform host dialogs that force generic labels must be paired with a
product-owned explanatory label before launch and a reviewed return
state after completion.

## Dismissal and Draft Rules

Close, dismiss, and `Esc` behavior is explicit:

| Situation | Required behavior |
|---|---|
| No user-authored or staged state | `Cancel` or close returns without mutation and emits focus return. |
| Unsaved structured input | `Esc` and close route to explicit discard, save-draft, or continue-review choices. |
| Partially reviewed consequence | Dismiss records `review_dismissed_no_apply`; reopening from the same object preserves scroll and disclosure state where practical. |
| Meaningful scroll, expanded rows, hunk choices, or setup step | Reopen preserves review state, or a durable history item explains what was not preserved. |
| Invoker removed while open | Focus returns to nearest safe ancestor, current batch or detail owner, or an announced placeholder. |
| Platform host flow canceled | The invoking surface resumes with host-owned cancellation explained; no silent product mutation occurs. |

Dismissing the presentation layer never resolves the underlying object
unless the action label explicitly says so and the owning object accepts
that mutation through its own contract.

## Long-Running Handoff

Dialogs and sheets do not own long-running truth. If an action started
from a dialog, sheet, or takeover continues after the initiating surface
closes, the product must mint a durable target before close:

- `durable_job_row` for running build, test, update, install, restore,
  sync, download, remote attach, notebook, AI apply, or setup work;
- `history_lane_row` or `mutation_journal_entry` for completed,
  failed, partial, or reviewable mutation history;
- `activity_center_item` for attention and reopen routing; or
- `inspected_object_state` when the canonical object itself owns the
  running state.

The handoff target carries at least label, actor or subsystem, progress
state, current phase or step, open-details action, and cancel or retry
where possible. Cost, policy, network, trust, evidence, and recovery
impact link to the relevant detail view or evidence row.

A long-running action may show a spinner briefly while the job row is
being minted, but a vanishing spinner with no durable row, history item,
or inspected object state is non-conforming.

## Nested Overlay Policy

Product-owned nested dialogs and nested sheets are prohibited:

- A permission sheet may not open a product confirmation dialog on top
  of itself. It updates in place, expands, or promotes to a dedicated
  review surface.
- A destructive modal may not open a second product modal for "Are you
  sure?". The first surface must carry the real consequence and action.
- A setup sheet may not open a product progress dialog. It hands off to
  a durable job row or keeps progress in the same sheet.

Allowed exceptions:

- platform-mandated authentication or sign-in;
- platform file picker or save picker.

Even in an exception, the product must preserve origin, explain why the
host-owned overlay is needed, and resume into the invoking surface or a
durable return target.

## Record Shape

Every conforming surface emits one `review_surface_record` when opened,
promoted, denied, or handed off. The schema freezes these groups:

- `surface_selection` - the matrix need, chosen class, size, dedicated
  takeover requirement, and rationale;
- `origin` - invoking object, command, window, workspace scope,
  breadcrumb or title preservation, and return target;
- `consequence_visibility` - required visible fields, consequence block
  posture, safe default focus, and responsive chrome omissions;
- `action_rows` - specific action labels, role, focus default, platform
  host status, and resulting state;
- `dismissal` - close, `Esc`, discard, focus return, and state
  preservation;
- `overlay_stack_policy` - parent surface, child overlay request,
  platform exception status, and denial reason;
- `long_job_handoff` - durable target requirements for work that
  outlives the surface; and
- policy, redaction, client scope, and emitted time.

## Non-Conforming Patterns

- A destructive path whose only labels are `Yes` and `No`.
- A cross-boundary action whose consequence block scrolls away from the
  primary action.
- A dense diff, log, evidence table, or batch review packed into a
  small modal.
- A product-owned dialog opened on top of another product dialog or
  sheet.
- `Esc` silently discarding staged input or partially reviewed choices.
- A long-running apply that closes into no durable job row, history row,
  mutation journal, or inspected object state.
- A focus trap that loses the invoking control without returning to a
  safe ancestor or announced placeholder.
