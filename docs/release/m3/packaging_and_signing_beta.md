# Beta packaging, signing, update, policy, and schema-governance baseline

This packet is the beta release-engineering entry point for package
identity, signing hooks, update metadata, rollback, policy references,
schema governance, compatibility evidence, benchmark evidence, and
support/export reconstruction. The machine-readable source of truth is
[`/artifacts/release/m3/artifact_graph.json`](../../../artifacts/release/m3/artifact_graph.json),
validated by
[`/tools/ci/m3/artifact_graph/`](../../../tools/ci/m3/artifact_graph).

The graph is a governed beta baseline, not a claim that final package
bytes have landed. Rows whose package bytes are still pending are marked
`metadata_seed_pending_package_bytes` and carry signing and provenance
hooks rather than verified signatures. If any proof is missing, stale, or
degraded, downstream surfaces keep the row visible but narrow the claim.

## Governed Objects

The graph binds these release-bearing objects into one coordinated
artifact set:

- desktop shell, CLI, and remote-agent package nodes;
- signed update metadata and rollback-target metadata;
- policy bundle and schema-governance refs;
- artifact-graph, update-manifest, artifact-verification, claim, and
  policy schema pins;
- docs/help truth, compatibility report, benchmark snapshot, symbol
  manifest, SBOM/notice anchor, provenance rehearsal, claim manifest,
  support projection, and release-packet refs.

The support/export projection at
[`/artifacts/release/m3/artifact_graph_support_projection.json`](../../../artifacts/release/m3/artifact_graph_support_projection.json)
is generated from the graph. Support packets cite node ids, computed
digests, signature state, provenance state, policy refs, schema refs, and
rollback refs from that projection rather than reconstructing
relationships from release notes or support prose.

Install topology and fleet rollout diagnostics are published separately at
[`/artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json`](../../../artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json)
with schema coverage at
[`/schemas/release/install_diagnostics.schema.json`](../../../schemas/release/install_diagnostics.schema.json).
Those rows bind install mode, channel, updater owner, state roots,
managed-package reports, and the same exact-build identity refs used by
the artifact graph.

Ring rollout, silent deployment, and rollback-safe package promotion are
published at
[`/artifacts/release/m3/ring_rollout/packet.json`](../../../artifacts/release/m3/ring_rollout/packet.json)
with the headless gate in
[`/tools/ci/m3/ring_rollout/`](../../../tools/ci/m3/ring_rollout). The packet
joins exact-build install diagnostics, silent deployment result records,
state-root audit rows, support-export projection, and ring-history evidence.
Managed and self-serve rollout lanes quote the same `canary`, `pilot`,
`broad`, and `lts` operational ring vocabulary, and every promotion or
rollback action preserves prior package visibility instead of leaving ambiguous
channel state.

Update rollback is published at
[`/artifacts/release/m3/update_rollback/rollback_plan.json`](../../../artifacts/release/m3/update_rollback/rollback_plan.json)
with generated support truth at
[`/artifacts/release/m3/update_rollback/support_export_projection.json`](../../../artifacts/release/m3/update_rollback/support_export_projection.json)
and the headless gate in
[`/tools/ci/m3/update_rollback/`](../../../tools/ci/m3/update_rollback).
The plan binds the current beta exact build
`build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`
to rollback target `release_candidate:aureline.2_0_4_stable` and
target `exact_build_identity_ref`
`build-id:aureline:stable:2.0.4:aarch64-apple-darwin:release:1f40c9d2b4a1`.
Support, docs, Help, and migration rows use the same tokens:
`retained_prior_artifact_set`, `schema_rollback_hook`,
`downgrade_eligibility_state`, and `exact_build_identity_ref`.

Correction train, hotfix, and backport triage is published at
[`/artifacts/release/m3/correction_train/packet.json`](../../../artifacts/release/m3/correction_train/packet.json)
with generated support truth at
[`/artifacts/release/m3/correction_train/support_export_projection.json`](../../../artifacts/release/m3/correction_train/support_export_projection.json)
and the headless gate in
[`/tools/ci/m3/correction_train/`](../../../tools/ci/m3/correction_train).
The packet id `correction.train.beta.release_control` binds each
correction row to `correction_scope`, `correction_risk`,
`correction_evidence`, `target_channels`, `triage_lane`,
`backport_decision`, `rollback_target`, and `known_issue_update` so
release, support, and docs read the same packet form.

