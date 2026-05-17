# M3 design-partner and managed-pilot beta pack

This pack is the reviewer-facing entrypoint for design partners and
managed-pilot organizations onboarding into the M3 beta admission lane.
It binds cohort guardrails, scorecards, known-limits vocabulary,
rollback and support-export rules, and escalation lanes into one
artifact that reads from the same canonical truth as the rest of the
M3 beta program.

The pack is governed. The canonical machine source is
`artifacts/milestones/m3/beta_enablement_starter_pack.yaml`; the
validator is `ci/check_m3_beta_enablement_starter_pack.py`. When the
pack and the canonical source disagree, the canonical source wins and
this doc MUST be updated in the same change set.

- Lane id: `starter_pack_lane:design_partner`
- Primary cohort: `cohort:design_partner_managed_pilot`
- Primary beta surfaces:
  - `beta_surface:packaging_update_rollback`
  - `beta_surface:policy_proxy_transport`
  - `beta_surface:support_export_diagnostics`
  - `beta_surface:importer_and_migration`
- Secondary cohorts: `cohort:external_alpha_migration`,
  `cohort:certified_archetype`
- Secondary surface: `beta_surface:compatibility_publication`

## How to use this pack

1. Read the
   [beta admission matrix](../../milestones/m3/beta_admission_matrix.md)
   for the canonical claim surface and cohort vocabulary.
2. Confirm the design-partner agreement, declared deployment profile,
   support-export redaction review, and a policy bundle or proxy
   envelope are in place before intake.
3. Read the cohort scorecards below to know the current effective
   support class and any open waivers.
4. Run the rollback and support-export drills called out in
   [`docs/release/update_and_rollback_contract.md`](../../release/update_and_rollback_contract.md)
   on partner hardware before a beta build is asked to carry partner
   workload.
5. File partner-class reports through the private routes named in
   [`docs/community/m3/issue_rfc_routing_beta.md`](../../community/m3/issue_rfc_routing_beta.md).

## Cohort guardrails

The cohort guardrails describe intake requirements, minimum evidence
classes, graduation criteria, downgrade rules, and exit paths.
Tooling reads
`artifacts/milestones/m3/cohort_guardrails.yaml`; this pack does not
restate them.

- Cohort guardrails (canonical):
  [`artifacts/milestones/m3/cohort_guardrails.yaml`](../../../artifacts/milestones/m3/cohort_guardrails.yaml)

## Scorecards

The design-partner cohort is tracked through two scorecards (partner
and managed pilot). Each scorecard fires downgrade rules automatically
through the scorecard validator.

- Design-partner cohort scorecard:
  [`artifacts/milestones/m3/cohorts/design_partner_scorecard.md`](../../../artifacts/milestones/m3/cohorts/design_partner_scorecard.md)
- Managed-pilot cohort scorecard:
  [`artifacts/milestones/m3/cohorts/managed_pilot_scorecard.md`](../../../artifacts/milestones/m3/cohorts/managed_pilot_scorecard.md)
- Scorecard index (downgrade automation rules):
  [`artifacts/milestones/m3/cohorts/scorecard_index.yaml`](../../../artifacts/milestones/m3/cohorts/scorecard_index.yaml)

## Upstream design-partner evidence carried into beta

Beta intake inherits the design-partner packet, task pack, feedback
taxonomy, and partner guide from the alpha train. They remain
authoritative until the corresponding beta evidence is current.

- Alpha design-partner intake packet:
  [`artifacts/milestones/m2/design_partner_intake_packet.md`](../../../artifacts/milestones/m2/design_partner_intake_packet.md)
- Alpha design-partner task pack:
  [`artifacts/milestones/m2/design_partner_task_pack.md`](../../../artifacts/milestones/m2/design_partner_task_pack.md)
- Design-partner feedback taxonomy:
  [`artifacts/feedback/design_partner_feedback_taxonomy.yaml`](../../../artifacts/feedback/design_partner_feedback_taxonomy.yaml)
- Alpha partner guide:
  [`docs/alpha/design_partner_guide.md`](../../alpha/design_partner_guide.md)

