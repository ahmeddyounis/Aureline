# Provenance badge, license row, notice row, and supply-chain status contract

This contract freezes one vocabulary for provenance, licensing, notice,
and supply-chain posture across install review, marketplace-style
discovery, release publication, About / update surfaces, support
exports, public-proof packets, and mirror or offline review. It exists so
compact badges, full detail rows, and evidence packets all describe the
same facts instead of translating supply-chain state into surface-local
copy.

The contract is pre-implementation. It defines the reusable record shape,
allowed vocabulary combinations, projection rules, export behavior, and
fixture corpus. It does not implement a registry, marketplace, updater,
package resolver, release signer, or support-bundle generator.

## Companion Artifacts

- [`/schemas/governance/provenance_badge.schema.json`](../../schemas/governance/provenance_badge.schema.json)
  - boundary schema for one `provenance_badge_record`.
- [`/fixtures/governance/provenance_badge_cases/`](../../fixtures/governance/provenance_badge_cases/)
  - worked records covering signed official assets, mirrored assets,
  side-loaded local archives, third-party imports, stale review, and
  unknown provenance.
- [`/docs/governance/post_install_notice_and_provenance_contract.md`](./post_install_notice_and_provenance_contract.md)
  - durable post-install disclosure contract that consumes this
  vocabulary for product builds, installers, extensions/framework
  packs, mirrored or offline artifacts, and generated user exports.
- [`/docs/governance/provenance_and_compliance_baseline.md`](./provenance_and_compliance_baseline.md)
  - repository provenance, license, notice, and placeholder SBOM
  baseline.
- [`/docs/governance/dependency_review_policy.md`](./dependency_review_policy.md)
  - dependency and import row admission policy.
- [`/docs/release/supply_chain_trust_framework_matrix.md`](../release/supply_chain_trust_framework_matrix.md)
  - release artifact-family trust-pattern matrix.
- [`/docs/verification/install_review_packet.md`](../verification/install_review_packet.md)
  - install-review packet that consumes this vocabulary on package and
  extension flows.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  - boundary manifest whose rows can cite provenance posture without
  redefining source or support classes.

## Normative Sources Projected Here

- `.t2/docs/Aureline_PRD.md` sections on extension review, SBOM /
  provenance verification, mirrorable registries, air-gapped deployment,
  and contributor provenance.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections on
  supply-chain trust patterns, artifact / registry services,
  exact-build identity, package / license / vulnerability architecture,
  and mirrorable docs or registry artifacts.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections on registry
  mirrors and offline bundles, publisher continuity, marketplace
  discovery, package explorer, install review, and mirror provenance.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` "License, provenance, and
  supply-chain component contract".
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` appendices for
  notice inventory rows, SBOM / provenance cards, package explorer rows,
  and artifact provenance cards.

If this contract disagrees with those sources, those sources win and
this contract, schema, and fixtures update in the same change.

## Summary

Every provenance-bearing surface projects one
`provenance_badge_record`. The record separates:

- compact badge truth, which is the small set of cues safe to render in
  list rows, cards, About panels, package results, and screenshots;
- full detail rows, which carry the license, notice, signature,
  attestation, SBOM, trust-root continuity, advisory, upstream-health,
  support-class, freshness, and mirror/offline verification posture; and
- linkage and export fields, which let release packets, dependency
  ledgers, install-review sheets, support bundles, and offline review
  packets quote the same state by stable ref.

Unsupported, unknown, stale, not-reviewed, missing, partial, mirrored,
or side-loaded state is always explicit. Absence of a row never means
"verified".

## Record Scope

A `provenance_badge_record` MAY describe any of these subject classes:

| Subject class | Examples |
|---|---|
| `release_artifact` | desktop binary, CLI binary, SDK library, docs pack, schema export |
| `update_metadata` | channel manifest, rollback target, update feed row |
| `extension_package` | registry package, offline extension bundle, private mirror row |
| `package_dependency` | workspace dependency, lockfile node, package-manager result |
| `third_party_import` | copied code, bundled docs, mirrored pack, binary asset |
| `policy_bundle` | signed policy bundle, entitlement snapshot, emergency action |
| `model_or_tool_pack` | local model pack, external tool pack, provider gateway row |
| `support_or_proof_packet` | support bundle, public-proof packet, release evidence packet |
| `local_archive` | user-supplied archive, manually attached profiler / symbol artifact |

