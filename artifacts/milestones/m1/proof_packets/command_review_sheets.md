# Proof packet: command diagnostics + invocation preview sheets

Purpose: anchor proof that the shell renders dedicated review sheets for
disabled commands (diagnostics) and consequence-bearing commands (invocation
preview) without forking command identity, enablement reasons, or preview
posture into surface-local state.

Canonical sources (non-exhaustive):

- `docs/ux/command_review_sheets_contract.md`
- `crates/aureline-shell/src/commands/mod.rs`
- `crates/aureline-shell/src/commands/diagnostics_sheet.rs`
- `crates/aureline-shell/src/commands/invocation_preview.rs`
- `crates/aureline-shell/src/bin/aureline_shell.rs`
- `fixtures/commands/review_sheets/`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Sheet-record logs: `.logs/review_sheets/` (runtime export)

