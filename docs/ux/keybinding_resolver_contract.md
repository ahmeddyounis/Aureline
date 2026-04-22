# Keybinding resolver contract

This document freezes the keybinding-resolution, conflict-review,
disabled-command explainability, import-bridge fidelity, and
leader-sequence inspection contract every shell, settings,
migration, docs/help, and support surface reads when it needs to
answer "what happens if I press this?", "why did that command
win?", "why is this command unavailable here?", or "did the
imported shortcut stay exact?".

The machine-readable boundary is
[`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json);
worked examples live in
[`/fixtures/commands/keybinding_conflict_examples/`](../../fixtures/commands/keybinding_conflict_examples/).
This contract builds on the frozen command-descriptor and
interaction-safety lanes rather than redefining command risk or
approval semantics:

- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  freezes command identity, disabled-reason vocabulary, preview
  class, approval posture, and result-evidence requirements.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  and
  [`/schemas/migration/migration_session.schema.json`](../../schemas/migration/migration_session.schema.json)
  freeze the durable migration-session and shortcut-digest
  records this contract now extends with high-frequency
  shortcut-diff rows.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  freezes the preview / apply / revert, approval, and audit
  envelopes that high-risk commands inherit even when invoked by
  shortcut.

This document is normative. If it disagrees with the PRD, TAD,
TDD, or the command-descriptor lane, those documents win and
this contract plus the schema export MUST be updated in the same
change.

## Why freeze this now

Without a frozen resolver contract, Aureline would ship one
shortcut truth in the editor, another in modal overlays, another
in settings, another in migration review, and another in support
export. The same sequence could look "available" in one surface,
"shadowed" in another, and "exactly imported" in a migration
digest even though the target command, mode, or approval posture
changed.

This contract prevents that drift before a live router lands:

- one precedence model names which layer wins and which layers
  lost;
- one conflict-review packet answers how the winner could change;
- one disabled-command packet explains why a bound command still
  may not run;
- one import-bridge fidelity taxonomy forbids labeling a changed
  shortcut as exact; and
- one leader / multi-stroke overlay row exposes waiting-state,
  timeout, and pivot paths without hand-maintained copies.

## Scope

Frozen at this revision:

- the canonical precedence model for platform reservation,
  emergency/security hard blocks, admin/policy locks, temporary
  overlays, user/profile bindings, workspace recommendations,
  extension bindings, and core defaults;
- one `keybinding_resolution_packet_record` that exposes the
  inspected sequence, the active scope, the precedence trace, the
  winner, the losers, and the next-safe actions;
- one `keybinding_conflict_review_packet_record` that makes
  winner/loser reasoning, outcome-change paths, and repair pivots
  directly addressable;
- one `disabled_command_explanation_packet_record` for bindings
  whose command metadata still denies execution on the current
  surface, trust state, or policy posture;
- one `keybinding_import_bridge_record` and one
  `shortcut_diff_after_import_row_record` so migration and
  profile-import surfaces use a closed shortcut-fidelity
  vocabulary; and
- one `leader_overlay_row_record` with waiting-state, timeout,
  and pivot semantics for partial, ambiguous, and host-limited
  sequences.

Out of scope until a superseding decision row opens:

- the live resolver implementation and the keybinding editor or
  remapping UI;
- shipping every incumbent keymap bridge or every modal-editing
  dialect;
- localised product copy beyond the stable ids, refs, and typed
  fields frozen here; and
- runtime telemetry aggregation or support-bundle renderers beyond
  the records defined here.

## Canonical precedence model

The resolver evaluates the layers below in strict order. Earlier
rows beat later rows. The packet MUST expose every layer in the
precedence trace; hidden imperative fallthrough is non-conforming.

| Rank | Layer | What it means | What it may do |
|---|---|---|---|
| 0 | `platform_reserved` | OS, browser, or host shell captures the sequence before Aureline dispatch | stop dispatch and explain the host reservation |
| 1 | `emergency_security_hard_block` | incident freeze, security kill switch, trust-critical deny rule, or emergency escape reservation | deny dispatch regardless of bound command source |
| 2 | `admin_policy_lock` | managed policy forbids, remaps, or narrows the binding | deny or force the policy-owned outcome |
| 3 | `temporary_mode_overlay` | temporary mode, leader state, operator-pending state, review mode, or scoped overlay changes meaning | override durable bindings while the overlay is active |
| 4 | `user_profile_binding` | durable user or profile truth, including imported keymaps once applied | win over workspace, extension, and default bindings |
| 5 | `workspace_recommendation` | workspace-scoped recommendation or project-provided shortcut suggestion | apply only when the user/profile layer is silent |
| 6 | `extension_binding` | built-in or third-party extension-contributed binding | win only when higher layers are silent |
| 7 | `core_default` | shipped Aureline fallback | baseline discoverability and help truth |

Rules:

1. `platform_reserved`, `emergency_security_hard_block`, and
   `admin_policy_lock` are blocking layers, not convenience
   aliases. Later bindings do not run when one of these layers
   wins.
2. Imported keymaps do not create a secret precedence tier. They
   materialise as `user_profile_binding` or
   `workspace_recommendation`; their imported origin is carried by
   import-bridge metadata, not by a hidden extra layer.
3. Within the same layer, the resolver breaks ties by declared
   specificity in this order: exact surface support, exact mode,
   exact focus/context, exact completed sequence shape, then exact
   scope. If two candidates still tie after that, the packet MUST
   emit `same_layer_collision_requires_review`; an arbitrary
   winner is forbidden.
4. High-risk preview, approval, and audit rules come from the
   canonical command descriptor. Shortcut source never widens or
   narrows authority.

## Sequence shapes, waiting states, and timeout rules

The resolver recognises four sequence shapes:

- `single_stroke`
- `multi_stroke_chord`
- `leader_sequence`
- `operator_pending_sequence`

Rules:

1. A partial prefix MUST produce an explicit waiting state when it
   could still become a valid multi-stroke, leader, or
   operator-pending sequence. Silent failure is non-conforming.
2. `leader_sequence` and `multi_stroke_chord` help rows are
   generated from the same command registry and command ids used
   by menus, docs, automation, and palette results. Hand-
   maintained copies are forbidden.
3. A sequence MAY be `Exact` only when the imported and resolved
   shortcut preserve command identity, gesture meaning, stroke
   count, leader requirement, mode semantics, surface reach, and
   preview/approval/audit posture.
4. If the current surface cannot honour a sequence faithfully, the
   packet says so with `unsupported_on_surface`,
   `blocked_by_host`, or the relevant hard-block reason. The
   resolver does not approximate silently.
5. Every waiting or blocked sequence-help row exposes at least one
   pivot to palette, settings, docs, conflict review, or
   migration guidance without losing current context.

## Packet families

The full field sets live in
[`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json).
The notable records are:

