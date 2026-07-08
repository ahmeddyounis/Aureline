# M5 selection-or-lock-state contract primitive

One reusable design-system primitive — the **selection-or-lock-state contract** — so every claimed
M5 tab, tree, dense list, grid/table, badge, settings row, and inspector entry renders its
`Selected`, `Current`, `Disabled`, `Read-only`, and `Locked` states the same way, with the semantic
distinctions and owner-reason-recovery truth the acceptance criteria demand.

This lane narrows the `selection_or_lock_state` family of the frozen
[shared-component-state-taxonomy component matrix](./m5-shared-component-state-taxonomy-component-matrix.md)
(M05-932) into a single resolver, the sibling of the interactive-state contract
([`m5_interactive_state_contract_primitive`](./m5_interactive_state_contract_primitive.md), M05-933).

## What it guarantees

- **Selected vs Current** — a merely selected item never reads as the actively current one. The
  selection is carried by a selection marker; the current item by a distinct current-location
  indicator. They never share a cue, so they can never collapse.
- **Read-only vs Disabled** — an inspectable-but-read-only item never collapses into a silently
  disabled one. A read-only state must preserve its inspectability (the resolver refuses a read-only
  state that has lost it) and names why it is read-only and how to gain edit rights.
- **Locked vs Disabled** — an explicit trust/policy lock is never hidden behind a plain disabled
  treatment. A locked state must name its owner (`policy_lock` / `trust_lock` / `permission_lock` /
  `ownership_lock` / `source_lock`); a disabled state must not carry a lock owner (the resolver
  refuses it — that state should be modeled as `locked`).
- **Owner / cause / recovery when explainable** — whenever a state is explainable (disabled,
  read-only, or locked) the contract surfaces the state cause, the owner / block reason, and the
  recovery action. It never applies a silent, color-only style change.

## Resolver

`resolve_selection_or_lock_state_contract` takes one item's kind, the selection-or-lock state it is
entering, the lock owner and state cause behind it, whether a recovery path is available, whether a
read-only item stays inspectable, the high-contrast context, and the opaque item-identity /
state-style / disclosure references, and produces one `M5ResolvedSelectionOrLockStateContract`:

- the derived presentation posture (selected / current / disabled / read-only / locked treatment),
  one-to-one from the state so no state collapses into another;
- the required non-color cues that carry the state beyond hue;
- the required disclosures the state must publish (state cause, owner, block reason, recovery
  action, and never a silent style-only change);
- the hard guarantees that selected and current stay distinct, read-only preserves inspectability, a
  lock is never hidden behind disabled, the state is never color-only, owner/recovery is named when
  explainable, and the state stays keyboard- and screen-reader-explainable.

It errors on `LockWithoutOwner`, `DisabledMaskingLock`, `ReadOnlyNotInspectable`,
`MissingDisclosureDetail`, `NonSelectionOrLockState`, empty identity/state-style references, or
forbidden material.

## Matrix and artifacts

A single parity matrix — `M5SelectionOrLockStateContractPacket` — binds one row per claimed
collection surface (tab, tree item, dense list row, grid/table row, badge, settings row, inspector
entry) to the shared anatomy, states, postures, cues, disclosures, lock owner classes, state cause
classes, export fields, mandatory labels, and accessibility routes.

- Schema: [`schemas/ui/m5-selection-lock-state-contract.schema.json`](../../schemas/ui/m5-selection-lock-state-contract.schema.json)
- Support export: `artifacts/release/m5-selection-lock-state-contract-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-selection-lock-state-contract-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-selection-lock-state-contract-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-selection-lock-state-contract-primitive/`

The headless emitter `aureline_design_system_m5_selection_or_lock_state_contract` is the only
mint-from-truth path for the checked-in artifacts and fixtures.

```sh
cargo run -q -p aureline-design-system \
  --bin aureline_design_system_m5_selection_or_lock_state_contract -- support-export
```
