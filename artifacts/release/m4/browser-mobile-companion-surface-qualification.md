# Browser/Mobile Companion Surface Qualification

Canonical packet: `artifacts/release/m4/browser-mobile-companion-surface-qualification.json`

This packet qualifies browser and mobile companion surfaces as scoped clients, not as a second IDE. Stable rows must name their allowed scope, unsupported actions, state freshness, authority boundary, and exact desktop handoff behavior.

## Browser Review And Light Edit

Stable companion scope covers review, comments, and selected text-file light edits in the browser client. Terminal control, debugger control, workspace trust changes, generated-file rewrites, merge/rebase, and environment mutation remain desktop-authoritative.

The browser scope header must show client scope, target object, freshness, allowed actions, provider/tenant, and `Open in desktop`. Desktop handoff preserves provider object, file, hunk, comment thread, route class, tenant, policy posture, replay expiry, prior role, and return anchor.

## Mobile Notification Triage

Stable companion scope covers metadata-minimized notification triage and handoff cards. Notifications may show cached state, but cached state must carry last-sync and payload-class cues before any user relies on it.

Approvals, reruns, rollback, debugger control, and terminal control remain desktop or fresh-connection actions. Notification opens must not claim that those high-authority actions completed when only a handoff packet was generated.

## Desktop Handoff

Desktop handoff is a first-class Stable surface. It opens the exact desktop target or a truthful placeholder when the link is expired, disconnected, or policy-blocked. Handoff packets preserve object identity, route class, return anchor, tenant, policy posture, expiry truth, prior role, and the requirement that control be requested again.

## Preview Rows

Browser docs context, browser session follow, and mobile session join are included in the promoted build as Preview rows. They must render `Preview` language in product, docs, Help/About, and support exports until accessibility, privacy, and current proof packets exist.

Session follow and join/rejoin paths are observer-only, follow-only, or join-only. They may not imply hidden shared control, presenter authority, terminal authority, debugger authority, or local IDE parity.

## Privacy Validation

Companion payloads use metadata-only notification content by default, disclose provider/tenant identity before action, and preserve auth/tenant reminders on desktop handoff. Expired or disconnected handoffs reopen as exact-target placeholders rather than generic home screens.

## Support Export Projection

Support exports ingest each row's surface id, visible label, scopes, unsupported actions, freshness, freshness cues, authority, desktop handoff truth, evidence refs, and rationale. This lets support reconstruct which companion scopes were Stable, which stayed Preview, and why.
