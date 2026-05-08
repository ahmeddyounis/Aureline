# Command enablement / disabled-reason cases

These fixtures exercise the command enablement engine by providing:

- a canonical `command_id` (resolved through the seeded command registry), and
- an evaluation context (client scope, trust state, and dependency availability),
- an expected `(decision_class, disabled_reason_code)` pair.

The goal is to keep the disabled-reason vocabulary and repair-hook behavior
consistent across all shell surfaces (palette, menus, keybindings, diagnostics,
help/export).

