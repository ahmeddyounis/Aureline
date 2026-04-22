# Blocker-aging SLAs

This document is the canonical aging and escalation table for the
program blockers that can otherwise hide in issue notes, scorecard
comments, or meeting memory.

Companion artifacts:

- [`/artifacts/governance/dependency_ledger.yaml`](../../artifacts/governance/dependency_ledger.yaml)
  — stable blocker and dependency ids, latest safe decision points, and
  fallback posture per dependency row.
- [`/artifacts/governance/correction_trigger_table.yaml`](../../artifacts/governance/correction_trigger_table.yaml)
  — current scorecard and risk-linked triggers for descoping,
  rebaseline, or exception-packet escalation.
- [`/docs/governance/dri_map.md`](./dri_map.md)
  — authority and lane ownership.
- [`/docs/governance/evidence_freshness_policy.md`](./evidence_freshness_policy.md)
  — stale-proof downgrade rules.
- [`/docs/governance/commitment_and_rebaseline_policy.md`](./commitment_and_rebaseline_policy.md)
  — response windows and program-level correction rules.

If these artifacts disagree, the dependency ledger wins for ids and
latest safe decision points, the correction-trigger table wins for the
required correction action, and this document wins for the aging clock
and escalation timing.

Business hours are used unless a row says otherwise. Stale evidence
downgrades scorecards immediately under the freshness policy; the SLAs
below govern how long the block may remain unowned or uncontained, not
whether the lane may stay green.

## Summary Table

| Blocker class | Clock starts when | Acknowledge by | Resolution plan by | Default containment if still open |
|---|---|---|---|---|
| **Architecture-freeze blocker** | An open dependency row blocks a protected lane or committed M1 workstream. | 24 h | 48 h | Narrow the affected lane or freeze scope at the last safe decision point. |
| **Stale evidence** | A required packet expires or a rerun trigger fires. | 24 h | 48 h | Downgrade the scorecard immediately and hold claims to the last proven scope. |
| **Owner gap** | A protected lane, release blocker, or active incident has no named owner, backup, or active waiver. | 4 h for active incidents; 24 h otherwise | 24 h | Freeze the dependent lane or release path until a named owner or waiver exists. |
| **Unresolved waiver** | A waiver expires, or a second renewal request is opened on the same protected path. | 0 h at expiry; same day on renewal | 24 h | Freeze the lane, narrow the claim, or route through the correction-trigger table. |

## Architecture-Freeze Blockers

- A protected blocker must have a named DRI and a visible dependency row
  on the same business day it becomes real.
- Within 24 business hours, the DRI must acknowledge the blocker in the
  dependency ledger, scorecard, backlog row, or all three.
- Within 48 business hours, the DRI must record a resolution plan with a
  next artifact, an owner, and the fallback posture if the dependency
  stays open.
- Once the blocker crosses its `latest_safe_decision_point`, the default
  state is containment, not optimism. The lane narrows or the widening
  freezes until the blocker closes.

Escalation path:

1. Lane DRI.
2. Architecture council.
3. Product scope review if committed scope or dates are at risk.
4. Release council if milestone dates, acceptance thresholds, or public
   claims need to move.

## Stale Evidence

- The scorecard or claim row downgrades as soon as proof is stale.
- Within 24 business hours, the evidence owner must either take the
  rerun or assign it explicitly.
- Within 48 business hours, one of two things must be true:
  the proof has a scheduled refresh with owner and date, or the claim
  and scorecard have already narrowed to the last truthful scope.
- If stale proof is still blocking the same lane at the next checkpoint,
  use the correction-trigger table rather than reopening the same debate
  in free text.

Escalation path:

1. Evidence owner.
2. Owning lane DRI.
3. Docs/public-truth or supportability owner when the stale proof is
   claim-bearing.
4. Release council or shiproom on release-grade or stable-facing rows.

## Owner Gaps

- A protected lane may not sit without a named owner, backup owner, or
  active waiver past the first business day.
- Active support or release incidents use the tighter 4-hour
  acknowledgment floor because they block operators immediately.
- Missing owner metadata is a freeze condition for dependent work. It is
  not a warning-only hygiene issue.

Escalation path:

1. Owning lane DRI if present, otherwise governance-packets DRI.
2. Architecture council.
3. Release council for release-blocking or public-claim lanes.

## Unresolved Waivers

- Waiver expiry is an immediate containment event. At expiry, the lane
  freezes or the claim narrows unless the waiver is renewed or closed in
  the same review window.
- A second renewal on the same protected path is not ordinary upkeep.
  It routes through the correction-trigger table and must result in
  descoping, rebaseline, or explicit correction work.
- A waiver may not remain in effect only because the team has not
  revisited it yet. Renewal requires a current reason, expiry, and
  escalation path.

Escalation path:

1. Waiver owner and lane DRI.
2. The forum that approved the waiver class.
3. Architecture council.
4. Release council when the waiver touches milestone-close or
   stable-facing truth.

## Current Solo-Maintainer Path

The repository is still in a solo-maintainer posture. Until the backup
waiver closes, the concrete escalation route for every class above is:

1. self-escalation entry in the shiproom or milestone packet;
2. contributor-community thread; and
3. public repository notice if the blocker remains unresolved.

That path is intentionally weaker than a staffed multi-person route.
The weakness is part of the visible program risk, not a reason to relax
the SLA.
