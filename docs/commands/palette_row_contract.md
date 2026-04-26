# Command palette row and action-footer contract

This document freezes the command-palette projection layer that sits
on top of the canonical command registry. The palette is a governed
view over command descriptors, discoverability records, keybinding
resolver output, enablement decisions, and shareability metadata; it is
not a second command registry, a hidden launcher, or a fuzzy-search
surface with private dispatch rules.

Machine-readable companions:

- [`/schemas/commands/palette_result.schema.json`](../../schemas/commands/palette_result.schema.json)
  defines query-session and result-row records.
- [`/schemas/commands/palette_action_footer.schema.json`](../../schemas/commands/palette_action_footer.schema.json)
  defines selected-row footer actions and alternate invocation rows.
- [`/fixtures/commands/palette_rows/`](../../fixtures/commands/palette_rows/)
  contains seed rows and footer variants projected from the existing
  command-registry seed records.

The command descriptor remains the canonical product object. Palette
records are materialized projections used by the shell, docs/help,
settings, keybinding help, support export, and automation explainers.
If the palette row disagrees with the descriptor, registry entry,
shareability record, or keybinding resolver, the palette row is wrong.

## Scope

Frozen here:

- palette anatomy, including query field state, query-session state,
  result grouping, result-list rows, inline explanation zones,
  preview/detail affordances, action footer, and keyboard-only
  interaction;
- result-row fields for user label, subtitle, stable command identity,
  lifecycle state, support class, origin and target badges,
  side-effect class, automation labels, current shortcut, disabled
  reason, and docs/help anchor;
- selected-row action footer and alternate invocation rows for split or
  alternate open, copy command identity, copy CLI/headless form, add to
  recipe, and inspect-why-not-automatable actions; and
- projection rules that keep palette rows, docs/help pages, settings,
  migration guidance, and why-unavailable explainers generated from
  one source record instead of handwritten per-surface copies.

Out of scope:

- ranking quality, fuzzy scorer internals, provider performance
  tuning, and final visual styling;
- the live command router and preview/apply runtime; and
- adding new command authority or capability classes. Those remain
  owned by the command descriptor, shareability, keybinding, and
  invocation-session contracts.

## Canonical source chain

Every command result row names the records that produced it:

| Source | Owns | Palette may project |
|---|---|---|
| `command_descriptor_record` | command ID, canonical verb, args, capability scope, preview/approval posture, lifecycle, result contract, docs/help anchor | identity, lifecycle/support chips, docs link, preview/detail posture |
| `command_registry_entry_record` | user label, summary, discoverability, aliases, badges, current keybinding refs, side-effect class, automation labels, disabled reason records | row label/subtitle, group, badges, shortcut display, disabled reason, automation chips |
| `shareability_metadata_record` | copy forms, deep-link posture, CLI/headless equivalent, automation-safety cues, why-unavailable parity | action footer, copy/CLI/add-to-recipe eligibility, why-not-automatable explanation |
| keybinding resolver output | winning chord, source layer, conflict/shadow/platform-reserved state | current shortcut cell and migration/conflict explanation zone |
| enablement decision | enabled/disabled/hidden state plus repair hook | disabled row state, inline reason chip, next safe action |

The palette must never author labels, command IDs, copy forms, CLI
forms, automation labels, disabled reasons, or help anchors locally. A
row may carry materialized human-readable text for rendering, but the
row must also carry the source refs that prove where that text came
from.

## Palette anatomy

