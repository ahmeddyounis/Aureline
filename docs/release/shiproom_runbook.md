# Shiproom runbook

This runbook is the reviewer-facing operating order for Aureline's
shiproom. It keeps release evidence, supportability evidence, docs
truth, and architecture carry-forward decisions in one control loop
instead of scattered meeting notes.

Companion artifacts:

- `artifacts/release/shiproom_dashboard_seed.yaml`
- `docs/release/release_evidence_packet_template.md`
- `docs/release/release_evidence_packet_example.md`
- `docs/governance/evidence_freshness_policy.md`
- `artifacts/governance/evidence_freshness_slos.yaml`
- `artifacts/governance/evidence_rerun_triggers.yaml`
- `schemas/support/support_packet_index.schema.json`
- `artifacts/governance/claim_manifest_seed.yaml`
- `artifacts/governance/public_truth_parity_matrix.yaml`
- `artifacts/milestones/M0_scorecard.yaml`
- `artifacts/milestones/M0_architecture_pack/coverage_and_freeze_exceptions.yaml`
- `artifacts/governance/decision_index.yaml`
- `artifacts/governance/ownership_matrix.yaml`
- `docs/support/support_center_concept.md`

## Operating rule

Start on the shiproom dashboard. Open the next packet only when the
current panel is yellow or red. If a reviewer cannot point to a stable
requirement id, claim row, exact-build identity, waiver id, or freeze
exception id, the item is not promotion-grade proof.

## Evidence review order

1. Candidate envelope and rollback floor

- Confirm `exact_build_identity_ref` values, release channel, deployment
  profile scope, and rollback target or rollback evidence.
- Stop immediately if the candidate cannot show one coordinated build
  identity set, one rollback path, or one release-packet home.

2. Readiness scorecard and owner coverage

- Read the current explicit calls from the scorecard and the milestone
  signoff packet.
- Verify every protected lane still resolves to a DRI and every missing
  backup resolves through an active waiver.

3. Waivers, freeze exceptions, and aging

- Review every active waiver ref, backup waiver, and open
  `freeze_exception_id`.
- An expired waiver or a repeated exception on the same protected path is
  a `no_go` until the work narrows the claim or rebaselines the path.

4. Evidence freshness and missing proof

- Review `captured_at`, `stale_after`, `generated_at`, and any named
  `trigger_revision` mismatches against
  `artifacts/governance/evidence_freshness_slos.yaml` and
  `artifacts/governance/evidence_rerun_triggers.yaml`, plus any named
  missing packet refs.
- Treat stale or missing required proof as `hold_for_refresh` at
  minimum; treat stale exact-build, rollback, security, or public-proof
  evidence as `no_go`.

5. Claim-manifest and channel parity

- Read `effective_claim_posture` before reading any channel copy.
- Verify docs, Help/About, service health, support export, and release
  packet surfaces do not widen above the claim row and keep the same
  `known_limit_refs`.

6. Support packet family coverage

- Review the canonical support packet families in
  `schemas/support/support_packet_index.schema.json`.
- Confirm the candidate can answer exact-build support, route/origin
  reconstruction, known-limit correlation, security triage, and
  rollback review without inventing packet-local ids.

7. Security and incident posture

- Review incident-workspace, advisory, revocation, and redaction/export
  posture when the candidate touches trust, auth, policy, install, or
  networked lanes.
- No high-risk issue may leave shiproom with an unclear handoff route or
  export posture.

8. Final call and owner handoff

- Record one shiproom status, one next action, and the receiving owners
  before closing the review.
- Any unresolved yellow or red item must name the destination owner, due
  packet, and disclosure or narrowing action.

## Go/no-go statuses

| Shiproom status | When to use it | Required follow-through |
|---|---|---|
| `go` | release packet is `releasable`, required evidence is current, no red blocker remains, and owner handoffs are closed | publish or promote using the reviewed exact-build set |
| `go_with_narrowing` | promotion is acceptable only with the already-recorded `preview_only` or `narrow_claims` posture | keep downgraded claim rows, known-limit refs, and release-note disclosures visible |
| `hold_for_refresh` | proof is missing, stale, or incomplete, but the candidate can become reviewable without reopening scope | refresh the named packet or evidence row before the next shiproom slot |
| `no_go` | blocked by expired waiver, unresolved red risk, unsafe rollback/security/support posture, or a claim that cannot be narrowed honestly | stop promotion; open correction, rebaseline, or explicit downgrade work |

Status mapping:

- `go` implies the release packet can stay `releasable`.
- `go_with_narrowing` implies the release packet stays `preview_only`
  or `narrow_claims`.
