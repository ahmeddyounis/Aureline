# M5 evaluate/REPL sheet and console-emission evidence

This set is the checked-in proof path for Aureline's typed M5 evaluate/REPL review sheets
and console emissions: the canonical records every debugger evaluate pane, REPL, notebook
console, replay inspector, and exported support packet reads to show whether an
expression is treated as pure, unknown, or may-mutate before dispatch and after a result
returns, whether evaluation was approved, withheld, denied, blocked, or expired, and
whether a console line is interactive user input or target output that is live or
replayed. It materializes the evaluate-request/result and console-emission families named
by the [M5 debug-contracts matrix](./m5_debug_contracts.md).

The published set is
[`fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json`](../../fixtures/debug/m5_evaluate_repl_sheets/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_evaluate_repl_sheets/mod.rs` by the gate at
`crates/aureline-debug/tests/m5_evaluate_repl_sheets.rs`.

## Materialized evaluations

| Evaluate | Purity | Disposition | Context | Dispatched | Note |
|---|---|---|---|---|---|
| `debug.evaluate:pure_frame_read:0001` | pure | not_required | frame / live | yes (completed) | clean harmless read |
| `debug.evaluate:pure_global_error:0002` | pure | not_required | global / live | yes (raised_error) | error, no value body |
| `debug.evaluate:unknown_session_pending:0003` | unknown | pending | session / live | no | held for review |
| `debug.evaluate:unknown_notebook_approved:0004` | unknown | approved | repl / notebook | yes (completed) | reviewer named |
| `debug.evaluate:mutate_frame_approved:0005` | may_mutate | approved | frame / live | yes (no_value) | mutation observed |
| `debug.evaluate:mutate_thread_denied:0006` | may_mutate | denied | thread / live | no | secret expression withheld |
| `debug.evaluate:mutate_replay_blocked:0007` | may_mutate | blocked | session / inspect-only | no | inspect-only block |
| `debug.evaluate:unknown_repl_expired:0008` | unknown | expired | repl / live | no | approval lapsed |
| `debug.evaluate:mutate_frame_redacted:0009` | may_mutate | approved | frame / live | yes (completed) | secret result withheld |

The set materializes the full purity vocabulary (pure, unknown, may_mutate), the full
disposition vocabulary (not_required, pending, approved, denied, blocked, expired), every
outcome (completed, no_value, raised_error), every actor class (human, ai_agent,
automation), every context scope, both context authorities, a reviewer-named approval, an
inspect-only block, and secret-redacted expression and result bodies.

## Materialized console emissions

| Emission | Direction | Stream | Liveness | Body | Note |
|---|---|---|---|---|---|
| `debug.console:eval_input_pure:0001` | user_input | evaluate_input | live | present | linked to `pure_frame_read` |
| `debug.console:eval_result_pure:0002` | target_output | evaluate_result | live | present | linked result echo |
| `debug.console:stdin_live:0003` | user_input | stdin | live | present | interactive input |
| `debug.console:stdout_live:0004` | target_output | stdout | live | present | target output |
| `debug.console:stderr_pii:0005` | target_output | stderr | live | withheld | personal-data redaction |
| `debug.console:debug_replayed_secret:0006` | target_output | debug_console | replayed | withheld | replayed, secret withheld |
| `debug.console:stdout_replayed_notebook:0007` | target_output | stdout | replayed | withheld | replayed in notebook, policy-withheld |
| `debug.console:eval_result_mutate:0008` | target_output | evaluate_result | live | present | linked to `mutate_frame_approved` |

The set materializes both directions (user_input, target_output), every stream class, both
liveness states (live, replayed_capture), every redaction class, evaluate-linked input and
result emissions, and notebook- and replay-context emissions.

## Proof claims

| Claim | Evidence |
|---|---|
| Evaluate/REPL surfaces tell the user whether an expression is pure, unknown, or may-mutate before dispatch and after a result returns | invariants `evaluate.purity_vocabulary_complete` + `evaluate.one_canonical_posture_pill` + `evaluate.side_effect_risk_disclosed_before_dispatch` |
| No claimed M5 debug surface can silently run unknown or mutating evaluation under a harmless-inspect label | invariant `evaluate.unknown_or_mutating_never_runs_unless_approved` + the `unknown_or_mutating_evaluation_never_runs_unless_approved` test |
| A withheld, denied, blocked, or expired approval state never permits dispatch and is preserved in UI, CLI, and support packets | invariants `evaluate.withheld_request_carries_no_result` + `evaluate.blocked_denied_expired_states_preserved` + the `blocked_denied_and_expired_states_are_preserved` test |
| Debugger evaluate never bypasses approval/redaction posture; an inspect-only context blocks effectful evaluation | invariant `evaluate.inspect_only_context_blocks_effectful_evaluation` + the `running_an_effectful_eval_on_an_inspect_only_context_fails_validation` test |
| Actor lineage names who requested and who reviewed an evaluation | invariant `evaluate.actor_lineage_preserved` + the `approved_effectful_evaluations_name_a_reviewer` test |
| No raw expression source or redacted value body crosses the boundary | invariant `evaluate.no_raw_expression_or_value_body` + the `no_raw_expression_text_and_redacted_results_withhold_bodies` test |
| Console history and export packets distinguish interactive input from target output rather than flattening them into one transcript | invariant `console.interactive_input_and_target_output_separated` + the `console_separates_user_input_from_target_output` test |
| A replayed console line is never shown as live | invariant `console.replayed_never_shown_as_live` + the `replayed_console_lines_are_never_shown_as_live` test |
| Console export preserves redaction review rather than flattening it | invariants `console.redaction_review_preserved` + `set.redaction_vocabulary_complete` + the `redacted_console_emissions_withhold_their_bodies` test |
| Support/export packets retain evaluate and console state rather than flattening them into rendered chrome | invariant `set.export_retains_evaluate_and_console_state` + the `fixture_round_trips_and_is_export_safe` test |
| Every cited proof packet and producer exists on disk | the `every_proof_packet_and_producer_exists_on_disk` freeze-gate test |

## Verification

```sh
cargo test -p aureline-debug
```
