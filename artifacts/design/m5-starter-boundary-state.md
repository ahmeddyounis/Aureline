# Starter boundary states

- Packet: `m5-starter-boundary-state-controls:stable:0001`
- Surface: `M5 starter boundary states: mirror-only, offline-cache-only, sign-in-required, remote/managed-workspace, and non-durable temporary-staging honesty with no silent trust or install across claimed scaffold surfaces`
- Starter boundary states: 6 (3 blocked, 1 non-durable)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Starter boundary states

- **Public registry starter** — boundary `public_registry` → `direct_public_access`, availability `available` → `reachable_now`, owner `first_party_registry`, freshness `live`, source `first_party_starter`, deep link `template_manifest`
- **Mirror-only starter** — boundary `mirror_only` → `mirror_mediated`, availability `mirror_reachable_only` → `reachable_via_mirror`, owner `team_mirror`, freshness `mirror_synced`, source `mirrored_starter`, deep link `starter_registry_entry`
- **Offline-cache-only starter** — boundary `offline_cache_only` → `offline_cache_backed`, availability `cache_only_offline` → `reachable_from_cache`, owner `local_cache`, freshness `cache_stale`, source `local_only_starter`, deep link `docs_anchor`
- **Sign-in-required starter** — boundary `sign_in_required` → `auth_gated`, availability `sign_in_pending` → `blocked_pending_sign_in`, owner `managed_service`, freshness `freshness_unknown`, source `team_managed_starter`, deep link `policy_reference`
- **Managed-workspace starter** — boundary `remote_or_managed_workspace` → `managed_remote`, availability `provisioning_pending` → `blocked_pending_provisioning`, owner `managed_service`, freshness `live`, source `team_managed_starter`, deep link `policy_reference`
- **Non-durable temporary-staging starter** — boundary `non_durable_temp_staging` → `non_durable_staging`, availability `unavailable` → `not_reachable`, owner `unknown_owner`, freshness `ephemeral`, source `unknown_source_starter`, deep link `docs_anchor`
