# Release status surface contract

This document freezes the release-status surface object family Aureline
uses to render candidate cards, version-bump rows, promotion timelines,
artifact provenance links, and rollback or revocation panels. It is a
projection of the release-center, publish-target, update-manifest, and
exact-build identity contracts; it is not a second release vocabulary or
release automation implementation.

The goal is to make release posture legible before full release-center
screens exist. A status card should answer the same questions in update
review, release notes, compatibility reports, migration guidance, support
exports, and headless dry-run output:

- what build or artifact family is under review;
- which channel, support window, compatibility posture, and stage apply;
- which evidence is current, stale, waived, missing, or not applicable;
- which exact artifacts, signatures, attestations, checksum bundles, and
  mirror/origin paths back the status;
- what recovery action exists and what artifact family or consumer state
  it changes.

Companion artifacts:

- [`/schemas/release/release_candidate_card.schema.json`](../../schemas/release/release_candidate_card.schema.json)
  - boundary schema for release-candidate cards, nested version-bump
    rows, target scope, evidence freshness, provenance linkage, support
    window, compatibility posture, deprecation timing, and feedability
    flags.
- [`/schemas/release/promotion_timeline_entry.schema.json`](../../schemas/release/promotion_timeline_entry.schema.json)
  - boundary schema for timeline entries used by release-center UI,
    headless publication output, release notes, audits, and support
    reconstruction.
- [`/schemas/release/rollback_revocation_panel.schema.json`](../../schemas/release/rollback_revocation_panel.schema.json)
  - boundary schema for rollback, yank, revoke, repin, and deprecate
    panels across installed, remote, registry, marketplace, mirror, and
    channel states.
- [`/fixtures/release/status_surface_cases/`](../../fixtures/release/status_surface_cases/)
  - worked status cases for local builds, staged candidates, promoted
    stable releases, and revoked or yanked artifact states.
- [`/docs/release/release_center_object_model_contract.md`](./release_center_object_model_contract.md),
  [`/schemas/release/release_center_object.schema.json`](../../schemas/release/release_center_object.schema.json),
  and
  [`/schemas/release/publish_target.schema.json`](../../schemas/release/publish_target.schema.json)
  - canonical release-center, publish-target, stage, audience, and
    publication-action object model this surface projects.
