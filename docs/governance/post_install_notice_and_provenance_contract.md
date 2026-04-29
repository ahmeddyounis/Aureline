# Post-install notice and provenance disclosure contract

This contract defines the disclosure record Aureline exposes after an
artifact is installed, imported, mirrored, side-loaded, or exported. It
exists so users, administrators, procurement reviewers, support, and
downstream redistributors can inspect notices, licenses, provenance,
SBOMs, signatures, attestations, build identity, mirror freshness, and
revocation state without depending on the original download page,
installer UI, or private release notes.

The contract is pre-implementation. It defines the record shape,
vocabulary, access-point rules, and fixture corpus. It does not author
legal notice text, implement a notice portal, build an updater, or
verify signatures.

## Companion Artifacts

- [`/schemas/governance/post_install_disclosure.schema.json`](../../schemas/governance/post_install_disclosure.schema.json)
  - boundary schema for one `post_install_disclosure_record`.
- [`/fixtures/governance/post_install_cases/`](../../fixtures/governance/post_install_cases/)
  - worked cases for official signed builds, mirrored artifacts with
  stale revocation snapshots, side-loaded extensions, and generated
  exports with redistribution hints.
- [`/docs/governance/provenance_badge_contract.md`](./provenance_badge_contract.md)
  - shared source-class, verification, license, notice, support,
  freshness, and export-safe provenance vocabulary.
