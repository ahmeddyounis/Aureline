# Durable-job envelope fixtures

Worked examples for
[`docs/ux/durable_job_envelope_contract.md`](../../../docs/ux/durable_job_envelope_contract.md)
and
[`schemas/ux/durable_job_envelope.schema.json`](../../../schemas/ux/durable_job_envelope.schema.json).

The corpus exercises every progress-phase class in the grammar plus the
badge-source partition rules and the notification-fanout receipt
states:

- `preparing_index_pass.json` — `preparing` phase, indexer pre-flight.
- `queued_remote_attach.json` — `queued` phase with a remote boundary.
- `running_build_progress.json` — `running` phase with a labeled
  progress bar and provider-state sensitivity.
- `waiting_input_review_approval.json` — `waiting_input` phase routed
  to a workspace-trust review.
- `partially_complete_package_update.json` — `partially_complete`
  phase with included, excluded, and failed counts.
- `completed_save_sync.json` — `completed` phase with a completion
  summary and a completion-unread badge.
- `failed_test_run.json` — `failed` phase with retry offered and a
  canonical-object-derived needs-attention badge.
- `cancelled_download.json` — `cancelled` phase with a not-a-badge
  source so the cancel does not inflate counts.
- `held_quiet_hours_completion.json` — `held` phase aggregated under a
  grouped burst with a held quiet-hours fanout receipt.
- `suppressed_admin_policy.json` — `suppressed` phase with a non-null
  policy source ref and suppressed-policy fanout receipts.

Together they exercise the actor / target / phase / progress /
sensitivity / affordance fields, the badge-source partition rules
(envelope state, canonical object, grouped burst, not a badge source),
and the notification-fanout receipts that carry one canonical event /
object lineage across durable rows, status strips, status items, OS
notifications, lock-screen summaries, and companion pushes.
