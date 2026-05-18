# Release-center object model, publish targets, and break-glass publication contract

This document freezes the release-center object model Aureline uses for
candidate review, publication, rollback, revocation, yanking, repinning,
mirror publication, registry publication, and emergency publication. It is
a contract layer, not a release automation implementation.

The goal is straightforward: release center, headless automation,
support exports, advisory surfaces, About/provenance, service health,
rollback tools, and mirror operators must all reconstruct the same
publication action from stable objects and refs. Screenshots, chat logs,
and free-text operator notes are not publication evidence.

Companion artifacts:

- [`/schemas/release/release_center_object.schema.json`](../../schemas/release/release_center_object.schema.json)
  - boundary schema for release candidates, artifact-family rows,
  stages, audiences, publication actions, and worked publication cases.
- [`/schemas/release/release_candidate.schema.json`](../../schemas/release/release_candidate.schema.json)
  - boundary schema for release-candidate objects and version-bump
  proposal objects.
- [`/schemas/release/publish_target.schema.json`](../../schemas/release/publish_target.schema.json)
  - boundary schema for publish-target descriptors.
- [`/schemas/release/promotion_timeline.schema.json`](../../schemas/release/promotion_timeline.schema.json)
  - boundary schema for promotion timeline steps, artifact bundle cards,
  and rollback or revocation records.
- [`/crates/aureline-release/src/release_center_model/`](../../crates/aureline-release/src/release_center_model/)
  - Rust object model consumed by UI, headless, support, and audit
  projections.
- [`/fixtures/release/publish_target_cases/`](../../fixtures/release/publish_target_cases/)
  - structural fixtures for staged preview, stable publication,
  mirror-only emergency push, support-bundle backreference, and
  break-glass reconciliation.
