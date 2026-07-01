# M5 Lifecycle-State, Degraded-Vocabulary, and Critical-Journey Checkpoint Matrix Contract

Task: M05-732 — Freeze the M5 lifecycle-state, degraded-vocabulary, and
critical-journey checkpoint matrix (batch B85).

This lane freezes Aureline's canonical M5 object-state model and its
protected-journey checkpoint inventory into one export-safe packet. It is the
single source of truth for whether a claimed M5 surface may publish a
lifecycle-state or checkpoint claim, so later M5 rows can no longer invent
private state vocabularies or anonymous checkpoints.

## Track invariant

Every long-lived M5 object has:

- an **explicit state machine** drawn from one controlled state vocabulary,
- one **visible primary status surface**,
- one **exportable status code**,
- one **controlled last-failure reason**, and
- one **named recovery affordance**.

Controlled terms keep the same meaning across UI, CLI, docs/help, support
exports, and telemetry; and protected journeys show **milestone checkpoints**
instead of anonymous spinners.

## Controlled state vocabulary

The fifteen controlled lifecycle states are frozen in the vocabulary set and may
not be extended by a downstream row:

`ready`, `warming`, `partial`, `stale`, `rebuilding`, `restricted`,
`policy_blocked`, `reconnecting`, `degraded`, `read_only_degraded`,
`unavailable`, `rollback_available`, `deprecated`, `experimental`,
`retest_pending`.

The matrix also freezes the primary-status-surface, recovery-affordance,
last-failure-reason-class, and journey-checkpoint vocabularies. Each frozen list
is validated against the typed `ALL` arrays in the module, so the vocabulary
cannot silently drift.

## Governed object families (13)

`workspace`, `extension`, `remote_session`, `collaboration_session`,
`ai_action`, `update_rollback`, `notebook_runtime`, `request_api_run`,
`preview_session`, `pipeline_run`, `data_session`, `profiler_capture`,
`companion_session`.

Each object-state row binds the family to the states its machine admits (always
including `ready`), its one primary status surface, its one exportable status
code field, its one last-failure reason field plus the controlled reason classes
it reports, and its one named recovery affordance. Stable objects must carry a
proof packet; every object names its downgrade triggers and consumer surfaces.

## Protected critical journeys (13)

`workspace_restore`, `remote_reconnect`, `extension_activation`,
`collaboration_join`, `ai_action_run`, `update_rollback_journey`,
`notebook_execution`, `request_run`, `preview_build`, `pipeline_run_journey`,
`data_session_connect`, `profiler_capture_journey`, `companion_attach`.

Each journey names the ordered milestone checkpoints it shows (at least two,
unique, ending in a `ready`, `partial_ready`, or `recoverable_failure`
terminal), so a protected journey never falls back to an anonymous spinner. A
journey that drops `shows_named_checkpoints` or presents a malformed sequence
fails validation.

## Downgrade and narrowing

Downgrade narrows the claim rather than hiding the object. The narrowed fixtures
prove this: a degraded remote session narrows to Beta and a retest-pending
notebook runtime narrows to Preview, and both remain present in the matrix with
their full state binding intact.

## Schemas and artifacts

- Object-state schema: [`schemas/lifecycle/m5-object-state.schema.json`](../../schemas/lifecycle/m5-object-state.schema.json)
- Journey-checkpoint schema: [`schemas/lifecycle/m5-journey-checkpoint.schema.json`](../../schemas/lifecycle/m5-journey-checkpoint.schema.json)
- Canonical support export: `artifacts/release/m5-lifecycle-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-lifecycle-proof/matrix.csv`
- Markdown lifecycle report: `artifacts/lifecycle/m5-lifecycle-matrix.md`
- Narrowed fixtures: `fixtures/state/m5-lifecycle-scenarios/`

## Source of truth

The Rust module
`crates/aureline-shell/src/freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix`
is authoritative. The seed builder `seeded_m5_lifecycle_matrix()` is the single
producer of the checked-in support export; the headless emitter
`aureline_shell_m5_lifecycle_matrix` regenerates every artifact, and the inline
`validate()` is the gate. Round-trip tests assert the on-disk artifacts and
fixtures match the seed builders bit-for-bit.

## Regeneration

```sh
BIN=./target/debug/aureline_shell_m5_lifecycle_matrix
cargo build -p aureline-shell --bin aureline_shell_m5_lifecycle_matrix
$BIN support-export > artifacts/release/m5-lifecycle-proof/support_export.json
$BIN csv            > artifacts/release/m5-lifecycle-proof/matrix.csv
$BIN report         > artifacts/lifecycle/m5-lifecycle-matrix.md
$BIN fixture-remote-session-degraded-narrowed  > fixtures/state/m5-lifecycle-scenarios/remote_session_degraded_narrowed.json
$BIN fixture-notebook-runtime-retest-narrowed  > fixtures/state/m5-lifecycle-scenarios/notebook_runtime_retest_narrowed.json
$BIN validate
```