## Known-limits vocabulary

Partners read the same known-limits surface as every other M3 lane.
"Missing capability" and "scope expansion" requests are routed through
the same known-limits vocabulary — do not introduce parallel partner-
only scope.

- Cross-milestone known-limits contract:
  [`docs/product/known_limits_contract.md`](../../product/known_limits_contract.md)
- External-alpha known-limits packet (still in scope for beta intake):
  [`artifacts/feedback/external_alpha_known_limits.md`](../../../artifacts/feedback/external_alpha_known_limits.md)

## Rollback, support export, and offboarding

Partner trust depends on these drills being reproducible on partner
hardware. The contracts below are the canonical surfaces tooling
audits.

- Update and rollback contract:
  [`docs/release/update_and_rollback_contract.md`](../../release/update_and_rollback_contract.md)
- Usage-export and offboarding contract:
  [`docs/governance/usage_export_and_offboarding_contract.md`](../../governance/usage_export_and_offboarding_contract.md)

### Automatic downgrade triggers

The cohort scorecard fires these triggers automatically; tooling
applies them without reviewer interpretation.

| Trigger | Auto-state | Effect on the lane |
|---|---|---|
| Support-bundle redaction break | `retest_pending` | Support export drill must pass redaction review before partner workload routes through the bundle. |
| Rollback drill failure on partner hardware | `retest_pending` | Beta packaging cannot widen until the drill passes; partner-channel rollback is the recovery path. |
| Enterprise proxy lab unavailable | `limited` | Policy / proxy / transport claims narrow until the lab is reachable; partner pilots see a `limited` chip. |
| Evidence past freshness window | `evidence_stale` | Scorecards, claim packets, and Help/About copy render as `evidence_stale` until refreshed. |

## Compatibility surface

Partners read the same matrix-derived compatibility report as the rest
of the program. Do not invent "verified" or "stable" copy.

- Generated compatibility report (markdown):
  [`artifacts/compat/m3/compatibility_report.md`](../../../artifacts/compat/m3/compatibility_report.md)
- Certified-archetype scorecard index:
  [`artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml`](../../../artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml)
- Public-proof review packet template:
  [`artifacts/milestones/m3/review_packet_template.md`](../../../artifacts/milestones/m3/review_packet_template.md)

## Issue routing and disclosure posture

Design partners and managed pilots route most reports through private
channels by default. Only the canonical routes below are authorized;
file through the matrix in
`artifacts/governance/issue_routing.yaml` rather than minting partner-
only escalation paths.

| Issue class | Default route | Disclosure expectation |
|---|---|---|
| `private_partner_case` | Private partner channel | Sanitised public summary on close, when applicable |
| `design_partner_case` | Private partner channel | Sanitised public summary on close, when applicable |
| `supportability_escalation` | Private support channel | Private indefinite |
| `compatibility_regression` | Public issue tracker | Required public summary |
| `security_issue` | Private security channel (see SECURITY.md) | Public advisory at disclosure |
| `waiver_request` | Governance packet queue | Required |

Any escalation that would move a partner case from a private route to
a public route MUST cite a named disclosure transition row in
`artifacts/governance/issue_routing.yaml` (for example,
`private_partner_to_public_sanitised_summary`). Do not cross-post a
partner artefact into a public issue without the transition.

## Escalation

When a drill, scorecard, or compatibility row blocks the lane and the
trigger is not in the table above:

- Owner handoff (intake, triage, release-council escalation): see the
  cohort scorecard above.
- Decision rights:
  [`docs/governance/decision_rights_and_signoff_matrix.md`](../../governance/decision_rights_and_signoff_matrix.md)
- Routing matrix:
  [`docs/governance/issue_routing_matrix.md`](../../governance/issue_routing_matrix.md)

## How to verify

This pack is governed by `ci/check_m3_beta_enablement_starter_pack.py`.
Run the validator and refresh the capture in the same change set when
any cohort binding, surface binding, scorecard ref, or issue-routing
reference changes:

```
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root .
```

Use `--check` in CI to fail when the capture on disk would drift:

```
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root . --check
```
