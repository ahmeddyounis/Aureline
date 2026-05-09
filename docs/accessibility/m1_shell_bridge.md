# Shell accessibility bridge groundwork (seed)

Status: seeded

This document explains the initial shell accessibility bridge that maps core
desktop shell surfaces into the shared accessibility-tree contract. The goal is
to make shell semantics reviewable early (roles, names, focus exposure, and
degraded-state posture) without scraping paint output.

Companion artifacts:

- Contract: `docs/accessibility/accessibility_tree_contract.md`
- Implementation: `crates/aureline-shell/src/a11y/shell_bridge.rs`
- Contract records: `crates/aureline-shell/src/a11y/tree_contract.rs`
- Seed fixtures:
  - `fixtures/accessibility/m1_shell_cases/shell_start_center_placeholders.json`
  - `fixtures/accessibility/m1_shell_cases/shell_command_palette_overlay.json`
- Early review packet: `artifacts/accessibility/m1/early_accessibility_review.md`

## What is mapped

The bridge emits `accessibility_tree_node_record` entries for:

- canonical shell zones (title/context bar, activity rail, sidebars, main
  workspace, inspector, bottom panel, status bar, transient overlay);
- Start Center action list when the focused editor group is empty;
- command palette dialog + query input + grouped results when the palette is
  open;
- embedded docs/help boundary chrome (host-owned, non-embedded semantics);
- terminal placeholder posture (explicit degraded/support-class state).

## How to capture live shell snapshots

Set `AURELINE_CAPTURE_A11Y_TREE=1` before launching the native shell. When
enabled, the shell writes export-safe JSON snapshots under
`.logs/accessibility_trees/` and dedupes identical trees across redraws.

Example:

```bash
AURELINE_CAPTURE_A11Y_TREE=1 cargo run -p aureline-shell --bin aureline_shell
```

Output directory:

- `.logs/accessibility_trees/*.shell_accessibility_tree.json`

The snapshot payload contains:

- `root_node_id` and `tree_epoch_ref` for stable joins across captures
- `nodes[]` as contract-shaped `accessibility_tree_node_record` objects

## Node id conventions (current)

The bridge currently uses stable, human-readable ids:

- Root: `node.app.root`
- Zones: `node.shell.zone.<zone_name>`
- Start Center:
  - `node.start_center.region`
  - `node.start_center.action_list`
  - `node.start_center.action.<1-based index>`
- Command palette:
  - `node.palette.dialog`
  - `node.palette.searchbox`
  - `node.palette.results`
  - command rows: `node.palette.result.command.<sanitized command id>`
  - file rows: `node.palette.result.file.redacted.<row index>`
- Embedded docs/help boundary: `node.embedded.docs_help.boundary_card`
- Terminal placeholder: `node.panel.terminal.placeholder`

## Privacy posture

The bridge is intended to be safe for export and review:

- command ids are allowed as stable source anchors (e.g. `cmd:workspace.open_folder`)
- file results are represented as redacted summary rows (no raw paths)
- terminal placeholders are represented as degraded summary nodes (no raw bytes)

Any future expansion that risks exposing raw paths, raw terminal output, secrets,
or user identifiers should be gated behind explicit redaction classes and
support/export contracts.

