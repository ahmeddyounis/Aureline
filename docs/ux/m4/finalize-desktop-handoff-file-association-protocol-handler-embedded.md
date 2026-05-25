# Desktop handoff, file association, protocol-handler ownership, embedded auth-return-path, and system-browser default conformance — contract

This is the reviewer-facing companion for the stable lane that hardens
**OS-originated desktop entry and return paths** to Aureline's durable truth
model: one governed record per handoff posture that binds **typed target
intent**, **explicit handler ownership across side-by-side channels**,
**system-browser default conformance** for claimed-identity and auth rows,
**trust / profile / tenant review ahead of widened authority**, **truthful
recovery** for moved / removable / network / missing targets, **per-OS
conformance**, and a **public claim ceiling** with an automatic
narrow-below-Stable verdict.

This lane finalizes the beta native desktop contract packet
(`aureline_shell::platform_integration`) into a Stable governed record. Where
that packet asserts the beta promise for native desktop integration, this lane
proves that *every* claimed-stable entry path — a file association, a protocol
handler, a system open, a default-browser auth callback, a reveal-in-shell, a
recent-item or jump-list reopen, a removable-volume or network-share reopen, a
native open/save — resolves the literal requested target (or a truthful
placeholder), names which channel owns the handler, and defaults auth to the
system browser unless an exception is surfaced explicitly.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded/`](../../../fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded/)
- Schema:
  [`/schemas/ux/finalize-desktop-handoff-file-association-protocol-handler-embedded.schema.json`](../../../schemas/ux/finalize-desktop-handoff-file-association-protocol-handler-embedded.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded.md`](../../../artifacts/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded.md)
- Typed source: `aureline_shell::desktop_handoff_conformance_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_desktop_handoff_conformance_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/desktop_handoff_conformance_stable_fixtures.rs`

## Why one governed handoff record

An OS-originated entry path fuses problems competitors routinely conflate:
*which install owns the handler*, *what literal target was requested*, *whether
auth returns through the system browser or an embedded web view*, and *what
happens when the target is gone*. When these are fused, a side-by-side Preview
install silently steals the Stable channel's file association, a protocol handler
reopens a generic home pane instead of the deep-linked object, an embedded web
view swallows an auth callback with no disclosed exception, or a missing
removable root disappears instead of rendering a recoverable placeholder.

This lane mints one governed `desktop_handoff_conformance_record` per handoff
posture. It does **not** reinvent the entry-surface vocabulary, the
handler-ownership classes, the target-availability classes, or the
system-browser exception classes: each record is a genuine projection of the
live native desktop contract packet (`aureline_shell::platform_integration`),
the native-handoff vocabulary (`aureline_shell::deeplink::native_handoff`), and
the system-browser return-paths page
(`aureline_shell::system_browser_return_paths`). The record binds, for one
entry-path identity:

1. **Typed target intent.** The literal target label and ref, the source-locator
   / deep-link intent, the requested action, the resulting mode, and the
   canonical object identity are preserved end to end. A Stable posture proves
   `intent.no_generic_shell_reopen` — the path never reopens a generic shell or
   the wrong install.
2. **Handler ownership.** `handler_ownership.owning_channel_ref` and
   `owner_build_ref` are explicit; `side_by_side_channels[]` enumerates Stable,
   Preview, Beta, portable, and admin-managed; `no_last_writer_wins` and
   `spoof_resistant` hold. One install can never silently steal another's file
   association, protocol handler, auth callback, or recent-item registration.
3. **System-browser default conformance.** For claimed-identity / auth rows
   (`auth_default.applies`), the row either defaults to system-browser handoff
   (`default_to_system_browser` + `system_browser_default` + no exception + no
   embedded browser) or surfaces an explicit exception
   (`exception_class` + `exception_scope_ref` + `return_path_ref` +
   `recovery_on_exception_ref`). An embedded browser with no disclosed exception
   narrows the posture.
4. **Trust / profile / tenant review.** `trust_review.trust_profile_tenant_checked`
   and `no_silent_authority_widening` hold; review precedes any widened authority
   or resumed remote action.
5. **Truthful recovery.** When `recovery.availability` requires a placeholder
   (moved, missing, unmounted, remote-unreachable, …) the posture renders a
   recoverable placeholder with `last_seen_identity_ref`, an
   `unsaved_local_state_posture_token`, and the explicit `locate_target`,
   `open_cached_context`, and `close_placeholder` actions. No mutating work or
   stale authority replays silently. Native open/save/reveal surface the
   `canonical_target_path_label`, `write_posture_token`, and
   `profile_remote_boundary_note` when the target is not the local default.
6. **Per-OS conformance.** `platform_conformance[]` covers macOS, Windows, and
   Linux, each with current proof.
7. **A public claim ceiling and automatic narrowing.** `claim_ceiling.asserts_*`
   may never exceed the proven pillars, and a posture that cannot prove a pillar,
   or whose lowest binding surface marker is below Stable, narrows below Stable
   with a named `stable_qualification.narrowing_reasons[]` entry instead of
   inheriting an adjacent green row.

## Binding surfaces read the shared record

`surface_projections[]` enumerates the four binding surfaces that ingest this
record verbatim rather than cloning prose:

- `desktop_handoff_review` — the in-product desktop handoff-review surface.
- `cli_inspect` — the `aureline_shell_desktop_handoff_conformance_stable`
  headless inspector (`scenario`, `all`, `plaintext`, `index`).
- `help_about` — the Help/About handoff posture.
- `support_export` — the redacted diagnostics support export (the per-record
  `support_export_lines()` plaintext block).

The lowest binding-surface marker drives `surface_lifecycle_marker`; a binding
surface still in preview narrows the posture to Preview.

## The claimed-stable matrix

See the release-evidence packet for the full table. The matrix spans every
required entry path and a deliberate span of Stable and narrowed rows, including
two adversarial drills (a side-by-side last-writer-wins handler theft and an
embedded-browser auth capture) that the lane narrows below Stable with a named
reason.

## Reading a record

Each fixture is one `desktop_handoff_conformance_record`. Start from
`entry_path`, `stable_qualification` (claim class + narrowing reasons), and
`pillars`. The `intent`, `handler_ownership`, `auth_default`, `trust_review`, and
`recovery` blocks carry the per-pillar evidence; `platform_conformance[]` carries
the per-OS proof; `surface_projections[]`, `recovery_routes[]`, `routes[]`, and
`accessibility` carry the discover/operate/recover parity. `honesty_marker_present`
is set whenever there is anything narrowed, below-Stable, placeholder-backed, or
exception-bearing to disclose.

## Guardrails

No hover-only routes, no focus ambiguity, no toast-only truth, and no
hard-coded theme/state semantics. The lane does not widen public scope from this
row alone: if delivery proves a narrower claim than planned, the posture
downgrades and names the reason in the record rather than papering over the gap.