- [`/docs/release/update_and_rollback_contract.md`](./update_and_rollback_contract.md)
  and
  [`/schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json)
  - update-manifest, rollback, revoke, yank, mirror import, helper
  negotiation, and existing release-action fields this contract extends.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  - exact-build identity model every release-bearing row resolves
  through.
- [`/docs/release/release_center_provenance_linkage.md`](./release_center_provenance_linkage.md),
  [`/schemas/release/release_provenance_crosswalk.schema.json`](../../schemas/release/release_provenance_crosswalk.schema.json),
  and
  [`/artifacts/release/release_support_crosswalk.yaml`](../../artifacts/release/release_support_crosswalk.yaml)
  - cross-surface linkage rules that keep release-center rows,
  About/provenance, service health, update/rollback, advisories, and
  support bundles aligned on one exact-build identity.
- [`/docs/security/high_risk_control_quorum.md`](../security/high_risk_control_quorum.md),
  [`/schemas/security/break_glass_event.schema.json`](../../schemas/security/break_glass_event.schema.json),
  [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md),
  and
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  - break-glass quorum, emergency-action, revocation, and
  reconciliation contracts.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  - support/export packet surfaces that preserve release-center refs
  rather than converting them to prose.

Normative sources this contract projects from:

- `.t2/docs/Aureline_Technical_Design_Document.md` sections on
  release-center, publish-target, promotion, rollback, and
  operational publication semantics.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections and appendices
  covering release candidate cards, publish-target review sheets,
  provenance cards, rollback/yank/revoke sheets, and release evidence
  rows.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections on
  release publication, emergency action, revocation, mirror/offline
  distribution, and supportability.

If this document disagrees with those sources, the normative sources
win and this document plus its companion schemas update in the same
change.

## Scope

Frozen here:

- release-center object ids for candidate, artifact family, stage,
  audience, and publication action rows;
- publish-target classes for local preview, internal ring, public
  preview, stable, LTS, mirror feed, registry/marketplace, and
  emergency channel;
- the fields every publish target must disclose before mutation:
  audience, visibility, mutability, rollback path, support class,
  auth-source class, dry-run availability, evidence freshness, and
  exact-build backreferences;
- break-glass publication rules, mandatory evidence or waiver packet
  refs, and post-action reconciliation;
- cross-surface parity rules for UI and headless flows.

Out of scope:

- production release automation, channel services, package-manager
  backends, signing infrastructure, registry credentials, or operator
  credential custody;
- final product UI implementation;
- raw release artifacts, raw signatures, raw logs, raw support bundle
  bodies, or raw credentials.

## Invariants

1. **Release center is the publication object family.** UI and
   headless publication flows use the same candidate ids, publish-target
   ids, stage ids, publication-action ids, evidence refs, support refs,
   and exact-build refs.
2. **Targets disclose side effects before action.** A conforming
   publish target names its class, destination, audience, visibility,
   mutability, rollback path, support class, auth-source class,
   dry-run state, evidence freshness, and exact-build linkage before a
   publish, rollback, revoke, yank, repin, or mirror push executes.
3. **Publication acts on release graph nodes.** A release-center action
   names artifact-family refs and exact-build backreferences. It does
   not publish an unconstrained folder, tag, or operator note.
4. **Rollback and revocation preserve history.** A rollback, revoke,
   yank, or repin is a new publication-action row linked to prior
   actions. It never deletes the original publication row.
5. **Break-glass is structured exception metadata.** Emergency
   containment still produces ordinary release-center action rows plus
   a `break_glass_event_record`, emergency/revocation refs, and
   reconciliation refs. It is not a side channel.
6. **Mirrors preserve origin identity.** A mirror feed or air-gap
   bundle can narrow freshness, availability, or support posture. It
   cannot re-anchor artifacts to new exact-build identities.
7. **Support can reconstruct from refs.** Given the release-center
   action, publish target, update manifest, exact-build identities,
   evidence refs, and support-bundle linkage, support can reconstruct
   what happened without screenshots.

## Object vocabulary

| Object | Required identity | Required content | Reconstruction rule |
|---|---|---|---|
| Release candidate | `candidate_id` | version label, channel family, current stage, scoped artifact-family refs, publish-target refs, audience refs, evidence freshness, blockers, waiver refs, known-issue refs, rollback target, exact-build backreferences | Candidate rows are the entry point for release review and support reconstruction. |
| Artifact-family row | `artifact_family_ref` | artifact family class, exact-build backreference, digest refs, signature refs, release-graph node refs, rollback-atom ref, publication state | Artifact rows quote exact-build identity and release graph nodes; they do not restate build truth. |
| Publish target | `publish_target_id` | target class, destination, visibility, audience semantics, mutability, rollback path, support class, auth source, dry-run status, evidence freshness, exact-build backreferences | Publication sheets and headless dry-run output render directly from this record. |
| Stage | `stage_id` | stage class, candidate ref, source/target stage refs, entered/exited times, target refs, action refs, evidence freshness, reversible window, break-glass state | Promotion timeline steps use stage ids, not CI-only labels. |
| Audience | `audience_id` | audience class, visibility class, deployment profile scope, support class, service-health refs, advisory refs | Audience rows decide who sees the target and which support promises apply. |
| Publication action | `action_id` plus `release_center_event_ref` | requested action, candidate ref, target ref, source/destination stages, actor class, auth source, dry-run, evidence freshness, exact-build refs, update manifest, support linkage, rollback/revoke fields, break-glass block, parity links | This is the durable row for publish, promote, rollback, revoke, yank, repin, mirror-only emergency push, and reconstruction. |

## Shared field meanings

- `auth_source_class` names the authority source used for publication.
  Examples include local-only, CI release identity, release vault token,
  emergency quorum, mirror receipt, registry publisher identity, and
  support export ref. Raw credentials are never stored.
- `dry_run.availability_class` states whether the target supports a
  current dry run, a stale dry run, no dry run by design, no dry run
  because the target lacks the capability, or a failed dry run.
- `evidence_freshness.freshness_class` states whether required evidence
  is current, current with waiver, stale but non-blocking, stale and
  blocking, missing and blocking, or not applicable.
- `exact_build_backreferences[]` bind every publishable artifact family
  to an exact-build identity ref, build id, digest refs, provenance row
  refs, About/provenance row refs, and support-bundle refs.
- `surface_parity_links` preserve the refs rendered by release center,
  headless automation, About/provenance, service health, advisories,
  rollback panels, and support bundles.

## Publish-target classes

| Target class | Audience semantics | Mutability | Rollback / recovery path | Support class | Auth and dry-run expectation |
|---|---|---|---|---|---|
| `local_preview` | Developer-local or release-team inspection; no public visibility | Mutable local pointer, disposable preview state | Discard local preview or regenerate from candidate refs | `unsupported_local` or `internal_only` | `none_local` or local keychain; dry run may be not applicable but scope preview must still list exact-build refs. |
| `internal_ring` | Release team, internal dogfood, or managed pilot ring | Mutable ring pointer over immutable artifacts | Repin previous ring pointer or staged rollback manifest | `internal_only` or `preview_supported` | CI release identity or release vault token; current dry run required before widening. |
| `public_preview` | Design partners, beta users, or public preview users | Immutable artifact version with mutable channel pointer | Repin previous pointer or staged rollback manifest | `preview_supported` | Release authority disclosed; dry run current or promotion blocks. |
| `stable` | Public stable users and default update consumers | Immutable version with mutable channel pointer only through governed repin | Staged rollback manifest or repin to last known good | `public_supported` | Release authority, current release evidence, exact-build rows, and rollback preview required. |
| `lts` | LTS admins and pinned enterprise deployments | Immutable published version; backport and repin rules are stricter than stable | LTS backport, repin, or coordinated rollback manifest | `lts_supported` | Release authority plus LTS evidence current; no break-glass promotion. |
| `mirror_feed` | Mirror operators, air-gapped admins, or self-hosted admins | Mirror snapshot can be superseded but cannot rewrite origin identity | Mirror snapshot supersedence or manual-import receipt repair | `mirror_operator_supported` | Mirror receipt or release authority; dry run may be mirror import preflight rather than live publish dry run. |
| `registry_marketplace` | Registry consumers, marketplace consumers, or extension/package admins | Version immutable; metadata or listing pointer may be mutable by target policy | Yank new installs, republish replacement, or revoke if trust-affecting | `registry_supported` | Registry publisher identity disclosed; manifest/permission diff and package dry run required where supported. |
| `emergency_channel` | Security responders and affected admins; visibility is bounded to containment | Emergency time-boxed; ordinary scope cannot be widened | Revoke/disable, emergency reconciliation, or superseding signed action | `emergency_support_only` | Emergency quorum or admitted break-glass event; dry run can be unavailable only when the emergency record explains why. |

## Action semantics

### Publish and promote

Publish and promote actions move a candidate or target pointer forward.
They require a candidate ref, target ref, source and destination stage
refs, exact-build backreferences, current evidence or scoped waiver
refs, approval refs, auth-source disclosure, and dry-run/scope preview
state. Stable and LTS promotion never use break-glass.

### Rollback

Rollback is a publication action whose recovery target is a
last-known-good exact-build set. It must name affected artifact refs,
rollback manifest refs, continuity notes, and support-bundle linkage.
Rollback can leave unaffected artifact sets installable only when the
release graph remains consistent.

### Revoke

Revoke records durable trust or installability removal. It must link
revocation records, affected subjects, advisory or emergency refs when
applicable, and support/export refs. Revoke does not delete prior
publication history.

### Yank

Yank blocks future consumption of a package, extension, or version while
preserving historical visibility. Yank is not a rollback unless a
replacement pointer or rollback manifest is also linked.

### Repin

Repin changes a mutable channel, ring, or metadata pointer to an
already-known exact-build target. It must preserve the previous pointer,
the new pointer, the reason, and the support/reconstruction refs.

### Mirror-only emergency push

Mirror-only emergency push updates a customer mirror, air-gap bundle, or
manual-import path without claiming the authoritative public channel has
advanced. It must preserve origin digest, signer continuity, revocation
or emergency-action refs, mirror freshness, manual-import receipt refs,
and exact-build backreferences.

## Break-glass publication rules

Break-glass publication is allowed only for containment actions admitted
by the high-risk control quorum contract. It is never a replacement for
normal release approval.

Who may use it:

- `security_operator` or `release_operator` may invoke admitted
  emergency containment actions when the signed quorum policy allows
  the `audited_single_responder_containment` profile.
- `org_admin` may preserve a self-hosted or air-gapped emergency import
  receipt when the official signed emergency action already exists.
  That does not become an official channel publish.
- `mirror_operator` may perform mirror-only emergency push or manual
  import receipt preservation inside the owned mirror scope. The origin
  release identity and emergency-action refs remain authoritative.
- `support_operator` may assemble or link support/export packets. A
  support operator alone does not publish, widen, revoke, or repin an
  official target.

Mandatory packet refs:

- one `break_glass_event_record` ref for every admitted invocation;
- one emergency action, revocation, advisory, or incident-workspace ref
  explaining the containment reason;
- exact-build identity refs or a written scope reason when the action
  affects a channel pointer or trust root rather than specific bytes;
- release-evidence packet refs, or waiver packet refs that name the
  missing or stale evidence and the narrowed scope;
- support/export refs and admin/mirror receipt refs when the action is
  visible to support, self-hosted, mirrored, or air-gapped consumers.

Post-action reconciliation:

- the release-center action carries
  `break_glass_publication.state_class` equal to
  `active_pending_reconciliation` until closed;
- reconciliation must name a target row, follow-up refs, and a
  `reconcile_by` timestamp no later than the admitted break-glass
  window unless the emergency action contract requires a shorter window;
- close-out is one of retrospective co-sign, superseding signed action,
  explicit invalid-invocation withdrawal, or expired-without-
  reconciliation;
- About/provenance, service health, advisory, rollback, mirror import,
  admin export, and support surfaces must quote the same break-glass
  event id and release-center action id;
- scope resume requires the retrospective quorum or superseding signed
  action required by the high-risk control quorum contract.

How break-glass differs from other actions:

- Normal publish/promote widens a candidate under the ordinary quorum
  and evidence gates. Break-glass does not widen ordinary release scope.
- Revoke removes or disables a subject durably. Break-glass may invoke
  an emergency revoke path, but the revocation record remains the
  durable state.
- Yank blocks new consumption while preserving history. Break-glass is
  not needed for routine yanks and cannot hide prior publication.
- Repin moves a pointer to a known target. Break-glass may annotate an
  emergency repin only when it is containment and has the required audit
  row.
- Mirror-only emergency push updates a mirror/import path. It is not a
  public channel publish and does not change origin identity.

## Cross-surface parity

Every surface must project from the same refs:

| Surface | Required refs | Forbidden shortcut |
|---|---|---|
| Release center | candidate id, target id, stage id, action id, evidence refs, exact-build refs, support refs | screenshots of CI or free-text operator summary |
| Headless automation | same release-center action id, target id, auth-source class, dry-run ref, evidence refs | separate automation-only action vocabulary |
| About/provenance | exact-build identity refs, provenance row refs, release-center action refs when a build is published or rolled back | version string without exact-build ref |
| Service health | release-center action ref, emergency/revocation refs, mirror/offline freshness refs when user-visible health changes | banner-only incident text |
| Advisory and emergency action | affected exact-build refs, release-channel refs, publish-target refs, break-glass event refs | separate affected-version table |
| Rollback/update center | update manifest ref, rollback manifest ref, action id, last-known-good exact-build refs | rollback button without artifact set and continuity note |
| Supportability | support bundle refs, support packet index refs, redaction profile refs, action id, target id, exact-build refs | support note that cannot join back to release center |

## Schema rules

- `publish_target_record` is the pre-action target disclosure shape.
- `publication_action_record` is the durable mutation/reconstruction
  row.
- `release_publication_case_record` exists for fixtures and contract
  tests. It bundles a candidate summary, publish target, action row,
  expected reconstruction inputs, and parity assertions.
- Adding a new target class, stage class, action class, auth-source
  class, mutability class, rollback path class, support class, evidence
  freshness class, or break-glass state is additive only when the new
  value carries a matching narrative contract update and fixture.
- Repurposing an existing class is breaking and requires a new governed
  decision row.
