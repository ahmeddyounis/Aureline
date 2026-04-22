# Input adapter and desktop lifecycle failure modes

This document is the degraded-state companion to
[`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md).
It freezes the failure and recovery table for the shell boundary so
platform adapters, multi-window verification, support/export, and later
implementation work do not invent local fallback stories.

If this document and ADR 0016 disagree, ADR 0016 wins and this document
must be updated in the same change.

Related contracts:

- [`/docs/platform/desktop_platform_conformance_matrix.md`](../platform/desktop_platform_conformance_matrix.md)
- [`/docs/qa/multi_window_verification.md`](../qa/multi_window_verification.md)
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
- [`/docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md)
- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)

## Shared recovery rule

The shell may restore layout, titles, cwd hints, transcripts, outputs,
and other local-safe evidence automatically. It may not silently rerun
commands, silently reacquire remote or privileged control, silently
reuse approval or auth state, or silently replay text input.

## Failure and degraded-state table

| Failure or event | Primary detector | Required degraded posture | Shell response | Explicit user action required | Must never happen |
|---|---|---|---|---|---|
| keychain, secret store, or trust store unavailable | platform adapter | local-safe continuation plus inspect-only or session-only fallback where the owning contract allows it | keep the shell usable, surface a durable degraded-state row, disable durable secret writes and trust-dependent routes, and point to the repair path | reauthenticate, repair the store, inspect certificates, or retry once the OS state returns | plaintext secret fallback, hidden trust bypass, or background approval reuse |
| unsupported text-input path on the active surface | platform adapter plus shell input normalizer | narrow only the affected text-entry path | preserve visible focus and current text state, explain which path is unsupported, and either block the action explicitly or narrow to one visible text target | switch input method, retry in a supported field, or use the explicit fallback route | corrupt text, drop codepoints silently, or reinterpret AltGr/dead keys as commands |
| IME reset, preedit target loss, or composition target invalidated by layout churn | shell input normalizer | composition remains visible or the action blocks explicitly | keep or relocate the visible preedit target, pause filtering if needed, and keep the owning window and focus target stable | finish or cancel composition explicitly after the target is visible again | silent commit, silent cancel, or background target switch |
| accessibility bridge loss or partial bridge failure | platform adapter plus bridge health reporting | truthful accessibility degradation with keyboard-complete host fallback where possible | surface a durable degraded-state notice, keep host focus and command routes coherent, and narrow any stable accessibility claim on the affected row | reconnect the bridge, restart the affected process, or use the exported/textual fallback if needed | silently claim normal assistive-tech parity or hide focus state |
| focus handoff target lost on dialog, sheet, palette, or overlay close | shell focus router | nearest visible owner in the same window becomes the fallback target | emit the typed focus-loss reason from the interaction-safety contract and keep the user in a reachable visible control | usually none beyond continuing work; a repair route may be offered when the owner surface is gone | focus loss with no announcement, focus landing in a hidden control, or cross-window focus jump |
| adaptive-collapse edge case in compact layouts | shell adaptive-layout controller | optional detail moves to sheet, overflow, or drawer before identity/trust/task state is narrowed | preserve title/context identity, dominant task surface, preview/approval state, and visible focus while moving secondary chrome | open the overflowed or sheeted surface explicitly when needed | hide a critical action behind an undisclosed overflow, lose keyboard continuity, or collapse the only recovery route |
| off-screen restore after display topology change | platform adapter plus restore controller | safe-bounds remap with layout intent preserved over stale coordinates | recenter windows and owner dialogs, preserve the visible active pane, and keep restore provenance inspectable | none in the common case; the user may choose a manual layout reset if they want a different arrangement | stranded windows, orphaned sheets, or focus off a reachable display |
| wake from sleep on windows with live remote, debug, terminal, notebook, or callback state | platform adapter plus shell lifecycle controller | local-safe shell restore plus explicit live-authority rebind | restore window shells, titles, outputs, and placeholders; mark remote or privileged surfaces as reconnect or rebind required | reconnect, reauthorize, rerun, or explicitly rejoin live control as required by the surface | silent terminal input replay, debug continue, publish, callback execution, or approval-ticket reuse |
| display reconnect or scale-bucket change while windows remain open | platform adapter plus shell lifecycle controller | visible-focus-first topology adjustment | update safe bounds, scale bucket, and owner-dialog placement without changing window ownership or command routes | usually none beyond continuing work | hidden focus drift, silent zone collapse, or off-screen owner dialogs |
| removable volume return or missing-root recovery | VFS plus platform adapter mount facts | placeholder-first continuity with explicit locate or retry path | preserve surrounding layout, show the missing-root placeholder, and keep titles/history/provenance visible until the root returns or the user closes it | locate the root, retry, continue with cached context where allowed, or close the affected pane/workspace | silent path remap to a different target, hidden write retry, or presenting disappearance as local data loss |

## Automatic restore vs explicit revalidation

The shell may restore these automatically:

- window shells and zone openness
- pane topology and visible placeholders
- titles, cwd hints, and non-mutating transcripts or outputs
- focus chain and visible inspector state
- safe-bounds remaps after display changes

The shell must revalidate these explicitly:

- remote routes and callback ownership
- credentials, trust-store state, and policy or entitlement state
- terminal, debug, notebook, collaboration, and other live control
- browser handoff sessions, approval tickets, and trust-changing flows

## Notes for later implementation

- The platform adapter reports facts and failures; it does not decide
  whether a protected action may bypass preview, approval, or trust
  review.
- A degraded input or accessibility path should narrow only the
  affected surface or claim row whenever the rest of the shell can
  remain truthful and usable.
- Multi-window and platform-conformance work should cite the exact row
  in this table rather than rephrasing the recovery story locally.
