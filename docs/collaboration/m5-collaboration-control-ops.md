# M5 collaboration-control component matrix — operations contract

This document is the human-readable companion to the frozen M5 collaboration-control matrix. The
authoritative gate is the Rust validator in
`crates/aureline-ui/src/m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix/`;
the checked-in schemas, fixtures, dashboard, and release proof bundle are canonical for this lane. This doc
names what the matrix freezes so later implementation does not re-interpret control-grant, recording, or
restore semantics per surface.

- Matrix schema: `schemas/collaboration/m5-collaboration-control-component-matrix.schema.json`
- Support export: `artifacts/release/m5-collaboration-control-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-collaboration-control-proof/matrix.csv`
- Design report: `artifacts/design/m5-collaboration-control-component-matrix.md`
- Health dashboard: `dashboards/m5-collaboration-control-health.json`
- Narrowed fixtures: `fixtures/collaboration/m5-shared-control/`
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_collaboration_control_matrix -- <subcommand>`

## Governed object classes

Every sensitive collaboration moment binds to one explicit collaboration-control object; the six classes are:

1. **shared_terminal_debug_view** (`schemas/ui/m5-shared-terminal-debug-view.schema.json`) — the live shared
   terminal / debugger view: live stream, single active driver, view-first default, input provenance.
2. **control_grant** (`schemas/collaboration/m5-control-grant.schema.json`) — the explicit grant of terminal /
   debug write control: granted authority, single active driver, grant scope / expiry, revoke / reclaim path.
3. **presenter_token** (`schemas/ui/m5-presenter-handoff-sheet.schema.json`) — the presenter / moderator token
   and handoff: presenter holder, handoff target, moderation scope, token expiry / reclaim.
4. **consent_envelope** (`schemas/ui/m5-collaboration-join-review-sheet.schema.json`) — the join-time consent
   envelope: consent scope, guest scope and route visibility, recording / retention consequences, renewal.
5. **retention_review** (`schemas/ui/m5-collaboration-retention-sheet.schema.json`) — the recording / retention
   / sealed-archive review: recording state, retention mode and duration, replayable-archive and export scope.
6. **session_restore_view** (`schemas/collaboration/m5-session-policy-manifest.schema.json`) — the replay-free
   session-restore view: read-only reattach, no input replay, retention scope preserved, fresh-grant required.

## Frozen state vocabulary (`session_state`)

`viewer`, `commenter`, `editor`, `driver`, `navigator`, `presenter_moderator`, `live_only`, `metadata_audit`,
`replayable_text_comment_timeline`, `elevated_support_evidence`, `control_requested`, `control_granted`,
`control_expired`, `recording_active`, `consent_renewal_required`, `restore_view_only`. Only `driver` is the
single active driver; every other state must not read as holding terminal / debug write control.

## Control-authority source (never flattened)

`granted_by_explicit_control_grant`, `delegated_by_presenter_token`, `inferred_from_presence_or_follow`,
`expired_or_revoked_grant`. Only an explicit control grant is authoritative — control is never acquired from
presence or follow alone.

## Consent / retention gate

`consent_current_recording_allowed`, `blocked_by_missing_join_consent`, `blocked_by_consent_renewal_required`,
`blocked_by_retention_scope_widening`, `blocked_by_guest_scope_or_route_expansion`. Recording / retention is
blocked while consent is missing, lapsed, or not yet given for a proposed widening.

## Consumer lanes

`shared_terminal_debug_view`, `collaboration_join_review_sheet`, `control_grant_prompt`,
`presenter_handoff_sheet`, `paste_secret_guard`, `collaboration_retention_sheet`, `session_restore_view`,
`support_export_packet`, `help_docs`. Surface families: `desktop_session_surface`,
`shared_terminal_debug_surface`, `companion_browser_surface`, `incident_support_surface`, `support_export`,
`help_docs`.

## Downgrade triggers

`control_acquired_without_explicit_grant`, `more_than_one_active_driver_on_a_sensitive_surface`,
`recording_or_retention_started_silently`, `prior_input_replayed_on_join_or_restore`,
`raw_secret_or_clipboard_revealed_without_guard`, `control_authority_unstated`, `active_driver_unstated`,
`view_first_default_unstated`, `consent_scope_unstated`, `retention_state_unstated`,
`restore_replay_safety_unstated`, `collaboration_control_matrix_stale`. A claimed class narrows automatically
when its matrix row is missing or its proof has gone stale.

## Hard guardrails (each is a per-row invariant that MUST be `false`)

1. Do not let presence, follow mode, browser handoff, or companion resume acquire terminal / debug control
   without an explicit grant.
2. Do not allow more than one active driver on a sensitive surface.
3. Do not start recording, transcript retention, replayable archives, or guest-scope / route widening silently.
4. Do not replay prior terminal / debug input on join or restore.
5. Do not reveal raw secrets, command text, variable bodies, or clipboard contents without a guard and consent
   posture.

## Acceptance criteria

- **AC1** — A reviewed component matrix names every object, actor, visible badge, downgrade trigger, and export
  surface required for shared-control and consent truth (the six rows above, the `session_state` vocabulary,
  the consumer lanes, and the downgrade triggers, all frozen in one packet).
- **AC2** — Follow-on rows can implement one consistent collaboration-control and retention vocabulary without
  redefining grants, recording, or restore semantics per surface, because the vocabulary, per-domain schemas,
  and hard guardrails are frozen here and bound back to the already-landed paste / secret guard,
  stable-proof-index, and migration-task-row packets.