| Element | Contract |
|---|---|
| Query field | Owns focus, input method composition, typed query ref, placeholder/refined-mode label refs, and the selected provider scope. Raw query text stays inside the query session and is retained only according to the session privacy block. |
| Query session | Tracks query state, provider states, selected row, held modifier intent, result groups, local history posture, and keyboard contract. The session may stream richer rows, but it must keep earlier rows navigable and truth-labeled. |
| Category/source grouping | Groups are explicit records, usually category, source, recent, migration, docs/help, or settings groups. A row must name the group that explains why it appears. |
| Result list | Renders `palette_result_row_record` objects. Every row resolves to a stable command ID plus command revision; non-command file/symbol/settings providers use their own schemas and may appear beside command rows, but command rows do not borrow their identities. |
| Inline explanation zones | Small row-adjacent zones for disabled reason, policy block, preview requirement, automation blocker, shortcut conflict, source/origin, migration alias, or history/privacy note. Zones cite structured refs, not local prose. |
| Preview/detail affordance | Shows command arguments, target summary, docs/help, preview/approval posture, or why-unavailable details. It reads the same descriptor and enablement decision that invocation would use. |
| Action footer | Shows selected-row actions and alternate invocation rows. Footer actions are projections of shareability metadata and command metadata, not local palette shortcuts. |
| Keyboard-only interaction | Arrow navigation, search, confirm, alternate invocation, detail/preview, footer action traversal, copy actions, and dismiss are reachable by keyboard and screen-reader narration. |

## Query session and privacy

Command palette query sessions are local-first. The session record
declares:

- query state: idle, composing, filtering, results streaming,
  provider degraded, empty, cancelled, or committed;
- provider states for command registry, keybinding resolver,
  docs/help, settings, recent history, and any active file/symbol
  provider;
- held modifier intent, so the footer and row narration can explain
  what `Enter`, `Alt+Enter`, or `Cmd/Ctrl+Enter` would do before the
  user commits;
- history posture: no history, local profile history, local per-device
  history, or managed history allowed by policy; and
- retention and export posture for query text, selected command refs,
  and recent invocation refs.

The palette may retain lightweight query history only when the session
declares the retention class. Managed or synced environments may retain
history only through a named records-governance posture. Support export
must redact raw query text unless the session explicitly allows export.

## Result-row contract

Each command result row carries these fields or source refs:

| Field | Source and rule |
|---|---|
| User label | Materialized from `command_registry_entry_record.title` / descriptor label refs. The palette may not rename it. |
| Subtitle | Materialized from registry summary, category/path, target summary, or contextual scope; must carry a `subtitle_source_class`. |
| Stable command identity | `command_id`, `command_revision_ref`, and `canonical_verb` copied from the descriptor. |
| Lifecycle state | Descriptor `lifecycle_state`, release channel, and declared freshness; no generic "available" badge. |
| Support class | Descriptor `support_class`, so docs/help, settings, and support export see the same support posture. |
| Origin badge | Registry origin badge: core, built-in extension, third-party extension, imported bridge, policy-provided, or labs. |
| Target badges | Registry target badges such as current workspace, focused selection, remote repository, docs reference, or managed admin surface. |
| Side-effect class | Registry dominant side-effect class. High-risk or externally visible classes must be visible inline or in one-step detail. |
| Automation labels | Registry automation labels plus shareability automation-safety cues where present. Silent omission is not allowed; unknown support is a real state. |
| Current shortcut | Current keybinding display state and source layer from resolver output. `Unassigned`, shadowed, platform-reserved, policy-blocked, and unsupported states must not collapse to blank. |
| Disabled reason | Enablement snapshot plus registry disabled-reason record and repair hook. Disabled or policy-blocked rows must explain the blocker and next safe route. |
| Docs/help anchor | Descriptor `docs_help_anchor_ref`, reused by docs/help, settings, command docs, CLI help, and footer copy actions. |
| Projection targets | Surfaces that may render the same row materialization: palette, docs/help, settings command detail, keybinding help, migration guidance, or support export. |

Rows with `hidden_with_reason` may be absent from ordinary palette
results, but protected discovery flows, support export, migration
guidance, and accessibility routes must still be able to render their
reason and replacement guidance.

## Disabled and policy-blocked rows

Disabled rows remain selectable when discoverability matters. A row
that cannot run must still answer:

- which command is blocked;
- which boundary owns the block: trust, policy, context, dependency,
  lifecycle, client scope, platform, or migration bridge;
