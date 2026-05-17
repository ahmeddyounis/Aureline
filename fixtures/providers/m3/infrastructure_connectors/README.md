# Infrastructure source-intelligence alpha fixtures

This directory contains the protected fixture packet and derived projections for
read-only infrastructure source intelligence across claimed Terraform,
Kubernetes, container, CI, and policy connectors.

The canonical packet is [`page.json`](./page.json). The derived projections are
generated from the same provider-owned packet:

- [`search_projection.json`](./search_projection.json)
- [`review_projection.json`](./review_projection.json)
- [`support_export.json`](./support_export.json)

Verify the packet and projections with:

```bash
cargo run -q -p aureline-provider --bin aureline_provider_infrastructure_intelligence_alpha -- validate --fixture fixtures/providers/m3/infrastructure_connectors/page.json
cargo run -q -p aureline-provider --bin aureline_provider_infrastructure_intelligence_alpha -- search-projection --fixture fixtures/providers/m3/infrastructure_connectors/page.json
cargo run -q -p aureline-provider --bin aureline_provider_infrastructure_intelligence_alpha -- review-projection --fixture fixtures/providers/m3/infrastructure_connectors/page.json
cargo run -q -p aureline-provider --bin aureline_provider_infrastructure_intelligence_alpha -- support-export --fixture fixtures/providers/m3/infrastructure_connectors/page.json
```

The fixture intentionally includes stale and partial container evidence. Those
rows carry explicit `stale_within_window`, `partial_index`, and
`partial_retrieval` labels so search, review, AI context, and support surfaces
cannot present cached relationship intelligence as complete live truth.
