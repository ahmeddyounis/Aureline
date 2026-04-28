# Decision-rights and signoff matrix

This document is the human-readable companion to the role-based
decision-rights matrix in
[`/artifacts/governance/signoff_matrix.yaml`](../../artifacts/governance/signoff_matrix.yaml)
and the row-id registry in
[`/artifacts/governance/promotion_decision_rows.yaml`](../../artifacts/governance/promotion_decision_rows.yaml).
It freezes who is accountable, who must concur, which packet must
exist, and what evidence bundle must be cited before a launch-bearing
decision can widen scope, claims, or release posture.

Companion artifacts:

- [`/artifacts/governance/signoff_matrix.yaml`](../../artifacts/governance/signoff_matrix.yaml)
  — authoritative machine-readable matrix.
- [`/artifacts/governance/promotion_decision_rows.yaml`](../../artifacts/governance/promotion_decision_rows.yaml)
  — stable row ids, packet-id fields, status vocabulary, and
  reconstruction joins for scorecards, shiproom, release packets, and
  operational-readiness reviews.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — active DRI and waiver resolution. This document stays role-based;
  the ownership matrix resolves roles to people.
- [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  and [`./forum_packet_templates.md`](./forum_packet_templates.md) —
  packet profiles and forum routing.
- [`../release/shiproom_runbook.md`](../release/shiproom_runbook.md)
  and
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  — release review order and stable-facing promotion gates.

## Operating rules

1. **Packet first.** A launch-bearing decision is not closed until the
   required packet type exists and names a `packet_id`, a matrix
   `decision_class_id`, a `signoff_row_ref`, and one or more
   `evidence_bundle_ids`.
2. **Role-based matrix, explicit owner resolution.** Matrix rows name
   roles and forums, not contact details. A scorecard or packet resolves
   the active owner through the ownership matrix. If the role cannot
   resolve to an active owner, the decision is degraded and not
   approvable.
3. **Conservative path wins.** Shared ownership and reviewer
   disagreement hold, narrow, downgrade, or block the decision. They do
   not silently widen public commitments.
4. **Temporary handoffs expire closed.** An unresolved owner or expired
   temporary handoff renders `degraded_owner_unresolved` or
   `degraded_handoff_expired`; it must not appear equivalent to an
   actively owned approved row.
5. **Reconstructability is required.** A waiver or promotion must be
   reconstructable from its packet id, evidence bundle ids, signoff row
   ref, owner roles, concurrence roles, timestamp, and outcome rationale.

## Matrix

| Decision class | Accountable owner roles | Required concurrence | Required packet | Conservative path |
|---|---|---|---|---|
| `architecture_freeze` | `chief_architect`, `architecture_council` | `product_lead`, `performance_owner`, `release_owner` | `architecture_freeze_packet` | Keep affected interfaces provisional or blocked until the freeze packet and all concurrence are present. |
| `protected_metric_waiver` | `performance_owner` | `architecture_council`, `product_lead`; repeated waivers also escalate to sponsor / shiproom review | `protected_metric_waiver_packet` | Keep the existing threshold and block claim widening unless the waiver is packeted with expiry and mitigation. |
| `discipline_waiver` | the relevant `security_lead`, `reliability_owner`, `accessibility_owner`, or `trust_owner` | `product_lead`, `release_owner`; release-train-crossing waivers also escalate to sponsor / shiproom review | `discipline_waiver_packet` | Reject or hold the waiver and narrow the affected claim. |
| `stable_cutline_change` | `product_lead` | `sponsor`, `chief_architect`, `release_owner` | `stable_cutline_change_packet` | Keep or narrow the v1.0 cutline; late broadening needs full concurrence. |
| `support_class_or_archetype_claim_publication` | `product_lead`, `certification_qe_owner` | `release_owner`, `docs_lead`, `affected_subsystem_lead` | `claim_publication_packet` | Publish only the strongest claim class all required roles can defend from current evidence. |
| `stable_release_promotion` | `release_owner` | `performance_owner`, `security_lead`, `accessibility_owner`, `supportability_owner`, `certification_qe_owner`, `docs_lead` | `stable_release_promotion_packet` | No-go or hold. Stable-facing candidates do not ship on late-proof placeholders or active release waivers. |
| `lts_line_creation` | `release_owner`, `enterprise_support_owner` | `sponsor`, `security_lead`, `product_lead` | `lts_line_creation_packet` | Do not create or advertise the LTS line until backport, security, and support-window ownership are packeted. |
| `workflow_bundle_certification_or_downgrade` | `compatibility_ecosystem_review_owner` | `certification_qe_owner`, `docs_release_owner` | `workflow_bundle_decision_packet` | Downgrade, retest, or withhold the bundle claim until evidence, docs, and release posture agree. |

## Evidence bundles

The matrix uses semantic evidence bundle ids so packets can join
different artifact families without inventing local names:

| Evidence bundle id | Covers |
|---|---|
| `evidence.bundle.architecture.freeze` | Interface-freeze rows, decision index rows, ADRs, and the freeze narrative. |
| `evidence.bundle.protected_metric.waiver` | Protected metrics, fitness functions, latency budgets, and benchmark-publication packet inputs. |
| `evidence.bundle.discipline.waiver` | Requirement rows, waiver packet shape, requirement lifecycle state, and discipline-specific mitigation evidence. |
| `evidence.bundle.cutline.scope` | Commitment classes, rebaseline policy, launch decision rows, and claim-manifest impact. |
| `evidence.bundle.claim.publication` | Assurance-claim rows, claim parity, workflow-bundle ids, public-proof rows, and known-limit refs. |
| `evidence.bundle.stable.promotion` | Release packet, promotion gates, ring history, signing quorum, and shiproom review order. |
| `evidence.bundle.lts_line.creation` | Channel posture, support-window and backport rules, release-family versioning, and LTS-line evidence. |
| `evidence.bundle.workflow_bundle.decision` | Workflow-bundle ids, scoreboard families, public-proof packet schema, and claim rows. |

## How packets cite the matrix

Every packet that closes one of these decisions should carry:

- `decision_row_id` from
  [`promotion_decision_rows.yaml`](../../artifacts/governance/promotion_decision_rows.yaml);
- `decision_class_id` from
  [`signoff_matrix.yaml`](../../artifacts/governance/signoff_matrix.yaml);
- `packet_id` using the row's packet-id field and shape;
- `signoff_row_ref` pointing back to the matrix row;
- `accountable_owner_role_ids` and `concurrence_role_ids`;
- `evidence_bundle_ids` plus concrete artifact refs or evidence ids;
- `decision_status` from the closed vocabulary; and
- `decided_at_utc` or `reviewed_at_utc` plus outcome rationale.

Scorecards and shiproom packets should render the row id and closed
status vocabulary directly. They should not write statuses such as
"probably ok", "waiting on release", or "approved in meeting" when a
closed status exists.

## Degraded states

Two degraded states are explicit:

- `degraded_owner_unresolved` — the accountable role or required
  concurrence role cannot resolve to an active owner through the
  ownership matrix.
- `degraded_handoff_expired` — a temporary handoff or waiver-backed
  owner substitution has expired.

Either state blocks approval. The next packet must restore ownership,
renew or close the waiver, or narrow the claim. A degraded row may stay
visible in scorecards and release packets, but it must render as
degraded and cannot be treated as approved or approved with narrowing.

## Escalation

Escalation is also role-based. If required concurrence fails:

- Architecture and protected-metric rows escalate through architecture,
  release, and shiproom review, with scope held or thresholds preserved.
- Discipline waiver rows escalate through release and shiproom review;
  release-train-crossing waivers require sponsor review.
- Cutline and claim-publication rows escalate through product scope,
  release, compatibility, and shiproom review as applicable.
- Stable promotion and LTS-line rows escalate to release council
  and shiproom review and default to no-go or rejection.
- Workflow-bundle rows escalate through compatibility, release, and
  product-scope review and default to downgrade, retest, or withheld
  certification.
