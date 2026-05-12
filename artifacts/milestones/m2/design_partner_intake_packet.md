# External Alpha Design Partner Intake Packet

This packet is the canonical intake form for external alpha design partners. It
captures partner role, stack, privacy posture, reproducible task script,
completion criteria, feedback route, blocker severity, and rollback posture
before any partner-derived evidence enters release, support, benchmark, or
public-proof lanes.

## Canonical Inputs

- Alpha scope matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Alpha go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Upstream intake checklist: `artifacts/program/design_partner_intake_checklist.yaml`
- Task pack: `artifacts/milestones/m2/design_partner_task_pack.md`
- Feedback taxonomy: `artifacts/feedback/design_partner_feedback_taxonomy.yaml`
- Known limits: `artifacts/feedback/external_alpha_known_limits.md`
- Partner guide: `docs/alpha/design_partner_guide.md`
- Issue routing matrix: `artifacts/governance/issue_routing.yaml`
- Validator: `ci/check_design_partner_alpha.py`

If this packet and `artifacts/program/design_partner_intake_checklist.yaml`
disagree, the checklist remains authoritative for corpus, benchmark, fixture,
and public-proof admission. This packet is the partner-facing application of
that checklist for the external alpha lane.

## Intake Decision

An intake is ready for partner task execution when:

- `partner_role` maps to a target persona or adjacent support role;
- `stack_profile` maps to an in-scope alpha wedge;
- `workflow_bundle_ref`, `archetype_row_ref`, and `scoreboard_row_ref` resolve
  to existing alpha artifacts;
- `privacy_posture` and `privacy_review_state` are recorded before evidence is
  requested;
- `reproducible_task_scripts` names at least one task script from the task pack;
- `completion_criteria` states the pass/fail rubric;
- `blocker_severity` uses the feedback taxonomy vocabulary; and
- `rollback_posture` states whether the claim remains supported, narrows to a
  known limit, or blocks until replacement evidence lands.

## Required Fields

| Field | Purpose |
|---|---|
| `partner_ref` | Opaque partner reference. Public packets use only this value or a redacted label. |
| `partner_role` | Developer role or support role completing the task. |
| `stack_profile` | Stack family, toolchain notes, operating system class, and local/helper-backed posture. |
| `wedge_ref` | One of `alpha_wedge:typescript_javascript` or `alpha_wedge:python`. |
| `workflow_bundle_ref` | Launch bundle id and revision from the upstream intake checklist. |
| `archetype_row_ref` | Archetype row id and revision for the chosen reference workspace. |
| `scoreboard_row_ref` | Scoreboard row that the task result updates or blocks. |
| `task_script_id` | Stable task id from the task pack. |
| `privacy_posture` | Whether the partner can share public-safe, internal-only, or restricted evidence. |
| `privacy_review_state` | Redaction state before raw artifacts move. |
| `redaction_review_ref` | Opaque review evidence or `not_required_for_metadata_only`. |
| `reproducibility_packet_ref` | Build, workspace, fixture, and task evidence needed to replay the report. |
| `completion_criteria` | Pass/fail rubric copied from the task pack. |
| `blocker_severity` | Severity from the feedback taxonomy. |
| `known_limit_refs` | Known-limit ids when the report narrows scope, support, docs, migration, or privacy. |
| `rollback_posture` | Claim action: keep, narrow, block pending packet, or withdraw dependent evidence. |

## Cohort Guardrails

- Supported cohorts are named design partners working on TypeScript /
  JavaScript web-app/service or Python service/data-app tasks only.
- The default deployment posture is local desktop. Helper-backed language,
  task, Git, Python environment, or devcontainer support is limited and must be
  called out in the task result.
- Managed cloud, browser or mobile companion parity, full notebook parity, and
  new language wedges require scope review before they can be represented as
  alpha task failures.
- Partner-derived repositories, traces, and support packets do not become public
  fixtures or public-proof evidence until the upstream checklist records privacy,
  license, retention, owner, and publication review.
- No raw user content is required to file feedback. Metadata-first reports are
  acceptable and preferred until redaction is cleared.

## Privacy Review Gate

Evidence is shareable only when `privacy_review_state` is one of:

| State | Effect |
|---|---|
| `redaction_review_not_required` | Metadata-only report; no raw partner artifact is attached. |
| `redaction_cleared` | The named artifact passed secret, personal-data, path/name, and retention review for the selected route. |
| `redaction_review_required` | Do not attach raw artifacts; file the report with metadata and a redaction review request. |
| `raw_content_blocked` | Raw artifacts may not be shared. Use summaries, derived metrics, or sanitized replacements only. |
| `partner_consent_required` | Hold publication or public summary until the partner approval ref is recorded. |

## Feedback Routing Loop

1. Select the task script and complete the intake fields.
2. Run the task against the required fixture or privacy-cleared partner
   workspace.
3. Record pass/fail, observed behavior, redaction state, and evidence refs.
4. Choose a `feedback_category` and `blocker_severity` from the taxonomy.
5. If the report narrows scope or support, add `known_limit_refs` and update the
   known-limits packet in the same change.
6. Route through the route class named by the taxonomy. Private partner,
   support, and security routes stay private unless a disclosure transition is
   approved.
7. Re-run the validator and attach the validation capture to the proof packet.

## Intake Template

```yaml
partner_ref: partner_ref:redacted_design_partner_001
partner_role: full_stack_web_developer
stack_profile:
  language_stack: typescript_javascript_web_app
  os_arch_profile: macos_arm64_local
  deployment_posture: local_desktop
wedge_ref: alpha_wedge:typescript_javascript
workflow_bundle_ref:
  bundle_id: launch_bundle:typescript_web_app.seed
  bundle_revision: 1
archetype_row_ref:
  archetype_row_id: archetype_row:ts_web_app_or_service
  archetype_revision: 1
scoreboard_row_ref: scoreboard_row:alpha_scope.ts_js_run_test_debug
task_script_id: task.alpha.ts_js.test_debug_loop
privacy_posture: partner_metadata_only_until_redaction_clears
privacy_review_state: redaction_review_required
redaction_review_ref: review:privacy.pending_partner_report_001
reproducibility_packet_ref: packet:external_alpha.partner_report_001
completion_criteria: pass_when_task_pack_rubric_all_required_checks_pass
feedback_category: alpha_task_blocker
blocker_severity: task_blocking
known_limit_refs: []
rollback_posture: block_scoreboard_row_until_packet_current
evidence_refs:
  - evidence:partner_report.redacted_summary_001
```

## Acceptance State Coverage

| Acceptance state | Proof path |
|---|---|
| Partner can onboard from one packet | This packet plus `docs/alpha/design_partner_guide.md`. |
| Task pack names daily-loop scenarios and pass/fail rubrics | `artifacts/milestones/m2/design_partner_task_pack.md`. |
| Feedback routes into named taxonomy and known limits | `artifacts/feedback/design_partner_feedback_taxonomy.yaml` and `artifacts/feedback/external_alpha_known_limits.md`. |
| Privacy-sensitive artifacts are redaction reviewed | `privacy_review_state`, `redaction_review_ref`, and protected feedback fixtures under `fixtures/feedback/external_alpha_cases/`. |

