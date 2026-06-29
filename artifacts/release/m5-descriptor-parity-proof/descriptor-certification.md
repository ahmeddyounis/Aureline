# M5 Descriptor / Badge Certification

- Packet: `m5-descriptor-certification:stable:0001`
- Label: `M5 descriptor / badge certification`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Lanes: 7 (7 current, 0 stale, 0 expired, 0 missing)
- Consumers: 8 (8 certified, 0 narrowed, 0 blocked)
- Downgrade rules: 14
- Release gate: pass
- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion

## Certified runtime lanes

| Lane | Dimension | Schema | Parity proof | Freshness | Status |
|------|-----------|--------|--------------|-----------|--------|
| `descriptor_object` | `descriptor_parity` | `schemas/provenance/m5-descriptor-object.schema.json` | `artifacts/release/m5-descriptor-parity-proof/descriptor-objects.json` | `current` | `mapped` |
| `descriptor_badge_matrix` | `descriptor_parity` | `schemas/provenance/m5-descriptor-badge-matrix.schema.json` | `artifacts/release/m5-descriptor-parity-proof/descriptor-badge-matrix.json` | `current` | `mapped` |
| `badge_vocabulary` | `badge_runtime` | `schemas/provenance/m5-badge-vocabulary.schema.json` | `artifacts/release/m5-descriptor-parity-proof/badge-vocabulary.json` | `current` | `mapped` |
| `claim_narrowing` | `freshness_integration` | `schemas/provenance/m5-claim-narrowing.schema.json` | `artifacts/release/m5-descriptor-parity-proof/claim-narrowing.json` | `current` | `mapped` |
| `descriptor_join` | `badge_runtime` | `schemas/provenance/m5-descriptor-join.schema.json` | `artifacts/release/m5-descriptor-parity-proof/descriptor-join.json` | `current` | `mapped` |
| `omission_guard` | `freshness_integration` | `schemas/provenance/m5-omission-guard.schema.json` | `artifacts/release/m5-descriptor-parity-proof/omission-guard.json` | `current` | `mapped` |
| `client_scope_card` | `descriptor_parity` | `schemas/provenance/m5-client-scope-card.schema.json` | `artifacts/release/m5-descriptor-parity-proof/client-scope-card.json` | `current` | `mapped` |

## Certified consumers

| Consumer | Status | Claim → effective | Gate | Reads | Binds |
|----------|--------|-------------------|------|-------|-------|
| `release_center` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, descriptor_join, omission_guard, client_scope_card | provenance, freshness, qualification, client_scope |
| `help_about` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, omission_guard, client_scope_card | provenance, freshness, qualification |
| `marketplace` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, omission_guard, client_scope_card | provenance, qualification, client_scope |
| `docs_help` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, omission_guard | provenance, qualification |
| `certification` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, descriptor_join, omission_guard, client_scope_card | provenance, freshness, qualification, client_scope |
| `evaluation_packs` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, descriptor_join, omission_guard | provenance, freshness, qualification |
| `support_export` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, descriptor_join, omission_guard, client_scope_card | provenance, freshness, qualification, client_scope |
| `companion_handoff` | `mapped` | `stable` → `stable` | `governed` | descriptor_object, descriptor_badge_matrix, badge_vocabulary, claim_narrowing, descriptor_join, client_scope_card | freshness, qualification, client_scope |
