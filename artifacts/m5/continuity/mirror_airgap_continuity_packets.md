# Artifact: mirror-only and air-gapped continuity packets

**Contract ref:** `continuity:m5_mirror_airgap_continuity_packets:v1`  
**Schema:** `schemas/continuity/mirror_airgap_packet.schema.json`  
**Doc:** `docs/m5/continuity/mirror-airgap-continuity.md`  
**Runtime owner:** `aureline_continuity::m5_mirror_airgap_continuity_packets`

## Qualification

| Condition | Status |
|---|---|
| Every offline row declares trust-root continuity that survives offline | ✓ Stable |
| Every mirror-backed row names a mirror and a current freshness window | ✓ Stable |
| Offline import/export paths disclosed on every offline row | ✓ Stable |
| Advisory/revocation source disclosed on every offline row | ✓ Stable |
| Public fallback stated as prohibited, unavailable, or policy-gated | ✓ Stable |
| Every claimed offline row points to a current packet | ✓ Stable |
| Surface fact reuse complete + vocabulary identical | ✓ Stable |
| No silent public fallback, no live public advisory fetch on isolated rows | ✓ Stable |
| **Overall** | **Stable** |

## Packets

| Surface | Profile | Posture | Trust root | Mirror freshness | Advisory source | Public fallback |
|---|---|---|---|---|---|---|
| Mirror-only self-hosted package and policy registry | `self_hosted` | `mirror_only` | `customer_managed_trust_root` | `fresh_within_window` | `mirror_replicated` | `prohibited` |
| Air-gapped sovereign deployment boundary | `sovereign` | `air_gapped` | `offline_trust_root` | `not_applicable` | `offline_bundle` | `unavailable` |
| Self-hosted deployment with restricted egress | `self_hosted` | `self_hosted_restricted` | `customer_managed_trust_root` | `stale_within_grace` | `mirror_replicated` | `policy_gated` |
| Local desktop core continuity | `local_only` | `local_only` | `os_store_trust_root` | `not_applicable` | `local_cache_only` | `not_applicable` |

## Offline exchange and trust-root renewal

| Posture | Offline import | Offline export | Trust-root renewal |
|---|---|---|---|
| `mirror_only` | `mirror_pull_push` | `mirror_pull_push` | `customer_operated_rotation` |
| `air_gapped` | `signed_offline_bundle` | `physical_media_transfer` | `offline_signed_rotation` |
| `self_hosted_restricted` | `mirror_pull_push` | `signed_offline_bundle` | `mirror_replicated_rotation` |
| `local_only` | `not_applicable` | `not_applicable` | `customer_operated_rotation` |

## Claim-narrowing cases

Each case mutates one seeded packet and shows the claim narrowing automatically:

- `case_silent_public_fallback_withdrawn` — a mirror-only row silently falls back
  to public endpoints → **withdrawn** (`silent_public_fallback`)
- `case_advisory_live_public_fetch_withdrawn` — an air-gapped row sources
  advisories from a live public fetch → **withdrawn**
  (`advisory_implies_live_public_fetch`)
- `case_trust_root_breaks_offline_withdrawn` — a mirror-only trust root requires a
  live public reissue to renew → **withdrawn** (`trust_root_breaks_offline`)
- `case_public_fallback_undisclosed_preview` — a self-hosted-restricted row does
  not state its public-fallback policy → **preview**
  (`public_fallback_undisclosed`)
- `case_trust_root_undeclared_preview` — a mirror-only row does not declare its
  trust-root continuity → **preview** (`trust_root_continuity_undeclared`)
- `case_mirror_never_synced_preview` — a mirror-only row whose mirror has never
  synced → **preview** (`mirror_never_synced`)
- `case_packet_evidence_missing_preview` — a claimed air-gapped row carries no
  packet → **preview** (`packet_evidence_missing`)
- `case_mirror_stale_needs_sync_beta` — a mirror-only mirror has aged out and
  needs a fresh sync → **beta** (`mirror_freshness_stale`)

## Canonical evidence packets

- `artifacts/m5/continuity/mirror_airgap/mirror_airgap_continuity_page.json`
- `artifacts/m5/continuity/mirror_airgap/offline_continuity_registry.json`
- `artifacts/m5/continuity/mirror_airgap/mirror_airgap_continuity_support_export.json`

## Fixture references

- `fixtures/continuity/mirror_airgap_cases/page.json`
- `fixtures/continuity/mirror_airgap_cases/summary.json`
- `fixtures/continuity/mirror_airgap_cases/registry.json`
- `fixtures/continuity/mirror_airgap_cases/support_export.json`
- `fixtures/continuity/mirror_airgap_cases/case_silent_public_fallback_withdrawn.json`
- `fixtures/continuity/mirror_airgap_cases/case_advisory_live_public_fetch_withdrawn.json`
- `fixtures/continuity/mirror_airgap_cases/case_trust_root_breaks_offline_withdrawn.json`
- `fixtures/continuity/mirror_airgap_cases/case_public_fallback_undisclosed_preview.json`
- `fixtures/continuity/mirror_airgap_cases/case_trust_root_undeclared_preview.json`
- `fixtures/continuity/mirror_airgap_cases/case_mirror_never_synced_preview.json`
- `fixtures/continuity/mirror_airgap_cases/case_packet_evidence_missing_preview.json`
- `fixtures/continuity/mirror_airgap_cases/case_mirror_stale_needs_sync_beta.json`
