# M5 Work-Item Component Surface Certification

- Packet: `m5-work-item-component-certification:stable:0001`
- As of: `2026-07-09T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-work-item-component-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Lineage preserved on every surface: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:work-item-detail** — surface=work_item_detail claimed=provider_committed certified=provider_committed status=green narrowed_axes=0 lineage_preserved=true
- **cert:status-transition-review** — surface=status_transition_review claimed=provider_committed certified=provider_committed status=green narrowed_axes=0 lineage_preserved=true
- **cert:work-item-inbox** — surface=work_item_inbox claimed=reviewable_projection certified=reviewable_projection status=green narrowed_axes=0 lineage_preserved=true
- **cert:support-export** — surface=support_export claimed=reviewable_projection certified=reviewable_projection status=green narrowed_axes=0 lineage_preserved=true
- **cert:incident-review** — surface=incident_review claimed=provider_committed certified=stale_freshness_projection status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:docs-help** — surface=docs_help claimed=provider_committed certified=read_only_projection status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:cli-headless** — surface=cli_headless claimed=provider_committed certified=local_only_projection status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:offline-handoff-export** — surface=offline_handoff_export claimed=provider_committed certified=unpublishable_packet_projection status=yellow narrowed_axes=1 lineage_preserved=true
