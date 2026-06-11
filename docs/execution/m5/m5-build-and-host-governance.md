# M5 build-and-host governance matrix

This document describes the canonical packet that freezes the **M5
build-intelligence, host-boundary, and managed-workspace execution-truth
matrix** — one row per marketed M5 execution surface — and that automatically
narrows or withholds the claim of any underqualified surface before it is
published. It is the user-facing companion to the governed artifact at
`artifacts/execution/m5/m5-build-and-host-governance.json` and the typed model in
the `aureline-execution` crate (`m5_build_and_host_governance`).

New M5 execution, preview, infrastructure, and managed-runtime surfaces must stay
explicit about **how a target was discovered, how confident the product is in it,
where the work runs, which control plane still owns mutable truth, and what
recovery path applies** when the target or service plane changes. This packet is
the single source those surfaces resolve through, so no new depth lane needs custom
prose to explain target identity, host boundary, control-plane owner, or
managed-workspace lifecycle.

## What this packet covers

The packet carries one row for every claimed M5 execution surface:

1. **`local_build_target`** — local build target on the developer machine.
2. **`framework_pack_build`** — framework-pack build intelligence target.
3. **`remote_preview_session`** — remote preview session.
4. **`managed_workspace_runtime`** — managed-workspace runtime.
5. **`connector_backed_service`** — connector/control-plane backed service.
6. **`cluster_context_exec`** — cluster-context execution target.
7. **`live_resource_target`** — live-resource (infrastructure) target.
8. **`incident_replay_target`** — incident-replay / ops-adjacent target.

Each row answers, for its surface:

- **How was the target discovered?** A `target_discovery_class` of
  `declared_manifest`, `workspace_probe`, `adapter_inferred`, `control_plane_listed`,
  `user_supplied`, or `undiscovered`.
- **How confident is the adapter?** An `adapter_confidence` of `verified`, `high`,
  `heuristic`, or `unverified` — so confidence always stays visible.
- **Where does it run?** A `host_boundary` (execution origin) of `local_host`,
  `managed_workspace`, `remote_attached`, `cluster_context`, `bridged_host`, or
  `unbound_host`.
- **Who owns mutable truth?** A `control_plane_ownership` of `product_owned`,
  `co_owned`, `external_owned`, or `unknown_owner`.
- **Where is the managed workspace in its lifecycle?** A
  `managed_workspace_lifecycle` of `active`, `provisioning`, `suspended`, `draining`,
  `terminated`, or `not_applicable`.
- **What does it do to live resources?** A `mutation_class` of `read_only`,
  `preview_only`, `reversible_apply`, `irreversible_apply`, or `destructive_apply`.
- **Is preview/approval satisfied?** An `approval_state` of `not_required`,
  `approved`, `preview_pending`, `approval_required_unmet`, or `bypassed`.
- **What is the live-resource context?** A `persistence_class` of `durable`,
  `session_scoped`, `ephemeral`, or `unknown`, and an `expiry_class` of `no_expiry`,
  `scheduled_expiry`, `expired`, or `unknown`.
- **How fresh is the evidence?** An `evidence_freshness` of `current`, `stale`,
  `expired`, or `unknown`.
- **Can the mutation be undone?** A `rollback_posture` of `reversible_verified`,
  `reversible_unverified`, `compensating_only`, `irreversible`, or `not_applicable`.
- **What is backing it?** A `target_identity_ref`, a `host_boundary_ref`, a
  `control_plane_ref`, a `mutation_preview_ref`, a `rollback_ref`, and a
  `support_export_ref` that binds the row into desktop, CLI, support exports, and
  release surfaces.
- **What does the gate publish?** A `published_claim`, a `claim_decision`, and the
  headline `narrowing_reasons` that explain it.

## The claim gate narrows automatically

The execution claim a surface may publish is **not** copied from `declared_claim`.
It is recomputed and the `published_claim`, `claim_decision`, and `narrowing_reasons`
fields must equal that recomputation or validation fails. The gate lowers the
published claim to the weakest of:

- the **capability floor** — the surface's own `declared_claim`;
- the **discovery ceiling** — `declared_manifest`/`workspace_probe` permit
  `authoritative`, `adapter_inferred`/`control_plane_listed` cap at `qualified`,
  `user_supplied` caps at `provisional`, and `undiscovered` withholds the claim;
- the **confidence ceiling** — `verified`/`high` permit `authoritative`, `heuristic`
  caps at `qualified`, and `unverified` caps at `provisional`;
- the **host ceiling** — `local_host`/`managed_workspace` permit `authoritative`,
  `remote_attached`/`cluster_context` cap at `qualified`, `bridged_host` caps at
  `provisional`, and `unbound_host` withholds the claim;
