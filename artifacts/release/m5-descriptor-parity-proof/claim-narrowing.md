# M5 claim-narrowing parity

- Registry: `m5-claim-narrowing-registry:stable:0001`
- Label: `M5 claim-narrowing parity across public-truth consumers`
- Cases: 6
- Minted: `2026-07-06T00:00:00Z`
- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion

## Cases

| Case | Descriptor | Claim state | Effective | Reasons |
|------|------------|-------------|-----------|--------|
| `claim-narrowing:fully-supported` | `m5-descriptor-object:release-artifact-graph:stable:0001` | `fully_supported` | `stable` | 0 |
| `claim-narrowing:limited` | `m5-descriptor-object:marketplace-extension:limited:0001` | `limited` | `beta` | 3 |
| `claim-narrowing:retest-pending` | `m5-descriptor-object:docs-reference:retest:0001` | `retest_pending` | `beta` | 1 |
| `claim-narrowing:evidence-stale` | `m5-descriptor-object:evaluation-pack:stale:0001` | `evidence_stale` | `beta` | 1 |
| `claim-narrowing:unsupported-client` | `m5-descriptor-object:companion-action:scoped:0001` | `unsupported_client` | `beta` | 3 |
| `claim-narrowing:unsupported` | `m5-descriptor-object:sideloaded-doc:blocked:0001` | `unsupported` | `unavailable` | 8 |

## Consumer convergence

### `claim-narrowing:fully-supported` → `fully_supported`

| Consumer | Surface | Claim state | Converges |
|----------|---------|-------------|-----------|
| `release_center` | release/help provenance card | `fully_supported` | yes |
| `help_about` | Help/About support row | `fully_supported` | yes |
| `marketplace` | marketplace listing row | `fully_supported` | yes |
| `docs_help` | docs/help reference badge | `fully_supported` | yes |
| `certification` | certification claim row | `fully_supported` | yes |
| `evaluation_packs` | evaluation-pack claim summary | `fully_supported` | yes |
| `support_export` | support-export claim line | `fully_supported` | yes |
| `companion_handoff` | companion handoff summary | `fully_supported` | yes |

_No narrowing — claim stands at its ceiling._

### `claim-narrowing:limited` → `limited`

| Consumer | Surface | Claim state | Converges |
|----------|---------|-------------|-----------|
| `release_center` | release/help provenance card | `limited` | yes |
| `help_about` | Help/About support row | `limited` | yes |
| `marketplace` | marketplace listing row | `limited` | yes |
| `docs_help` | docs/help reference badge | `limited` | yes |
| `certification` | certification claim row | `limited` | yes |
| `evaluation_packs` | evaluation-pack claim summary | `limited` | yes |
| `support_export` | support-export claim line | `limited` | yes |
| `companion_handoff` | companion handoff summary | `limited` | yes |

**Restores when:**

- `source_class` (`community`) → `provide_provenance`
- `signature_state` (`unsigned`) → `provide_provenance`
- `qualification_evidence` (`limited`) → `complete_evidence`

### `claim-narrowing:retest-pending` → `retest_pending`

| Consumer | Surface | Claim state | Converges |
|----------|---------|-------------|-----------|
| `release_center` | release/help provenance card | `retest_pending` | yes |
| `help_about` | Help/About support row | `retest_pending` | yes |
| `marketplace` | marketplace listing row | `retest_pending` | yes |
| `docs_help` | docs/help reference badge | `retest_pending` | yes |
| `certification` | certification claim row | `retest_pending` | yes |
| `evaluation_packs` | evaluation-pack claim summary | `retest_pending` | yes |
| `support_export` | support-export claim line | `retest_pending` | yes |
| `companion_handoff` | companion handoff summary | `retest_pending` | yes |

**Restores when:**

- `qualification_evidence` (`retest_pending`) → `refresh_evidence`

### `claim-narrowing:evidence-stale` → `evidence_stale`

| Consumer | Surface | Claim state | Converges |
|----------|---------|-------------|-----------|
| `release_center` | release/help provenance card | `evidence_stale` | yes |
| `help_about` | Help/About support row | `evidence_stale` | yes |
| `marketplace` | marketplace listing row | `evidence_stale` | yes |
| `docs_help` | docs/help reference badge | `evidence_stale` | yes |
| `certification` | certification claim row | `evidence_stale` | yes |
| `evaluation_packs` | evaluation-pack claim summary | `evidence_stale` | yes |
| `support_export` | support-export claim line | `evidence_stale` | yes |
| `companion_handoff` | companion handoff summary | `evidence_stale` | yes |

**Restores when:**

- `freshness_state` (`stale`) → `refresh_evidence`

### `claim-narrowing:unsupported-client` → `unsupported_client`

| Consumer | Surface | Claim state | Converges |
|----------|---------|-------------|-----------|
| `release_center` | release/help provenance card | `unsupported_client` | yes |
| `help_about` | Help/About support row | `unsupported_client` | yes |
| `marketplace` | marketplace listing row | `unsupported_client` | yes |
| `docs_help` | docs/help reference badge | `unsupported_client` | yes |
| `certification` | certification claim row | `unsupported_client` | yes |
| `evaluation_packs` | evaluation-pack claim summary | `unsupported_client` | yes |
| `support_export` | support-export claim line | `unsupported_client` | yes |
| `companion_handoff` | companion handoff summary | `unsupported_client` | yes |

**Restores when:**

- `client_kind` (`companion_scoped`) → `use_desktop_client`
- `authority_class` (`scoped_authority`) → `use_desktop_client`
- `handoff_requirement` (`desktop_handoff_required`) → `use_desktop_client`

### `claim-narrowing:unsupported` → `unsupported`

| Consumer | Surface | Claim state | Converges |
|----------|---------|-------------|-----------|
| `release_center` | release/help provenance card | `unsupported` | yes |
| `help_about` | Help/About support row | `unsupported` | yes |
| `marketplace` | marketplace listing row | `unsupported` | yes |
| `docs_help` | docs/help reference badge | `unsupported` | yes |
| `certification` | certification claim row | `unsupported` | yes |
| `evaluation_packs` | evaluation-pack claim summary | `unsupported` | yes |
| `support_export` | support-export claim line | `unsupported` | yes |
| `companion_handoff` | companion handoff summary | `unsupported` | yes |

**Restores when:**

- `source_class` (`not_provided`) → `provide_provenance`
- `signature_state` (`not_provided`) → `provide_provenance`
- `freshness_state` (`missing`) → `refresh_evidence`
- `freshness_evidence` (`not_provided`) → `complete_evidence`
- `qualification_evidence` (`retest_pending`) → `refresh_evidence`
- `client_kind` (`browser_reference`) → `use_desktop_client`
- `authority_class` (`reference_only`) → `use_desktop_client`
- `handoff_requirement` (`console_handoff_required`) → `use_desktop_client`

