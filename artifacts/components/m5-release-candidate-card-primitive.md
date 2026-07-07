# M5 Release-Candidate Card and Promotion-Blocked-Banner Primitive

- Packet: `m5-release-candidate-card-primitive:stable:0001`
- Label: `M5 release-candidate card and promotion-blocked-banner primitive: identity, channel, scoped artifact set, blocker summary, evidence freshness, known issues, and rollback path`
- Release-candidate consumers: 5 (5 stable)
- Promotability postures: promotable, promotable_with_reservations, promotable_under_waiver, narrowed_pending_reverify, narrowed_rollback_undefined, blocked_hard_blocker, blocked_stale_evidence, blocked_missing_evidence, blocked_unknown_state
- Evidence-freshness states: evidence_fresh, evidence_aging, evidence_stale, evidence_missing, evidence_freshness_unknown
- Block reasons: hard_blocker_open, evidence_stale, evidence_missing, candidate_state_unknown, blocker_pending_reverify, rollback_target_undefined
- Rollback-path readinesses: rollback_target_pinned, no_prior_to_roll_back_to, rollback_target_undefined
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Release-candidate consumers

- **Release-Center Card**: `stable`
  - Owner: Release-center card owner
  - Scope: The release-center card renders the shared candidate primitive so a multi-family candidate with fresh evidence and a pinned rollback target reads as promotable, while a full-train candidate with an open hard blocker reads as blocked with a self-contained banner naming the reason and next action
  - Worked resolutions: 2
    - `5.2.0-rc.1` on `stable_channel` → `promotable` (evidence `evidence_fresh`, rollback `rollback_target_pinned`, banner `clear`)
    - `5.2.0-rc.2` on `stable_channel` → `blocked_hard_blocker` (evidence `evidence_fresh`, rollback `rollback_target_pinned`, banner `hard_blocker_open`)
- **Update-Center Card**: `stable`
  - Owner: Update-center card owner
  - Scope: The update-center card renders the shared candidate primitive so a single-family candidate whose qualification evidence has gone stale reads as blocked-stale-evidence with a refresh-evidence next action, while a candidate missing required evidence reads as blocked-missing-evidence with a provide-evidence next action
  - Worked resolutions: 2
    - `5.1.5-rc.1` on `beta_channel` → `blocked_stale_evidence` (evidence `evidence_stale`, rollback `rollback_target_pinned`, banner `evidence_stale`)
    - `5.1.5-rc.2` on `beta_channel` → `blocked_missing_evidence` (evidence `evidence_missing`, rollback `rollback_target_pinned`, banner `evidence_missing`)
- **CLI Release Inspect**: `stable`
  - Owner: CLI release-inspect owner
  - Scope: The CLI release-inspect surface renders the shared candidate primitive so a preview candidate that has not yet been evaluated reads as blocked-unknown-state with a run-evaluation next action and no-prior-to-roll-back-to readiness, while a backport candidate whose blocker was resolved reads as narrowed-pending-reverify with a reverify next action
  - Worked resolutions: 2
    - `5.3.0-0.nightly` on `nightly_channel` → `blocked_unknown_state` (evidence `evidence_freshness_unknown`, rollback `no_prior_to_roll_back_to`, banner `candidate_state_unknown`)
    - `5.0.9-rc.1` on `stable_channel` → `narrowed_pending_reverify` (evidence `evidence_fresh`, rollback `rollback_target_pinned`, banner `blocker_pending_reverify`)
- **Admin Release Report**: `stable`
  - Owner: Admin release-report owner
  - Scope: The admin release report renders the shared candidate primitive so an LTS full-train candidate with no pinned rollback target reads as narrowed-rollback-undefined with a define-rollback-target next action rather than inferring a target from the version, while a hotfix candidate held under a disclosed waiver reads as promotable-under-waiver
  - Worked resolutions: 2
    - `4.9.7-rc.1` on `lts_maintenance_channel` → `narrowed_rollback_undefined` (evidence `evidence_fresh`, rollback `rollback_target_undefined`, banner `rollback_target_undefined`)
    - `5.1.4-hotfix.1` on `stable_channel` → `promotable_under_waiver` (evidence `evidence_fresh`, rollback `rollback_target_pinned`, banner `clear`)
- **Support / Evaluation Export**: `stable`
  - Owner: Support / evaluation export owner
  - Scope: The support / evaluation export renders the shared candidate primitive so a candidate whose evidence is aging reads as promotable-with-reservations rather than clean or blocked, and a preview candidate with only soft blockers reads as promotable-with-reservations — the same candidate/blocker vocabulary a support or evaluation reviewer reads elsewhere
  - Worked resolutions: 2
    - `5.2.0-rc.3` on `beta_channel` → `promotable_with_reservations` (evidence `evidence_aging`, rollback `rollback_target_pinned`, banner `clear`)
    - `5.3.0-rc.1` on `preview_channel` → `promotable_with_reservations` (evidence `evidence_fresh`, rollback `rollback_target_pinned`, banner `clear`)
