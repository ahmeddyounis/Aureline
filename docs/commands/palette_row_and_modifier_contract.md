# Command-palette row, modifier-action, and automation-cue contract

This document freezes the combined command-palette row contract that
desktop palette rows, docs examples, CLI discoverability aids,
automation explainers, and support captures may all point at. The
palette row is a materialized projection over the command descriptor,
command registry entry, keybinding resolver output, diagnostic
projection, enablement decision, and shareability metadata. It is not a
new command registry and it does not own command names, disabled prose,
automation posture, or alternate execution authority.

Machine-readable companions:

- [`/schemas/commands/palette_row.schema.json`](../../schemas/commands/palette_row.schema.json)
  defines the combined row, modifier-action, automation-cue,
  explanation-link, and degraded-state record.
- [`/fixtures/commands/palette_row_cases/`](../../fixtures/commands/palette_row_cases/)
  contains seed rows for enabled, preview-gated, provider-backed
  UI-only, and hidden/deprecated cases.
- [`/schemas/commands/palette_result.schema.json`](../../schemas/commands/palette_result.schema.json)
  and
  [`/schemas/commands/palette_action_footer.schema.json`](../../schemas/commands/palette_action_footer.schema.json)
  remain detailed split projections for query-session and selected-row
  footer materializations. They must not disagree with this combined
  row contract.

The command descriptor remains the canonical product object. If a row,
footer action, CLI aid, support packet, or docs example disagrees with
the descriptor, registry entry, resolver, diagnostics, or shareability
metadata, the projection is wrong.

## Scope

Frozen here:

- required command-row elements: primary label, secondary scope detail,
  origin badge, winning shortcut hint, reason chip, automation labels,
  lifecycle cue, and target or authority hint;
- modifier and footer actions for primary invoke, alternate placement,
  alternate target, copying the canonical command ID, copying a CLI
  form, adding to a recipe, and inspecting why a command is not
  automatable;
- no-bypass rules that preserve trust, policy, permission, preview,
  approval, capability, and target-authority boundaries for every
  modifier action;
- cross-links from command IDs, disabled reason codes, docs/help
  anchors, automation posture, shareability metadata, CLI registries,
  and support captures; and
- fallback and degraded states for hidden, deprecated,
  provider-backed, UI-only, policy-blocked, or not-automatable rows.

Out of scope:

- the live palette UI, ranking, fuzzy scoring, animation, and visual
  styling;
- the command router, preview/apply runtime, and approval-ticket body;
  and
- adding new command authority, capability, disabled-reason, or
  automation vocabularies. Those remain owned by the upstream command,
  diagnostics, keybinding, and shareability contracts.

## Source chain

Every row names the records that produced it:

| Source | Owns | Row projects |
| --- | --- | --- |
| `command_descriptor_record` | stable command ID, canonical verb, typed arguments, capability scope, preview/approval posture, lifecycle, support class, docs/help anchor | identity, lifecycle cue, preview/approval cue, target or authority hint |
| `command_registry_entry_record` | title, summary, discoverability, aliases, badges, current shortcut refs, automation labels, dominant side effect | primary label, secondary detail, origin badge, target badges, automation labels |
| keybinding resolver output | winning chord, source layer, shadow/conflict/platform state | winning shortcut hint and shortcut reason link |
| enablement/diagnostic projection | enabled/disabled/hidden state, disabled reason code, owner boundary, repair hook, protected target badge | reason chip and next safe route |
| `shareability_metadata_record` | copy forms, CLI/headless posture, invocation ref, recipe/automation safety cues, why-unavailable parity | copy actions, CLI form, add-to-recipe action, why-not-automatable action |

Rows may carry materialized text for rendering, but every materialized
label or chip also carries a source ref. The palette may not rename a
command, invent a local shortcut label, collapse an unknown automation
state to blank, or convert a disabled reason into local prose.

## Required Row Elements

Every command row carries these elements, even when a compact surface
renders only a subset:

| Element | Rule |
| --- | --- |
| Primary label | Comes from the registry/descriptor label ref. It is the user-facing action label, not a palette-local alias. |
| Secondary scope detail | Explains category, path, current target, provider, or active scope. It must name its source class so support and docs can reproduce the same detail. |
| Origin badge | Explicitly says core, built-in extension, third-party extension, imported bridge, policy-provided, or labs. Rows must not pretend extension or bridge actions are first-party. |
| Winning shortcut hint | Shows resolver output for the active platform and scope: assigned, unassigned, shadowed, conflict, platform-reserved, policy-blocked, or unsupported. Blank shortcut cells are non-conforming. |
| Reason chip | Uses a controlled reason class: disabled, policy blocked, preview required, approval required, deprecated, provider degraded, UI-only, hidden with reason, not automatable, or none. |
| Automation labels | Re-export registry automation labels and shareability safety cues. Unknown automation support is a first-class cue; silent omission is not allowed. |
| Lifecycle cue | Re-exports lifecycle, support class, release channel, freshness, and replacement command where applicable. No generic "available" badge. |
| Target or authority hint | Names the affected target, authority boundary, or capability scope so users can distinguish local, remote, managed, policy, credential, extension, and bridge actions before invoking. |

