# Desktop handoff, file association, protocol-handler ownership, embedded auth-return-path, and system-browser default conformance — release evidence

Reviewer-facing evidence packet for the lane that finalizes **desktop handoff,
file association, protocol-handler ownership, embedded auth-return-path, and
system-browser default conformance** on claimed-stable desktop shell surfaces:
one canonical record per handoff posture that binds typed target intent,
explicit handler ownership across side-by-side channels, system-browser default
conformance, trust / profile / tenant review ahead of widened authority,
truthful recovery for moved / removable / network / missing targets, per-OS
conformance, a public claim ceiling, an automatic narrow-below-Stable verdict,
recovery and route parity across the activity center / command palette / status
bar / menus, accessibility across normal / high-contrast / zoomed layouts, and
postures that stay available without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded/`](../../../fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded/)
- Schema: [`/schemas/ux/finalize-desktop-handoff-file-association-protocol-handler-embedded.schema.json`](../../../schemas/ux/finalize-desktop-handoff-file-association-protocol-handler-embedded.schema.json)
- Companion doc: [`/docs/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded.md`](../../../docs/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded.md)
- Typed source: `aureline_shell::desktop_handoff_conformance_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_desktop_handoff_conformance_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/desktop_handoff_conformance_stable_fixtures.rs`
- Projected from: `aureline_shell::platform_integration` (native desktop contract packet), `aureline_shell::deeplink::native_handoff`, `aureline_shell::system_browser_return_paths`

## The claimed-stable matrix

| Record | Entry path | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `file_association_stable.json` | file_association | **stable** | stable | — |
| `protocol_handler_owned_stable.json` | protocol_handler | **stable** | stable | — |
| `system_open_workspace_stable.json` | system_open | **stable** | stable | — |
| `system_browser_auth_return_stable.json` | default_browser_auth_callback | **stable** | stable | — |
| `reveal_in_shell_read_only_stable.json` | reveal_in_system_shell | **stable** | stable | — |
| `recent_item_reopen_stable.json` | recent_item_reopen | **stable** | stable | — |
| `jump_list_action_stable.json` | jump_list_action | **stable** | stable | — |
| `native_save_boundary_stable.json` | native_open_save | **stable** | stable | — |
| `removable_volume_recovery_stable.json` | removable_volume_reopen | **stable** | stable | — |
| `network_share_recovery_stable.json` | network_share_reopen | **stable** | stable | — |
| `device_code_exception_disclosed_stable.json` | default_browser_auth_callback | **stable** | stable | — |
| `help_about_preview_surface.json` | file_association | preview (narrowed) | preview | `surface_not_yet_stable` |
| `last_writer_wins_ownership_drill.json` | protocol_handler | beta (narrowed) | stable | `handler_ownership_not_explicit` |
| `embedded_browser_no_exception_drill.json` | default_browser_auth_callback | beta (narrowed) | stable | `auth_not_system_browser_default` |

Coverage verdict: **11 Stable, 3 narrowed**, covering all ten required entry
paths. Each narrowed row names a reason and drops below the launch cutline rather
than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **OS-level entry and return paths stay bound to typed target intent.** Every
  record carries `intent` with the literal target, the source-locator /
  deep-link intent (`deep_link_intent_ref`), the requested action, the resulting
  mode, the canonical object identity, and `preserves_literal_target` /
  `preserves_typed_intent` / `preserves_resulting_mode` /
  `no_generic_shell_reopen`. Guarded by `claim_ceiling_never_overclaims` and the
  builder's typed-intent pillar derivation; `system_open_workspace_stable.json`
  proves a system open resolves the exact workspace, not a generic home.
- **Handler ownership is visible for side-by-side Stable / Preview / Beta /
  portable installs.** `handler_ownership.side_by_side_channels[]` enumerates all
  five channels, `owning_channel_ref` / `owner_build_ref` are explicit, and
  `no_last_writer_wins` / `spoof_resistant` hold. The
  `last_writer_wins_ownership_drill.json` proves the lane detects a side-by-side
  install silently stealing the protocol handler and narrows below Stable —
  guarded by `handler_ownership_enumerates_side_by_side_channels`.
- **Claimed identity and auth rows default to system-browser handoff; exceptions
  are surfaced explicitly.** `auth_default` carries `default_to_system_browser`,
  `system_browser_default`, `exception_class`, and, for an exception, its
  `exception_scope_ref`, `return_path_ref`, and `recovery_on_exception_ref`.
  `system_browser_auth_return_stable.json` is the system-browser default,
  `device_code_exception_disclosed_stable.json` discloses an admin device-code
  exception (still Stable), and `embedded_browser_no_exception_drill.json` is an
  embedded web view with no disclosed exception — narrowed below Stable. Guarded
  by `auth_rows_default_to_system_browser_or_disclose_exception`.
- **OS open, protocol, and auth-return flows preserve literal target, trust
  posture, and safe recovery.** `trust_review.trust_profile_tenant_checked` and
  `no_silent_authority_widening` hold on every Stable row; the protocol-handler
  posture proves a trust review runs before widened authority.
- **Per-OS conformance covers default-browser auth return, protocol-handler
  ownership, file-association routing, side-by-side channel ownership,
  reveal-in-shell, recent-item / jump-list reopen, removable-root recovery, and
  network-share recovery.** `platform_conformance[]` carries macOS, Windows, and
  Linux rows with current proof on every record — guarded by
  `per_os_conformance_is_complete_with_proof`.
- **Removable volumes, network shares, and missing roots render recoverable
  placeholders.** `removable_volume_recovery_stable.json` (missing/unmounted) and
  `network_share_recovery_stable.json` (remote-unreachable) carry
  `placeholder_required`, a `last_seen_identity_ref`, an
  `unsaved_local_state_posture_token`, and the `locate_target` /
  `open_cached_context` / `close_placeholder` actions — guarded by
  `moved_or_missing_targets_render_truthful_placeholders`.
- **Native open/save/reveal surface canonical path, write posture, and
  profile/remote boundary.** `reveal_in_shell_read_only_stable.json` (read-only
  posture) and `native_save_boundary_stable.json` (remote profile share,
  overwrite-with-review) carry `canonical_target_path_label`,
  `write_posture_token`, and `profile_remote_boundary_note`.
- **Any surface lacking stable qualification is automatically narrowed.**
  `help_about_preview_surface.json` proves every handoff pillar but narrows to
  Preview by its lowest binding surface marker — guarded by
  `narrowed_rows_drop_below_cutline_and_name_a_reason`.
- **Discover / operate / recover from keyboard and mouse without account or
  managed services.** `routes[]` reaches the activity center, command palette,
  status bar, and a menu command keyboard-first; `recovery_routes[]` is complete
  and keyboard reachable; `accessibility` holds across normal / high-contrast /
  zoomed; `available_without_account` and `available_without_managed_services`
  are true on every record.

## How to reproduce

```sh
# Stable corpus index — scenario id, entry path, claim, surface marker.
cargo run -q -p aureline-shell \
  --bin aureline_shell_desktop_handoff_conformance_stable -- index

# Per-record plaintext truth block (support-export shape).
cargo run -q -p aureline-shell \
  --bin aureline_shell_desktop_handoff_conformance_stable -- plaintext

# Refresh the on-disk fixtures.
cargo run -q -p aureline-shell \
  --bin aureline_shell_desktop_handoff_conformance_stable -- emit-fixtures \
  fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded

# Replay + invariant gate.
cargo test -p aureline-shell --test desktop_handoff_conformance_stable_fixtures
```

## Scope ceiling

This packet certifies the desktop handoff-conformance lane only. It does not
widen public scope beyond the postures and entry paths enumerated above; a
posture that proves a narrower claim than planned downgrades and names the reason
in its record instead of inheriting an adjacent green row.
