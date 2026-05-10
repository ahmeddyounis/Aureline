# Explorer file-tree fixture corpus

These fixtures drive the protected dogfood walk for the virtualized file-tree
model with stable node ids and explorer actions, owned by the `explorer`
module under `crates/aureline-shell/src/explorer/`. Each case pairs a small,
synthetic workspace shape with a canonical sequence of explorer actions and
the expected viewport snapshots after each step.

## Schema

Each `*.json` file under this directory follows the shape:

```json
{
  "schema_version": 1,
  "case_id": "fixtures.explorer.file_tree.<short_id>",
  "title": "Plain-language description of the protected walk this case proves.",
  "workspace": {
    "workspace_id": "wksp:<id>",
    "root": {
      "root_id": "root:<id>",
      "root_kind": "local_repo_root | local_folder | virtual_document_root | ...",
      "presentation_label": "<label shown on the root mount row>",
      "partial_truth": "loaded | manifest_known | cached | unavailable"
    },
    "extra_nodes": [
      {
        "logical_path": "src/main.rs",
        "kind": "directory | file | generated_artifact | virtual_document | special_file",
        "display_label": "main.rs",
        "readiness": "loaded | partially_enumerated | manifest_known | cached | unavailable",
        "generated_artifact_hint": null,
        "special_file_hint": null
      }
    ]
  },
  "actions": [ <ExplorerAction JSON, see actions.rs> ],
  "expected_viewport": {
    "filter_query": null,
    "selection_node_id": "node:<...>",
    "rows": [
      { "node_id": "node:<...>", "depth": 0, "is_expanded": true, "matches_filter": true, "kind": "root_mount" }
    ]
  },
  "expected_records": [
    {
      "action_class": "expand",
      "command_id": "cmd:workspace.explorer_expand_node",
      "outcome": "applied"
    }
  ]
}
```

## Cases

| File | Walk | Failure drill |
| --- | --- | --- |
| `small_project_open.json` | open a small project, expand a directory, open a file | none — proves the nominal protected walk |
| `generated_artifact_hint.json` | open a generated document and confirm provenance hint surfaces | none — proves the lineage cue stays attached to the row |
| `filter_churn_stable_ids.json` | rapid filter / expand / clear cycle | proves node ids stay stable across virtualization churn |
| `partially_ready_root.json` | open a workspace whose root is `manifest_known` only | proves degraded readiness label remains honest end-to-end |
