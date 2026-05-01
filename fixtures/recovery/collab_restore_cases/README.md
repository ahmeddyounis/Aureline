# Restored collaboration state fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/recovery/collab_restore_and_presentation_contract.md`](../../../docs/recovery/collab_restore_and_presentation_contract.md)
and validated by

- [`/schemas/recovery/restored_collab_state.schema.json`](../../../schemas/recovery/restored_collab_state.schema.json)
  — closed restored-collab-kind, restoration-posture,
  live-authority-status, no-auto-resume, required-regrant-action,
  reopen-as, forbidden-claim, surface-family, and
  re-exported follow / presenter / control-grant vocabularies
  plus the typed export-lane block and packet/export linkage.

Each fixture names the restored kind it covers, the typed
restoration posture, the typed live-authority status, the
no-auto-resume directives (for shared-control records), and the
contract sections it motivates.

**Scope rules**

- Fixtures validate against the restored-collab-state schema;
  they do not redefine follow, presenter, control-grant,
  workspace-authority, window-topology, restore-provenance,
  recovery-ladder, support-bundle, or release-evidence
  vocabularies (those are cited by opaque ref).
- A new fixture MUST exercise at least one
  `restored_collab_kind_class`, `restoration_posture_class`,
  `live_authority_status_class`, `no_auto_resume_class`,
  `required_regrant_action_class`, `reopen_as_class`,
  `forbidden_claim_class`, `surface_family`, or re-exported
  follow / presenter / grant value the existing set does not
  already cover, and MUST cite the contract section it
  motivates.
- Monotonic timestamps and stable ids are opaque; they read
  well rather than reflect any real clock.

**Index**

| Fixture | Kind(s) | Posture | Authority status | Doc section |
|---|---|---|---|---|
| [`presenter_broadcast_restored_as_local_context.yaml`](./presenter_broadcast_restored_as_local_context.yaml) | `restored_presenter_state`, `restored_presenter_grant_state`, `restored_role_badge`, `restored_shared_cursor_visibility` | `restored_as_local_context_only` | `not_live_explicit_regrant_required` (presenter), `not_live_no_grant` (badge / cursor) | §1, §2, §3, §4, §5, §7, §8, §9, §10 |
| [`shared_terminal_grant_placeholder_pending_explicit_regrant.yaml`](./shared_terminal_grant_placeholder_pending_explicit_regrant.yaml) | `restored_shared_terminal_control_grant_state` | `restored_as_placeholder_pending_session_rejoin` | `not_live_explicit_regrant_required` | §1, §2, §3, §4, §5, §6, §7, §9 |
| [`shared_debugger_grant_revoked_evidence_only.yaml`](./shared_debugger_grant_revoked_evidence_only.yaml) | `restored_shared_debugger_control_grant_state` | `degraded_grant_revoked` | `not_live_grant_revoked_evidence_only` | §3, §4, §5, §6, §7, §8 |
| [`expired_collaboration_session_evidence_only.yaml`](./expired_collaboration_session_evidence_only.yaml) | `restored_presenter_handoff_history`, `restored_follow_target`, `restored_role_badge` | `degraded_session_expired` | `unrecoverable_collaboration_authority_evidence_only` | §3, §4, §7, §8 |
| [`missing_collaborator_degraded_follow_summary.yaml`](./missing_collaborator_degraded_follow_summary.yaml) | `restored_follow_target` | `degraded_collaborator_missing` | `not_live_explicit_regrant_required` | §2, §3, §4, §7 |
| [`role_badge_restored_no_live_authority.yaml`](./role_badge_restored_no_live_authority.yaml) | `restored_role_badge` (driver, approver) | `restored_as_local_context_only` | `not_live_no_grant` | §2, §3, §4 |
| [`speaker_note_locality_local_to_workspace.yaml`](./speaker_note_locality_local_to_workspace.yaml) | `restored_speaker_note_locality`, `restored_shared_cursor_visibility` | `restored_as_local_context_only` | `not_live_no_grant` | §2, §3, §4 |
| [`shared_notebook_kernel_grant_admin_signed_admission_required.yaml`](./shared_notebook_kernel_grant_admin_signed_admission_required.yaml) | `restored_shared_notebook_kernel_control_grant_state` | `restored_as_placeholder_pending_session_rejoin` | `not_live_explicit_regrant_required` | §3, §5, §6, §9 |
