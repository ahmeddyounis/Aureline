# Top-incumbent flow matrix (beta)

This page is the narrative companion to the M3 migration scoreboard.
The scoreboard data and the per-row classifications come from one
mint-from-truth path -- the seeded corpus in
[`crate::migration_corpus`](../../../crates/aureline-shell/src/migration_corpus/mod.rs)
-- so the live migration center, the docs scoreboard, and the
support-export wrapper never disagree on what beta promises.

Authoritative artifacts:

- [`/artifacts/migration/m3/migration_scoreboard.md`](../../../artifacts/migration/m3/migration_scoreboard.md)
  -- markdown scoreboard generated from the seeded corpus.
- [`/fixtures/migration/m3/incumbent_flows/scoreboard.json`](../../../fixtures/migration/m3/incumbent_flows/scoreboard.json)
  -- JSON snapshot of the same scoreboard record consumed by every
  surface.
- [`/fixtures/migration/m3/migration_wizard/`](../../../fixtures/migration/m3/migration_wizard/)
  -- the beta migration-wizard projection the scoreboard composes
  with. Every row quotes the wizard `mapping_report_id` and
  `rollback_checkpoint_ref` so reviewers can pivot between the
  wizard, the scoreboard, and the support export.

## What the scoreboard promises

The scoreboard covers the four named incumbent ecosystems Aureline
claims switching support for during beta:

