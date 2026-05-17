# Beta correction train packet template

Use this template for planned correction trains, including beta-only
correction trains and supported-line patch trains that are not emergency
hotfixes. The machine-readable packet is
[`artifacts/release/m3/correction_train/packet.json`](correction_train/packet.json),
and the boundary schema is
[`schemas/release/correction_train_packet.schema.json`](../../../schemas/release/correction_train_packet.schema.json).

## Packet header

- Packet id:
- Train ref:
- Release candidate ref:
- Exact build identity refs:
- Generated at:
- Owner:
- Backup reviewer:
- Decision forum:

## Shared packet form

Every correction row, hotfix row, and backport row uses the same field
families so release, support, and docs can read one packet without
translation:

- `correction_scope` — affected claim rows, profiles, artifact refs,
  release lines, target channels, compatibility refs, and
  `rollback_target`.
- `correction_risk` — risk level, user-data risk, trust/security risk,
  migration/schema risk, blast radius, workaround state, and whether a
  public claim must narrow.
- `correction_evidence` — evidence refs, protected path refs, rerun refs,
  adjacent sweep refs, support packet refs, freshness state, and latest
  rerun timestamp.
- `target_channels` — channel refs and dispositions for hotfix, backport,
  correction train, next-cycle, hold, or not-applicable outcomes.
- `triage_lane` — one of `hotfix`, `backport`,
  `correction_train_only`, or `next_cycle`.
- `backport_decision` — one of `yes`, `no`, `defer`, or
  `not_applicable`, with rationale and owner.
- `rollback_target` — named release candidate or line that users/support
  can return to if the correction fails.
- `known_issue_update` — release notes, docs/help, and support-note refs
  updated in the same lane.

## Correction row

```yaml
item_id:
title:
issue_class:
severity_class:
lifecycle_state:
correction_scope:
  affected_claim_refs: []
  affected_profile_refs: []
  affected_artifact_refs: []
  affected_release_lines: []
  target_channels: []
  rollback_target:
  compatibility_refs: []
correction_risk:
  risk_level:
  user_data_risk:
  security_or_trust_risk:
  migration_or_schema_risk:
  blast_radius:
  workaround_state:
  claim_narrowing_required:
  risk_summary:
correction_evidence:
  evidence_refs: []
  protected_path_refs: []
  rerun_refs: []
  adjacent_sweep_refs: []
  support_packet_refs: []
  freshness_state:
  last_rerun_at:
triage_lane:
  lane_decision:
  decision_state:
  rationale:
  decision_owner:
  decided_at:
  target_train_ref:
  hotfix_packet_ref:
  correction_packet_ref:
  backport_matrix_ref:
target_channels:
  - channel_ref:
    channel_class:
    disposition:
    exact_build_identity_ref:
    rollback_target:
    known_issue_update:
    docs_update_ref:
    support_note_ref:
backport_decision:
  - release_line_ref:
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
```

## Triage workflow

| Issue class | Default lane | Backport rule |
|---|---|---|
| Security, signing, policy escape, or remote code execution | `hotfix` | `yes` for every affected supported stable and long-support line |
| Data loss, corruption, rollback, or migration breakage | `hotfix` unless mitigated and explicitly reviewed | `yes` for supported lines where the contract is claimed |
| Trust boundary or permission prompt failure | `hotfix` on exposed supported surfaces | `yes` wherever the exposed surface is supported |
| Protected performance or crash regression | `correction_train_only` unless stable quality bars break | `yes` only when the support class is materially violated |
| SDK, interface, or extension regression | `backport` when a stable contract is broken | `yes` only for supported stable interface rows |
| Non-protected polish | `next_cycle` | `not_applicable`; must not ride an emergency release |

## Closure checklist

- Every affected supported release line has an explicit
  `backport_decision`.
- Every out-of-band lane names a `rollback_target`.
- Known-issue, docs/help, release notes, and support-note refs are updated
  in the same packet.
- Protected paths and adjacent failure domains have rerun evidence before
  closure.
- The support projection validates with:

```bash
python3 -m tools.ci.m3.correction_train --repo-root . --check
```
