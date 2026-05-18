# Navigation Target Fidelity Report

Record kind: `navigation_target_fidelity_report_record`  
Schema: `schemas/navigation/navigation_target.schema.json`  
Contract: `docs/navigation/m3/navigation_target_beta_contract.md`  
Corpus: `fixtures/navigation/m3/target_accuracy`

Status: `needs_review`

The checked-in corpus validates without blocking violations. The aggregate
state is `needs_review` because several rows intentionally exercise partial
scope, generated boundaries, framework/runtime-only proof, and drifted
continuity; those rows are acceptable only while their downgrade labels remain
visible.

| Case | Lane | Targets | Refs | Edges | Rename | Disambig | Continuity | Surfaces | State |
|---|---|---:|---:|---:|---:|---:|---:|---|---|
| `nav-target:cross-language-definition-reference` | `tsjs_python_beta` | 3 | 6 | 0 | 0 | 0 | 0 | `editor_ui`, `cli_headless` | `promotable` |
| `nav-target:hierarchy-framework-runtime-edges` | `graph_framework_runtime_beta` | 1 | 0 | 3 | 0 | 1 | 0 | `graph_overlay` | `needs_review` |
| `nav-target:generated-boundary-disambiguation` | `generated_framework_beta` | 3 | 1 | 0 | 0 | 1 | 0 | `ai_context` | `needs_review` |
| `nav-target:drifted-bookmark-breadcrumb` | `shell_continuity_beta` | 4 | 0 | 0 | 0 | 1 | 3 | `shell_continuity` | `needs_review` |
| `nav-target:rename-conflicts-partial-scope` | `rename_preview_beta` | 1 | 2 | 0 | 1 | 0 | 0 | `review_workspace` | `needs_review` |
| `nav-target:export-evidence-parity` | `support_export_beta` | 1 | 1 | 0 | 0 | 0 | 0 | `support_export` | `needs_review` |

Validation coverage:

- Relation kinds covered: `definition`, `declaration`, `implementation`,
  `reference`, `type`, `call`, `route-binding`, `owner-link`, `doc-link`.
- Consumer surfaces covered: `editor_ui`, `cli_headless`, `ai_context`,
  `review_workspace`, `support_export`, `graph_overlay`, `shell_continuity`.
- Guardrails covered: fallback/downgrade labels, generated boundary notes,
  runtime/framework proof separation, ambiguous disambiguation sets, bookmark
  drift review, and metadata-only support export.

Verification command:

```sh
cargo test -p aureline-navigation
```
