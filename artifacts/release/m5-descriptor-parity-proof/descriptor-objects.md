# M5 public-truth descriptor objects

- Registry: `m5-descriptor-object-registry:stable:0001`
- Label: `M5 public-truth descriptor objects`
- Objects: 3
- Minted: `2026-07-06T00:00:00Z`
- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion

## Descriptor objects

| Descriptor | Artifact | Source / signature | Freshness / evidence | Authority / handoff | Claim → effective | Narrowings |
|------------|----------|--------------------|----------------------|---------------------|-------------------|-----------|
| `m5-descriptor-object:release-artifact-graph:stable:0001` | `release-artifact-graph:0001` | `first_party_signed` / `signed_attested` | `current` / `complete` | `full_authority` / `not_required` | `stable` → `stable` | 0 |
| `m5-descriptor-object:companion-extension:narrowed:0001` | `marketplace-extension:0042` | `mirror` / `signed_unverified` | `stale` / `partial` | `scoped_authority` / `desktop_handoff_required` | `stable` → `beta` | 8 |
| `m5-descriptor-object:sideloaded-doc:blocked:0001` | `docs-reference:9001` | `not_provided` / `not_provided` | `missing` / `not_provided` | `reference_only` / `console_handoff_required` | `beta` → `unavailable` | 8 |

## Named narrowings

### `m5-descriptor-object:release-artifact-graph:stable:0001`

- none — stands at its claimed class

### `m5-descriptor-object:companion-extension:narrowed:0001`

| Facet | Value | Effect | Floor |
|-------|-------|--------|-------|
| `source_class` | `mirror` | `narrow` | `beta` |
| `signature_state` | `signed_unverified` | `narrow` | `beta` |
| `freshness_state` | `stale` | `narrow` | `beta` |
| `freshness_evidence` | `partial` | `narrow` | `beta` |
| `qualification_evidence` | `limited` | `narrow` | `beta` |
| `client_kind` | `companion_scoped` | `narrow` | `beta` |
| `authority_class` | `scoped_authority` | `narrow` | `beta` |
| `handoff_requirement` | `desktop_handoff_required` | `narrow` | `beta` |

### `m5-descriptor-object:sideloaded-doc:blocked:0001`

| Facet | Value | Effect | Floor |
|-------|-------|--------|-------|
| `source_class` | `not_provided` | `block` | `unavailable` |
| `signature_state` | `not_provided` | `narrow` | `beta` |
| `freshness_state` | `missing` | `block` | `unavailable` |
| `freshness_evidence` | `not_provided` | `block` | `unavailable` |
| `qualification_evidence` | `retest_pending` | `narrow` | `beta` |
| `client_kind` | `browser_reference` | `narrow` | `beta` |
| `authority_class` | `reference_only` | `narrow` | `beta` |
| `handoff_requirement` | `console_handoff_required` | `narrow` | `beta` |

