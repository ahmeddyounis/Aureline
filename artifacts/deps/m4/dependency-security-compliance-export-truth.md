# Proof packet: Dependency, security, compliance, and export truth

Artifact: `artifacts/deps/m4/dependency-security-compliance-export-truth.json`

## Purpose

This packet is the canonical dependency-security-compliance export truth for the
M4 stable line. It ties SBOM references, notices, advisories, suppressions, and
review decisions back to an exact build context. It distinguishes `No active
findings` from `No current feed data`, preserves suppression
actor/reason/scope/expiry/reopen behavior, and produces redaction-safe
projections for UI, CLI, support bundles, release packets, and public proof.

## Vocabulary

| Domain | Tokens |
|---|---|
| Advisory source class | `live_public_feed`, `enterprise_mirror`, `imported_report`, `stale_local_cache`, `offline_bundle` |
| Advisory freshness | `current`, `stale`, `missing`, `mirror_only`, `feed_outage` |
| Advisory severity | `low`, `moderate`, `high`, `critical`, `unknown` |
| Suppression state | `not_suppressed`, `active_time_bound`, `expired_reopened`, `policy_locked` |
| License-review posture | `approved`, `approved_with_notice`, `review_required`, `denied_by_policy`, `unknown_requires_review` |
| Notice source | `spdx_sbom`, `human_readable_notice`, `reuse_compliant`, `import_manifest`, `missing` |
| Export scope | `ui_inspection`, `cli_headless`, `support_bundle`, `release_packet`, `public_proof` |
| Lockfile-risk class | `resolved_exact`, `policy_pinned`, `out_of_date`, `unresolved`, `vulnerable` |
| Findings state | `no_active_findings`, `no_current_feed_data`, `findings_present`, `feed_outage` |

## Build context

- **Build id**: `build-id:aureline:dev:0.0.0:x86_64-apple-darwin:dev:abc1234`
- **Workspace scope**: `Cargo.toml`
- **Profile**: `dev`
- **Lockfile fingerprint**: `sha256:cargo-lock:abc1234def5678`

## Advisory rows

| Advisory id | Source | Freshness | Severity | Findings state | Suppression |
|---|---|---|---|---|---|
| `advisory:rustsec-2024-0001` | live_public_feed | current | high | findings_present | Active time-bound |
| `advisory:mirror-2024-001` | enterprise_mirror | mirror_only | moderate | findings_present | Policy locked |
| `advisory:imported-2024-002` | imported_report | current | low | no_active_findings | — |
| `advisory:stale-cache-2024-003` | stale_local_cache | stale | unknown | no_current_feed_data | Expired reopened |
| `advisory:offline-2024-004` | offline_bundle | feed_outage | critical | feed_outage | — |
| `advisory:clean-scan-2024-005` | live_public_feed | current | unknown | no_active_findings | — |
| `advisory:missing-feed-2024-006` | live_public_feed | missing | unknown | no_current_feed_data | — |

## Suppression rows

| Suppression id | State | Actor | Scoped to | Expires | Reopens on expiry |
|---|---|---|---|---|---|
| `suppression:transient_serde_review` | active_time_bound | team:security | advisory:rustsec-2024-0001 | 2026-06-16 | Yes |
| `suppression:platform_policy_lock` | policy_locked | team:platform | advisory:mirror-2024-001 | — | No |
| `suppression:expired_reopen_example` | expired_reopened | team:security | advisory:stale-cache-2024-003 | 2026-05-15 (expired) | Yes |
| `suppression:not_suppressed_example` | not_suppressed | system:auto | advisory:offline-2024-004 | — | — |

## License/notice rows

| Row id | Package | Posture | Notice source | SPDX |
|---|---|---|---|---|
| `license:first_party_workspace` | workspace | approved | spdx_sbom | Apache-2.0 |
| `license:serde` | serde@1.0.200 | approved_with_notice | human_readable_notice | MIT OR Apache-2.0 |
| `license:unknown_vendor` | vendor-crate@0.1.0 | unknown_requires_review | missing | — |
| `license:denied_legacy` | legacy-crate@0.9.0 | denied_by_policy | import_manifest | GPL-2.0 |
| `license:review_pending` | new-crate@0.2.0 | review_required | reuse_compliant | BSD-3-Clause |

## Lockfile-risk rows

| Row id | Package | Risk class | Resolved | Advisory refs |
|---|---|---|---|---|
| `risk:serde_resolved` | serde@1.0.200 | resolved_exact | 1.0.200 | rustsec-2024-0001 |
| `risk:tokio_pinned` | tokio@1.37.0 | policy_pinned | 1.37.0 | mirror-2024-001 |
| `risk:chrono_out_of_date` | chrono@0.4.38 | out_of_date | 0.4.38 | stale-cache-2024-003 |
| `risk:unresolved_optional` | optional-crate@0.3.0 | unresolved | — | — |
| `risk:openssl_vulnerable` | openssl@0.10.64 | vulnerable | 0.10.64 | offline-2024-004 |

## Export scopes targeted

- `ui_inspection`
- `cli_headless`
- `support_bundle`
- `release_packet`
- `public_proof`

## Summary

- 7 advisory rows (2 active, 2 no_active_findings, 2 no_current_feed_data, 1 feed_outage)
- 4 suppression rows (2 active, 1 expired reopened)
- 5 license/notice rows (2 blocking release)
- 5 lockfile-risk rows (3 actionable risk)

## Owner sign-off

- `team:release_engineering` — signed off 2026-06-02
- `team:security` — signed off 2026-06-02 for advisory and suppression rows
- `team:platform` — signed off 2026-06-02 for license and lockfile-risk rows
