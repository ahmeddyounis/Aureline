# M5 durable progress-indicator & job-row actor, phase, action & history parity

Generated from the seeded packet in
[`crate::m5_durable_progress_certification`](../../crates/aureline-shell/src/m5_durable_progress_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_durable_progress_certification -- markdown > \
  artifacts/shell/m5-durable-progress-certification.md
```

- Packet id: `m5-durable-progress-certification:stable:0001`
- Source schema ref: `schemas/shell/m5-durable-progress-certification.schema.json`
- Certifies matrix packet: `m5-shell-primitives:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 9
- Green: 5
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification dimensions

- `durable_presence`
- `progress_attribution`
- `grouped_history`
- `progress_export`

## Certification rows

| Family | Status | Qualification | Presence | Attribution | History | Export | Durable-row | Waiver |
| ------ | ------ | ------------- | -------- | ----------- | ------- | ------ | ----------- | ------ |
| Indexing / workspace-scan progress | `green` | `stable` | `durable_reviewable_after_focus_loss` | `actor_phase_action_object_attributed` | `grouped_history_and_blocked_reasons_preserved` | `progress_and_history_reconstructable` | `true` | — |
| Notebook / runtime execution progress | `green` | `stable` | `durable_reviewable_after_focus_loss` | `actor_phase_action_object_attributed` | `grouped_history_and_blocked_reasons_preserved` | `progress_and_history_reconstructable` | `true` | — |
| Request / data-load progress | `green` | `stable` | `durable_reviewable_after_focus_loss` | `actor_phase_action_object_attributed` | `grouped_history_and_blocked_reasons_preserved` | `progress_and_history_reconstructable` | `true` | — |
| Download progress | `yellow` | `stable` | `disclosed_reduced_history_retention` | `actor_phase_action_object_attributed` | `grouped_history_and_blocked_reasons_preserved` | `progress_and_history_reconstructable` | `true` | — |
| Update / install progress | `green` | `stable` | `durable_reviewable_after_focus_loss` | `actor_phase_action_object_attributed` | `grouped_history_and_blocked_reasons_preserved` | `progress_and_history_reconstructable` | `true` | — |
| Sync / replication progress | `yellow` | `stable` | `durable_reviewable_after_focus_loss` | `actor_phase_action_object_attributed` | `disclosed_compacted_history` | `progress_and_history_reconstructable` | `true` | `waiver:sync-compacted-grouped-history:0001` |
| Branch-agent / automation progress | `green` | `stable` | `durable_reviewable_after_focus_loss` | `actor_phase_action_object_attributed` | `grouped_history_and_blocked_reasons_preserved` | `progress_and_history_reconstructable` | `true` | — |
| Provider-handoff progress | `yellow` | `stable` | `durable_reviewable_after_focus_loss` | `disclosed_coarse_attribution` | `grouped_history_and_blocked_reasons_preserved` | `progress_and_history_reconstructable` | `true` | — |
| Support / export job progress | `yellow` | `stable` | `durable_reviewable_after_focus_loss` | `actor_phase_action_object_attributed` | `grouped_history_and_blocked_reasons_preserved` | `disclosed_partial_capture` | `true` | — |

## Auto-narrowed rows

- `download` (`yellow`) — Under the seeded download lane older completed download rows compact into a summary sooner than the standard retention window while every in-flight download and its recent history stay reviewable after focus loss; the reduction is disclosed and the row is narrowed below green.
- `sync` (`yellow`) — The sync lane preserves each blocked/paused reason and keeps every in-flight job reviewable, but older grouped replication batches roll up into a digest with a reopen path sooner than the standard retention window; the compaction is disclosed behind a waiver and never destructive, so the row is narrowed below green while the reduction is in force.
- `provider_handoff` (`yellow`) — Under the seeded provider-handoff lane a grouped batch shows the handoff subsystem and provider but folds per-job phase into a summary while the actor, cancel/retry/open-details actions, and authoritative-object link stay present; the reduction is disclosed and the row is narrowed below green.
- `support_export` (`yellow`) — The support/export lane's own support export reconstructs current progress and discloses a partial capture of the recent job-history chronology while the high-volume export log is still being trimmed; the partial capture is disclosed and the row is narrowed below green.

## Exact certification causes

- `download` — `spinner_only_state` (disclosed: `true`) — Under one surface the durable-history retention window is disclosedly reduced (older completed rows compact into a summary sooner) while every in-flight job and its recent history stay reviewable after focus loss; the reduction is disclosed and the row is narrowed below green.
- `sync` — `progress_lost_on_look_away` (disclosed: `true`) — The grouped history is disclosedly compacted (older grouped batches roll up into a digest with a reopen path) while each blocked/paused reason stays reconstructable; the compaction is disclosed and waivered, and the row is narrowed below green.
- `provider_handoff` — `grouped_progress_unattributed` (disclosed: `true`) — Under one surface the attribution is disclosedly coarse (a grouped batch shows the subsystem but folds per-job phase into a summary) while the actor, action affordances, and authoritative-object link stay present; the reduction is disclosed and the row is narrowed below green.
- `support_export` — `proof_stale` (disclosed: `true`) — The support export reconstructs current progress and discloses a partial capture of the recent job-history chronology while it is still being trimmed; the partial capture is disclosed and the row is narrowed below green.

## Active waivers

- `waiver:sync-compacted-grouped-history:0001` (`sync`, owner: Shell/activity owner, expires `2026-09-30T00:00:00Z`) — Under the seeded sync lane every in-flight job stays reviewable and each blocked/paused reason stays reconstructable, but older grouped replication batches roll up into a digest with a reopen path sooner than the standard retention window rather than staying enumerated per batch. The compaction is disclosed, never destructive, and the grouped digest keeps its reopen path into durable history.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_durable_progress_certification -- validate
cargo test -p aureline-shell --test m5_durable_progress_certification_fixtures
```
