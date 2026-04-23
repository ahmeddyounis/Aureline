# Workset artifact, scope-truth chip, and scope-widen diff fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/workspace/scope_truth_packet.md`](../../../docs/workspace/scope_truth_packet.md)
and validated by the schema at
[`/schemas/workspace/workset_artifact.schema.json`](../../../schemas/workspace/workset_artifact.schema.json).

Each fixture names the record kind it exercises, the scope class /
membership policy / source class / portability class / readiness
state / narrowing cause / diff class it covers, and the worked-
example section of the scope-truth packet it motivates.

**Scope rules**

- Fixtures validate against the single workset-artifact schema;
  they do not encode wire bytes or runtime execution-context
  envelopes.
- A new fixture MUST exercise at least one scope class, one
  membership policy, one source class, one portability class,
  one readiness state, one narrowing cause, or one diff class,
  and MUST cite the section of the scope-truth packet that
  motivates it.
- Monotonic timestamps and stable ids are opaque; they are chosen
  to read well rather than to reflect any real clock or system
  state.
- Filesystem-root refs are opaque pointers to ADR-0006
  filesystem-identity records. Raw absolute paths never appear.
- Include/exclude patterns are user-authored and are carried
  verbatim. Workspace-relative form is used throughout; absolute
  paths do not cross the schema boundary.
- Policy bodies, credential material, and the exact hidden-member
  list of an admin-policy narrowing never appear in this
  directory.

**Index**

| Fixture                                                                              | Record kind                    | Key classes exercised                                                                                                                  | Doc section |
|--------------------------------------------------------------------------------------|--------------------------------|----------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [`current_repo_fallback.json`](./current_repo_fallback.json)                         | `workset_artifact_record`      | `current_repo` / `explicit_root_list` / `local_only` / `portable_with_rebinding` / `ready` / empty `patterns`                         | §1.2, §4.1  |
| [`selected_workset_multi_root.json`](./selected_workset_multi_root.json)             | `workset_artifact_record`      | `selected_workset` / `explicit_root_list` / `workspace_shared` / `portable_with_rebinding` / `warm` / multi-root (3 roots)            | §1.2, §1.4, §5 |
| [`sparse_slice_pattern_narrowed.json`](./sparse_slice_pattern_narrowed.json)         | `workset_artifact_record`      | `sparse_slice` / `glob_pattern` / `local_only` / `portable_with_rebinding` / `partial` / exclude patterns + mixed `partial_truth`     | §1.2, §1.3, §1.5 |
| [`full_workspace_multi_root.json`](./full_workspace_multi_root.json)                 | `workset_artifact_record`      | `full_workspace` / `explicit_root_list` / `workspace_shared` / `portable_with_rebinding` / `warm` / 7 roots                            | §1.2, §5    |
| [`policy_limited_admin_hidden.json`](./policy_limited_admin_hidden.json)             | `workset_artifact_record`      | `policy_limited_view` / `admin_policy` / `managed` / `managed_provider_locked` / hidden list not visible                              | §1.2, §1.6, §2.4 |
| [`scope_widen_diff_selected_to_full.json`](./scope_widen_diff_selected_to_full.json) | `scope_widen_diff_record`      | `member_added` (x4) / `pattern_broadened` / `readiness_changed` / `widens_scope` / `targeted_index` / `remote_fetch_required = false` | §3, §4.2    |

**Coverage contract**

This fixture set MUST keep at least one artifact for each of the
five scope classes (`current_repo`, `selected_workset`,
`sparse_slice`, `full_workspace`, `policy_limited_view`) and at
least one `scope_widen_diff_record`. Adding a fixture for a new
narrowing cause, portability class, or diff class is welcome;
removing a scope class this directory already covers is a
breaking change.