| Ecosystem | Source row | Section |
| --------- | ---------- | ------- |
| VS Code / Code-OSS | [`migration_source:vs_code_code_oss`](../source_ecosystem_coverage_matrix.md) | [VS Code flows](#vscode) |
| JetBrains IDEs | [`migration_source:jetbrains_family`](../source_ecosystem_coverage_matrix.md) | [JetBrains flows](#jetbrains) |
| Vim / Neovim | [`migration_source:vim_neovim`](../source_ecosystem_coverage_matrix.md) | [Vim flows](#vim) |
| Emacs | [`migration_source:emacs`](../source_ecosystem_coverage_matrix.md) | [Emacs flows](#emacs) |

Each section names at least one flow row in every required mapping
class so a switching user always sees the full Exact / Translated /
Partial / Shimmed / Unsupported spread for their ecosystem:

- **Exact** -- semantics map directly with no caveat retained.
- **Translated** -- semantics map through a declared Aureline command
  or setting id. Translation depends on stable resolver layers.
- **Partial** -- a subset applies; the caveat is retained in the
  migration report and the wizard's shortcut delta digest.
- **Shimmed** -- continuity depends on a governed shim (a token
  mapping, a workspace-manifest bridge, or a capability layer). The
  shim is named, not hidden.
- **Unsupported** -- no safe Aureline target exists. Apply is denied
  for the source object and the gap stays visible in the wizard's
  pre-apply gap list and the retained report.

Every non-`Exact` row carries an explicit list of `downgrade_triggers`
so a publication, support export, or claim manifest can automatically
downgrade the row when:

- a quoted Aureline command id or setting id is renamed;
- a referenced source profile fixture or scorecard rotates;
- a permission, runtime, or extension-host policy changes;
- a post-import validation state changes;
- the keybinding resolver layer or workspace manifest schema changes.

The scoreboard validator (`validate_migration_scoreboard`) rejects:

1. a missing required ecosystem (VS Code, JetBrains, Vim, or Emacs);
2. an empty ecosystem section;
3. a scoreboard that does not cover all five required classifications;
4. a non-`Exact` row with no downgrade trigger;
5. a `Partial` / `Shimmed` / `Unsupported` row with no caveat;
6. a row with no evidence ref, no docs/help ref, or no wizard mapping
   report ref;
7. a missing scoreboard or matrix publication ref.

## How surfaces consume the scoreboard

- **In-product migration center.** The shell consumes
  [`MigrationScoreboard`](../../../crates/aureline-shell/src/migration_corpus/mod.rs)
  records so the migration center renders the same per-ecosystem
  table seen in the docs scoreboard and the support export.
- **CLI / headless inspector.** The
  `aureline_shell_migration_corpus` binary is the only mint-from-truth
  path for `fixtures/migration/m3/incumbent_flows/`. It also renders
  the markdown scoreboard, so the doc artifact stays bit-for-bit
  equal to the seeded record.
- **Support / cohort export.** `MigrationCorpusSupportExport` quotes
  the scoreboard plus every stable `flow_id` so cohort packets and
  support evidence reviewers pivot from a single set of case ids.
- **Docs and help.** Each row carries a `docs_help_refs` list. This
  page and the per-ecosystem sections below are the docs landing
  point; the source ecosystem coverage matrix is the broader catalog.

## Required headless commands

```sh
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- scoreboard
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- vscode
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- jetbrains
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- vim
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- emacs
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- scoreboard-md
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- compact
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- validate
```

`validate` exits non-zero (status `3`) if any acceptance invariant is
violated; CI fails closed on a regression in any of the named
incumbent flows.

## VS Code / Code-OSS flows {#vscode}

VS Code flows include common settings, command-palette shortcuts,
high-frequency keymap chords, native extension replacement (ESLint),
and an explicitly unsupported webview extension runtime. The full
flow table lives in
[`/artifacts/migration/m3/migration_scoreboard.md`](../../../artifacts/migration/m3/migration_scoreboard.md).

Notable downgrade triggers:

- `aureline_command_id_renamed` and `vscode_command_id_renamed`
  narrow the command-palette and shortcut rows.
- `shortcut_delta_digest_changed` and `platform_reserved_chord_changed`
  narrow the high-frequency keymap chord row to a manual-review row.
- `webview_governance_contract_changed` keeps the unsupported webview
  row blocked.

## JetBrains IDEs flows {#jetbrains}

JetBrains flows cover the common keymap preset, run/debug
configuration with manual review, formatter and code-style hints,
project root and module content-root shims, and the unsupported
source IDE plugin runtime.

Notable downgrade triggers:

- `aureline_command_id_renamed` and `jetbrains_action_id_renamed`
  narrow the keymap preset row.
- `post_import_validation_state_changed` keeps the run/debug row
  partial until a reviewer accepts the execution context.
- `workspace_manifest_schema_changed` narrows the project-root shim.

## Vim / Neovim flows {#vim}

Vim flows cover the modal-editing profile (normal/visual/operator),
leader-key mappings, selected snippet directories, clipboard/search
defaults shim, and the unsupported Lua plugin runtime.

Notable downgrade triggers:

- `leader_overlay_schema_changed` narrows the leader-key row.
- `snippet_engine_compat_changed` keeps the snippet row partial.
- `modal_profile_shim_changed` and
  `clipboard_capability_layer_changed` narrow the defaults shim.
- `lua_runtime_policy_changed` keeps the plugin runtime row blocked.

## Emacs flows {#emacs}

Emacs flows cover the global keymap and command aliases, project
defaults with manual review, the theme token-mapping shim, and the
unsupported Elisp package runtime.

Notable downgrade triggers:

- `emacs_command_alias_changed` narrows the global keymap row.
- `post_import_validation_state_changed` keeps the project defaults
  row partial until a reviewer accepts the import.
- `theme_token_schema_changed` and
  `design_token_vocabulary_changed` narrow the theme shim.
- `elisp_runtime_policy_changed` keeps the package runtime row
  blocked.

## Acceptance posture

The scoreboard delivers the M3 incumbent-flow acceptance gates:

- **The corpus covers the named incumbent flows and records current
  Exact / Translated / Partial / Shimmed / Unsupported results.** The
  validator rejects a missing ecosystem, an empty section, or a
  missing required classification.
- **The scoreboard is current enough to downgrade overclaimed paths
  automatically.** Every non-`Exact` row carries `downgrade_triggers`
  the validator enforces. A consumer that hits a trigger must
  renarrow the row before the next publication.
- **Docs, cohort packets, and in-product migration UI consume the
  same mapping data.** The scoreboard JSON, the markdown scoreboard,
  this matrix, and the support-export wrapper are produced by the
  same seeded record and quote the same `wizard_mapping_report_ref`,
  `rollback_checkpoint_ref`, and `flow_id` values.
