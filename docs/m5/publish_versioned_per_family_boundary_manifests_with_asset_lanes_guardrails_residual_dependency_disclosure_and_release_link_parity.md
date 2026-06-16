# Publish versioned, per-family boundary manifests with asset lanes, guardrails, residual-dependency disclosure, and release-link parity

This document is the human-readable companion to the canonical versioned-boundary-manifest register checked in at `artifacts/governance/m5-versioned-boundary-manifests.json` and described by the schema at `schemas/governance/m5-versioned-boundary-manifests.schema.json`. The typed consumer is `aureline_governance::m5_versioned_boundary_manifests`.

## Purpose

The open/local-boundary and upstream-durability matrix (`artifacts/governance/m5-boundary-and-upstream-durability.json`) freezes the standing durability posture of every asset lane: where the open/local core ends, which compliance controls hold, who holds emergency authority, and whether critical upstreams are owned. It answers *is the lane durable right now?* — but it is one matrix, it is not bound to a release train, and it does not, per claimed feature family, publish the inspectable boundary claim a user, admin, or procurement reviewer reads off the product.

This register is that publication layer. For every claimed M5 family it records one **versioned** boundary manifest that states, in one copy-safe record:

- which capabilities stay open/local and which may be productized, expressed as a per-asset-lane **disposition** (`open_local_retained`, `productizable_add_on`, `managed_only`, `restricted_asset`) bound to the lane's boundary posture and support class, each joined back to its durability-matrix row;
- the **guardrails** that preserve the claim — the local core stays useful, new managed value never silently redefines it, the open-core claim carries per-lane detail rather than vague slogans, residual dependencies are disclosed, and the manifest is linked from release evidence;
- the **residual proprietary/hosted dependencies** the family still rests on, each with its class, the lane it affects, whether it is disclosed on the user/admin truth surfaces, and where that disclosure is published;
- the **release link** binding the manifest to a release train, so the manifest's declared label is held in **parity** with the release evidence.

## The two guardrails the spec calls out

The register makes the two anti-patterns from the source documents impossible to ship silently:

- **No vague "open core."** Every manifest must declare the `asset_lane_detail_published` guardrail *and* carry at least one per-asset-lane entry. A manifest that drops its lane detail fails validation, so an "open core" claim can never be published without the asset-lane breakdown and guardrails behind it.
- **No silent redefinition of local-core usefulness.** Every manifest must declare the `no_silent_local_redefinition` guardrail. When new managed value starts fronting a path the local core used to own, that guardrail goes `unsatisfied`, the manifest narrows on the guardrail axis, and the family drops below the cutline until the local default is restored.

## Release-link parity across families

The spine of the register is that a manifest may never publish a label greener than the release train backing it:

- `declared_label` may never rank higher than the release link's `train_label`; an over-claim sets the link state to `parity_broken` and narrows the manifest on the parity axis.
- A missing link (`missing`) or stale link evidence (`stale`) narrows the manifest on the release-link axis.
- The cross-family `release_link_parity` block summarizes parity over the whole train: how many families are in parity, how many have a broken link, how many over-claim, and whether every family is linked at all.

A **published** manifest must be linked, fresh, in parity, fully disclosed, guardrail-clean, proof-fresh, and owner-signed. A narrowed manifest drops its `effective_label` below the launch cutline and may never publish an effective label wider than the one it declares.

## Per-axis narrowing, never one global flag

A manifest narrows on the *specific* axis that thinned out, and the worst axis wins by precedence:

- `narrowed_parity` — the declared label over-claims the release evidence (`release_parity_broken`).
- `narrowed_release_link` — the link is missing or stale (`release_link_missing`, `release_link_stale`).
- `narrowed_disclosure` — a residual proprietary/hosted dependency is undisclosed (`undisclosed_residual_dependency`).
- `narrowed_guardrail` — a guardrail preserving the claim is unsatisfied (`guardrail_unsatisfied`).
- `narrowed_stale` — the proof packet, owner sign-off, or waiver thinned out (`manifest_proof_stale`, `manifest_proof_missing`, `owner_signoff_missing`, `waiver_expired`).

Every narrowing reason is watched by a stop rule. An **inherited** narrowing — a family whose declared label already sits below the cutline, or a gap held by an unexpired waiver — is gated upstream and does not itself hold promotion. A **manifest-layer** failure on a family whose declared label is still at or above the cutline holds promotion through a stop rule, recorded in `publication`.

## Consumption

Downstream Help/About, service-health, docs-publication, support-export, and evaluation-pack surfaces should ingest `reuse_projection()` from the typed model rather than cloning status text, so every surface renders one source of truth — the projection carries the family, the manifest version, the declared and effective labels, the support class, the manifest state, the parity flag, the undisclosed-dependency count, the active reasons, and the reuse surfaces for every manifest.

## Regeneration and proof

The artifact, the negative fixtures, the cases manifest, and the frozen validation capture are emitted by `tools/regenerate_m5_versioned_boundary_manifests.py`, whose summary/parity/promotion logic mirrors the typed Rust consumer. Inline unit coverage lives in `crates/aureline-governance/src/m5_versioned_boundary_manifests/tests.rs`; the protected gate is `crates/aureline-governance/tests/m5_versioned_boundary_manifests.rs`, run by `.github/workflows/check_m5_versioned_boundary_manifests.yml`, and it cross-checks the typed model against the frozen capture and proves the negative fixtures are rejected.
