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

## Rollback Rule

Rollback targets the coordinated artifact set. Desktop, CLI, helper,
update metadata, policy bundle, schemas, docs/help truth, support
projection, and sidecars move together or stay blocked. A binary-only
rollback is outside the beta contract.

## Schema And Policy Rule

Schema and policy refs are versioned inside the graph under
`policy_and_schema_governance`. Adding a new governed artifact family,
repurposing an enum, or changing required support-projection fields
requires updating:

- [`/schemas/release/artifact_graph.schema.json`](../../../schemas/release/artifact_graph.schema.json);
- [`/artifacts/release/m3/artifact_graph.json`](../../../artifacts/release/m3/artifact_graph.json);
- [`/artifacts/release/m3/artifact_graph_support_projection.json`](../../../artifacts/release/m3/artifact_graph_support_projection.json);
- [`/fixtures/release/artifact_graph_cases/manifest.yaml`](../../../fixtures/release/artifact_graph_cases/manifest.yaml);
- this packet.

## Downgrade Behavior

If compatibility, benchmark, docs/help, signature, provenance, policy, or
schema evidence is stale or missing, release and support surfaces must
render the narrowed state. They may not promote a broader beta claim from
free-text package names, semantic version strings, or manually rebuilt
artifact relationships.
