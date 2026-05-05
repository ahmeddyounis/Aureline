# Shiproom dashboard contract

This document freezes the daily decision surface Aureline shiproom uses
to judge readiness, blockers, stale evidence, and rollback posture for
beta widening, stable promotion, and milestone close. It is a contract
on what the dashboard MUST display, where each panel reads from, what
freshness rules govern panel data, what degraded states are admissible,
and how panel state feeds the promotion checklist.

It is not a UI specification and not an integration with any release
tool. It describes the panel contract and the artifacts that back it,
so the same dashboard call can be reconstructed from the underlying
packets without privileged tribal knowledge.

Companion artifacts:

- [`/schemas/release/shiproom_panel.schema.json`](../../schemas/release/shiproom_panel.schema.json)
  - boundary schema for `shiproom_panel_record` panels and their
    source-ref, freshness, row, and degraded-state vocabularies.
- [`/artifacts/release/shiproom_alert_thresholds.yaml`](../../artifacts/release/shiproom_alert_thresholds.yaml)
  - machine-readable daily alert thresholds with named default actions.
- [`/artifacts/release/promotion_checklist.yaml`](../../artifacts/release/promotion_checklist.yaml)
  - minimum yes/no answers required before widening a channel or
    closing a milestone.
- [`/fixtures/release/shiproom_dashboard_cases/`](../../fixtures/release/shiproom_dashboard_cases/)
  - worked panel records, alert firings, and checklist outcomes.
- [`/docs/release/shiproom_runbook.md`](./shiproom_runbook.md)
  - reviewer-facing operating order; this contract is what the
    dashboard panels must show before that runbook executes.
- [`/artifacts/release/shiproom_dashboard_seed.yaml`](../../artifacts/release/shiproom_dashboard_seed.yaml)
  - existing seed dashboard expressed in the legacy ad hoc shape; the
    schema in this contract is what new dashboards conform to.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  - freshness vocabulary and stale-propagation rules quoted by panel
    freshness rows.

If this contract disagrees with the normative source documents
(`.t2/docs/Aureline_PRD.md`, `_Technical_Architecture_Document.md`,
`_Technical_Design_Document.md`, `_UI_UX_Spec_Document.md`,
`_Milestones_Document.md`), the normative sources win and this
contract plus its companion schema, threshold table, checklist, and
fixtures update in the same change.

## Goal

Seed one daily decision surface for beta and stable promotion so the
following can be judged from current controlled data, not detective
work:

- whether the candidate names one coordinated build identity, rollback
  target, and release-evidence packet;
- whether protected paths still resolve to a DRI and current proof;
- whether claim rows, support-window copy, docs/help truth, and
  release-note copy still match the underlying evidence;
- whether stale packets, repeated freeze exceptions, declining
  compatibility, supportability drill regressions, or upgrade/rollback
  failures have crossed an alert threshold;
- whether the promotion checklist returns yes on every minimum row.

## Scope

Frozen here:

- the required panel set the dashboard MUST display every shiproom
  session;
- the closed `panel_class` vocabulary, the required source-ref shape
  per panel class, and the closed degraded-state vocabulary;
- freshness rules for panels, panel rows, and source refs;
- the daily alert threshold register and the closed default-action
  vocabulary;
- the promotion checklist rows that gate beta widening, stable
  widening, and milestone close;
- the audit rule that a stale or missing source visibly degrades a
  panel rather than leaving it green by omission.

Out of scope:

- building the actual dashboard UI;
- integrating with a release tool, CI system, or status page;
- writing release automation, signing services, or update services.

## Invariants

1. **Every panel resolves to current source artifacts.** A panel that
   cannot name at least one current `source_artifact_ref` is rendered
   in a degraded state from the closed vocabulary; it is never rendered
   green.
2. **Stale or missing required source degrades the panel.** When a
   required source ref is past its freshness ceiling or missing, the
   panel reports `panel_red_stale_required_source` or
   `panel_red_missing_required_source`. The panel does not silently
   keep yesterday's call.
3. **Panel state is auditable back to a source.** Every panel row
   names the source artifact path, the row-level evidence refs, and
   the freshness rule the row was checked against.
4. **Alert thresholds carry default actions.** Every threshold row
   names a default action drawn from the closed action vocabulary so a
   firing alert is not just a colour change.
5. **Promotion checklist rows are yes/no.** Each checklist row names
   the panel(s) it reads from, the answer required to proceed, and the
   default action when the answer is no.
6. **Beta and stable have distinct gates.** The checklist makes the
   delta between beta widening and stable widening explicit instead of
   reusing one merged gate.
7. **Milestone close consumes the same panels.** Closing a milestone
   reuses the dashboard panels and the checklist; it does not invent
   a parallel sign-off form.
