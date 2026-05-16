# Desktop entry ownership

This page is the reviewer entry point for the desktop-entry ownership
audit that lives in
[`crates/aureline-install/src/ownership_audit/`](../../../crates/aureline-install/src/ownership_audit/)
and is wired into the live shell through
[`crates/aureline-shell/src/ownership_audit/`](../../../crates/aureline-shell/src/ownership_audit/)
plus the headless inspector binary
`aureline_shell_ownership_audit`. The reviewer-facing projection of
the audit rows is checked in at
[`artifacts/ux/m3/protocol_handler_audit.md`](../../../artifacts/ux/m3/protocol_handler_audit.md).

The audit answers three reviewer questions on every OS-level handoff
surface:

1. **Which build owns this surface today?** Each row carries a typed
   [`OwnerVerdictClass`] (selected owner, candidate only, not
   registered, admin policy owned, managed fleet owned, displaced
   owner) so install review, About, diagnostics, CLI, and support
   export agree on the answer without scraping installer logs.
2. **What happens when side-by-side installs coexist?** Each row
   lists the coexisting channels and at least one
   [`SideBySideDisclosureClass`] token (per-channel suffixed scheme,
   user/admin selection never last-writer-wins, channel-owner summary
   in review, handler-owner change previewed before commit, portable
   does not steal installed ownership, managed owner shown but not
   overrideable). Last-writer-wins is closed out by the validator.
3. **Are deep links and file opens going through the same checks as
   in-product invocation?** Every dispatching row enumerates the
   [`DeepLinkRouteCheckClass`] tokens the validator applies
   (origin trust, reviewed-sheet preview, target/workspace scope,
   single-use replay, handler-ownership verification) and asserts the
   in-product invocation runs the same family.

## Acceptance posture

The audit delivers the M3 desktop-entry acceptance gates:

- **Which channel/build owns each OS-level handoff surface.** Every
  row sets `owner_verdict`, `selected_owner_channel`, and
  `candidate_owner_channels` against the upstream install topology
  row; side-by-side rows name the disclosure tokens that keep the
  surface out of last-writer-wins behavior. The validator rejects a
  side-by-side row whose disclosure list is empty or whose
  `silent_steal_blocked` flag is false.
- **What happens when side-by-side installs coexist.** Each
  coexisting layout row lists its coexisting channels and at least
  one real disclosure token. The portable row contributes
  `portable_does_not_steal_installed_ownership`; the managed row
  contributes `managed_owner_shown_not_overrideable`; the
  side-by-side preview row contributes
  `handler_owner_change_preview_before_commit` and the explicit
  selection rule. The audit row
  `ownership.windows.displaced_preview.protocol_handler` carries a
  ref to the topology stale-handler diagnostic
  `install.handler.diagnostic.windows.displaced_stable_owner` so the
  displaced-owner case is diagnosable without installer logs.
- **Deep links and file opens route through the same trust, preview,
  and scope checks as in-product invocation.** Every dispatching row
  lists the five deep-link route checks the live validator in
  [`crates/aureline-shell/src/deeplink/`](../../../crates/aureline-shell/src/deeplink/)
  applies and sets `in_product_invocation_uses_same_checks=true`. The
  validator rejects a dispatching row that drops the parity flag or
  the route-check list. Non-dispatching surfaces (recent-item
  registration) are exempt and explicitly recorded.
- **Portable mode and managed installs disclose their limits instead
  of silently stealing ownership.** Portable rows must set
  `portable_claim=never_claims_host_global_ownership` and
  `owner_verdict=not_registered`; managed rows must disclose
  `admin_policy_owns_handler`, `managed_ring_owns_handler`, or
  `user_visible_not_overrideable`. The validator rejects a portable
  row that claims a selected owner and a managed row that drops the
  managed disclosure.

## Bounded scope

The audit is a typed projection over the install-topology alpha
packet. It does not install, update, repair, rollback, register OS
entry points, or mutate desktop state. It exists to make the claimed
ownership truth inspectable and testable before any installer
mutation primitive ships.

## Cross-surface parity

The packet exposes two projections that always agree on the per-row
truth:

- `OwnershipAuditPacket::surface_projection()` for product surfaces
  (About, update, diagnostics, install review, CLI).
- `OwnershipAuditPacket::support_export_projection()` for the
  metadata-safe support-export wrapper (no paths or secrets).

Both projections carry the same per-row owner verdict, disclosure
tokens, deep-link route checks, parity flag, and (when present)
displaced-owner diagnostic ref. The shell side mirrors this with
`load_seeded_ownership_audit_packet`,
`seeded_ownership_audit_surface_projection`, and
`seeded_ownership_audit_support_export` so the live shell, the
headless inspector binary, and the install crate's fixture tests all
read the same checked-in fixture.

## Verification

```bash
cargo test -p aureline-install
cargo test -p aureline-shell --lib ownership_audit
cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- packet
cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- surface
cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- validate
```

The aureline-install tests cover packet validation, portable /
managed / displaced-owner posture, side-by-side disclosure
requirements, deep-link route-check parity, and the surface /
support-export round-trip. The aureline-shell tests cover the
fixture-loaded validation and the metadata-safe support-export
wrapper. The headless inspector binary lets reviewers and support
tooling inspect the audit without standing up the live shell.

[`OwnerVerdictClass`]: ../../../crates/aureline-install/src/ownership_audit/mod.rs
[`SideBySideDisclosureClass`]: ../../../crates/aureline-install/src/ownership_audit/mod.rs
[`DeepLinkRouteCheckClass`]: ../../../crates/aureline-install/src/ownership_audit/mod.rs
