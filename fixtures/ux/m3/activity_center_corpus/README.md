# Durable Attention Activity-Center Corpus

This fixture corpus is generated from
`crates/aureline-shell/src/durable_attention_beta.rs` through the
`aureline_shell_durable_attention` inspector. It joins the existing
activity-center beta projection, notification privacy beta projection,
and frozen durable-attention fixtures into one packet for durable job
rows, badge-class counting, quiet-hours suppression, exact reopen, and
support-export lineage.

## Files

| File | Purpose |
| --- | --- |
| `packet.json` | Full durable-attention packet with summary, state machine, cases, audits, proofs, and support lineage. |
| `state_machine.json` | Durable job-row state machine for running, queued/waiting, needs-approval, completed, failed, cancelled, and history-only states. |
| `cases.json` | Conformance cases across indexing, restore, install/update/download, remote reconnect, task/test/debug, AI review, git/review, companion handoff, and admin suppression. |
| `badge_audit.json` | Badge-class review proving counts derive from durable rows, canonical objects, or grouped bursts rather than mixed local counters. |
| `quiet_hours_audit.json` | Quiet-hours, cross-client dedupe, and admin suppression audit rows. |
| `exact_reopen_proofs.json` | Proof rows showing every activation reopens the exact object, a truthful placeholder, or an explained revalidation path. |
| `support_export_lineage.json` | Support-export rows proving lineage can be reconstructed without scraping toasts, badge copy, or raw private material. |

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- packet                 > fixtures/ux/m3/activity_center_corpus/packet.json
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- state-machine          > fixtures/ux/m3/activity_center_corpus/state_machine.json
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- cases                  > fixtures/ux/m3/activity_center_corpus/cases.json
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- badge-audit            > fixtures/ux/m3/activity_center_corpus/badge_audit.json
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- quiet-hours-audit      > fixtures/ux/m3/activity_center_corpus/quiet_hours_audit.json
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- reopen-proofs          > fixtures/ux/m3/activity_center_corpus/exact_reopen_proofs.json
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- support-export-lineage > fixtures/ux/m3/activity_center_corpus/support_export_lineage.json
```

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_durable_attention -- validate
cargo test -q -p aureline-shell --test durable_attention_beta_fixtures
```
