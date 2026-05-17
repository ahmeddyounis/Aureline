# Beta Ring Rollout And Rollback-Safe Promotion

This packet makes beta rollout control inspectable for managed pilots and
self-serve beta lanes. It is a release-control layer, not an installer or
fleet-controller implementation.

## Source Artifacts

- Canonical rollout packet:
  [`artifacts/release/m3/ring_rollout/packet.json`](../../../artifacts/release/m3/ring_rollout/packet.json)
- Silent deployment results:
  [`artifacts/release/m3/ring_rollout/silent_deployment_results.json`](../../../artifacts/release/m3/ring_rollout/silent_deployment_results.json)
- Ring-history packet:
  [`artifacts/release/m3/ring_rollout/ring_history_packet.json`](../../../artifacts/release/m3/ring_rollout/ring_history_packet.json)
- Support-export projection:
  [`artifacts/release/m3/ring_rollout/support_export_projection.json`](../../../artifacts/release/m3/ring_rollout/support_export_projection.json)
- State-root audit:
  [`artifacts/release/m3/state_root_audit.md`](../../../artifacts/release/m3/state_root_audit.md)
- Headless gate:
  [`tools/ci/m3/ring_rollout/`](../../../tools/ci/m3/ring_rollout)
- Fixture manifest:
  [`fixtures/release/ring_rollout_cases/manifest.yaml`](../../../fixtures/release/ring_rollout_cases/manifest.yaml)

## Controlled Rings

Managed and self-serve rollout lanes both use the same operational ring
vocabulary:

1. `canary`
2. `pilot`
3. `broad`
4. `lts`

Validation-ring posture remains separate and is recorded in
`ring_history_packet.json`. The rollout packet must not substitute validation
ring names for operational rollout rings.

## Promotion And Rollback Rules

Every promote or rollback action in the rollout packet carries:

- exact-build identity from install diagnostics and the artifact graph;
- install diagnostics row ref;
- state-root audit ref;
- silent deployment result refs;
- prior package ref, candidate package ref, and rollback target ref;
- pre-action and post-action package visibility;
- post-action channel state with exactly one active package per channel.

A promotion is non-conforming if it hides the prior package rather than keeping
it visible as the rollback target. A rollback is non-conforming if it leaves two
active packages for one channel or drops the candidate package from audit
history.

## Silent Deployment

The silent deployment result packet covers install, update, verify, rollback,
and uninstall. Each row emits:

- `running_build_identity_ref`;
- `state_root_refs`;
- `fleet_ring_context_ref`;
- support-export ref;
- managed-package report ref when the row is managed.

This keeps unattended deployment diagnosable without scraping host-specific
installer logs.

## Verification

Run:

```bash
python3 -m tools.ci.m3.ring_rollout --repo-root . --check
```

The gate validates the rollout packet, silent deployment results, install
diagnostics, artifact graph, state-root map, ring-history packet, generated
state-root audit, support projection, and validation capture together.
