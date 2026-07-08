# M5 Sequence-Help-Strip Primitive

- Packet: `m5-sequence-help-strip-primitive:stable:0001`
- Label: `M5 sequence-help-strip primitive: sequence-help state, sequence step kind, command-backing state, current-mode-or-leader reference, valid next keys, cancel key, example-command reference, screen-reader announcement, full-cheat-sheet reference, derived help posture (ready-for-input/awaiting-next-key/partial-sequence/unbound-dead-end/conflicting-binding/disabled-in-context), and bounded show-valid-next-keys/run-example-command/resolve-conflicting-binding/cancel-sequence/open-full-cheat-sheet actions`
- Modal / command-language consumers: 5 (5 stable)
- Help postures: ready_for_input, awaiting_next_key, partial_sequence, unbound_dead_end, conflicting_binding, disabled_in_context
- Help actions: show_valid_next_keys, run_example_command, resolve_conflicting_binding, cancel_sequence, open_full_cheat_sheet
- Help states: ready, awaiting_next_key, partial_match, no_binding, conflicting_binding, disabled_in_context
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Modal / command-language consumers

- **Leader-Sequence Overlay**: `stable`
  - Owner: Leader-sequence overlay owner
  - Scope: The leader-sequence overlay renders the shared sequence-help strip so a leader key ready for its first key shows the current leader, the valid next keys, the cancel key, and an example command, and a chord awaiting its next key shows the same current-mode / next-keys / cancel / example guidance — every open leader sequence is inspectable before it completes, with a screen-reader announcement and no reliance on pointer hover
  - Worked sequences: 2
    - `strip:leader-overlay:leader-root` (`ready` / `leader_key`) → `ready_for_input` (awaiting `true`, dead-end `false`, ambiguous `false`, backed `true`)
    - `strip:leader-overlay:leader-git` (`awaiting_next_key` / `chord`) → `awaiting_next_key` (awaiting `true`, dead-end `false`, ambiguous `false`, backed `true`)
- **Modal-Operator Strip**: `stable`
  - Owner: Modal-operator strip owner
  - Scope: The modal-operator strip renders the shared sequence-help strip so a partial operator awaiting its motion shows the operator mode, the valid motion keys, the cancel key, and an example command, and a terminal action disabled in the current context is shown honestly as disabled — still naming the current mode, the cancel key, and the example command and keeping the full cheat sheet reachable — so a keyboard-first user always knows why the operator will not complete
  - Worked sequences: 2
    - `strip:modal-operator:delete-motion` (`partial_match` / `operator`) → `partial_sequence` (awaiting `true`, dead-end `false`, ambiguous `false`, backed `true`)
    - `strip:modal-operator:record-macro-disabled` (`disabled_in_context` / `terminal_action`) → `disabled_in_context` (awaiting `false`, dead-end `false`, ambiguous `false`, backed `true`)
- **Partial-Command Hint**: `stable`
  - Owner: Partial-command hint owner
  - Scope: The partial-command hint renders the shared sequence-help strip so a keystroke run that resolves to no binding is shown honestly as an unbound dead end — naming the current mode, keeping the cancel key and the full cheat sheet reachable, and never failing silently — and a partial motion match shows the valid next keys, the cancel key, and an example command so an ambiguous partial command is always interpretable without external docs
  - Worked sequences: 2
    - `strip:partial-command:unbound-prefix` (`no_binding` / `prefix_argument`) → `unbound_dead_end` (awaiting `false`, dead-end `true`, ambiguous `false`, backed `false`)
    - `strip:partial-command:go-motion` (`partial_match` / `motion`) → `partial_sequence` (awaiting `true`, dead-end `false`, ambiguous `false`, backed `true`)
- **Command-Palette Sequence Hint**: `stable`
  - Owner: Command-palette sequence hint owner
  - Scope: The command-palette sequence hint renders the shared sequence-help strip so a conflicting binding is shown honestly as ambiguous — offering a resolve-conflicting-binding action, showing the conflicting next keys, the cancel key, and an example command — and a leader ready for its first palette-prefix key shows the valid prefix keys, the cancel key, and an example command, so an ambiguous command-language sequence is always resolvable in-product
  - Worked sequences: 2
    - `strip:command-palette:ctrl-k-conflict` (`conflicting_binding` / `chord`) → `conflicting_binding` (awaiting `false`, dead-end `false`, ambiguous `true`, backed `true`)
    - `strip:command-palette:prefix-ready` (`ready` / `leader_key`) → `ready_for_input` (awaiting `true`, dead-end `false`, ambiguous `false`, backed `true`)
- **Support Sequence Export**: `stable`
  - Owner: Support sequence export owner
  - Scope: The support sequence export renders the shared sequence-help strip so an awaiting-next-key motion exports its current mode, valid next keys, cancel key, and example command intact, and a disabled operator with no command backing exports honestly as disabled — keeping its current mode, cancel key, and cheat-sheet route — so support can reconstruct exactly what a keyboard-first user saw, with no raw keystroke log or buffer leaking across the boundary
  - Worked sequences: 2
    - `strip:support-export:go-line-motion` (`awaiting_next_key` / `motion`) → `awaiting_next_key` (awaiting `true`, dead-end `false`, ambiguous `false`, backed `true`)
    - `strip:support-export:indent-disabled` (`disabled_in_context` / `operator`) → `disabled_in_context` (awaiting `false`, dead-end `false`, ambiguous `false`, backed `false`)
