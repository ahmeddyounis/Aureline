# Stabilize Marketplace Discovery Ranking And Anti-Abuse

**Status:** Stable discovery governance lane implemented in `crates/aureline-extensions`.

## Goal

Marketplace discovery rows must explain why a package is prominent, ordinary, narrowed, under review, quarantined, or revoked without falling back to raw install count. The canonical packet is `marketplace_discovery_packet`, defined by `crates/aureline-extensions/src/stabilize_marketplace_discovery_ranking_and_anti_abuse/` and `schemas/extensions/marketplace-ranking-and-anti-abuse.schema.json`.

## Contract

The packet binds these stable truths:

- Typed ranking signals for query relevance, category fit, current-version compatibility, runtime health, maintenance freshness, verified/official status, rollback/uninstall quality, and documentation/security posture.
- Anti-abuse controls for publisher identity verification, namespace reservation, typosquat detection, look-alike detection, review/install fraud detection, suspicious-package quarantine, malware/static policy scanning, and rapid revocation.
- Publisher tier classes: `official_pack`, `verified_publisher`, `enterprise_approved`, `community`, and `under_review`.
- Quarantine/revocation classes: `none`, `review_hold`, `quarantined`, `revoked`, and `emergency_disabled`.
- Enterprise curation paths across `public_registry`, `quarantine_holding`, `approved_mirror`, `private_registry`, and `offline_bundle`, with package identity, provenance, and support-class preservation flags.
- Surface truth flags for marketplace cards, search results, details views, admin review, mirrored catalogs, and support exports.
- Transparency events for removals, appeals, emergency disables, verified-publisher actions, quarantine, and revocation.

## Stable Rules

A `stable` claim only holds when all required ranking signals and anti-abuse controls are present, ranking reasons are inspectable, raw install count is not the primary ranking input, publisher status is mechanically sourced, tiered publisher identity is verified, all abuse reasons are user/admin/support/mirror visible, enterprise curation preserves identity and provenance, transparency events are attributable and exportable, and all required surfaces show the same truth.

The packet automatically narrows:

- `withdrawn` for active quarantine, revocation, malware/policy blocks, or lost curation identity/provenance.
- `preview` for missing signals, unexplainable ranking, vanity-metric ranking, non-mechanical publisher status, publisher review, missing abuse controls, hidden abuse reasons, typosquat/look-alike review, fraud review, surface gaps, or non-exportable transparency.
- `beta` for stale compatibility, runtime/crash/resource regressions, stale maintenance, high rollback/uninstall rates, or documentation/security gaps.

## Fixtures

Canonical fixtures live under `fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/`:

- `verified_publisher_prominent_stable.json`
- `stale_compatibility_and_runtime_regression_narrows.json`
- `typosquat_lookalike_quarantined_withdrawn.json`
- `review_install_fraud_under_review_preview.json`
- `enterprise_approved_mirror_stable.json`
- `rapid_revocation_removed_from_ranking.json`

## Verify

```bash
cargo test -p aureline-extensions stabilize_marketplace_discovery_ranking_and_anti_abuse
```
