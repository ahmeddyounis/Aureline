# Session Role Admission And Retention Qualification

Canonical packet: `artifacts/collab/m4/session-role-admission-and-retention-qualification.json`

Schema: `schemas/collab/session-envelope.schema.json`

This packet qualifies M4-visible collaboration-adjacent lanes. A lane may render as Stable only when it can show workspace identity, inviter or approver, requested role, expiry, client boundary, admission state, retention mode, guest scope, local-vs-managed retention truth, export/delete rights, support-export posture, and downgrade behavior before the user relies on it.

## Stable Scope

Stable coverage is limited to:

- desktop invite disclosure for observer-first admission
- host lobby/admission review for approved external observers
- presenter/follow metadata with explicit breakaway and return-to-presenter affordances
- session export/delete/hold truth for local and managed copies

Browser observer join and mobile rejoin remain Preview rows. They are visible in the promoted build only with Preview language and cannot imply editor, presenter, driver, terminal, debugger, or hidden control authority.

## Invitation

The invite row is stable because it names the workspace, host, requested observer role, absolute expiry, desktop client boundary, live-only retention posture, local-only invite decision telemetry, and the available accept/decline/details actions before acceptance.

Accepting an invite never grants shared terminal, shared debugger, presenter, or edit control. Those authorities require a later review event.

## Lobby And Admission

The lobby row is stable for approved external observers only. The host sees requester identity through an opaque participant reference, the approver, the requested observer role, ticketed guest scope, metadata-audit retention, managed audit retention, and explicit admit/deny/defer actions.

Guest admission is a visible review event. External guests remain observer-first; any later role widening must create a separate consent or authority event.

## Presenter Follow And Breakaway

Presentation and guided-follow behavior is stable only as metadata and view control. The envelope names the presenter/moderator, viewer role, breakaway state, `return_to_presenter` and `stay_independent` actions, metadata-only retention, and support-export projection.

Follow and breakaway state is never treated as edit history, document causality, terminal control, debugger control, or retained source content.

## Export Delete And Hold

The export/delete row separates local and managed copies:

- local redacted export manifests can be deleted from the device
- managed evidence follows support policy
- legal hold blocks managed content deletion
- hold summary export remains available

Exports are redaction-safe, attributable, timezone-aware through the packet timestamps, and projected consistently into desktop, browser companion, mobile follow, docs/Help, and support export.

## Preview And Labs Rows

Browser observer join and mobile rejoin after policy narrowing remain Preview. They can show session identity, requested observer role, retention/export posture, local note preservation, and desktop handoff, but they are not part of the Stable contract.

Preview rows must render below Stable everywhere they appear: shell, browser companion, mobile follow, docs, Help/About, and support exports.

## Downgrade Behavior

Downgrades narrow authority first and preserve local continuity:

- client-scope narrowing keeps observer-only or handoff-only behavior
- policy narrowing requires fresh rejoin review
- guest-boundary narrowing blocks role widening without review
- relay degradation must preserve local unsent work or local notes
- legal hold changes delete semantics without hiding exportable hold summaries

Local unsent work, local notes, and local continuity are never deleted because relay, policy, or scope changed.

## Support Export Projection

Support exports ingest the canonical envelope id, lane kind, displayed label, workspace identity, inviter/approver, requested role, expiry, client boundary, admission state, retention mode, guest scope, local-copy truth, managed-copy truth, legal-hold state, presenter/follow state, consent trigger, downgrade class, participant actions, and proof refs. Support tools should not clone alternate status text.
