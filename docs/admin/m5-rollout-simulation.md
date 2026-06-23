# Rollout-simulation contract

This document covers the *dry-run rollout simulation*: the typed previews Aureline
shows before a policy import, promotion, bundle rollout, mirror-source change,
trust-root change, or route/egress expansion is allowed to widen privilege or
feature access on its claimed managed-cloud, self-hosted, sovereign/air-gapped,
and mirrored/offline profiles.

Where the [admin-plane matrix](./m5-admin-plane.md) *names and freezes the
contract* and the [admin-plane render](./m5-admin-render.md) lane *renders the
current admin state*, this lane *simulates the next state*. Every rollout
scenario is a forward dry-run: it shows which endpoints and features a change
would move, classifies the change as **tightening** (a restriction) or
**widening** (a new permission, egress class, AI provider, registry source, or
trust root), and states the review strength, staged-rollout requirement, and
rollback path a promotion must clear *before* it can broaden access — all without
a separate vendor console. Nothing here applies a change: every scenario is a
`dry_run` preview.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/admin/m5-rollout-simulation.schema.json`](../../schemas/admin/m5-rollout-simulation.schema.json)
  — boundary schema for `m5_rollout_simulation_bundle`.
- [`/fixtures/admin/m5-rollout-simulation/canonical_simulation.json`](../../fixtures/admin/m5-rollout-simulation/canonical_simulation.json)
  — the published canonical simulation bundle; the freeze gate asserts the
  in-code builder equals it byte-for-byte.
- [`/artifacts/admin/m5-rollout-simulation.md`](../../artifacts/admin/m5-rollout-simulation.md)
  — the human-readable companion (per-profile scenario tables).
- `crates/aureline-policy/src/m5_rollout_simulation/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_rollout_simulation` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Binds back to the matrix

The simulation layer is not free-form. It renders the
[`policy_diff`](./m5-admin-plane.md) and
[`endpoint_posture_card`](./m5-admin-plane.md) surfaces forward in time, and binds
back to the frozen matrix:

- **Every state it shows is one the matrix admits.** Each impacted endpoint's
  `posture_before`/`posture_after` and each profile's `claim_state` must be in the
  matrix's `applicable_states` for the endpoint-posture surface
  (`rollout_sim.surface_states_within_matrix`).
- **The surfaces it binds exist and are locally explainable.** Both bound
  surfaces are present in the matrix, `locally_explainable`, and
  `typed_not_portal_only` (`rollout_sim.bound_surfaces_in_matrix`).

So an edit that shows a state the matrix does not admit, or binds a surface the
matrix does not define, flips an invariant and fails the freeze gate.

## Rollout flows simulated

Every dry-run flow the spec requires a managed plane to preview is exercised
somewhere in the bundle (`rollout_sim.change_kinds_covered`):

- `policy_import` — importing a new policy bundle.
- `policy_promotion` — promoting a staged bundle to enforced.
- `bundle_rollout` — rolling a bundle out to endpoints.
- `mirror_source_change` — changing the mirror source a profile syncs from.
- `trust_root_change` — changing a trust root / signing key.
- `route_egress_expansion` — opening a new network route or egress class.

## Tightening versus widening

The core honesty rule: **widening is gated harder than tightening.** A change's
`direction` is `tightening`, `widening`, `mixed`, or `no_effect`. A widening or
mixed change names at least one **widening dimension** — the privilege-broadening
kinds the spec calls out (`rollout_sim.widening_dimensions_covered`):

- `new_permission`, `new_egress_class`, `new_ai_provider`,
  `registry_source_change`, `trust_root_change`.

The differentiation is enforced, not described
(`rollout_sim.widening_requires_stronger_review`):

- A **widening or mixed** scenario clears review at least as strong as
  dual-control, a **staged** (never immediate) rollout, a **non-instant**
  rollback, and names at least one widening dimension.
- A **pure tightening** needs at most a single admin review and names no widening
  dimension.

And restrictions are not over-gated: at least one tightening across the bundle is
a genuinely light, immediately-applicable restriction
(`rollout_sim.tightening_not_overgated`), so a simple restriction is never held to
the widening floor. A feature is flagged `newly_widened` only on a widening
scenario (`rollout_sim.widened_features_only_on_widening`).

## Each scenario is a reviewable dry-run

Every scenario names (`rollout_sim.scenarios_are_reviewable_dry_runs`):

- the **impacted endpoints**, each with a before/after posture and a per-endpoint
  impact note,
- the **impacted features**, each with the user-visible consequence and whether it
  is newly widened,
- the **review requirement**, **staged-rollout requirement**, and **rollback
  path** before any promotion,
- the **simulation outcome** (`safe_to_promote`, `promote_with_staged_rollout`,
  `hold_for_review`, `blocked_stale_evidence`, `blocked_boundary_recheck`), and
- `dry_run: true` — the scenario simulates a change, it never applies one.

A scenario blocked pending a boundary recheck reports the boundary-recheck outcome
and a pinned, signed-only delivery, and vice versa, so a boundary block is never
silently promotable (`rollout_sim.boundary_recheck_consistent`).

## Auto-narrowing on stale evidence

A managed/self-hosted/sovereign claim is honest only while its evidence is fresh.
Each profile reports three freshness inputs — `simulation_freshness` (the stalest
scenario, `rollout_sim.simulation_freshness_is_worst_case`), `mirror_freshness`,
and `endpoint_posture_freshness` — and a `claim_state`:

- When **all** evidence is fresh, the claim reads `active_enforced`
  (confirmed) and `narrow_reasons` is empty.
- When **any** is stale, the claim downgrades off confirmed and `narrow_reasons`
  names exactly which dimension went stale: `simulation_evidence_stale`,
  `mirror_freshness_stale`, or `endpoint_posture_stale`
  (`rollout_sim.claim_auto_narrows_on_stale`). A stale offline mirror downgrades
  to `mirror_offline_last_known`; any other stale evidence downgrades to
  `unconfirmed_stale`.

A scenario whose own simulation evidence is stale is held with the
`blocked_stale_evidence` outcome and can never read as safe to promote
(`rollout_sim.stale_scenarios_held`).

## Profiles covered

The bundle simulates one packet per claimed managed-bearing profile:
`managed_cloud`, `self_hosted`, `sovereign_air_gapped`, and `mirrored_offline`
(`rollout_sim.profiles_covered`). Each maps to a matrix admin path and a
deployment profile. The sovereign profile auto-narrows (simulation and posture
evidence stale) and the mirrored profile auto-narrows (mirror stale), so the
no-console-required explainability holds on the offline rows too.

## Cross-surface parity

There is exactly **one typed packet per profile**, and each packet declares the
consumers that render it: shell admin center, CLI/headless inspect, Help/About,
support export, and release evidence. Because every consumer serializes the same
packet, the simulated blast radius is identical across UI, CLI/support export,
Help/About, and release/public-truth by construction
(`rollout_sim.consumer_parity`).

## Invariants

The builder computes each invariant's `holds` flag from the simulated data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `rollout_sim.surface_states_within_matrix` — every endpoint state shown and
  every claim state is one the frozen matrix admits for the endpoint-posture
  surface.
- `rollout_sim.bound_surfaces_in_matrix` — each profile binds the policy-diff and
  endpoint-posture surfaces, both present, locally explainable, and typed.
- `rollout_sim.widening_requires_stronger_review` — widening clears dual-control+
  review, a staged rollout, a non-instant rollback, and names a dimension;
  tightening needs at most a single admin review.
- `rollout_sim.tightening_not_overgated` — at least one tightening is a light,
  immediately-applicable restriction.
- `rollout_sim.scenarios_are_reviewable_dry_runs` — every scenario is a dry-run
  naming impacted endpoints/features, review, staging, and rollback.
- `rollout_sim.stale_scenarios_held` — a scenario with stale simulation evidence
  is blocked, never safe to promote.
- `rollout_sim.boundary_recheck_consistent` — a boundary-recheck block lines up
  review, outcome, and pinned signed-only delivery.
- `rollout_sim.claim_auto_narrows_on_stale` — the claim is confirmed only when all
  evidence is fresh, and names the stale dimension otherwise.
- `rollout_sim.simulation_freshness_is_worst_case` — the reported simulation
  freshness is the stalest scenario.
- `rollout_sim.widening_dimensions_consistent` — a scenario names a widening
  dimension exactly when it widens or mixes.
- `rollout_sim.widened_features_only_on_widening` — a feature is flagged newly
  widened only on a widening scenario.
- `rollout_sim.profiles_covered` — the managed-cloud, self-hosted,
  sovereign/air-gapped, and mirrored/offline profiles are all simulated.
- `rollout_sim.change_kinds_covered` — every rollout flow is simulated somewhere.
- `rollout_sim.widening_dimensions_covered` — every widening dimension is
  simulated somewhere.
- `rollout_sim.consumer_parity` — one typed packet serves shell, CLI/headless,
  Help/About, support export, and release evidence identically.
- `rollout_sim.stable_ids_unique` — profile, scenario, and endpoint ids are unique
  within scope.
- `rollout_sim.export_safe` — every stable id is an opaque token with no URL
  scheme or absolute path.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw provider
payloads, or absolute paths — only opaque object refs, stable tokens, rendered
metadata-safe value summaries, and short reviewable sentences.
`is_support_export_safe()` enforces that `raw_payload_excluded` is true, every
file ref is repo-relative, and every stable token id is opaque, so the bundle is
safe to embed in a support export verbatim.

## Composes with

This contract simulates the surfaces the [admin-plane matrix](./m5-admin-plane.md)
freezes and the [admin-plane render](./m5-admin-render.md) lane renders, and
composes with the per-surface contracts the matrix binds, notably
[`/docs/admin/policy_diff_alpha.md`](./policy_diff_alpha.md) and
[`/docs/admin/org_admin_seat_and_fleet_contract.md`](./org_admin_seat_and_fleet_contract.md).

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_rollout_simulation > \
  fixtures/admin/m5-rollout-simulation/canonical_simulation.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_rollout_simulation

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_rollout_simulation -- --lines
```
