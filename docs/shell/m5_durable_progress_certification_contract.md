# M5 durable progress-indicator & job-row actor, phase, action & history parity contract

This lane is the **durable-progress certification capstone** on top of the frozen
[M5 status-bar, transient-inspect, pane-control, and durable-progress-component
matrix](m5_shell_primitives_contract.md). Where the matrix *freezes* the durable-work
primitives — the ambient progress indicator and the durable job row, with their progress
states, source/provider/freshness labels, accessibility routes, and mandatory labels —
this lane *certifies* that, in every claimed M5 durable-work job family, durable work is
never represented only by a transient spinner or toast and stays reviewable after the user
looks away; that every progress row attributes its actor/subsystem, phase, current step,
cancel/retry/open-details actions, and a link back to the authoritative object or evidence
packet; that grouped completion/failure history and blocked/paused reasons are preserved
in durable, reopenable history; and that current progress and recent job history are
reconstructable from a support export without relying on transient toasts or a live
dashboard.

The lane exists so that M5 can honestly claim mature shell quality: users never lose
progress after looking away, never mistake an anonymous spinner for attributed durable
work, and never have to reproduce a stuck or failed job by hand because its progress and
history are diagnosable from the support export.

## Governed job families

The certification proof covers exactly nine claimed M5 durable-work job families, and
refuses to ship if any is missing. Multi-job summaries, grouped completions, and
reopen-after-focus-loss are certified within each family's row rather than as separate
families:

- `indexing` — Indexing / workspace-scan progress
- `notebook_runtime` — Notebook / runtime execution progress
- `request_data_load` — Request / data-load progress
- `download` — Download progress
- `update` — Update / install progress
- `sync` — Sync / replication progress
- `branch_agent` — Branch-agent / automation progress
- `provider_handoff` — Provider-handoff progress
- `support_export` — Support / export job progress

## Per-family certification row

Each row names the progress primitives it drives (`progress_indicator` and
`durable_job_row`) and — pulled straight from the union of the frozen matrix's two
progress rows — the progress states, source/provider/freshness labels, required labels,
accessibility routes, consumer surfaces, and downgrade triggers. Because progress rows
carry source/provider and freshness truth (a job row shows a provider-attributed handoff
and labels sampled or in-flight values), this lane certifies the full six-label set —
`identity`, `state`, `keyboard_route`, `source_provider`, `freshness`, and `reopen_path` —
and carries the frozen source-freshness labels on every row. It is certified across four
posture axes:

- **durable presence** — `durable_reviewable_after_focus_loss` (green),
  `disclosed_reduced_history_retention` (yellow: older completed rows compact into a
  summary sooner while every in-flight job and its recent history stay reviewable after
  focus loss), or `transient_spinner_or_toast_only` (red: durable work is shown only
  through a transient spinner or toast, so progress is lost the moment the user looks
  away).
- **progress attribution** — `actor_phase_action_object_attributed` (green),
  `disclosed_coarse_attribution` (yellow: a grouped batch folds per-job phase into a
  summary while the actor, action affordances, and authoritative-object link stay
  present), or `attribution_or_object_link_missing` (red: a progress row hides its actor /
  phase attribution or drops the link to the authoritative object).
- **grouped history** — `grouped_history_and_blocked_reasons_preserved` (green),
  `disclosed_compacted_history` (yellow: older grouped batches roll up into a digest with
  a reopen path while each blocked/paused reason stays reconstructable — backed by a
  waiver), or `history_or_blocked_reason_lost` (red: a failed batch vanishes with its
  reason or a paused job gives no reason).
- **progress export** — `progress_and_history_reconstructable` (green),
  `disclosed_partial_capture` (yellow: current progress reconstructs while the recent
  job-history chronology is a disclosed partial capture), or
  `progress_state_absent_from_capture` (red: current progress or the recent job-history
  chronology is absent from the support-export capture).

Each row also carries the hard invariant `never_spinner_or_toast_only`; `false` is a
blocker (a job represented only by a transient spinner or toast, with no durable
reopenable row).

## Derived status and the structural lints

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when
it discloses a reduced history retention, a coarse attribution, a compacted grouped
history (backed by a waiver), or a partial support-export capture. It drops to `red` when
any axis reaches its blocked state, a job is spinner-or-toast-only, or its progress states
/ required labels are incomplete. Those structural lints — `progress_states_complete` and
`required_labels_complete` — are what prevent a later progress surface from shipping
without its full queued / running / grouped-batch / paused / succeeded / failed / canceled
/ reopenable-history transition set or its
identity/state/keyboard-route/source-provider/freshness/reopen-path labels. The Rust
validator in `crates/aureline-shell/src/m5_durable_progress_certification` is the
authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_compacted_history`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact certification causes, and the
  blocking findings the lane refuses to ship with.
- **Certification dashboard** — a light projection the shell / activity center / release
  automation reads to auto-narrow a claimed family when its durable-progress proof falls
  out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix
  ref, build id, each family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels —
never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_durable_progress_certification`)
is the only mint-from-truth path for:

- `artifacts/release/m5-durable-progress-certification-proof/packet.json`
- `artifacts/release/m5-durable-progress-certification-proof/dashboard.json`
- `artifacts/release/m5-durable-progress-certification-proof/support_export.json`
- `artifacts/release/m5-durable-progress-certification-proof/matrix.csv`
- `artifacts/shell/m5-durable-progress-certification.md` (this report's rendered companion)
- `fixtures/ui/m5-durable-progress-certification/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-durable-progress-certification.schema.json`](../../schemas/shell/m5-durable-progress-certification.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_durable_progress_certification -- validate
cargo test -p aureline-shell --test m5_durable_progress_certification_fixtures
cargo test -p aureline-shell m5_durable_progress_certification
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_durable_progress_certification --"
$BIN packet         > artifacts/release/m5-durable-progress-certification-proof/packet.json
$BIN dashboard      > artifacts/release/m5-durable-progress-certification-proof/dashboard.json
$BIN support-export > artifacts/release/m5-durable-progress-certification-proof/support_export.json
$BIN csv            > artifacts/release/m5-durable-progress-certification-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-durable-progress-certification.md
$BIN packet         > fixtures/ui/m5-durable-progress-certification/packet.json
$BIN dashboard      > fixtures/ui/m5-durable-progress-certification/dashboard.json
$BIN support-export > fixtures/ui/m5-durable-progress-certification/support_export.json
$BIN compact        > fixtures/ui/m5-durable-progress-certification/compact.txt
```
