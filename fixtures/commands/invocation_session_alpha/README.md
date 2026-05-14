# Alpha Invocation-Session Fixtures

These fixtures are the alpha baseline for command invocation and result
packets. They cover:

- a direct success path (`open_folder_success.invocation.json`);
- a disabled path with typed repair hook (`clone_repository_dependency_disabled.invocation.json`);
- a preview/approval result with checkpoint and rollback refs
  (`import_profile_preview_success.result.json`);
- a preview failure that preserves unapplied refs and evidence
  (`restore_checkpoint_preview_failed.invocation.json`).

The `aureline-commands` alpha registry tests parse these fixtures and compare
their command IDs, preview/approval posture, and result contract refs against
`artifacts/commands/alpha_command_registry.yaml`.
