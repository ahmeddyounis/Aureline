# Button, Icon Button, and Split Button Contract

This contract freezes how Aureline expresses action controls so users can
rely on one consistent hierarchy across toolbars, dialogs, review
surfaces, and dense inline affordances. It exists to prevent
surface-local "primary vs destructive vs blocked" interpretations from
encoding contradictory behavior before implementation hardens.

The contract is normative. Where this document disagrees with the UI/UX
spec, design-system style guide, interaction-safety contract, or other
upstream sources it cites, the upstream source wins and this document
plus its schema and fixtures update in the same change. Where this
document disagrees with a downstream surface’s private button rules,
this document wins and the surface is non-conforming.

Companion artifacts:

- [`/schemas/ux/button_action.schema.json`](../../schemas/ux/button_action.schema.json)
  — boundary schema for button action records and action-group ordering
  fixtures.
- [`/fixtures/ux/button_cases/`](../../fixtures/ux/button_cases/)
  — worked fixtures for toolbar icon groups, dialog action rows, split
  buttons, policy-locked buttons, and pending/progress buttons.

This contract composes with, and does not replace:

- [`/docs/ux/control_family_contract.md`](./control_family_contract.md)
  for the shared meanings of `disabled`, `locked`, `pending`, and
  `preview_required` / `review_required` actionability.