The record carries opaque refs and typed posture. Raw artifact bytes,
raw SBOM bodies, raw signatures, raw registry URLs, raw license files,
raw notice text, raw advisory payloads, and raw customer identifiers do
not cross this boundary.

## Compact Badge Contract

Compact badges are for scan speed. They MAY render only these fields:

- `badge_label`
- `verification_summary_class`
- `badge_tone`
- `freshness_class` when stale, pending review, or unknown
- `support_class` when unsupported, preview, or narrower than stable
- `advisory_indicator` when active, fixed-with-history, revoked, or
  compromise-suspected
- `open_details_action_ref`

Compact badges MUST NOT be the only place that communicates:

- license obligations or notice completeness;
- trust-root rotation, continuity, or key-compromise history;
- SBOM format, attestation kind, transparency-log state, or checksum
  algorithm;
- mirror snapshot, offline import receipt, freshness floor, or manual
  verification posture;
- upstream health, maintainer risk, or support-window expiry; or
- why an unsupported, unknown, not-reviewed, stale, partial, or
  publisher-asserted state was reached.

Those facts belong in full detail rows or a review sheet. A compact
badge whose state is not fully verified MUST provide an open-details
route.

## Source and Badge Vocabulary

| `source_class` | Required `badge_label` | Meaning |
|---|---|---|
| `official` | `Official` | Produced by an Aureline-controlled or foundation-approved lane. Verification may still fail or be stale, but the asserted origin is official. |
| `mirrored` | `Mirrored` | Obtained from a mirror or offline bundle that preserves upstream identity. Mirror state must be disclosed separately. |
| `side_loaded` | `Side-loaded` | User, admin, or local workflow supplied the artifact outside a registry / update path. Verification may be present or absent but installability is review-gated. |
| `third_party_import` | `Third-party import` | Copied, bundled, vendored, or mirrored third-party bytes governed by the import register. |
| `community` | `Community` | Published by an unverified community publisher under baseline checks. |
| `policy_pinned` | `Policy pinned` | Admin or policy source pins or approves this item; policy does not replace cryptographic verification. |
| `local_build` | `Local build` | Built locally or in a developer lane; never equivalent to official release proof. |
| `unknown_provenance` | `Unknown provenance` | Origin, signer, or source ledger cannot be resolved. This state must remain visible and cannot collapse into an empty badge slot. |

`badge_tone` is a visual severity hint only. It is derived from the
detail rows and MUST NOT be treated as evidence.

## Verification Vocabulary

| Axis | Values | Projection rule |
|---|---|---|
| `signature_state` | `signed_verified`, `signed_unverified`, `signature_missing`, `signature_revoked`, `signature_mismatch`, `not_applicable` | Compact badge may summarize; detail row names signer ref, transparency-log ref, and failure / revocation state. |
| `attestation_state` | `attestation_verified`, `attestation_present_unverified`, `attestation_missing`, `attestation_stale`, `attestation_policy_blocked`, `not_applicable` | Signature and attestation are separate. A verified signature does not imply verified provenance. |
| `sbom_state` | `sbom_attached_verified`, `sbom_attached_unverified`, `sbom_missing`, `sbom_stale`, `sbom_policy_blocked`, `not_applicable` | Detail row names SPDX / CycloneDX availability by opaque ref. |
| `checksum_state` | `checksum_verified`, `checksum_present_unverified`, `checksum_missing`, `checksum_mismatch`, `not_applicable` | Checksums are integrity cues, not publisher identity. |
| `trust_root_state` | `trust_root_current`, `trust_root_rotated_with_continuity`, `trust_root_rotation_pending_review`, `trust_root_revoked`, `trust_root_unknown`, `not_applicable` | Detail row carries continuity statement or rotation / revocation refs. |
| `mirror_verification_state` | `origin_verified_through_mirror`, `mirror_snapshot_current`, `mirror_snapshot_stale`, `offline_receipt_verified`, `offline_receipt_expired`, `mirror_continuity_broken`, `not_applicable` | Mirrored and offline records must disclose this even when the origin signature verifies. |

