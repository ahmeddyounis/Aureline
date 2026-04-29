# Accessibility Tree Node Cases

Seed fixtures for the accessibility-tree node taxonomy and inspector
snapshot contract.

| Fixture | Purpose |
|---|---|
| `shell_editor_diagnostic_inline_widget.json` | Shell, editor line, gutter, diagnostic, inline action, and live-region relationships. |
| `virtualized_collection_row_index_truth.json` | List, tree, table, hidden selected rows, and exact virtualized index truth. |
| `log_notebook_status_downgrade.json` | Terminal/log, notebook output summary, status notice, and explicit experimental downgrade language. |
| `inspector_snapshot_virtualized_collection.json` | Support/QE snapshot joining tree projections, focus chain, selection, row-index truth, and announcement contract state. |

The JSON files intentionally carry only opaque refs, bounded labels,
counts, state classes, and schema refs. Raw source text, paths,
terminal bytes, prompts, credentials, and user identifiers are excluded.
