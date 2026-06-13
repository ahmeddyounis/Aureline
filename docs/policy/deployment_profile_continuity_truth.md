# Deployment profile continuity truth

This contract freezes one canonical packet for deployment-boundary honesty
across local, managed, self-hosted, mirrored, and offline-capable claims. The
packet is produced by
`aureline_policy::deployment_profile_continuity_truth` and is intended to be
the single fact source for About, Help, diagnostics, service-health, admin, and
support-export surfaces when they need to answer four questions quickly and
consistently:

1. What profile is running right now, which tenant or org boundary applies, and
   which control-plane and data-plane services are actually in play?
2. Which still-vendor-hosted or third-party dependencies remain on profiles
   that market a self-hosted, mirrored, or offline-compatible posture?
3. What remains usable locally when auth, policy, mirrors, or distribution
   freshness degrade?
4. Which mirrored or offline artifacts are current, stale, or inspect-only, and
   what stays blocked until refresh or import succeeds?

## Stable conditions

The packet qualifies `stable` only when all of the following hold at once:

1. Every claimed deployment profile has one deployment summary card and one
   local-safe fallback card.
2. Every summary card names tenant scope where relevant, region, key ownership,
   mirror/offline posture, and exact control-plane versus data-plane services.
3. Every vendor-hosted or third-party dependency referenced by a summary card
   is disclosed by a residual-dependency row with a failure consequence,
   local-safe alternative, and disable or replacement path.
4. Every mirror-backed, offline-grace, or air-gapped profile carries at least
   one mirror-freshness card naming source, signer continuity, verification
   time, refresh action, and any blocked actions.
5. Every degraded, reauth-gated, mirrored, or offline profile explains what is
   usable now, what is cached or stale, and what remains blocked until refresh
   succeeds.
6. Deployment summary, residual dependency, mirror freshness, and local-safe
   fallback facts all declare one canonical source reused by About, Help,
   diagnostics, service-health, admin, and support-export surfaces.

## Hard guardrail

One condition withdraws the packet immediately:

- A self-hosted or air-gapped claim that hides any still-vendor-hosted or
  third-party dependency. In that case the packet reports
  `sovereign_boundary_overclaimed` and the claim is withdrawn rather than
  silently weakened.

## Output shape

The packet contains:

- deployment summary cards with profile, tenant scope, region, key ownership,
  mirror/offline state, exact plane-separated service lists, and local-safe
  baseline
- residual dependency rows for hosted control-plane, content, telemetry, or AI
  paths that still leave the customer boundary
- mirror freshness cards for signed policy bundles, offline entitlement
  snapshots, docs packs, or similar governed artifacts
- local-safe fallback cards describing usable workflows, cached/stale truth,
  blocked actions, and safe next steps
- a surface-reuse matrix proving the same fact source reaches About, Help,
  diagnostics, service-health, admin, and support export

The packet is metadata-only. It intentionally excludes raw hostnames, raw
tenant identifiers, raw trust roots, raw KMS handles, raw bundle payloads, and
all secret material.

## Canonical paths

- Doc: `docs/policy/deployment_profile_continuity_truth.md`
- Artifact: `artifacts/policy/deployment_profile_continuity_truth.md`
- Schema: `schemas/policy/deployment_profile_continuity_truth.schema.json`
- Fixtures: `fixtures/policy/deployment_profile_continuity_truth/`

## Verify

```sh
cargo test -p aureline-policy deployment_profile_continuity_truth --locked
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- page
```
