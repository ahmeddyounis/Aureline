# Stable AI graduation packets — summary

**Policy epoch:** `policy-epoch:stable:0004`
**Registry state:** `provider-model-registry:stable:2026-06-01`
**Graduation state:** `ai-graduation-state:stable:2026-06-01`
**Eval thresholds:** `artifacts/ai/m4/eval_thresholds.yaml#threshold-set:ai-stable:2026-06-01`

## Promoted surfaces

| Surface | Packet ID | Selected provider | Cost envelope | Latency envelope |
|---|---|---|---|---|
| `surface:inline-chat-local-first` | `graduation-packet:inline-chat-local-first:stable:0001` | `provider-entry:first-party-local-chat:0002` | `metered_per_request_low_volume_band` | `streaming_first_token_under_500ms` |
| `surface:review-chat-cheapest` | `graduation-packet:review-chat-cheapest:stable:0001` | `provider-entry:managed-hosted-chat:0002` | `bundled_no_incremental_cost` | `streaming_first_token_under_500ms` |
| `surface:ai-apply-scoped` | `graduation-packet:ai-apply-scoped:stable:0001` | `provider-entry:managed-hosted-apply:0001` | `bundled_no_incremental_cost` | `p50_under_5s_p99_under_30s` |

All three surfaces are promoted from `preview` to `stable` rollout state with `generally_admitted` lifecycle class.
