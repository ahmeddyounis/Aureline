# Backup-restore, failover, and key-rotation drill packets (beta)

Reviewer-facing drill packets for the enterprise drill baseline. Every packet
is a JSON record of one rehearsal of backup-restore, failover, or key-rotation
on a claimed managed / enterprise row family, exercised against the same beta
profile the page disclosed.

The canonical record kind is
`security_enterprise_drill_baseline_packet_record`. The schema lives at
[`/schemas/security/enterprise_drill_baseline.schema.json`](../../../../schemas/security/enterprise_drill_baseline.schema.json).
The matrix that lists row families, drill kinds, profiles, freshness states,
and claim impacts lives at
[`enterprise_drill_baseline_matrix.yaml`](enterprise_drill_baseline_matrix.yaml).

## Files

| File | Drill kind | Row family | Profile | Outcome | Evidence freshness | Claim impact if stale |
| --- | --- | --- | --- | --- | --- | --- |
| `managed_policy_distribution_backup_restore_001.json` | `backup_restore` | `managed_policy_distribution` | `mirror_only` | `restored_from_trusted_snapshot` | `fresh` | `no_impact` |
| `managed_policy_distribution_failover_001.json` | `failover` | `managed_policy_distribution` | `connected` | `failed_over_to_declared_fallback` | `stale_within_window` | `no_impact` |
| `managed_policy_distribution_key_rotation_001.json` | `key_rotation` | `managed_policy_distribution` | `enterprise_managed` | `rotated_then_recovered` | `stale_beyond_window` | `downgrade_family_claims` |
| `managed_credential_handle_backup_restore_001.json` | `backup_restore` | `managed_credential_handle` | `offline` | `downgraded_awaiting_admin` | `missing` | `downgrade_affected_claim` |
| `managed_credential_handle_failover_001.json` | `failover` | `managed_credential_handle` | `connected` | `failed_over_to_declared_fallback` | `fresh` | `no_impact` |
| `managed_credential_handle_key_rotation_001.json` | `key_rotation` | `managed_credential_handle` | `enterprise_managed` | `rotated_then_recovered` | `fresh` | `no_impact` |
| `enterprise_identity_session_backup_restore_001.json` | `backup_restore` | `enterprise_identity_session` | `mirror_only` | `restored_from_trusted_snapshot` | `fresh` | `no_impact` |
| `enterprise_identity_session_failover_001.json` | `failover` | `enterprise_identity_session` | `connected` | `failed_over_to_declared_fallback` | `fresh` | `no_impact` |
| `enterprise_identity_session_key_rotation_001.json` | `key_rotation` | `enterprise_identity_session` | `enterprise_managed` | `rotated_then_recovered` | `fresh` | `no_impact` |

Each packet records the before / after state labels, the before / after
authority tokens on the affected claim, an export-safe explanation, the
recorded evidence freshness, the claim impact if the evidence goes stale, and
hard `sibling_lanes_unwidened`, `local_editing_preserved`,
`raw_private_material_excluded`, and `no_public_endpoint_fallback`
guarantees.

## How to regenerate

The drill packets, the page, the summary, and the support export are all seeded
from `crates/aureline-auth/src/enterprise_drill_baseline/mod.rs` and surfaced by
the headless inspector binary.

```sh
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- page
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-packets
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- summary
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- defects
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- validate
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-backup-restore
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-failover
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-key-rotation
```

## Invariants

- A drill packet must not widen authority on a sibling lane
  (`sibling_lanes_unwidened = true`).
- A drill packet must preserve local editing
  (`local_editing_preserved = true`).
- A drill packet must exclude raw private / secret material
  (`raw_private_material_excluded = true`).
- A drill packet must not silently fall back to a public endpoint
  (`no_public_endpoint_fallback = true`).
- The seeded baseline must include at least one drill packet for each of the
  three drill kinds (`backup_restore`, `failover`, `key_rotation`) on every
  claimed managed / enterprise row family
  (`managed_policy_distribution`, `managed_credential_handle`,
  `enterprise_identity_session`).
- A drill packet's `evidence_freshness` of `stale_beyond_window` or `missing`
  must declare a non-`no_impact` `claim_impact_if_stale`; a `fresh`
  evidence_freshness must not assert a downgrade. The validator's
  `stale_evidence_without_downgrade` and
  `fresh_evidence_with_unexpected_downgrade` defects enforce this.

See the reviewer-facing landing page at
[`/docs/security/m3/enterprise_drill_baseline.md`](../../../../docs/security/m3/enterprise_drill_baseline.md).
