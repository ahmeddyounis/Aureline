# Beta backport packet template

Use this template when a correction must be evaluated against supported
stable or long-support lines. A backport packet records a decision for
every affected supported line even when the outcome is `no` or `defer`.

## Header

- Backport packet ref:
- Source correction row:
- Source train ref:
- Affected release lines:
- Owner:
- Backup reviewer:
- Decision forum:

## Required standard fields

- `correction_scope`: affected claims, profiles, artifacts, release
  lines, target channels, compatibility refs, and `rollback_target`.
- `correction_risk`: user-data risk, security/trust risk,
  migration/schema risk, blast radius, workaround, and claim-narrowing
  state.
- `correction_evidence`: source fix refs, affected-line repro evidence,
  rerun refs, adjacent sweeps, support packet refs, and freshness.
- `target_channels`: each stable, long-support, beta, mirror, or offline
  channel receiving a patch, being held, or being explicitly skipped.
- `triage_lane`: `backport` when the correction rides a normal train but
  still needs supported-line action; `hotfix` when the source lane is
  emergency.
- `backport_decision`: `yes`, `no`, `defer`, or `not_applicable`, with
  rationale, owner, due date, and target release when applicable.
- `rollback_target`: named release candidate or release line for every
  `yes` decision.
- `known_issue_update`: release notes, docs/help, and support-note refs
  updated in the same lane.

## Matrix row

```yaml
release_line_ref:
support_line_class:
channel_class:
affected:
decision:
rationale:
decision_owner:
decision_due_at:
target_release_ref:
rollback_target:
known_issue_update:
support_note_ref:
docs_update_ref:
compatibility_or_interface_diff_ref:
```

## Decision rules

- `yes` requires target release, rollback target, known-issue update,
  docs/help update, support note, and affected-line rerun evidence.
- `no` requires rationale and explicit proof that the line is unaffected,
  unsupported, outside the claim, or safer with a claim narrowing.
- `defer` requires a due date, owner, support-note caveat, and claim
  posture that does not overstate support while the decision is pending.
- `not_applicable` is allowed only when the line is not affected or is
  not a supported line for the claimed surface.
