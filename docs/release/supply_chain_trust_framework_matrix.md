# Supply-chain trust-framework adoption matrix

This document freezes Aureline's release posture for reusing mature
supply-chain trust patterns. It names the preferred trust framework
for each artifact family group, the evidence each family must carry,
and the deviation policy used when a family intentionally does not
follow the preferred pattern.

Companion artifacts:

- [`/artifacts/release/trust_framework_rows.yaml`](../../artifacts/release/trust_framework_rows.yaml)
  - machine-readable crosswalk from artifact family groups to trust,
  provenance, mirror, rotation, and emergency handling controls.
- [`/schemas/release/provenance_stack_exception.schema.json`](../../schemas/release/provenance_stack_exception.schema.json)
  - boundary schema for time-bounded exceptions when the preferred
  trust pattern is not followed.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  - exact-build artifact family vocabulary this matrix maps onto.
- [`/docs/release/release_artifact_graph.md`](./release_artifact_graph.md)
  - release-artifact graph and bundle-completeness rules.
- [`/docs/release/build_farm_and_remote_cache_policy.md`](./build_farm_and_remote_cache_policy.md)
  - trust-domain and clean-room rebuild policy.
- [`/docs/governance/provenance_and_compliance_baseline.md`](../governance/provenance_and_compliance_baseline.md)
  - repository baseline for provenance, SBOM, licensing, and
  compliance hygiene.
- [`/docs/security/emergency_distribution_policy.md`](../security/emergency_distribution_policy.md)
  - mirror-safe emergency distribution, manual import, supersedence,
  and expiry policy.
- [`/docs/extensions/registry_and_offline_bundle_seed.md`](../extensions/registry_and_offline_bundle_seed.md)
  - extension registry, mirror, and offline-bundle contract.
