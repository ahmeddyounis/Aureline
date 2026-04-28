# Advisory surface contract

This document freezes the pre-implementation contract for how Aureline
renders security advisories, emergency banners, revocation notices, and
disclosure links across product UI, docs/help, release packets, support
exports, and mirror-safe distribution notes.

The contract is intentionally a surface projection, not a replacement for
the underlying security records. It reads from the advisory, emergency
action, revocation, manual-import, exact-build, install-profile, and
support/export records that already exist, then publishes the minimum
fields every user-facing or operator-facing surface must show before a
person can decide whether a notice is informational, blocking, or requires
immediate remediation.

Companion artifacts:

- [`/schemas/security/advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json)
  - machine-readable boundary for the `advisory_surface_record`.
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  - canonical advisory identity, severity, disclosure, evidence, and
  affected-install linkage.
- [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  - canonical emergency-action and revocation records that banners and
  revocation notices project from.
- [`/schemas/security/manual_import_receipt.schema.json`](../../schemas/security/manual_import_receipt.schema.json)
  - receipt shape for manual, mirrored, and air-gapped emergency imports.
- [`/docs/security/severity_matrix.md`](./severity_matrix.md)
  - severity vocabulary, advisory identity model, monitored-contact path,
  and affected-install linkage rules.
- [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  and [`/docs/security/emergency_distribution_policy.md`](./emergency_distribution_policy.md)
  - emergency object model and mirror/manual-import distribution policy.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and [`/docs/release/install_topology_plan.md`](../release/install_topology_plan.md)
  - exact build and install-profile identities named by every applies-to
  row.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  - support/export packet linkage that advisory surfaces preserve.
- [`/fixtures/security/advisory_cases/`](../../fixtures/security/advisory_cases/)
  - worked advisory-surface fixtures covering staged disclosure, active
  emergency disable, mirror-only advisory, and superseded advisory chains.

Normative source alignment:

- `.t2/docs/Aureline_PRD.md` sections covering vulnerability disclosure,
  incident severity, emergency response, offline entitlement, mirror
  support, and release/support evidence.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` appendix rows for
  security response, advisory publication, revocation, mirror/manual import,
  and support/export evidence.
- `.t2/docs/Aureline_Technical_Design_Document.md` appendix rows for
  advisory and emergency-action states, affected-install assessments,
  revocation continuity, and mirror/offline distribution.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections for advisory cards,
  emergency notices, disclosure links, notification durability, and history
  rows.

If this contract disagrees with those source documents, the source
documents win and this file, schema, and fixtures update together.

## Why this exists

The repository already has records for security advisories, emergency
actions, revocations, and manual-import receipts. Those records are the
source of truth. What was still missing was the shared surface contract
that tells every consumer how to render those records without inventing a
local dialect.

Without one surface model:

- an in-product card could show an advisory as resolved while a mirror note
  still treats it as active;
- a revocation could disappear from history after the emergency banner
  clears;
- a manual import could claim live freshness even though it only has a
  signed snapshot;
- a user could copy a CVE or GHSA id from one surface but not from release
  notes or support export;
- a docs/help page could omit the exact build or install profile affected;
  and
- a support packet could flatten rollback, revocation, and disclosure
  links into prose that cannot be verified.

This contract closes those gaps by forcing every advisory-related surface
to carry one identity envelope, one severity and action-state vocabulary,
one applies-to row set, one disclosure posture, one acknowledgement/review
state, one supersedence link, and one history state.

## Scope

Frozen at this revision:

- advisory card, emergency banner, revocation notice, and disclosure link
  surface kinds;
- severity, action-required, blocking, immediate-remediation, and
  mitigation-complete states;
- affected-surface and applies-to row fields;
- local-only, managed, offline-mirror, and manual-import notice behavior;
- exact-build, package/install-profile, rollback/revocation, support/export,
  release-evidence, and disclosure-link refs;
- disclosure time, public/private posture, acknowledgement/review state, and
  superseded-by fields;
- advisory identity and copy-safe id fields for CVE, GHSA, and Aureline
  advisory ids; and
- resolved-versus-active history behavior so a mitigated advisory remains
  inspectable.

Out of scope:

- live incident-response tooling;
- final UI layout, typography, icons, animations, and notification routing;
- raw advisory text publication workflow;
- CVE CNA or GHSA submission automation;
- emergency bundle bytes or signing implementation; and
- network transport implementation for mirrors, update feeds, or manual
  import.

## Surface record

Every rendered advisory-related surface is represented by one
`advisory_surface_record` validated by
[`advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json).
The same record shape supports four `surface_kind` values:

| Surface kind | Purpose | Required source truth |
|---|---|---|
| `advisory_card` | Standard advisory card or row in product, docs/help, release packet, or support export | `advisory_record_ref`, advisory identity, severity, disclosure, applies-to rows |
| `emergency_banner` | Durable banner or activity row for an emergency action that changes safe behavior | `emergency_action_refs`, action state, notice-behavior rows, required actions |
| `revocation_notice` | Durable notice for a revoked artifact, trust root, channel, package, or capability | `revocation_refs`, revoked subject rows, rollback or recovery links |
| `disclosure_link` | Copy/open/export link row attached to advisory, release, docs/help, mirror note, or support packet | disclosure posture, destination refs, offline availability, visibility boundary |

The record is strict. Unknown fields are not admitted because these surfaces
are release/security boundaries. Optional information can be added only as a
new schema field with a version bump.

## Required field groups

Every `advisory_surface_record` carries these groups:

| Group | Required fields | Why it matters |
|---|---|---|
| Identity | `surface_id`, `advisory_identity`, `copy_safe_ids` | Lets reviewers copy CVE, GHSA, and Aureline advisory ids without opening another surface |
| Severity and action | `severity_class`, `surface_severity_class`, `action_state` | Distinguishes informational, blocking, and immediate-remediation notices |
| Affected scope | `affected_surface_class`, `affected_install_linkage`, `applies_to_rows` | Shows exactly which builds, profiles, packages, or subjects are affected |
| Disclosure | `disclosure` | Records disclosure time and public/private posture |
| Acknowledgement | `acknowledgement` | Records whether review, acknowledgement, or remediation is still owed |
| Links | `linked_records`, `disclosure_links` | Preserves advisory, emergency, revocation, rollback, support, export, release, and docs refs |
| Distribution | `notice_delivery_rows` | Keeps local-only, managed, mirror, and manual-import behavior honest |
| History | `history` | Keeps resolved, superseded, and withdrawn notices reachable after mitigation |

## Advisory identity and copy-safe ids

The advisory identity envelope is isomorphic with
`advisory_record.advisory_identity`:

- `aureline_advisory_id` is always required.
- `cve_id` is nullable until assigned.
- `ghsa_id` is nullable until minted.
- `additional_alias_refs` remain opaque and do not create new top-level
  identity systems.

`copy_safe_ids[]` is the surface-specific copy contract. Each row says:

- which id type is being copied;
- the exact value copied;
- whether copying is available in the current surface;
- whether the id is public, private, or pending public disclosure; and
- a short label suitable for screen readers, release packets, and support
  exports.

Rules:

1. A visible CVE, GHSA, or Aureline advisory id MUST appear in
   `copy_safe_ids[]`.
2. Copying an id MUST NOT expose reporter identity, tenant identity, private
   registry names, raw hostnames, raw paths, or raw evidence.
3. A staged or private disclosure may expose the Aureline advisory id to
   authorized reviewers while keeping CVE or GHSA aliases unavailable until
   publication. The unavailable id still appears as a typed row with a
   pending state rather than disappearing.

## Severity and action states

`severity_class` reuses the security severity vocabulary from
[`advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json).

`surface_severity_class` is the compact rendering vocabulary:

- `informational`
- `low`
- `moderate`
- `high`
- `critical`
- `operational_emergency`

`action_state` tells the user or operator what kind of response is needed:

| Action state | Meaning | Surface obligation |
|---|---|---|
| `informational` | No immediate action is owed | show affected rows and history link without blocking work |
| `review_recommended` | Reviewer should inspect the notice | expose acknowledgement/review path |
| `action_required` | A user or admin action is required | show primary action and affected rows without opening a drawer |
| `blocking` | Safe continuation is blocked for the affected row | show blocked capability and recovery path |
| `immediate_remediation` | Emergency action is required now | banner/activity row required; durable history required |
| `mitigation_complete` | Current install is no longer affected | step down prominence but keep history and links reachable |

Rules:

1. `immediate_remediation` requires at least one linked emergency action or
   revocation ref.
2. `mitigation_complete` MUST NOT delete or hide the surface. It changes
   prominence and history state only.
3. A low-severity advisory cannot carry emergency-action or revocation refs.
   If emergency behavior is needed, severity is reclassified in the advisory
   record and the history remains visible.

## Affected surface and applies-to rows

`affected_surface_class` names the top-level domain of the surface:

- `build_binary`
- `docs_pack`
- `extension`
- `package_or_dependency`
- `install_profile`
- `release_channel`
- `trust_root`
- `capability_or_route`
- `managed_cloud_surface`
- `workspace_trust_policy`
- `ai_context_assembly`

The more precise truth lives in `applies_to_rows[]`. Each row carries:

- row id;
- affected surface class;
- subject refs;
- exact build identity refs;
- install-profile card refs;
- package or component refs;
- deployment-profile scope;
- current installed-match state;
- current mitigation state;
- required action; and
- a short continuity note.

Rules:

1. A surface with no applies-to rows is non-conforming.
2. Rows MUST bind to exact-build identity refs and install-profile card refs
   when the underlying subject is a build, package/install profile, update
   channel, mirror bundle, or release artifact.
3. A mirror-only or manual-import row MUST preserve both upstream origin and
   mirror/import freshness. It must not rewrite a snapshot into live truth.
4. A resolved row remains present with `mitigation_state =
   mitigation_complete`; it does not disappear from the applies-to set.

## Notice behavior by profile

`notice_delivery_rows[]` freezes how the same advisory projects into the
deployment profiles that can see it.

| Profile | Required behavior |
|---|---|
| `local_only` | Local work remains available unless the affected row itself is blocked. Notices can come from local cache or bundled metadata, but freshness is labeled. No vendor account or managed service may be implied. |
| `managed` | Managed push/pull may deliver the notice, but local edit/search/Git continuity remains separate from managed-control availability. The notice names the managed source and policy owner when relevant. |
| `offline_mirror` | Mirror or offline-bundle metadata must show mirror freshness, source class, signer continuity, and whether the mirror is within grace. No public-network fallback may be implied. |
| `manual_import` | Imported metadata must cite a manual-import receipt, detached-signature verification outcome, importer scope, and snapshot freshness. A manual import never claims `authoritative_live`. |

Rules:

1. Every surface MUST carry at least one delivery row. Surfaces used by
   release packets or mirror notes SHOULD carry all four rows when the
   advisory affects all distribution profiles.
2. Emergency banners and revocation notices that affect mirrored or
   air-gapped profiles MUST include `offline_mirror` or `manual_import`
   rows as applicable.
3. Manual-import and mirror-only notices MUST be renderable without network
   access from the fields in the record.

## Disclosure posture and timing

The `disclosure` group is required on every surface:

- `disclosure_class` reuses the advisory record vocabulary.
- `visibility_class` is one of `private`, `internal`, `staged_private`,
  `public`, or `mirror_only`.
- `current_disclosure_at` is the timestamp that governs the current visible
  posture.
- `private_disclosed_at` and `public_disclosed_at` are nullable but present.
- `embargo_until` is nullable but present.
- `disclosure_note` summarizes why the posture is current.

Rules:

1. A public surface must carry `public_disclosed_at`.
2. A staged private surface must carry `private_disclosed_at` and either a
   null `public_disclosed_at` or a future `embargo_until`.
3. Mirror-only disclosure is not the same as private disclosure. It means
   the notice is intentionally distributed through a mirror/offline packet
   and must name mirror freshness and origin.

## Acknowledgement and review state

The `acknowledgement` group is required and carries:

- `acknowledgement_state`;
- `review_state`;
- `acknowledged_by_ref`;
- `acknowledged_at`;
- `reviewed_by_ref`;
- `reviewed_at`; and
- `next_review_due_at`.

Rules:

1. A notice that requires action cannot be removed by acknowledgement alone.
2. Acknowledgement records that a reviewer saw the notice. Mitigation records
   that the affected row is no longer exposed.
3. A manual-import or mirror-only notice may be `acknowledged` while still
   `review_required` if freshness or signer continuity has not been reviewed.

## Linked records and disclosure links

`linked_records` preserves the exact record graph:

- advisory record;
- private triage workspace;
- incident workspace packets;
- emergency actions;
- revocations;
- disable bundles;
- rollback actions;
- support packets;
- export packets;
- release evidence packets; and
- docs/help refs.

`disclosure_links[]` is the open/copy/export contract. Each row names:

- link kind;
- destination ref;
- visibility boundary;
- offline availability;
- mirror safety;
- whether the link is a copy action, in-product navigation, export, docs/help
  link, support packet, or external handoff; and
- a short label that does not leak raw paths, private hostnames, account data,
  or raw evidence.

Rules:

1. Emergency banners MUST include at least one recovery or support/export
   link.
2. Revocation notices MUST include at least one rollback, repin, recovery,
   revocation, or support/export link.
3. Disclosure-link records MUST NOT point to a live external destination
   without stating whether the link is public, private, mirror-only, or
   policy-blocked.

## History, supersedence, and resolved state

The `history` group is required and carries:

- `history_state`;
- `supersedes_surface_refs`;
- `superseded_by_surface_ref`;
- `resolved_at`;
- `resolved_by_ref`;
- `retained_history_refs`; and
- `history_note`.

History states:

- `active`
- `active_emergency`
- `mitigated_active_history`
- `resolved_history`
- `superseded_history`
- `withdrawn_history`

Rules:

1. A superseded surface MUST carry `superseded_by_surface_ref`.
2. A resolved surface MUST carry `resolved_at`, a retained history ref, and
   enough linked records to reach the advisory, emergency action, revocation,
   support/export packet, or release-evidence packet that closed it.
3. Emergency state ending does not remove the advisory. The surface moves to
   `resolved_history` or `mitigated_active_history` and keeps the same copy-
   safe ids.

## Rendering obligations

Any consumer rendering this record must show, without opening a detail drawer:

- title;
- severity;
- action state;
- current mitigation state;
- affected row summary;
- current disclosure posture;
- primary action or recovery path; and
- copy-safe advisory id availability.

Detail, docs/help, release-packet, mirror note, and support/export consumers
must additionally preserve:

- all applies-to rows;
- all notice delivery rows;
- disclosure timing fields;
- acknowledgement/review state;
- linked rollback/revocation/support/export refs; and
- history/supersedence refs.

No surface may:

- reduce an emergency banner to a transient toast only;
- remove a mitigated advisory from history;
- claim live freshness from a mirror or manual-import snapshot;
- hide which exact builds or install profiles are affected;
- publish a public link without a visibility boundary; or
- copy advisory ids from a private surface in a way that leaks unrelated
  account, host, path, or evidence data.

## Change control

Adding a new `surface_kind`, `action_state`, `affected_surface_class`,
`notice_profile_class`, `visibility_class`, `history_state`,
`acknowledgement_state`, `review_state`, `mitigation_state`, or link kind is
additive-minor and requires an `advisory_surface_schema_version` bump plus
fixture coverage.

Repurposing an existing value is breaking and requires a governance decision
co-signed by security/trust and release owners. Existing history values are
never deleted; they are superseded.
