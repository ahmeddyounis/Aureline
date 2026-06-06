# AI Route Locus Matrix

This artifact summarizes the stable route-locus evidence emitted by
`ProviderModelRegistryPacket` and `AiRouteSpendTruthPacket`.

| Route or hop | Registry identity | Execution locus | Required stable receipt fields | Block or downgrade behavior |
|---|---|---|---|---|
| Local model route | provider entry, model entry, local-model-pack version | local in-process, local sandbox, or local companion service | routing-policy version, prompt-pack version, local-model-pack provenance, artifact digest, runtime ABI, storage class, license/provenance note | block or downgrade with `local_model_pack_withdrawn` when the pack is withdrawn or provenance is incomplete |
| Managed hosted route | provider entry, model entry | first-party managed hosted vendor path | routing-policy version, prompt-pack version, retention posture, region posture, quota family, fallback chain, rollback/deny lever | block or downgrade with `policy_blocked`, `expired_evidence`, `quota_or_budget_blocked`, or `provider_lifecycle_changed` |
| BYOK route | provider entry, model entry | vendor or self-hosted remote endpoint reached by user/org credential | routing-policy version, prompt-pack version, auth mode, retention posture, region posture, quota family, fallback chain, rollback/deny lever | block or downgrade when policy, region, retention, quota, or credential posture is incompatible |
| Enterprise gateway route | provider entry, model entry | tenant gateway-brokered route | routing-policy version, prompt-pack version, gateway auth mode, region posture, quota family, fallback chain, rollback/deny lever | block or downgrade when policy, gateway lifecycle, evidence freshness, or quota closes the route |
| External tool / MCP hop | external-tool entry plus tool-schema-pack version | local subprocess, local loopback, same-tenant gateway, remote vendor service, or extension locus | tool entry id, execution locus, side-effect class, auth posture, allowed data classes, compatible tool-schema range, tool-schema-pack version | block with `missing_compatible_tool_schema_range` or `pack_lifecycle_blocked`; never inherit the model route label |

The checked support exports are:

- `artifacts/ai/m3/provider_model_registry_beta_support_export.json`
- `artifacts/ai/m4/stabilize_ai_route_and_spend_truth/support_export.json`

Both are metadata-only and intentionally omit raw provider payloads, endpoint
URLs, credentials, exact token counts, exact prices, and raw tool output.
