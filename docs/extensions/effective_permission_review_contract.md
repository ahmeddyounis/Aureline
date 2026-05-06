# Extension effective-permission review sheet, manifest-scope diff, and publisher-continuity packet contract

This document is the narrative companion to the boundary schema at
[`/schemas/extensions/effective_permission_review.schema.json`](../../schemas/extensions/effective_permission_review.schema.json),
the machine-readable continuity packet seed at
[`/artifacts/extensions/publisher_continuity_packet.yaml`](../../artifacts/extensions/publisher_continuity_packet.yaml),
and the worked install-review fixtures under
[`/fixtures/extensions/install_review_cases/`](../../fixtures/extensions/install_review_cases/).

It freezes one reviewer-facing field set that install/update/restore
surfaces can project **before** the extension host and installer ship:

- **Declared vs effective permission disclosure** (including transitive
  widening through dependency closure and narrowing through policy or
  registry ceilings),
- a **manifest-scope diff** that separates package-level changes from
  workspace/profile impact, and
- a **publisher-continuity packet** that keeps signer lineage and
  transfer/revocation/mirror/parity truth visible even when the backing
  registry or mirror is offline.

The schema is authoritative when this document and the schema disagree.
This contract MUST be updated in the same change that bumps the schema
or materially changes the frozen vocabularies it re-exports.

This contract is deliberately narrow. It does **not** implement an
extension host, a package installer, a marketplace backend, a mirror
service, or a policy engine. Its job is to keep trust review inspectable
and machine-readable early enough that later runtime work cannot ship
with registry-specific guesswork or prose-only permission explanations.

## Normative sources projected here

- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md)
  — manifest row, effective-permission summary, publisher continuity row,
  and policy-pack constraint row vocabulary.
- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md)
  — install-review summary slots and disclosure rules.
- [`/docs/verification/install_review_packet.md`](../verification/install_review_packet.md)
  — requested vs effective permission diff discipline and transitive
  visibility rules.
- [`/docs/extensions/publisher_lifecycle_and_registry_parity_contract.md`](./publisher_lifecycle_and_registry_parity_contract.md)
  — revocation/quarantine/mirror-promotion event vocabulary and
  private-registry parity model.
- [`/docs/extensions/registry_and_offline_bundle_seed.md`](./registry_and_offline_bundle_seed.md)
  — mirror continuity and offline restore truth.
- [`/docs/ecosystem/extension_lockfile_and_recommendation_contract.md`](../ecosystem/extension_lockfile_and_recommendation_contract.md)
  — workspace/profile lockfile and recommendation-set roles and stable
  path conventions.
- [`/docs/extensions/runtime_budget_packet.md`](./runtime_budget_packet.md)
  — runtime-budget class register referenced by install-review fact grids.

## What this contract freezes

1. One `effective_permission_review_sheet_record` that binds:
   declared/requested/inherited/effective permission sets, transitive
   closure contributions, host and budget class, lifecycle/support
   summary, target scope, affected artifacts, rollback checkpoint, mirror
   posture, and publisher continuity.
2. One `manifest_scope_diff_record` composed of row kinds that always
   separate:
   package-target changes, workspace impact, profile impact, and
   policy/private-registry narrowing.
3. One `publisher_continuity_packet_record` that keeps signer lineage,
   transfer/successor/orphan state, revocation/quarantine posture, mirror
   promotion, and private-registry parity assertions visible on install,
   update, restore, and export paths.
4. A seed corpus of worked fixtures for the cases required below so
   later UI and CLI/headless surfaces can render the same explanation
   without inventing per-surface prose.

## Record kinds

| Record kind | Purpose | Home |
|---|---|---|
| `effective_permission_review_sheet_record` | Reviewer-facing permission review sheet: what was declared, what becomes effective, why, and what else changes alongside permission truth. | `/schemas/extensions/effective_permission_review.schema.json` |
| `manifest_scope_diff_record` | Structured diff rows grouped by package target vs workspace/profile impact vs policy/private-registry narrowing. | `/schemas/extensions/effective_permission_review.schema.json` |
| `publisher_continuity_packet_record` | Publisher continuity packet: signer lineage, transfer/successor, revocation/quarantine, mirror promotion, and parity assertions. | `/schemas/extensions/effective_permission_review.schema.json` + `/artifacts/extensions/publisher_continuity_packet.yaml` |
| `extension_install_review_case_record` | Fixture wrapper that ties a case id to the three records above for install/update/restore review drills. | `/schemas/extensions/effective_permission_review.schema.json` + `/fixtures/extensions/install_review_cases/*.yaml` |

