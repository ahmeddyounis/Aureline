# Proof packet: external alpha design-partner intake

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:24:31Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_design_partner_truth_set@2026-05-15
validator: ci/check_design_partner_alpha.py
validation_capture: artifacts/milestones/m2/captures/design_partner_intake_validation_capture.json
claim_change_state: no_claim_widening
```

Entry page: `docs/alpha/design_partner_guide.md`
Intake packet: `artifacts/milestones/m2/design_partner_intake_packet.md`
Task pack: `artifacts/milestones/m2/design_partner_task_pack.md`
Feedback taxonomy: `artifacts/feedback/design_partner_feedback_taxonomy.yaml`
Known limits: `artifacts/feedback/external_alpha_known_limits.md`
Protected feedback cases: `fixtures/feedback/external_alpha_cases/`
Validator: `ci/check_design_partner_alpha.py`
Latest capture: `artifacts/milestones/m2/captures/design_partner_intake_validation_capture.json`

This packet anchors the external alpha onboarding path. It proves that the
partner guide, intake packet, task pack, feedback taxonomy, known-limits packet,
and protected feedback cases resolve to the same alpha scope matrix and
scoreboard rows.

## Protected Proof Path

Run:

`python3 ci/check_design_partner_alpha.py --repo-root . --report artifacts/milestones/m2/captures/design_partner_intake_validation_capture.json`

The validator checks that:

- the guide, intake packet, task pack, taxonomy, and known-limits packet all
  cite the canonical alpha scope and upstream intake artifacts;
- the task pack covers every protected workflow and fixture named by the alpha
  wedge matrix;
- feedback categories route to known issue-route classes and scoreboard rows;
- known-limit categories cite the external alpha known-limits packet; and
- protected feedback fixtures cover task completion, redaction-blocked sharing,
  and known-limit routing for the claimed wedges.
