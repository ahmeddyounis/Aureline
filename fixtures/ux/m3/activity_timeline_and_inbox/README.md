# Activity Timeline and Attention Inbox Corpus

This fixture corpus is generated from
`crates/aureline-shell/src/activity_timeline/` by the
`aureline_shell_activity_timeline` headless inspector. It frozen-snapshots
the shared event/history row primitive, the timeline group, the narrative
summary card, and the attention-inbox triage item that the activity
center, AI evidence lane, policy-change lane, provider-sync lane, update
history, reconnect flow, and recovery flow all project through.

## Files

| File | Purpose |
| --- | --- |
| `packet.json` | Full chronology + inbox conformance packet (summary + snapshot). |
| `snapshot.json` | Deterministic snapshot consumed by chrome (rows, groups, summary cards, inbox). |
| `event_rows.json` | The shared activity-event-row primitive, one row per lane phase. |
| `timeline_groups.json` | Grouped timeline views, including phase boundaries and collapse defaults. |
| `narrative_summary_cards.json` | Narrative summary cards that cite member rows by id. |
| `attention_inbox.json` | Triage inbox snapshot covering actionable, snoozed, acknowledged, resolved, and muted classes. |
| `summary.json` | Lane / verb / outcome / importance coverage assertions for the packet. |

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- packet        > fixtures/ux/m3/activity_timeline_and_inbox/packet.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- snapshot      > fixtures/ux/m3/activity_timeline_and_inbox/snapshot.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- rows          > fixtures/ux/m3/activity_timeline_and_inbox/event_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- groups        > fixtures/ux/m3/activity_timeline_and_inbox/timeline_groups.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- summary-cards > fixtures/ux/m3/activity_timeline_and_inbox/narrative_summary_cards.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- inbox         > fixtures/ux/m3/activity_timeline_and_inbox/attention_inbox.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- summary       > fixtures/ux/m3/activity_timeline_and_inbox/summary.json
```

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_activity_timeline -- validate
cargo test -q -p aureline-shell --test activity_timeline_and_inbox_fixtures
```
