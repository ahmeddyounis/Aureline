# M5 discoverability-release-proof certification contract

Menu-affordance, keybinding-resolver, leader-help, and command-documentation release-evidence proof for
every claimed M5 command surface (task **M05-747**, batch B86).

This lane is the **release-evidence publication capstone** over the frozen
[M5 menu-affordance, keybinding-resolver, and command-documentation matrix][matrix] (task M05-740). Where the
sibling parity lanes certify menu/context-menu parity, keybinding-resolver inspectability, leader/blocked
command explainability, and command-documentation truth one dimension at a time, this lane **bundles all four
discoverability truth dimensions into one release-evidence proof**, ties every claimed surface family to its
current menu/help/keybinding/leader/documentation proof, publishes the proof under the release-evidence index,
and auto-narrows a surface whose parity, narration, or docs/help anchors are stale or missing — so a
discoverability regression is detected mechanically before a stable/beta claim widens. It mints no parallel
command vocabulary — every surface's canonical command binding, qualification, owner, required labels,
lifecycle label, preview class, feature families, declared consumer surfaces, and applicable downgrade
triggers are pulled straight from the frozen matrix, and the six desktop profiles are re-exported from the
[desktop-profile certification][profiles].

[matrix]: ./m5_discoverability_affordances_contract.md
[profiles]: ../shell/m5_desktop_profile_certification_contract.md

## Certified surface families

Row key is the frozen `M5CommandSurfaceFamily` — one release-proof row per family:

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

## Discoverability truth dimensions

Each row is certified on four tri-state dimensions (each maps to the named discoverability truth the
acceptance criteria require a current proof row for) plus a headless-parity hard invariant:

| Dimension | Full (green) | Disclosed narrowing (yellow) | Blocked (red) |
| --------- | ------------ | ---------------------------- | ------------- |
| **Menu-affordance truth** (command-surface-parity proof) | `menu_affordance_parity_certified` | `disclosed_reduced_affordance_hint` | `alternate_label_or_authority_invented` |
| **Keybinding-resolver truth** (keybinding-resolver-inspector proof) | `shortcut_resolution_inspectable` | `disclosed_reduced_resolver_detail` | `winning_or_shadowed_binding_hidden` |
| **Leader-help truth** (command-explainer proof) | `leader_and_blocked_explainer_certified` | `disclosed_reduced_explainer_detail` (**requires an active waiver**) | `blocked_intent_silent_or_generic` |
| **Command-documentation truth** (command-documentation proof) | `command_doc_record_certified` | `disclosed_reduced_doc_detail` | `doc_record_stale_or_mismatched` |

The derived green/yellow/red status is **recomputed**, never asserted: any hard blocker forces `red`, any
disclosed narrowing forces `yellow`, otherwise `green`. A disclosed reduced explainer detail — reducing how a
blocked command explains itself — is the sensitive narrowing and stays `yellow` only when an active waiver
discloses it.

## Bundled proof dimensions and release-evidence index

The release-evidence proof ties every family to its four discoverability proof dimensions and publishes under
the release-evidence index, naming the sibling per-dimension proof lanes it rolls up:

- `menu_affordance` → `artifacts/release/m5-command-surface-parity-proof/packet.json`
- `keybinding_resolver` → `artifacts/release/m5-keybinding-resolver-inspectors-proof/packet.json`
- `leader_help` → `artifacts/release/m5-command-explainers-proof/packet.json`
- `command_documentation` → `artifacts/release/m5-command-documentation-proof/packet.json`

## Desktop profiles

The proof must tie to all six claimed desktop profiles (re-exported from the desktop-profile certification),
or the row blocks:

- `compact_desktop`
- `standard_desktop`
- `expanded_desktop`
- `mixed_dpi`
- `multi_monitor`
- `dependency_missing_restore`

## Structural completeness lints (hard blockers)

A row blocks (`red`) unless it certifies:

- every one of the **four discoverability proof dimensions**;
- across every one of the **six desktop profiles**;
- every **consumer surface** the matrix declares for the family; and
- the same proof preserved in **headless / CLI execution** (`headless_parity_preserved`).

## Records

- **Packet** (`ReleaseProofPacket`): the full set of per-family rows, derived counts, active waivers, exact
  conformance causes, blocking findings, the release-evidence index anchor, and the bundled proof lanes.
  Boundary schema: `schemas/commands/m5-discoverability-release-proof.schema.json`.
- **Dashboard** (`ReleaseProofDashboard`): the light projection the release center / command palette / menu /
  keybinding UI / help / Support Center / CLI tooling reads to auto-narrow a surface's discoverability claim.
- **Support export** (`ReleaseProofSupportExport`): the packet + dashboard + copy-safe case ids a support
  bundle, shiproom review, or migration packet pivots on.

## Seed posture

Six families are green; four auto-narrow to yellow: the command / action bar discloses a shortened affordance
hint, the keybinding resolver layer discloses a reduced inspector detail, the leader / sequence help overlay
carries a waivered reduced explainer detail on the compact-layout profile, and the command-documentation
surface discloses a reduced doc detail on one legacy surface. No row is blocked, so the packet is clean and
every row is publishable.

## Published artifacts

- Markdown report: `artifacts/commands/m5-discoverability-release-proof.md`
- Packet: `artifacts/release/m5-discoverability-release-proof-proof/packet.json`
- Dashboard: `artifacts/release/m5-discoverability-release-proof-proof/dashboard.json`
- Support export: `artifacts/release/m5-discoverability-release-proof-proof/support_export.json`
- CSV: `artifacts/release/m5-discoverability-release-proof-proof/matrix.csv`
- Protected fixtures: `fixtures/commands/m5-discoverability-release-proof/packet.json`, `dashboard.json`,
  `support_export.json`, `compact.txt`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_release_proof -- validate
cargo test -p aureline-shell --lib m5_discoverability_release_proof
cargo test -p aureline-shell --test m5_discoverability_release_proof_fixtures
```

Regenerate the artifacts and fixtures (the headless emitter is the only mint-from-truth path) with the
`packet`, `dashboard`, `support-export`, `csv`, `markdown`, and `compact` subcommands of
`aureline_shell_m5_discoverability_release_proof`.
