# M5 keybinding resolver inspection certification contract

Keybinding resolver inspectors, conflict-review sheets, and import-bridge outcome rows for every claimed
M5 command surface (task **M05-742**, batch B86).

This lane is the **resolver-inspection capstone** over the frozen
[M5 menu-affordance, keybinding-resolver, and command-documentation matrix][matrix] (task M05-740). It
certifies, for every one of the ten governed command-surface families, that shortcut resolution is
*inspectable*: a user, a doc, an automation, or a support reviewer can see which binding wins, why it
wins, what lost, how an imported shortcut translated with one of the controlled bridge-outcome states, and
how a leader / multi-stroke shortcut resolves — without relying on hidden resolver knowledge. It mints no
parallel command vocabulary — every surface's canonical command binding, qualification, owner, required
labels, shortcut-source classes, conflict reasons, import-translation states, stale-target states,
why-unavailable reasons, feature families, declared consumer surfaces, and applicable downgrade triggers
are pulled straight from the frozen matrix, and the winner/shadowed resolution is derived from the
matrix's shortcut-source precedence.

[matrix]: ./m5_discoverability_affordances_contract.md

## Certified surface families

Row key is the frozen `M5CommandSurfaceFamily` — one inspection row per family:

- `menu_item`
- `menu_group`
- `context_menu`
- `command_bar`
- `keybinding_resolver_layer`
- `conflict_review_sheet`
- `import_bridge_row`
- `disabled_command_explainer`
- `leader_sequence_help`
- `command_documentation_surface`

## Inspection dimensions

Each row is certified on four tri-state inspection dimensions (each maps to an acceptance criterion or
implementation requirement) plus a headless-parity hard invariant:

| Dimension | Full (green) | Disclosed narrowing (yellow) | Blocked (red) |
| --------- | ------------ | ---------------------------- | ------------- |
| **Resolver inspection** (AC1) | `winner_shadowed_source_and_fallback_certified` | `disclosed_reduced_inspector_detail` | `winning_or_shadowed_binding_hidden` |
| **Bridge outcome** (AC2) | `controlled_states_and_migration_actions_certified` | `disclosed_partial_bridge_coverage` | `generic_imported_wording_used` |
| **Leader sequence inspection** (AC3) | `precedence_timeout_cancel_narration_certified` | `disclosed_reduced_sequence_hint` (**requires an active waiver**) | `sequence_availability_requires_hidden_knowledge` |
| **Resolver export** | `command_id_and_winning_source_reconstructable` | `disclosed_partial_capture` | `winning_source_absent_from_capture` |

The derived green/yellow/red status is **recomputed**, never asserted: any hard blocker forces `red`, any
disclosed narrowing forces `yellow`, otherwise `green`. A disclosed reduced sequence hint — folding a
leader continuation's next-key list — is the sensitive narrowing and stays `yellow` only when an active
waiver discloses it.

## Inspector fields

The resolver inspector must reveal all seven fields the implementation requirements name, or the row
blocks:

- `source_layer`
- `scope`
- `current_mode`
- `active_winner`
- `losing_candidates`
- `reserved_unavailable_state`
- `fallback_command_path`

The `winning_source_class` and `shadowed_source_classes` are **derived** from the matrix's declared
shortcut-source classes by precedence: the highest-precedence source wins, the rest are shadowed.

## Controlled bridge outcomes and migration actions

Conflict-review sheets and import-bridge rows report one of six controlled bridge-outcome states rather
than generic imported wording — `exact`, `translated`, `alias_only`, `partial`, `shimmed`, `unsupported`
— and offer one of three migration actions where migration remains incomplete: `open_docs`, `manual_fix`,
`no_action_needed`. A row blocks unless it renders all six controlled states and offers all three actions.

## Structural completeness lints (hard blockers)

A row also blocks (`red`) unless it certifies:

- every one of the **seven inspector fields**;
- every one of the **six controlled bridge-outcome states**;
- every one of the **three migration actions**;
- every **consumer surface** the matrix declares for the family; and
- the same resolution preserved in **headless / CLI execution** (`headless_parity_preserved`).

## Records

- **Packet** (`ResolverInspectorPacket`): the full set of per-family rows, derived counts, active waivers,
  exact conformance causes, and blocking findings. Boundary schema:
  `schemas/commands/m5-keybinding-resolver-inspectors.schema.json`.
- **Dashboard** (`ResolverInspectorDashboard`): the light projection the keybinding UI / command palette /
  Support Center / CLI / help / migration tooling reads to auto-narrow a surface's resolver-inspection
  claim.
- **Support export** (`ResolverInspectorSupportExport`): the packet + dashboard + copy-safe case ids a
  support bundle, doc, or migration packet pivots on.

## Seed posture

Six families are green; four auto-narrow to yellow: the keybinding resolver layer discloses a reduced
inspector detail, the import-bridge row discloses a partial bridge coverage, the leader / sequence help
overlay carries a waivered reduced sequence hint, and the command-documentation surface discloses a
partial resolver/export capture. No row is blocked, so the packet is clean and every row is publishable.

## Published artifacts

- Markdown report: `artifacts/commands/m5-keybinding-resolver-inspectors.md`
- Packet: `artifacts/release/m5-keybinding-resolver-inspectors-proof/packet.json`
- Dashboard: `artifacts/release/m5-keybinding-resolver-inspectors-proof/dashboard.json`
- Support export: `artifacts/release/m5-keybinding-resolver-inspectors-proof/support_export.json`
- CSV: `artifacts/release/m5-keybinding-resolver-inspectors-proof/matrix.csv`
- Protected fixtures: `fixtures/commands/m5-keybinding-resolver-inspectors/packet.json`,
  `dashboard.json`, `support_export.json`, `compact.txt`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors -- validate
cargo test -p aureline-shell --lib m5_keybinding_resolver_inspectors
cargo test -p aureline-shell --test m5_keybinding_resolver_inspectors_fixtures
```

Regenerate the artifacts and fixtures (the headless emitter is the only mint-from-truth path) with the
`packet`, `dashboard`, `support-export`, `csv`, `markdown`, and `compact` subcommands of
`aureline_shell_m5_keybinding_resolver_inspectors`.
