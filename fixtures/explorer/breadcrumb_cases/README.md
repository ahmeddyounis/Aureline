# Explorer breadcrumb fixture corpus

These fixtures drive the protected dogfood walk for the file-tree filter,
reveal-current-file behavior, and the path-ancestry breadcrumb trail in the
editor chrome. They prove that a reveal in the explorer and the breadcrumb
trail in the editor both resolve to the same canonical
`(workspace_id, root_id, logical_uri)` tuple — across a filter cycle, a
deep nested reveal, and a degraded `manifest_known` root.

The fixtures share the same `workspace.extra_nodes` shape as
`fixtures/explorer/file_tree_cases/*.json`; the only differences are the
top-level `actions` shape (`apply_filter`, `reveal`) and the new
`expected_outcomes.breadcrumb_path` projection.

## Schema

```json
{
  "schema_version": 1,
  "case_id": "fixtures.explorer.breadcrumb.<short_id>",
  "title": "Plain-language description of the reveal walk this case proves.",
  "workspace": { /* same shape as file_tree_cases */ },
  "actions": [
    { "action": "apply_filter", "query": "payments" },
    { "action": "reveal", "logical_path": "src/payments/charge.rs" }
  ],
  "expected_outcomes": {
    "filter": { /* ExplorerFilterOutcome subset */ },
    "reveal": {
      "revealed_logical_path": "src/payments/charge.rs",
      "ancestry_logical_paths": ["", "src", "src/payments"],
      "matches_filter": true,
      "filter_query": "payments",
      "breadcrumb_path": {
        "workspace_id": "wksp:reveal-deep",
        "root_id": "root:src",
        "root_badge": "local",
        "leaf_logical_path": "src/payments/charge.rs",
        "segments": [
          { "logical_path": "", "display_label": "reveal-deep", "kind": "root_mount", "is_root": true, "is_leaf": false },
          { "logical_path": "src", "display_label": "src", "kind": "directory", "is_root": false, "is_leaf": false },
          { "logical_path": "src/payments", "display_label": "payments", "kind": "directory", "is_root": false, "is_leaf": false },
          { "logical_path": "src/payments/charge.rs", "display_label": "charge.rs", "kind": "file", "is_root": false, "is_leaf": true }
        ]
      }
    }
  }
}
```

`logical_path` is the relative path under the root mount; the empty string
refers to the root mount itself. The fixture loader expands these to the
canonical `aureline-ws://<workspace_id>/<root_id>/<logical_path>` URI before
asserting against the runtime projection.

## Cases

| File | Walk | Failure drill |
| --- | --- | --- |
| `reveal_deep_path.json` | reveal a three-level-deep file and confirm breadcrumbs match the canonical ancestry | none — proves the nominal reveal walk |
| `filter_then_reveal_keeps_identity.json` | apply a filter, then reveal a node whose label does not match | proves filter+reveal does not jump to a different node identity |
| `partially_ready_breadcrumb.json` | reveal a node under a `manifest_known` root | proves breadcrumbs stay honest under degraded root readiness |
