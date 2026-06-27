# M5 Descriptor / Badge Governance Matrix

- Packet: `m5-descriptor-badge-matrix:stable:0001`
- Label: `M5 descriptor / badge matrix`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Descriptors: 4 (4 current, 0 stale, 0 expired, 0 missing)
- Consumers: 8 (8 governed, 0 narrowed, 0 blocked)
- Downgrade rules: 14
- Release gate: pass
- Consumed by: release center, Help/About, marketplace, docs/help, support, companion

## Descriptor objects and badge families

| Descriptor | Badge family | First consumer | Owner | Schema | Proof | Freshness |
|------------|--------------|----------------|-------|--------|-------|-----------|
| `provenance` | `provenance_badge` | `help_about` | release_provenance_owner | `schemas/provenance/m5-provenance-descriptor.schema.json` | `artifacts/public-truth/descriptors/m5-provenance-descriptor.json` | `current` |
| `freshness` | `freshness_badge` | `release_center` | release_freshness_owner | `schemas/provenance/m5-freshness-descriptor.schema.json` | `artifacts/public-truth/descriptors/m5-freshness-descriptor.json` | `current` |
| `qualification` | `qualification_badge` | `release_center` | release_qualification_owner | `schemas/provenance/m5-qualification-descriptor.schema.json` | `artifacts/public-truth/descriptors/m5-qualification-descriptor.json` | `current` |
| `client_scope` | `client_scope_badge` | `companion_handoff` | companion_scope_owner | `schemas/provenance/m5-client-scope-descriptor.schema.json` | `artifacts/public-truth/descriptors/m5-client-scope-descriptor.json` | `current` |

## Descriptor value vocabularies

- `provenance`: `first_party_signed`, `vendor`, `community`, `mirror`, `offline_bundle`, `side_loaded`, `not_provided`
- `freshness`: `current`, `stale`, `expired`, `missing`
- `qualification`: `stable`, `beta`, `preview`, `experimental`, `deprecated`, `unavailable`
- `client_scope`: `desktop_full`, `companion_scoped`, `mobile_companion`, `embedded_panel`, `browser_reference`, `handoff_only`

## Downgrade rules

| Trigger family | Trigger value | Effect | Floor |
|----------------|---------------|--------|-------|
| `provenance` | `vendor` | `narrow` | `beta` |
| `provenance` | `community` | `narrow` | `beta` |
| `provenance` | `mirror` | `narrow` | `beta` |
| `provenance` | `offline_bundle` | `narrow` | `beta` |
| `provenance` | `side_loaded` | `narrow` | `beta` |
| `provenance` | `not_provided` | `block` | `unavailable` |
| `freshness` | `stale` | `narrow` | `beta` |
| `freshness` | `expired` | `block` | `unavailable` |
| `freshness` | `missing` | `block` | `unavailable` |
| `client_scope` | `companion_scoped` | `narrow` | `beta` |
| `client_scope` | `mobile_companion` | `narrow` | `beta` |
| `client_scope` | `embedded_panel` | `narrow` | `beta` |
| `client_scope` | `browser_reference` | `narrow` | `beta` |
| `client_scope` | `handoff_only` | `narrow` | `beta` |

## Public-truth consumers

| Consumer | Status | Claim → effective | Gate | Binds |
|----------|--------|-------------------|------|-------|
| `release_center` | `mapped` | `stable` → `stable` | `governed` | provenance, freshness, qualification, client_scope |
| `help_about` | `mapped` | `stable` → `stable` | `governed` | provenance, freshness, qualification |
| `marketplace` | `mapped` | `stable` → `stable` | `governed` | provenance, qualification, client_scope |
| `docs_help` | `mapped` | `stable` → `stable` | `governed` | provenance, qualification |
| `certification` | `mapped` | `stable` → `stable` | `governed` | provenance, freshness, qualification, client_scope |
| `evaluation_packs` | `mapped` | `stable` → `stable` | `governed` | provenance, freshness, qualification |
| `support_export` | `mapped` | `stable` → `stable` | `governed` | provenance, freshness, qualification, client_scope |
| `companion_handoff` | `mapped` | `stable` → `stable` | `governed` | freshness, qualification, client_scope |
