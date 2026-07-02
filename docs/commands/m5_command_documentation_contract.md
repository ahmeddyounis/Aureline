# M5 command-documentation certification contract

Command-documentation surfaces, canonical examples, and alias/deprecation notes for every claimed M5
command family (task **M05-743**, batch B86).

This lane is the **command-documentation capstone** over the frozen
[M5 menu-affordance, keybinding-resolver, and command-documentation matrix][matrix] (task M05-740). It
certifies, for every one of the ten governed command-surface families, that the surface publishes
*documentation truth*: the same command id, primary label, aliases, lifecycle / deprecation state,
supported surfaces, invocation-schema summary, side-effect / risk class, and result / rollback semantics
that the shipped command record carries — with canonical examples that stay fresh across help, onboarding,
migration, CLI/headless, and support surfaces rather than drifting into a second naming system. It mints no
parallel command vocabulary — every surface's canonical command binding, qualification, owner, required
labels, lifecycle label, feature families, declared consumer surfaces, and applicable downgrade triggers
are pulled straight from the frozen matrix.

[matrix]: ./m5_discoverability_affordances_contract.md

## Certified surface families

Row key is the frozen `M5CommandSurfaceFamily` — one documentation row per family:

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

## Documentation dimensions

Each row is certified on four tri-state documentation dimensions (each maps to an acceptance criterion or
implementation requirement) plus a headless-parity hard invariant:

| Dimension | Full (green) | Disclosed narrowing (yellow) | Blocked (red) |
| --------- | ------------ | ---------------------------- | ------------- |
| **Documentation record** (AC1) | `command_record_examples_and_lifecycle_certified` | `disclosed_reduced_doc_detail` | `doc_record_missing_or_mismatched` |
| **Cross-surface naming** (AC2) | `canonical_naming_and_replacement_stable` | `disclosed_surface_paraphrase` (**requires an active waiver**) | `naming_or_replacement_drifted` |
| **Example freshness** (AC3) | `canonical_examples_fresh_and_not_alias_only` | `disclosed_partial_example_refresh` | `stale_or_alias_only_example_shipped` |
| **Doc export parity** | `command_id_and_replacement_reconstructable` | `disclosed_partial_capture` | `doc_truth_absent_from_capture` |

The derived green/yellow/red status is **recomputed**, never asserted: any hard blocker forces `red`, any
disclosed narrowing forces `yellow`, otherwise `green`. A disclosed surface paraphrase — shortening a
canonical label on a constrained surface — is the sensitive narrowing and stays `yellow` only when an
active waiver discloses it.

## Documentation-record fields

The documentation record must publish all eight fields the implementation requirements name, or the row
blocks:

- `command_id`
- `primary_label`
- `aliases`
- `lifecycle_state`
- `supported_surfaces`
- `invocation_schema_summary`
- `side_effect_risk_class`
- `result_rollback_semantics`

## Parity cards and derivation anchors

Each surface renders seven parity cards showing how the same command appears across every reach — `menu`,
`button`, `palette`, `cli_headless`, `ai_tool`, `recipe`, `voice_companion_hint` — and derives three
anchors from the shared command record rather than duplicating them by hand — `docs_help_anchor`,
`shortcut_notation`, `accessibility_narration_hint`.

## Structural completeness lints (hard blockers)

A row blocks (`red`) unless it certifies:

- every one of the **eight documentation-record fields**;
- every one of the **seven parity cards**;
- every one of the **three derivation anchors**;
- every **consumer surface** the matrix declares for the family; and
- the same documentation preserved in **headless / CLI execution** (`headless_parity_preserved`).

## Records

- **Packet** (`CommandDocPacket`): the full set of per-family rows, derived counts, active waivers, exact
  conformance causes, and blocking findings. Boundary schema:
  `schemas/commands/m5-command-documentation.schema.json`.
- **Dashboard** (`CommandDocDashboard`): the light projection the command palette / help / onboarding /
  Support Center / CLI / migration tooling reads to auto-narrow a surface's documentation claim.
- **Support export** (`CommandDocSupportExport`): the packet + dashboard + copy-safe case ids a support
  bundle, doc, or migration packet pivots on.

## Seed posture

Six families are green; four auto-narrow to yellow: the command / action bar discloses a reduced
documentation detail, the context menu carries a waivered surface paraphrase, the import-bridge row
discloses a partial example refresh, and the command-documentation surface discloses a partial copy-safe
export capture. No row is blocked, so the packet is clean and every row is publishable.

## Published artifacts

- Markdown report: `artifacts/commands/m5-command-documentation.md`
- Packet: `artifacts/release/m5-command-documentation-proof/packet.json`
- Dashboard: `artifacts/release/m5-command-documentation-proof/dashboard.json`
- Support export: `artifacts/release/m5-command-documentation-proof/support_export.json`
- CSV: `artifacts/release/m5-command-documentation-proof/matrix.csv`
- Protected fixtures: `fixtures/commands/m5-command-documentation/packet.json`, `dashboard.json`,
  `support_export.json`, `compact.txt`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_documentation -- validate
cargo test -p aureline-shell --lib m5_command_documentation
cargo test -p aureline-shell --test m5_command_documentation_fixtures
```

Regenerate the artifacts and fixtures (the headless emitter is the only mint-from-truth path) with the
`packet`, `dashboard`, `support-export`, `csv`, `markdown`, and `compact` subcommands of
`aureline_shell_m5_command_documentation`.
