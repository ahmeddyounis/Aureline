# Collaboration join-manifest, retention disclosure, and shared-debug view-first sequence

This document freezes the **guest-visible join-manifest** and the
**guest-visible session manifest** for collaboration sessions that
support follow and shared-debug overlays.

The goal is to keep **retention/recording posture** and **control
posture** legible **before entry**, and to keep guests **view-first**
until a **typed explicit grant** is recorded.

## Non-negotiable invariants

- **View-first by default:** joining a session admits observation and
  follow only. Guests remain view-only until an explicit grant exists.
- **No implicit control:** follow/presenter state, presence, recency, or
  join admission MUST NOT imply shared terminal/debug control.
- **Retention/recording disclosed pre-join:** retention mode, recording
  posture, and any irreversible share warnings are visible before join
  and remain reconstructable from audit/export/support artifacts.
- **Auditability:** join denied/approved, follow enablement, control
  requests, control grants/revocations, and retention/recording changes
  produce reconstructable typed records.

## Published records (cross-surface)

**Join + manifest boundary (this doc)**

- `schemas/collaboration/session_manifest.schema.json`
  - `collaboration_session_join_request_record`
  - `collaboration_session_join_decision_record`
  - `collaboration_session_manifest_record`
  - `collaboration_session_manifest_audit_event_record`

**Retention + recording disclosure (join dialog / badge / export)**

- `schemas/collaboration/session_policy_manifest.schema.json`
  - `collaboration_session_policy_manifest_record`
  - `collaboration_session_consent_event_record`
  - `collaboration_session_policy_audit_event_record`
- `schemas/collaboration/recorded_artifact_row.schema.json`
  - `collaboration_recorded_artifact_row_record`
  - `collaboration_recorded_artifact_audit_event_record`
- `schemas/collaboration/delete_path_status.schema.json`
  - `collaboration_delete_path_status_record`
  - `collaboration_delete_path_audit_event_record`

**Follow / presenter (view authority only)**

- `schemas/collaboration/follow_and_presenter_state.schema.json`
  - `presenter_state_record`
  - `follow_target_record`

**Shared control (explicit grants only)**

- `schemas/collaboration/control_grant.schema.json`
  - `control_grant_record`
  - `control_grant_revocation_record`

**Shared debug view stream (inspect-only default)**

- `schemas/execution/debug_session.schema.json`
  - `debug_session_record` with `debug_posture_class = debug_posture_live_shared_inspect_only`

## Join-manifest “publish rows” (what must be visible before entry)

The join manifest renders (or equivalently discloses) the following row
kinds before admission:

- `roles_row` — requested role vs admitted role; host identity.
- `tenant_or_policy_scope_row` — tenant/org scope plus policy context.
- `retention_envelope_row` — retention mode and consent cues.
- `recording_mode_row` — recording/transcript/replay posture as visible
  recorded-artifact rows.
- `follow_and_presenter_capabilities_row` — follow is view authority
  only; no mutating authority.
- `shared_debug_view_stream_row` — shared debug is inspect-only until a
  grant exists.
- `control_capabilities_row` — which control lanes exist and what
  badges are required to request/receive control.
- `required_badges_row` — the admission badges rendered before a guest
  assumes control is possible.

## Sequence (policy-gated, host-approved, view-first)

```mermaid
sequenceDiagram
  autonumber
  actor Host
  actor Guest
  participant Collab as Collaboration Service
  participant Policy as Policy Engine
  participant Presence as Presence
  participant Follow as Follow/Presenter
  participant Debug as Debugger
  participant Grants as Control Grants
  participant Audit as Audit/Chronology

  Host->>Collab: Publish session envelope (roles + retention envelope)
  Collab->>Policy: Validate publish (tenant/policy/recording mode)
  Policy-->>Collab: Allowed scope + required badges + disclosure posture

  Guest->>Collab: Join request (declared role)
  Collab->>Policy: Validate join request
  Policy-->>Collab: join_policy_result_class + required badges
  Collab-->>Guest: Pre-join manifest (retention/recording + control posture)

  Collab-->>Host: Join request pending approval
  Host->>Collab: Host approval or denial
  alt Join denied
    Collab-->>Guest: Join denied (typed denial reason)
    Collab-->>Audit: Join denial event
  else Join admitted (view-only)
    Collab-->>Guest: Post-join session manifest (still view-first)
    Guest->>Presence: Subscribe presence
    Guest->>Follow: Subscribe follow state
    Host->>Debug: Enable shared debug follow mode
    Debug-->>Guest: Debug view stream (inspect-only)
    Collab-->>Audit: Join + follow + recording/retention events

    opt Control request (explicit)
      Guest->>Collab: Request control (terminal/debug lane)
      Collab->>Policy: Validate control request + badges
      Policy-->>Collab: Admit or deny request
      Host->>Collab: Explicitly grant or deny control
      Collab->>Grants: Mint control_grant_record (typed lane + expiry)
      Grants-->>Guest: Control grant receipt (lane-scoped; revocable)
      Grants-->>Audit: Control grant minted / later revoked
    end
  end
```

If any surface lets follow/presence/join implicitly confer mutating
terminal/debug authority, or if it allows replay of a revoked grant, it
is non-conforming.

## Worked-example corpus

The following fixtures exercise the join-manifest contract and its
cross-surface bindings:

- `fixtures/collaboration/join_follow_cases/join_denied_policy_denies.yaml`
- `fixtures/collaboration/join_follow_cases/join_approved_view_only_follow_and_debug_view.yaml`
- `fixtures/collaboration/join_follow_cases/control_requested_granted_revoked.yaml`
- `fixtures/collaboration/join_follow_cases/retention_recording_change_mid_session.yaml`
