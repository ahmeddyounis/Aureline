# M5 no-rerun session-recovery and authority-replay-fence registries

This lane is the no-hidden-rerun recovery implement lane over the frozen
[M5 window-restore matrix](./m5_window_restore_contract.md). It turns the *session-recovery-posture* grammar and
the *authority-replay-fence* grammar into registry resolvers that produce export-safe, honest projections, so the
shell, recovery, diagnostics, admin, workspace, session, docs, CLI, and support surfaces resolve one canonical
recovery-orchestration truth instead of a per-surface, hand-copied auto-rerun. Every claimed M5 restore resolves
each session-scoped surface — terminals, debug sessions, notebooks, previews, remote shells, and collaboration
panes — to one explicit reconnect-or-rerun posture (transcript restored, session ended, reconnect available,
rerun required, or context unavailable) instead of silently rerunning commands, and it fences off any silent
reacquisition of a privileged ticket, remote-attach authority, publish/deploy flow, notebook execution, or
shared-control grant, so context is preserved after restart without replaying mutating or privileged activity,
provenance keeps whether a surface is live, stale evidence, or awaiting fresh user intent, and a restore that
only reopened context or evidence can never read as truly live continuity.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_no_rerun_session_recovery_and_authority_replay_fence_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/shell/m5-no-rerun-session-recovery-and-authority-replay-fence-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/shell/m5-restore-fidelity.schema.json`](../../schemas/shell/m5-restore-fidelity.schema.json) and
  [`schemas/shell/m5-window-topology.schema.json`](../../schemas/shell/m5-window-topology.schema.json) as its
  canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-no-rerun-session-recovery-and-authority-replay-fence-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/ui/m5-no-rerun-session-recovery-and-authority-replay-fence-registries/`
  (`reconnect_posture_beta_narrowed.json`, `context_only_continuity_preview_narrowed.json`).

## Two registries

1. **Session-recovery posture** (`resolve_session_recovery_posture_entry`) — resolves each session-scoped surface
   to one stable recovery-posture object: the recovery-posture state and canonical recovery mode, the session
   surface, the session scope, the prior authority snapshot, the provenance class, the reconnect plan, and the
   distinct reauthorization plan. A clean entry names a canonical registry token, a classified recovery-posture
   state, and a window-restore role, covers the canonical / accessible / audit resolution forms, publishes a
   complete object, decides the explicit posture before any replay, and discloses reauthorization when it
   requires fresh user intent. Otherwise it degrades honestly — session-scoped work that replayed before the
   explicit posture was decided degrades to `replay_preceded_posture`.
2. **Authority-replay fence** (`resolve_authority_replay_fence_entry`) — blocks silent reacquisition of a
   privileged ticket, remote-attach authority, publish/deploy flow, notebook execution, or shared-control grant.
   A clean entry names a classified authority-replay-fence class and provides the preserved-surface-role /
   prior-authority-class / provenance-hint disclosure triple; a fence that reruns session-scoped work or
   reacquires broader authority automatically, hides that reauthorization is required, or overclaims live
   continuity on a deferred privileged flow degrades to `authority_replay_fence_reacquires_or_overclaims`.

## Per-recovery posture reference

The recovery-posture state carries its canonical recovery mode, and the resolver publishes the full posture
object, so the registry — never a hand-copied per-surface recovery assumption — is the single source of truth.
`recovery_posture_object_is_complete` rejects an object missing any field, `posture_precedes_replay` rejects a
replay-first restore, and `authority_replay_fence_holds` rejects a fence that reran session-scoped work or
silently reacquired authority.

| recovery-posture state | recovery mode | session surface | session scope | prior authority | provenance | reconnect plan |
| --- | --- | --- | --- | --- | --- | --- |
| transcript restored | transcript_restored | `session-surface.terminal.main` | `session-scope.workspace` | `authority-snapshot.none` | `provenance.stale-evidence` | `reconnect-plan.none` |
| reconnect available | reconnect_available | `session-surface.remote-shell.secondary` | `session-scope.remote` | `authority-snapshot.remote-attach` | `provenance.awaiting-fresh-intent` | `reconnect-plan.available` |
| context unavailable | context_unavailable | `session-surface.notebook.detached` | `session-scope.detached` | `authority-snapshot.publish-deploy` | `provenance.awaiting-fresh-intent` | `reconnect-plan.available` |
| rerun required | rerun_required | `session-surface.debugger.third` | `session-scope.workspace` | `authority-snapshot.shared-control` | `provenance.awaiting-fresh-intent` | `reconnect-plan.none` |
| session ended | session_ended | `session-surface.terminal.main` | `session-scope.workspace` | `authority-snapshot.none` | `provenance.stale-evidence` | `reconnect-plan.none` |

A replay-first restore degrades to `replay_preceded_posture`, an incomplete object degrades to
`recovery_posture_object_incomplete`, and a silent reacquisition degrades to
`authority_replay_fence_reacquires_or_overclaims`, so a replay-first restore, an incomplete object, or a silent
reacquisition can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Session-scoped surfaces never rerun or regain authority automatically after restore.** The explicit posture
  is decided before any replay: a replay-first example degrades, an unbound example degrades, a clean
  explicit-posture entry is present, and no clean entry replayed first.
- **Users can distinguish context-only restore from truly live session continuity.** Clean posture entries cover
  the canonical transcript-restored / session-ended / reconnect-available / rerun-required / context-unavailable
  states and the first shell / recovery / diagnostics / admin / support surfaces, an object-incomplete example
  degrades, and no clean posture entry published an incomplete object.
- **Recovery drills fail when restore replays session-scoped work or hides that reauthorization is required.**
  Clean authority-replay-fence entries cover the privileged-ticket / publish-deploy / shared-control classes with
  full resolution-form coverage while providing the disclosure triple, and a fence that silently reacquires
  authority or hides that reauthorization is required degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- support-export
cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- csv
cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- report
cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- recovery-posture-table
cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- fixture-reconnect-posture-beta-narrowed
cargo run -p aureline-ui --example dump_m5_no_rerun_session_recovery_and_authority_replay_fence_registries -- fixture-context-only-continuity-preview-narrowed
```