## License, Notice, and Status Vocabulary

| Axis | Values | Projection rule |
|---|---|---|
| `license_state` | `license_allowed`, `license_allowed_with_notice`, `license_restricted`, `license_policy_blocked`, `license_unknown`, `not_applicable` | Compact badges never summarize license obligations alone. A license row must be available on review, support, and export surfaces. |
| `notice_state` | `notice_complete`, `notice_partial`, `notice_missing`, `notice_not_required`, `notice_policy_hidden`, `notice_unknown` | Missing, partial, or hidden notices render as explicit rows. |
| `support_class` | `supported_stable`, `supported_preview`, `supported_limited`, `community_supported`, `unsupported`, `support_unknown` | Unsupported and unknown support state remains visible even if provenance is verified. |
| `freshness_class` | `current`, `warm_cached`, `stale_requires_review`, `review_pending`, `expired`, `unknown` | Stale or unknown freshness cannot widen a claim. |
| `upstream_health_state` | `healthy`, `watch`, `at_risk`, `blocked`, `not_reviewed`, `not_applicable` | Protected-path or import rows cite the dependency / import ledger and upstream scorecard. |
| `advisory_state` | `none_known`, `active_advisory`, `fixed_with_history`, `revoked_or_yanked`, `compromise_suspected`, `advisory_unknown` | Advisory history persists after a fix; compact badges may show only the indicator and route to detail. |
| `artifact_compromise_state` | `none_known`, `suspected`, `confirmed`, `remediated`, `unknown` | Suspected, confirmed, remediated, and unknown states require a detail row and support/export projection. |

## Allowed Combinations

The schema enforces the combinations below. Downstream surfaces may add
stricter policy, but they may not weaken these floors.

| Combination | Required behavior |
|---|---|
| Official verified asset | `source_class = official`, `badge_label = Official`, verified signature, current or rotated-with-continuity trust root, no active compromise, and release / SBOM / attestation refs where applicable. |
| Official but verification failed | Still uses `badge_label = Official`, but `verification_summary_class` is `verification_failed` or `revoked`, `badge_tone = blocked`, and the detail rows name the failed axis. It may not render as a healthy official badge. |
| Mirrored asset | `source_class = mirrored`, `badge_label = Mirrored`, non-null `mirror_ref`, non-`not_applicable` mirror verification state, and upstream origin refs. Mirror freshness and continuity are separate from origin signature state. |
| Side-loaded or local archive | `source_class = side_loaded`, `badge_label = Side-loaded`, side-load review refs, install / enable review refs when mutable, and no auto-update or stable-support implication unless a policy row explicitly grants it. |
| Third-party import | `source_class = third_party_import`, `badge_label = Third-party import`, dependency or import ledger refs, license row, notice row, upstream-health state, and provenance flow refs. |
| Stale review | `freshness_class = stale_requires_review` or `expired`; compact badge must show stale or expired state and the detail rows must name the last reviewed time and refresh trigger. |
| Unknown provenance | `source_class = unknown_provenance`, `badge_label = Unknown provenance`, no verified signature / attestation / SBOM claim, `support_class` is `unsupported` or `support_unknown`, and an unknown-provenance detail row is required. |

## Full Detail Rows

Every `provenance_badge_record` carries `detail_rows`. The minimum row
set is determined by subject class and state:

- `provenance_summary` is required on every record.
- `signature` is required unless `signature_state = not_applicable`.
- `attestation` is required when an attestation is present, expected, or
  policy-blocked.
- `sbom` is required when an SBOM is present, expected, stale, missing,
  or policy-blocked.
- `license` is required on dependency, import, package, extension,
  model/tool pack, and redistributed release subjects.
- `notice` is required when notices are complete, partial, missing,
  hidden, or unknown.
- `mirror_or_offline` is required when `source_class = mirrored` or
  `mirror_verification_state` is not `not_applicable`.
- `trust_root` is required when trust root is rotated, pending review,
  revoked, or unknown.
- `advisory_history` is required when advisory state is not `none_known`.
- `support_and_freshness` is required when support is limited,
  unsupported, unknown, stale, pending review, or expired.
- `unknown_provenance` is required when source class is unknown.