## Effective-permission review sheet

The effective-permission review sheet is the object the user/admin opens
from install/update/restore flows (and from support export) to answer one
question without prose: **what can this extension do after resolution,
and why does that differ from what the manifest says?**

The sheet MUST remain renderable when:

- the registry is unreachable (offline review),
- the artifact was restored from an offline bundle or local archive, or
- the install state was restored from a workspace/profile export.

### Required fields (summary)

At minimum, every `effective_permission_review_sheet_record` carries:

- identity: `sheet_id`, `created_at`, `subject_ref`, `subject_version_ref`;
- declared/effective permission truth:
  `declared_permissions_digest`, `requested_vs_effective_permission_diff`,
  and `transitive_permission_visibility`;
- runtime envelope: `host_contract_family`, `artifact_transport_family`,
  and `runtime_budget_class_ref`;
- lifecycle/support: `capability_lifecycle_row_refs` and
  `support_window_class`;
- target and impact: `target_scope`, `affected_artifacts`, and
  `rollback_checkpoint_ref`;
- continuity posture: `mirror_posture` and `publisher_continuity_packet_ref`;
- disclosure: `disclosure_flag_set`, `irreversibility_flag_set`,
  `review_event_refs`, and `redaction_class`.

**Rule:** a surface that hides the permission-diff block, the transitive
closure block when widening exists, or the continuity posture rows denies
commit with `review_disclosure_incomplete`.

## Manifest-scope diff

The manifest-scope diff is the structured summary row set shown inline
on the install-review sheet (and exported in support artifacts). It is
distinct from the permission diff: it explains **what changes**, and
where those changes land (package target vs workspace vs profile) without
smuggling scope changes into a single “update” chip.

### Row grouping (frozen)

Every `manifest_scope_diff_record` carries an ordered list of diff rows.
Each row MUST set `scope_bucket` to one of:

- `package_target` — changes to the extension artifact identity or its
  declared manifest claims (host binding, declared permissions, declared
  capability inheritance, signature class).
- `workspace_impact` — changes to workspace artifacts (lockfile,
  recommendation set, workspace-bound policy projections).
- `profile_impact` — changes to profile artifacts (profile lockfile,
  profile recommendation set, profile export payload).
- `policy_or_registry_narrowing` — changes that occur only because an
  admin policy pack, mirror rule, or private-registry ceiling narrows
  what would otherwise install (for example, policy forcing step-up,
  registry denying helper binaries, mirror capping trust inheritance).

**Rule:** policy/private-registry narrowing MUST appear as its own
diff-row bucket even when the outcome is “install denied”; hiding the
narrowing behind a generic “blocked” chip is non-conforming.

## Publisher continuity packet

Publisher continuity is not “registry metadata”. The continuity packet
is a machine-readable object that follows the extension through:

- install and update review,
- restore from offline bundles or profile/workspace exports, and
- support/offboarding exports.

The continuity packet composes:

- the ADR-0012 `publisher_continuity_row` (signer lineage and transfer),
- publisher lifecycle events (quarantine/revocation/mirror promotion),
- mirror continuity state (offline/mirror posture), and
- private-registry parity assertions.

**Rule:** continuity truth MUST remain visible even when the source is a
private registry or a mirror; registry-specific “trust tier” labels are
forbidden.

## Required seed cases

The fixture corpus MUST cover, at minimum:

- policy-narrowed permissions (step-up required and denied),
- dependency-supplied capability that widens effective permission beyond
  the top-level manifest declaration,
- publisher transfer / successor continuity,
- revoked mirror metadata / broken continuity posture,
- offline review (no authoritative reverify),
- restored install state (workspace/profile restore preserving continuity
  refs).

The worked fixtures live under
[`/fixtures/extensions/install_review_cases/`](../../fixtures/extensions/install_review_cases/)
and MUST remain stable enough that later UI and CLI/headless surfaces can
use the same case ids for parity tests.

