# M5 Provider-Account / Offline-Capture Component Surface Certification

- Packet: `m5-provider-account-offline-capture-component-certification:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-provider-account-offline-capture-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Lineage preserved on every surface: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:work-item-detail** — surface=work_item_detail claimed=provider_committed certified=provider_committed status=green narrowed_axes=0 lineage_preserved=true
- **cert:status-transition-review** — surface=status_transition_review claimed=provider_committed certified=provider_committed status=green narrowed_axes=0 lineage_preserved=true
- **cert:support-export** — surface=support_export claimed=reviewable_projection certified=reviewable_projection status=green narrowed_axes=0 lineage_preserved=true
- **cert:issue-intake** — surface=issue_intake claimed=reviewable_projection certified=reviewable_projection status=green narrowed_axes=0 lineage_preserved=true
- **cert:provider-settings** — surface=provider_settings claimed=provider_committed certified=limited_scope_projection status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:browser-handoff** — surface=browser_handoff claimed=provider_committed certified=stale_session_projection status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:docs-help** — surface=docs_help claimed=provider_committed certified=policy_blocked_mapping status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:cli-headless** — surface=cli_headless claimed=provider_committed certified=local_only_packet status=yellow narrowed_axes=1 lineage_preserved=true
