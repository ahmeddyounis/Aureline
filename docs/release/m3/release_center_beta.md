# Beta release-center pack

The beta release-center pack is the canonical release-control packet for
the current candidate. It does not replace the artifact graph; it quotes
the graph, symbol manifest, proof links, compatibility rows, and support
pivots in one metadata-only packet so release, support, security, docs,
and Help/About surfaces read the same refs.

Machine-readable truth lives at
[`/artifacts/release/m3/release_center_pack/pack.json`](../../../artifacts/release/m3/release_center_pack/pack.json)
and validates against
[`/schemas/release/release_center.schema.json`](../../../schemas/release/release_center.schema.json).
The generated support/export projection lives at
[`/artifacts/release/m3/release_center_pack/support_export_projection.json`](../../../artifacts/release/m3/release_center_pack/support_export_projection.json).

## What the pack binds

- Candidate `release_candidate:aureline.2_1_0_beta_1`, version
  `2.1.0-beta.1`, and exact build
  `build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`.
- The complete beta artifact graph and generated artifact-graph support
  projection.
- The exact-build symbol manifest for shell, CLI, and renderer source-map
  support.
- SBOM, signed-attestation, provenance, clean-room rebuild, and release
  evidence links.
- Compatibility, claim-manifest, benchmark, docs/help, rollback, advisory,
  and distributed-compatibility pivots.

The support projection is the first consuming surface. Support and
security reviewers can pivot from the pack to crash symbolication,
advisory, compatibility, SBOM/attestation, rollback, and docs/help refs
without private path lookup or raw package bytes.

## Promotion gate

Beta promotion must include:

```bash
python3 -m tools.ci.m3.release_center_pack --repo-root . --check
```

The gate fails when the pack drifts from the artifact graph, when the
symbol manifest names a different exact build, when SBOM or attestation
links are missing, when support/security pivots require private lookup,
or when checked-in generated outputs are stale.

The GitHub Actions gate is
[`check_beta_release_center_pack`](../../../.github/workflows/check_beta_release_center_pack.yml).

## Downgrade behavior

If the graph, symbol manifest, SBOM/attestation, compatibility report,
claim manifest, docs/help proof, or support projection is stale or
missing, the pack remains visible but promotion stays blocked or the
public claim narrows. The pack never widens a beta claim from a semantic
version string, a package file name, or manually reconstructed artifact
relationships.
