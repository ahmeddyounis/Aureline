# Release artifact graph and publication-completeness rules

This document freezes the release-artifact graph Aureline uses to turn
build outputs, docs/help truth, benchmark evidence, security notices,
and promotion gates into one auditable publication set instead of a
directory of loosely related files. The machine-readable companion is
[`/artifacts/release/artifact_graph_rules.yaml`](../../artifacts/release/artifact_graph_rules.yaml).

Companion artifacts:

- [`/artifacts/release/artifact_graph_rules.yaml`](../../artifacts/release/artifact_graph_rules.yaml)
  — machine-readable node-family and bundle-completeness rules.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  — exact-build identity model every release-bearing node resolves
  through.
- [`/docs/release/release_evidence_packet_template.md`](./release_evidence_packet_template.md)
  — aggregate release-truth packet template.
- [`/docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  — public benchmark/public-proof packet template.
- [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — revisioned protected-metrics file cited by claim-bearing benchmark
  packets.
- [`/docs/benchmarks/public_comparison_rules.md`](../benchmarks/public_comparison_rules.md)
  — external publication and head-to-head comparison rules.
- [`/docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  — docs/help truth contract used by release docs applicability.
- [`/docs/security/severity_matrix.md`](../security/severity_matrix.md)
  — advisory/public-disclosure and revocation obligations.
- [`/artifacts/evidence/evidence_metadata_fields.yaml`](../../artifacts/evidence/evidence_metadata_fields.yaml)
  — shared evidence freshness and provenance fields.
- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  and
  [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  — promotion and continuity evidence anchors.

Normative sources this graph projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  §25.7 "Release artifact graph and promotion evidence", Appendix AG
  "Benchmark governance and publication matrix", and Appendix AO
  "Build, CI/CD, Provenance, and Artifact-Publication Matrix".
- `.t2/docs/Aureline_Technical_Design_Document.md`
  Appendix N "Release artifact graph and promotion evidence" and
  Appendix O "Benchmark corpus governance and protected-metrics policy".

## Why publish this now

The repository already has the pieces of a release-truth bundle:

- exact-build identities,
- release-evidence packets and waivers,
- clean-room provenance capture,
- docs/help truth contracts,
- benchmark run-result records and dashboards,
- install-topology and continuity drills, and
- advisory / incident packet schemas.

What it did not have was one graph that says which of those artifacts
own which claim, which nodes are mandatory for a publishable bundle,
which nodes are conditional, what may remain internal, and which
surface is only allowed to quote a stable ref back to an upstream
record. Without that graph:

- a release note, docs pack, and support export can all restate the
  same fact in different words;
- benchmark publication can devolve into dashboard screenshots and
  remembered caveats;
- symbol/source-map retention can become tribal knowledge instead of a
  manifest-backed node in the promotion set; and
- advisories, known-limit notes, and promotion evidence can drift away
  from the exact-build identities they are supposed to narrow.

This document closes that gap.

## Graph invariants

- **One node family owns one fact family.** Release truth, benchmark
  claims, docs/help truth, debug-artifact manifests, advisories, and
  promotion evidence each have a single canonical home. Downstream
  packets quote stable refs; they do not re-mint parallel truth.
- **Every release-bearing node resolves through an exact-build
  identity when applicable.** Version strings alone are never enough.
- **The graph is versioned.** The machine-readable rules file carries a
  `schema_version`; adding or repurposing a node family is a governed
  change.
- **Non-applicable is explicit.** A bundle may omit a conditional node
  family, but it must do so because the family is not applicable to the
  claim, not because the evidence was forgotten.
- **Public packets summarize; supporting bodies may stay internal.**
  Raw build logs, raw traces, raw symbol payloads, private triage
  packets, and similar artifacts may remain internal only when the
  public packet still cites a stable ref and a truthful disclosure note.

## Node families

| Node family | Owns this fact family | Canonical source artifacts | Public expectation | Internal-only allowance |
|---|---|---|---|---|
| `build_identity` | Which exact build each claimed artifact belongs to | `docs/build/exact_build_identity_model.md`, `schemas/build/exact_build_identity.schema.json` | Required for every publishable release or benchmark packet | None; public packets must carry the exact-build refs they cite |
| `runtime_binary` | Runnable first-party payloads (desktop, CLI, helpers, published packs) | exact-build identities plus release packet refs | Required for every claimed runtime payload | Raw build logs and raw smoke logs may stay internal |
| `debug_artifact_manifest` | Symbols, source maps, crash-symbol archives, profiler sidecars, and their retention posture | exact-build identity evidence links and the release graph rules | Required whenever debug bytes are externalized, stripped, mirrored, or withheld | Raw symbol/source-map bytes may stay internal or mirror-only, but the manifest and retention posture may not |
| `docs_help_truth` | Docs-pack applicability, Help/About version match, and release-facing docs truth | `docs/docs/docs_pack_manifest_contract.md`, `schemas/docs/docs_pack_manifest.schema.json`, `schemas/docs/help_status_badge.schema.json` | Required whenever the release claims docs/help applicability or ships a docs pack | Raw docs bodies may stay outside the packet; manifest refs and version-match state may not |
| `schema_contract_export` | Public schemas, manifest contracts, and machine-readable exports that describe the release | `schemas/` plus the matching narrative contract doc under `docs/` | Required whenever a public contract changed or is claimed as shipped | Internal generators and intermediate codegen logs may stay internal |
| `supply_chain_evidence` | SBOMs, provenance statements, attestations, reproducibility packs, and publication manifests | `docs/governance/provenance_and_compliance_baseline.md`, `docs/build/cleanroom_rebuild_lane.md`, `artifacts/release/provenance_capture_seed.json` | Required for every publishable release-truth bundle | Raw signing material and private signing workflow detail stay internal |
| `benchmark_public_proof` | Public benchmark claims, benchmark-governance caveats, and methodology disclosures | `docs/benchmarks/benchmark_publication_pack_template.md`, `docs/benchmarks/benchmark_lab_run_results.md`, `docs/benchmarks/public_comparison_rules.md`, `schemas/benchmarks/run_result.schema.json`, `artifacts/bench/protected_metrics.yaml`, `artifacts/bench/fitness_function_catalog.yaml`, `fixtures/benchmarks/corpus_manifest.yaml` | Required whenever a release packet or public claim cites performance evidence | Raw traces, restricted fixture bytes, private cohort identifiers, and internal dashboards may stay internal by ref |
| `known_limit_and_disclosure_note` | Known issues, narrowed claims, exclusions, waivers, and caveats required to interpret the bundle honestly | `docs/release/release_evidence_packet_template.md`, `schemas/release/waiver_packet.schema.json`, `docs/` public-truth lane | Required for every publishable release or benchmark packet | Internal discussion can stay internal; the public narrowing note cannot |
| `advisory_or_revocation_notice` | Active security advisories, revocations, emergency disables, and affected-install scope | `docs/security/severity_matrix.md`, `schemas/security/advisory_record.schema.json`, `schemas/security/incident_workspace_packet.schema.json` | Required whenever an active advisory or revocation touches the claimed scope | Private triage evidence may stay internal if the public advisory carries stable refs and disclosure state |
| `promotion_evidence` | Rollout-ring, install-topology, rollback-target, continuity-drill, and waiver state for channel movement | `artifacts/release/install_topology_matrix.yaml`, `docs/release/install_topology_plan.md`, `artifacts/support/deployment_drill_catalog_seed.yaml`, `docs/release/release_evidence_packet_template.md` | Required for every preview/beta/stable/LTS promotion claim | Internal operator transcripts may stay internal; ring decision refs and continuity posture may not |
| `release_truth_bundle` | The aggregate packet that binds the graph into one publishable release set | `docs/release/release_evidence_packet_template.md`, `artifacts/evidence/evidence_metadata_fields.yaml` | Required for every release-facing bundle | None; this is the public aggregation surface |

## Bundle completeness

### Publishable release-truth bundle

A publishable release-truth bundle MUST contain:

1. a release-evidence packet id and packet state;
2. the exact-build identity set for every externally claimed artifact
   family;
3. one `runtime_binary` node per published binary/helper/library or
   an explicit statement that no runnable payload is in scope;
4. `supply_chain_evidence`;
5. `known_limit_and_disclosure_note`;
6. `promotion_evidence`; and
7. `release_truth_bundle` aggregation over the cited stable refs.

It MUST also contain these node families when applicable:

- `debug_artifact_manifest` when debug bytes are stripped, mirrored,
  published separately, or retained only for support use;
- `docs_help_truth` when the release claims docs/help applicability or
  ships a docs pack;
- `schema_contract_export` when a public contract changed or ships in
  the promoted set;
- `benchmark_public_proof` when performance evidence is cited for the
  release decision or public claim; and
- `advisory_or_revocation_notice` when an active advisory, revocation,
  or forced-disable state touches the release.

The following may remain internal, but only by stable ref and never by
silent omission:

- raw build logs and raw CI console output,
- raw signing workflow material and private attestation inputs,
- raw symbol/source-map payload bytes,
- raw benchmark traces and raw restricted fixture bytes,
- private incident-triage packet bodies, and
- support-bundle payload bodies beyond the public redaction envelope.

### Publishable benchmark packet

A publishable benchmark packet MUST contain:

1. the packet id, state, and publication posture;
2. exact command line and config refs/digests;
3. the exact-build identity ref or coordinated identity set;
4. release-channel and version context;
5. run-context, comparability, or quarantine posture;
6. corpus-manifest revision, protected-metrics revision, and
   fitness-catalog revision;
7. the task script or success criterion the reader is meant to trust;
8. docs/help applicability via the docs-pack revision and
   `version_match_state`;
9. known limits, exclusions, and any waiver/advisory that narrows the
   claim; and
10. competitor settings whenever a public head-to-head comparison is
    made.

The following may remain internal by stable ref:

- raw trace bundles,
- machine serial numbers and lab-only host identifiers,
- restricted or private fixture bytes,
- private design-partner repository names where policy forbids them in
  public packets,
- competitor license keys or paid configuration material, and
- internal review discussion or signoff chatter.

## Contract-surface index

| Surface | Canonical source artifacts | Graph families it may own | Rule |
|---|---|---|---|
| Release evidence | `docs/release/release_evidence_packet_template.md`, `artifacts/evidence/evidence_metadata_fields.yaml`, `artifacts/release/artifact_graph_rules.yaml`, `schemas/release/waiver_packet.schema.json` | `release_truth_bundle`, `known_limit_and_disclosure_note`, `promotion_evidence` | Aggregates by stable ref; does not become the canonical source for leaf evidence |
| Docs/help truth | `docs/docs/docs_pack_manifest_contract.md`, `schemas/docs/docs_pack_manifest.schema.json`, `schemas/docs/help_status_badge.schema.json`, `docs/build/exact_build_identity_model.md` | `docs_help_truth` | Owns docs applicability and version-match truth; release notes and benchmark packets quote it |
| Benchmark claims | `docs/benchmarks/benchmark_publication_pack_template.md`, `docs/benchmarks/benchmark_lab_run_results.md`, `docs/benchmarks/public_comparison_rules.md`, `schemas/benchmarks/run_result.schema.json`, `artifacts/bench/protected_metrics.yaml`, `artifacts/bench/fitness_function_catalog.yaml`, `fixtures/benchmarks/corpus_manifest.yaml` | `benchmark_public_proof` | Owns public benchmark caveats, comparability, protected-metrics revisioning, and competitor-configuration disclosure |
| Support/export packets | `artifacts/support/`, `docs/state/profile_and_state_map.md`, `docs/build/exact_build_identity_model.md`, `docs/security/severity_matrix.md`, `docs/docs/docs_pack_manifest_contract.md` | support-export projections over `build_identity`, `docs_help_truth`, `advisory_or_revocation_notice`, and redaction-bound state/export records | Support/export surfaces quote upstream refs and redaction classes; they do not invent a release-only or benchmark-only dialect |
| Debug-artifact manifests | `docs/build/exact_build_identity_model.md`, `schemas/build/exact_build_identity.schema.json` | `debug_artifact_manifest` | Owns debug/source-map retention and exact-build linkage; release packets and support exports quote the manifest rather than re-deriving it |

## Current repository posture

This repository is still pre-implementation, so several node families
are template- or seed-backed rather than emitted by a release
pipeline. That is acceptable at this stage as long as the graph stays
honest:

- the graph names the current canonical source artifact,
- seed or placeholder state is disclosed in the packet itself,
- exact-build identities and evidence freshness still travel by stable
  ref, and
- later automation extends these node families instead of minting a
  parallel graph.
