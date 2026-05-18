# Run Lineage And Interruption Fixtures

Checked-in fixtures pin the canonical run-lineage scenarios replayed by
`aureline_runtime::seeded_run_history_support_export`.

Each case names the expected freshness marker, old/current relationship,
interruption token, required rerun-review drift fields, and continuity markers
that must survive look-away, sleep/resume, window switch, or runtime restart.
The boundary schemas live at
[`/schemas/runtime/run_summary.schema.json`](../../../../schemas/runtime/run_summary.schema.json)
and
[`/schemas/runtime/rerun_review.schema.json`](../../../../schemas/runtime/rerun_review.schema.json).

| Fixture | Freshness | Interruption | Rerun review |
| --- | --- | --- | --- |
| `current_local_passed_sleep_resume.json` | `current` | none | no review required |
| `remote_disconnect_current_context_review.json` | `stale` | `remote_disconnect` | exact vs current review required |
| `auth_expiry_stale_evidence.json` | `stale` | `auth_expiry` | exact vs current review required |
| `stale_import_manual_replay.json` | `imported` | `manual_replay_requirement` | manual replay review required |
