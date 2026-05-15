# M3 clean-room rebuild rehearsal packet

This packet rehearses the clean-room rebuild path that the M3 beta
publication train depends on. It proves the candidate build can be
reproduced with exact-build identity preserved across symbols and
metadata, without claiming public release-grade attestation. The
rehearsal reuses the existing clean-room rebuild lane
(`ci/cleanroom_rebuild.sh`) and the alpha clean-room rebuild dry run
(`artifacts/release/clean_room_rebuild_alpha.md`); the M3 packet adds the
beta-train binding so the M3 claim manifest, public-proof index, and
proof-consumption walkthrough resolve to one canonical rebuild artifact
set.

Reviewer-facing entrypoints:

- Public-proof index row: `m3_public_proof:exact_build_identity`
  (`artifacts/milestones/m3/public_proof_index.md`)
- Claim manifest: `artifacts/release/m3/claim_manifest.md`
- Shared build-identity anchor: `artifacts/build/build_identity.json`
- Reproducible-build baseline:
  `docs/build/reproducible_build_baseline.md`
- Clean-room rebuild lane: `docs/build/cleanroom_rebuild_lane.md`
- Provenance capture seed: `artifacts/release/provenance_capture_seed.json`
- Validator: `ci/check_m3_publication_rehearsal.py`
- Latest validation capture:
  `artifacts/release/m3/clean_room_rebuild_rehearsal/captures/clean_room_rebuild_rehearsal_validation_capture.json`

## Rebuild entry points

| Purpose | Ref | Posture |
|---|---|---|
| Clean-room rebuild command | `ci/cleanroom_rebuild.sh --out-dir target/cleanroom-rebuild` | Requires a clean checkout; emits build identity, artifact digest manifest, SBOM stub, provenance summary, input manifest, and provenance capture. |
| Offline rebuild variant | `ci/cleanroom_rebuild.sh --offline --out-dir target/cleanroom-rebuild` | Exercises pinned inputs without public network fallback where local caches can satisfy the build. |
| Identity check | Compare emitted `build_identity.json` to `artifacts/build/build_identity.json` axis-by-axis. | Fixed axes (commit, toolchain channel, target triple, profile, workspace version, source_date_epoch) MUST match. |
| Symbol / metadata check | Compare the emitted `artifact_digests.json` to the per-artifact rows in this packet's canonical block. | Each declared artifact family resolves to one entry with a recorded publishability class. |

## Comparable family result

| Family | Comparison basis | Result | Known differences |
|---|---|---|---|
| `ide_binary.primary_shell_spike` | build_identity.json axes + artifact_digests.json row | Comparable as development prototype only. | Binary bytes are not yet a release claim; signing material absent. |
| `workspace_build_identity.primary` | build_identity.json axes | Comparable | None — the rehearsal is bound to one identity record. |
| `workspace_sbom_stub.primary` | sbom_workspace.json placeholder | Comparable as placeholder only | SPDX/CycloneDX conformance is not claimed; SBOM is a stub. |
| `workspace_provenance_summary.primary` | provenance_summary.json placeholder | Comparable as placeholder only | Final SLSA/attestation signing absent. |
| `cleanroom_artifact_digest_manifest.primary` | artifact_digests.json | Comparable | Internal control artifact; declares publishability per-row. |
| `cleanroom_input_manifest.primary` | cleanroom_input_manifest.json | Comparable | Records pinned toolchain, lockfile, cargo config, workspace manifest digests. |
| `cleanroom_provenance_capture.primary` | provenance_capture.json | Comparable | Records lane class, producer identifier, exact-build linkage, and limitations. |

## Acceptance evidence

The rehearsal is acceptable as a controlled beta dry run only while:

- The rebuild result is tied to the same exact-build identity as
  `artifacts/build/build_identity.json` (shared axes match).
- The artifact-family rows below are checked-in and resolvable to the
  alpha clean-room rebuild dry-run lane outputs.
- The known-limits set is explicit; no release-grade signing or
  byte-identical-binary claim is admitted.
