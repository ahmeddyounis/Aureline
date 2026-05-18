# Native Desktop Beta Contract

This contract defines the beta support boundary for native desktop entry,
reopen, and interruption flows. It composes the existing handoff, notification,
install ownership, target binding, and lifecycle artifacts rather than minting a
parallel source of truth.

## Contract Objects

- `desktop_entry_event_record` in
  [`schemas/platform/desktop_entry_event.schema.json`](../../../schemas/platform/desktop_entry_event.schema.json)
  is the cross-surface event shape for system open, file association, auth
  callback, protocol handler, recent item, jump-list action, reveal-in-shell,
  badge activation, and privacy-safe native notification reopen.
- `native_desktop_contract_packet` is the aggregate proof packet exported by
  `aureline_shell::platform_integration`.
- Fixtures live in
  [`fixtures/platform/m3/native_desktop_contract/`](../../../fixtures/platform/m3/native_desktop_contract/).

## Required Behavior

Every desktop entry event must disclose:

- source surface and origin class;
- requested action class and canonical command or route class;
- literal target plus canonical target identity;
- owning channel and build owner;
- trust/profile context and policy epoch;
- resulting mode and recovery surface;
- recovery actions when the target is missing, stale, expired, or unavailable.

Notification reopen and badge activation are summary-only. They may open the
exact durable object or a truthful placeholder, but they must not open a generic
home screen and must not complete mutations from the OS surface. Lock-screen and
OS payloads stay redacted and category-level.

Wake/resume, removable-root loss, network-share loss, credential-store lock,
and display-topology drift must preserve local work where possible, pause
privileged or mutating work, and offer explicit locate, reconnect, recenter,
reauthenticate, or cached-context actions. No callback, notification, or resume
path may silently replay mutating work or reuse stale authority.

## Beta Support Matrix

The current beta proof packet covers:

| Surface | Proof requirement |
|---|---|
| system open | literal target, canonical target, channel/build owner, trust context, exact open or placeholder |
| auth callback | origin, replay posture, step-up/restart path, no stale authority reuse |
| file associations | handler ownership, trust review, exact file identity, no silent ownership steal |
| recent items | exact recent-work identity or placeholder with locate/cached-context recovery |
| privacy-safe native notification | redacted summary, exact durable reopen target, one safe in-product action |

Per-platform drills cover channel precedence, handler spoof-resistance, recent
reopen fidelity, lock-screen redaction, and wake/resume truth on the claimed
macOS, Windows, and GNOME Wayland rows.

