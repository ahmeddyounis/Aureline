# M5 state-distinction explanation helper primitive

One reusable design-system primitive — the **state-distinction explanation helper** — so every
claimed M5 onboarding / help surface, blocked-action explanation row, settings row, activity row,
and workspace-entry surface can explain the most easily confused state distinctions **in place**,
with the same taxonomy words the components themselves expose, instead of forcing users into external
docs or tribal knowledge.

This lane builds explanation helpers over the four state precedence / distinctness rules the frozen
[shared-component-state-taxonomy component matrix](./m5-shared-component-state-taxonomy-component-matrix.md)
(M05-932) already owns, alongside the interactive-state contract
([`m5_interactive_state_contract_primitive`](./m5_interactive_state_contract_primitive.md), M05-933),
the selection-or-lock-state contract
([`m5_selection_or_lock_state_contract_primitive`](./m5_selection_or_lock_state_contract_primitive.md),
M05-934), and the loading / pending / warning-error / degraded state-block contract
([`m5_loading_pending_degraded_state_contract_primitive`](./m5_loading_pending_degraded_state_contract_primitive.md),
M05-935).

## The four confusable distinctions

Each distinction is exactly one of the taxonomy's frozen precedence / distinctness rules, so the
helper never invents a distinction the taxonomy does not already own:

- **current vs selected** (`current_distinct_from_selected`) — the live route / context owner is not
  the same as a durable selection.
- **read-only vs disabled** (`read_only_over_disabled`) — an inspectable-but-not-editable control is
  not a non-actionable one; inspectability is preserved.
- **locked vs disabled** (`locked_over_disabled`) — an explainable policy / trust / ownership lock is
  not a bare disabled control; the lock stays explainable.
- **pending vs loading** (`pending_distinct_from_loading`) — a submitted user action awaiting commit
  is not generic background work.

## What it guarantees

- **Explained in place** — every explanation is delivered on the surface itself (an inline chip, an
  expanded drawer, or a blocked/limited-state copy object), never deferred to external docs.
- **States stay distinct** — the primary and contrasted states never collapse into one another.
- **No one-off language** — each explanation links back to the canonical shared taxonomy (the
  resolver refuses one that links to no taxonomy), so no surface improvises a private state label or
  contradicts the taxonomy.
- **Blocked-action & contextual-teaching alignment** — the blocked/limited-copy delivery names the
  state cause, the owner / block reason, and the next safe action, keeping blocked-action help and
  contextual teaching aligned with the same component-state truth the components render. It applies
  only to a distinction with a blocked or limited side (`read-only`/`disabled`, `locked`/`disabled`,
  `pending`/`loading`), never to `current`/`selected`.
- **Never color-only, always keyboard/screen-reader explainable** — every delivery publishes a
  non-empty non-color cue set and stays reachable and announced through non-visual routes.

## Resolver

`resolve_state_distinction_explanation` takes one consumer surface, the confusable distinction it
teaches, the delivery form, the recovery-disclosure class and state cause behind the confusable
state, whether a recovery path is available, the high-contrast context, and the opaque
explanation-identity / taxonomy / distinction-copy / blocked-limited-copy references, and produces
one `M5ResolvedStateExplanation`:

- the derived precedence rule, primary state, and contrasted state, one-to-one from the distinction;
- the required non-color cues per delivery form (the chip names the primary state and marks the
  distinction; the drawer additionally names the contrasted state and links to the taxonomy; the
  blocked/limited copy names the blocked side and its recovery affordance);
- the required disclosures per delivery form (every form forbids a silent style-only change; the
  drawer adds state cause and recovery action; the blocked/limited copy adds the owner and block
  reason);
- the hard guarantees that the two states stay distinct, the explanation is delivered in place,
  invents no one-off language, stays aligned with the taxonomy and with blocked-action help, is never
  color-only, and stays keyboard- and screen-reader-explainable.

It errors on `EmptyExplanationIdentity`, `EmptyTaxonomyRef`, `EmptyDistinctionCopyRef`,
`BlockedLimitedCopyOnUnblockableDistinction`, `BlockedLimitedCopyMissing`,
`BlockedLimitedCopyOnNonBlockedDelivery`, `RecoveryClassMismatch`, or forbidden material.

## Matrix and artifacts

A single parity matrix — `M5StateExplanationPacket` — binds one row per claimed consumer surface
(onboarding/help, blocked-action row, settings row, activity row, workspace-entry surface) to the
shared anatomy, distinctions, precedence rules, delivery forms, cues, disclosures,
recovery-disclosure classes, state cause classes, export fields, mandatory labels, and accessibility
routes.

- Schema: [`schemas/ui/m5-state-distinction-explanation-helper.schema.json`](../../schemas/ui/m5-state-distinction-explanation-helper.schema.json)
- Support export: `artifacts/release/m5-state-distinction-explanation-helper-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-state-distinction-explanation-helper-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-state-distinction-explanation-helper-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-state-distinction-explanation-helper-primitive/`

The headless emitter `aureline_design_system_m5_state_distinction_explanation_helper` is the only
mint-from-truth path for the checked-in artifacts and fixtures.

```sh
cargo run -q -p aureline-design-system \
  --bin aureline_design_system_m5_state_distinction_explanation_helper -- support-export
```
