# M5 discoverability-access-parity certification contract

Keyboard, screen-reader, touch, and support-export parity for menu, keybinding-help, and command-doc
surfaces across every claimed M5 desktop profile (task **M05-746**, batch B86).

This lane is the **accessibility / support-export parity capstone** over the frozen
[M5 menu-affordance, keybinding-resolver, and command-documentation matrix][matrix] (task M05-740). It
certifies, for every one of the ten governed command-surface families, that the discoverability surface
stays *usable without pointer hover* and *diagnosable after the fact*: it is fully keyboard- and
screen-reader-addressable with a focus-return and a touch / context-action equivalent for any hover-only
reach; the same command discoverability and blocked-state behaviour reconstructs from a structured,
copy-safe support/export packet rather than a screenshot or private team memory; it stays reachable and
stable across the claimed reduced-motion, high-zoom, compact-layout, and multi-window desktop profiles; and
the parity checks are wired into release evidence so a stale help anchor, missing narration text, or
hover-only discoverability regression auto-narrows the claim before release widening. It mints no parallel
command vocabulary — every surface's canonical command binding, qualification, owner, required labels,
lifecycle label, preview class, feature families, declared consumer surfaces, and applicable downgrade
triggers are pulled straight from the frozen matrix.

[matrix]: ./m5_discoverability_affordances_contract.md

## Certified surface families

Row key is the frozen `M5CommandSurfaceFamily` — one access-parity row per family:

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

## Access-parity dimensions

Each row is certified on four tri-state dimensions (each maps to an acceptance criterion or implementation
requirement) plus a headless-parity hard invariant:

| Dimension | Full (green) | Disclosed narrowing (yellow) | Blocked (red) |
| --------- | ------------ | ---------------------------- | ------------- |
| **Non-pointer reach** (AC1 / impl req 1) | `keyboard_screen_reader_and_touch_parity_certified` | `disclosed_reduced_touch_fallback` (**requires an active waiver**) | `hover_only_or_narration_missing` |
| **Support-export evidence** (AC2 / impl req 2) | `structured_incident_evidence_reconstructable` | `disclosed_partial_capture` | `blocked_state_absent_from_capture` |
| **Profile stability** (impl req 3) | `reachable_and_stable_across_all_profiles` | `disclosed_reduced_profile_coverage` | `surface_unreachable_or_unstable_in_profile` |
| **Release evidence** (AC3 / impl req 4) | `parity_checks_gate_release_evidence` | `disclosed_partial_evidence_refresh` | `stale_anchor_or_regression_unblocked` |

The derived green/yellow/red status is **recomputed**, never asserted: any hard blocker forces `red`, any
disclosed narrowing forces `yellow`, otherwise `green`. A disclosed reduced touch fallback — reducing a
surface's touch reach on a constrained surface — is the sensitive narrowing and stays `yellow` only when an
active waiver discloses it.

## Non-pointer reach channels

The surface must stay reachable through all five non-pointer reach channels the implementation requirements
name, or the row blocks:

- `pointer_default`
- `keyboard_path`
- `screen_reader_narration`
- `focus_return`
- `touch_context_action`

## Accessibility-incident fields

The support-export must capture all five accessibility-incident fields the implementation requirements name
so a reviewer can reconstruct the incident without a screenshot, or the row blocks:

- `command_id`
- `source_layer`
- `conflict_or_blocker_reason`
- `lifecycle_state`
- `help_anchor_ref`

## Desktop access profiles

The surface must stay reachable and stable across all four desktop access profiles the implementation
requirements name, or the row blocks:

- `reduced_motion`
- `high_zoom`
- `compact_layout`
- `multi_window`

## Structural completeness lints (hard blockers)

A row blocks (`red`) unless it certifies:

- every one of the **five non-pointer reach channels**;
- every one of the **five accessibility-incident fields**;
- every one of the **four desktop access profiles**;
- every **consumer surface** the matrix declares for the family; and
- the same parity preserved in **headless / CLI execution** (`headless_parity_preserved`).

## Records

- **Packet** (`AccessParityPacket`): the full set of per-family rows, derived counts, active waivers, exact
  conformance causes, and blocking findings. Boundary schema:
  `schemas/commands/m5-discoverability-access-parity.schema.json`.
- **Dashboard** (`AccessParityDashboard`): the light projection the command palette / menu / keybinding UI /
  help / Support Center / CLI tooling reads to auto-narrow a surface's accessibility / export claim.
- **Support export** (`AccessParitySupportExport`): the packet + dashboard + copy-safe case ids a support
  bundle, doc, or migration packet pivots on.

## Seed posture

Six families are green; four auto-narrow to yellow: the command / action bar carries a waivered reduced
touch fallback, the import-bridge row discloses a partial support-export capture, the leader / sequence help
overlay discloses a reduced profile coverage on the compact-layout profile, and the command-documentation
surface discloses a partial release-evidence refresh. No row is blocked, so the packet is clean and every
row is publishable.

## Published artifacts

- Markdown report: `artifacts/commands/m5-discoverability-access-parity.md`
- Packet: `artifacts/release/m5-discoverability-access-parity-proof/packet.json`
- Dashboard: `artifacts/release/m5-discoverability-access-parity-proof/dashboard.json`
- Support export: `artifacts/release/m5-discoverability-access-parity-proof/support_export.json`
- CSV: `artifacts/release/m5-discoverability-access-parity-proof/matrix.csv`
- Protected fixtures: `fixtures/commands/m5-discoverability-access-parity/packet.json`, `dashboard.json`,
  `support_export.json`, `compact.txt`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_access_parity -- validate
cargo test -p aureline-shell --lib m5_discoverability_access_parity
cargo test -p aureline-shell --test m5_discoverability_access_parity_fixtures
```

Regenerate the artifacts and fixtures (the headless emitter is the only mint-from-truth path) with the
`packet`, `dashboard`, `support-export`, `csv`, `markdown`, and `compact` subcommands of
`aureline_shell_m5_discoverability_access_parity`.
