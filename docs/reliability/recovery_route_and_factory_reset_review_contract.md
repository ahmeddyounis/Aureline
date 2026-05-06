# Recovery-route scenario router, factory-reset review sheet, and external-recovery glossary contract

This document freezes the cross-surface contract every reliability,
support, repair, and recovery surface uses when it must **route a
large-failure scenario to a reviewed recovery path** and when a user is
about to confirm a **broad destructive factory reset**.

The goal is to prevent recovery copy and UI from collapsing into one
generic reset narrative: users must be able to see **which scenario they
are actually in**, the **narrowest safe path** that applies, and—when a
factory reset is still chosen—what will be **deleted**, **retained**,
**externally recoverable**, **not exported**, **policy-blocked**, or
**unsupported** before they commit.

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or one of the upstream contracts below, those documents win
and this contract plus the schemas and fixtures MUST be updated in the
same change.

The companion schemas live at:

- [`/schemas/recovery/recovery_route_decision.schema.json`](../../schemas/recovery/recovery_route_decision.schema.json)
- [`/schemas/recovery/factory_reset_review.schema.json`](../../schemas/recovery/factory_reset_review.schema.json)

Worked fixtures live under:

- [`/fixtures/recovery/recovery_route_cases/`](../../fixtures/recovery/recovery_route_cases/)

This contract composes with — and never re-defines — the recovery,
support, state, and reliability rules frozen elsewhere:

- [`/docs/reliability/recovery_scenario_contract.md`](./recovery_scenario_contract.md)
  — scenario families, safe-first-action matrix, destructive-risk
  labeling.
- [`/docs/reliability/corruption_rescue_compare_contract.md`](./corruption_rescue_compare_contract.md)
  — quarantined-copy preservation and per-artifact rescue/restore
  compare sheets.
- [`/docs/reliability/export_before_reset_contract.md`](./export_before_reset_contract.md)
  — export-before-reset checklist and verification-result contract.
- [`/docs/reliability/continuity_status_card_contract.md`](./continuity_status_card_contract.md)
  — recovery-promise class vocabulary and verification posture.
- [`/docs/recovery/recovery_rung_matrix.md`](../recovery/recovery_rung_matrix.md)
  — recovery rung selection: narrowest effective blast radius.
- [`/docs/release/update_and_rollback_contract.md`](../release/update_and_rollback_contract.md)
  — update apply / rollback candidate and exact-build continuity.
