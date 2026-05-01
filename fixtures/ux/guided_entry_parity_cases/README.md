# Guided-entry parity audit cases

Worked parity audit cases for the command discoverability coverage
matrix and the onboarding hint-source ledger:

- `/docs/ux/command_discoverability_coverage_matrix.md`
- `/artifacts/ux/discoverability_coverage_rows.yaml`
- `/artifacts/ux/hint_source_ledger.yaml`

Each case is a single YAML document that exercises one
`case_category` from the closed set:

| `case_category` | Audit focus |
|---|---|
| `start_center_first_run` | Start Center primary zones (Open / Clone / Import / Restore / Recent work) reach canonical commands through palette, menu, inline zone, and onboarding card. |
| `clone_review` | Palette row, global menu, Start Center Clone zone, and contextual tip share canonical command id and docs anchor. |
| `import_profile` | Import command parity across palette, menu, Start Center Import zone, migration bridge card, and contextual tip. |
| `restore_from_checkpoint` | Restore parity across palette, menu, Start Center Restore card, and recent-work row. |
| `missing_target_recovery` | Recent-work row missing-target recovery is anchored on canonical Open / Restore / Remove rather than free-text "Locate" prose. |
| `trust_stage` | Trust-stage admission preserves canonical command identity through trust prompt and why-unavailable explainer. |
| `workspace_admission` | Workspace-admission flow reuses canonical command refs across palette, menu, and policy explainer. |
| `imported_keymap` | Imported-keymap chord is bound to the canonical palette command via `keymap_bridge:*`. |
| `migration_bridge` | Migration bridge card resolves to the canonical formatter / settings command without minting parallel ids. |

Each fixture carries:

- `command_id` &mdash; canonical action under audit.
- `coverage_row_ref` &mdash; row in
  `/artifacts/ux/discoverability_coverage_rows.yaml`.
- `hint_source_ledger_ref` &mdash; entry in
  `/artifacts/ux/hint_source_ledger.yaml`.
- `route_audit_rows` &mdash; one row per `surface_route_class` under
  audit, with `route_role_class`, `source_anchor_refs`, and
  `parity_finding_class`.
- `fuw_row_refs` &mdash; first-useful-work rows mirrored, where
  applicable.
- `parity_finding_class` &mdash; one of `parity_complete`,
  `parity_complete_with_disclosed_exception`,
  `parity_partial_known_gap`, or one of the four
  `parity_violation_*` classes.
- `exception_class`, `exception_owner_role`, `exception_review_ref`
  &mdash; populated when the row falls below the multi-route floor
  with a typed exception.

Cases are reusable: docs / help, migration, keymap-bridge, and
learnability evidence cite case ids without rewriting command names
or shortcut copy.
