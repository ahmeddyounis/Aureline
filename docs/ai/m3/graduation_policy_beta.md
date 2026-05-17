# AI graduation policy for beta surfaces

This page is the docs projection of the checked-in AI graduation state at
`artifacts/ai/m3/graduation_packets/graduation_state.json`. Release truth,
docs truth, CLI/headless inspection, and support export must read that same
state ref before a claimed beta AI surface can widen.

## Current beta graduation state

| Claimed AI surface | Packet | Eval set | Thresholds | Cost profile | Kill switch | Owner | Gate |
|---|---|---|---|---|---|---|---|
| Inline chat local-first beta | `graduation-packet:ai-chat:registry-beta` | `eval-set:surface:inline-chat-local-first:beta:2026-05-17` | `artifacts/ai/m3/eval_thresholds.yaml#threshold-set:ai-beta:2026-05-17` | `cost-profile:ai.inline-chat-local-first:local-companion:beta` | `kill-switch:ai.inline-chat-local-first:beta` | `owner:ai-platform` | `promotable` |
| Review chat cheapest beta | `graduation-packet:review-chat:registry-beta` | `eval-set:surface:review-chat-cheapest:beta:2026-05-17` | `artifacts/ai/m3/eval_thresholds.yaml#threshold-set:ai-beta:2026-05-17` | `cost-profile:ai.review-chat-cheapest:managed-hosted:beta` | `kill-switch:ai.review-chat-cheapest:beta` | `owner:ai-platform` | `promotable` |

The support-export projection is checked in at
`artifacts/ai/m3/graduation_packets/support_export_projection.json`. It is
generated from `ProviderModelRegistryPacket::support_export_projection_with_graduation`
so the route state and the graduation state cannot diverge silently.

## Gate rules

Every claimed beta AI row must have a current packet naming:

- the eval set and threshold set used to judge the row;
- the provider, model, prompt pack, and tool pack admitted by the row;
- protected eval, red-team, latency, and cost evidence refs;
- a coarse latency envelope and coarse cost profile;
- a fallback posture and rollback plan;
- a kill-switch ref and owner ref.

If the packet is missing, expired, points at a different provider or model than
the selected registry route, lacks any required evidence kind, or fails to name
owner/eval/threshold/cost/kill-switch refs, the runtime projection downgrades
the row. Missing or stale packets downgrade to `evidence_stale`; mismatched or
incomplete packets downgrade to `retest_pending`; no admitted route blocks the
row as `unsupported`.

## Verification

```sh
cargo test -p aureline-ai graduation --no-fail-fast
```
