# Sequence and modal discoverability contract

This document freezes how Aureline makes modal editing, leader
sequences, operator-pending commands, register state, macro state, and
colon-style command entry teachable without creating a second command
system.

Machine-readable companions:

- [`/schemas/commands/leader_overlay.schema.json`](../../schemas/commands/leader_overlay.schema.json)
  defines modal-state cue records, sequence-help rows, leader-overlay
  rows, shortcut-teaching rows, and command-language parity rows.
- [`/fixtures/commands/sequence_help_examples/`](../../fixtures/commands/sequence_help_examples/)
  contains worked rows for partial leader help, operator-pending
  help, imported-keymap conflict teaching, and colon-command parity.

The command descriptor remains the canonical product object. A modal
sequence, leader row, command-palette result, docs/help page, settings
row, migration card, automation recipe step, or colon-style command
entry may render different UI, but they all resolve back to the same
stable command id, command revision, enablement decision, preview
posture, approval posture, and docs/help anchor.

If this document disagrees with the command-descriptor contract,
keybinding resolver contract, or command-palette row contract, the
command descriptor wins and this document must be updated in the same
change.

## Scope

Frozen here:

- visible modal-state cues for current mode, pending operator, count,
  macro recording or replay, register boundary, and active leader or
  sequence prefix;
- sequence-help and leader-overlay row anatomy, including typed keys so
  far, available next keys, command labels and ids, current mode,
  timeout or cancel hint, and pivot paths;
- shortcut-teaching rules for current shortcut display, imported keymap
  differences, conflict review, active winner disclosure, and
  high-frequency action teaching; and
- parity rules proving palette, leader overlays, modal sequences, and
  colon-style command entry are views into one command graph.

Out of scope:

- implementing modal editing, keymap dispatch, command-line parsing, or
  runtime sequence timing;
- choosing final visual styling for overlays, strips, or pickers; and
- shipping a complete Vim, Neovim, Emacs, Helix, or product-native
  preset.

## Canonical source chain

| Source | Owns | Modal/sequence surfaces may project |
|---|---|---|
| `command_descriptor_record` | stable command id, canonical verb, typed args, capability scope, preview, approval, lifecycle, docs/help anchor | command identity, authority posture, disabled reason, docs/help pivot |
| `command_registry_entry_record` | label, aliases, discoverability, badges, current shortcut refs, machine-facing names | command label, alias route, group label, shortcut teaching |
| `keybinding_resolution_packet_record` | inspected sequence, active mode/scope, precedence trace, winner, losers, waiting state | typed prefix truth, active winner, conflicting bindings, next-safe actions |
| `leader_overlay_row_record` | overlay row materialization for a current prefix | available next keys, waiting/timeout state, pivots |
| migration/import bridge records | imported source, fidelity class, changed behavior axes, rollback refs | reviewable imported-keymap differences |

Rules:

1. Modal and sequence surfaces must not mint local command ids, local
   labels, local enablement rules, or local preview/approval posture.
2. A sequence help row that points to a command must carry the stable
   command id and command revision used by palette, docs, automation,
   and support export.
3. A sequence that cannot resolve must emit a typed state such as
   `partial_waiting`, `ambiguous_waiting`, `conflict_requires_review`,
   `blocked_by_host`, `blocked_by_policy`, or `unsupported_surface`.
   Silent failure is non-conforming.

## Modal state cues

Whenever the current surface changes key meaning, Aureline must expose a
keyboard-reachable modal-state cue. The cue may render as a mode strip,
status-bar segment, inline overlay, screen-reader announcement, or
settings/help row, but it must carry the same structured state.

Required cue families:

| Cue | Contract |
|---|---|
| Current mode | Name the canonical visible mode such as `normal`, `insert`, `visual`, `replace`, `select`, `command`, `terminal_passthrough`, `modeless`, or `unsupported`. Include the source profile when it materially changes meaning. |
| Pending operator | If an operator is waiting for a motion or text object, show the operator, count if present, what is being awaited, and the resulting command id when known. |
| Count | If a numeric prefix changes scope, show the count and what it applies to until the sequence resolves, cancels, times out, or blocks. |
| Macro recording or replay | Show recording/replay state, register, replay scope, and whether review is required. Hidden macro state is not allowed. |
| Register boundary | Distinguish unnamed, named, system clipboard, remote clipboard bridge, search, macro, blackhole, and policy-blocked routes. Redact previews by default. |
| Leader or sequence prefix | Show typed keys so far, sequence state, timeout/cancel hint, and available next keys or recovery pivots. |

Mode changes must never rely on color alone. Every mode cue must include
at least one non-color cue such as text, icon shape, screen-reader
announcement, status-bar label, mode-strip label, or focus-region label.
Color tokens may reinforce state, but they cannot be the only state
carrier.

## Sequence Help Rows

A `sequence_help_row_record` answers "what is Aureline waiting for and
what can I press next?" for leader sequences, multi-stroke chords,
operator-pending sequences, and colon-style command entry.

Each row must carry:

- the literal sequence and typed key list so far;
- the sequence shape and resolution state;
- the current modal-state snapshot;
- available next keys, grouped for scanability;
- resulting command label, stable command id, command revision,
  canonical verb, preview class, approval posture, and capability class
  for any next key that resolves to a command;