Reproducible release-candidate evidence is published at
[`/artifacts/release/m3/reproducible_rc_packet/packet.json`](../../../artifacts/release/m3/reproducible_rc_packet/packet.json)
with generated support truth at
[`/artifacts/release/m3/reproducible_rc_packet/support_export_projection.json`](../../../artifacts/release/m3/reproducible_rc_packet/support_export_projection.json)
and the headless gate in
[`/tools/ci/m3/clean_room_rebuild/`](../../../tools/ci/m3/clean_room_rebuild).
The packet compares the promoted artifact graph to the clean-room
rebuilt graph snapshot and blocks publication when exact-build identity,
digest rows, or release-center candidate refs diverge.

Release-center publication truth is published at
[`/artifacts/release/m3/release_center_pack/pack.json`](../../../artifacts/release/m3/release_center_pack/pack.json)
with generated support truth at
[`/artifacts/release/m3/release_center_pack/support_export_projection.json`](../../../artifacts/release/m3/release_center_pack/support_export_projection.json)
and the headless gate in
[`/tools/ci/m3/release_center_pack/`](../../../tools/ci/m3/release_center_pack).
The pack binds the candidate exact-build identity, artifact graph,
symbol manifest, SBOM/attestation links, claim and compatibility refs,
rollback refs, advisory refs, and support pivots into one inspectable
release-center packet.

Publish, rollback, revocation, and advisory rehearsals are published at
[`/artifacts/release/m3/rehearsals/packet.json`](../../../artifacts/release/m3/rehearsals/packet.json)
with generated support truth at
[`/artifacts/release/m3/rehearsals/support_export_projection.json`](../../../artifacts/release/m3/rehearsals/support_export_projection.json)
and the headless gate in
[`/tools/ci/m3/rehearsals/`](../../../tools/ci/m3/rehearsals). The packet
binds each operational drill to the current beta artifact graph, fixture
input, support/export row, mirror/offline implication, and downgrade or
blocker decision.

## Promotion Rule

The beta candidate may widen only when the graph validator passes:

```bash
python3 -m tools.ci.m3.artifact_graph --repo-root . --check
```

The validator checks graph shape, repo refs, policy/schema pins, required
artifact families, bundle completeness, rollback coverage, release-center
ids, support-projection fields, and the support-export fixture under
[`/fixtures/release/artifact_graph_cases/`](../../../fixtures/release/artifact_graph_cases).

Rollout promotion additionally requires:

```bash
python3 -m tools.ci.m3.ring_rollout --repo-root . --check
```

This gate fails if silent deployment rows omit exact-build diagnostics, if the
state-root audit drifts from install diagnostics, if managed and self-serve
lanes diverge on ring vocabulary, or if a promotion / rollback leaves more than
one active package state for a channel.

Rollback admission additionally requires:

```bash
python3 -m tools.ci.m3.update_rollback --repo-root . --check
```

This gate fails if retained prior artifacts are metadata-only, if schema
rollback hooks are not bound to reviewed update checkpoints, if downgrade
truth is blocked or implicit, or if docs/help/support surfaces stop
quoting the same rollback vocabulary and exact-build refs.

Correction-train admission additionally requires:

```bash
python3 -m tools.ci.m3.correction_train --repo-root . --check
```

This gate fails if a security or trust row is demoted out of `hotfix`, if
an affected supported line lacks a `backport_decision`, if hotfix or
backport rows omit `rollback_target` or `known_issue_update`, or if
docs/help/support surfaces stop quoting the same correction vocabulary.

Reproducible release-candidate admission additionally requires:

```bash
python3 -m tools.ci.m3.clean_room_rebuild --repo-root . --check
```

This gate fails if clean-room rebuild evidence is stale, if the rebuilt
artifact graph no longer matches the promoted candidate graph, if exact-
build identity refs diverge, or if support/export cannot consume the same
metadata-only proof packet.

Release-center pack admission additionally requires:

```bash
python3 -m tools.ci.m3.release_center_pack --repo-root . --check
```

