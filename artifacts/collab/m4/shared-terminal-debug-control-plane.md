# Shared Terminal Debug Control Plane

Canonical packet: `artifacts/collab/m4/shared-terminal-debug-control-plane.json`

Schema: `schemas/collab/shared-control-grant.schema.json`

This packet qualifies shared terminal, shared debugger, presenter handoff, support-follow, and restore/rejoin control lanes. A promoted shared-control lane may render as Stable only when it proves a separate control transport, explicit grant state, one active driver, visible holder or view-only state, guardrails for high-risk actions, replay-free join/restore, and exportable lineage.

## Stable Scope

Stable coverage is limited to:

- desktop shared terminal control with a time-boxed active driver grant
- desktop shared debugger control with explicit presenter handoff and single active driver
- support-follow as view-only follow or breakaway state
- desktop rejoin as view-only until a fresh control grant is accepted

Browser companion join is deliberately Preview. It can render a view-only stream and route users to desktop or a grant review path, but it cannot claim Stable shared-control parity.

## Shared Terminal Active Driver

The terminal row is Stable because it names the terminal target, uses a control channel separate from the session presence channel, records the issuer, accepter, current driver, expiry, revoke action, and driver lineage, and declares guardrails for high-risk paste, secret detection, clipboard bridge, signal sends, and expiry/revocation.

Only one driver is active. Followers and observers may inspect output and request handoff, but they do not inherit input, paste, resize, or signal authority from presence or follow state.

## Shared Debugger Handoff

The debugger row is Stable because presenter handoff is visible and auditable. The lineage records the requested handoff, accepted handoff, accepted grant, and active driver transition. Debug step/continue, breakpoint edit, debug evaluate, and live-process attach are scoped separately.

Debug evaluate, variable/environment reveal, and live-process attach remain approval-gated. The packet retains grant metadata and target context, not raw variable bodies or debug-evaluate payloads.

## Browser And Mobile Boundaries

Browser and mobile follow clients enter view-only. They can render the view stream, show presenter or return-to-presenter state, preserve local notes, open on desktop, or request a control review. They cannot silently widen from follow/view into terminal or debugger control.

The current browser companion row renders as Preview because the safety behavior exists but the Stable cross-client evidence set is intentionally not claimed.

## Support Follow View Only

Support-follow is Stable only as a non-driving lane. The support agent can follow, break away, inspect support evidence policy, and request a control review. The support console cannot acquire shell or debugger control through support presence, incident membership, or follow state.

Any future support-control path must use the same explicit grant object, expiry, revoke action, guardrails, and lineage as the desktop control lane.

## Restore And Rejoin

Join, rejoin, reconnect, and restore rows are replay-free. Restored terminal/debugger views can show transcript or view state, target context, presenter/follow state, and prior grant expiry, but they cannot replay input, signals, debug actions, or hidden authority.

Live control after restore requires a fresh accepted grant. The UI must show view-only state or the current driver, target context, guardrails, and grant expiry or restore state before the user can rely on the surface.

## Support Export Projection

Support exports ingest the canonical packet id, control id, target context, surface kind, client boundary, grant state, grant scope, holder lineage, restore posture, replay status, guardrails, raw-content retention posture, UI indicators, and evidence refs.

Exports may reconstruct grant/accept/revoke/handoff chronology from lineage metadata. Ordinary pairing and support-follow defaults do not retain raw terminal bytes, command bodies, clipboard payloads, debug-evaluate payloads, variable bodies, or environment values.

## Downgrade Rule

Any exposed shared-control row that cannot prove channel separation, explicit grant state, single active driver semantics, guardrail coverage, replay-free restore, and support-export lineage must render below Stable or be withdrawn.
