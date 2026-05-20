# Restore packet — Schema migration (alpha → beta, Layout only)

Drawn from corpus drill `migration.alpha_to_beta_layout_only`.

| Field | Value |
| --- | --- |
| Source event | `import` |
| Producer build | `producer-aureline-dev-0001` (older alpha package) |
| Schema outcome | `compatible` |
| Fidelity result | **Layout only** |
| Prior artifact | available for compare and export |
| Visible in | diagnostics, support export, crash recovery |

## What the migration preserved

- the four state layers stayed separated (workspace authority, window topology,
  profile defaults, machine-local hints);
- machine-local hints stayed **excluded** from export;
- path and host redaction stayed available;
- the live-authority handle stayed a **named exclusion** — it was not
  rehydrated;
- the fidelity downgrade stayed visible (Layout only), not widened to Exact;
- the remembered-state inspector and the export / import review sheets still
  projected without error.

## Missing dependencies (reopened as placeholders)

| Preserved pane | Dependency |
| --- | --- |
| `pane-preview-ext-0001` | `missing_extension` |
| `pane-notebook-0001` | `missing_remote` |
| `pane-terminal-0001` | `non_reentrant_live_surface` |

## Named exclusions (what was deliberately left out)

- raw secret material
- approval / capability ticket
- delegated credential
- live authority handle
- machine-unique trust anchor
- state root, raw path, raw host, raw command line, raw log, raw source
  content, provider payload

## Reviewer reading

An older alpha portable-state package was migrated forward to the beta
boundary. The migration neither widened meaning, rehydrated live authority, nor
suppressed the downgrade — it is a faithful **Layout only** restore with the
prior artifact preserved for compare and export.
