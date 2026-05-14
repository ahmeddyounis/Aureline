# Desktop Continuity and Interruption Alpha

This page is the reviewer-facing entry point for the bounded native desktop
continuity alpha. It composes existing shell and auth contracts so OS entry,
display topology changes, sleep/wake, removable-root loss, and credential-store
interruptions share one inspectable packet.

## Runtime Sources

- `crates/aureline-shell/src/desktop_continuity_alpha.rs` mints the packet and
  support/export projection.
- `crates/aureline-shell/src/deeplink/native_handoff.rs` remains canonical for
  system-open, file-association, auth-callback, dock/taskbar recent, jump-list,
  and native file-affordance review rows.
- `crates/aureline-shell/src/windowing/display_safety.rs` remains canonical for
  mixed-DPI and safe-bounds adjustment logs.
- `crates/aureline-auth/src/secrets/mod.rs` remains canonical for credential
  store locked, unavailable, and trust-store-changed continuity states.

Runtime evidence is written to:

- `.logs/recovery/desktop_continuity_alpha_latest.json`
- `.logs/recovery/native_boundary_handoff_latest.json`

The governed matrix is
`artifacts/ux/desktop_continuity_alpha_matrix.yaml`.

## Required Truth

OS-originated entry must state the literal target, resulting mode, owning
channel/build, canonical target identity, trust/profile boundary, and recovery
path before high-risk work continues. Summary-only surfaces such as dock,
taskbar, jump-list, and notification entry cannot run mutating or privileged
actions directly.

Unavailable paths degrade to recovery cards, not generic failure. The alpha
packet carries Locate, Reconnect, Open cached context, Export context, and Close
placeholder choices where the target is missing, moved, unmounted, stale, or
requires review.

Display topology rows prefer visible recovery over pixel replay. Display detach,
mixed-DPI moves, and fullscreen or snapped restore drift preserve pane/focus
intent, move windows into safe bounds, and record when placement fidelity
downgrades to `compatible_restore`.

Sleep, wake, and network transitions use visible state tokens such as
`reconnecting`, `stale`, `resume_review_needed`, and `local_fallback`. They do
not rerun commands, reattach debug or remote authority, replay callbacks, or
reapply writes without explicit user intent.

Credential-store interruption rows name the store class, unlock state,
affected capability classes, denial reason, and safe retry/recovery actions.
The rows explicitly forbid plaintext downgrade, silent in-memory promotion, and
stale authority reuse.

## Support Export

`DesktopContinuitySupportExport` is metadata-only. Each row carries:

- interruption cause token;
- continuity state tokens;
- recovery choice tokens;
- resulting fidelity token;
- object identity or target ref;
- source evidence ref.

Support tooling can reconstruct cause, state, recovery, and fidelity from those
fields without scraping rendered UI text. Raw secret values and raw runtime
handle ids are excluded.

## Fixture Coverage

- `fixtures/ux/desktop_continuity_alpha/manifest.yaml`
- `fixtures/ux/desktop_continuity_alpha/support_export_reconstruction.json`
- `fixtures/workspace/desktop_continuity_alpha/mixed_dpi_restore_visible.json`
- `fixtures/auth/credential_store_interruption_alpha/manifest.yaml`
- `fixtures/auth/credential_store_interruption_alpha/interruption_support_projection.json`

The fixtures quote upstream platform and auth fixtures rather than copying their
vocabularies. If native handoff, window display, restore, notification, or
secret-broker vocabulary changes, update the upstream contract first and then
update this matrix and fixture packet in the same change.