- **`keybinding_resolution_packet_record`.** The one-shot answer
  to "what does this sequence do here?". It carries the inspected
  sequence, active scope, waiting or resolved state, complete
  precedence trace, winning resolution, losing candidates, and
  next-safe actions.
- **`keybinding_conflict_review_packet_record`.** The review
  packet for "which command won, what lost, why, and what would
  change the outcome?". It is what migration review, settings,
  docs, and support should quote when a shortcut dispute needs
  inspection rather than silent fallback.
- **`disabled_command_explanation_packet_record`.** Used when a
  binding resolves to a command id but the command descriptor
  still denies execution because of trust, policy, client scope,
  dependency absence, or another typed disabled reason.
- **`keybinding_import_bridge_record`.** One row describing how an
  imported shortcut mapped into Aureline, including the frozen
  fidelity class and every behavior-change axis that prevents an
  `Exact` claim.
- **`leader_overlay_row_record`.** One row in a which-key or
  sequence-help surface. It freezes current prefix, next-key
  options, availability truth, timeout posture, and pivot paths.
- **`shortcut_diff_after_import_row_record`.** One high-frequency
  migration digest row reserved for commands whose shortcut
  changes are likely to break daily muscle memory.

## Winning resolution and conflict review

The resolution and conflict-review packets both expose one
`winning_resolution`. That winner is not always a command:

