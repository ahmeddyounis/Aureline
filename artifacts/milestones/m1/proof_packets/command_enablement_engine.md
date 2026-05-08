# Proof packet: command enablement engine + disabled-reason vocabulary

Purpose: anchor proof that command enablement is computed by one shared engine
and that shell surfaces can render typed `(decision_class, disabled_reason_code,
repair_hook_ref)` tuples without per-surface ad hoc logic.

Canonical sources (non-exhaustive):

- `docs/commands/command_descriptor_contract.md`
- `schemas/commands/command_descriptor.schema.json`
- `docs/commands/disabled_reason_vocabulary.md`
- `crates/aureline-commands/src/enablement.rs`
- `fixtures/commands/disabled_reason_cases/`
- `crates/aureline-shell/src/bin/aureline_shell.rs`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`

