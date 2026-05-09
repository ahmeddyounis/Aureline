# Early accessibility review: shell bridge groundwork (seed)

Purpose: capture the first accessibility-tree bridge review notes for the core
shell surfaces that ship early: Start Center, command palette search, embedded
docs/help boundary chrome, and terminal placeholder posture.

Primary sources:

- Contract: `docs/accessibility/accessibility_tree_contract.md`
- Shell bridge entrypoint: `docs/accessibility/m1_shell_bridge.md`
- Implementation: `crates/aureline-shell/src/a11y/shell_bridge.rs`
- Seed fixtures:
  - `fixtures/accessibility/m1_shell_cases/shell_start_center_placeholders.json`
  - `fixtures/accessibility/m1_shell_cases/shell_command_palette_overlay.json`

## Review goals (seed)

- Keyboard traversal exposes major regions and does not lose focus after opening
  or closing overlays.
- Screen-reader roles and names exist for Start Center actions and palette
  results (no icon-only or paint-only semantics).
- Degraded placeholder surfaces announce narrowed support-class posture rather
  than silently disappearing from the tree.
- Export-safe posture holds (no raw paths, raw terminal bytes, credentials, or
  user identifiers in the tree records).

## How to capture a live shell tree snapshot

Run the native shell with capture enabled:

```bash
AURELINE_CAPTURE_A11Y_TREE=1 cargo run -p aureline-shell --bin aureline_shell
```

Then inspect:

- `.logs/accessibility_trees/*.shell_accessibility_tree.json`

The capture is deduped across redraws; change focus or open/close the command
palette to force a new tree.

## Seeded surface expectations

### Start Center (no editor tabs)

Fixture: `fixtures/accessibility/m1_shell_cases/shell_start_center_placeholders.json`

Expected:

- `node.start_center.action_list` exposes a `list` with 5 `list_row` children.
- One row is `selected` and `focused` (keyboard target).
- Each row carries a stable command id in `relationships.source_anchor_refs`.

### Command palette overlay

Fixture: `fixtures/accessibility/m1_shell_cases/shell_command_palette_overlay.json`

Expected:

- `node.palette.searchbox` is the focus owner when the palette is open.
- Result rows remain `selected` without also being the focus owner unless focus
  moves out of the searchbox.
- File results are represented as redacted summary rows (no raw paths).

### Embedded docs/help boundary chrome

Expected:

- `node.embedded.docs_help.boundary_card` exists under the inspector zone with a
  stable label derived from the host-owned boundary card contract.

### Terminal placeholder posture

Expected:

- `node.panel.terminal.placeholder` is present under the bottom panel zone with:
  - `states.degraded: true`
  - `support_status.support_class: experimental`
  - `support_status.support_state: summary_only`
  - a support-state label in `states.state_labels`

## Known gaps / follow-ups

- Platform adapter integration (UIA / NSAccessibility / AT-SPI) is not part of
  this seed; current work produces contract-shaped snapshots and fixtures to
  anchor follow-on bridge work.
- Palette file-result redaction currently models file rows as summary-only list
  items; expand into richer file semantics only when export-safe naming and
  policy posture are in place.

