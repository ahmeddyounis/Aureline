# M5 command-explainer certification contract

Leader / partial-sequence overlays, disabled-command and why-unavailable explainers, shared remediation,
and copy-safe blocker export for every claimed M5 command family (task **M05-744**, batch B86).

This lane is the **explainer capstone** over the frozen
[M5 menu-affordance, keybinding-resolver, and command-documentation matrix][matrix] (task M05-740). It
certifies, for every one of the ten governed command-surface families, that Aureline can *explain blocked
or in-progress keyboard-first intent*: a leader / partial-sequence overlay narrates next-available actions;
a disabled-command / why-unavailable explainer names the blocker class, next safe action, and copy-id /
open-help actions; the same reason packet and remediation language stay stable across the palette, menu,
keybinding UI, onboarding tips, and support/export flows; and the blocker reason and command id reconstruct
from a copy-safe, diffable export without a screenshot. It mints no parallel command vocabulary — every
surface's canonical command binding, qualification, owner, required labels, lifecycle label, feature
families, why-unavailable reasons, declared consumer surfaces, and applicable downgrade triggers are pulled
straight from the frozen matrix.

[matrix]: ./m5_discoverability_affordances_contract.md

## Certified surface families

Row key is the frozen `M5CommandSurfaceFamily` — one explainer row per family:

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

## Explainer dimensions

Each row is certified on four tri-state explainer dimensions (each maps to an acceptance criterion or
implementation requirement) plus a headless-parity hard invariant:

| Dimension | Full (green) | Disclosed narrowing (yellow) | Blocked (red) |
| --------- | ------------ | ---------------------------- | ------------- |
| **Leader overlay** (AC3 / impl req 1) | `typed_prefix_next_keys_and_timeout_narrated` | `disclosed_reduced_sequence_overlay` (**requires an active waiver**) | `sequence_availability_requires_hidden_knowledge` |
| **Blocked explainer** (AC1 / impl req 2) | `blocker_class_next_action_and_actions_certified` | `disclosed_reduced_explainer_detail` | `blocked_command_fails_silently_or_generic` |
| **Remediation parity** (AC2 / impl req 3) | `shared_reason_packet_across_all_surfaces` | `disclosed_surface_local_remediation_note` | `surface_local_error_prose_invented` |
| **Explainer export** (copy-safe introspection) | `blocker_and_command_id_reconstructable` | `disclosed_partial_capture` | `blocker_reason_absent_from_capture` |

The derived green/yellow/red status is **recomputed**, never asserted: any hard blocker forces `red`, any
disclosed narrowing forces `yellow`, otherwise `green`. A disclosed reduced sequence overlay — folding the
overlay's resulting-label detail on a constrained surface — is the sensitive narrowing and stays `yellow`
only when an active waiver discloses it.

## Leader-overlay fields

The leader / partial-sequence overlay must narrate all six fields the implementation requirements name, or
the row blocks:

- `typed_prefix`
- `current_mode`
- `available_next_keys`
- `resulting_command_label_and_id`
- `timeout_cancel_posture`
- `surface_unsupported_note`

## Blocker classes

The disabled-command / why-unavailable explainer must be able to name all seven blocker classes the
implementation requirements name — `context`, `trust`, `policy`, `lifecycle`, `missing_dependency`,
`stale_target`, `mode_overlay` — the coarse taxonomy each explainer groups the matrix's finer
`M5UnavailableReason` set under (carried per-row as `covered_unavailable_reasons`, pulled from the matrix).

## Remediation actions and reach modes

Each explainer offers three remediation actions — `next_safe_action`, `copy_command_id`, `open_help` — and
stays reachable in all five reach modes so the explanation is never hidden behind hover or modal confusion —
`pointer_default`, `keyboard_only`, `screen_reader`, `compact_layout`, `touch_context_action`.

## Structural completeness lints (hard blockers)

A row blocks (`red`) unless it certifies:

- every one of the **six leader-overlay fields**;
- every one of the **seven blocker classes**;
- every one of the **three remediation actions**;
- every one of the **five reach modes**;
- every **consumer surface** the matrix declares for the family; and
- the same explanation preserved in **headless / CLI execution** (`headless_parity_preserved`).

## Records

- **Packet** (`CommandExplainerPacket`): the full set of per-family rows, derived counts, active waivers,
  exact conformance causes, and blocking findings. Boundary schema:
  `schemas/commands/m5-command-explainers.schema.json`.
- **Dashboard** (`CommandExplainerDashboard`): the light projection the command palette / menu / keybinding
  UI / onboarding / Support Center / CLI tooling reads to auto-narrow a surface's explanation claim.
- **Support export** (`CommandExplainerSupportExport`): the packet + dashboard + copy-safe case ids a
  support bundle, doc, or migration packet pivots on.

## Seed posture

Six families are green; four auto-narrow to yellow: the leader / sequence help overlay carries a waivered
reduced sequence overlay, the command / action bar discloses a reduced explainer detail, the context menu
discloses a short surface-local remediation note, and the import-bridge row discloses a partial copy-safe
export capture. No row is blocked, so the packet is clean and every row is publishable.

## Published artifacts

- Markdown report: `artifacts/commands/m5-command-explainers.md`
- Packet: `artifacts/release/m5-command-explainers-proof/packet.json`
- Dashboard: `artifacts/release/m5-command-explainers-proof/dashboard.json`
- Support export: `artifacts/release/m5-command-explainers-proof/support_export.json`
- CSV: `artifacts/release/m5-command-explainers-proof/matrix.csv`
- Protected fixtures: `fixtures/commands/m5-command-explainers/packet.json`, `dashboard.json`,
  `support_export.json`, `compact.txt`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_explainers -- validate
cargo test -p aureline-shell --lib m5_command_explainers
cargo test -p aureline-shell --test m5_command_explainers_fixtures
```

Regenerate the artifacts and fixtures (the headless emitter is the only mint-from-truth path) with the
`packet`, `dashboard`, `support-export`, `csv`, `markdown`, and `compact` subcommands of
`aureline_shell_m5_command_explainers`.
