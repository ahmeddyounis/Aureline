# Two-person control, break-glass audit, and post-incident reconciliation contract

This document is the normative contract for how Aureline invokes,
audits, and reconciles high-risk freeze, revocation, trust-root, and
emergency-policy actions. It freezes the rule that high-risk control
paths do not depend on a single maintainer in steady state, that any
admitted single-responder (break-glass) path is time-boxed and
attributable, and that every break-glass invocation ends at a
retrospective co-sign, a superseding signed action, or an explicit
withdrawal recorded on the same control-event id.

Companion artifacts:

- [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  — canonical action matrix for protected release, policy, emergency,
  and security-sensitive artifact approvals. The action ids, quorum
  profiles, and break-glass profile rules cited in this document live
  there.
- [`/docs/governance/maintainer_coverage_policy.md`](../governance/maintainer_coverage_policy.md)
  — reviewer-depth, backup-owner, and waiver rules the quorum policy
  builds on.
- [`/schemas/security/break_glass_event.schema.json`](../../schemas/security/break_glass_event.schema.json)
  — machine-readable boundary for `break_glass_event_record`. One row
  per break-glass invocation; referenced by
  `private_triage_workspace_packet_record.break_glass_refs.audit_row_ref`,
  release-evidence packets, support-export packets, and post-incident
  review records.
- [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — shared object model for channel freezes, kill switches, trust-root
  rotations, capability narrowing, and emergency update pauses; plus
  the revocation_record form. Break-glass events resolve against these
  ids rather than minting parallel identity.
- [`/schemas/security/private_triage_workspace_packet.schema.json`](../../schemas/security/private_triage_workspace_packet.schema.json)
  — private-triage packet with `break_glass_state` and `break_glass_refs`
  fields; a triage packet that names a break-glass invocation MUST cite
  the same `audit_row_ref` this contract defines.
- [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  — emergency-action object model and reconciliation hooks.
- [`/docs/security/emergency_distribution_policy.md`](./emergency_distribution_policy.md)
  — mirror-safe distribution, manual-import receipt, and metadata-chain
  rules that break-glass distribution still follows.
- [`/fixtures/security/break_glass_cases/`](../../fixtures/security/break_glass_cases/)
  — worked fixtures covering the admitted break-glass classes and the
  forbidden classes.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` §10.15 and §10.18 (trust, emergency
  actions, and distribution posture).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6,
  §22.8, and §26.7 (release, security, and emergency-control
  architecture).
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13
  (advisory, emergency action, revocation, and compensating-control
  contract surface).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix AL and
  Appendix CX (update-center, activity-center, support-export
  continuity).

If this document disagrees with those sources or with
`signing_quorum.yaml`, the normative sources win and this document plus
the schema update in the same change.

## 1. Scope

Frozen at this revision:

- which control actions default to two-person (or three-person)
  quorum, the minimum required roles and forums for each, and the
  retrospective quorum profile any admitted break-glass invocation
  must close out against;
- which break-glass classes are admitted, which are forbidden, and
  the maximum admissible duration for each admitted class;
- the `break_glass_event_record` shape the audit row carries, its
  cross-record linkage to advisories, emergency actions, revocations,
  release-evidence, support exports, and post-incident review records,
  and its lifecycle states;
- the reconciliation workflow that turns every break-glass invocation
  into either a retrospective co-sign, a superseding signed action,
  or an explicit withdrawal.

Not in scope:

- paging policy, named on-call rotations, staffing commitments, or
  who answers the monitored contact path;
- the production signing infrastructure that validates the quorum
  signatures live;
- public advisory drafting, release-note copy, or the UI for the
  update-center / activity-center surfaces that consume break-glass
  state.

## 2. Quorum principles

The action matrix in
[`signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
is authoritative for the per-action quorum profile and break-glass
profile. This document names the principles the matrix enforces so
reviewers can check that a new action row fits:

- **No single-human release path.** Every promotion, publication, or
  policy-widening action resolves to at least two-person control.
- **Default two-person control for high-risk actions.** Channel
  freeze, capability kill switch, emergency-policy publication,
  revocation, and every action that narrows trust posture requires at
  least `two_person_cross_forum_emergency_control` in steady state.
- **Three-person control for trust-root change.** Planned signer or
  trust-root replacement uses `three_person_trust_root_change`;
  emergency containment for compromise still uses the freeze /
  revocation rows and returns to this profile for the permanent
  replacement.
- **Release widening never uses break-glass.** Stable / LTS
  promotion, widening a claim scope, and permanent signer-roster
  change are listed under `break_glass_profile: none`. If quorum is
  missing, the action waits or the scope narrows.
- **Break-glass is containment only.** The admitted single-responder
  profile freezes, disables, pauses, or publishes a bounded emergency
  packet. It does not widen access, normalize a single-person action,
  or mutate audit history.
- **Break-glass must be audited and time-boxed.** Every invocation
  mints a `break_glass_event_record` row with the required audit
  fields from the `audited_single_responder_containment` profile.
- **Retrospective co-sign required before scope resume.** A
  break-glass invocation is not closed until a quorum that matches the
  `retrospective_quorum_profile` on the break-glass profile co-signs
  the event or a superseding signed action lands.

The principles are not aspirational: each one maps to a rule this
document and `signing_quorum.yaml` enforce together. A control path
that violates a principle is non-conforming even if the PR template
is otherwise green.

## 3. High-risk action classes and default quorum

The action rows below are the high-risk classes this contract binds.
Each row resolves to an action id in
[`signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml);
the quorum profile there is authoritative. This table names the intent
so downstream surfaces can cite one class id rather than restate the
rule.

| Action class | signing_quorum.yaml action id | Default quorum profile | Break-glass admitted? | Retrospective quorum profile on close-out |
|---|---|---|---|---|
| Channel freeze or resume | `channel_freeze_or_resume` | `two_person_cross_forum_emergency_control` | yes (`audited_single_responder_containment`) | `two_person_cross_forum_emergency_control` |
| Revocation, disable bundle, or kill-switch publication | `revocation_disable_or_kill_switch_publication` | `two_person_cross_forum_emergency_control` | yes (`audited_single_responder_containment`) | `two_person_cross_forum_emergency_control` |
| Emergency policy bundle publication | `emergency_policy_bundle_publish` | `two_person_cross_forum_emergency_control` | yes (`audited_single_responder_containment`) | `two_person_cross_forum_emergency_control` |
| High-severity security publication | `high_severity_security_publication` | `two_person_cross_forum_emergency_control` | yes (`audited_single_responder_containment`) | `two_person_cross_forum_emergency_control` |
| Trust-root or signer-roster change | `signer_roster_or_trust_root_change` | `three_person_trust_root_change` | no | not applicable — no admitted break-glass |
| Stable or LTS promotion | `stable_or_lts_promotion` | `three_person_stable_promotion` | no | not applicable — no admitted break-glass |
| Preview or beta promotion | `preview_or_beta_promotion` | `two_person_release_control` | no | not applicable — no admitted break-glass |
| Routine policy bundle publication | `routine_policy_bundle_publish` | `two_person_release_control` | no | not applicable — no admitted break-glass |
| Release evidence acceptance | `release_evidence_acceptance` | `two_person_release_control` | no | not applicable — no admitted break-glass |

### 3.1. Authority minimums per profile

The required roles and forums are named in full on the quorum profile
row in `signing_quorum.yaml`. Summary:

- `two_person_release_control` — release council forum; `release_operator`
  plus `evidence_owner_or_backup`; author-only forbidden.
- `three_person_stable_promotion` — release council plus shiproom
  executive scope review; `candidate_owner`, `evidence_owner`, and
  `publishing_operator_or_backup`; author-only forbidden.
- `two_person_cross_forum_emergency_control` — security trust review
  plus release council; `security_operator` plus `release_operator`;
  author-only forbidden.
- `three_person_trust_root_change` — security trust review plus
  release council; `security_operator`, `release_operator`, and
  `backup_signer_or_auditor`; author-only forbidden.

The `author_only_forbidden` bit is not a suggestion: an action whose
signatures all resolve to the same human is non-conforming and the
surface consuming the action MUST refuse to project it as quorum-backed.

## 4. Break-glass admissibility

The admitted profile is `audited_single_responder_containment`. It is
the only single-responder path permitted by this contract.

### 4.1. Admitted causes

A break-glass invocation is admissible only when at least one of the
following holds:

- `security_severity.operational_emergency` is declared;
- an active signed-artifact compromise is observed and the live
  authoritative path cannot wait for the full quorum;
- verified malicious distribution is observed on a mirror or
  manual-import channel;
- an urgent channel freeze is required to protect live users.

These map 1:1 to the `allowed_when` values on the break-glass profile
in `signing_quorum.yaml`.

### 4.2. Forbidden classes

Break-glass is never admitted for:

- stable or LTS promotion;
- widening a claim, disclosure, or support scope;
- permanent signer-roster change;
- deleting or mutating audit history (including retroactively changing
  a closed break-glass event record);
- informal approval of an ordinary protected-path change that happens
  to be urgent for product reasons but is not a security-containment
  action.

These map 1:1 to the `forbidden_for` values on the break-glass profile.
An invocation that resolves to a forbidden cause fails closed: the
surface MUST refuse the action and the operator MUST wait for the
quorum or narrow scope.

### 4.3. Duration envelope

`max_duration_hours: 24` from the break-glass profile is the
authoritative ceiling. A `break_glass_event_record` whose lifecycle
state has not moved from `invoked_pending_reconciliation` to a closed
state within 24 hours of `invoked_at` is non-conforming and every
consuming surface MUST mark it overdue.

### 4.4. Expiry interaction with the emergency-action record

The emergency-action or revocation record the break-glass invocation
produced still carries its own `expires_at` and `review_deadline_at`
per the emergency-action model. The break-glass expiry is the ceiling
on the invocation itself, not on the emergency action; a
short-lived break-glass invocation may produce a longer-lived
emergency-action record whose own expiry and supersedence rules
continue to apply after the invocation closes.

## 5. Break-glass audit record

Every break-glass invocation mints exactly one
`break_glass_event_record` row. The machine-readable contract lives in
[`/schemas/security/break_glass_event.schema.json`](../../schemas/security/break_glass_event.schema.json).
The required fields are:

- `break_glass_event_schema_version` (constant 1 at this revision).
- `record_kind` (const `break_glass_event_record`).
- `break_glass_event_id` — opaque id stable for the life of the
  invocation. This is the id the triage packet, advisory, emergency
  action, revocation, release-evidence packet, support export, and
  post-incident review all cite.
- `signing_quorum_action_ref` — opaque ref into
  `signing_quorum.yaml#actions[]` naming which action row this
  invocation used.
- `break_glass_profile_ref` — opaque ref into
  `signing_quorum.yaml#break_glass_profiles[]` (currently always
  `audited_single_responder_containment`).
- `invoking_actor` — actor class, role class, attribution ref, and a
  reviewable operator note. Raw human identifiers, chat handles, or
  pager ids MUST NOT appear; the record carries opaque refs only.
- `reason_class` — one of the admitted causes listed in §4.1.
- `urgency_class` — reused from the emergency-action model.
- `scope` — affected subjects, deployment-profile scope, channel
  scope, and at least one affected-install-profile-card ref, shared
  with the emergency-action model so a surface does not mint a second
  scope vocabulary.
- `related_emergency_action_refs` — opaque refs to the produced
  `emergency_action_record` ids. Every invocation that produced or
  advanced an emergency action names at least one ref.
- `related_revocation_refs` — opaque refs to produced
  `revocation_record` ids. Every invocation that produced or
  advanced a revocation names at least one ref.
- `related_advisory_refs` — opaque refs to the advisory records the
  invocation affected.
- `related_private_triage_packet_ref` — the private-triage workspace
  packet that owns the incident during response. The triage packet's
  `break_glass_refs.audit_row_ref` MUST equal this record's
  `break_glass_event_id`.
- `invoked_at` / `must_close_by` / `closed_at` — invocation time, the
  time by which close-out must happen (at most 24 hours after
  `invoked_at`), and the close-out time (null while open).
- `lifecycle_state` — one of `invoked_pending_reconciliation`,
  `reconciled_with_retrospective_cosign`, `superseded_by_signed_action`,
  `expired_without_reconciliation`, or `withdrawn_invalid_invocation`.
- `retrospective_cosign` — the retrospective quorum profile ref, the
  cosigner role refs, the forum refs, and the cosign timestamp. Null
  while the invocation is still open; required on any closed state
  other than `withdrawn_invalid_invocation`.
- `temporary_mitigation_note` — reviewable sentence describing what
  the break-glass step did in product terms.
- `reconciliation_plan_note` — reviewable sentence describing how the
  invocation will be closed out.
- `history_links` — opaque refs into release-evidence packets,
  support-export packets, admin exports, incident-workspace packet,
  post-incident review records, and the decision-history rows that
  ratified the invocation.
- `redaction_class` — redaction posture for the record (log_safe,
  support_export_only, evidence_packet_only, release_public,
  private_triage_only).
- `minted_at` — when the record was first minted.

Fields MUST appear exactly as named; surfaces MUST NOT invent
break-glass-local scope, continuity, or supersedence fields. Adding a
new value to a vocabulary (for example a new `reason_class` value) is
additive-minor and bumps `break_glass_event_schema_version`.
Repurposing an existing value is breaking and requires a new decision
row co-signed by security_trust_review and release_council.

## 6. Reconciliation workflow

Every break-glass invocation follows one of three close-out paths.
The lifecycle state moves only along the paths below. A record that
advances off-lane is non-conforming and the consuming surface MUST
refuse to project it as closed.

### 6.1. Retrospective co-sign

- **Lifecycle:** `invoked_pending_reconciliation` →
  `reconciled_with_retrospective_cosign`.
- **Required:** a quorum that matches the
  `retrospective_quorum_profile` on the break-glass profile (currently
  always `two_person_cross_forum_emergency_control`) co-signs the
  record before `must_close_by`.
- **Record change:** `closed_at` is populated; the
  `retrospective_cosign` block names the profile ref, the cosigner
  role refs, the forum refs, and the cosign timestamp.
- **Surface effect:** the produced emergency action or revocation
  continues under its own expiry and supersedence rules; support
  exports, admin exports, and post-incident review rows cite the
  closed event id.

### 6.2. Superseded by signed action

- **Lifecycle:** `invoked_pending_reconciliation` →
  `superseded_by_signed_action`.
- **Required:** a signed action that lands under the full default
  quorum for the action row (for example, a freeze produced under the
  admitted single-responder path is replaced by a freeze co-signed
  under `two_person_cross_forum_emergency_control`, or a revocation is
  replaced by a superseding revocation record).
- **Record change:** `closed_at` is populated; the
  `retrospective_cosign` block names the quorum the superseding action
  carried; `related_emergency_action_refs` / `related_revocation_refs`
  name the superseding records.
- **Surface effect:** consuming surfaces follow the superseding
  record; the break-glass event id remains cited but carries the
  superseded state.

### 6.3. Expired without reconciliation

- **Lifecycle:** `invoked_pending_reconciliation` →
  `expired_without_reconciliation`.
- **Required:** `must_close_by` has passed without a retrospective
  co-sign and without a superseding signed action.
- **Record change:** `closed_at` is populated (set to the expiry
  crossing); `retrospective_cosign` remains null; a
  `reconciliation_plan_note` MUST describe the correction path.
- **Surface effect:** a correction signal is raised: update center,
  activity center, and support export surfaces mark the containment
  action overdue; the owning release-evidence or shiproom packet
  cannot claim quorum coverage for the affected scope; a post-incident
  review record is required before the same scope resumes.

### 6.4. Withdrawn invalid invocation

- **Lifecycle:** `invoked_pending_reconciliation` →
  `withdrawn_invalid_invocation`.
- **Required:** the invoking operator or a security-trust-review
  reviewer determines the invocation was not admissible (for example
  the underlying cause did not resolve to an admitted class, or the
  invocation was a mistake).
- **Record change:** `closed_at` is populated;
  `retrospective_cosign` remains null; `temporary_mitigation_note`
  names what was undone; the underlying emergency-action or
  revocation record (if any was minted) is withdrawn via its own
  supersedence / withdrawal rules.
- **Surface effect:** the invocation is visible in history as an
  explicit retraction rather than a silent rollback. A withdrawn
  invocation is not reused for future accounting.

A `break_glass_event_record` MUST NOT move between closed states.
Reopening a closed invocation is non-conforming; if new information
makes a closed invocation incorrect, a new `break_glass_event_record`
is minted (or a new emergency action is produced under the full
quorum) rather than mutating the closed row.

## 7. Cross-surface referencing rule

Every release, security, and support artifact that touches a
break-glass invocation MUST cite the same `break_glass_event_id`:

- **Private-triage workspace packet.** `break_glass_state` moves off
  `break_glass_not_invoked`; each entry in `break_glass_refs` carries
  an `audit_row_ref` equal to the `break_glass_event_id`.
- **Advisory records.** When the triage they feed invoked
  break-glass, `related_advisory_refs` on the event names the
  advisory; downstream advisory surfaces cite the event id rather than
  free-text "emergency response" copy.
- **Emergency-action and revocation records.** The emergency action
  or revocation produced by the invocation names the event id via
  `history_links.decision_history_refs` or an explicit
  `break_glass_event_ref` on the emergency-action record's history
  link block; the event, in turn, names the emergency-action or
  revocation id.
- **Release-evidence packets.** The packet names the event id in its
  decision-history links; a packet that claims quorum coverage while
  an unreconciled break-glass event is open for the affected scope is
  non-conforming.
- **Support-export and admin-export packets.** Both quote the event
  id under the redaction class on the event record; support exports
  MAY carry a reviewable summary sentence but MUST NOT invent a
  second event id.
- **Post-incident review records.** The review row cites the event
  id, the emergency-action / revocation ids produced by the
  invocation, the retrospective cosigners (when applicable), the
  correction status, and the follow-up plan.

Cross-surface fan-out MUST preserve ids. If a surface cannot carry an
opaque id (narrow UI, one-line banner), it cites the event id
indirectly through the emergency-action or advisory record rather
than minting a surface-local label.

## 8. Acceptance criteria

A high-risk control path meets this contract iff:

- every applicable action in `signing_quorum.yaml` names a
  `default_quorum_profile` whose `min_distinct_humans` is at least 2
  and whose `author_only_forbidden` is true;
- actions admitted for break-glass cite the
  `audited_single_responder_containment` profile and no other;
- actions forbidden from break-glass (stable / LTS promotion,
  permanent signer-roster change, release-evidence acceptance,
  routine policy publication, preview / beta promotion) resolve to
  `break_glass_profile: none`;
- every break-glass invocation produces a
  `break_glass_event_record` validating against the schema;
- every invocation closes within 24 hours of `invoked_at` to one of
  the four admitted lifecycle states in §6, or is explicitly marked
  expired;
- the private-triage workspace packet, advisory, emergency-action,
  revocation, release-evidence packet, support-export packet, admin
  export, and post-incident review records all cite the same
  `break_glass_event_id` without re-keying.

A path that violates any bullet above is non-conforming and the
consuming surface MUST refuse to project it as quorum-backed.

## 9. Change discipline

- Adding a new `reason_class`, `lifecycle_state`,
  `retrospective_cosign` field, or a new `history_link` slot on the
  `break_glass_event_record` is additive-minor and bumps
  `break_glass_event_schema_version`.
- Adding a new quorum profile or a new break-glass profile in
  `signing_quorum.yaml`, or a new action row citing an existing
  profile, is additive-minor and updates this document in the same
  change.
- Raising the `max_duration_hours` ceiling, admitting a new
  forbidden class, or changing any `retrospective_quorum_profile`
  mapping is breaking and requires a new decision row co-signed by
  security_trust_review and release_council.
- Renaming an action id is breaking; aliasing is not admitted at
  this revision.
- Removing or repurposing any enumerated value on the event schema
  (including `lifecycle_state`) is breaking and follows the same
  decision-row discipline.

## 10. Out of scope

- Named on-call rotations, paging policy, or staffing commitments
  beyond the quorum floor and audit requirements frozen here.
- The production signing infrastructure that validates retrospective
  cosigns live; the schema carries opaque cosigner refs only.
- Public advisory drafting, release-note copy, or the activity-center
  and update-center UI that consume the break-glass state.
- Cross-vendor coordination (coordinated disclosure groups, vendor
  partner channels); those routes use the private-triage workspace
  packet and do not mint a second break-glass identity.
