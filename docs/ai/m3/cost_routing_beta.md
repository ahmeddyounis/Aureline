# AI cost routing beta

The beta cost-routing lane makes the cheapest-qualifying-model rule executable
for claimed AI rows and gives support/evidence surfaces a single exportable
packet for route and spend truth.

The runtime owner is `aureline_ai::routing_policy`.

## Contract

`CostRoutingBetaPacket` is built from:

- the provider/model registry fixture at
  `/fixtures/ai/provider_model_registry_beta/registry_packet.json`;
- the current graduation state at
  `/artifacts/ai/m3/graduation_packets/graduation_state.json`;
- one generated `AiRoutingPacket` per claimed AI surface;
- one generated `SpendReceiptRecord` per claimed AI surface.

Each claimed surface row exposes:

- selected provider/model and execution locus;
- route policy class and route-selection reason;
- selected cost-envelope class, cost-visibility class, quota family, budget
  owner, and budget-routing policy ref;
- the cheapest qualifying candidate the policy considered;
- whether the selected route is the cheapest qualifying route;
- route-selection disclosure ref when policy selects a non-cheapest route;
- route receipt ref, spend receipt ref, and evidence-lineage row.

## Policy behavior

Cheapest-qualifying policies select the lowest-cost eligible route after
capability, lifecycle, quota, and execution-locus checks. Policy-limited routes
such as local-first may select a non-cheapest route, but only with a visible
route-selection disclosure ref and a spend receipt that repeats the selected
cost band and budget owner.

Allowed execution loci are enforced during registry resolution. If a policy
narrows a surface to local execution, hosted candidates are removed before
cheapest-route comparison rather than being selected and hidden behind fallback
copy.

## Export

The checked-in support export is:

`/artifacts/ai/m3/cost_routing_beta_support_export.json`

It contains claimed-surface route rows, matching spend receipts, and
`RouteSpendLineage` rows suitable for evidence packets. The export is metadata
only and must not contain provider URLs, credentials, raw provider payloads, raw
token counts, raw prices, user identifiers, or billing-account ids.

## Verification

```sh
cargo test -p aureline-ai routing_policy --no-fail-fast
cargo run -q -p aureline-ai --example dump_cost_routing_beta
```
