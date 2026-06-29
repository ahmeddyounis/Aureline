# M5 descriptor join parity

- Registry: `m5-descriptor-join-registry:stable:0001`
- Label: `M5 descriptor export/support/admin join parity`
- Joins: 6
- Minted: `2026-07-06T00:00:00Z`
- Carriers: export packet, support bundle, admin report, copy-safe summary
- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion

## Joins

| Join | Descriptor | Artifact | Claim state | Effective | Reasons |
|------|------------|----------|-------------|-----------|--------|
| `descriptor-join:fully-supported` | `m5-descriptor-object:release-artifact-graph:stable:0001` | `release_artifact_graph/release-artifact-graph:0001` | `fully_supported` | `stable` | 0 |
| `descriptor-join:limited` | `m5-descriptor-object:marketplace-extension:limited:0001` | `marketplace_extension/marketplace-extension:0042` | `limited` | `beta` | 3 |
| `descriptor-join:retest-pending` | `m5-descriptor-object:docs-reference:retest:0001` | `docs_reference/docs-reference:5001` | `retest_pending` | `beta` | 1 |
| `descriptor-join:evidence-stale` | `m5-descriptor-object:evaluation-pack:stale:0001` | `evaluation_pack/evaluation-pack:7001` | `evidence_stale` | `beta` | 1 |
| `descriptor-join:unsupported-client` | `m5-descriptor-object:companion-action:scoped:0001` | `companion_action/companion-action:8001` | `unsupported_client` | `beta` | 3 |
| `descriptor-join:unsupported` | `m5-descriptor-object:sideloaded-doc:blocked:0001` | `docs_reference/docs-reference:9001` | `unsupported` | `unavailable` | 8 |

## Carrier parity

### `descriptor-join:fully-supported` → `fully_supported`

Copy-safe summary: `m5-descriptor-object:release-artifact-graph:stable:0001 · artifact release_artifact_graph/release-artifact-graph:0001 · claim fully_supported · effective stable · 0 reason(s)`

| Carrier | Identity | Binding | Reasons | Reasons kept |
|---------|----------|---------|---------|--------------|
| `export_packet` | `m5-descriptor-object:release-artifact-graph:stable:0001` | `release_artifact_graph/release-artifact-graph:0001` | 0 | yes |
| `support_bundle` | `m5-descriptor-object:release-artifact-graph:stable:0001` | `release_artifact_graph/release-artifact-graph:0001` | 0 | yes |
| `admin_report` | `m5-descriptor-object:release-artifact-graph:stable:0001` | `release_artifact_graph/release-artifact-graph:0001` | 0 | yes |
| `copy_safe_summary` | `m5-descriptor-object:release-artifact-graph:stable:0001` | `release_artifact_graph/release-artifact-graph:0001` | 0 | yes |

_No narrowing — claim stands at its ceiling._

### `descriptor-join:limited` → `limited`

Copy-safe summary: `m5-descriptor-object:marketplace-extension:limited:0001 · artifact marketplace_extension/marketplace-extension:0042 · claim limited · effective beta · 3 reason(s)`

| Carrier | Identity | Binding | Reasons | Reasons kept |
|---------|----------|---------|---------|--------------|
| `export_packet` | `m5-descriptor-object:marketplace-extension:limited:0001` | `marketplace_extension/marketplace-extension:0042` | 3 | yes |
| `support_bundle` | `m5-descriptor-object:marketplace-extension:limited:0001` | `marketplace_extension/marketplace-extension:0042` | 3 | yes |
| `admin_report` | `m5-descriptor-object:marketplace-extension:limited:0001` | `marketplace_extension/marketplace-extension:0042` | 3 | yes |
| `copy_safe_summary` | `m5-descriptor-object:marketplace-extension:limited:0001` | `marketplace_extension/marketplace-extension:0042` | 3 | yes |

**Downgrade reasons (attributable):**

- `source_class` (`community`) → `limited` (narrow)
- `signature_state` (`unsigned`) → `limited` (narrow)
- `qualification_evidence` (`limited`) → `limited` (narrow)

### `descriptor-join:retest-pending` → `retest_pending`

Copy-safe summary: `m5-descriptor-object:docs-reference:retest:0001 · artifact docs_reference/docs-reference:5001 · claim retest_pending · effective beta · 1 reason(s)`

