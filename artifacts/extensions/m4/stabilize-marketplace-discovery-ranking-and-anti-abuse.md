# Artifact: Stabilize marketplace discovery ranking and anti-abuse

**Status:** Implemented
**Verification class:** Conformance / interoperability suite + security / privacy review + release evidence review

## Summary

This artifact publishes the canonical stable packet for marketplace discovery ranking, anti-abuse controls, verified-publisher tiers, quarantine/revocation truth, enterprise curation paths, and support/export parity. The implementation lives in `crates/aureline-extensions/src/stabilize_marketplace_discovery_ranking_and_anti_abuse/`, the schema lives at `schemas/extensions/marketplace-ranking-and-anti-abuse.schema.json`, and fixtures live under `fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/`.

The packet derives the effective discovery tier from typed evidence. Prominence can be held only when ranking is explainable without raw install count, required quality and compatibility signals are present, anti-abuse controls are visible, publisher status is mechanically sourced, mirrored/private lanes preserve identity and provenance, and moderation events are exportable.

## Acceptance Coverage

- Prominent results are explainable from typed signals rather than raw install count.
- Stale compatibility and runtime regressions narrow discovery to `beta`.
- Typosquat/look-alike and suspicious-package drills produce visible quarantine and withdrawal.
- Review/install fraud produces a review state without hiding the package history.
- Verified publisher, official-style status, enterprise-approved, community, and under-review states come from one publisher status model.
- Public to quarantine to approved mirror/private registry flows preserve package identity, provenance, and support-class truth.
- Revocation and emergency-disable events remain visible and exportable across mirror/offline support lanes.

## Verification

```bash
cargo test -p aureline-extensions stabilize_marketplace_discovery_ranking_and_anti_abuse
```

## Follow-Ups

- Wire this packet into the eventual marketplace UI row renderer once that surface lands.
- Replace closed string vocabularies with shared enums when the registry/status crate exposes the final model.
