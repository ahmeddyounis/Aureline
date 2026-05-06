# Filesystem-identity and semantic-readiness fixtures

These fixtures are short, reviewable scenarios that anchor the
cross-surface vocabulary frozen in
[`/docs/filesystem/filesystem_identity_vocabulary.md`](../../docs/filesystem/filesystem_identity_vocabulary.md)
and validated by the schema at
[`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json).

They are intentionally complementary to the VFS decision-example
fixtures under
[`/fixtures/runtime/vfs_decision_examples/`](../runtime/vfs_decision_examples/),
which exercise the ADR-0006 save / watcher / capability pipeline.
The fixtures here cover the cross-surface reading of the same
objects: the names each surface uses, the alias disclosure each
renders, and the semantic-readiness chip a non-VFS surface shows
over a partially-ready workspace.

**Scope rules**

- Every fixture names the vocabulary record kinds or semantic-
  readiness states it exercises and the worked-example section of
  the vocabulary document it motivates.
- Fixtures validate against
  `/schemas/filesystem/save_target_token.schema.json`; they do not
  encode wire bytes or the ADR-0005 subscription envelope.
- A new fixture MUST exercise at least one frozen identity layer,
  one semantic-readiness state, or one projection field, and MUST
  cite the vocabulary section that motivates it.

**Index**

| Fixture                                                                | Exercises                                                                                   | Vocabulary section |
|------------------------------------------------------------------------|---------------------------------------------------------------------------------------------|--------------------|
| [`path_case_change.json`](./path_case_change.json)                     | Case-only rename on a case-insensitive root; `alias_kind: case_only_variant`                | §4.1               |
| [`symlink_alias.json`](./symlink_alias.json)                           | Symlink / junction alias; two presentation paths, one canonical object                      | §4.2               |
| [`moved_file.json`](./moved_file.json)                                 | Rename that preserves the canonical identity token                                           | §4.3               |
| [`partially_ready_workspace.json`](./partially_ready_workspace.json)   | Composite workspace-scoped readiness with child producer records                             | §4.4               |
| [`semantic_readiness_cases/`](./semantic_readiness_cases/)             | Readiness projection cases and why-not-ready inspector view contract                         | §2–§3              |
