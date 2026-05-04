# Managed continuity and account-exit rehearsal cases

This directory holds the worked rehearsal cases the managed-continuity
verification packet relies on. Every case resolves to one
`managed_continuity_rehearsal_record` whose shape, vocabulary, and
required-field rules are frozen in
[`/docs/verification/managed_continuity_and_account_exit_packet.md`](../../../docs/verification/managed_continuity_and_account_exit_packet.md).

Each rehearsal:

- quotes a workspace-lifecycle record id (when the rehearsal class
  binds the workspace-lifecycle axis) or an account-exit packet id
  (when the rehearsal class binds the account-exit axis), or both;
- carries an `export_before_suspend_rows` list whose entries cover
  the seven artifact kinds named in the spec (workspace files,
  review packets, evidence packets, notebooks, workflow bundles,
  queued provider drafts, support captures);
- carries a `local_baseline_admissible_surfaces` list drawn from the
  closed surface set re-exported by the managed-workspace-lifecycle
  contract;
- carries a `reopen_locally_afterward` list naming what the user can
  reopen on the local host after the rehearsal close;
- cites `local_baseline_proof.yaml` directly so shiproom and
  public-proof packets can quote the proof verbatim instead of
  rewording the local-baseline claim per audience.

The required rehearsal set covers every scenario named in the spec:

| Fixture | Rehearsal class |
|---|---|
| `suspended_workspace_local_docs_and_review.yaml` | `managed_workspace_suspended` |
| `account_exit_with_export_path.yaml` | `account_exit` |
| `grace_period_downgrade.yaml` | `plan_grace_downgrade` |
| `local_only_fallback_after_service_outage.yaml` | `local_only_fallback_after_service_outage` |
| `seat_loss_local_baseline.yaml` | `seat_loss` |
| `policy_suspension_local_baseline.yaml` | `policy_suspension` |

`manifest.yaml` is the authoritative roster.
