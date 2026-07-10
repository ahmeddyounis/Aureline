# Generator-preview sheets and run-config scaffold cards

- Packet: `m5-generator-preview-run-config-controls:stable:0001`
- Surface: `M5 generator-preview sheets and run-config scaffold cards: generator identity / version, parameters, created-versus-modified paths, managed-versus-user-owned files, dependency / config impact, rollback / regenerate posture, target kind, environment / profile, launch command, required toolchain, and local / container / SSH / managed execution-boundary truth across claimed framework actions`
- Generator-preview sheets: 6 (4 write files, dependencies, or config)
- Run-config scaffold cards: 6 (3 write config or dependencies)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-10T00:00:00Z)

## Generator-preview sheets

- **Add CI config (blocked)** — impact `config_change`, apply `blocked`, write `unknown_or_blocked`, files `modifies_file` (+0 / ~1), ownership `user_owned`, recovery `forward_fix_only`
- **Add ORM integration** — impact `dependency_change`, apply `preview_ready`, write `review_required_write`, files `creates_file` (+2 / ~0), ownership `managed_generated`, recovery `rollback_and_regenerate`
- **Generate new route module** — impact `file_write`, apply `review_required`, write `review_required_write`, files `creates_and_modifies` (+1 / ~1), ownership `mixed_ownership`, recovery `rollback`
- **Add build task** — impact `script_or_task_change`, apply `rollback_available`, write `reversible_applied`, files `modifies_file` (+0 / ~1), ownership `managed_generated`, recovery `regenerate`
- **Preview component scaffold** — impact `no_change`, apply `regenerate_available`, write `no_op_preview`, files `no_file_change` (+0 / ~0), ownership `managed_generated`, recovery `no_recovery_needed`
- **Third-party codemod (unknown impact)** — impact `unknown_impact`, apply `apply_ready`, write `unknown_or_blocked`, files `no_file_change` (+0 / ~0), ownership `unknown_ownership`, recovery `no_recovery_needed`

## Run-config scaffold cards

- **Run web app (dev)** — target `web_app`, profile `development`, boundary `local_process`, toolchain `toolchain_ready`, mutation `creates_config_file`, write `review_required_write`, recovery `rollback`
- **Run API (container)** — target `api_server`, profile `debug`, boundary `container`, toolchain `toolchain_ready`, mutation `edits_config_file`, write `review_required_write`, recovery `rollback_and_regenerate`
- **Run worker (SSH remote)** — target `background_job`, profile `production`, boundary `ssh_remote`, toolchain `toolchain_version_mismatch`, mutation `adds_dependency`, write `review_required_write`, recovery `rollback`
- **Run tests (managed workspace)** — target `test_suite`, profile `test`, boundary `managed_workspace`, toolchain `toolchain_ready`, mutation `no_write_preview`, write `no_op_preview`, recovery `no_recovery_needed`
- **Run CLI (cloud remote)** — target `cli_tool`, profile `custom_profile`, boundary `cloud_remote`, toolchain `toolchain_missing`, mutation `rollback_available`, write `reversible_applied`, recovery `rollback`
- **Run imported config (unknown)** — target `unknown_target`, profile `custom_profile`, boundary `unknown_boundary`, toolchain `toolchain_unknown`, mutation `unknown_mutation`, write `unknown_or_blocked`, recovery `forward_fix_only`
