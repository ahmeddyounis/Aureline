# Durable progress and exact-target reopen for the M5 depth lanes

The M5 depth lanes mint new long-running or reviewable work objects. This page
is the human-facing companion to the durable activity-object qualification
audit minted by the `aureline-shell` `m5_activity_objects` module. The audit is
the canonical truth object; later dashboards, docs, release-center views, and
support exports should ingest it instead of cloning its status text.

## The durable-attention contract

Aureline's stable shell already promises that long-running or reviewable work
never disappears into an ephemeral toast: the authoritative object lands in the
durable activity center and reopens the exact target it acted on. The M5 depth
lanes carry that contract forward to ten new job families:

- **Notebook run** (`notebook_run`) — a notebook execution session.
- **Query run** (`query_run`) — a request or query run.
- **Result-grid export** (`result_grid_export`) — a result-grid export.
- **Profiler capture** (`profiler_capture`) — a profiler capture.
- **Replay session** (`replay_session`) — a trace replay session.
- **Pipeline action** (`pipeline_action`) — a pipeline rerun/cancel object.
- **Preview route** (`preview_route`) — a live preview route.
- **Sync state change** (`sync_state_change`) — a workspace sync state change.
- **Offboarding job** (`offboarding_job`) — an export-and-wipe job.
- **Incident packet** (`incident_packet`) — incident-packet generation.

No new family invents a parallel history model with different reopen semantics.
Each family extends the same durable activity-center object and its
export/reopen contract.

## The eight durable-attention guarantees

For each registered family the audit projects the canonical activity-object
descriptor against the qualification result the family certifies for each of
the eight durable-attention guarantees:

| Guarantee | What it proves |
| --------- | -------------- |
| `activity_center_landing` | The work object lands in the activity center, not a toast-only status item. |
| `exact_target_reopen` | The object reopens its exact authoritative target. |
| `reopen_after_focus_loss` | The exact-target reopen survives focus loss. |
| `reopen_after_restart` | The exact-target reopen survives an app restart. |
| `reopen_after_degraded_provider` | The exact-target reopen survives a degraded provider state. |
| `lifecycle_action_semantics` | Dismiss, mute, snooze, acknowledge, resolve, and reopen stay differentiated where the product requires them. |
| `support_export_identity` | CLI/headless output and support bundles refer to the same row id instead of reconstructing activity from logs. |
| `companion_fanout_honesty` | Companion fanout refers to the same object and labels stale or failed delivery honestly. |

A qualified guarantee carries the durable attention packet, durable (not
toast-only) truth, and an evidence-freshness stamp the audit requires — plus
the reopen outcome, survival outcome, action semantics, export identity, or
fanout label its guarantee needs. A red result (a lost reopen target,
toast-only truth, a lost reopen after focus loss, restart, or degraded
provider, collapsed lifecycle actions, a reconstructed export identity, or a
silent fanout failure) is a blocker. A family that invents an ad-hoc parallel
history model, a marketed guarantee claimed with no evidence, and stale
evidence on a marketed guarantee are all blockers, so release tooling can
narrow a marketed family instead of shipping it as implicitly stable.

## Differentiated lifecycle actions

Reviewable M5 work is not collapsed into one generic close action. Each family
declares the differentiated lifecycle actions it exposes on its durable row —
`dismiss`, `mute`, `snooze`, `acknowledge`, `resolve`, and `reopen` — and the
`lifecycle_action_semantics` guarantee certifies that the distinctions survive.

## Support and companion safety

Object identity stays exportable and support-safe: the same durable activity
objects the desktop shell shows are quoted by CLI/headless output, support
bundles, and companion fanout. The support-export wrapper lets a reviewer pivot
from a support case to the family and descriptor revision that flagged a stale
or red durable result, and stale or failed companion fanout is labelled
honestly rather than hidden behind a silent success.

## Canonical artifacts and verification

- Schema: `schemas/ux/m5-activity-object.schema.json`
- Audit fixture: `fixtures/ux/m5/activity-center/report.json`
- Support-export fixture: `fixtures/ux/m5/activity-center/support_export.json`
- Published audit: `artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md`
- CI gate: `tools/ci/m5/activity_objects_check.py`

The headless inspector is the only mint-from-truth path for the fixtures and
the published audit. Regenerate and verify with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- report > \
  fixtures/ux/m5/activity-center/report.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- support-export > \
  fixtures/ux/m5/activity-center/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- compact > \
  fixtures/ux/m5/activity-center/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- report-md > \
  artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md
cargo test -p aureline-shell --test m5_activity_objects_fixtures
python3 tools/ci/m5/activity_objects_check.py
```
