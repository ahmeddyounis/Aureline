# M5 Local-History-Row and Checkpoint-Group-Card Primitive

- Packet: `m5-local-history-row-checkpoint-group-card-primitive:stable:0001`
- Label: `M5 local-history-row and checkpoint-group-card primitive: snapshot origin, actor lineage, capture fidelity, trigger, object identity, branch/worktree, mutation class, retention, row posture, checkpoint lineage, file-count truth, pre/post risk, card posture, and bounded reveal/open/compare/restore/export actions`
- Recovery consumers: 5 (5 stable)
- Row postures: restorable_snapshot, automated_capture, metadata_only_reference, purge_pending_snapshot, unattributed_snapshot, expired_unrestorable
- Card postures: atomic_checkpoint, multi_file_group, generated_artifact_group, imported_group, high_risk_group, restore_blocked_group
- Row actions: reveal_lineage, open, compare, restore, export_evidence
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Recovery consumers

- **Editor Recovery**: `stable`
  - Owner: Editor recovery owner
  - Scope: The editor recovery timeline renders the shared local-history row and checkpoint-group card so a restorable manual-save row names its timestamp, actor, trigger, object identity, branch/worktree, mutation class, and retention state with open/compare/restore before restore, a metadata-only autosave reference reads as metadata-only with no restorable body, and an atomic single-action checkpoint restores as one attributable moment
  - Worked rows: 2
    - `src/editor/buffer.rs` (`local_user`) → `restorable_snapshot` (restore `true`, automated `false`)
    - `src/editor/view.rs` (`local_user`) → `metadata_only_reference` (restore `false`, automated `false`)
  - Worked cards: 1
    - `checkpoint: format buffer.rs` (1 files) → `atomic_checkpoint` (restore `true`, managed `false`)
- **Refactor History**: `stable`
  - Owner: Refactor history owner
  - Scope: The refactor history surface renders the shared local-history row and checkpoint-group card so a purge-pending refactor-apply row discloses that its history is pending purge, a multi-file grouped transaction preserves its file-count truth with a preview-scope before restore, and a high-risk dependency-change group requires review before restore rather than reading as a plain checkpoint
  - Worked rows: 1
    - `src/refactor/mod.rs` (`local_user`) → `purge_pending_snapshot` (restore `true`, automated `false`)
  - Worked cards: 2
    - `checkpoint: extract module (5 files)` (5 files) → `multi_file_group` (restore `true`, managed `false`)
    - `checkpoint: upgrade dependencies (4 files)` (4 files) → `high_risk_group` (restore `true`, managed `false`)
- **AI Apply Review**: `stable`
  - Owner: AI apply review owner
  - Scope: The AI apply review surface renders the shared local-history row and checkpoint-group card so an AI-apply row reads as an automated capture and never as if a user typed it, and a generated-artifact group discloses that it touches generated or managed files with a preview-scope before restore
  - Worked rows: 1
    - `src/ai/apply.rs` (`ai_agent`) → `automated_capture` (restore `true`, automated `true`)
  - Worked cards: 1
    - `checkpoint: regenerate bindings (3 files)` (3 files) → `generated_artifact_group` (restore `true`, managed `true`)
- **Importer Actions**: `stable`
  - Owner: Importer actions owner
  - Scope: The importer actions surface renders the shared local-history row and checkpoint-group card so an external-import row with an unknown actor reads as unattributed and prompts a reveal-lineage before trust, and an imported config-migration checkpoint preserves its origin as one attributable moment without being confused with Git history
  - Worked rows: 1
    - `config/settings.toml` (`unknown_actor`) → `unattributed_snapshot` (restore `true`, automated `false`)
  - Worked cards: 1
    - `checkpoint: imported migration (2 files)` (2 files) → `imported_group` (restore `true`, managed `false`)
- **Support Evidence**: `stable`
  - Owner: Support evidence owner
  - Scope: The support evidence surface renders the shared local-history row and checkpoint-group card so an expired-and-purged repair row whose captured object was removed still reveals its actor lineage and timestamp even though it can no longer restore, and a restore-blocked rollback group reads as restore-blocked rather than falsely offering a restore — the same row and card vocabulary a support reviewer reads elsewhere
  - Worked rows: 1
    - `src/repair/transaction.rs` (`automation_task`) → `expired_unrestorable` (restore `false`, automated `true`)
  - Worked cards: 1
    - `checkpoint: rollback repair (2 files)` (2 files) → `restore_blocked_group` (restore `false`, managed `false`)