- [`/docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  - release artifact graph and publication-completeness rules.
- [`/docs/release/update_and_rollback_contract.md`](../release/update_and_rollback_contract.md)
  - update, rollback, mirror import, and revocation reconstruction
  contract.
- [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
  - generated-artifact lineage record cited by generated export
  disclosures.

## Normative Sources Projected Here

- `.t2/docs/Aureline_PRD.md` sections on standards, SBOM/license
  inventory, official build IDs, signed updates, third-party notices,
  and open-source compliance.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix AO
  for build/provenance publication, Appendix AS for revocation
  evidence, Appendix BL for license/SBOM exports, and Appendix DE for
  generated-artifact provenance.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix CX for
  post-install notice and provenance rows.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` section 16.26
  and the release provenance card templates.

If this document disagrees with those sources, those sources win and
this contract, schema, and fixtures update in the same change.

## Invariants

1. **The subject is explicit.** Every disclosure states whether it is
   describing the product build, an installer payload, an extension or
   framework pack, a mirrored transport artifact, or a generated user
   artifact.
2. **Missing data is visible.** Missing, partial, hidden, unknown,
   stale, or policy-blocked provenance, notice, license, SBOM,
   signature, attestation, lineage, or revocation data is represented
   by a typed row. Silence never means "clean".
3. **Source class and subject class stay separate.** `Official`,
   `Mirrored`, `Side-loaded`, and `Unknown provenance` describe source
   or transport posture. They do not replace the artifact subject.
4. **Trust evidence is layered.** Build ID, channel, digest, signature,
   attestation, SBOM, notice inventory, license state, revocation
   freshness, and mirror or import receipt are separate fields.
5. **Post-install access survives.** Disclosure remains reachable from
   About, update center, installed-state inspectors, diagnostics
   exports, and review sheets after installation. Surface-specific
   details may add links, but they may not invent a different truth.
6. **Exports preserve caveats.** Diagnostics, support, public-proof,
   offline-review, and generated-artifact exports carry the same
   source class, subject class, evidence refs, gaps, stale states, and
   omission reasons as the live disclosure.

## Record Scope

A `post_install_disclosure_record` MAY describe these subject classes:

| `surface_subject_kind` | Examples | Boundary statement |
|---|---|---|
| `product_build` | Desktop shell, CLI binary, helper shipped with the product | Describes the installed Aureline build or helper, not extensions or generated user files. |
| `installer_payload` | Installer, package-manager payload, portable bundle | Describes the transport and payload installed onto the machine. |
| `extension_package` | Installed registry extension, side-loaded extension archive | Describes one extension package and its post-install state. |
| `framework_pack` | First-party or third-party framework tooling pack | Describes the pack and its update/revocation status. |
| `mirrored_transport_artifact` | Offline update bundle, mirrored docs pack, mirrored registry package | Describes both upstream origin and mirror/receipt freshness. |
| `generated_user_artifact` | Exported SBOM, generated report, notebook export, packaged generated output | Describes generated lineage and redistribution cues, not product release trust. |

The record carries opaque refs and reviewable sentences. Raw artifact
bytes, raw signatures, raw SBOM bodies, raw notice text, raw registry
URLs, raw license files, raw advisory payloads, private mirror
endpoints, and customer identifiers do not cross this boundary.

## Source Classes

| `source_class` | Required label | Minimum cue |
|---|---|---|
| `official` | `Official` | Origin or producer ref, build/generation identity where applicable, and current verification state. |
| `mirrored` | `Mirrored` | Upstream origin ref, mirror ref, revocation snapshot ref, freshness cue, and refresh/import action. |
| `side_loaded` | `Side-loaded` | Side-load review ref, explicit support/auto-update limitation, and every missing or unverified evidence axis. |
| `unknown_provenance` | `Unknown provenance` | Unknown origin cue, unsupported or review-required posture, and missing-data rows for unresolved evidence. |

Policy approval, admin pinning, or mirror availability does not replace
cryptographic verification. It renders as a separate disclosure or
evidence ref.

## Required Field Groups

| Group | Purpose |
|---|---|
| `review_context` | Names the subject class and explains what the surface is, and is not, describing. |
| `artifact` | Names artifact class, display name, version/digest, build ID, channel, exact-build ref, installer receipt, and generated-lineage ref. |
| `source` | Carries source class, source label, origin/upstream/mirror/side-load refs, acquisition route, and source disclosure. |
| `verification` | Separates signature, attestation, checksum, revocation state, revocation freshness, snapshot ref, checked time, and evidence refs. |
| `notice_inventory` | Separates license state, notice state, notice inventory availability, notice refs, SBOM state, SBOM formats, and compliance disclosure. |
| `visible_cues` | Provides short user-visible source, provenance, license, notice, and revocation cues, plus typed missing/partial data rows. |
| `access_points` | Lists every durable surface from which the disclosure can be reached after install. Unavailable access is visible with a reason. |
| `actions` | Names open/export/inspect/refresh/review actions by stable refs. |
| `redistribution` | Explains whether the artifact is safe to redistribute as-is, needs notice/license review, is blocked by policy, or is not applicable. |
| `export_projection` | States support, public-proof, diagnostics, and offline-review projection refs plus redaction and omission behavior. |

## Access-Point Rules

Every record MUST include access-point rows for:

- `about`;
- `update_center`;
- `installed_state_inspector`;
- `diagnostics_export`; and
- `review_sheet`.

Additional rows MAY cover extension details, installer receipts,
generated-artifact viewers, export review, marketplace or package
detail, support bundles, and offline review.

Access-point rows use these reachability states:

| `reachability_class` | Meaning |
|---|---|
| `available` | The surface can open the disclosure or a scoped detail view now. |
| `available_read_only` | The surface can inspect the disclosure but cannot mutate or refresh it. |
| `unavailable_visible` | The access point is not available in this context, and the row explains why. |
| `policy_hidden_visible` | Policy hides detail, but the existence and omission reason remain visible. |
| `not_applicable_visible` | The access point does not apply to this subject class, and that non-applicability is explicit. |

A link that only existed in the installer or original download page is
not sufficient. A diagnostics export or support bundle that includes
the artifact must include the disclosure ref or an omission reason.

## Minimum Cue Matrix

| Subject | Required cues |
|---|---|
| Product build | build ID, channel, exact-build ref, signature state, attestation state, SBOM formats or missing SBOM row, notice inventory state, revocation freshness, About and update-center actions. |
| Installer payload | installer receipt ref, payload build ID, platform trust or signature state, notice/SBOM availability, post-install disclosure action, rollback/repair refs where applicable. |
| Extension or framework pack | package identity, publisher/producer ref, source class, permission or side-load review ref, signature/attestation state, license/notice state, compatibility or support limitation, revocation freshness. |
| Mirrored transport artifact | upstream origin ref, mirror ref, mirror/acquisition route, revocation snapshot ref, snapshot freshness, stale/expired cue when applicable, refresh or manual-import action. |
| Generated user artifact | generated-lineage ref, generator/build identity where known, input/output digest refs by lineage, notice/license state for bundled content, open-source-input action, and redistribution hint. |

## Missing and Partial Data

The `visible_cues.missing_or_partial_data` array is required even when
empty. When any of the following are unavailable, stale, partial,
unknown, or policy-hidden, a row MUST name the affected `data_class`,
visible label, disclosure, and resolution action if one exists:

- provenance;
- signature;
- attestation;
- SBOM;
- license;
- notice inventory;
- revocation snapshot;
- mirror origin;
- generated lineage; and
- redistribution terms.

The UI vocabulary is deliberately plain: `Not provided`, `Partial`,
`Unknown provenance`, `Stale`, and `Policy hidden` are acceptable cues.
A blank cell, omitted row, or disabled action without explanation is
non-conforming.

## Redistribution Hints

Generated and exported artifacts can outlive the installed product.
Their disclosure therefore carries a `redistribution_hint_class`:

| Class | Meaning |
|---|---|
| `not_applicable` | This subject is not a user-redistributable generated artifact. |
| `allowed_with_notice` | Required notice and license refs are present. |
| `review_before_redistribution` | The artifact can be exported, but redistribution requires reviewing listed notices, licenses, lineage, or policy refs. |
| `blocked_by_policy` | Policy prevents redistribution. |
| `unknown_review_required` | Required redistribution inputs are missing or unknown. |

The hint is advisory product truth, not legal advice. It must point to
notice, license, lineage, or policy refs rather than embedding legal
content.

## Surface Projection Matrix

| Surface | Required projection |
|---|---|
| About | Current product build disclosure, third-party notice action, SBOM/export action where applicable, and route to installed extensions/framework packs. |
| Update center | Current and target build disclosures, mirror/freshness/revocation cues, rollback/repair refs, and stale-snapshot refresh action. |
| Installed-state inspector | Per-artifact subject class, source class, verification state, notice/license/SBOM state, and open-detail actions. |
| Diagnostics export | Disclosure id, subject/source classes, evidence refs, stale/missing rows, redaction class, and omission reasons. |
| Review sheet | Full detail rows for install/update/side-load/export review, including missing data and redistribution hints. |
| Generated-artifact viewer/export review | Generated-lineage ref, source input action, output digest or lineage ref, notice/license state, and redistribution hint. |

## Governance Gates

- A post-install surface that hides missing provenance, notice, license,
  SBOM, signature, attestation, generated-lineage, or revocation data is
  non-conforming.
- A mirrored artifact whose revocation snapshot is stale or expired must
  render stale/expired state and a refresh/import action; it may not
  render as merely "verified".
- A side-loaded artifact may be usable after review, but it must not
  inherit official update, support, or license claims by proximity to
  the product build.
- A generated export may cite the official build or generator that
  produced it, but the disclosure must still say it is a generated user
  artifact and carry redistribution cues.
- Support and public-proof exports may redact detail, but they must keep
  the same disclosure id, subject class, source class, stale/missing
  states, and omission reasons.

## Fixture Corpus

The worked corpus under
[`/fixtures/governance/post_install_cases/`](../../fixtures/governance/post_install_cases/)
covers:

- official signed build;
- mirrored artifact with stale revocation snapshot;
- side-loaded extension; and
- generated export with redistribution hint.
