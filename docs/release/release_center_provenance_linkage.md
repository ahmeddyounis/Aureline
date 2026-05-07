# Release-center provenance linkage and support crosswalk

This document freezes the linkage rules that keep release-center rows,
exact-build identity, About/provenance, service-health disclosures, and
support exports aligned. The goal is that publication truth does not
fork from crash, symbolication, or support surfaces.

Release-center objects and support bundles MUST NOT rely on internal-only
paths, operator notes, or per-surface version strings to re-identify a
released build. They MUST instead quote stable ids and the shared
exact-build identity record.

Companion artifacts:

- [`/schemas/release/release_provenance_crosswalk.schema.json`](../../schemas/release/release_provenance_crosswalk.schema.json)
  - boundary schema for the machine-readable crosswalk and its worked
    linkage cases.
- [`/artifacts/release/release_support_crosswalk.yaml`](../../artifacts/release/release_support_crosswalk.yaml)
  - machine-readable mapping of which fields must match across surfaces.
- [`/fixtures/release/release_center_linkage_cases/`](../../fixtures/release/release_center_linkage_cases/)
  - worked cases for normal publication, mirroring, hotfix symbol
    refresh, revocation, rollback, and stale-docs support capture.

Upstream contracts this document joins rather than restating:

- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  - the canonical build identity join key and artifact-family vocabulary.
- [`/docs/release/release_center_object_model_contract.md`](./release_center_object_model_contract.md)
  and
  [`/schemas/release/release_center_object.schema.json`](../../schemas/release/release_center_object.schema.json)
  - release-center ids, exact-build backreferences, and support linkage.
- [`/schemas/release/publish_target.schema.json`](../../schemas/release/publish_target.schema.json)
  - publish-target ids and their exact-build backreferences.
- [`/docs/about/about_provenance_and_boundary_contract.md`](../about/about_provenance_and_boundary_contract.md)
  and
  [`/schemas/about/about_card.schema.json`](../../schemas/about/about_card.schema.json)
  - the About/provenance join fields and user-visible provenance summary.
- [`/schemas/release/whats_new_card.schema.json`](../../schemas/release/whats_new_card.schema.json)
  - the service-health / release-notes disclosure envelope that must
    carry exact-build refs when claim-bearing.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md),
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json),
  and
  [`/schemas/support/support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json)
  - the support bundle build/install truth blocks and export manifest.
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  - advisory rows that bind security disclosure to exact-build identities.
- [`/schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json)
  - upgrade/rollback manifests that bind install flows to exact-build ids.

Normative sources this linkage must obey:

- `.t2/docs/Aureline_Technical_Design_Document.md` sections on provenance,
  release-center object model, and cross-surface truth vocabulary.
- `.t2/docs/Aureline_Milestones_Document.md` sections on exact-build
  identity spanning release-center, symbolication, and support exports.

If this document disagrees with those sources, those sources win and the
crosswalk and worked cases update in the same change.

## Canonical join keys

This linkage is defined in terms of stable ids rather than UI strings:

- `exact_build_identity_ref` is the primary cross-surface join key.
  Every release-bearing surface MUST carry it when it claims to speak
  about a shipped build.
- `build_id` is a human-facing label (useful for screenshots and basic
  triage) but MUST NOT be treated as a unique identifier on its own.
- `artifact_ref` identifies one concrete artifact-family member
  (binary, symbol archive, source map bundle, docs pack, SBOM, etc.) as
  an opaque stable id.
- `release_center_event_ref`, `candidate_id`, and `publish_target_id`
  identify release-center rows; these ids MUST be stable across the UI,
  headless publication records, and support reconstruction.
- `support_bundle_id` (an opaque id) identifies a support bundle
  archive; release-center rows and update manifests MUST refer to
  support bundles by stable ids, never by storage paths or URLs.

## Exact-build backreference contract

Release-center rows bind artifact families back to exact-build identity
records through `exact_build_backreferences[]` rows. A backreference row
is the canonical place to state:

