# Beta hotfix packet template

Use this template only when the correction lane is `hotfix`. Hotfixes
carry the smallest viable change set and use the same signing,
provenance, support-export, rollback, and known-issue rules as ordinary
release lanes.

## Header

- Hotfix packet ref:
- Source correction row:
- Release candidate ref:
- Exact build identity refs:
- Target channels:
- Rollback target:
- Owner:
- Backup reviewer:
- Approval forum:

## Required standard fields

- `correction_scope`: affected claims, profiles, artifacts, supported
  release lines, target channels, and compatibility refs.
- `correction_risk`: security/trust, data-loss, migration/schema, blast
  radius, workaround, and claim-narrowing state.
- `correction_evidence`: reproducer, protected-path reruns, adjacent
  sweeps, support packet refs, freshness, and latest rerun timestamp.
- `target_channels`: the exact channels receiving the hotfix and whether
  each channel is held, patched, or narrowed.
- `triage_lane`: must be `hotfix`; rationale must name why the normal
  train is insufficient.
- `backport_decision`: explicit `yes`, `no`, or `defer` for every
  affected supported line.
- `rollback_target`: named release candidate or supported line used if
  the hotfix is revoked.
- `known_issue_update`: release notes, docs/help, and support-note refs
  updated in the same lane.

## Hotfix admission

```yaml
hotfix_packet_ref:
source_correction_item_ref:
issue_class:
severity_class:
admission_reason:
minimized_change_set:
  included_artifact_refs: []
  excluded_scope_refs: []
  no_feature_broadening: true
release_truth:
  known_issue_update:
  docs_update_ref:
  support_note_ref:
  claim_update_refs: []
rollback:
  rollback_target:
  retained_artifact_set_ref:
  revocation_path_ref:
evidence:
  reproducer_refs: []
  protected_path_rerun_refs: []
  adjacent_sweep_refs: []
  support_packet_refs: []
  freshness_state:
backport_decision:
  - release_line_ref:
    decision:
    rationale:
    owner:
    target_release_ref:
    rollback_target:
```

## Non-admissible hotfix content

- Feature broadening, new target personas, and speculative cleanup.
- SDK, schema, or support-window changes unless the correction packet and
  interface diff report say so.
- Copy polish unrelated to the blocker.
- Any correction without a current rollback target or known-issue update.
