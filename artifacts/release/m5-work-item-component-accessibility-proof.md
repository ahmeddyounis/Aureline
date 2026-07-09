# M5 Work-Item Component Accessibility & Auto-Narrowing

- Packet: `m5-work-item-component-accessibility-parity:stable:0001`
- As of: `2026-07-09T00:00:00Z`
- Families: 8 certified across 8 / 8 frozen families
- Status: 4 green / 4 yellow / 0 red

## Rows

- **a11y:work-item-row-freshness-stale** (work_item_row) — family=work_item_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=stale_freshness_projection status=narrowed_disclosed
  - Auto-narrow: provider_committed → stale_freshness_projection (dimension=provider_freshness, trigger=local_versus_provider_state_hidden) — Provider projection has gone stale and only a cached read remains — shown as a stale-freshness projection that must refresh before it is trusted as live, with its canonical ID and provider authority still preserved, never as a live committed work item
- **a11y:provider-chip-group-write-scope-blocked** (provider_chip_group) — family=provider_chip_group keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=read_only_projection status=narrowed_disclosed
  - Auto-narrow: provider_committed → read_only_projection (dimension=write_scope, trigger=provider_authority_unstated) — Granted write scope is read-only or held by policy — shown as a read-only projection that names its provider authority and policy source, never as a committed write path
- **a11y:relation-strip** (relation_strip) — family=relation_strip keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_projection effective_claim=reviewable_projection status=parity
- **a11y:sync-pending-pill-local-only** (sync_pending_pill) — family=sync_pending_pill keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=local_only_projection status=narrowed_disclosed
  - Auto-narrow: provider_committed → local_only_projection (dimension=sync_state, trigger=sync_pending_state_hidden) — Change is held on this machine and nothing has been synced to the provider — shown as a local-only projection with its queued-draft count and publish-later route preserved, never as a committed provider write
- **a11y:work-item-detail-header** (work_item_detail_header) — family=work_item_detail_header keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=provider_committed status=parity
- **a11y:status-transition-sheet** (status_transition_sheet) — family=status_transition_sheet keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=provider_committed status=parity
- **a11y:related-evidence-card** (related_evidence_card) — family=related_evidence_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_projection effective_claim=reviewable_projection status=parity
- **a11y:offline-handoff-packet-card-unpublishable** (offline_handoff_packet_card) — family=offline_handoff_packet_card keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=unpublishable_packet_projection status=narrowed_disclosed
  - Auto-narrow: provider_committed → unpublishable_packet_projection (dimension=packet_publishability, trigger=publish_later_continuity_hidden) — Captured handoff packet cannot publish safely and nothing has been handed off — shown as an unpublishable-packet projection with its destination, export boundary, and retry-or-export route preserved, never as a committed provider write