- [`/docs/identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md)
  - signed policy-bundle, entitlement, and admin-audit packet seed.
- [`/docs/ai/provider_model_registry_contract.md`](../ai/provider_model_registry_contract.md)
  - provider, model, local-model-pack, and external-tool gateway
  registry contract.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  - support/export bundle contract.

Normative sources this matrix projects from:

- `.t2/docs/Aureline_PRD.md` sections on build-vs-buy, standards,
  release engineering, extension supply-chain controls, code signing,
  emergency response, and contributor provenance.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections
  25.7 and 25.8, plus Appendix AO.
- `.t2/docs/Aureline_Technical_Design_Document.md` section 11.2.1,
  Appendix N, and the provenance/freshness/client-scope matrix.

If this document disagrees with those sources, those sources win and
this document plus the machine-readable rows and schema update in the
same change.

## Policy

Aureline reuses established supply-chain trust patterns by default:

- **TUF-style metadata** for delegated update or registry metadata,
  channel separation, target digests, expiry, rollback protection, and
  monotonic supersedence.
- **Sigstore-style or equivalent identity-backed signing** for
  official release, extension, policy, model-pack, and emergency
  artifacts where platform-native signing alone is not enough to
  explain signer identity.
- **SLSA-style provenance** for official build pipelines, with
  exact-build identity, build recipe, input digest, artifact digest,
  and controlled signing or attestation domain linkage.
- **SPDX as the primary SBOM and license inventory output** for
  official release artifacts.
- **CycloneDX export** where security tooling, extension distribution,
  VEX-style workflows, or enterprise interchange need that shape.
- **OCI Distribution-compatible or equivalent content-addressed
  transport** for mirrorable registry, extension, model, update,
  policy, and offline-bundle paths.

An artifact family that cannot follow the preferred pattern is not
allowed to land in a vague "secure later" bucket. It must either narrow
the release claim or carry a
[`provenance_stack_exception`](../../schemas/release/provenance_stack_exception.schema.json)
record with an owner, risk statement, mitigation, expiry, review
forum, and compensating evidence.

## Control Expectations

| Artifact family group | Examples and exact-build families | Preferred trust patterns | Required SBOM / provenance outputs | Mirror, rotation, and supersedence handling |
|---|---|---|---|---|
| Release payloads | Desktop binary, CLI binary, SDK library, source bundle, reproducibility pack, SBOM, signed attestation | TUF-style channel metadata, platform-native signing, Sigstore-style or equivalent identity-backed signing, SLSA-style provenance | SPDX primary SBOM, CycloneDX when security interchange is needed, exact-build identity, signed attestation, reproducibility or clean-room proof | Mirrors preserve artifact digests and signatures; trust-root rotation requires signed continuity or cross-signing; emergency supersedence uses channel freeze, revocation, and replacement release evidence |
| Update metadata | Channel manifests, update feeds, rollback targets, install topology rows | TUF-style delegated metadata with timestamp/snapshot/targets separation and monotonic expiry | Provenance links to exact-build identities and release-evidence packet refs; SBOMs are referenced, not re-emitted | Mirrors must carry target digests, expiry, and revocation state; stale metadata cannot widen claims; supersedence replaces the manifest with a signed successor |
| Extension distribution artifacts | Registry rows, official extension packs, SDK publication bundles, offline bundles, mirror continuity rows | OCI-compatible content-addressed transport, TUF-style registry metadata, Sigstore-style publisher identity, SLSA-style provenance for verified tiers | SPDX or CycloneDX for verified/high-trust packages, publisher attestation refs, permission manifest refs | Mirrors may narrow but not widen trust; publisher key rotation and namespace transfer require continuity records; revocation and kill-switch metadata travel through public, mirror, and offline paths |
| Policy and emergency artifacts | Policy bundles, entitlement snapshots, admin-audit packets, emergency actions, manual-import receipts, trust-root rotations | Signed append-only or monotonic policy objects, TUF-style expiry and supersedence for distributed policy, identity-backed signing for authoritative issuers | Provenance references signer identity, policy epoch, distribution source, and manual-import receipt; SBOM is not applicable unless a policy bundle carries executable payloads | Mirror and manual-import receipts preserve source, signer continuity, and freshness; trust-root rotations require continuity statements; expired or superseded policy cannot remain silently authoritative |
| Docs and public contract artifacts | Docs pack, reference pack, schema export, destination descriptors, reviewed-pack binding | Signed or digest-pinned docs packs, content-addressed mirror transport, SLSA-style publication provenance where generated from release pipelines | SPDX notices for bundled third-party content, exact-build or source revision refs, docs-version-match evidence | Mirrored docs retain pack revision, source, and freshness; schema/docs supersedence uses same-change-set release truth; stale packs render downgraded claims |
| Model and AI artifacts | Local model packs, provider registry entries, model registry entries, external-tool gateway rows | Content-addressed pack identity, identity-backed signatures for official packs, OCI-compatible or equivalent mirrors, provenance and license/model-card refs | License/provenance packet for pack origin, dependency/SBOM export when executable runtime dependencies are shipped, capability evidence refs | Mirrors preserve pack digest and signer continuity; withdrawn or quarantined models supersede through registry audit events; trust-root changes require explicit pack or provider continuity |
| Symbol and debug sidecars | IDE symbols, CLI symbols, source maps, crash-symbol archives, profiler sidecars | Exact-build identity linkage, signed manifest refs, release-artifact graph retention controls, SLSA-style provenance for generated sidecars | Attestation or manifest links to the originating build identity; SBOM is usually referenced from the paired payload rather than duplicated | Sidecar mirrors cannot re-anchor identity; retention follows the paired release family; emergency supersedence can revoke or quarantine affected sidecar node sets without deleting unaffected payloads |
| Support and release-control artifacts | Support runbook bundle, release-evidence packet, support bundle, advisory/revocation records | Signed packet or digest-pinned evidence identity, exact-build refs, provenance/freshness/client-scope descriptors | Evidence packet refs, exact-build refs, advisory refs, and redaction/export posture; SBOMs are referenced when the support packet discusses released bytes | Support and emergency packets preserve mirror/manual-import receipts; superseded evidence remains visible by stable id; trust-root or advisory rotations cite the signed successor record |

## Crosswalk Rules

The machine-readable companion contains the full row set. Reviewers
use the following rules when reading it:

1. Every `artifact_family_group` row names the exact-build family
   classes it covers when those classes already exist in
   `artifact_family_map.yaml`.
2. Rows that govern conceptual artifact families not yet represented
   as exact-build families still name the control records and schemas
   that own them.
3. `sbom_document` and `signed_attestation` are release payload
   controls, not optional side notes. They remain promotion-bearing
   once the channel requires supply-chain proof.
4. Mirror continuity records inherit origin identity. A mirror may
   narrow trust, freshness, or installability, but it may not repackage,
   re-sign as if it were the origin, or widen a stale/revoked artifact.
5. Trust-root rotation is a controlled change. It must cite a continuity
   statement, a cross-signing transition, or an exception record.
6. Emergency supersedence is explicit. Revocations, channel freezes,
   kill switches, policy disables, or replacement manifests carry
   signed successor refs and do not hide the superseded record.
7. Missing support for a preferred pattern is handled by claim
   narrowing or by a schema-valid exception. Silent omission is
   non-conforming.

## Exception Policy

A provenance-stack exception is admissible only when all of the
following are true:

- the affected family group and preferred trust controls are named;
- the exception owner and evidence owner are accountable handles or
  team aliases;
- the risk statement, user/operator-visible effect, mitigation, and
  planned exit are reviewable;
- at least one compensating control or evidence ref is present;
- the expiry is time-bounded and has reapproval triggers;
- the review forum is one of the governed release, security, product,
  architecture, ecosystem, support, or shiproom forums; and
- release packets, shiproom review, mirror publication, and support
  exports can cite the exception by stable id until it closes.

Exceptions do not weaken emergency behavior. A family that deviates
from the preferred trust pattern still must preserve mirror receipts,
trust-root rotation history, revocation/supersedence state, and
support reviewability.
