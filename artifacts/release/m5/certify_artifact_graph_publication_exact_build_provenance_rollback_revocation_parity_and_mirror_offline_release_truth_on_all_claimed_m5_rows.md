# M5 Publication-Certification Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5
publication-certification register.

- **Canonical JSON**: `artifacts/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.json`
- **Schema**: `schemas/governance/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.schema.json`
- **Typed consumer**: `crates/aureline-release/src/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows/mod.rs`
- **Overview page**: `docs/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.md`
- **Headless emitter**: `cargo run -q -p aureline-release --bin aureline_release_certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows -- register`

The register is the single source of truth for certifying that every claimed M5
artifact family — notebook packs, request/data assets, profiler/replay artifacts,
framework/template packs, docs packs, model packs, companion/offboarding packets,
and managed outputs — ships as **one inspectable publication artifact graph**.
Each row binds the family to its stable claim and grades it across the seven
publication-truth dimensions:

1. **release_center_parity** — the release-center object and headless flow render
   identical artifact-graph truth.
2. **clean_room_rebuild** — a fresh clean-room rebuild reproduces the published
   artifact.
3. **exact_build_symbolication** — exact-build symbol/source-map linkage supports
   symbolication of the published build.
4. **publish_target_review** — the publish target is scoped and reviewed; it never
   inherits ambient credentials.
5. **rollback_record** — a scoped rollback record targets the smallest affected
   node set.
6. **revocation_record** — a revocation / emergency-disable record reaches every
   channel at parity.
7. **mirror_offline_parity** — hosted, mirrored, and offline channels publish the
   family at parity with current drill evidence.

Two spec invariants are first-class, machine-checkable fields rather than prose:

- **`publish_target.inherits_ambient_credentials`** enforces the track invariant
  that *publish targets never inherit ambient credentials*. A family that inherits
  ambient credentials cannot hold its `publish_target_review` dimension and narrows
  below the launch cutline.
- **`mirror_offline`** (hosted/mirrored/offline parity flags plus the drill
  freshness state) enforces the guardrail that *no family claims mirror/offline
  parity without current drill evidence*. A family without full, current parity
  cannot hold its `mirror_offline_parity` dimension and narrows.

A family is certified only when every dimension passes, the publish target is
scoped, mirror/offline parity is proven with current drill evidence, the proof
packet is current, the owner manifest is signed, and downgrade automation is
defined with its frozen-fallback rollback plan verified. Any family whose
release-truth evidence is stale, partial, or missing narrows below the cutline,
the promotion verdict holds, and downstream surfaces (release center, Help/About,
service-health, support export) ingest the projection instead of re-asserting
"stable". The register reports into the canonical M5 evidence index named by
`evidence_index_ref`.

Regenerate the on-disk fixtures with the headless emitter's `emit-fixtures`
subcommand after changing the certification rows.