- [`/docs/release/update_and_rollback_contract.md`](./update_and_rollback_contract.md)
  and
  [`/schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json)
  - update, rollback, revoke, yank, repair, helper negotiation, and
    reserved release-center fields.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  - exact-build identity records every status surface resolves through.
- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md)
  - channel, branch, support-window, downgrade, and last-known-good
    vocabulary.

Normative sources this contract projects from:

- `.t2/docs/Aureline_Technical_Design_Document.md` sections on release
  center, version-bump proposals, publish targets, promotion timelines,
  rollback, and revocation architecture.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` appendices covering release
  candidate summary cards, artifact provenance cards, rollback/yank/
  revoke sheets, evidence rows, channel identity cards, support-window
  rows, compatibility summaries, deprecation notes, and migration
  review.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` sections on
  release candidate cards, version-bump rows, artifact provenance cards,
  promotion timelines, and rollback/revocation panels.
- `.t2/docs/Aureline_Milestones_Document.md` sections on release
  publication, provenance, attestation, advisory, and revocation
  discipline.

If this document disagrees with those sources, the normative sources win
and this document plus its companion schemas and fixtures update in the
same change.

## Scope

Frozen here:

- release-candidate card fields for version, channel, target scope,
  current stage, blockers, waivers, evidence freshness, rollback path,
  current artifact identity, and exact-build identity;
- version-bump row fields for previous and next versions, semantic
  change class, artifact scope, widened reach, owner, next gate,
  migration flags, and linked evidence;
- promotion-timeline entry fields for stage movement, semantic change,
  affected artifact scope, widened reach, owner, next gate, reversible
  window, break-glass state, and evidence links;
- rollback/revocation panel fields that distinguish `rollback`, `yank`,
  `revoke`, `repin`, and `deprecate`, and name whether the action
  changes installed clients, remote agents, registries, marketplace
  listings, update channels, mirrors, docs/schema packs, or support
  packets;
- artifact-provenance-card linkage for signature state, attestation
  state, checksum bundle, SBOM state, mirror/origin class, validation
  time, verifier class, and explicit limitations.

Out of scope:

- release-center screen implementation;
- release automation, signing services, package-manager adapters,
  registry credentials, mirror publishers, or update services;
- raw binaries, raw signatures, raw attestation bodies, raw logs, raw
  support bundles, raw credentials, or raw marketplace payloads.

## Invariants

1. **Status surfaces project release-center objects.** A status card,
   timeline entry, rollback panel, update review row, release note, or
   compatibility summary names release-center refs, publish-target refs,
   update-manifest refs, exact-build refs, artifact-family refs, and
   evidence refs. It does not mint an independent release state machine.
2. **Stage and recovery are shown together.** A candidate card always
   carries current stage, target scope, blockers or waivers, evidence
   freshness, artifact/build identity, and recovery path in the same
   object.
3. **Version changes are scoped.** A version-bump row names the artifact
   or public surface that changed. It never shows a naked version number
   without semantic change class, compatibility posture, and linked
   evidence.
4. **Promotion timelines say what widened.** A timeline entry names the
   source stage, destination stage, artifact families, audience or
   visibility change, owner, next gate, and evidence refs. Uploading
   bytes is not the same as widening a channel.
5. **Rollback and revocation verbs are not synonyms.** `rollback`,
   `yank`, `revoke`, `repin`, and `deprecate` have distinct action
   classes, affected state classes, consumer scopes, historical
   visibility, and recovery paths.
6. **Provenance is layered.** Signature, attestation, checksum bundle,
   SBOM, mirror/origin path, validation time, and exact-build identity
   are rendered as separate facts. Presence of one layer never implies
   another layer was verified.
7. **Support and compatibility posture remain first-class.** Channel
   identity, support window, compatibility posture, deprecation timing,
   and end-of-support risk travel with the candidate card so release
   notes, update review, compatibility reports, and migration guidance
   consume the same record.
8. **History remains inspectable.** A yanked, revoked, repinned,
   deprecated, or rolled-back artifact keeps historical publication refs
   and audit/export refs. Recommended state changes do not erase prior
   publication truth.

## Object Vocabulary

| Object | Required identity | Required content | Reconstruction rule |
|---|---|---|---|
| Release-candidate card | `card_id` plus `candidate_ref` | version, channel, target scope, current stage, blocker/waiver rows, evidence freshness, rollback path, artifact provenance links, version-bump rows, release-center/update-manifest refs, support and compatibility posture | Entry point for status surfaces. A card reconstructs candidate state from release-center and update-manifest refs. |
| Version-bump row | `version_bump_row_id` | prior version, next version, semantic change class, affected artifact scope, widened reach, owner, next gate, linked evidence, migration flags | Explains version intent for update review, release notes, compatibility reports, and migration guidance. |
| Promotion-timeline entry | `timeline_entry_id` | source/destination/current stages, semantic change class, affected artifact scope, widened reach, owner, next gate, evidence freshness, linked evidence, reversible window, break-glass state | Shows release movement and audience widening without relying on CI-only history. |
| Artifact provenance link | `artifact_ref` plus `exact_build_identity_ref` | artifact family, build id, digest/checksum bundle, signature state, attestation state, SBOM state, origin path class, validation time, verifier class, limitation note | Renders artifact-provenance cards beside status surfaces without overstating proof. |
| Rollback/revocation panel | `panel_id` | exact action class, affected state surfaces, affected artifact scope, consumer scope, historical visibility, recovery path, evidence freshness, provenance links, support/audit refs | Lets review and support reconstruct whether the action changes installed clients, remotes, registries, marketplace listings, mirrors, or channel pointers. |

## Release-Candidate Card

A release-candidate card is the compact status object rendered in release
center, update review, release notes preparation, compatibility review,
migration guidance, and support exports.

Required field groups:

| Field group | Purpose |
|---|---|
| `source_refs` | Links the card to release-center candidates, publication actions, publish targets, update manifests, and support bundles. |
| `version` | Shows product/release label, version label, semantic change class, build id, and exact-build identity refs. |
| `channel` | Shows channel class, support class, support window, compatibility posture, deprecation state, and end-of-support risk. |
| `target_scope` | Names the artifact families and target surfaces under review: local build output, update channel, managed ring, registry namespace, marketplace listing, mirror feed, docs claim, or support packet. |
| `current_stage` | Names stage class, stage ref, entered time, owner, and summary using release-center stage vocabulary. |
| `blockers` and `waivers` | Keeps stale evidence, missing signatures, missing attestations, compatibility blockers, policy locks, owner gaps, active waivers, and rollback gaps visible beside the promote action. |
| `evidence_freshness` | Uses the shared freshness classes: current, current with waiver, stale blocking, stale non-blocking, missing blocking, or not applicable. |
| `rollback_path` | Names the recovery verb, affected artifact families, last-known-good or repin target, update manifest refs, continuity notes, and support/export refs. |
| `artifact_provenance_links` | Provides the card-level artifact provenance rows that can be expanded into provenance cards. |
| `version_bump_rows` | Provides scoped version changes that feed release notes, compatibility reports, update review, and migration guidance. |
| `status_feeds` | Declares which downstream surfaces can consume this card without re-keying the object. |

The card MAY represent a local build, staged candidate, promoted channel,
published artifact, paused release, or revoked/yanked state. Those are
surface states only. Stage, channel, action, and recovery still come from
the release-center and update-manifest vocabularies.

## Version-Bump Rows

Every version-bump row must name:

- the changed scope, such as desktop shell, CLI, remote agent, SDK
  library, docs pack, schema export, extension package, marketplace
  metadata, policy bundle, support runbook, or release-evidence packet;
- previous and next versions;
- semantic change class: patch, minor, major, pre-release, hotfix,
  security-only, backport, schema-epoch, no public change, or manual
  review required;
- affected artifact refs and target scope;
- widened reach, if any, including audience, visibility, channel, or
  deployment-profile movement;
- owner and next gate;
- linked evidence refs and migration flags.

Version-bump rows are embedded on release-candidate cards so update
review, release notes, compatibility reports, and migration guidance can
reuse the same row family. A release note may summarize a version-bump
row, but it must not replace the row as evidence.

## Promotion Timeline Entries

A promotion-timeline entry records one movement or state change. It can
represent local build recording, target staging, candidate promotion,
stable publication, rollback application, yanking, revocation, repinning,
or deprecation announcement.

Every timeline entry must disclose:

- source stage, destination stage, and resulting current stage;
- semantic change class and affected artifact scope;
- widened reach, including audience, visibility, channel, rollout ring,
  deployment profile, docs claim, or marketplace reach;
- owner, actor class, approval refs, and next gate;
- evidence freshness and linked evidence refs;
- reversible window and rollback/repin refs when available;
- break-glass state when emergency publication or containment was used.

Promotion timelines are exportable. Release-center UI, headless dry-run
output, audit exports, support packets, release notes, and incident
postmortems should render the same timeline ids.

## Rollback And Revocation Panels

Rollback/revocation panels model decisions that alter recommendation,
installability, trust, visibility, or support posture after publication.
The action class is closed:

| Action | Meaning | Historical visibility |
|---|---|---|
| `rollback` | Move an installed, channel, remote, or managed-fleet state to a last-known-good exact-build set. | Prior publication remains in audit history. |
| `yank` | Block future consumption of a package, extension, or version while retaining historical publication truth. | Listing or installability may change; audit history remains. |
| `revoke` | Remove trust, signing validity, or admissibility for affected artifacts or metadata. | Revocation is a durable trust event linked to advisories/support refs. |
| `repin` | Move a mutable channel, ring, remote helper, or metadata pointer to an already-known exact-build target. | Previous and new pointers remain inspectable. |
| `deprecate` | Announce a support-window or compatibility transition with replacement and removal timing. | Surface remains visible with migration guidance until removal rules apply. |

Every panel must name affected state surfaces: installed client, remote
agent, registry version, marketplace listing, update channel, mirror
feed, docs/schema pack, or support packet. It must also name affected
artifact families and consumer scope. A panel that says only "undo
update" or "disable version" is non-conforming.

## Artifact Provenance Linkage

Artifact provenance links are intentionally narrower than proof claims.
They answer what was recorded and what was checked here:

- `digest` names algorithm, digest value or digest ref, and content
  address ref;
- `checksum_bundle` names bundle ref, generated time, and export scope;
- `signature` names signature state, signature ref, signer identity ref,
  trust root ref, revocation ref, and transparency log ref;
- `attestation` names attestation state, attestation refs, issuer/source
  class, and pipeline/build ref;
- `sbom` names SBOM state, formats, and inventory refs;
- `origin` names official origin, local build output, customer mirror,
  air-gap bundle, registry namespace, marketplace listing, managed feed,
  or support export path;
- `validation` names validation time, verifier class, validation scope,
  and limitation note.

Allowed verification language is exact: `verified`, `present
unverified`, `not checked here`, `missing`, `revoked`, `superseded`, or
`blocked by policy`. A mirror can preserve origin identity while
narrowing freshness or availability; it cannot convert a mirror receipt
into an official origin claim.

## Cross-Surface Consumption

A conforming release-candidate card can feed:

- update review: channel, stage, evidence freshness, rollback path,
  exact-build identity, and provenance links;
- release notes: version-bump rows, widened reach, compatibility notes,
  evidence refs, and deprecation timing;
- compatibility reports: target scope, support window, compatibility
  posture, affected artifact families, and evidence refs;
- migration guidance: semantic change class, migration flags,
  deprecation state, replacement refs, backup or rollback notes, and
  end-of-support risk;
- support exports: candidate refs, publication action refs, update
  manifest refs, exact-build refs, provenance links, rollback/revocation
  panel refs, and audit refs.

Consumers may hide fields for density, but they must not re-key,
rename, or reinterpret the underlying release objects.

## Fixture Matrix

The worked fixture set covers:

| Fixture class | Required status truth |
|---|---|
| Local build | Local-build output, no public support claim, incomplete proof layers, discard-local-build recovery, and no channel widening. |
| Staged candidate | Candidate stage, target scope, blockers or waivers, current/stale evidence, promotion next gate, and repin or rollback path. |
| Promoted stable | Stable channel identity, support window, current evidence, exact-build refs, promoted timeline entry, and rollback path. |
| Revoked/yanked artifact | Revocation and yank verbs, affected artifact families, affected state surfaces, consumer scope, historical visibility, advisory/support refs, and recovery/repin/deprecation options. |

## Conformance Checklist

- A status card must include `candidate_ref`, `publish_target_refs`, or
  `update_manifest_refs` before it is consumed by release-center,
  update-review, support, or release-note tooling.
- Any stale or missing evidence that blocks promotion must appear on the
  card and timeline entry that exposes the promote action.
- A version-bump row must include semantic change class and affected
  artifact scope.
- A timeline entry must name what widened.
- A rollback/revocation panel must include action class, affected state
  surfaces, affected artifact families, consumer scope, historical
  visibility, and recovery path.
- Artifact provenance links must separate signature, attestation,
  checksum bundle, SBOM, origin/mirror class, and validation time.
- Channel identity, support window, compatibility posture, deprecation
  timing, and end-of-support risk must remain attached to the status
  object, not copied into one-off release prose.
