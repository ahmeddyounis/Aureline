# Durable Attention Beta Contract

This contract binds durable job rows, activity-center rows, badge
counts, quiet-hours suppression, OS/companion fanout, exact reopen, and
support export into one beta-review packet. It composes existing
contracts instead of creating a second task model:

- [`../attention_activity_taxonomy.md`](../attention_activity_taxonomy.md)
- [`../durable_work_contract.md`](../durable_work_contract.md)
- [`../durable_job_envelope_contract.md`](../durable_job_envelope_contract.md)
- [`../notification_delivery_contract.md`](../notification_delivery_contract.md)
- [`../os_notification_and_quiet_hours_contract.md`](../os_notification_and_quiet_hours_contract.md)
- [`activity_center_beta.md`](activity_center_beta.md)
- [`notification_privacy_and_quiet_hours.md`](notification_privacy_and_quiet_hours.md)

The schema of record for the packet is
[`/schemas/ux/durable_job_row.schema.json`](../../../schemas/ux/durable_job_row.schema.json).

## Contract Surface

The shell projection lives in
[`crates/aureline-shell/src/durable_attention_beta.rs`](../../../crates/aureline-shell/src/durable_attention_beta.rs).
It exports six record sets under the shared contract ref
`shell:durable_attention_beta:v1`:

- `durable_job_row_state_machine_entry_record` — one state-machine row
  for `running`, `queued_waiting`, `needs_approval`, `completed`,
  `failed`, `cancelled`, and `history_only`.
- `durable_attention_conformance_case_record` — one conformance case
  per durable-attention family.
- `durable_attention_badge_class_audit_row_record` — badge-class count
  lineage and dedupe proof.
- `durable_attention_quiet_hours_audit_row_record` — quiet-hours,
  admin suppression, critical bypass, and cross-client dedupe audit.
- `durable_attention_exact_reopen_proof_record` — exact object,
  truthful placeholder, or explained revalidation reopen proof.
- `durable_attention_support_export_lineage_row_record` — support
  export lineage without raw private material.

## Acceptance Rules

Durable work is conforming only when all of these are true:

- Every long-running or reviewable case names a durable job id,
  canonical event id, canonical object target, actor/subsystem,
  execution origin, scope label, phase label, age label, and
  open-details path.
- Running, queued, approval-blocked, completed, failed, cancelled, and
  history-only states are represented by the state machine. Each state
  rejects transient-only treatment.
- Badge counts map to one badge class and one source: envelope state,
  canonical object, grouped burst, or no badge source. Mixed-class
  totals are non-conforming.
- Repeated failures from one root cause coalesce under grouped-burst or
  subsystem/object/phase dedupe and remain one durable owner object.
- Quiet-hours and admin suppression preserve durable history, audit
  refs, and support-export lineage. Critical or blocking trust
  scenarios may bypass holds only through typed escalation.
- OS notifications, lock-screen summaries, compact shell surfaces, and
  companion pushes reopen the narrowest truthful destination. Generic
  home fallback is non-conforming.
- External shortcuts may reopen in product but cannot bypass preview,
  approval, revalidation, or trust logic.
- Support export can reconstruct source, target, phase, state, badge,
  suppression, fanout, reopen proof, and audit refs without scraping
  transient toasts or badge text.

## Fixture Corpus

Fixtures live under
[`/fixtures/ux/m3/activity_center_corpus/`](../../../fixtures/ux/m3/activity_center_corpus/):

- `packet.json`
- `state_machine.json`
- `cases.json`
- `badge_audit.json`
- `quiet_hours_audit.json`
- `exact_reopen_proofs.json`
- `support_export_lineage.json`

The corpus covers indexing, restore, install/update/download, remote
reconnect, task, test, debug, AI review, git/review, companion
handoff, and admin suppression cases.

## Headless Inspector

```sh
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- packet
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- state-machine
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- cases
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- badge-audit
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- quiet-hours-audit
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- reopen-proofs
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- support-export-lineage
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- validate
```

## Verification

```sh
cargo test -q -p aureline-shell --lib durable_attention_beta
cargo test -q -p aureline-shell --test durable_attention_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- validate
```

The fixture test asserts that checked-in JSON is byte-for-byte equivalent
to the seeded packet after deserialization, that all required families
and states are present, that repeated failures coalesce, and that
quiet-hours/reopen proofs preserve durable truth.
