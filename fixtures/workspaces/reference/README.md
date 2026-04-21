# Reference-workspace seeds

These fixtures describe small, reviewable **workspace shapes** that
the benchmark lab, entry-restore measurement, archetype-detection
scoreboarding, and VFS enumeration work instrument against. They are
the shared "what was opened" inputs to the corpus manifest at
[`/fixtures/benchmarks/corpus_manifest.yaml`](../../benchmarks/corpus_manifest.yaml).

Each fixture is a JSON document that:

- declares a stable `reference_workspace_id` (the corpus manifest and
  the task-success corpus seed cite it verbatim);
- declares a `resolution_mode` — either `described_bytes` (the
  harness synthesises the named files from the fixture at run time)
  or `live_repo_slice` (the harness resolves an include / exclude
  pattern list against the live repository);
- declares the workspace shape: roots, entries, byte counts,
  encodings, line endings;
- declares expected discovery state: archetype-detection outcome,
  semantic-readiness state, language tags, presence of package
  manifest / lockfile / DVCS metadata;
- names which corpus scenarios use it (so the scenario and the
  fixture identity cannot drift);
- carries the metadata every corpus fixture needs: `size_class`,
  `visibility_class`, `retention_class`, `license_status`,
  `toolchain_assumption`, `host_platform_class`, and any
  `degraded_notes`.

## Scope rules

- Reference workspaces describe shape, not project bytes. Benchmarks
  that require a real dependency tree, virtual environment, node_modules,
  or target directory populate it out of band and record the populate
  step in the benchmark-report packet. The fixture is still the
  stable identity of the opened workspace.
- A reference workspace MUST carry an `archetype_hint` from the
  closed archetype vocabulary in the corpus-manifest companion
  document ([`/docs/benchmarks/fixture_classes.md`](../../../docs/benchmarks/fixture_classes.md)).
- A reference workspace MUST NOT vendor third-party source files.
  Either the bytes are `synthetic_no_real_content` or the fixture
  points at the live repo via `live_repo_slice`.
- Monotonic timestamps and stable IDs are opaque; reference
  workspaces do not encode any real clock or filesystem state.
- Any fixture that would require extra privacy, export, or retention
  review before broader CI use (for example: a real customer
  repository sample) MUST NOT land here. The corpus manifest's
  `visibility_class: restricted` slot is reserved for those; they
  live elsewhere or not at all.

## Index

| Fixture                                                                 | Reference workspace ID                          | Archetype hint             | Size class | Resolution mode     |
|-------------------------------------------------------------------------|-------------------------------------------------|----------------------------|------------|---------------------|
| [`micro_local_folder.json`](./micro_local_folder.json)                 | `refws.micro_local_folder`                      | `misc_folder`              | `micro`    | `described_bytes`   |
| [`small_rust_self_host_slice.json`](./small_rust_self_host_slice.json) | `refws.small_rust_self_host_slice`              | `rust_workspace_self_host` | `small`    | `live_repo_slice`   |
| [`ts_web_app_archetype_seed.json`](./ts_web_app_archetype_seed.json)   | `refws.ts_web_app_archetype_seed`               | `ts_web_app`               | `tiny`     | `described_bytes`   |
| [`python_data_app_archetype_seed.json`](./python_data_app_archetype_seed.json) | `refws.python_data_app_archetype_seed`   | `python_data_app`          | `tiny`     | `described_bytes`   |
| [`misc_folder_unknown_archetype.json`](./misc_folder_unknown_archetype.json) | `refws.misc_folder_unknown_archetype`      | `misc_folder`              | `micro`    | `described_bytes`   |
| [`partially_ready_restore_seed.json`](./partially_ready_restore_seed.json)   | `refws.partially_ready_restore_seed`       | `misc_folder`              | `tiny`     | `described_bytes`   |

## Relationship to other fixture families

- The **task-success corpus seed** at
  [`/artifacts/product/task_success_corpus_seed.yaml`](../../../artifacts/product/task_success_corpus_seed.yaml)
  reserves `fixture.archetype_*` ids. The reference-workspace seeds
  under this directory materialise three of those reserved slots
  (TS web app, Python data app, misc folder); the reservation ids
  are carried as `task_success_corpus_ref` on the relevant fixtures
  so the two corpora stay on one identity.
- The **entry-restore fixtures** at
  [`/fixtures/workspace/entry_restore_examples/`](../../workspace/entry_restore_examples/)
  are `project_entry_action_record` / `restore_prompt_record` /
  `migration_result_record` fixtures. They record *what happened*
  on open. Reference-workspace seeds record *what was opened*;
  the two stacks compose, they do not overlap.
- The **filesystem-identity fixtures** at
  [`/fixtures/filesystem/`](../../filesystem/) record readiness and
  alias semantics on individual objects. Reference workspaces name
  the roots; readiness records attach per-root.
- The **large-file fixtures** at
  [`/fixtures/text/large/`](../../text/large/) are byte-materialised
  individual files, not workspaces. They are referenced from the
  corpus manifest under the `large_file_trigger` corpus class.