8. **Privileged tribal knowledge is non-conforming.** A reviewer who
   only has the dashboard, the schema, and the linked source artifacts
   must be able to justify the call. If a panel relies on memory of
   prior calls, it is non-conforming.

## Required panel set

Every shiproom dashboard MUST render one panel record per row below.
A missing panel renders the dashboard non-conforming. A panel that
cannot reach its sources renders in a closed degraded state.

| `panel_class` | Question the panel answers | Required `source_artifact_ref` classes |
|---|---|---|
| `build_identity` | Does the candidate name one coordinated exact-build set, release-evidence packet, rollback atom, and ring-history packet? | `exact_build_identity_set`, `release_evidence_packet`, `artifact_family_map`, `ring_history_packet` |
| `readiness_scorecard` | Do protected lanes still resolve to a DRI and current proof? | `milestone_scorecard`, `ownership_matrix`, `freeze_exception_catalog` |
| `protected_path_health` | Are signing, attestation, transparency-log, mirror, and rollback paths current? | `artifact_family_map`, `trust_framework_rows`, `pipeline_lane_rules`, `update_manifest_or_rollback_evidence` |
| `certified_archetype_or_compatibility` | Are certified archetypes and compatibility reports current and passing? | `certified_archetype_report`, `compatibility_report`, `claim_manifest`, `version_skew_register` |
| `support_center_readiness` | Are support packet families covered with current contract refs and bundle linkage? | `support_packet_index`, `support_evidence_pack_matrix`, `deployment_drill_catalog` |
| `transport_and_policy_posture` | Are signing, transparency, revocation, mirror trust, and policy bundles in their expected posture? | `trust_framework_rows`, `cache_trust_classes`, `pipeline_lane_rules`, `policy_bundle_or_revocation_manifest` |
| `migration_and_interface_health` | Do migration and interface contracts still match the candidate's surface area? | `version_skew_register`, `compatibility_report`, `migration_packet_or_release_notice`, `claim_manifest` |
| `docs_and_claim_parity` | Do docs, Help/About, release notes, and known-limit copy match the claim manifest? | `claim_manifest`, `public_truth_parity_matrix`, `docs_pack_or_help_about`, `release_notice` |
| `correction_or_backport_posture` | Is any open correction line or backport lane staffed with named owners and freshness? | `channel_matrix_backport_lane_rows`, `correction_trigger_table`, `claim_manifest`, `release_notice` |
| `open_blockers` | Which blockers, waivers, freeze exceptions, or assumptions are unresolved beyond their thresholds? | `decision_index`, `ownership_matrix`, `freeze_exception_catalog`, `blocker_aging_slas` |

A dashboard MAY render additional panels for context, but those panels
do not feed the promotion checklist unless they are listed in the
checklist's `panel_class_refs`.

## Panel record shape

The boundary schema for each panel is in
[`/schemas/release/shiproom_panel.schema.json`](../../schemas/release/shiproom_panel.schema.json).
Every panel record carries:

- `record_kind: shiproom_panel_record` and `schema_version: 1`;
- `panel_id`, `panel_class`, `title`;
- `panel_status` from a closed vocabulary covering green, yellow with
  named explanation, red blocking, red missing required source, red
  stale required source, red repeated freeze exception, and yellow
  with active waiver;
- `freshness` with `freshness_class`, `captured_at`, `stale_after`,
  `source_revision`, and `freshness_policy_ref`;
- `source_refs`, where each entry names `source_artifact_ref`,
  `source_role_class`, `required` flag, `freshness_rule_ref`, and
  `missing_disposition_class`;
- `rows` of panel-class-specific evidence rows with their own
  status, evidence refs, and summary;
- `promotion_decision_inputs` declaring whether the panel feeds beta
  widening, stable widening, milestone close, or correction-line
  staffing;
- `audit_back_to_source` declaring `auditable: true` plus the source
  ref paths the call rests on;
- optional `alert_threshold_refs` and `promotion_checklist_row_refs`
  pointing at rows in the threshold register and the checklist.

## Freshness rules

