# Emergency-action, revocation, and continuity object model

This document freezes the object model Aureline uses for trust-affecting
emergency actions, revocations, and their continuity/distribution state.
It exists so channel freezes, kill switches, trust-root rotations,
capability narrowing, emergency update pauses, and revocations behave as
signed product objects tied to exact builds and install lanes, not as ad
hoc banners, blog posts, or mirror-local notes.

Companion artifacts:

- [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — machine-readable boundary for `emergency_action_record` and
  `revocation_record`.
- [`/docs/security/severity_matrix.md`](./severity_matrix.md)
  — shared severity vocabulary, advisory identity model, monitored
  contact path, and incident-workspace packet rules this model reuses.
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  and
  [`/schemas/security/incident_workspace_packet.schema.json`](../../schemas/security/incident_workspace_packet.schema.json)
  — adjacent security-response records that link to emergency actions
  and revocations by stable ref instead of minting a parallel identity
  system.
- [`/docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  and
  [`/docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md`](../adr/0017-release-posture-artifact-families-and-promotion-gates.md)
  — release-artifact, rollback-atom, mirror/manual-import, and
  advisory/revocation granularity rules this model projects from.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support/export surfaces that preserve emergency-action and
  revocation refs instead of flattening them into free-text issue notes.
- [`/fixtures/security/emergency_action_examples/`](../../fixtures/security/emergency_action_examples/)
  — worked examples covering channel freeze, signer continuity, mirror
  freshness, manual-import receipt refs, supersedence, revocation, and
  post-incident reconciliation.

Normative sources this model projects from:

- `.t2/docs/Aureline_PRD.md` §10.15, §10.18, and the transport/mirror
  sections governing offline and air-gapped distribution.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6.1,
  §22.8, §26.7, and the release/security/offline architecture appendices.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13 and the
  verification-lane seed covering advisories, emergency notices,
  affected-install assessments, and revocations.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix AL, Appendix CX,
  the update-center/activity-center/service-impairment sections, and the
  mirror/offline continuity templates.
- [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  for the action ids, quorum floors, and break-glass audit fields that
  emergency actions and revocations cite.

If this document disagrees with those sources, those sources win and
this document plus the schema update in the same change.

## Why this exists

The repository already froze:

- one severity vocabulary and advisory identity model;
- one exact-build identity model and install-topology vocabulary;
- one release-artifact graph and mirror/manual-import emergency path;
- one support-bundle and support-packet family contract; and
- one signing-quorum policy for freeze, revocation, disable, and
  trust-root actions.

What it did not yet freeze was the **shared product object** that
connects those pieces once the product must tell a user or admin:

- which capability or channel changed state;
- why the change happened;
- which builds, install lanes, and deployment profiles are affected;
- whether the currently-seen action came from the live authority, a
  mirror snapshot, a preloaded runtime rule, or a manual-imported file;
- whether signer continuity is intact, rotated with proof, or broken;
- what still works locally right now;
- what the user or admin must do next and by when; and
- how the action is superseded, reconciled, or linked into the
  post-incident trail.

Without one governed object model, every surface would invent a local
dialect:

- banners would reduce the event to a short warning and lose build or
  install-lane identity;
- activity-center rows would invent a second urgency scale;
- update-center rows would have their own mirror/offline freshness story;
- mirror import and manual-import receipts would describe signer state
  differently from the advisory;
- support packets would quote prose instead of durable ids; and
- post-incident review would have to reconstruct which emergency action
  actually narrowed local behavior.

This model closes that gap.

## Scope

Frozen at this revision:

- one `emergency_action_record` for channel freezes, kill switches,
  trust-root rotations, capability narrowing, and emergency update
  pauses;
- one `revocation_record` for durable revocation state without a
  separate incompatible identity layer;
- one shared affected-scope model reusing advisory
  `affected_install_linkage`, exact-build refs, install-profile card
  refs, deployment-profile scope, and typed affected-subject refs;
- one shared distribution-state model that represents authoritative,
  mirrored, manual-imported, offline, preloaded, and cached emergency
  metadata with explicit freshness and signer continuity;
- required local-continuity notes, required-action rows, deadline
  semantics, durable history refs, and reconciliation hooks; and
- explicit surface-consumption rules for banners, activity center,
  update center, admin exports, mirror imports, manual-import receipts,
  and support packets.

Out of scope:

- live signing infrastructure or transparency-log implementation;
- the raw bytes of a disable bundle or policy bundle;
- transport protocol implementation for managed push/pull, registry
  publication, mirror sync, or file import; and
- final banner, update-center, or admin-plane UI implementation.

## One model, two record kinds

The schema freezes two record kinds in one file:

| Record kind | Purpose | Why it exists separately |
|---|---|---|
| `emergency_action_record` | Temporary or bounded trust-affecting action such as channel freeze, kill switch, trust-root rotation, capability narrowing, or emergency update pause | these actions often expire, require reconciliation, or are superseded by a later action or fixed release |
| `revocation_record` | Durable revocation state over a subject or subject set | a revocation should remain inspectable even after the triggering emergency action expires or is superseded |

The two kinds deliberately share the same:

- severity vocabulary (`security_severity_class`);
- affected-install linkage fields;
- deployment-profile scope;
- distribution-state rows;
- signer-continuity fields;
- local-continuity fields;
- required-action rows;
- history-link grammar; and
- reconciliation hooks.

That is the mechanism that keeps advisories, emergency actions, and
revocations on one artifact-identity graph instead of fragmenting.

## Shared fields and vocabularies

### Severity and notice state

The record reuses the advisory severity vocabulary from
[`/docs/security/severity_matrix.md`](./severity_matrix.md):

- `security_severity.operational_emergency`
- `security_severity.critical`
- `security_severity.high`
- `security_severity.medium`
- `security_severity.low`

The record also carries a **separate** `surface_notice_class` so
surfaces can distinguish ordinary severity from the stronger user-facing
state `emergency_action_required`.

Closed set:

- `emergency_action_required`
- `critical_notice`
- `high_notice`
- `moderate_notice`
- `low_notice`

Rules:

1. `emergency_action_required` is not a synonym for severity. It is the
   user/admin-facing state used when a kill switch, trust-root
   transition, or channel freeze changes what can safely run right now.
2. A record with `surface_notice_class = emergency_action_required`
   MUST still carry one ordinary `security_severity_class`.
3. Surfaces render the notice state directly from the record; they do
   not infer it from icon color or invent a surface-local boolean.

### Reason, trigger, and enforcement posture

Every record carries:

- `reason_class` — why the action exists;
- `trigger_source_class` — what authoritative mechanism caused it; and
- `enforcement_posture_class` — what the action actually does.

These are separate because "runtime policy narrowed one feature" and
"signed emergency bundle froze a channel" are materially different
events even when both are high severity.

### Affected scope

Emergency actions and revocations reuse the same identity model the
advisory already uses:

- `affected_subjects[]`
- `affected_install_linkage`
- `deployment_profile_scope[]`

Rules:

1. If a record claims it affects installed behavior, it names the same
   install-profile card refs and exact-build refs that advisory,
   release-evidence, and support surfaces already use.
2. Free-text "1.x stable" is non-conforming once exact-build refs or
   install-profile card refs are available.
3. Revocations do not get a looser identity story than advisories.
   They use the same build/install joins.

### Signer continuity

Every record carries a `signer_continuity` object because mirrors,
offline imports, and trust-root rotations need more nuance than a
single "signed/unsigned" flag.

Closed `continuity_state` set:

- `same_signer_chain`
- `rotated_with_continuity_statement`
- `cross_signed_transition`
- `continuity_review_required`
- `continuity_broken`
- `unknown_offline`

Rules:

1. `unknown_offline` is honest and admissible for mirrored or offline
   views; the product must not label an imported snapshot as live-
   verified when it was not.
2. A trust-root rotation or signer change should point at the new and
   previous signer refs plus any continuity statement or successor root
   ref in the same record.
3. Signer continuity is distinct from freshness. A mirror may have a
   continuity-valid snapshot that is still stale or superseded.

### Distribution state

The `distribution_statuses[]` rows let one record describe how the same
action appears across live, mirrored, manual-imported, offline, cached,
or preloaded environments.

Each row names:

- `source_class` — authoritative origin, mirror, manual-import bundle,
  offline snapshot, local cache, or runtime preload;
- `path_class` — managed push/pull, mirror sync, file import, offline
  transfer, local-cache projection, or runtime preload;
- `freshness_class` — current, stale-within-grace, stale-past-grace,
  manual snapshot current/stale, offline snapshot unexpired/expired, or
  unknown;
- `validation_state` — whether this path was verified current, verified
  but stale, failed verification, is pending review, or is only a cached
  inherited state;
- `observed_signer_continuity_state` — the continuity state visible on
  that path; and
- `receipt_ref` — the stable id a mirror-import row, manual-import
  receipt, or support packet preserves when it quotes the same action.

Rules:

1. Mirror or offline paths MUST project freshness as mirror/offline
   state, not as implicit live authority.
2. A file/manual import that has detached-signature proof but no live
   freshness recheck is allowed and should render as verified plus
   snapshot freshness, not as "live now".
3. Supersedence remains explicit on distribution rows via
   `supersedes_known_snapshot_ref`; a stale imported action does not
   silently disappear when a newer one is known elsewhere.

### Local continuity

Every record carries one `local_continuity` block.

Minimum contents:

- `posture_class`
- `note`
- `retained_capabilities[]`
- `blocked_capabilities[]`

Rules:

1. "What still works locally?" is required product truth, not optional
   documentation.
2. `retained_capabilities[]` and `blocked_capabilities[]` must be
   truthful at the same scope as `affected_install_linkage`.
3. Local continuity is not the same as mitigation. A record may say
   local editing still works while also requiring an admin to pause
   updates or import a superseding snapshot.

### Required actions and deadlines

Each `required_actions[]` row states:

- who must act (`actor_class`);
- what they must do (`action_class`);
- whether it blocks recovery or is only advisory (`blocking_class`);
- how the deadline should be interpreted (`deadline_semantics`); and
- the absolute deadline when one exists (`deadline_at`).

Rules:

1. Deadline semantics must stay typed. "Soon" or "ASAP" is not enough
   for machine-readable export.
2. The record may have several actions with different owners and
   deadlines; surfaces must not flatten them into one generic button.
3. For `emergency_action_required`, at least one required-action row
   must exist.

### History and reconciliation

Two blocks prevent emergency state from becoming ahistorical:

- `history_links` preserves durable refs to decision history, activity
  events, support packets, admin exports, release-evidence packets,
  public notices, and distribution receipts.
- `reconciliation` preserves the current follow-up state, owner,
  post-incident review refs, and resume-condition note.

Rules:

1. Superseded or expired emergency actions remain inspectable through
   `history_links`; silent deletion is forbidden.
2. `reconciliation.resume_condition_note` tells the user/admin what
   must become true before a frozen channel resumes or a narrowed
   capability widens again.
3. Post-incident review refs belong on the same object, so the banner,
   support packet, and admin export can all point at the same durable
   trail.

## Emergency-action record

`emergency_action_record` is the bounded response object.

Closed `action_kind` set:

- `channel_freeze`
- `capability_kill_switch`
- `trust_root_rotation`
- `capability_narrowing`
- `emergency_update_pause`

Rules:

1. `channel_freeze`, `capability_kill_switch`, and
   `trust_root_rotation` MUST render as
   `surface_notice_class = emergency_action_required`.
2. `channel_freeze` and `emergency_update_pause` carry the channel refs
   in `enforcement_details.affected_channel_refs`.
3. `capability_kill_switch` and `capability_narrowing` carry the
   affected capability refs in
   `enforcement_details.affected_capability_refs`.
4. `trust_root_rotation` carries `successor_signer_ref` and/or
   `successor_trust_root_ref` in `enforcement_details`.
5. The action may expire or be superseded, but its durable history and
   relation to advisories/revocations remain.

## Revocation record

`revocation_record` is the durable state object.

It adds:

- `revocation_id`
- `blast_radius_class`
- `revoked_subject_refs[]`

Rules:

1. Revocation is a durable state, not a missing listing or hidden
   package removal.
2. `revoked_subject_refs[]` points at the same exact-build,
   install-profile, signer, trust-root, extension, route, docs-pack, or
   model-pack identities other surfaces already use.
3. `blast_radius_class` names how far the revocation reaches:
   exact subject, exact-build subset, install-profile subset,
   channel-wide, deployment-profile-wide, or trust-chain-wide.
4. Rollback/downgrade or successor guidance belongs in
   `enforcement_details.recovery_guidance_note`; a revocation cannot
   leave the user/admin with only "blocked" and no path explanation.

## Surface-consumption rules

The following surfaces project directly from the record. They may
reformat or hide rows by policy, but they may not invent different field
names or omit the shared ids.

| Surface | Required fields from the record | Forbidden shortcut |
|---|---|---|
| **Contextual banner** | `surface_notice_class`, `title`, `summary`, `local_continuity.note`, the highest-priority `required_actions[]` row, and any visible deadline | inventing a local `is_emergency` boolean or replacing continuity with generic "some features may be unavailable" |
| **Activity center / durable attention** | stable id (`action_id` / `revocation_id`), `surface_notice_class`, `reason_class`, `record_state`, `updated_at` via distribution/history, `local_continuity.note`, and reopen/history refs | flattening the object into a toast-only message |
| **Update center** | `affected_install_linkage`, `enforcement_posture_class`, `enforcement_details.minimum_safe_version_ref`, `distribution_statuses[]`, `signer_continuity`, and `required_actions[]` | inventing a separate affected-version table or separate mirror freshness story |
| **Admin export / admin handoff** | the full record, including `history_links`, `reconciliation`, `distribution_statuses[]`, and signer continuity | replacing durable refs with prose-only summary |
| **Mirror import view** | the applicable `distribution_statuses[]` row, `receipt_ref`, `freshness_class`, `validation_state`, `observed_signer_continuity_state`, and `supersedes_known_snapshot_ref` | pretending the mirror import is live-authoritative |
| **Manual-import receipt** | the same `distribution_statuses[]` row plus the stable action/revocation id, `required_actions[]`, and `local_continuity.note` | minting receipt-local freshness or signer terms that disagree with the action object |
| **Support packet / support bundle** | stable action/revocation id, `affected_install_linkage`, `local_continuity`, `distribution_statuses[]`, `history_links`, and `reconciliation` refs | collapsing the action into an unstructured support note |

## Change control

- Adding a new action kind, reason class, trigger source class,
  enforcement posture class, surface-notice class, signer-continuity
  state, distribution freshness class, distribution validation state,
  required-action class, deadline semantics class, record-state class,
  blast-radius class, or reconciliation state is additive-minor and
  requires a schema/document update in the same change.
- Repurposing an existing value is breaking and requires a new decision
  row co-signed by `security_trust_review` and `release_council`.
- Any change that alters affected-install identity or continuity meaning
  must update this document, the schema, and the adjacent advisory /
  support / release references in the same change.

## Current follow-up boundary

This model intentionally stops at the object boundary. Future work may
still land:

- disable-bundle bytes and raw policy/update transport schemas;
- a concrete `/SECURITY.md` monitored contact path;
- first-class admin-plane or update-center implementations; and
- live publication/import tooling over the distribution rows defined
  here.
