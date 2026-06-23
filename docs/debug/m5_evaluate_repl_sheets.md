# M5 evaluate/REPL sheets and console emissions

This contract materializes two governed debugger object families that the
[M5 debug-contracts matrix](./m5_debug_contracts.md) names — the **evaluate
request/result** and the **console emission** — as concrete, typed, serde-serializable
[`EvaluateRecord`] and [`ConsoleEmission`] records, each carrying one canonical pill. It
is the canonical M5 source every debugger evaluate pane, REPL, notebook console, replay
inspector, and exported support packet reads to show *whether an expression is treated as
pure, unknown, or may-mutate* before it is dispatched and after a result returns,
*whether evaluation was approved, withheld, denied, blocked, or expired*, and *whether a
console line is interactive user input or target output* that is live or replayed.
Debugger evaluate/REPL surfaces consume these records directly instead of letting a
mutation-capable inspection hide inside debugger chrome.

Authoritative product anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.1–§7.6.10.4 and §9.46 on
  debug launch/session, breakpoints, variables/watches, evaluate side-effect governance,
  chronology capture, replay, and notebook-debug parity.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §14.5 on debug session headers, frame
  mapping, variables/watch panes, evaluate/REPL review, and dump/source-map truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` `FR-DEBUG-001` and the debug
  surface rules on stable breakpoints, variables, stack views, chronology cues, and
  artifact-linked evidence.

This lane composes with the live breakpoint/call-stack/variables/watch/evaluate truth
already frozen in
`crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/`
and the console event bus in `crates/aureline-runtime/src/m5_task_event_envelope_bus/`;
it keeps the *reviewed evaluate-governance and console-truth model* every surface reads.

## Vocabulary alignment

This module *refines* the matrix evaluate-purity family. The matrix names the family with
`evaluate_side_effect_free` / `evaluate_unknown_side_effects` / `evaluate_mutating` /
`evaluate_blocked_inspect_only`; this lane pins:

- the [`EvaluatePurityClass`] vocabulary `pure` ↔ `evaluate_side_effect_free`, `unknown`
  ↔ `evaluate_unknown_side_effects`, `may_mutate` ↔ `evaluate_mutating`; and
- the matrix `evaluate_blocked_inspect_only` state as the
  [`ApprovalDisposition::Blocked`] disposition against an inspect-only context.

Purity and approval stay **orthogonal**: purity names the side-effect class, the
disposition names the approval posture, and a single posture pill derives the governance
flags from both plus the context authority.

## The evaluate record

The module
[`crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs`](../../crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs)
owns the typed records. An `EvaluateRecord` carries:

- a stable `evaluate_id` and an opaque `expression_digest` (never raw expression source),
  plus an `EvaluateRedactionClass` for the expression itself;
- an `ExpressionContext` — the `session_id`, optional `thread_id` / `frame_id`, an
  `EvaluateContextScope` (`frame`, `thread`, `global_scope`, `session`, `repl`), an
  `EvaluateContextAuthority` (`live_mutable`, `inspect_only`), and an optional
  `notebook_cell_ref` or `replay_capture_ref`;
- an `EvaluatePurityClass` (`pure`, `unknown`, `may_mutate`) and an `ApprovalDisposition`
  (`not_required`, `pending`, `approved`, `denied`, `blocked`, `expired`);
- the canonical `EvaluatePosturePill`;
- an `ActorLineage` — the opaque `requested_by_ref`, an `EvaluateActorClass` (`human`,
  `ai_agent`, `automation`), the `origin_surface`, and optional `reviewed_by_ref` /
  `on_behalf_of_ref`; and
- an optional `EvaluateResult`, present only when dispatch was permitted.

The posture pill's flags are **derived**, never asserted:

```
approval_required          = purity in {unknown, may_mutate}
discloses_side_effect_risk = purity in {unknown, may_mutate}
permits_dispatch           = disposition in {not_required, approved}
requires_review_affordance = approval_required && disposition != approved
is_blocked                 = disposition == blocked
blocked_by_inspect_only    = is_blocked && context_authority == inspect_only
```

So an evaluate/REPL surface tells the user whether an expression is pure, unknown, or
may-mutate before dispatch and after a result returns, and a mutation-capable inspection
can never hide under a harmless-inspect label.

### Approval, blocking, and lineage

- **Approval is never bypassed.** A pure expression carries `not_required`; an unknown or
  may-mutate expression must carry a real disposition. A pending, denied, blocked, or
  expired evaluation never permits dispatch, and a **withheld request carries no result**.
- **Inspect-only contexts block effectful evaluation.** An effectful expression issued
  against an `inspect_only` context (a core file or replay capture) never permits dispatch
  — it is blocked rather than silently mutating a recording.
- **Blocked, denied, and expired states are preserved.** All three are materialized and
  none permit dispatch, so a non-cleared approval state is never lost in UI, CLI, or
  support packets.
- **Actor lineage is preserved.** Every record names its requesting actor and class; an
  approval-cleared effectful evaluation names its reviewer.

### The evaluate result

An `EvaluateResult` carries an `EvaluateOutcome` (`completed`, `no_value`,
`raised_error`), a reviewable `result_summary`, an explicit `side_effect_note`, an
`observed_mutation` flag, and an `EvaluateRedactionClass`. A `result_repr_digest` is
present exactly when a value body is present — a completed, non-redacted outcome — so a
redacted or void or errored result never implies a readable value.

## The console emission — input/output separation

One `ConsoleEmission` struct materializes the console-emission family. Each carries:

- a stable `emission_id`, a `sequence` for stable ordering, and a `ConsoleStreamClass`
  (`stdin`, `evaluate_input`, `stdout`, `stderr`, `debug_console`, `evaluate_result`);
- a `ConsoleDirection` (`user_input`, `target_output`) **derived from the stream class**,
  and a `ConsoleLiveness` (`live`, `replayed_capture`);
- the `session_ref` plus optional `thread_ref` / `frame_ref` / `notebook_cell_ref` /
  `replay_capture_ref`, and an optional `linked_evaluate_id`;
- an opaque `body_digest` (present only when not redacted), an `EvaluateRedactionClass`,
  a `replayable` flag; and
- the canonical `ConsoleEmissionPill`.

The pill's flags are **derived** from the stream class, liveness, and redaction:

```
direction        = stream.direction()           // stdin/evaluate_input → user_input
is_user_input    = direction == user_input
is_live          = liveness == live
is_replayed      = !is_live
requires_disclosure = is_replayed
is_redacted      = redaction != not_redacted
body_present     = !is_redacted
```

So console history and export packets distinguish interactive input from target output,
never present a replayed line as live, and preserve redaction review rather than
flattening one transcript.

## Contract rules (frozen invariants)

The canonical set computes each invariant's `holds` flag from the built records; an
inconsistent edit flips an invariant and fails the freeze gate.

- **`evaluate.one_canonical_posture_pill`** — every evaluation carries one pill whose
  tokens come from the frozen vocabulary and whose flags equal their derivation.
- **`evaluate.purity_vocabulary_complete`** — pure, unknown, and may-mutate are all
  materialized.
- **`evaluate.disposition_vocabulary_complete`** — not-required, pending, approved,
  denied, blocked, and expired are all materialized.
- **`evaluate.side_effect_risk_disclosed_before_dispatch`** — every unknown or may-mutate
  expression discloses its risk and requires approval; a pure one never claims a risk.
- **`evaluate.unknown_or_mutating_never_runs_unless_approved`** — an expression that
  requires approval and is not approved never permits dispatch.
- **`evaluate.withheld_request_carries_no_result`** — a result is present only when
  dispatch was permitted.
- **`evaluate.blocked_denied_expired_states_preserved`** — the blocked, denied, and
  expired states are all materialized and none permit dispatch.
- **`evaluate.inspect_only_context_blocks_effectful_evaluation`** — an effectful
  expression against an inspect-only context is blocked rather than dispatched.
- **`evaluate.actor_lineage_preserved`** — every evaluation names its requester and class,
  and every approval-cleared effectful evaluation names its reviewer.
- **`evaluate.no_raw_expression_or_value_body`** — every evaluation carries an opaque
  expression digest, and every redacted result withholds its body.
- **`console.interactive_input_and_target_output_separated`** — both directions are
  materialized and each emission's direction matches its stream class.
- **`console.one_canonical_emission_pill`** — every emission carries one pill whose tokens
  come from the frozen vocabulary and whose flags equal their derivation.
- **`console.replayed_never_shown_as_live`** — a replayed line always discloses and is
  never shown as live.
- **`console.redaction_review_preserved`** — every redacted emission withholds its body
  and is marked redacted.
- **`console.session_and_evaluate_linkage_preserved`** — every emission carries its
  session linkage, and an evaluate-linked emission resolves to an evaluation in the set.
- **`set.redaction_vocabulary_complete`** — not-redacted, secret, personal-data, and
  policy-withheld are all materialized across expressions, results, and console bodies.
- **`set.export_retains_evaluate_and_console_state`** — every record retains its typed
  tokens and cites an export-safe proof packet.

## First consumers

- core debugger evaluate pane and REPL / debug console;
- notebook debug surface (per-cell evaluate and console output area);
- profiler / trace / replay workspace and replay inspector;
- incident / crash review and exported transcripts;
- support export / escalation packets; and
- AI context, composer, and tool-call evidence.

## Checked-in artifacts

- Spec module:
  [`crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs`](../../crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs)
- Boundary schema:
  [`schemas/debug/m5_evaluate_repl_sheets.schema.json`](../../schemas/debug/m5_evaluate_repl_sheets.schema.json)
- Published fixture:
  [`fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json`](../../fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json)
- Evidence note:
  [`artifacts/debug/m5_evaluate_repl_sheets.md`](../../artifacts/debug/m5_evaluate_repl_sheets.md)
- Freeze gate:
  [`crates/aureline-debug/tests/m5_evaluate_repl_sheets.rs`](../../crates/aureline-debug/tests/m5_evaluate_repl_sheets.rs)

## Regenerating

```sh
cargo run -p aureline-debug --example dump_m5_evaluate_repl_sheets \
  > fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json
cargo test -p aureline-debug
```

[`EvaluateRecord`]: ../../crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs
[`ConsoleEmission`]: ../../crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs
