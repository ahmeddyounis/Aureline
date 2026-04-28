# Source Ecosystem Coverage Matrix

This document defines the migration lanes Aureline may name in product
docs, onboarding, compatibility reports, release evidence, support
exports, and migration-center help. It turns broad switching language into
stable source rows with explicit import targets, quality bars, caveats,
owners, and proof burdens.

Companion artifacts:

- [`/artifacts/migration/source_ecosystem_rows.yaml`](../../artifacts/migration/source_ecosystem_rows.yaml)
  is the canonical machine-readable source row inventory.
- [`/artifacts/migration/quality_bar_rubric.yaml`](../../artifacts/migration/quality_bar_rubric.yaml)
  is the canonical quality-bar vocabulary.
- [`/fixtures/migration/source_profile_examples/`](../../fixtures/migration/source_profile_examples/)
  contains example source profiles for the claimed rows.
- [`/docs/migration/migration_center_object_model.md`](./migration_center_object_model.md)
  defines session, outcome, shortcut-digest, and restore-record packets.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](./first_run_import_diff_and_rollback_contract.md)
  defines first-run dry-run planning, preview diff, apply, rollback,
  and imported-profile history packets for those source rows.
- [`/docs/migration/compatibility_scorecard_contract.md`](./compatibility_scorecard_contract.md),
  [`/schemas/migration/compatibility_scorecard.schema.json`](../../schemas/migration/compatibility_scorecard.schema.json),
  and [`/artifacts/migration/top_imported_workflow_rows.yaml`](../../artifacts/migration/top_imported_workflow_rows.yaml)
  define the imported-extension, imported-workflow, and workflow-bundle
  scorecard rows that keep blocker and native-alternative claims current.

The rows here are scope definitions, not a claim that every importer is
already implemented. A release may market a source lane only by citing the
matching row id and its current evidence state.

## Non-Claimed Source Policy

Only the source ecosystems listed in
`artifacts/migration/source_ecosystem_rows.yaml#source_ecosystem_rows` are
in the marketed migration claim:

- VS Code / Code-OSS
- JetBrains IDEs
- Vim / Neovim
- Emacs
- Sublime / TextMate

Other editors, hosted IDEs, browser IDEs, terminal multiplexers, language
servers, package managers, and bespoke dotfile frameworks are absent by
design. They may appear later as community, experimental, or generic import
paths, but no docs, onboarding, compatibility, or release-evidence surface
may imply they are part of the migration claim without a new governed row.

## Coverage Matrix

| Source row | v1.0 goal | Primary import targets | Quality bar | Caveat |
|---|---|---|---|---|
| `migration_source:vs_code_code_oss` | Preserve common daily text and workflow surfaces with minimal manual repair. | Settings, keybindings, snippets, tasks, launch configs, themes, selected compatible extensions, project roots. | `migration_quality:high_fidelity_common_text_workflow_surfaces` | Extension runtime parity, workbench layout parity, and extension storage import are non-goals. Compatible extension suggestions require trust and permission review. |
| `migration_source:jetbrains_family` | Transfer core navigation, editing, run/debug, and project habits without claiming plugin parity. | Keymaps, themes, selected run/debug configs, project roots, code-style hints, selected surface preferences. | `migration_quality:partial_but_polished_migration` and `migration_quality:medium_fidelity_keyboard_navigation_migration` | IDE indexes, caches, product-specific project models, and plugins are not imported as native truth. |
| `migration_source:vim_neovim` | Make modal editing feel credible enough for sustained daily use. | Modal-editing profiles, keymaps and leader maps, clipboard/search defaults, selected snippets, selected editor options. | `migration_quality:high_fidelity_editing_feel` | Arbitrary plugin state, Lua/Vimscript execution, registers, macro history, and terminal/session state are non-goals. |
| `migration_source:emacs` | Preserve keyboard navigation and command discovery for users switching from Emacs-shaped workflows. | Keymaps, command aliases, selected themes, project defaults, selected snippets/templates, optional modal-editing presets where configured. | `migration_quality:medium_fidelity_keyboard_navigation_migration` | Elisp package runtime, package state, arbitrary `init.el` semantics, buffers, and org-mode runtime parity are non-goals. |
| `migration_source:sublime_textmate` | Import high-value portable assets cleanly. | Themes/color schemes, snippets, selected syntax bundles, selected project roots, selected build/task hints where canonical equivalents exist. | `migration_quality:high_value_asset_import` | Package runtime parity, command/plugin execution, and full build-system parity are non-goals. |