1. which artifact-family member is being discussed (`artifact_ref` and
   `artifact_family_class`);
2. which exact build it belongs to (`exact_build_identity_ref`,
   `build_id`);
3. which public provenance projections cite it (`provenance_row_refs`,
   `about_provenance_row_refs`); and
4. which support exports are known to carry it (`support_bundle_refs`).

Backreference rows MUST NOT embed:

- raw file paths (local, internal, or vendor-hosted);
- raw URLs to internal object stores; or
- free-text “where to find it” operator notes.

If an artifact is mirrored or carried through air-gapped media, the
artifact’s *identity* MUST remain anchored to the original
`exact_build_identity_ref`; mirroring may narrow freshness or
availability, but it MUST NOT mint a new build identity for the same
build.

## Human-visible provenance string contract

Multiple surfaces render a short provenance line (release center,
About/provenance, service health, support-bundle preview/export). This
line MUST remain consistent because it is the string most likely to
appear in screenshots, support tickets, crash reports, and external
communications.

### Required fields (semantic content)

Every provenance line MUST communicate, at minimum:

- product and version label (user-facing);
- channel class (`stable`, `lts`, `beta`, `preview`, `nightly`, `hotfix`,
  or `dev_local`);
- one stable join key (`exact_build_identity_ref` or a stable alias to
  it); and
- origin posture (for example official vs mirrored vs side-loaded) using
  the same vocabulary as About/provenance and the provenance-badge model.

### Forbidden content

The provenance line MUST NOT include:

- internal build-farm hostnames or job identifiers;
- internal mirror endpoints or storage paths;
- raw signing material identifiers or secret-bearing tokens.

### Support-bundle summary alignment

Support bundles carry a short build-truth note (`build_truth_note`) and
an export manifest with build identity fields. The build-truth note MUST
not contradict the provenance line shown on release-center, About, or
service-health surfaces for the same `exact_build_identity_ref`.

When docs packs are stale or unavailable, the provenance line MUST stay
anchored to `exact_build_identity_ref` and the wording MUST narrow by
stating that the docs pack is stale/unavailable rather than silently
switching to a version-only claim.

## Cross-surface linkage rules

The machine-readable crosswalk in
[`/artifacts/release/release_support_crosswalk.yaml`](../../artifacts/release/release_support_crosswalk.yaml)
records the mechanical rules. At a high level:

1. Release-center rows quote `exact_build_identity_ref` via
   `exact_build_backreferences[]`.
2. Publish targets quote the same `exact_build_identity_ref` set via
   `exact_build_backreferences[]` and list the same `support_bundle_refs`
   when a support export is relied on for reconstruction.
3. Update manifests quote `exact_build_identity_ref` per artifact row
   and carry `support_refs` that include support packets/bundles used
   for rollback or reconstruction.
4. About/provenance cards quote `build_identity.exact_build_identity_ref`
   and render provenance-badge terms derived from the same underlying
   evidence.
5. Service-health / what’s-new cards quote `subject.exact_build_identity_ref`
   whenever they make a claim about “what you are running”.
6. Support bundles quote `build_and_install_context.primary_exact_build_identity_ref`
   and MUST preserve enough stable refs that support can resolve a
   release-center row (either directly through cited ids, or indirectly
   through the exact-build id and the published crosswalk).

## Worked linkage cases

The fixture cases under
[`/fixtures/release/release_center_linkage_cases/`](../../fixtures/release/release_center_linkage_cases/)
demonstrate required behavior for:

- a normal release publication;
- a mirrored release publication;
- a hotfix where symbol/source-map supportability is refreshed after
  publication without changing the exact-build identity anchor;
- a revoked artifact where the provenance string narrows but the build
  identity does not fork;
- a rollback candidate where support can reconstruct the release-center
  row from the bundle; and
- a support bundle captured from a build whose docs pack is stale or
  unavailable (wording narrows; ids remain stable).

