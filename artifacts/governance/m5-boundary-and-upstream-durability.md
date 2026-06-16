# Open/local-boundary and upstream-durability matrix

Governance-domain pointer to the canonical matrix that freezes, per asset lane, the open-versus-paid boundary posture, the repository-compliance and third-party-import controls, the emergency signing/registry/security authority, and the continuity rules. Where the open-versus-paid boundary audit (`artifacts/release/open_paid_boundary_audit.json`) attests one release line, the signing-quorum policy (`artifacts/governance/signing_quorum.yaml`) defines the approval floor, the third-party-import manifest (`artifacts/governance/third_party_import_manifest.yaml`) inventories imports, and the critical-upstream scorecard (`artifacts/governance/upstream_health_scorecard.yaml`) scores upstream risk, this matrix binds all of them into one per-lane durability record and decides whether each lane carries a durable boundary claim or is narrowed.

## What it binds

For every claimed asset lane the matrix binds one durability row to:

- the **open/local boundary posture** and support class, with a `must_remain_open` flag for the lanes whose ordinary local usefulness may never be blurred by commercial or managed value;
- the **repository-compliance / review controls** the lane must satisfy, each carrying its own satisfied/unsatisfied/not-applicable state;
- the **emergency authority** holding the lane — primary and backup owners, signer quorum, and registry-emergency and security-response owners — so no release/signing/registry/security lane depends on one irreplaceable human;
- the **continuity rules** — backup coverage, single-point-of-failure posture, and owned critical upstreams.

A lane is durable only when every axis holds. Otherwise it narrows on the specific axis that thinned out — never one global flag — and drops its effective label below the launch cutline. An inherited narrowing (a lane already below the cutline, or a gap held by an unexpired waiver) is gated upstream; a durability-layer failure on a still-stable lane holds publication.

## Canonical sources

- **Matrix JSON**: `artifacts/governance/m5-boundary-and-upstream-durability.json`
- **Schema**: `schemas/governance/m5-boundary-and-upstream-durability.schema.json`
- **Fixtures**: `fixtures/governance/m5-boundary-and-upstream-durability/`
- **Typed consumer**: `crates/aureline-governance/src/m5_boundary_and_upstream_durability/mod.rs`
- **Companion doc**: `docs/m5/freeze_the_m5_open_boundary_repository_compliance_third_party_import_and_maintainer_signer_durability_matrix.md`
- **Validation capture**: `artifacts/governance/captures/m5-boundary-and-upstream-durability_validation_capture.json`

## Reuse

Release packets, docs/boundary manifests, repository-compliance scans, and shiproom gates reuse this one source of truth via the typed consumer's `reuse_projection()` rather than minting per-surface boundary copy, so every reviewing surface reconstructs the current boundary and durability posture from the same machine-readable source.