- the **control-plane ceiling** — `product_owned` permits `authoritative`,
  `co_owned`/`external_owned` cap at `qualified`, and `unknown_owner` withholds the
  claim;
- the **workspace ceiling** — `active`/`not_applicable` permit `authoritative`,
  `provisioning` caps at `qualified`, `suspended`/`draining` cap at `provisional`,
  and `terminated` withholds the claim;
- the **mutation ceiling** — `read_only`/`preview_only` permit `authoritative`,
  `reversible_apply` caps at `qualified`, `irreversible_apply` caps at `provisional`,
  and `destructive_apply` withholds the claim;
- the **approval ceiling** — `not_required`/`approved` permit `authoritative`,
  `preview_pending` caps at `qualified`, `approval_required_unmet` caps at
  `provisional`, and `bypassed` withholds the claim;
- the **freshness ceiling** — `current` permits `authoritative`, `stale`/`unknown`
  cap at `qualified`, and `expired` caps at `provisional`;
- the **rollback ceiling** — `reversible_verified`/`not_applicable` permit
  `authoritative`, `reversible_unverified`/`compensating_only` cap at `qualified`,
  and `irreversible` caps at `provisional`.

The `claim_decision` then names the result: `publish` for an authoritative claim,
`narrow_to_qualified`, `narrow_to_provisional`, or `withhold` for a withheld claim.

The `narrowing_reasons` are the nine canonical, spec-aligned execution-truth
triggers, each recomputed from the observed states:

- **`target_undiscovered`** — the discovery class is `undiscovered`.
- **`adapter_confidence_low`** — the adapter confidence is `unverified`.
- **`host_unbound`** — the host boundary is `unbound_host`.
- **`control_plane_unknown`** — the control-plane owner is `unknown_owner`.
- **`workspace_unavailable`** — the managed workspace is `terminated`.
- **`unsafe_mutation`** — the mutation class is `destructive_apply`.
- **`approval_bypassed`** — the approval state is `bypassed`.
- **`evidence_stale`** — freshness is `stale` or `expired`.
- **`rollback_incomplete`** — the rollback posture is `reversible_unverified` or
  `irreversible`.

This is what lets release/desktop/CLI/support tooling **prove** that stale or
underqualified surfaces narrow before publication: a surface that is undiscovered,
low-confidence, unbound, unknown-owned, workspace-unavailable, destructive,
approval-bypassed, stale, or rollback-incomplete simply cannot carry an
`authoritative` published claim, because the recomputed gate decision overrides the
stored one. No new M5 surface can imply a generic "runs here" or "connected" state.

## Governance stays surface-specific and provenance-bound

A verified local build target must never lend its confidence to a live-resource
mutation or an externally owned cluster context. The packet enforces this several
ways:

- Every claimed surface must carry exactly one row (`MissingSurfaceRow` /
  `DuplicateSurfaceRow` otherwise), so no surface inherits a claim from an adjacent
  one, and a row may not cover a surface outside the claimed set
  (`UnclaimedSurfaceRow`).
- Every row must carry its own non-empty `target_identity_ref`, `host_boundary_ref`,
  `control_plane_ref`, `mutation_preview_ref`, `rollback_ref`, and
  `support_export_ref`.

A publishable surface — one that publishes `authoritative` — must additionally be
genuinely clean: an authoritative-ceiling discovery, confidence, host, control-plane,
workspace, mutation, approval, and rollback state, current evidence, an authoritative
capability floor, and no narrowing reason (`PublishedSurfaceNotClean` otherwise).

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each surface's discovery
class, adapter confidence, host boundary, control-plane owner, workspace lifecycle,
mutation, approval, freshness, rollback, persistence, and expiry states, declared and
published claim, decision, and narrowing-reason tokens, plus `publishable_count`,
`narrowed_count`, and `withheld_count`. Desktop and CLI target pickers, service-health
and Help/About, support exports, and release/public-truth surfaces should ingest this
projection directly rather than restating M5 execution, host, and managed-workspace
status by hand, so every claim surface uses the same discovery, confidence, host,
ownership, mutation, and rollback vocabulary as the underlying packet.

## Validation

`M5BuildAndHostGovernanceMatrix::validate()` reports every violation, including an
unsupported schema version or record kind, non-canonical closed vocabularies, empty
required fields, duplicate surface ids, duplicate or missing surface rows,
unclaimed-surface rows, duplicate narrowing reasons, an overstated published claim, a
decision that disagrees with the gate, narrowing reasons that disagree with the
recomputed set, a publishable surface that is not clean, and a summary block that
disagrees with the rows.
