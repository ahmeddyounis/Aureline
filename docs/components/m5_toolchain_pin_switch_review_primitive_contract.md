# M5 Toolchain-Pin Row, Precedence Inspector, and Switch-Review Card Primitive Contract

This document is the human-readable contract for the reusable M5 toolchain-pin-row,
precedence-inspector, and switch-review-card primitive implemented in
`crates/aureline-shell/src/implement_the_m5_toolchain_pin_row_precedence_inspector_and_switch_review_card_winning_scope_shadowed_layer_and_revert_or_repair_primitive/`.
It narrows the `toolchain_pin_row` family frozen in the runtime-boundary component
matrix (`schemas/ui/m5-runtime-boundary-components.schema.json`, task M05-852) into a
working primitive with a real resolver, and adds the precedence inspector and the
switch-review card that the acceptance criteria require.

## Goal

A user can review or change one interpreter, SDK, shell, kernel, or runtime choice
without guessing which scope currently wins, why it won, what it shadows, or what would
change after switching.

## The two halves

1. **Resolver — `resolve_toolchain_selection`.** Takes one toolchain target's ordered
   candidate layers (each with a scope, a source, and an opaque selection), its
   selection health, and an optional switch request. Produces one
   `M5ResolvedToolchainSelection`.
2. **Parity matrix — `M5ToolchainPinSwitchReviewPrimitivePacket`.** Binds one row per
   claimed M5 environment selector to the shared pin-row, precedence-inspector, and
   switch-review-card anatomy, and carries worked resolutions so the support / export
   packet reconstructs toolchain resolution from one shared model.

## Winning layer and precedence

Each candidate layer expresses a selection at one `M5PinScope`. Scopes are ordered by
precedence rank (lower wins):

| rank | scope | class |
| ---- | ----- | ----- |
| 0 | `policy_scope` | override |
| 1 | `session_scope` | override |
| 2 | `project_scope` | durable pin |
| 3 | `workspace_scope` | durable pin |
| 4 | `user_scope` | durable pin |
| 5 | `host_scope` | default |
| 6 | `global_default_scope` | default |

The **winning layer** is the present candidate with the lowest rank. Every other present
layer is ranked below it and carries an explicit `shadow_reason` — so a workspace or
policy override can never *silently* shadow a lower durable pin (**AC1**).

## Derived pin state

`M5ToolchainPinState` is derived in this order:

1. No explicit pin present (only host / global defaults) → `unpinned`.
2. Selection health is `missing_unavailable` → `pinned_missing_fallback`.
3. Winner is an override (`policy` / `session`) shadowing a differing durable pin →
   `pin_overridden`.
4. Winner is a durable pin and another durable pin disagrees → `pin_conflict`.
5. Otherwise → `pinned_resolved`.

## Actions (revert or repair)

`M5PinAction` is derived so a review or change is never a dead-end:

- `review_precedence` — always available.
- `clear_override` — when the winning layer is an override scope.
- `revert_to_shadowed_pin` — when a shadowed durable pin is present.
- `repair_selection` — when the selection health is degraded / mismatched / missing
  (**AC3**: repair and revert stay explicit when the current selection is degraded or
  mismatched).

## Switch-review card (blast radius before switching)

When a switch is requested, the resolver predicts an `M5SwitchReview` so a user can
review the winning / losing layers and the predicted blast radius *before* switching
environments (**AC2**):

- **blast radius** (`M5RepairBlastRadius`): reconnect → `multi_target_scoped`; a policy /
  host / global target → `host_environment_scoped`; a restart → `toolchain_scoped`;
  otherwise `workspace_scoped`.
- **reversibility** (`M5ReversibilityClass`): a safe local-only fallback with no
  reconnect → `fully_reversible_checkpoint`; a fallback with reconnect →
  `reversible_with_backup`; a reconnect with no fallback →
  `reversal_requires_manual_steps`; a restart with no fallback → `partially_reversible`;
  otherwise `reversible_with_backup`.
- The immediate changes, the restart / reconnect requirement, the newly blocked actions,
  and the safe local-only fallback are always shown.

## Acceptance criteria mapping

- **AC1 — no silent shadow.** Every non-winning layer carries a `shadow_reason`;
  `shadows_durable_pin` and `discloses_shadowed_pins` are exported; the packet lint
  `shadow_disclosure_unproven` requires a worked example proving it.
- **AC2 — review layers and blast radius before switching.** The precedence inspector
  exports the ordered stack; the switch-review card exports the predicted blast radius
  and reversibility; the packet lint `switch_blast_radius_unproven` requires a worked
  switch example.
- **AC3 — repair / revert stay explicit when degraded.** A degraded selection always
  keeps `repair_selection`; the packet lint `degraded_repair_unproven` requires a worked
  degraded example.

## Invariants

Every selector row asserts, and validation enforces, that the surface never:

- silently shadows a durable pin (`silently_shadows_durable_pin = false`);
- shows a degraded selection as cleanly resolved (`shows_degraded_as_resolved = false`);
- invents a private selection grammar (`invents_private_selection_grammar = false`);
- presents a switch without its blast radius (`hides_switch_blast_radius = false`).

## Export safety

Raw pin-file paths, raw version strings, raw usernames, tokens, credentials, and user
text bodies stay outside the support boundary. Every target title, selection, and switch
target is carried only as an opaque, export-safe representation; the resolver and the
packet validation both reject obviously forbidden material.

## Source contracts

- `schemas/ui/m5-toolchain-pin-row.schema.json` — the packet (validation target).
- `schemas/ui/m5-context-precedence-inspector.schema.json` — the precedence-inspector
  component fragment.
- `schemas/shell/m5-shell-zone.schema.json` — the frozen shell topology.
- `schemas/ui/m5-runtime-boundary-components.schema.json` — the frozen matrix narrowed
  here.
- `schemas/runtime/finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json`
  — the toolchain-manager parity source.
- `schemas/settings/precedence_resolution.schema.json` — the precedence-resolution
  source.

## Artifacts

- `artifacts/release/m5-toolchain-pin-switch-review-proof/support_export.json` — the
  canonical support export (the crate embeds this via `include_str!`).
- `artifacts/release/m5-toolchain-pin-switch-review-proof/matrix.csv` — the
  machine-readable matrix.
- `artifacts/components/m5-toolchain-pin-switch-review-primitive.md` — the Markdown
  report.
- `fixtures/ui/m5-toolchain-pin-switch-review-primitive/` — the narrowed fixtures.

All artifacts are minted only by the
`aureline_shell_m5_toolchain_pin_switch_review_primitive` emitter; the tests assert the
checked-in artifacts equal the seed builders.
