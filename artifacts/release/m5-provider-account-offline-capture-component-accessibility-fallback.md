# M5 Provider-Account / Offline-Capture Component Accessibility & Auto-Narrowing

- Packet: `m5-provider-account-offline-capture-component-accessibility-fallback:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Families: 5 certified across 5 / 5 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:provider-account-row-scope-limited** (provider_account_row) — family=provider_account_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=limited_scope_projection status=narrowed_disclosed
  - Auto-narrow: provider_committed → limited_scope_projection (dimension=connection_and_scope, trigger=write_scope_unstated) — Granted write scope is narrower than a full commit — shown as a limited-scope projection with its provider identity, tenant scope, and connection state still preserved, never as a fully committed account
- **a11y:provider-account-row-session-stale** (provider_account_row) — family=provider_account_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=stale_session_projection status=narrowed_disclosed
  - Auto-narrow: provider_committed → stale_session_projection (dimension=connection_and_scope, trigger=connection_state_unstated) — Session has expired and only a cached read remains — shown as a stale-session projection that must re-authenticate before it is trusted as live, never as a committed account
- **a11y:project-or-board-mapping-row-policy-blocked** (project_or_board_mapping_row) — family=project_or_board_mapping_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=policy_blocked_mapping status=narrowed_disclosed
  - Auto-narrow: provider_committed → policy_blocked_mapping (dimension=mapping_origin, trigger=mapping_origin_unstated) — Default-destination mapping is held by policy and cannot resolve a committed target — shown as a policy-blocked mapping that names the origin and policy source, never a resolved destination
- **a11y:sync-behavior-row** (sync_behavior_row) — family=sync_behavior_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=provider_committed status=parity
- **a11y:offline-capture-row** (offline_capture_row) — family=offline_capture_row keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=provider_committed effective_claim=local_only_packet status=narrowed_disclosed
  - Auto-narrow: provider_committed → local_only_packet (dimension=offline_capture, trigger=offline_capture_state_unstated) — Captured packet is held on this machine and nothing has been published — shown as a local-only packet with its queued-draft count and publish-later route preserved, never as a committed provider write
- **a11y:privacy-redaction-row** (privacy_redaction_row) — family=privacy_redaction_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_projection effective_claim=reviewable_projection status=parity