- The freshness window MUST NOT exceed the
  `release_evidence_packet_proof` ceiling reachable through the
  `docs_claim_truth_proof` class the M3 exact-build identity row uses.

## Known limits and exclusions

This rehearsal attaches the rehearsal-specific limits:

- `known_limit:m3.clean_room_rebuild.rehearsal_only_no_release_signing`
- `known_limit:m3.clean_room_rebuild.byte_identical_binary_not_claimed`
- `known_limit:m3.clean_room_rebuild.mirror_assumptions_declared_not_verified`

These mirror the limitations recorded in the alpha clean-room rebuild
dry run; the M3 packet quotes them by id rather than restating prose.

## Refresh trigger

Refresh the packet when any of these change:

- `artifacts/build/build_identity.json`
- `artifacts/release/provenance_capture_seed.json`
- `ci/cleanroom_rebuild.sh`
- `tools/build/bootstrap.sh` / `tools/build/build.sh`
- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`
- `artifacts/release/m3/claim_manifest.json`

## Failure drill

To confirm the guardrail is live:

1. Temporarily remove or rename one of the artifact-family entries (for
   example `cleanroom_input_manifest.primary`) in the canonical block.
2. Re-run `python3 ci/check_m3_publication_rehearsal.py --repo-root .`;
   the validator MUST fail with an actionable error.
3. Restore the entry and re-run; it MUST pass.

## Canonical machine source

The block below is the canonical machine truth for this rehearsal packet.
The validator parses YAML between the sentinel markers and ignores the
surrounding prose.

<!-- BEGIN canonical:clean_room_rebuild_rehearsal -->
```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: clean_room_rebuild_rehearsal
packet_id: m3_clean_room_rebuild.rehearsal.first
evidence_id: evidence.m3.clean_room_rebuild.rehearsal.first
title: M3 clean-room rebuild rehearsal — exact-build identity preserved
milestone_id: m3
release_channel_scope: beta
as_of: "2026-05-15"
packet_state: rehearsal_only

ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup

coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
  claim_row_refs:
    - m3_claim_row:canonical.build.exact_build_identity
  covered_lanes:
    - m3_public_proof
    - exact_build_identity
  public_proof_index_row_ref: m3_public_proof:exact_build_identity

result_status: rehearsal_only
visibility_class: public

freshness:
  captured_at: "2026-05-15T21:35:59Z"
  stale_after: P14D
  freshness_class: warm_cached
  proof_class_id: docs_claim_truth_proof
  stale_propagation_profile: claim_narrow_and_hold
  source_revision: commit:b7ee32adb5eb
  trigger_revision: m3_clean_room_rebuild.rehearsal@rev1

environment:
  channel_context: beta
  deployment_context:
    - desktop_self_managed
  environment_summary: >
    M3 beta rehearsal for the clean-room rebuild path. Reuses the alpha
    clean-room rebuild dry run and the existing reproducible-build
    baseline; binds the rebuild to the M3 claim manifest and public-proof
    index without introducing a new rebuild contract.

rebuild_lane:
  command: ci/cleanroom_rebuild.sh --out-dir target/cleanroom-rebuild
  offline_command: ci/cleanroom_rebuild.sh --offline --out-dir target/cleanroom-rebuild
  lane_doc_ref: docs/build/cleanroom_rebuild_lane.md
  baseline_doc_ref: docs/build/reproducible_build_baseline.md
  alpha_dry_run_ref: artifacts/release/clean_room_rebuild_alpha.md
  provenance_capture_seed_ref: artifacts/release/provenance_capture_seed.json

exact_build_identity:
  shared_axes_ref: artifacts/build/build_identity.json
  comparison_basis: >
    Compare the fixed build-identity axes (commit, workspace_version,
    toolchain_channel, target_triple, profile, source_date_epoch) first,
    then compare the emitted artifact_digests.json row by row. Binary
    byte-identity is not claimed at this stage.
  byte_identity_claimed: false
  symbol_metadata_preservation_claimed: true
  symbol_metadata_preservation_rule: >
    The rebuild MUST emit one build_identity.json with axes identical to
    the M3 anchor and an artifact_digests.json that lists each artifact
    family declared below. The artifact_family_ref ids MUST match the
    clean-room rebuild lane's emitted ids; missing or extra families
    fail the rehearsal.

