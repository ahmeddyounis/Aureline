# Proof packet: command palette query session + grouped results

Purpose: anchor proof that the command palette owns a query-session object
(query text posture, provider readiness, and ranking-source attribution) and
projects grouped results from the canonical command registry and enablement
decisions without inventing palette-local command identity.

Canonical sources (non-exhaustive):

- `docs/commands/palette_query_session_contract.md`
- `docs/commands/palette_row_contract.md`
- `schemas/commands/palette_query_session.schema.json`
- `schemas/commands/palette_result.schema.json`
- `fixtures/commands/palette_query_cases/`
- `crates/aureline-shell/src/palette/query_session.rs`
- `crates/aureline-shell/src/palette/results_view.rs`
- `crates/aureline-shell/src/bin/aureline_shell.rs`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Smoke outputs (optional): `artifacts/milestones/m1/smoke_outputs/`
- Screenshots (optional): `artifacts/milestones/m1/screenshots/`