- [`/docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md)
  — entitlement lifecycle and seat recovery.
- [`/docs/recovery/restore_chooser_contract.md`](../recovery/restore_chooser_contract.md)
  — restore chooser and hydration posture for device replacement.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  — repair transaction preview, checkpoints, rollback/compensation, and
  evidence refs.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — support bundle body, redaction, and export posture.

Out of scope:

- implementing diagnosis engines, repair runners, reset runners, backup
  engines, or UI rendering;
- defining new recovery-scenario families or reset kinds (those are
  owned upstream by the referenced contracts).

## 1. External-recovery glossary (normative)

Surfaces MUST use the definitions below verbatim and MUST NOT substitute
private prose for these terms.

- **Authoritative backup** (`authoritative_backup`) — signed, verified,
  content-addressable backup of user-authored durable state. The only
  promise class that may be presented as a sole restore source for any
  restore target.
- **Local checkpoint** (`local_checkpoint`) — local-history checkpoints,
  autosave journals, and dirty-buffer sentinels. Authoritative for
  in-flight workspace state and window layout, never a substitute for
  profile-wide durable truth or retained evidence.
- **Sync replica** (`sync_replica`) — opt-in profile and layout replica.
  May rehydrate profile-shaped state after device loss; never carries
  unsaved workspace authority and never carries evidence bodies.
- **Mirror cache** (`mirror_cache`) — signed mirror or offline-bundle
  cache of upstream artifacts. Supports continued reads of known-good
  bytes; holds no user-authored truth and is never an authoritative
  restore source.
- **Convenience export** (`convenience_export`) — portable-state
  packages, support bundles, patch exports, and other user-consumable
  exports written for audit or transfer. Never authoritative on its own.
- **Externally recoverable artifact** — an artifact class is externally
  recoverable after a destructive action when at least one named source
  exists that is reachable from the user’s identity and carries the same
  authority class as the local body: an authoritative backup, a sync
  replica (profile-shaped state only), a managed-admin-seat re-issuance
  path (seat/license tokens only), or a signed offline bundle (mirror/
  docs-pack-shaped state only).

## 2. Scenario router — record model

One `recovery_route_decision_record` per routing decision. The record is
the stable inspectable body surfaces use to route a scenario to a
reviewed recovery path without collapsing into one generic reset story.

| Field | Purpose |
|---|---|
| `decision_id` | Stable id cited by logs, support bundles, and CLI output. |
| `generated_at` | Producer-local monotonic timestamp. |
| `scenario_family_class` | Closed scenario-family class (re-exported from the recovery-scenario contract). |
| `deployment_profile_scope_class` | Profile/deployment posture (`individual_local`, `managed_tenant`, etc). |
| `requested_route_class` (optional) | Route intent the surface was asked for (often a broad reset). |
| `recommended_route_class` | The reviewed route the router selects. |
| `recommended_reset_kind_class` (optional) | When the selected route is a reset-gated path, the specific reset kind. |
| `linkage` | Typed refs to the scenario card, compare sheet, export-before-reset checklist, factory-reset review, etc. |
| `glossary` | Const declaration that the external-recovery glossary terms above are in force. |
| `honesty_invariants` | Const guarantees the record cannot silently waive. |

## 3. Scenario router — closed route classes

The router resolves to exactly one closed `recommended_route_class`:

- `corruption_rescue_compare_route` — route through the corruption-rescue
  compare sheet for the narrowest affected artifact class.
- `update_and_rollback_route` — route through update/rollback candidate
  review and exact-build identity continuity.
- `entitlement_or_account_recovery_route` — route through entitlement /
  account recovery (reauth, entitlement refresh, or managed admin-seat
  recovery), not destructive reset.
- `control_plane_outage_route` — route through outage/failover posture:
  restate local-safe actions; no destructive reset while outage is
  active.
- `restore_chooser_route` — route through restore chooser and hydration
  phases for device replacement.
- `export_before_reset_checklist_route` — route through the export-before-
  reset checklist for the specific reset kind (never a generic warning).
- `factory_reset_review_route` — route through the factory-reset review
  sheet after (and only after) export-before-reset review.
- `support_bundle_export_route` — refuse local mutation and route to a
  support bundle export and escalation path.

## 4. Scenario router — selection rules

Rules (frozen):

1. **Narrowest safe path wins.** When multiple routes could plausibly
   apply, the router MUST select the narrowest reviewed blast radius
   before any broad reset path.
2. **Scenario-coded, not symptom-coded.** The router resolves to one
   closed scenario family (from the recovery-scenario contract) and
   keeps that family visible; it never routes via one generic “reset”
   narrative.
3. **Factory reset is not a first safe repair candidate.** A generalized
   factory reset is outside the normal ladder and MUST NOT be offered as
   the default or first safe route for any scenario family.
4. **Outage and derived-index cases forbid destructive routing.**
   `control_plane_outage` and `workspace_index_corruption` MUST NOT
   route to reset-gated paths.

## 5. Scenario router — required scenario coverage

The router MUST cover at least the following scenario families:

- `profile_corruption` → `corruption_rescue_compare_route` (suspect
  profile artifacts quarantine/compare/restore). Broad resets route
  only through `export_before_reset_checklist_route` and, for
  `factory_reset`, `factory_reset_review_route`.
- `workspace_index_corruption` → `corruption_rescue_compare_route`
  constrained to derived disposable artifacts only (rebuild path). Reset
  routes are forbidden.
- `failed_update` → `update_and_rollback_route` (rollback candidate
  review first). Broad reset routes remain last-resort and remain gated
  behind export-before-reset.
- `seat_loss` (seat/account loss) → `entitlement_or_account_recovery_route`
  (policy/entitlement refresh and managed admin-seat recovery), not
  reset.
- `control_plane_outage` → `control_plane_outage_route` (local-safe
  actions + evidence/export), not reset.
- `device_replacement` → `restore_chooser_route`. Factory reset of an
  abandoned device is permitted only after export-before-reset review
  plus the factory-reset review sheet.

## 6. Factory-reset review — record model

One `factory_reset_review_record` per factory reset confirmation. The
record is the shared inspectable body that a destructive confirmation
dialog, CLI flow, support bundle, and evidence packet can cite.

The review sheet MUST:

- name the **scenario family** the factory reset is associated with;
- enumerate reviewed artifact classes under **deleted**, **retained**,
  **externally recoverable**, **not exported**, **policy-blocked**, and
  **unsupported** buckets; and
- carry typed refs to the checkpoint/export/verification artifacts that
  justified proceeding.

## 7. Factory-reset review — bucket semantics (frozen)

Buckets are interpreted as follows:

- **Deleted** — artifact classes the factory reset will delete locally.
- **Retained** — artifact classes explicitly out of scope for this
  factory reset (if any).
- **Externally recoverable** — classes with a named external recovery
  source after reset (authoritative backup, sync replica, managed admin
  seat, offline bundle).
- **Not exported** — classes that will not have a convenience export
  produced as part of the export-before-reset gate (including user
  decline, size-capping, or deliberate omission).
- **Policy-blocked** — classes whose export step was refused by active
  policy.
- **Unsupported** — classes that cannot be exported by the product.

## 8. Export verification coverage (required)

Factory-reset review MUST cover the following export verification
postures by record fields and fixtures:

- verified export;
- partially verified export;
- export blocked by policy;
- export impossible for an unsupported class; and
- export intentionally skipped/declined by the user.

## 9. Acceptance

- Broad destructive recovery actions route through scenario-specific
  reviewed paths; factory reset confirmation uses the factory-reset
  review record and never reuses one generic reset warning.
- The scenario router selects the narrowest safe reviewed path and does
  not default to factory reset for profile corruption, index corruption,
  failed update, seat/account loss, control-plane outage, or device
  replacement.
- Glossary terms remain consistent across restore packets, export-before-
  reset checklists, and support bundles; surfaces do not invent private
  “backup/export/replica” semantics.

## 10. Changing this vocabulary

- **Additive-minor** changes (new route class; new bucket field; new
  glossary const) land in this document, both schemas, and the fixtures
  in the same change.
- **Repurposing** an existing route class, bucket meaning, or glossary
  term is **breaking** and requires a new decision row plus updated
  fixtures.

## Source anchors

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §18.31 — recovery surfaces,
  scenario-coded recovery copy, and export-before-reset gate for factory
  reset.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix DM.3 —
  repair rules: factory reset is outside the normal ladder.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.12.4 — narrowest
  blast radius preference and factory reset outside early safe repair.

## Linked artifacts

- Router decision schema:
  [`schemas/recovery/recovery_route_decision.schema.json`](../../schemas/recovery/recovery_route_decision.schema.json).
- Factory-reset review schema:
  [`schemas/recovery/factory_reset_review.schema.json`](../../schemas/recovery/factory_reset_review.schema.json).
- Fixtures:
  [`fixtures/recovery/recovery_route_cases/`](../../fixtures/recovery/recovery_route_cases/).