The row-level schema requires all eight elements. A UI-only command or
a hidden-with-reason command still carries the full element set when it
appears in protected discovery, support capture, docs, migration, or
CLI discoverability output.

## Modifier And Footer Actions

The selected row exposes a compact set of modifier/footer actions.
Each action carries an action class, label and narration refs,
keyboard gesture, semantics class, source copy/invocation refs,
unavailable reason, and a no-bypass guard.

| Action | Semantics |
| --- | --- |
| Primary invoke | Dispatches the command through the same command router, enablement decision, preview path, approval path, audit path, and capability class as every other issuing surface. |
| Alternate placement | Changes placement only, such as split pane, new tab, or new window. It never changes the command's authority or target class. |
| Alternate target | Changes the declared target only when the command descriptor and current enablement decision allow that target. It never widens capability class or skips approval. |
| Copy canonical command ID | Copies the stable `command_id`, not a row ID, search token, label, or alias. Copy does not dispatch. |
| Copy CLI form | Copies a documented CLI/headless skeleton or invocation ref from shareability metadata. It is not a pre-approved execution token. |
| Add to recipe | Inserts a typed recipe step preserving command ID, command revision, canonical verb, argument structure, and automation cues. It does not run the command. |
| Inspect why not automatable | Opens structured explanation from automation labels, shareability cues, disabled reasons, diagnostic projection refs, and CLI/headless posture. It does not dispatch. |

Modifier gestures such as `Enter`, `Alt+Enter`, and
`Cmd/Ctrl+Enter` are explanatory shortcuts over these same action
classes. Holding a modifier may preview the alternate action, but the
commit path remains the same governed action.

## No-Bypass Rules

Every modifier action, including copy and inspect actions, carries a
guard that pins these values to true:

- trust revalidation required;
- policy revalidation required;
- permission-prompt revalidation required;
- preview path preserved;
- approval path preserved;
- capability class preserved;
- target delta may not widen authority;
- automation insertion does not execute; and
- copy forms do not execute.

Consequences:

- A preview-required command invoked with `Alt+Enter` opens the same
  preview route in the alternate placement; it does not apply directly.
- An approval-gated command copied as CLI still requires approval when
  pasted into a headless or CI context.
- A recipe insertion captures a portable command step, not yesterday's
  live approval or current target object.
- An alternate target row may narrow or select among declared targets,
  but it may not create a new execution authority, remote route,
  credential scope, or automation path.

## Cross-Links And Alias Discipline

Each row's `explanation_links` block binds:

- `command_id`, `command_revision_ref`, and `canonical_verb`;
- descriptor docs/help anchor;
- disabled reason code and disabled-reason explainer ref when present;
- automation posture ref and shareability metadata ref;
- CLI registry ref when a CLI discoverability row exists;
- support capture ref; and
- alias-resolution ref plus an alias-drift guard.

The same block is what lets support staff, docs examples, CLI `--help`
rows, command docs, and automation explainers point at the same command
truth. Surface-specific aliases may help search, but they cannot become
new identities. A deprecated or imported alias must point back to the
replacement command ID or alias-resolution record.

## Fallback And Degraded States

Rows that cannot currently run remain honest:

| State | Required behavior |
| --- | --- |
| Hidden with reason | May be absent from ordinary palette search, but protected discovery, support capture, docs, migration, and CLI explainers can still render the row and reason. |
| Deprecated with successor | Shows lifecycle cue, replacement command ID, alias-resolution ref, and migration/docs route. New bindings target the successor. |
| Provider-backed degraded | Shows provider state and degraded reason. Copy/docs/support may remain available; dispatch revalidates provider capability before running. |
| UI-only explainable | Shows UI-only automation cue, no CLI equivalent or denied headless mode, and a why-not-automatable route. The row must not imply recipe/headless support. |
| Not automatable | Keeps command discovery intact while making macro/recipe/headless/AI blockers explicit. Add-to-recipe is disabled or hidden with reason. |
| Policy or trust blocked | Shows owner boundary, disabled reason code, repair hook or safe route, and copy/docs availability where policy permits. |

Fallback rows do not reduce the truth bar. A hidden or provider-backed
row still carries origin, shortcut state, automation posture, lifecycle,
and target/authority hints when rendered in any explainability surface.

## Projection Rules

1. Start with one command descriptor and matching registry entry.
2. Copy identity, lifecycle, preview/approval posture, support class,
   capability class, and docs/help anchor from the descriptor.
3. Copy label, scope detail, badges, shortcut refs, side-effect class,
   and automation labels from the registry entry and resolver output.
4. Overlay enablement and diagnostics without rewriting the canonical
   metadata.
5. Overlay shareability metadata for copy, CLI/headless, invocation,
   recipe, and why-unavailable actions.
6. Emit one row record that desktop palette, docs examples, CLI
   discoverability, support capture, keybinding help, menus, leader
   overlays, and automation builders may render with different layout
   but the same command identity and safety semantics.

Any surface that needs a field outside this contract should extend the
upstream descriptor, registry, diagnostics, keybinding, or shareability
record first, then project it into the row.
