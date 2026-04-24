# Archetype seed notes

One short notes file per archetype row in
[`/artifacts/compat/reference_workspace_rows.yaml`](../../../artifacts/compat/reference_workspace_rows.yaml).
The notes record the representative stack, why each required-mode
choice was made, and any open evidence questions the row will need to
answer before its target support class is admissible.

These files are reviewer-facing. They are not the rubric and not the
inventory — both of those live in
[`/artifacts/compat/archetype_rubric.yaml`](../../../artifacts/compat/archetype_rubric.yaml)
and
[`/artifacts/compat/reference_workspace_rows.yaml`](../../../artifacts/compat/reference_workspace_rows.yaml).
The narrative companion lives at
[`/docs/compat/reference_workspace_program_seed.md`](../../../docs/compat/reference_workspace_program_seed.md).

| Archetype row id | Notes file |
|---|---|
| `archetype_row:ts_web_app_or_service` | [`ts_web_app_or_service.md`](./ts_web_app_or_service.md) |
| `archetype_row:python_service_or_data_app` | [`python_service_or_data_app.md`](./python_service_or_data_app.md) |
| `archetype_row:java_or_kotlin_service` | [`java_or_kotlin_service.md`](./java_or_kotlin_service.md) |
| `archetype_row:rust_workspace` | [`rust_workspace.md`](./rust_workspace.md) |
| `archetype_row:go_service_or_monorepo_slice` | [`go_service_or_monorepo_slice.md`](./go_service_or_monorepo_slice.md) |
| `archetype_row:c_or_cpp_native_project` | [`c_or_cpp_native_project.md`](./c_or_cpp_native_project.md) |
| `archetype_row:dotnet_service_or_app` | [`dotnet_service_or_app.md`](./dotnet_service_or_app.md) |
| `archetype_row:notebook_first_data_workflow` | [`notebook_first_data_workflow.md`](./notebook_first_data_workflow.md) |
| `archetype_row:misc_local_folder_no_archetype` | [`misc_local_folder_no_archetype.md`](./misc_local_folder_no_archetype.md) |

Each file uses the same five sections so reviewers can scan the
inventory without re-learning the structure:

1. **Row binding** — the inventory ids the file backs.
2. **Representative stack** — what the archetype expects on disk.
3. **Required-mode rationale** — why each entry in
   `minimum_matrix_dimensions.required_modes` is on the row.
4. **Evidence already on file** — the fixtures, corpus rows, and
   compatibility/claim rows already attached.
5. **Open evidence questions** — the work the row needs to satisfy
   the next graduation step.