artifact_families:
  - artifact_family_ref: ide_binary.primary_shell_spike
    artifact_id: shell_spike
    publishability_class: development_prototype_non_publishable
    exact_build_artifact_family_class: ide_binary
    artifact_graph_seed_ref: artifact-graph-node:aureline:m0:cleanroom:shell_spike
    comparison_basis: build_identity_axes_plus_artifact_digest_row
  - artifact_family_ref: workspace_build_identity.primary
    artifact_id: build_identity.json
    publishability_class: internal_control_artifact
    exact_build_artifact_family_class: null
    artifact_graph_seed_ref: artifact-graph-node:aureline:m0:cleanroom:build_identity
    comparison_basis: build_identity_axes
  - artifact_family_ref: cleanroom_artifact_digest_manifest.primary
    artifact_id: artifact_digests.json
    publishability_class: internal_control_artifact
    exact_build_artifact_family_class: null
    artifact_graph_seed_ref: artifact-graph-node:aureline:m0:cleanroom:artifact_digests
    comparison_basis: per_artifact_digest_rows
  - artifact_family_ref: workspace_sbom_stub.primary
    artifact_id: sbom_workspace.json
    publishability_class: non_publishable_placeholder
    exact_build_artifact_family_class: sbom_document
    artifact_graph_seed_ref: artifact-graph-node:aureline:m0:cleanroom:sbom_workspace
    comparison_basis: placeholder_schema_only
  - artifact_family_ref: workspace_provenance_summary.primary
    artifact_id: provenance_summary.json
    publishability_class: non_publishable_placeholder
    exact_build_artifact_family_class: null
    artifact_graph_seed_ref: artifact-graph-node:aureline:m0:cleanroom:provenance_summary
    comparison_basis: placeholder_schema_only
  - artifact_family_ref: cleanroom_input_manifest.primary
    artifact_id: cleanroom_input_manifest.json
    publishability_class: internal_control_artifact
    exact_build_artifact_family_class: null
    artifact_graph_seed_ref: artifact-graph-node:aureline:m0:cleanroom:input_manifest
    comparison_basis: pinned_input_digests
  - artifact_family_ref: cleanroom_provenance_capture.primary
    artifact_id: provenance_capture.json
    publishability_class: internal_control_artifact
    exact_build_artifact_family_class: null
    artifact_graph_seed_ref: artifact-graph-node:aureline:m0:cleanroom:provenance_capture
    comparison_basis: lane_class_and_known_limits

artifact_links:
  exact_build_identity_refs:
    - artifacts/build/build_identity.json
  source_anchor_refs:
    - artifacts/milestones/m3/public_proof_index.md
    - artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md
    - artifacts/release/clean_room_rebuild_alpha.md
    - docs/build/cleanroom_rebuild_lane.md
    - docs/build/reproducible_build_baseline.md
  waiver_refs:
    - waiver:single_maintainer_backup
  known_limit_refs:
    - known_limit:m3.clean_room_rebuild.rehearsal_only_no_release_signing
    - known_limit:m3.clean_room_rebuild.byte_identical_binary_not_claimed
    - known_limit:m3.clean_room_rebuild.mirror_assumptions_declared_not_verified

rerun_trigger_refs:
  - exact_build_identity_chain_changed
  - claim_row_or_channel_binding_changed
  - schema_or_packet_header_contract_changed

consuming_surfaces:
  - artifacts/milestones/m3/proof_consumption_walkthrough.md
  - artifacts/milestones/m3/public_proof_index.md
  - artifacts/release/m3/claim_manifest.md

latest_capture:
  captured_at: "2026-05-15T21:35:59Z"
  command: python3 ci/check_m3_publication_rehearsal.py --repo-root .
  report_ref: artifacts/release/m3/clean_room_rebuild_rehearsal/captures/clean_room_rebuild_rehearsal_validation_capture.json
```
<!-- END canonical:clean_room_rebuild_rehearsal -->