- disabled, conflict, unsupported, host-blocked, or policy-blocked
  states with typed refs instead of generic failure text;
- timeout and cancel behavior; and
- pivot paths to palette, keybinding settings, command docs, migration
  guidance, conflict review, supported surfaces, or mode reset.

Partial and ambiguous sequences must stay inspectable. For example,
typing a prefix that can still become more than one command should open
sequence help in `partial_waiting` or `ambiguous_waiting` state and show
the valid continuations. A timeout may cancel, resolve an exact prefix,
keep help visible until explicit cancel, or open conflict review, but it
may not disappear without an explainable outcome.

Sequence help must be keyboard-complete. Users must be able to open it,
move through next-key rows, copy command ids, open command docs, inspect
conflicts, and dismiss it without pointer hover or external
documentation.

## Leader Overlays

A `leader_overlay_row_record` is a sequence-help projection optimized
for which-key style overlays. It is generated from the command registry
and keybinding resolver, not from a hand-maintained overlay dictionary.

Each leader row must show:

- the leader group label and typed prefix;
- current mode and surface support;
- available next keys and their availability state;
- source class (`core`, `imported_bridge`, `third_party_extension`,
  `workspace_override`, `policy_provided`, or equivalent);
- the active resolver layer;
- command label and id for command-bearing next keys;
- timeout or cancel hint; and
- pivots to palette, settings, docs, conflict review, migration
  guidance, supported-surface retry, or mode reset.

Leader overlays must not steal editor focus or block the next key. They
are assistance surfaces layered over the active input path.

## Shortcut Teaching

Frequent actions must teach the current shortcut where the action is
shown. For command rows, menus, leader overlays, sequence help, docs,
settings, and migration guidance, a high-frequency or daily-driver
action must not collapse the shortcut cell to blank. It must say one of:

- assigned sequence and source layer;
- unassigned;
- conflict requires review;
- platform reserved;
- shadowed;
- policy blocked; or
- unsupported on the current surface.

Imported keymap differences must remain reviewable after first run. A
translated, partial, shimmed, alias-only, or unsupported import may not
be displayed as exact. Rows must link to the import bridge or shortcut
diff record and preserve rollback or review paths when available.

Conflicting bindings must expose:

- both actions;
- both command ids and labels;
- both scopes or surfaces;
- both source classes and resolver layers;
- the active winner; and
- at least one path that would change or review the outcome.

This rule applies equally to leader overlays, sequence help, palette
shortcut cells, settings, migration review, docs/help, and support
export.

## Command-Language Parity

Palette, leader overlay, modal sequence, colon-style command entry,
docs/help, settings, migration guidance, automation recipes, and CLI
help are projections over one command graph.

Parity rules:

1. Every exposed leader sequence, key chord, operator command, and
   colon-style command entry maps to one canonical command id or to a
   typed unsupported/unbound state.
2. Palette search should be able to find command-bearing sequences by
   human label, command id, canonical verb, and literal sequence where
   practical.
3. Colon-style command aliases must be declared aliases or parity
   projections of a command descriptor. They may not introduce private
   command names with separate preview or approval rules.
4. A command that requires preview, approval, trust elevation, audit, or
   evidence from the palette requires the same posture from a leader
   sequence, modal sequence, colon-style command, recipe, or CLI route.
5. Unsupported surfaces narrow honestly. Terminals, rendered docs,
   webviews, property grids, browser companions, large-file degraded
   surfaces, or restricted inputs may opt out or reduce fidelity, but
   must expose `unsupported_surface`, `blocked_by_host`, or the
   equivalent typed reason before key meaning changes.

## Fixture Expectations

Fixtures under
[`/fixtures/commands/sequence_help_examples/`](../../fixtures/commands/sequence_help_examples/)
exercise:

- partial leader prefix with waiting state, next-key guidance, visible
  macro/register cues, and pivots;
- operator-pending sequence with count and destructive-scope guidance;
- shortcut teaching after an imported keymap creates a conflict; and
- colon-style command entry proving palette, leader sequence, docs, and
  command entry share the same command id.

Each fixture validates against
[`/schemas/commands/leader_overlay.schema.json`](../../schemas/commands/leader_overlay.schema.json)
after stripping fixture-only `$schema` and `__fixture__` keys.

## Related Contracts

- [`/docs/commands/command_descriptor_contract.md`](./command_descriptor_contract.md)
  owns command semantics.
- [`/docs/commands/palette_row_and_modifier_contract.md`](./palette_row_and_modifier_contract.md)
  owns the combined palette row, modifier-action, automation-cue, and
  degraded-state projection.
- [`/docs/commands/palette_row_contract.md`](./palette_row_contract.md)
  owns detailed query-session/result-row and action-footer projections
  that must stay aligned with the combined row contract.
- [`/docs/ux/keybinding_resolver_contract.md`](../ux/keybinding_resolver_contract.md)
  owns precedence, conflict-review, disabled-command, and import-bridge
  resolver packets.
- [`/docs/commands/command_graph_and_ui_slots_seed.md`](./command_graph_and_ui_slots_seed.md)
  owns slot projection and direct versus handoff surface semantics.
