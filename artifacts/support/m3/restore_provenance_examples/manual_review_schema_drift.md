# Restore packet — Manual review (import, schema equivalence missing)

Drawn from corpus drill `manual_review.schema_drift_preserves_prior`.

| Field | Value |
| --- | --- |
| Source event | `import` |
| Producer build | `producer:aureline:0.0.0-dev-skewed` |
| Schema outcome | `manual_review` |
| Fidelity result | **Compatible restore** (pending review) |
| Prior artifact | **required** — compare and export refs present |
| Visible in | diagnostics, support export, crash recovery |

## Missing dependencies (reopened as placeholders)

| Preserved pane | Dependency | Last-known provenance | Safe actions |
| --- | --- | --- | --- |
| `pane-authority-0001` | `schema_equivalence_missing` | workspace authority body (older schema) | compare, export evidence, manual review |

## Named exclusions (what was deliberately left out)

- secret material
- delegated approval
- live authority handle
- machine-unique trust anchor

## Reviewer reading

The imported authority body used an older schema with no equivalence map, so
import could not prove meaning was preserved. The outcome is **Manual review**:
nothing was applied as authority, the pane stays as an evidence placeholder, and
the prior artifact stays available for compare and export so a human can decide.
This is exactly the path that prevents an older package from silently widening
meaning.