| Carrier | Identity | Binding | Reasons | Reasons kept |
|---------|----------|---------|---------|--------------|
| `export_packet` | `m5-descriptor-object:docs-reference:retest:0001` | `docs_reference/docs-reference:5001` | 1 | yes |
| `support_bundle` | `m5-descriptor-object:docs-reference:retest:0001` | `docs_reference/docs-reference:5001` | 1 | yes |
| `admin_report` | `m5-descriptor-object:docs-reference:retest:0001` | `docs_reference/docs-reference:5001` | 1 | yes |
| `copy_safe_summary` | `m5-descriptor-object:docs-reference:retest:0001` | `docs_reference/docs-reference:5001` | 1 | yes |

**Downgrade reasons (attributable):**

- `qualification_evidence` (`retest_pending`) → `retest_pending` (narrow)

### `descriptor-join:evidence-stale` → `evidence_stale`

Copy-safe summary: `m5-descriptor-object:evaluation-pack:stale:0001 · artifact evaluation_pack/evaluation-pack:7001 · claim evidence_stale · effective beta · 1 reason(s)`

| Carrier | Identity | Binding | Reasons | Reasons kept |
|---------|----------|---------|---------|--------------|
| `export_packet` | `m5-descriptor-object:evaluation-pack:stale:0001` | `evaluation_pack/evaluation-pack:7001` | 1 | yes |
| `support_bundle` | `m5-descriptor-object:evaluation-pack:stale:0001` | `evaluation_pack/evaluation-pack:7001` | 1 | yes |
| `admin_report` | `m5-descriptor-object:evaluation-pack:stale:0001` | `evaluation_pack/evaluation-pack:7001` | 1 | yes |
| `copy_safe_summary` | `m5-descriptor-object:evaluation-pack:stale:0001` | `evaluation_pack/evaluation-pack:7001` | 1 | yes |

**Downgrade reasons (attributable):**

- `freshness_state` (`stale`) → `evidence_stale` (narrow)

### `descriptor-join:unsupported-client` → `unsupported_client`

Copy-safe summary: `m5-descriptor-object:companion-action:scoped:0001 · artifact companion_action/companion-action:8001 · claim unsupported_client · effective beta · 3 reason(s)`

| Carrier | Identity | Binding | Reasons | Reasons kept |
|---------|----------|---------|---------|--------------|
| `export_packet` | `m5-descriptor-object:companion-action:scoped:0001` | `companion_action/companion-action:8001` | 3 | yes |
| `support_bundle` | `m5-descriptor-object:companion-action:scoped:0001` | `companion_action/companion-action:8001` | 3 | yes |
| `admin_report` | `m5-descriptor-object:companion-action:scoped:0001` | `companion_action/companion-action:8001` | 3 | yes |
| `copy_safe_summary` | `m5-descriptor-object:companion-action:scoped:0001` | `companion_action/companion-action:8001` | 3 | yes |

**Downgrade reasons (attributable):**

- `client_kind` (`companion_scoped`) → `unsupported_client` (narrow)
- `authority_class` (`scoped_authority`) → `unsupported_client` (narrow)
- `handoff_requirement` (`desktop_handoff_required`) → `unsupported_client` (narrow)

### `descriptor-join:unsupported` → `unsupported`

Copy-safe summary: `m5-descriptor-object:sideloaded-doc:blocked:0001 · artifact docs_reference/docs-reference:9001 · claim unsupported · effective unavailable · 8 reason(s)`

| Carrier | Identity | Binding | Reasons | Reasons kept |
|---------|----------|---------|---------|--------------|
| `export_packet` | `m5-descriptor-object:sideloaded-doc:blocked:0001` | `docs_reference/docs-reference:9001` | 8 | yes |
| `support_bundle` | `m5-descriptor-object:sideloaded-doc:blocked:0001` | `docs_reference/docs-reference:9001` | 8 | yes |
| `admin_report` | `m5-descriptor-object:sideloaded-doc:blocked:0001` | `docs_reference/docs-reference:9001` | 8 | yes |
| `copy_safe_summary` | `m5-descriptor-object:sideloaded-doc:blocked:0001` | `docs_reference/docs-reference:9001` | 8 | yes |

**Downgrade reasons (attributable):**

- `source_class` (`not_provided`) → `unsupported` (block)
- `signature_state` (`not_provided`) → `limited` (narrow)
- `freshness_state` (`missing`) → `unsupported` (block)
- `freshness_evidence` (`not_provided`) → `unsupported` (block)
- `qualification_evidence` (`retest_pending`) → `retest_pending` (narrow)
- `client_kind` (`browser_reference`) → `unsupported_client` (narrow)
- `authority_class` (`reference_only`) → `unsupported_client` (narrow)
- `handoff_requirement` (`console_handoff_required`) → `unsupported_client` (narrow)

