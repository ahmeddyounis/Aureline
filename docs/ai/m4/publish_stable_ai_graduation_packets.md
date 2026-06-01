# Stable AI graduation packets

**Policy epoch:** `policy-epoch:stable:0004`  
**Registry state:** `provider-model-registry:stable:2026-06-01`  
**Eval thresholds:** `artifacts/ai/m4/eval_thresholds.yaml#threshold-set:ai-stable:2026-06-01`  
**Graduation state:** `ai-graduation-state:stable:2026-06-01`

This document is the docs-consumer projection for the stable AI graduation. It records the three AI surfaces promoted from preview to stable rollout state, their routing decisions, cost and latency envelopes, known limits, kill-switch handles, and gate references.

## Promoted surfaces

| Surface | Packet | Provider | Model | Cost | Latency | Owner | Gate | Kill-switch |
|---|---|---|---|---|---|---|---|---|
| `surface:inline-chat-local-first` | `graduation-packet:inline-chat-local-first:stable:0001` | `provider-entry:first-party-local-chat:0002` | `model-entry:first-party-local-chat:small:0002` | `metered_per_request_low_volume_band` | `streaming_first_token_under_500ms` | `team:ai-surfaces` | `gate:ai-stable:inline-chat-local-first:0001` | `kill-switch:ai-surface:inline-chat-local-first` |
| `surface:review-chat-cheapest` | `graduation-packet:review-chat-cheapest:stable:0001` | `provider-entry:managed-hosted-chat:0002` | `model-entry:managed-chat:general:0002` | `bundled_no_incremental_cost` | `streaming_first_token_under_500ms` | `team:ai-surfaces` | `gate:ai-stable:review-chat-cheapest:0001` | `kill-switch:ai-surface:review-chat-cheapest` |
| `surface:ai-apply-scoped` | `graduation-packet:ai-apply-scoped:stable:0001` | `provider-entry:managed-hosted-apply:0001` | `model-entry:managed-apply:review:0001` | `bundled_no_incremental_cost` | `p50_under_5s_p99_under_30s` | `team:ai-surfaces` | `gate:ai-stable:ai-apply-scoped:0001` | `kill-switch:ai-surface:ai-apply-scoped` |

## Surface details

### Inline chat — local-first

- **Routing policy:** `route-policy:ai:inline-chat:local-first:stable` (`local_first_then_cheapest`)
- **Lifecycle:** `generally_admitted`
- **Approval posture:** allowed without prompt (local companion route)
- **Eval gates:** task_success ≥ 0.80, safety_no_pii_leak ≥ 0.99, hallucination_rate ≤ 0.10, red-team pass rate ≥ 0.95, P99 latency first token ≤ 500 ms, cost per 1k tokens ≤ $0.002
- **Known limits:**
  - 32k token ceiling on local model
  - Local inference throughput is device-constrained; hosted fallback activates when local capacity is saturated
  - Tool-call support limited to `chat_completion` and `chat_with_tool_calls` feature classes

### Review chat — cheapest

- **Routing policy:** `route-policy:ai:review-chat:cheapest:stable` (`cheapest_qualifying`)
- **Route selection:** `bundled_no_incremental_cost` beats `metered_per_request_low_volume_band` by cost rank — selects managed hosted chat
- **Lifecycle:** `generally_admitted`
- **Approval posture:** per-session consent prompt before first hosted-route call
- **Eval gates:** task_success ≥ 0.82, safety_no_pii_leak ≥ 0.99, hallucination_rate ≤ 0.08, red-team pass rate ≥ 0.96, P99 latency first token ≤ 500 ms, cost per 1k tokens ≤ $0.000 (bundled)
- **Known limits:**
  - 128k token ceiling on managed hosted model
  - Retrieval truth limited to provider-labeled results only (`provider_limited_labeled`)
  - Per-session consent prompt required before first hosted-route call

### AI scoped-apply

- **Routing policy:** `route-policy:ai:apply-scoped:managed:stable` (`cheapest_qualifying`)
- **Lifecycle:** `generally_admitted`
- **Approval posture:** one-time per-apply user approval prompt required (write-capable surface)
- **Eval gates:** task_success ≥ 0.85, safety_no_pii_leak ≥ 0.995, hallucination_rate ≤ 0.05, red-team pass rate ≥ 0.97, P99 wall time ≤ 30 s, cost per 1k tokens ≤ $0.000 (bundled)
- **Known limits:**
  - 128k token ceiling on managed hosted model
  - Write-capable: each apply invocation requires a one-time per-apply user approval prompt
  - Non-AI manual review via the diff viewer remains reachable when the hosted route is unavailable
  - Scoped to workspace files declared in the apply session; cross-workspace writes are blocked

## Routing decisions

Route selection is deterministic. The final selected provider for each surface is:

| Surface | Policy class | Winner | Reason |
|---|---|---|---|
| `surface:inline-chat-local-first` | `local_first_then_cheapest` | `provider-entry:first-party-local-chat:0002` | Local route preferred over all hosted routes |
| `surface:review-chat-cheapest` | `cheapest_qualifying` | `provider-entry:managed-hosted-chat:0002` | `bundled_no_incremental_cost` (rank 0) beats `metered_per_request_low_volume_band` (rank 10) |
| `surface:ai-apply-scoped` | `cheapest_qualifying` | `provider-entry:managed-hosted-apply:0001` | Single eligible provider in policy |

## Artifacts

| Artifact | Path |
|---|---|
| Eval thresholds | `artifacts/ai/m4/eval_thresholds.yaml` |
| Graduation state | `artifacts/ai/m4/publish_stable_ai_graduation_packets/graduation_state.json` |
| Inline chat packet | `artifacts/ai/m4/publish_stable_ai_graduation_packets/inline_chat_local_first_stable.json` |
| Review chat packet | `artifacts/ai/m4/publish_stable_ai_graduation_packets/review_chat_cheapest_stable.json` |
| AI apply packet | `artifacts/ai/m4/publish_stable_ai_graduation_packets/ai_apply_scoped_stable.json` |
| Support export | `artifacts/ai/m4/publish_stable_ai_graduation_packets/support_export.json` |
| Registry fixture | `fixtures/ai/m4/publish_stable_ai_graduation_packets/registry_packet.json` |

## Red-team summary

All three surfaces passed red-team review at the required threshold (≥ 0.95 pass rate) covering:
- Prompt injection via user input
- Indirect injection via workspace content (files, search results)
- PII exfiltration via chat completion
- Scope violation attempts (write-capable apply surface)
- Cross-workspace access attempts (apply surface)

Full red-team reports are referenced in each graduation packet's `red_team_report_refs` array.
