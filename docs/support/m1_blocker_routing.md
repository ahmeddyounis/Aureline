# Blocker routing loop (internal dogfood)

This document defines how internal dogfood feedback becomes a routed blocker or
known gap. It is written so participants can file structured feedback quickly
and triage can reliably land issues on the correct class and escalation path.

Canonical truth sources:

- Feedback intake taxonomy: `artifacts/dogfood/feedback_taxonomy.yaml`
- Blocker taxonomy: `artifacts/milestones/m1/blocker_taxonomy.yaml`
- Known-gaps ledger: `artifacts/milestones/m1/known_gaps_ledger.yaml`
- Dogfood issue fields and evidence requirements: `docs/governance/dogfood_issue_taxonomy.md`

## Routing goals

- Avoid “chat-only” feedback: every blocker has a structured record.
- Keep one vocabulary: dogfood feedback categories map to blocker classes.
- Make escalation windows explicit so blockers do not silently age.

## Triage steps (per incoming report)

1. Verify the report includes the required fields from
   `artifacts/dogfood/feedback_taxonomy.yaml` (and uses the field names from
   `docs/governance/dogfood_issue_taxonomy.md`).
2. Classify the feedback category using `artifacts/dogfood/feedback_taxonomy.yaml`.
3. Map the category to the canonical blocker class in
   `artifacts/milestones/m1/blocker_taxonomy.yaml`.
4. If the report blocks the protected dogfood lane:
   - translate issue severity → blocker severity using the crosswalk in
     `artifacts/dogfood/feedback_taxonomy.yaml`
   - record (or update) a row in `artifacts/milestones/m1/known_gaps_ledger.yaml`
     with owner DRI, next action, fixture ref, and exact build identity ref
5. Escalate based on the blocker class SLA (see
   `artifacts/milestones/m1/blocker_taxonomy.yaml`):
   - hot path, fidelity, recovery, trust: default escalation within 48 hours
   - onboarding, boundary truth, public proof: default escalation within 72 hours

## Failure drill (misclassification)

To confirm the loop is resilient to human error:

1. Take a real report and intentionally misclassify it at intake.
2. During triage, correct the classification using the feedback taxonomy.
3. Confirm the corrected blocker class and escalation path are reflected in:
   - the issue labels/fields, and
   - the corresponding known-gaps ledger row when it is a daily blocker.

