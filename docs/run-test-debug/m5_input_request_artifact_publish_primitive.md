# M5 Input-Request / Artifact-Publish Primitive

Status: stable (M05-822, batch B96)

The reusable execution-lifecycle component matrix
([`m5-execution-lifecycle-component-matrix.schema.json`](../../schemas/ui/m5-execution-lifecycle-component-matrix.schema.json),
frozen in M05-820) *freezes* the run/attempt/input-request/artifact-publish/rerun/debug
component families as a governed contract. This primitive *narrows* two of those
families — `input_request_prompt` and `artifact_publish_row` — into one working
resolver with a real, tested implementation, the sibling of the run/attempt-header
primitive ([`m5_run_attempt_header_primitive.md`](m5_run_attempt_header_primitive.md),
M05-821).

A single bounded **execution interaction** — one run-and-attempt context that may be
requesting input and is producing artifacts while it is still live — projects onto four
surfaces that share one interaction identity, one run identity, and one attempt
identity:

- a typed **input-request prompt** (`M5ResolvedInputRequestPrompt`),
- a set of **artifact-publish rows** (`M5ResolvedArtifactPublishRow`),
- a **CLI / headless line** (`M5ResolvedInteractionCliLine`), and
- a **support-export projection** (`M5ResolvedInteractionExport`).

The resolver is
`resolve_execution_interaction(&M5ExecutionInteractionInput) -> Result<M5ResolvedExecutionInteraction, M5ExecutionInteractionError>`
in
[`crates/aureline-runtime/src/implement_the_m5_input_request_prompt_and_artifact_publish_row_primitive`](../../crates/aureline-runtime/src/implement_the_m5_input_request_prompt_and_artifact_publish_row_primitive).
The boundary schema is
[`schemas/ui/m5-input-request-artifact-publish.schema.json`](../../schemas/ui/m5-input-request-artifact-publish.schema.json).

## Typed input requests

Prompts are typed by `M5InputRequestKind`: `plain_text`, `secret_input`,
`file_path_selection`, `approval`, `choice`, and `device_browser_handoff`. Every prompt
declares its timeout / approval `M5InputConsequence` (reused from the frozen matrix) and
its `M5InputRequestDisposition` — what actually happened: `awaiting_response`,
`continued`, `timed_out`, `dismissed`, or `cancelled`.

The resolver derives the user-visible `M5InputResultPosture` from the disposition and
the consequence:

| disposition | consequence | result posture |
| --- | --- | --- |
| `awaiting_response` | any | `awaiting_response` |
| `continued` | any | `run_proceeds` |
| `cancelled` | any | `run_cancelled` |
| `timed_out` / `dismissed` | `timeout_cancels_run` | `run_cancelled` |
| `timed_out` / `dismissed` | `timeout_applies_default` | `run_proceeds_with_default` |
| `timed_out` / `dismissed` | `requires_approval` / `blocks_until_answered` / `dismiss_leaves_waiting` | `run_blocked_waiting` |

**AC1 — dismissed or timed-out requests no longer behave like silent failures.** Every
negative disposition (`timed_out`, `dismissed`, `cancelled`) resolves to an explicit,
attributable posture — cancelled, default-applied, or blocked-and-waiting — never a
silent stall. A timeout-governed request must carry a resolvable deadline, and a
`timeout_applies_default` request must name its default.

## Produced-object lineage, freshness, and retention

Each artifact row records its `artifact_ref`, its `producing_run_ref` and
`producing_attempt_ref`, the producing-step label, the `M5ArtifactKind` (`report`,
`trace`, `preview_endpoint`, `bundle`, `imported_provider_artifact`, `diagnostic_log`),
the `M5ArtifactFreshness`, the `M5RetentionClass` (reused), the `M5ArtifactTrustClass`,
and open / export action refs.

**AC2 — produced artifacts remain attributable after the live pane clears.** Every
artifact's producing run/attempt refs must match the interaction, so lineage survives
even when retention has evicted the bytes (`evicted_recoverable` / `evicted_gone`) or the
activity-center history has compressed. A broken producing ref is rejected
(`artifact_lineage_broken`); an evicted-gone artifact may still be exported via lineage
but no longer offers an open action.

**AC3 — users can tell whether an artifact is live, buffered, imported, sampled, or
provider-supplied before opening or exporting it.** `M5ArtifactFreshness` covers exactly
those five classes, disclosed on every row alongside an open / export action. A
`live`-freshness artifact must be produced by an actively executing run
(`live_artifact_from_inactive_run` is rejected).

## Redaction

Raw prompt bytes, secret values, artifact bytes, provider cursors, credentials, and raw
event payloads never cross this boundary. The resolver carries only opaque refs, typed
class tokens, booleans, and redacted labels; the packet's `validate()` re-scans the
export-safe JSON for forbidden material.

## Checked-in proof

The canonical packet is built by `seeded_m5_execution_interaction_packet()` and emitted
by the `dump_m5_input_request_artifact_publish_primitive` example. Regenerate:

```sh
cargo run -p aureline-runtime --example dump_m5_input_request_artifact_publish_primitive -- support \
  > artifacts/release/m5-input-request-artifact-publish-primitive-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_input_request_artifact_publish_primitive -- csv \
  > artifacts/release/m5-input-request-artifact-publish-primitive-proof/matrix.csv
cargo run -p aureline-runtime --example dump_m5_input_request_artifact_publish_primitive -- summary \
  > artifacts/release/m5-input-request-artifact-publish-primitive-proof/report.md
```

`current_stable_m5_execution_interaction_export()` reads the checked-in support export
via `include_str!` and re-validates it; `checked_support_export_matches_builder` asserts
it stays byte-aligned with the in-crate builder.
