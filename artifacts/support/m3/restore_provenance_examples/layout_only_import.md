# Restore packet — Layout only (import, missing extension and remote)

Drawn from corpus drill `layout_only.import_missing_extension_and_remote`.

| Field | Value |
| --- | --- |
| Source event | `import` |
| Producer build | `producer:aureline:0.0.0-dev` |
| Schema outcome | `layout_only` |
| Fidelity result | **Layout only** |
| Prior artifact | available for compare and export |
| Visible in | diagnostics, support export, crash recovery |

## Missing dependencies (reopened as placeholders)

| Preserved pane | Dependency | Last-known provenance | Safe actions |
| --- | --- | --- | --- |
| `pane-preview-ext-0001` | `missing_extension` | service topology preview extension | install extension, export evidence, remove pane |
| `pane-notebook-0001` | `missing_remote` | remote notebook kernel | reconnect, reauthenticate, export evidence |

## Named exclusions (what was deliberately left out)

- secret material
- delegated approval
- live authority handle
- machine-unique trust anchor

## Reviewer reading

The layout restored, but two surfaces could not hydrate live, so the restore is
**Layout only**, not Exact. Both panes kept their stable ids and reopened as
placeholders with recovery actions; nothing was rerun and no live authority was
reacquired. Excluded secrets, approvals, and live authority are named, not
silently dropped.