- which structured reason code applies;
- which repair hook or next safe route exists;
- whether a fallback command exists; and
- whether copy, docs/help, settings, or why-unavailable inspection
  remains allowed.

Policy-blocked rows may be narrowed or hidden from novice-oriented
contexts, but they must not disappear from support, migration,
settings, or admin explainability surfaces. There is no support-only
debug mode for this information; the governed row carries it.

## Action footer and alternate invocation rows

The footer appears for the selected row. It is a compact command-detail
surface, not a private command runner. Each footer action declares:

- action class;
- label and narration refs;
- keyboard gesture;
- enabled/disabled state;
- source copy form, CLI/headless block, invocation ref, or diagnostic
  ref where applicable;
- whether trust, policy, preview, approval, and permission gates must
  be revalidated; and
- every surface where the same action semantics must stay consistent.

Required action semantics:

| Action | Rule |
|---|---|
| Primary run/open | Uses the same enablement, preview, approval, trust, policy, and audit path as any other issuing surface. |
| Split/open alternate | Changes placement or target only. It never widens command authority or skips preview/approval. |
| Copy command ID | Copies the canonical stable command ID, not a palette-local row ID. |
| Copy CLI/headless form | Copies a documented skeleton or invocation ref from shareability metadata. It is not pre-approved execution. |
| Add to recipe | Inserts a typed recipe step preserving command ID, command revision, and argument structure. It does not run the live command. |
| Inspect why not automatable | Opens a structured explanation derived from automation labels, shareability cues, disabled reasons, and diagnostic projection refs. |

Alternate invocation rows describe placement or mode variants such as
current surface, split pane, new tab/window, external browser handoff,
headless skeleton, or recipe insertion. They may carry argument delta
refs, but they must declare that all gates are revalidated at dispatch.

## Projection rules

1. Start with one `command_registry_entry_record`.
2. Copy descriptor identity and docs/help anchor into the palette row.
3. Copy discoverability category, aliases, badges, automation labels,
   side-effect class, and disabled-reason records from the registry
   entry.
4. Overlay runtime enablement and keybinding resolver snapshots
   without rewriting the canonical metadata.
5. Overlay shareability metadata for copy, deep-link, CLI/headless,
   invocation-ref, recipe, and why-unavailable actions.
6. Emit one projection record that may be rendered in palette,
   docs/help, settings, keybinding help, migration guidance, and
   support export.

Docs/help and settings pages may choose a different layout, but they
must render the same command ID, lifecycle/support state, shortcut
state, automation posture, disabled reason, and action semantics as the
palette.

## Keyboard and accessibility contract

The palette remains keyboard-complete:

- focus enters the query field immediately on open;
- arrow keys move through result rows without changing row identity;
- `Enter` performs the primary action only if enabled or opens the
  required preview/review path;
- alternate gestures such as `Alt+Enter` and `Cmd/Ctrl+Enter` must be
  described by the selected row or footer before invocation;
- `Tab` / `Shift+Tab` traverse preview/detail and footer actions;
- copy actions have keyboard routes and narration;
- disabled rows announce the disabled reason and repair route; and
- closing the palette returns focus to the invoking surface or the
  nearest safe ancestor when that surface disappeared.

Screen-reader labels, shortcut narration, and "why unavailable" text
come from descriptor/accessibility refs and disabled-reason refs. A row
that needs local copy because a localized label has not loaded must
carry a degraded localization state rather than inventing a final label.

## Fixture expectations

Fixture rows under
[`/fixtures/commands/palette_rows/`](../../fixtures/commands/palette_rows/)
exercise:

- an enabled command row with current shortcut and copyable identity;
- a preview-required mutating command row with approval and side-effect
  cues;
- a policy-disabled or labs-disabled row with repair guidance; and
- footer variants for copy ID, copy CLI/headless form, split/open-alt,
  add to recipe, and inspect why not automatable.

The fixture corpus is intentionally small. It proves the shape and the
source-record projection rule; it does not assert final ranking,
scoring, or visual density.
