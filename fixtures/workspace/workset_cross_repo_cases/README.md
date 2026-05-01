# Workset switcher, scope banner, and cross-repo result-group fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/workspace/workset_scope_and_cross_repo_contract.md`](../../../docs/workspace/workset_scope_and_cross_repo_contract.md)
and are validated by the schemas at:

- [`/schemas/workspace/workset_switcher.schema.json`](../../../schemas/workspace/workset_switcher.schema.json)
- [`/schemas/workspace/cross_repo_result_group.schema.json`](../../../schemas/workspace/cross_repo_result_group.schema.json)

Each fixture names the record kind it exercises, the row class /
banner state / in-scope marker / result-state class / readiness
state / source class / narrowing cause it covers, and the worked
section of the scope-and-cross-repo contract it motivates.

**Scope rules**

- Fixtures validate against the schemas above; they do not
  encode wire bytes or runtime execution-context envelopes.
- A new fixture MUST exercise at least one switcher row class,
  banner state, in-scope marker, result-state class, hidden-row
  count class, or topology-reuse caveat, and MUST cite the
  section of the contract that motivates it.
- Monotonic timestamps and stable ids are opaque; they are
  chosen to read well rather than to reflect any real clock or
  system state.
- Filesystem-root refs are opaque pointers to ADR-0006
  filesystem-identity records. Raw absolute paths, raw remote
  URLs, raw policy bodies, raw query bodies, raw document
  bodies, and raw symbol definitions never appear.
- Workset ids are quoted by reference; no fixture mints a
  parallel stable scope id.

**Index**

| Fixture | Record kind | Key classes exercised | Doc section |
|---|---|---|---|
| [`multi_repo_workset_switcher.yaml`](./multi_repo_workset_switcher.yaml) | `workset_switcher_record` (with five nested rows) | `named_workset_row` / `current_repo_fallback_row` / `full_workspace_row` / `sparse_slice_row` / `ephemeral_session_row` ; `selected_workset` / `current_repo` / `full_workspace` / `sparse_slice` ; `warm` / `ready` / `partial` / `warming` ; `workspace_shared` / `local_only` / `ephemeral_session` ; multi-root (3 roots) | §1, §1.1, §1.2 |
| [`warm_vs_cold_scope_switch.yaml`](./warm_vs_cold_scope_switch.yaml) | `scope_banner_record` | `selected_workset` ; `active_warming` ; `warming_index` hidden-result count class ; `build_missing_indexes` / `widen_to_full_workspace` / `keep_current_scope` / `open_workset_switcher` / `reset_to_default_workset` actions ; `trusted_with_caveats` trust note | §2, §2.1, §2.3 |
| [`outside_current_scope_result_group.yaml`](./outside_current_scope_result_group.yaml) | `cross_repo_result_group_record` (with three nested rows) | `outside_current_scope` in-scope marker ; `outside_current_scope_disclosed` group state ; `outside_current_scope` / `imported_root` / `partially_loaded` row classes ; `imported_provider_index` / `lexical_local_index` lanes ; `topology_map` / `impact_explorer` / `cited_explainer` reuse with caveat preservation | §3, §3.1, §3.2, §5 |
| [`policy_limited_workset_banner.yaml`](./policy_limited_workset_banner.yaml) | `scope_banner_record` | `policy_limited_view` ; `active_policy_limited` ; `policy_hidden` count class ; `admin_policy` narrowing cause with `hidden_member_list_visible = false` ; `restricted_admin` trust note ; `managed`-aware action set without `export_workset_artifact` | §2, §2.1, §2.2, §2.3 |

**Coverage contract**

This fixture set MUST cover, at minimum: a multi-repo workset
(switcher record), a warm-vs-cold scope switch (banner record), an
outside-current-scope result group (cross-repo record), and a
policy-limited workset (banner record). Adding fixtures that
exercise additional row classes, banner states, in-scope markers,
result-state classes, or topology-reuse caveats is welcome;
removing a class this directory already covers is a breaking
change.
