# Provider and model graduation packets, rollout rings, and kill-switch or backout paths

This contract materializes the provider/model graduation surface into one
export-safe truth packet whose unit of truth is a route row. Shell, docs, support
export, and release tooling consume the packet directly instead of re-describing
rollout, kill-switch, or backout state by hand.

- Packet type: `aureline_ai::ProviderModelGraduationPacket`
- Schema: [`schemas/ai/ship-provider-and-model-graduation-packets-rollout-rings-and-kill-switch-or-backout-paths.schema.json`](../../../schemas/ai/ship-provider-and-model-graduation-packets-rollout-rings-and-kill-switch-or-backout-paths.schema.json)
- Support export: [`artifacts/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/support_export.json`](../../../artifacts/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/support_export.json)
- Fixtures: [`fixtures/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/`](../../../fixtures/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/)

## The route row

Each `ProviderModelGraduationRow` binds, for one provider/model route:

| Field | Meaning |
| --- | --- |
| `provider_id`, `model_id`, `route_label` | Identity tokens and label (never a raw endpoint URL). |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `current_ring` | Internal, canary, early-access, broad, or general availability. |
| `ring_state` | Pending, rolling, held, complete, kill-switched, or backed-out. |
| `kill_switch` | Scope, state, and fail-closed posture of the provider-neutral halt path. |
| `backout` | Rollback posture and verified flag of the reversal path. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `evidence_packet_refs` | Evidence backing a claimed route. |

## Invariants enforced by validation

- **Kill switch fails closed.** Every route — claimed or not — must carry a kill
  switch whose `fails_closed` flag is set, so dispatch is denied on any ambiguity
  and a switch can never fail open.
- **Claimed routes stay halt-able.** A claimed route (Stable, Beta, or Preview)
  must keep its kill switch `armed` or `fired`; a shipped route is never left
  with `not_armed`.
- **Broad exposure needs a verified backout.** A claimed route exposed through a
  broad-exposure ring (`broad` or `general_availability`) must carry a reversing
  backout posture (not `not_applicable`) that has been drilled and `verified`.
- **General availability means Stable.** A route at the `general_availability`
  ring must claim the Stable qualification; general availability never outruns
  the claim.
- **Halts narrow, never hide.** A route whose ring is `kill_switched` or
  `backed_out` may not keep claiming Stable; the halt narrows the claim.
- **Claimed routes carry evidence.** Stable, Beta, and Preview routes must list
  at least one evidence packet ref.
- **Narrow, never hide.** Every route carries a `proof_stale` downgrade rule and
  every downgrade rule must narrow strictly below the claimed qualification.

## Provenance and freshness

`source_contract_refs` must include this schema, this doc, the provider/model
registry schema, and the frozen M5 AI workflow matrix schema whose qualification,
downgrade, and rollback-posture vocabularies the packet reuses. The
`proof_freshness` block records the freshness SLO and asserts that stale proof
automatically narrows claimed routes. Reading the checked export through
`current_provider_model_graduation_export` re-validates every invariant, so a
stale or malformed artifact fails the consuming surface rather than shipping an
optimistic claim.

## Boundary

The packet carries no provider endpoints, credential bodies, raw provider
payloads, exact spend values, or internal kill-switch tokens. Validation rejects
obvious credential material and raw URLs in the serialized export.