This gate fails if the checked-in pack drifts from the artifact graph, if
the symbol manifest does not name the beta exact-build identity, if
SBOM/attestation links are absent, if support and security pivots require
private path lookup, or if the generated support projection and capture
are stale.

Release-control rehearsal admission additionally requires:

```bash
python3 -m tools.ci.m3.rehearsals --repo-root . --check
```

This gate fails if any required publish, rollback, revocation, or
advisory flow is absent; if a fixture input no longer matches the
canonical packet; if mirror/offline implications are missing; or if a
non-green result lacks a downgrade, hold, or blocker decision.

## Rollback Rule

Rollback targets the coordinated artifact set. Desktop, CLI, helper,
update metadata, policy bundle, schemas, docs/help truth, support
projection, and sidecars move together or stay blocked. The
`retained_prior_artifact_set` must resolve to
`build-id:aureline:stable:2.0.4:aarch64-apple-darwin:release:1f40c9d2b4a1`
for every required family. A binary-only rollback is outside the beta
contract.

## Schema And Policy Rule

Schema and policy refs are versioned inside the graph under
`policy_and_schema_governance`. Adding a new governed artifact family,
repurposing an enum, or changing required support-projection fields
requires updating:

- [`/schemas/release/artifact_graph.schema.json`](../../../schemas/release/artifact_graph.schema.json);
- [`/artifacts/release/m3/artifact_graph.json`](../../../artifacts/release/m3/artifact_graph.json);
- [`/artifacts/release/m3/artifact_graph_support_projection.json`](../../../artifacts/release/m3/artifact_graph_support_projection.json);
- [`/schemas/release/rollback_plan.schema.json`](../../../schemas/release/rollback_plan.schema.json);
- [`/artifacts/release/m3/update_rollback/rollback_plan.json`](../../../artifacts/release/m3/update_rollback/rollback_plan.json);
- [`/artifacts/release/m3/update_rollback/support_export_projection.json`](../../../artifacts/release/m3/update_rollback/support_export_projection.json);
- [`/schemas/release/reproducible_rc_packet.schema.json`](../../../schemas/release/reproducible_rc_packet.schema.json);
- [`/artifacts/release/m3/reproducible_rc_packet/packet.json`](../../../artifacts/release/m3/reproducible_rc_packet/packet.json);
- [`/artifacts/release/m3/reproducible_rc_packet/support_export_projection.json`](../../../artifacts/release/m3/reproducible_rc_packet/support_export_projection.json);
- [`/schemas/release/release_center.schema.json`](../../../schemas/release/release_center.schema.json);
- [`/artifacts/release/m3/release_center_pack/pack.json`](../../../artifacts/release/m3/release_center_pack/pack.json);
- [`/artifacts/release/m3/release_center_pack/support_export_projection.json`](../../../artifacts/release/m3/release_center_pack/support_export_projection.json);
- [`/schemas/release/rehearsal_packet.schema.json`](../../../schemas/release/rehearsal_packet.schema.json);
- [`/artifacts/release/m3/rehearsals/packet.json`](../../../artifacts/release/m3/rehearsals/packet.json);
- [`/artifacts/release/m3/rehearsals/support_export_projection.json`](../../../artifacts/release/m3/rehearsals/support_export_projection.json);
- [`/fixtures/release/artifact_graph_cases/manifest.yaml`](../../../fixtures/release/artifact_graph_cases/manifest.yaml);
- [`/fixtures/release/m3/reproducible_rc/manifest.yaml`](../../../fixtures/release/m3/reproducible_rc/manifest.yaml);
- [`/fixtures/release/m3/release_center_pack/manifest.yaml`](../../../fixtures/release/m3/release_center_pack/manifest.yaml);
- [`/fixtures/release/m3/rehearsal_inputs/manifest.yaml`](../../../fixtures/release/m3/rehearsal_inputs/manifest.yaml);
- [`/fixtures/release/update_rollback_plan_cases/manifest.yaml`](../../../fixtures/release/update_rollback_plan_cases/manifest.yaml);
- this packet.

## Downgrade Behavior

If compatibility, benchmark, docs/help, signature, provenance, policy, or
schema evidence is stale or missing, release and support surfaces must
render the narrowed state. They may not promote a broader beta claim from
free-text package names, semantic version strings, or manually rebuilt
artifact relationships.