- [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  for why-unavailable copy, alternate-route obligations, and
  translation-safe blocked-state phrasing.
- [`/docs/ux/prompt_grammar_contract.md`](./prompt_grammar_contract.md)
  for consequential prompt title/body/button label grammar and the
  prohibition on generic acknowledgements.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for consequence classes, safe default focus, required visible fields,
  and denial behavior on high-risk commits.
- [`/docs/ux/menu_command_bar_contract.md`](./menu_command_bar_contract.md)
  for command-graph parity between buttons, menus, palettes, and other
  invocation surfaces.

Normative source sections projected here include the button sections in
`.t2/docs/Aureline_UI_UX_Spec_Document.md` and
`.t2/docs/Aureline_UX_Design_System_Style_Guide.md` plus the destructive
action and preview/apply/revert sections referenced above.

## Scope

This contract applies to every first-party or must-match action control
that:

- triggers a command;
- changes state in the local workspace or any external/shared boundary;
- starts a job, submit, install/update, publish, revoke, or other
  consequence-bearing flow; or
- blocks, locks, defers, or forces preview/review before committing.

Purely decorative icons, navigation links, and row-selection affordances
are out of scope. (If an affordance performs a command, it is in scope
regardless of chrome.)

## Definitions

### Local region

A **local region** is the smallest UI area where the user decides what
to do next without scrolling or context switching. Examples:

- a dialog action row;
- a sheet footer action row;
- a toolbar group within a panel header;
- the primary-action strip on a card or row; or
- the final “apply / cancel” area of a staged form.

The region boundary matters because “primary” is a local meaning. A
surface can have multiple primary actions only when they live in
different regions with different scopes.

### Action control families and variants

Aureline uses a closed action control set:

- **Button** — text (optionally with an icon) that triggers one action.
- **Icon button** (ghost icon) — compact icon-only action for dense
  surfaces; always tooltiped and keyboard-equivalent.
- **Split button** — one default action plus alternate variants exposed
  via disclosure.

Button **variants** convey action hierarchy:

- `primary`
- `secondary`
- `tertiary/quiet`
- `destructive`

Icon buttons are treated as `ghost icon` hierarchy: they are compact
chrome, not a way to hide consequence class.

## Hierarchy and “one primary per local region”

### Primary

The **primary** action is the dominant action in a local region. Rules:

1. A local region MUST contain at most one primary action.
2. A region with a primary action MUST render all other non-destructive
   actions as secondary or tertiary/quiet.
3. A primary action MUST remain comprehensible without relying on nearby
   secondary actions (no “Cancel / OK” pairs; the primary names the
   outcome).

If a surface needs two “equally primary” actions, it must split the UI
into two regions with explicit scope boundaries (or redesign the flow so
one action is the default and the other becomes an alternate route).

### Secondary

**Secondary** actions are common but non-dominant actions. They do not
compete with the primary. Secondary actions may appear without a primary
in dense surfaces, but the local region must still have one clear
dominant route when a decision is high-stakes.

### Tertiary / quiet

**Tertiary/quiet** actions are low-chrome actions used to avoid visual
noise in dense surfaces. Quiet actions must still be discoverable and
keyboard reachable; quiet is not “hidden”.

Quiet actions MUST NOT be used for destructive outcomes and MUST NOT be
the only route to a consequence-bearing action when the surface also
claims a “review-first” posture.

### Destructive

**Destructive** is reserved for actions whose outcome is destructive.
Destructive styling is not a general “danger” accent and must not be
used for non-destructive warnings.

Destructive actions MUST:

- use explicit verb + target labels (never generic confirmation);
- name the resulting state and recovery class in nearby copy when stakes
  are high; and
- preserve the preview/review posture mandated by interaction safety and
  destructive action rules.

## Labels: verb-first and outcome truth

Button labels are part of Aureline’s command language. Rules:

1. Labels SHOULD be verb-first where practical.
2. Labels MUST name the outcome, not acknowledge a prompt.
3. Labels MUST stay specific on destructive, publish-capable,
   permissioned, policy-bearing, or external/shared actions.

Non-conforming label examples:

- `OK`, `Yes`, `No`, `Continue`, `Apply`, `Confirm`, `Submit`

Conforming examples:

- `Delete branch`, `Revoke token`, `Publish extension to {target}`
- `Open preview`, `Review changes`, `Export impact summary`
- `Keep in Preview`, `Continue read-only`, `Stay in restricted mode`

`Cancel` is allowed only when it means “no mutation occurs”. If closing
preserves or changes state (e.g., “keep draft”, “leave grant active”),
the label must name that outcome.

## Actionability: disabled vs locked vs pending vs preview-first

Action controls must not collapse distinct “not now” states into the
same greyed-out button.

### Disabled

`disabled` means **no valid action exists in the current context**.
Disabled controls may explain the current cause, but they do not cite
policy/trust/permission ownership as the blocker.

### Locked

`locked` means a policy, managed authority, trust, permission,
capability, ownership, or protected-source rule blocks the action.
Locked controls MUST expose:

- a short visible reason (not hover-only); and
- an inspect route when one exists (open policy detail, open trust
  prompt, open capability explanation).

### Pending

`pending` means the user submitted the action and the action is in
flight, queued, or awaiting validation/commit. Pending controls MUST:

- preserve hit target and label width (no layout jitter); and
- keep the submitted action’s meaning visible (spinner/progress augments
  the action; it does not replace it with an unrelated label).

### Preview-first / review-required

A control that requires preview or review MUST say so in its
actionability and must not masquerade as an “apply now” button. If the
first activation opens preview/review, that behavior is part of the
control contract and must be reflected in:

- the label (`Open preview`, `Review changes`, `Delete… (review first)`
  when needed);
- the blocked/why-unavailable explanation when apply is not allowed; and
- the command routing metadata (palette/menu parity and support export).

## Dialog action rows: ordering and default focus

Dialogs and review prompts must not force users to rely on muscle memory
for destructive choices.

Rules:

1. Order actions by outcome clarity and consequence class, not by
   historical UI habit.
2. The safest sensible action MUST be the default focus when the
   consequence class is `external_shared` or `irreversible_high_blast`,
   and the destructive action MUST NOT sit on the default Enter path
   unless the user is already inside an explicit destructive review.
3. Destructive actions MUST be visually and spatially separated from the
   safe action when both are present.
4. If a destructive button appears, nearby copy MUST name the affected
   object or scope before confirm.

This contract does not dictate left-to-right vs right-to-left placement.
It dictates the **semantic ordering**: safe/non-mutating actions are
grouped away from destructive commit actions, and the default focus is
safe when stakes are high.

## Icon buttons (ghost icon): tooltip and keyboard parity

Icon-only controls are allowed only when they remain unambiguous without
tribal knowledge.

Rules:

1. Every icon button MUST have a tooltip with the command label.
2. Every icon button MUST have a keyboard equivalent (command palette
   route or shortcut) and must resolve to the same command identity as
   any menu or palette entry.
3. The minimum hit target is 28 px even when the icon is visually
   smaller.
4. Icon-only buttons SHOULD be limited to common, low-risk actions in
   dense surfaces. Uncommon or destructive actions must use a labeled
   button or a routed review surface.
5. An unlabeled icon-only destructive action is non-conforming.

## Split buttons: default route, alternates, and auditability

Split buttons exist to express “one default action” plus alternates.
They must not be used to hide uncertainty or to smuggle riskier
alternates behind a default-looking affordance.

Rules:

1. A split button MUST declare one default action. The default action’s
   label and posture are always visible on the primary face.
2. The alternate routes MUST be discoverable via explicit disclosure.
3. If the default action is blocked (locked, disabled, preview-required)
   the control MUST preserve that posture on the primary face; alternates
   do not silently become the default.
4. If alternate branches differ materially in side effects, the choice
   MUST be auditably recorded as “which branch was chosen” in the same
   durable lineage used for jobs/history/mutation journals.

Auditability does not require user-visible telemetry words. It requires
that internal records and support exports can distinguish the default
branch from alternates when they lead to different outcomes.

## Conformance checklist

A surface conforms to this contract when a reviewer can verify:

1. Every local region has at most one primary action.
2. Destructive styling is used only for destructive outcomes.
3. Verb-first, outcome-specific labels are used; generic confirmations
   are absent on consequential actions.
4. Disabled, locked, pending, and preview-first/review-required actions
   are distinguishable without hover-only explanation.
5. Dialog action rows default focus to the safest action under
   high-consequence classes; destructive confirm is not the default Enter
   path in those cases.
6. Icon-only controls have tooltip + keyboard parity + 28 px hit target.
7. Split buttons keep default vs alternate routing explicit and record
   which branch was chosen when side effects differ materially.