- `hold_for_refresh` maps to `in_review` or `needs_review`.
- `no_go` maps to `blocked`.

## Support packet family map

These are the canonical support packet families shiproom and support
review must use:

| Family class | Primary job | Minimum linkage |
|---|---|---|
| `exact_build_support` | prove which coordinated build, symbolication posture, docs/help revision, and release packet the user is on | `exact_build_identity_ref`, `release_evidence_packet_ref`, `claim_row_refs` |
| `route_origin_reconstruction` | reconstruct what command or routed action ran where and under which authority path | `command_id`, `invocation_session_id`, `action_origin_class`, `action_target_class`, `action_route_class`, `action_exposure_class`, `exact_build_identity_ref` |
| `known_limit_correlation` | connect the issue to current docs/help truth, known limits, and support-window posture | `claim_row_refs`, `known_limit_refs`, `docs_pack_ref` or `version_match_state`, `exact_build_identity_ref` |
| `security_triage` | hand off trust-sensitive or security-sensitive cases without losing build or export truth | `incident_workspace_packet_ref`, `support_bundle_ref`, `exact_build_identity_ref`, `claim_row_refs` when a marketed row is affected |
| `rollback_review` | review install or state rollback safety before advising recovery or promotion rollback | `rollback_target_ref`, `rollback_evidence_ref`, `restore_provenance_ref`, `exact_build_identity_ref` |

The machine-readable contract for those families is
`schemas/support/support_packet_index.schema.json`.

## Exception logging

Every shiproom session appends one decision log entry. The log may live
in the shiproom packet or the candidate's release-evidence packet, but
it must carry stable ids.

Minimum fields:

- `reviewed_at_utc`
- `shiproom_status`
- `candidate_ref` or `packet_id`
- `exact_build_identity_refs`
- `blocking_requirement_ids`
- `blocking_claim_row_refs`
- `waiver_refs`
- `freeze_exception_ids`
- `next_action`
- `receiving_owner`
- `due_ref` or `due_date`
- `public_disclosure_required`

Rules:

- expired waivers auto-convert the session to `no_go` until they are
  renewed or closed;
- the second waiver or freeze exception on the same protected path must
  log `claim_narrowing` or `rebaseline`, never silent carry-forward;
- support-side exceptions must cite the impacted support packet family
  class; and
- a `go_with_narrowing` call must name the exact downgraded claim rows
  and their disclosure surfaces.

Example:

```yaml
reviewed_at_utc: 2026-04-21T00:00:00Z
shiproom_status: hold_for_refresh
candidate_ref: release-evidence.seed.current-repository-baseline
exact_build_identity_refs:
  - build-id:aureline:stable:0.7.3:x86_64-unknown-linux-gnu:release:a4d1c3f0e27b
blocking_claim_row_refs:
  - claim_row:build.exact_build_identity
  - claim_row:benchmark.publication_truth
waiver_refs:
  - single-maintainer-backup
freeze_exception_ids:
  - FE-014
next_action: Refresh benchmark proof and the exact-build release packet before widening support or public-proof language.
receiving_owner: "@ahmeddyounis"
public_disclosure_required: true
```

## Handoff expectations

Release owner

- Carry `exact_build_identity_ref` values, release channel/profile
  scope, rollback target or waiver, and the final shiproom status into
  the promotion packet.
- If the call is not `go`, carry the blocking packet refs and next
  refresh date.

Support owner

- Confirm which support packet family covers the case and which required
  refs are still missing.
- Preserve exact-build, route/origin, claim-row, and known-limit ids in
  any support export or field handoff.

Docs owner

- Propagate the `effective_claim_posture`, `known_limit_refs`,
  migration-note refs, and docs/help version-match state without
  rewriting the downgrade reason.
- If copy narrows, update release notes and docs/help surfaces in the
  same change or packet refresh.

Security owner

- Route trust-sensitive cases through the incident-workspace or
  advisory path, preserve export routing, and keep redaction posture
  explicit.
- If a security or policy blocker exists, the shiproom call stays
  `hold_for_refresh` or `no_go` until the handoff packet exists.

Architecture owner

- Carry forward open decisions, assumptions, dependencies, and
  `freeze_exception_id` values into the next scope review or rebaseline
  packet.
- When repeated exceptions appear, open correction or rebaseline work
  instead of leaving the dashboard yellow by habit.

## What shiproom does not do

- invent new requirement ids, claim rows, or support packet ids;
- accept a green summary when the dashboard still shows stale or missing
  proof;
- hide scope cuts inside verbal "known issue" language without the claim
  row and `known_limit_refs` being updated; or
- treat support exports as equivalent to telemetry or background uploads.
