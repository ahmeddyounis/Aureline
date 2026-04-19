# VFS decision-example fixtures

These fixtures are short, reviewable scenarios that anchor the
filesystem-identity layers, the save-mode taxonomy, the root-
capability envelope, the watcher-source and watcher-health taxonomy,
the cache-identity rules, and the protected-hot-path hook names
frozen in
[ADR 0006](../../../docs/adr/0006-vfs-save-cache-identity.md)
to concrete inputs and observable outcomes. They are not a test
suite; they are the vocabulary the VFS, buffer, editor, search,
graph, review, AI-apply, mutation-journal, support-export, and
benchmark lanes use when they instrument a hook or a code path.

**Scope rules**

- Every fixture names the envelope fields or hooks it exercises,
  the surface it stresses, and the observable outcome instrumentation
  should capture.
- Fixtures never assert latency numbers; the benchmark lab owns
  budgets. Fixtures only describe *what* to measure, not *how fast*.
- Fixtures describe the logical record contents as JSON that
  validates against
  [`/schemas/runtime/vfs_save_envelope.schema.json`](../../../schemas/runtime/vfs_save_envelope.schema.json);
  they do not encode wire bytes or the enclosing ADR-0004 event
  envelope.
- A new fixture MUST exercise at least one protected-hot-path hook
  or one frozen capability / save-mode / identity field and MUST
  cite the ADR section that motivates it.

**Index**

| Fixture                                               | Primary hooks / fields                                                                                          | Surface stressed                                                              |
|-------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------|
| [`local_atomic_save.json`](./local_atomic_save.json)  | `vfs_save_compare_before_write`, `vfs_save_atomic_commit`, `save_mode: atomic_replace`                          | Happy-path save on a local POSIX-like root                                    |
| [`external_change_conflict.json`](./external_change_conflict.json) | `vfs_save_compare_before_write`, `vfs_save_conflict`, `outcome: external_change_detected`            | Compare-before-write detects on-disk drift; pipeline refuses blind overwrite  |
| [`remote_conditional_conflict.json`](./remote_conditional_conflict.json) | `vfs_save_remote_conditional_commit`, `vfs_save_conflict`, `save_mode: conditional_remote_write` | Remote revision-token precondition fails; typed conflict raised               |
| [`case_only_rename_preview.json`](./case_only_rename_preview.json) | `vfs_rename_plan_previewed`, `supports_case_only_rename: true`, `case_sensitivity: insensitive_preserving` | Case-only rename on a case-insensitive root requires an explicit preview plan |
| [`alias_convergence.json`](./alias_convergence.json)  | `vfs_alias_converge`, `vfs_canonicalize`, `alias_kind: symlink`                                                 | Two presentation paths resolve to one canonical object; dirty buffer converges |
| [`symlink_escape_blocked.json`](./symlink_escape_blocked.json) | `vfs_save_blocked`, `symlink_escape_policy: block`, `outcome: read_only_or_policy_blocked`              | Symlink target outside the trusted root blocks the save                       |
| [`read_only_root_blocked.json`](./read_only_root_blocked.json) | `vfs_save_blocked`, `read_only: true`, `outcome: read_only_or_policy_blocked`                           | Save against a read-only archive view is blocked; surface offers save_as only |
| [`watcher_fallback_polling.json`](./watcher_fallback_polling.json) | `vfs_watcher_health_changed`, `watcher_health: fallback_polling`                                    | OS-native watcher overflowed; VFS falls back to polling; health state visible |
| [`generated_write_blocked.json`](./generated_write_blocked.json) | `vfs_save_blocked`, `outcome: generated_or_managed_write_blocked`, `drift_state: generator_changed`   | Direct edit to a generated artifact blocked by artifact-class policy          |
| [`durable_cache_supersession.json`](./durable_cache_supersession.json) | `vfs_cache_invalidate`, `vfs_cache_rebuild`, `durability_class: durable`                          | Durable cache fails supersession check on restart; rebuild begins             |
