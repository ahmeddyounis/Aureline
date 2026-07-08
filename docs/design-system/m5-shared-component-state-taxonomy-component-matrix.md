# M5 shared-component-state-taxonomy component matrix

The **shared-component-state matrix** freezes the canonical object model Aureline
uses for reusable component states across launch-critical M5 controls, dense
collections, prompts, and recovery surfaces. Where the
[component manifest](m5-component-manifest.md) carries the durable *component
contracts* and the [foundation package](m5-foundation-package.md) ships the
*tokens, density, motion, contrast, and state vocabulary*, this matrix names the
one shared **state taxonomy** those surfaces map their local state machines back
to — so component states stop drifting between controls, collections, prompts,
and recovery flows.

- Schema: [`schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json`](../../schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json)
- Support export: [`artifacts/release/m5-shared-state-taxonomy-proof/support_export.json`](../../artifacts/release/m5-shared-state-taxonomy-proof/support_export.json)
- Matrix CSV: [`artifacts/release/m5-shared-state-taxonomy-proof/matrix.csv`](../../artifacts/release/m5-shared-state-taxonomy-proof/matrix.csv)
- Design report: [`artifacts/design/m5-shared-state-taxonomy-component-matrix.md`](../../artifacts/design/m5-shared-state-taxonomy-component-matrix.md)
- Narrowed fixtures: `fixtures/ui/m5-shared-state-taxonomy/*.json`
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_shared_component_state_matrix`
- Validator: `crates/aureline-design-system` (`freeze_the_m5_shared_component_state_taxonomy_…` module) is the authoritative gate.

## The canonical state family

The matrix freezes thirteen canonical component-state classes. Every M5 surface
maps its local state machine back to these instead of minting a private state
name:

`default`, `hover`, `focus_visible`, `pressed_active`, `selected`, `current`,
`disabled`, `read_only`, `loading`, `pending`, `warning_error`, `locked`,
`degraded`.

## Contract families

The taxonomy is published as four governed contract families. The first names the
full family plus the rules; the other three partition the states between them:

| Contract family | Governs | Lifecycle |
| --------------- | ------- | --------- |
| `shared_component_state_taxonomy` | all thirteen states, precedence rules, and disclosure triggers | stable |
| `interactive_state` | `default`, `hover`, `focus_visible`, `pressed_active` | stable |
| `selection_or_lock_state` | `selected`, `current`, `disabled`, `read_only`, `locked` | stable |
| `degraded_state_application` | `loading`, `pending`, `warning_error`, `degraded` | stable |

## Precedence and distinctness rules

The shared taxonomy freezes the comparison rules that keep the states
semantically distinct instead of collapsing them into color-only treatments:

- `locked_over_disabled` — a lock takes precedence over a plain disabled treatment
  so the lock stays explainable.
- `read_only_over_disabled` — a read-only posture takes precedence over disabled
  so inspectability is preserved.
- `current_distinct_from_selected` — `current` and `selected` never collapse.
- `pending_distinct_from_loading` — a submitted action never reads as generic
  background work.

## When a state must be published

A state may not apply a silent, style-only change when a **disclosure trigger**
requires more. The taxonomy publishes: `state_cause_required`, `owner_required`,
`block_reason_required`, `recovery_action_required`, and
`silent_style_only_forbidden`. The selection-or-lock and degraded contracts carry
the shared **state-cause** vocabulary, the selection-or-lock contract discloses
the **lock owner** (policy, trust, permission, ownership, source, or none), and
the degraded contract names the **recovery disclosure** (consequence, recovery
action, freshness, retry path, fallback scope, or that no recovery is available).

## Hard invariants

Every row asserts — and the validator enforces — that the contract never:

- collapses `current` and `selected`,
- masks an explainable lock behind a plain disabled treatment,
- presents `pending` as generic `loading`, or
- omits consequence or recovery on a degraded, warning, or error state.

The matrix also keeps every state keyboard-visible, screen-reader explainable,
encoded by more than color alone, and present in the support export across every
deployment line.
