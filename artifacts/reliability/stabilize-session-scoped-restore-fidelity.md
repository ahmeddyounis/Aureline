# Session-Scoped Restore Fidelity Evidence

This artifact records the stable restore-fidelity packet for runtime-backed
panes.

## Canonical Inputs

- Schema: `schemas/recovery/session-restore-fidelity.schema.json`
- Rust packet: `crates/aureline-recovery/src/stabilize_session_scoped_restore_fidelity/mod.rs`
- Fixture corpus: `fixtures/recovery/stabilize-session-scoped-restore-fidelity/`
- Contract doc: `docs/reliability/stabilize-session-scoped-restore-fidelity.md`

## Evidence Summary

The packet defines controlled restore-fidelity classes, a placeholder-state
matrix, a no-hidden-rerun drill corpus, diagnostics text, and a support-export
projection.

Covered runtime-backed surfaces:

| Surface | Truthful restored state | Hidden work proof |
| --- | --- | --- |
| Terminal session | `transcript_restored` | no command rerun; transcript evidence retained |
| Task run | `rerun_required` | no task rerun; target is shown before action |
| Debug session | `session_ended` | no silent adapter attach |
| Notebook kernel | `rerun_required` | no kernel resume or cell rerun |
| Preview server | `static_evidence_only` | no server restart; static evidence shown |
| Remote tunnel | `reconnect_available` | no silent reconnect or authority reuse |

## Stable Claim Gate

A row is stable only when all of the following are true:

- layout slot is preserved;
- stale evidence or last-known metadata remains visible;
- auto-rerun is forbidden;
- silent reattach is forbidden;
- hidden authority reacquisition is forbidden;
- explicit user intent is required unless the same live runtime survived and was verified;
- target/runtime is named before rerun or reconnect;
- diagnostics and support export consume the same row.

