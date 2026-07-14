# M5 No-Rerun Session-Recovery and Authority-Replay-Fence Registries

- Packet: `m5-no-rerun-session-recovery-and-authority-replay-fence-registries:stable:0001`
- Label: `M5 no-rerun session-recovery and authority-replay-fence registries with one stable recovery-posture object resolved per session-scoped surface, the explicit posture decided before any replay, the prior authority snapshot and provenance kept distinct from the reauthorization plan, canonical / accessible / audit resolution-form coverage, and the preserved-surface-role / prior-authority-class / provenance-hint disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces`
- Consumer surfaces: 6
- Recovery-posture states: transcript_restored, session_ended, reconnect_available, rerun_required, context_unavailable, posture_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves each session-scoped surface to one stable recovery-posture object — session surface, session scope, prior authority snapshot, provenance class, reconnect plan, and the distinct reauthorization plan — from the shared registry, restores the terminal transcript read-only without rerunning it, and fences off any silent reacquisition of a privileged ticket; a posture object missing its session scope and a fence that silently reacquires a held authority degrade honestly instead of reading as a clean pass
  - Recovery-posture entries: 2 / authority-replay-fence entries: 2
- **restore_coordinator**: `stable`
  - Owner: Restore-coordinator owner
  - Scope: The restore coordinator resolves a reconnect-available posture that gates the remote shell behind disclosed reauthorization, and fences a previously held publish/deploy authority behind disclosed reauthorization rather than silently reacquiring it; a resolution-form gap on a posture entry and on a fence entry is caught before a screenshot can reintroduce a false-live reading
  - Recovery-posture entries: 2 / authority-replay-fence entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the context-unavailable posture and the deferred shared-control fence that discloses its fresh-intent requirement rather than overclaiming live, without manual reconstruction; a posture whose session-scoped work replayed before the explicit posture was decided is caught as a replay-first restore
  - Recovery-posture entries: 2 / authority-replay-fence entries: 1
- **workspace_service**: `stable`
  - Owner: Workspace-service owner
  - Scope: The workspace service resolves the rerun-required posture while keeping it bound to the registry, and fences the debug authority; a posture that is a hand-copied per-surface recovery assumption and a fence on an unclassified authority class degrade honestly
  - Recovery-posture entries: 2 / authority-replay-fence entries: 2
- **session_service**: `stable`
  - Owner: Session-service owner
  - Scope: The session service renders the same resolved recovery-posture and authority-replay-fence truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied recovery table
  - Recovery-posture entries: 2 / authority-replay-fence entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved recovery-posture and authority-replay-fence truth, so a hand-copied constant, an unstated registry token, a replay-first restore, or a silent reacquisition is visible in evidence rather than hidden behind a screenshot, and it distinguishes context-only restore from truly live session continuity
  - Recovery-posture entries: 2 / authority-replay-fence entries: 1
