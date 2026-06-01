# Stabilize AI route and spend truth, quota/budget surfaces, and admin explainability

This stable lane makes AI routing and spend visible enough that users, admins,
support, and release packets can all explain provider choice, quota exhaustion,
route downgrade, and actual cost class for the *same* run. The runtime owner is
`aureline_ai::stabilize_ai_route_and_spend_truth`.

Provider and route labels are not enough. For every material AI action on a
claimed stable row, the packet binds — for one evidence id — what route the
action intends to use, what budget family it draws from, what it actually
consumed, why a downgrade happened, and what manual path remains when AI is
unavailable.

## Contract

The packet does **not** re-derive routing, spend-receipt, registry, or
run-history truth. The `aureline_ai::routing::AiRoutingPacket` route lane, the
`aureline_ai::routing_policy::SpendReceiptRecord` spend lane, the
`aureline_ai::registry::ProviderModelRegistryPacket` registry lane, and the
frozen provider-route/spend-receipt boundary
(`schemas/ai/provider_route_receipt.schema.json` and
`schemas/ai/spend_receipt.schema.json`) remain canonical for their own slices.
This packet re-exports those classes verbatim, references their receipts by id,
and adds the visibility invariants a single evidence id needs to carry:

- **Preflight estimate card** — shown before send, carrying the intended route
  class (`local`, `byok`, `managed`, `enterprise_gateway`), an estimated cost
  band and latency band, the owning quota family flow and quota family, scarce
  local-resource cost classes, and approval/policy notes.
- **Live run strip** — present and route-disclosed while in flight, carrying the
  current phase plus the current route class, provider, and model.
- **Post-run receipt** — present after completion or failure, carrying the
  terminal outcome, the actual route class, the actual cost band with an actual
  (not estimate) measurement class, the quota family drawn, and the route and
  spend receipt refs.
- **Distinct quota families** — composer, review, agent/background, generation,
  and tool/connector-assisted stay distinct, each with its quota family, owner
  scope, current state, and budget-owner label, so a blocked action explains
  which budget actually closed and who owns it.
- **Route downgrade banner** — whenever Aureline downgrades because of quota,
  outage, policy, or regionality, both the original and current routes are
  preserved, the cause is named and disclosed, and silent route switching is
  forbidden.
- **Typed registry resolution** — provider id, model id/version, transport class,
  auth mode, retention posture, region posture, quota family, execution locus,
  local-model-pack provenance, and external-tool/MCP gateway hop loci. The route
  class must agree with the execution locus, and a local route must disclose its
  model-pack provenance so a local pack stays visible.
- **Local models are scarce too** — a local route surfaces time, memory, battery,
  or accelerator cost classes instead of implying `local` is free or
  consequence-free.
- **Non-AI fallback path** — always available and reachable without any AI route,
  so users are never stranded in an AI-only dead end.
- **Cumulative-spend posture** — any visible batch or branch-agent lane must show
  cumulative spend and remaining budget; a lane that cannot show cumulative
  receipt truth narrows below Stable rather than implying it.
- **Exportable evidence lineage** — the in-product evidence id is the join key
  the admin inspector and support export reconstruct the same provider/model
  route, budget family, downgrade reason, and actual receipt class from.

## Required behavior

`AiRouteSpendTruthPacket::validate` rejects a packet when:

- a material action does not show the estimate before send, the live run strip is
  absent or hides its route, or a dispatched action lacks a post-run receipt or
  carries a lingering estimate instead of an actual measurement;
- the typed registry does not resolve, the route class disagrees with the
  execution locus, or a local route omits its model-pack provenance;
- a local route does not surface a non-negligible scarce-resource cost class;
- the quota summary does not cover all five flows, or the owning flow's row
  blocked the action without a matching blocked outcome or disclosed downgrade;
- a downgrade does not preserve both routes, omits its cause or disclosure, or a
  route was switched silently;
- no non-AI fallback path remains;
- a visible batch/agent lane lacks cumulative receipt truth but still claims
  Stable; or
- an evidence/export ref is missing, or any field carries raw boundary material.

## Boundary

The packet is export-safe. It carries refs, registry ids, state tokens, coarse
classes, counts, and review labels only. Raw provider payloads, endpoint URLs,
credentials, raw token counts, exact prices, and billing-account ids stay outside
the support boundary.

## Truth source

The checked artifact at
`artifacts/ai/m4/stabilize_ai_route_and_spend_truth/support_export.json` is
canonical for this lane. Dashboards, docs, Help/About surfaces, and support
exports should ingest it instead of cloning status text. The boundary schema is
`schemas/ai/ai-route-receipt.schema.json`; the protected fixtures are in
`fixtures/ai/m4/stabilize_ai_route_and_spend_truth/`. The frozen contracts this
lane projects against are `docs/ai/spend_and_route_receipt_contract.md` and
`docs/ai/model_graduation_and_budget_contract.md`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai stabilize_ai_route_and_spend_truth
```
