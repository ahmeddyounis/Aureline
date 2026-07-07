# M5 Provider-Settings Repair-Entrypoint Row Primitive

- Packet: `m5-provider-settings-repair-entrypoint-row:stable:0001`
- Label: `M5 provider-settings repair-entrypoint row primitive: boundary class (network-egress/auth-stale/auth-scope/mapping-broken/provider-incompatible/policy-blocked), account connection state, repair posture and concrete entrypoint, linked diagnostics (network-egress/auth-session/support-bundle/provider-compatibility/export-redaction), continuity guarantees (queued drafts/cached read/reviewed export/no blind credential re-entry), and bounded reveal/open-entrypoint/open-diagnostics/export-evidence/request-escalation actions`
- Provider-settings consumers: 5 (5 stable)
- Boundary classes: network_egress_blocked, auth_stale_session, auth_scope_limited, mapping_broken, provider_incompatible, policy_blocked
- Repair entrypoints: open_network_egress_diagnostics, open_reauth_handoff, open_scope_review, open_mapping_repair, open_compatibility_report, open_policy_review
- Linked diagnostics: network_egress_diagnostic, auth_session_diagnostic, support_bundle_diagnostic, provider_compatibility_diagnostic, export_redaction_diagnostic
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Provider-settings consumers

- **Provider-Account Row**: `stable`
  - Owner: Provider-account row owner
  - Scope: The provider-account row names the real boundary so a stale session reads as reauth-session and links to the reviewed reauth handoff — queued drafts and cached read intact, never a blind credential prompt — and a limited-scope session reads as widen-scope and links to the scope review, each with its support-bundle and export/redaction diagnostics one click away
  - Repair entrypoints:
    - `repair:acme-eng:reauth:1` (`auth_stale_session` / `stale_session`) → `reauth_session_row` via `open_reauth_handoff` (diagnostics `3`, blind-reentry `false`)
    - `repair:acme-eng:scope:1` (`auth_scope_limited` / `limited_scope`) → `widen_scope_row` via `open_scope_review` (diagnostics `3`, blind-reentry `false`)
- **Project / Board Mapping Row**: `stable`
  - Owner: Project/board mapping row owner
  - Scope: The mapping row names the real boundary so a broken mapping reads as remap-target and links to the mapping repair with queued drafts preserved, and an incompatible provider reads as compatibility-review and links to the compatibility report — both wired to the provider-compatibility diagnostic and the reviewed export path rather than a bare error
  - Repair entrypoints:
    - `repair:acme-eng:mapping:1` (`mapping_broken` / `signed_in`) → `remap_target_row` via `open_mapping_repair` (diagnostics `3`, blind-reentry `false`)
    - `repair:acme-eng:compat:1` (`provider_incompatible` / `signed_in`) → `compatibility_review_row` via `open_compatibility_report` (diagnostics `3`, blind-reentry `false`)
- **Sync-Behavior Row**: `stable`
  - Owner: Sync-behavior row owner
  - Scope: The sync-behavior row names the real boundary so a network/egress block reads as network-egress-repair and links to the network diagnostics while the offline cached read keeps working and queued drafts stay put, and an incompatible provider reads as compatibility-review — the row is never an isolated sidebar divorced from the network and compatibility diagnostics
  - Repair entrypoints:
    - `repair:acme-eng:network:1` (`network_egress_blocked` / `offline_cached_read`) → `network_egress_repair_row` via `open_network_egress_diagnostics` (diagnostics `3`, blind-reentry `false`)
    - `repair:acme-eng:compat:2` (`provider_incompatible` / `signed_in`) → `compatibility_review_row` via `open_compatibility_report` (diagnostics `3`, blind-reentry `false`)
- **Privacy / Redaction Row**: `stable`
  - Owner: Privacy/redaction row owner
  - Scope: The privacy/redaction row names the real boundary so a policy-blocked boundary reads as policy-blocked and offers only a reviewed escalation — never a self-serve bypass — while a stale session reads as reauth-session and links to the reviewed reauth handoff with queued drafts intact, both keeping the reviewed export path and support-bundle diagnostics one click away
  - Repair entrypoints:
    - `repair:acme-eng:policy:1` (`policy_blocked` / `policy_blocked`) → `policy_blocked_row` via `open_policy_review` (diagnostics `3`, blind-reentry `false`)
    - `repair:acme-eng:reauth:2` (`auth_stale_session` / `stale_session`) → `reauth_session_row` via `open_reauth_handoff` (diagnostics `3`, blind-reentry `false`)
- **Provider Status Bar**: `stable`
  - Owner: Provider status bar owner
  - Scope: The provider status bar names the real boundary so a network/egress block reads as network-egress-repair with queued drafts and cached read preserved and links to the network diagnostics, and a policy-blocked boundary offers only a reviewed escalation — a user can tell the boundary, entrypoint, and preserved work from the bar alone, never retry-login folklore
  - Repair entrypoints:
    - `repair:acme-eng:network:2` (`network_egress_blocked` / `offline_cached_read`) → `network_egress_repair_row` via `open_network_egress_diagnostics` (diagnostics `3`, blind-reentry `false`)
    - `repair:acme-eng:policy:2` (`policy_blocked` / `policy_blocked`) → `policy_blocked_row` via `open_policy_review` (diagnostics `3`, blind-reentry `false`)
