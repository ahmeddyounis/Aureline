# M5 loading / pending / warning-error / degraded state-block contract primitive

One reusable design-system primitive — the **degraded-state-application contract** — so every
claimed M5 form, background job row, banner, card, dense row, and review sheet renders its
`Loading`, `Pending`, `Warning/Error`, and `Degraded` states the same way, with the semantic
distinctions and submission-lineage / what-still-works / next-safe-action truth the acceptance
criteria demand.

This lane narrows the `degraded_state_application` family of the frozen
[shared-component-state-taxonomy component matrix](./m5-shared-component-state-taxonomy-component-matrix.md)
(M05-932) into a single resolver, the sibling of the interactive-state contract
([`m5_interactive_state_contract_primitive`](./m5_interactive_state_contract_primitive.md), M05-933)
and the selection-or-lock-state contract
([`m5_selection_or_lock_state_contract_primitive`](./m5_selection_or_lock_state_contract_primitive.md),
M05-934).

## What it guarantees

- **Loading vs Pending** — background work in progress (`loading`) never reads as a user-submitted
  action awaiting commit (`pending`). A pending action is attributed to the exact user action that
  triggered it (the resolver refuses a pending state that names no submission lineage), and a
  background loading state never claims a user submission (the resolver refuses one that does). They
  never share a cue, so a pending action can never masquerade as generic loading.
- **Warning vs Error** — a warning-severity state never collapses into an error-severity state. A
  `warning_error` state must decide whether it is a `warning` or an `error` (the resolver refuses an
  undecided one), and the two never share a consequence glyph.
- **Error vs Degraded** — a hard error never collapses into a reduced-capability degraded mode. A
  degraded state must preserve its what-still-works partial capability (the resolver refuses a
  degraded state that has lost it) and names its lowered freshness / certainty and fallback scope.
- **Consequence / recovery when explainable** — whenever a state is explainable (warning-error or
  degraded) the contract surfaces the state cause, the owner / block reason, and the next safe
  action. It never applies a silent, color-only spinner or a generic error toast.
- **Submission lineage & health for the activity center** — long-running, failed, or
  reduced-capability states carry enough structured truth (block kind, state, severity, cause,
  submission lineage, recovery availability) for the activity center, the support export, and
  screen-reader narration to reconstruct what happened without collapsing into generic spinners or
  error toasts.

## Resolver

`resolve_degraded_state_application_contract` takes one block's kind, the degraded state it is
entering, the warning-vs-error severity, the recovery-disclosure class and state cause behind it,
whether a recovery path is available, whether a degraded block retains partial capability, the
high-contrast context, and the opaque block-identity / state-style / submission-lineage / disclosure
references, and produces one `M5ResolvedDegradedStateContract`:

- the derived presentation posture (loading / pending / warning-error / degraded treatment),
  one-to-one from the state so no state collapses into another;
- the required non-color cues that carry the state beyond hue (the warning-error posture picks its
  glyph from the severity so a warning and an error never share a cue);
- the required disclosures the state must publish (state cause, owner, block reason, recovery
  action, and never a silent style-only change);
- the hard guarantees that loading and pending, warning and error, and error and degraded stay
  distinct, a pending action never masquerades as loading, submission lineage and what-still-works
  are preserved, the state is never color-only, and the state stays keyboard- and
  screen-reader-explainable.

It errors on `PendingWithoutSubmissionLineage`, `LoadingWithSubmissionLineage`,
`WarningErrorSeverityUnset`, `SeverityStateMismatch`, `DegradedWithoutPartialCapability`,
`MissingDisclosureDetail`, `NonDegradedState`, empty identity/state-style references, or forbidden
material.

## Matrix and artifacts

A single parity matrix — `M5DegradedStateContractPacket` — binds one row per claimed workflow block
(form, background job row, banner, card, dense row, review sheet) to the shared anatomy, states,
postures, severities, cues, disclosures, recovery-disclosure classes, state cause classes, export
fields, mandatory labels, and accessibility routes.

- Schema: [`schemas/ui/m5-loading-pending-degraded-state-contract.schema.json`](../../schemas/ui/m5-loading-pending-degraded-state-contract.schema.json)
- Support export: `artifacts/release/m5-loading-pending-degraded-state-contract-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-loading-pending-degraded-state-contract-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-loading-pending-degraded-state-contract-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-loading-pending-degraded-state-contract-primitive/`

The headless emitter `aureline_design_system_m5_loading_pending_degraded_state_contract` is the only
mint-from-truth path for the checked-in artifacts and fixtures.

```sh
cargo run -q -p aureline-design-system \
  --bin aureline_design_system_m5_loading_pending_degraded_state_contract -- support-export
```
