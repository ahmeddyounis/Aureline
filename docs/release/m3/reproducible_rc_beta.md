# Reproducible beta release-candidate packet

This packet is the beta source of truth for clean-room rebuild evidence
and exact-build publication checks. The machine-readable packet lives at
[`/artifacts/release/m3/reproducible_rc_packet/packet.json`](../../../artifacts/release/m3/reproducible_rc_packet/packet.json)
and is validated by
[`/tools/ci/m3/clean_room_rebuild/`](../../../tools/ci/m3/clean_room_rebuild).

The packet binds candidate `release_candidate:aureline.2_1_0_beta_1` to
exact-build identity
`build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`,
the promoted artifact graph, the clean-room rebuild rehearsal packet,
and a rebuilt artifact-graph snapshot. The gate compares those rows
before publication widens; a digest mismatch blocks the row rather than
turning into a broader claim.

## Release-control Inputs

The headless gate reads:

- promoted graph:
  [`/artifacts/release/m3/artifact_graph.json`](../../../artifacts/release/m3/artifact_graph.json);
- promoted support projection:
  [`/artifacts/release/m3/artifact_graph_support_projection.json`](../../../artifacts/release/m3/artifact_graph_support_projection.json);
- clean-room rebuild packet:
  [`/artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md`](../../../artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md);
- rebuilt graph snapshot:
  [`/artifacts/release/m3/reproducible_rc_packet/rebuilt_artifact_graph.json`](../../../artifacts/release/m3/reproducible_rc_packet/rebuilt_artifact_graph.json);
- support/export projection:
  [`/artifacts/release/m3/reproducible_rc_packet/support_export_projection.json`](../../../artifacts/release/m3/reproducible_rc_packet/support_export_projection.json).

The clean-room lane remains honest about its current limits: byte-
identical binaries and release signing are not claimed by this beta
packet. The checked claim is that the rebuilt artifact graph preserves
the exact-build identity and digest rows for the promoted candidate.

## Publication Checks

The packet carries five blocking checks:

| Check | Blocks on |
|---|---|
| `clean_room_rebuild_graph_match` | rebuilt graph rows differ from promoted graph rows |
| `exact_build_identity_shared` | candidate, graph, support projection, or rebuilt rows name different build identities |
| `docs_schema_sdk_coupled` | release-bearing docs, schemas, or generated projections fall out of the promoted graph |
| `support_export_projection_current` | support/export projection is missing, stale, or regenerated differently |
| `release_center_candidate_bound` | release-center candidate refs diverge from the promoted artifact graph |

The validator fails closed for each check. A stale or mismatched proof
narrows or holds publication instead of relying on version strings,
release notes, or warmed caches.

## Support Projection

Support exports consume
`reproducible_rc_support_export`, not the prose on this page. The
projection is metadata-only, excludes raw package bytes, and repeats the
same artifact graph checks and publication-check states that the release
gate evaluated. Support and partner proof lanes can therefore cite one
packet without reconstructing release relationships by hand.

## Verification

```bash
python3 -m tools.ci.m3.clean_room_rebuild --repo-root . --check
cargo test -p aureline-support --test reproducible_rc_support_export
```
