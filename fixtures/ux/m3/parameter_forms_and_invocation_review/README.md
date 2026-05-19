# Command parameter forms and invocation review sheets — fixtures

Generated from `aureline-shell` headless minter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_command_forms -- catalog
cargo run -q -p aureline-shell --bin aureline_shell_command_forms -- report-md
cargo run -q -p aureline-shell --bin aureline_shell_command_forms -- validate
```

Contents:

- `catalog.json` — full catalog of paired parameter-form-state and
  invocation-review-sheet records seeded from the canonical command
  descriptor. The desktop product, CLI inspect surface, AI tool envelope,
  automation-recipe step editor, request/run/debug/template/repair
  workspaces, and voice grammars all project the same record for a given
  command id.

Schemas:

- `schemas/commands/parameter_form_state.schema.json`
- `schemas/commands/invocation_review_sheet.schema.json`
- `schemas/commands/command_descriptor.schema.json` (source of truth)

Scenarios covered:

| Scenario id | Form surface | Highlight |
|---|---|---|
| `bulk_replace_in_files_desktop_apply` | `template_or_generator_form` | Destructive bulk replace with checkpoint-backed revert and structured diff preview. |
| `debug_attach_to_process_desktop` | `run_debug_profile_form` | Debug attach with org-policy-pinned capability handle and step-up auth. |
| `request_workspace_send_request_desktop` | `request_workspace_form` | Outbound request with secret-handle auth and dry-run preview. |
| `recipe_run_blocked_by_policy` | `automation_recipe_step_editor` | Recipe step blocked by org-policy pin; review sheet stays in propose-only. |
| `release_publish_artifact_desktop` | `desktop_parameter_form` | Irreversible publish with signing-evidence-only redaction. |
| `repair_workspace_blocked_on_runtime_prompt` | `cli_inspect_surface` | CLI repair flow blocked on runtime prompt; unsupported field surfaced honestly on CLI scope. |