Detail rows use reviewable sentences and opaque refs. They do not embed
raw licenses, raw notices, raw vulnerability reports, raw signatures, or
raw private mirror URLs.

## Linkage Rules

A provenance record is not a replacement for upstream control artifacts.
It is a projection over them:

- Boundary-manifest rows cite `provenance_badge_ref` when a capability
  depends on a release, package, extension, policy, docs pack, mirror, or
  side-loaded artifact. The boundary row still owns local-core /
  managed-boundary classification.
- Dependency and import ledger rows remain canonical for dependency id,
  import id, owner, license class, upstream health, update cadence, and
  replacement posture. Provenance rows cite them through
  `dependency_ledger_refs` or `import_register_refs`.
- SBOM and attestation artifacts remain canonical by opaque refs.
  Provenance rows quote `sbom_refs`, `attestation_refs`,
  `transparency_log_refs`, and `provenance_statement_refs`; they do not
  inline the payloads.
- Extension or package install-review sheets quote the same
  `provenance_badge_record` used by marketplace rows, package explorer
  rows, and post-install disclosure views. Review sheets may add
  per-action risk, but may not restate provenance with different labels.
- Update-center, release-center, About, and post-install disclosure
  surfaces quote the same record as release evidence. Rollback or
  advisory rows cite the prior record and successor record rather than
  replacing history.
- Support bundles, public-proof packets, and offline review packets carry
  the export-safe projection, not a screenshot-only rendering.

## Export and Offline Behavior

The `export_projection` block states which fields survive:

- screenshots and compact card exports carry badge label, summary class,
  stale / unsupported / advisory indicators, and a stable detail ref;
- support bundles carry all detail row ids, opaque evidence refs, redaction
  class, and omission reasons;
- public-proof packets carry publishable refs, verification summary,
  support class, freshness, advisory state, and known gaps;
- offline review packets carry mirror snapshot refs, manual-import receipt
  refs, trust-root continuity state, last verified timestamp, freshness
  floor, and any stale / expired reason; and
- notice exports carry notice completeness state and notice artifact refs,
  not raw notice text unless the user or policy explicitly selects a
  broader export.

An offline or mirrored record that cannot verify freshness or continuity
renders `mirror_continuity_broken`, `offline_receipt_expired`, or
`stale_requires_review`; it never becomes a generic unavailable state.

## Surface Projection Matrix

| Surface | Required projection |
|---|---|
| Marketplace / package discovery | compact badge, source class, support class when narrower than stable, advisory indicator, open-details route |
| Install or update review sheet | compact badge plus signature, attestation, SBOM, license, notice, mirror/offline, advisory, and rollback / quarantine linkage rows |
| Dependency review | license, notice, upstream health, review freshness, advisory, and dependency / import ledger refs |
| Release publication | exact-build refs, signature, attestation, SBOM, trust-root continuity, advisory history, mirror/offline publication posture, and support class |
| About / update center | current running artifact ref, source class, verification summary, trust-root state, advisory indicator, and export support path |
| Post-install disclosure | what was installed, source class, verification summary, policy or support narrowing, and where to inspect license / notice / advisory rows |
| Support bundle / public proof | export projection, detail row ids, evidence refs, redaction class, omission reasons, and stale / unknown states |

## Governance Gates

- A source, verification, license, notice, advisory, support, or freshness
  value outside this vocabulary is non-conforming unless this contract
  and schema update in the same change.
- A compact-only badge for unknown, unsupported, stale, revoked,
  compromised, missing, or partial evidence is non-conforming.
- A mirror may narrow trust or freshness but may not re-anchor origin
  identity or hide broken continuity.
- Policy approval, admin pinning, or mirror availability does not replace
  cryptographic verification; it renders as a separate row.
- A support or public-proof export that drops provenance, notice, stale,
  unsupported, unknown, advisory, or mirror/offline state without an
  omission reason is non-conforming.

## Fixture Corpus

The worked corpus under
[`/fixtures/governance/provenance_badge_cases/`](../../fixtures/governance/provenance_badge_cases/)
covers the minimum acceptance set:

- signed official release asset;
- mirrored official asset with offline verification;
- side-loaded or local-archive asset;
- third-party import with license and notice posture;
- stale review with advisory history; and
- unknown provenance with unsupported / unknown state preserved.
