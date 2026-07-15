# Shared Build-Lane-Trust Consumers: One Registry Across Surfaces

- Packet: `m5-build-lane-trust-shared-consumers:stable:0001`
- Surface: `M5 build-lane-trust shared consumers (one registry across surfaces)`
- Consumer bindings: 12 (5 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer bindings

- **Contributor / PR lane (reads shared caches, never publishes release artifacts)** [`bltsc-contributor-pr-build-farm`]: family `contributor_pr` on `build_farm`, representation `desktop_full`, role `cache_posture`
- **Contributor / PR lane (reads shared caches, never publishes release artifacts)** [`bltsc-contributor-pr-cache`]: family `contributor_pr` on `cache_service`, representation `desktop_full`, role `cache_posture`
- **Contributor / PR lane (reads shared caches, never publishes release artifacts)** [`bltsc-contributor-pr-cli`]: family `contributor_pr` on `cli_export`, representation `exported_redacted`, role `cache_posture`
- **Protected-merge lane (controlled credentials and verified caches only)** [`bltsc-protected-merge-release-center`]: family `protected_merge` on `release_center`, representation `desktop_full`, role `publication_authority`
- **Protected-merge lane (controlled credentials and verified caches only)** [`bltsc-protected-merge-shiproom`]: family `protected_merge` on `shiproom`, representation `desktop_full`, role `publication_authority`
- **Protected-merge lane (controlled credentials and verified caches only)** [`bltsc-protected-merge-diagnostics`]: family `protected_merge` on `diagnostics`, representation `remote_projected`, role `publication_authority`
- **Release lane (verified or re-materialized inputs converging on one exact build identity)** [`bltsc-release-provenance`]: family `release` on `provenance_service`, representation `desktop_full`, role `reproducibility_proof`
- **Release lane (verified or re-materialized inputs converging on one exact build identity)** [`bltsc-release-diagnostics`]: family `release` on `diagnostics`, representation `desktop_full`, role `reproducibility_proof`
- **Release lane (verified or re-materialized inputs converging on one exact build identity)** [`bltsc-release-support`]: family `release` on `support_export`, representation `exported_redacted`, role `reproducibility_proof`
- **Emergency-hotfix lane (expedited yet verified inputs, one exact build identity for support)** [`bltsc-emergency-hotfix-docs`]: family `emergency_hotfix` on `docs_help`, representation `desktop_full`, role `support_identity`
- **Emergency-hotfix lane (expedited yet verified inputs, one exact build identity for support)** [`bltsc-emergency-hotfix-release-center`]: family `emergency_hotfix` on `release_center`, representation `compact_narrowed`, role `support_identity`
- **Emergency-hotfix lane (expedited yet verified inputs, one exact build identity for support)** [`bltsc-emergency-hotfix-support`]: family `emergency_hotfix` on `support_export`, representation `exported_redacted`, role `support_identity`
