# Workset-scope beta truth fixtures

Each fixture pairs a `workset_artifact_record` (the alpha artifact) with the
beta scope-truth one or more consumer surfaces project against it, and the
support-export packet a triage flow would replay. The fixtures cover every
admission outcome in the closed `broad_action_decision` vocabulary plus the
outside-current-scope marker.

Run the lane via:

```
cargo test -p aureline-workspace --test workset_scope_beta
```

## Cases

| File | Scenario |
| --- | --- |
| `full_workspace_authoritative.json` | Two-root, ready full workspace; every broad action is allowed and the lineage stays a single entry. |
| `sparse_slice_narrowed.json` | Pattern-narrowed front-end slice; refactor / AI / export are narrowed to the active scope, support archive narrows for lineage preservation. |
| `policy_limited_admin_hidden.json` | Admin-policy narrowed view over a managed-provider-locked workset; refactor / AI block on policy, export blocks on portability, support archive narrows. |
| `outside_current_scope_search_row.json` | Quick-open jumped into a sibling repo not in the active workset; broad actions block with `blocked_by_outside_scope` and the truth carries an explain note. |