## Quality-Bar Vocabulary

| Quality bar | Meaning | Required non-goal wording |
|---|---|---|
| `migration_quality:high_fidelity_common_text_workflow_surfaces` | Common settings, shortcuts, snippets, tasks, launches, themes, and compatible extension recommendations map to Aureline-native records or explicit review rows. | No runtime/plugin parity, no silent extension install, no hidden trust or egress widening. |
| `migration_quality:partial_but_polished_migration` | A narrower set of source concepts imports with high polish, but unsupported concepts remain visible with next steps. | Partial does not mean silent drop, heuristic parity, or aggregate-only score. |
| `migration_quality:high_fidelity_editing_feel` | Editing modes, motions, core operators, search, clipboard defaults, and high-frequency gestures stay close enough for daily keyboard work. | No arbitrary plugin execution, register/macro history import, or terminal/session-state parity. |
| `migration_quality:medium_fidelity_keyboard_navigation_migration` | Navigation, palette/command habits, and common keybindings are mapped where semantic equivalents exist, with conflicts visible. | No promise that every command, package, or composable editor runtime has an Aureline equivalent. |
| `migration_quality:high_value_asset_import` | Portable assets such as themes, snippets, and syntax bundles import with provenance, coverage reports, and rollback. | No package runtime parity or execution of source package code. |

The quality-bar rubric is authoritative for the exact evidence burden and
downgrade rules. The table above is only the human-readable summary.

## Shared Evidence Links

Every source row carries references that downstream surfaces can quote
without inventing parallel wording:

| Consumer | Required reference |
|---|---|
| Onboarding and task-success measurement | `docs/product/onboarding_measurement_plan.md#surface_migration_review` plus scenario refs in `artifacts/product/task_success_corpus_seed.yaml`. |
| Certified-archetype and compatibility scorecards | `artifacts/compat/reference_workspace_rows.yaml`, `artifacts/compat/qualification_matrix_seed.yaml`, `docs/migration/compatibility_scorecard_contract.md`, `schemas/migration/compatibility_scorecard.schema.json`, and `artifacts/migration/top_imported_workflow_rows.yaml`. |
| Migration center and support export | `docs/migration/migration_center_object_model.md`, `docs/migration/first_run_import_diff_and_rollback_contract.md`, `schemas/migration/migration_session.schema.json`, `schemas/migration/importer_outcome.schema.json`, `schemas/migration/import_plan.schema.json`, `schemas/migration/import_diff_preview.schema.json`, and `schemas/migration/import_rollback_checkpoint.schema.json`. |
| Docs and public-truth propagation | `artifacts/governance/public_truth_parity_matrix.yaml`, `artifacts/governance/claim_manifest_seed.yaml`, and the `migration_notes` publication channel. |
| Release evidence | `docs/release/compatibility_report_template.md`, `docs/release/certified_archetype_report_template.md`, and `docs/release/release_evidence_packet_template.md`. |

## Row Admission Rules

1. A public migration statement MUST cite one source row id and one quality
   bar id.
2. A source row may not be promoted by aggregate parity alone. Category
   scores for shortcuts, settings, tasks/launches, themes, extensions, and
   workspace metadata remain separate.
3. Unsupported and bridge-required outcomes are valid end states, not
   hidden footnotes.
4. Import may narrow behavior, but may never silently widen workspace
   trust, extension permissions, AI egress, network egress, managed
   entitlements, or destructive automation defaults.
5. Every applied migration path needs a dry-run diff, machine-readable
   outcome packet, shortcut digest when shortcuts are in scope, rollback
   checkpoint, docs/help link, and support/export ref.
6. Rows outside this matrix are not implied by nearby feature work or by a
   generic "other tools" phrase.