- `command_candidate`
- `platform_reserved`
- `emergency_security_hard_block`
- `admin_policy_lock`
- `waiting_state`
- `unbound`

Rules:

1. A packet that cannot say which of those outcomes won is
   incomplete. "No idea" is not a valid resolver result.
2. Every losing candidate carries both a typed loss reason and at
   least one `outcome_change_condition` naming what would change
   the result: remove a higher-precedence binding, exit the
   current mode, move focus, review policy, disable an extension
   binding, or rebind the sequence.
3. Conflict-review packets are the canonical place to explain
   precedence disputes. Settings, migration review, docs/help, and
   support surfaces should quote them rather than recreating local
   summaries.

## Disabled-command explanation

Binding a sequence to a command id does not guarantee that the
command may run on the current surface. The disabled-command
packet therefore freezes:

- the winning sequence and source layer;
- the canonical command id and command revision;
- the inherited `preview_class`, `approval_posture_class`, and
  `capability_scope_class`;
- the typed `disabled_reason_code` and `repair_hook_ref`;
- what would enable the command; and
- the next safe action or lower-authority fallback.

Rules:

1. The packet quotes the command-descriptor disabled reason rather
   than inventing a keybinding-only synonym.
2. If the command has a lower-authority fallback, the packet names
   it explicitly. If not, it says so plainly.
3. High-risk commands keep the same preview, approval, and audit
   semantics regardless of whether they were invoked by default,
   imported, workspace-suggested, or extension-contributed
   bindings.

## Import-bridge fidelity classes

Shortcut bridges use the closed fidelity set below:

| Class | Meaning |
|---|---|
| `exact` | command identity, gesture meaning, stroke count, leader requirement, mode semantics, surface reach, and preview/approval/audit posture are preserved |
| `translated` | the target command remains semantically equivalent, but at least one user-visible shortcut behavior changed |
| `alias_only` | Aureline can point to the same canonical command id or alias, but does not preserve a truthful direct shortcut parity claim |
| `partial` | only some surfaces, modes, or contexts preserve behavior; review remains required |
| `shimmed` | parity depends on an explicit bridge, extension, or shim layer; native parity is not claimed |
| `unsupported` | Aureline cannot truthfully represent the shortcut behavior |

Rules:

1. `exact` is forbidden when any behavior-change axis is present.
2. Introducing or removing a leader key, changing stroke count,
   narrowing the supported surface set, or changing preview or
   approval posture automatically disqualifies `exact`.
3. `shimmed` requires an explicit bridge or shim reference.
4. `unsupported` is not a subtype of `partial`; it is a terminal
   fidelity outcome.

## High-frequency shortcut diffs after import

The migration shortcut digest now reserves
`high_frequency_diff_rows` for the subset of imported shortcuts
that are most likely to affect daily muscle memory.

Each row names:

- the source tool and source command;
- the source and resulting sequences;
- the import-bridge fidelity class;
- the resulting resolver layer;
- the inherited preview, approval, and evidence posture of the
  resulting command;
- the conflict-review and rollback refs when applicable; and
- the next-safe follow-up action.

This is the only place migration, settings, docs, and support
should look when they need the "what changed for my daily
shortcuts?" answer.

## Related contracts

- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  owns command semantics.
- [`/docs/commands/command_graph_and_ui_slots_seed.md`](../commands/command_graph_and_ui_slots_seed.md)
  owns discoverability-slot translation.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  owns durable migration-session linkage.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  owns preview / approval / audit envelopes.
