# Host-Boundary Truth, Wrong-Target Reapproval, and Lifecycle Export (Beta)

This contract composes the runtime pieces that already existed in separate
lanes:

- execution-context target and policy identity;
- beta target-discovery source, confidence, freshness, and protected-action
  decisions;
- host-boundary cue stacks from the target-confidence model;
- action origin / target / route / exposure tokens;
- managed-workspace lifecycle lineage and local-editing continuity;
- review-vs-commit drift evaluation for wrong-target and stale-authority cases.

The implementation lives in
[`/crates/aureline-runtime/src/host_boundary/`](../../../crates/aureline-runtime/src/host_boundary/).
The schema lives at
[`/schemas/runtime/host_boundary_and_lifecycle.schema.json`](../../../schemas/runtime/host_boundary_and_lifecycle.schema.json).

## Beta Promise

Every task, terminal, debug, AI-tool, browser-handoff, transcript,
CLI/headless, and support/export surface reads the same
`host_boundary_truth_record`. A surface may render a shorter chip, but the
route token, host-boundary cue stack, discovery block, lifecycle label, and
reapproval posture remain the same packet fields.

The record is export-safe by construction. It carries opaque refs, tokens,
lineage ids, and review-safe summaries. It does not carry raw hostnames, raw
paths, command lines, environment bodies, provider payloads, or secrets.

## Record Shape

`HostBoundaryTruthRecord` preserves:

- normalized host, user/identity-mode, container, workspace, and route chips;
- `action_origin_class`, `action_target_class`, `action_route_class`, and
  `action_exposure_class`;
- ordered `host_boundary_cue_stack`, outermost to innermost;
- one `DiscoveryAuthorityBlock` with discovery source, freshness, alpha
  discovery confidence, resolver confidence, advertised capabilities, the
  authoritative subset after protected-action decisions, and every
  protected-action decision row;
- optional `ManagedLifecycleTruth` with reviewer label, matrix lifecycle state,
  local-editing continuity, activation-budget ref, and ordered lineage tokens;
- wrong-target correction and reapproval requirement classes;
- prior target / prior route refs when a correction or route drift occurred.

## Surface Projections

`HostBoundarySurfaceProjection` is intentionally thin. It quotes the truth
record id and repeats only the tokens needed for the receiving surface:
target, route, cue stack, identity chips, discovery source/freshness/confidence,
authoritative capability subset, lifecycle label, correction token, and
reapproval token.

The protected surfaces are:

| Surface token | Purpose |
| --- | --- |
| `task` | run, test, build, and task rows |
| `terminal` | live terminal headers and command-boundary rows |
| `debug` | launch, attach, frame, and adapter-control surfaces |
| `ai_tool` | AI tool calls and evidence rows |
| `browser_handoff` | browser sheets, callbacks, and return paths |
| `transcript` | restored transcripts and captured output |
| `cli_headless` | CLI and headless summaries |
| `support_export` | support bundles and evidence packets |

## Reapproval Rule

`HostBoundaryReviewBinding` captures the review-time target ref, workspace ref,
action target, action route, cue stack, managed lifecycle state, policy epoch,
and authority linkage. `evaluate_host_boundary_reapproval` compares that
binding with the commit-time `HostBoundaryTruthRecord`.

Any drift in target, route, host-boundary cue stack, workspace, lifecycle,
policy epoch, or authority linkage returns `reapproval_required`. Consumers
must show the drift rows and require a fresh review or approval before commit.
Silent reuse of a stale approval is non-conforming.

## Failure Drills

The fixture corpus lives under
[`/fixtures/runtime/m3/host_boundary_and_wrong_target/`](../../../fixtures/runtime/m3/host_boundary_and_wrong_target/).

It covers:

- remote terminal boundary truth with local + SSH cue stack;
- missing debug adapter capability narrowing only debug launch/attach while
  run/test/build remain in the authoritative subset;
- managed-workspace wrong-target correction requiring approval-ticket reissue;
- browser-handoff return over a resumed managed workspace, preserving route
  truth and lifecycle lineage;
- restored transcript over a suspended managed workspace with local-only write
  continuity and lifecycle lineage exported.

## Verify

```sh
cargo test -p aureline-runtime host_boundary
cargo test -p aureline-runtime --test host_boundary_truth_beta
```
