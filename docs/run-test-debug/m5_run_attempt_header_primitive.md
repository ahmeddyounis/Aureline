# M5 Run/Attempt-Header Primitive

Status: stable (B96 / W96, task M05-821)

The run/attempt-header primitive is the one reusable way every claimed M5 execution
surface renders **who ran what, on which attempt, against which target, in what
state** — before any action continues. It *narrows* the `run_attempt_header` family
of the frozen
[M5 execution-lifecycle component matrix](../../schemas/ui/m5-execution-lifecycle-component-matrix.schema.json)
(task M05-820) — plus the attempt selector that family implies — into one working
**resolver**, so no task pane, test runner, request view, notebook, AI run, publish
flow, or preview flow ever invents provider- or pane-local status strings.

- Resolver: `resolve_run_attempt_header(&M5RunAttemptHeaderInput) -> Result<M5ResolvedRunAttempt, M5RunAttemptResolutionError>`
- Crate module: `aureline-runtime::implement_the_m5_run_attempt_header_and_attempt_selector_primitive`
- Boundary schema: [`schemas/ui/m5-run-attempt-header.schema.json`](../../schemas/ui/m5-run-attempt-header.schema.json)
- Release proof: [`artifacts/release/m5-run-attempt-header-primitive-proof/`](../../artifacts/release/m5-run-attempt-header-primitive-proof/)

## One context, four projections

One run-and-attempt context (`M5RunAttemptHeaderInput`) projects onto four surfaces
that share **one header identity, one run ref, one attempt ref, and one outcome-state
label**:

| Projection | Type | Role |
| --- | --- | --- |
| Header | `M5ResolvedRunAttemptHeader` | Run label, initiator, target, boundary, context summary, age, outcome, truth class, admission/queue disclosure. |
| Attempt selector | `M5ResolvedAttemptSelector` | Every attempt of the *same* run, current attempt flagged, ordered by ordinal. |
| CLI / headless line | `M5ResolvedCliHeaderLine` | A deterministic single line in the same run/outcome/truth/boundary/admission vocabulary. |
| Support-export projection | `M5ResolvedRunAttemptExport` | The run/attempt IDs, ordinal, outcome, truth class, and boundary carried into any support packet. |

Run identity and attempt identity stay **distinct** everywhere a retry, rerun, or
resume can occur — history, activity center, support exports, and companion
summaries included. The run ref never equals the attempt ref, and the resolver
rejects a collapsed identity outright.

## Acceptance criteria

- **AC1 — one run with multiple attempts is distinguishable from multiple separate
  runs without leaving the surface.** The header keeps the run and attempt refs
  distinct; the attempt selector lists every attempt of the same run (`all_attempts_share_run`),
  with the current attempt flagged, so a retried run never reads as a different run.
  Proven by `M5ResolvedRunAttempt::distinguishes_attempts_from_runs`.
- **AC2 — header state labels stay consistent across surfaces.** The outcome-state
  label is derived from one closed outcome vocabulary (`Queued`, `Preparing`,
  `Running`, `Waiting for input`, `Partially complete`, `Passed`, `Failed`,
  `Cancelled`, `Stale output`), so the same run outcome reads identically across
  task, test, request, notebook, AI-mediated execution, publish, and preview flows.
  Proven by `M5ResolvedRunAttempt::state_labels_consistent`.
- **AC3 — exported evidence and support packets preserve the same run/attempt IDs and
  visible states shown in-product.** The support-export projection carries the run
  ref, attempt ref, attempt ordinal, outcome, and truth class byte-for-byte with the
  header, and declares the mandatory export fields. Proven by
  `M5ResolvedRunAttempt::export_preserves_ids_and_states`.

## Honesty rules the resolver enforces

- **Run ≠ attempt.** An empty or collapsed run/attempt identity is rejected
  (`RunAttemptIdentityCollapsed`). Sibling attempts must be complete, distinct from
  the current attempt and the run, and distinct from one another.
- **Queue truth.** A `queued` outcome must name a non-`immediate` admission-control
  class (`QueuedWithoutAdmissionReason`); any admission-queued run must carry a queue
  reason (`QueueReasonMissing`). Queue reason and admission-control class are surfaced
  in the same header vocabulary the CLI/headless line renders.
- **Captured versus live.** An actively executing outcome (`preparing` / `running` /
  `waiting_input`) must be shown as live truth (`ActiveOutcomeNotLive`); a stale
  output must never claim live control (`StaleOutputClaimsLive`).
- **Export-safe.** Raw run logs, provider cursors, credentials, and raw event
  payloads never cross the boundary; only opaque refs, typed class tokens, booleans,
  and redacted labels are carried. Obvious secret material is rejected
  (`ForbiddenMaterial`), and a degraded block must carry a precise, non-generic label.

## Surface matrix

The checked-in packet (`seeded_m5_run_attempt_header_packet`) proves the primitive
across all ten execution flows: task-run pane, test-run pane, request-run pane,
notebook execution, AI-mediated execution, publish flow, preview flow, history /
activity center, support / export replay, and companion summary. Each row binds the
shared contract, keeps the mandatory export fields, and cites the canonical schema
and frozen component-matrix contract. The support export, matrix CSV, and Markdown
report are regenerated from the same seeded builder via
`cargo run -p aureline-runtime --example dump_m5_run_attempt_header_primitive`.
