# Infrastructure Source-Intelligence Alpha

This document is the reviewer-facing landing page for the alpha
infrastructure source-intelligence lane. The lane lets claimed Terraform,
Kubernetes, container, CI, and policy connectors expose read-only topology and
relationship intelligence while keeping active control-plane mutation authority
outside the packet.

For M5 work, the canonical cross-surface vocabulary now lives in
[`/docs/infra/source-intelligence-and-resource-relationships.md`](../../infra/source-intelligence-and-resource-relationships.md).
This alpha page remains the provider-owned read-only projection layer that
should reuse that matrix rather than drifting from it.

The Rust implementation lives at
[`/crates/aureline-provider/src/infrastructure_intelligence/`](../../../crates/aureline-provider/src/infrastructure_intelligence/mod.rs).
The boundary schema lives at
[`/schemas/providers/infrastructure_intelligence.schema.json`](../../../schemas/providers/infrastructure_intelligence.schema.json).
The protected fixture lives at
[`/fixtures/providers/m3/infrastructure_connectors/page.json`](../../../fixtures/providers/m3/infrastructure_connectors/page.json).

## Alpha promise

- Every claimed connector declares a source class, target context, freshness
  label, read mode, and control-plane boundary.
- Every resource and relationship keeps `authored`, `rendered`, `planned`,
  `observed`, `cached`, and `provider_overlay` truth distinct.
- Provider overlays may enrich navigation, but they cannot silently replace
  repo-owned truth or local diff state.
- Search, review, AI context, and support surfaces consume projections derived
  from the same `InfrastructureIntelligenceAlphaPage`.
- Hidden AI writes, hidden provider writes, and in-product mutation authority
  are validation failures for this alpha lane.

## Connector and truth vocabulary

| Vocabulary | Values |
| --- | --- |
| Connector kind | `terraform_workspace`, `kubernetes_cluster`, `container_runtime`, `ci_provider`, `policy_engine` |
| Source class | `terraform_hcl`, `kubernetes_manifest`, `container_descriptor`, `ci_environment_descriptor`, `policy_access_config` |
| Truth layer | `authored`, `rendered`, `planned`, `observed`, `provider_overlay`, `cached` |
| Read mode | `read_only_source_intelligence`, `read_only_provider_overlay`, `imported_snapshot_only` |
| Boundary | `read_only_no_mutation_authority`, `compare_only_preview`, `external_handoff_required` |

`in_product_mutation_claimed` exists only as a validator rejection path. It is
not admitted in the protected alpha fixture.

## Shared projections

The provider packet owns the resource and relationship ids. Consumer surfaces
derive from that packet:

| Projection | Rust method | Fixture |
| --- | --- | --- |
| Search | `InfrastructureIntelligenceAlphaPage::search_projection` | [`search_projection.json`](../../../fixtures/providers/m3/infrastructure_connectors/search_projection.json) |
| Review | `InfrastructureIntelligenceAlphaPage::review_projection` | [`review_projection.json`](../../../fixtures/providers/m3/infrastructure_connectors/review_projection.json) |
| Support | `InfrastructureIntelligenceAlphaPage::support_export_projection` | [`support_export.json`](../../../fixtures/providers/m3/infrastructure_connectors/support_export.json) |

The `aureline-search` and `aureline-review` crates expose thin adapters over
those provider methods. They do not define separate relationship stores.

## Validator invariants

The validator rejects:

- connector rows that allow hidden AI/provider writes or fail to exclude
  mutation authority;
- active control-plane mutation authority on resource rows;
- provider overlays that replace repo-owned truth;
- relationships that point at unknown resource ids;
- search/retrieval partiality without an explicit reason;
- consumer projections that do not cite the same page id or that create a
  parallel truth store;
- promotion gates where docs, support, and UI truth drift but promotion remains
  unblocked.

## Verification

```bash
cargo test -p aureline-provider --test infrastructure_intelligence_alpha
cargo run -q -p aureline-provider --bin aureline_provider_infrastructure_intelligence_alpha -- validate --fixture fixtures/providers/m3/infrastructure_connectors/page.json
cargo test -p aureline-search infrastructure_intelligence
cargo test -p aureline-review infrastructure_intelligence
```

The fixture intentionally includes cached and partial container evidence so the
packet proves stale and partial infrastructure state remains labeled all the
way through search, review, AI context, and support export projections.