Panel freshness uses the existing freshness vocabulary from
[`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md):

- `current` — within the source's stale-after ceiling;
- `current_with_waiver` — past the ceiling but covered by an active
  named waiver row;
- `stale_blocking` — past the ceiling, no waiver, blocks promotion;
- `stale_non_blocking` — past the ceiling, does not block but must be
  refreshed before the next dashboard tick;
- `missing_blocking` — required source ref does not exist;
- `not_applicable` — panel is not applicable to the current candidate
  scope (must be explicit, never silent).

Panel-level freshness is the worst freshness across required
`source_refs`. A panel that calls itself `current` while one required
source is `stale_blocking` is non-conforming.

## Allowed degraded states

`panel_status` is closed:

| `panel_status` | When to use it |
|---|---|
| `panel_green` | All required source refs current, no blocking row, no active waiver. |
| `panel_yellow_explained` | Non-blocking concern recorded with a named row reason and an owner. |
| `panel_yellow_waiver_active` | Promotion-grade only because a named waiver is active and unexpired. |
| `panel_red_blocking` | A row blocks the dashboard call; reviewer cannot proceed without action. |
| `panel_red_missing_required_source` | A required source ref does not exist or could not be resolved. |
| `panel_red_stale_required_source` | A required source ref exists but is past its stale ceiling without a waiver. |
| `panel_red_repeated_freeze_exception` | A freeze exception has reappeared on the same protected path; auto-`no_go` per the runbook. |

A panel MUST NOT report `panel_green` while any required source is
stale, missing, or unowned.

## Alert thresholds

The daily alert thresholds are frozen in
[`/artifacts/release/shiproom_alert_thresholds.yaml`](../../artifacts/release/shiproom_alert_thresholds.yaml).
Every threshold row names:

- `threshold_id`;
- `panel_class_ref` (the panel the threshold reads from);
- `signal_class` (stale packet, blocker age, repeated freeze
  exception, declining compatibility pass rate, supportability drill
  regression, docs/help/About drift, upgrade or rollback failure);
- `threshold_expression` (closed signal-specific shape — count,
  duration, percentage, or boolean);
- `default_action` from the closed action vocabulary:
  `narrow_claim`, `hold_promotion`, `staff_correction_lane`,
  `block_milestone_close`, `refresh_evidence_packet`,
  `rebaseline_decision`;
- `escalation_owner_class`;
- `evidence_refs_required_on_action`.

A firing threshold without a default action is non-conforming. A
default action that does not name an evidence ref the action will
update is non-conforming.

## Promotion checklist

The yes/no checklist is frozen in
[`/artifacts/release/promotion_checklist.yaml`](../../artifacts/release/promotion_checklist.yaml).
Each row names:

- `checklist_row_id`;
- `gate_class` from `beta_widening`, `stable_widening`,
  `milestone_close`;
- `question` in plain language;
- `panel_class_refs` the answer reads from;
- `required_answer` (always `yes` to proceed);
- `default_no_action` from the same default-action vocabulary as the
  threshold register;
- `evidence_ref_examples` to anchor reviewers.

Minimum rows the checklist MUST cover:

- rollback target named and current (uses `build_identity` panel);
- support-window copy current and matching the claim row (uses
  `docs_and_claim_parity` and `support_center_readiness` panels);
- known-limits state attested in claim manifest and release notice
  (uses `docs_and_claim_parity` panel);
- evidence freshness within SLO across all required sources (uses
  every required panel);
- compatibility / certified-archetype reports current (uses
  `certified_archetype_or_compatibility` panel);
- migration and interface health current (uses
  `migration_and_interface_health` panel);
- support packet family coverage current (uses
  `support_center_readiness` panel);
- correction or backport lane staffed when active (uses
  `correction_or_backport_posture` panel);
- transport and policy posture in expected state (uses
  `transport_and_policy_posture` panel);
- no unresolved blockers past their aging SLA (uses `open_blockers`
  panel).

A checklist row that returns `no` without naming a default action is
non-conforming. A checklist that proceeds with any required row at
`no` is non-conforming.

## Cross-surface consumption

Conforming dashboard panels can be consumed by:

- shiproom runbook review order — panel statuses pin which step the
  reviewer is on;
- release-evidence packet refresh queues — `default_action`
  `refresh_evidence_packet` rows feed the queue;
- correction-line staffing — `staff_correction_lane` rows feed
  `artifacts/governance/correction_trigger_table.yaml`;
- milestone close — `block_milestone_close` rows gate the milestone
  scorecard handoff;
- public-proof — `docs_and_claim_parity`,
  `certified_archetype_or_compatibility`, and
  `migration_and_interface_health` panels feed the public claim
  posture before any release-note copy is published.

Consumers MAY hide panels for density, but MUST NOT re-key or
reinterpret panel state. A green dashboard summary that hides a red
panel is non-conforming.

## Conformance checklist

- The dashboard renders one panel for every `panel_class` row above.
- Every panel record validates against
  `schemas/release/shiproom_panel.schema.json`.
- Every panel names at least one required source ref and a
  `freshness_policy_ref`.
- Every panel that cannot reach its required sources reports a closed
  degraded state and not `panel_green`.
- Every alert threshold row names a `default_action` and the panel it
  reads from.
- Every checklist row names the panels it reads from and the default
  action on `no`.
- Every promotion call cites the panel ids, threshold ids, and
  checklist row ids that justified the call.
