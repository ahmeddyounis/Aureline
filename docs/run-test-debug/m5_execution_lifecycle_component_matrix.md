# M5 Execution-Lifecycle Component Matrix

Task: **M05-820** — Freeze the M5 run-attempt, input-request, artifact-publish,
rerun-review, and debug-hierarchy component matrix.

This contract freezes the reusable **execution-lifecycle component family** so
Milestone 5 stops depending on feature-local run/debug chrome and instead ships one
canonical component family for execution identity, retry scope, and
captured-versus-live control truth.

- **Schema:** [`schemas/ui/m5-execution-lifecycle-component-matrix.schema.json`](../../schemas/ui/m5-execution-lifecycle-component-matrix.schema.json)
- **Rust module:** `aureline_runtime::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`
- **Canonical support export (`include_str!`):** [`artifacts/release/m5-execution-lifecycle-component-proof/support_export.json`](../../artifacts/release/m5-execution-lifecycle-component-proof/support_export.json)
- **Design summary:** [`artifacts/design/m5-execution-lifecycle-component-matrix.md`](../../artifacts/design/m5-execution-lifecycle-component-matrix.md)
- **Fixtures:** [`fixtures/ui/m5-execution-lifecycle-components/`](../../fixtures/ui/m5-execution-lifecycle-components/)

## Reusable component families

The matrix defines every reusable execution-lifecycle primitive. Later M5 rows
reference one canonical family by name instead of restating run/debug identity truth
in feature-local prose.

| Family | Purpose |
| --- | --- |
| `run_attempt_header` | Frames run-versus-attempt identity and the stable run outcome. |
| `input_request_prompt` | Discloses timeout / approval consequences of an input request. |
| `artifact_publish_row` | Keeps producing-run lineage and retention truth for a produced artifact. |
| `rerun_comparison_sheet` | Discloses exact-versus-current-context differences before dispatch. |
| `debug_session_header` | Frames launch / attach / core / replay / inspect-only, live-versus-captured, and the local / remote / container / managed boundary. |
| `thread_process_tree` | Discloses live-versus-captured process/thread hierarchy truth. |
| `dump_crash_artifact_card` | Keeps producing-run lineage, retention, and symbolication for a dump/crash artifact. |

## State vocabularies

- **Truth class** (`truth_mode`): `live`, `captured`, `imported`, `planned`,
  `provider_reported`. Captured evidence never reads as live control.
- **Run outcome** (`run_attempt_header.outcome`): `queued`, `preparing`, `running`,
  `waiting_input`, `partially_complete`, `passed`, `failed`, `cancelled`,
  `stale_output` — stable across UI, CLI, and export.
- **Execution boundary** (`locality`): `local`, `remote`, `container`, `managed`.
- **Retention** (`retention`): `retained_durable`, `expires_scheduled`,
  `ephemeral_session_only`, `evicted_recoverable`, `evicted_gone`.
- **Input consequence**: `timeout_cancels_run`, `timeout_applies_default`,
  `requires_approval`, `blocks_until_answered`, `dismiss_leaves_waiting`.
- **Rerun context**: `exact_replay`, `current_context`, `modified_selection`,
  `modified_environment`.
- **Debug session mode**: `launch`, `attach`, `core`, `replay`, `inspect_only`.
- **Symbolication**: `symbolicated`, `partial_symbols`, `unsymbolicated`,
  `symbols_unavailable`.

## Honesty rules (guardrails)

Every row is validated against these invariants:

1. **Run identity and attempt identity stay distinct** — a run/attempt header never
   collapses the run and the attempt it renders.
2. **Outcomes remain stable across UI / CLI / export** — a stale output never reads
   as a live run.
3. **Produced artifacts never lose producing-run lineage or retention truth** — an
   artifact-publish row and a dump/crash card always name the producing run and
   disclose retention even when evicted.
4. **Rerun controls disclose exact-versus-current-context differences before
   dispatch** — never after.
5. **Debug hierarchy / cards keep launch / attach / core / replay / inspect-only,
   live-versus-captured, and local / remote / container / managed explicit** — a
   captured replay is never presented as live control.

A row that narrows below its full capability carries a typed `degraded` block with a
precise, non-generic label and a stable downgrade trigger.

## Boundary safety

Raw run logs, raw stdout/stderr bytes, raw crash dumps, provider cursors,
credentials, and raw event payloads never cross this boundary. The packet carries
only typed class tokens, opaque run / attempt / artifact / evidence refs, booleans,
and redacted labels, so support and diagnostics exports can reconstruct exactly what
a component would have shown without leaking source or live payloads.

## Acceptance criteria coverage

- A checked-in matrix defines every reusable execution-lifecycle primitive, its
  states, and its export / assistive parity expectations (the seven families above,
  each with a `required_labels` set and export-safe / assistive-ready parity).
- Later M5 rows reference one canonical component family (`consumer_projection.later_rows_reference_one_canonical_family`).
- Qualification and release packets have a stable anchor for execution-lifecycle
  component truth (the canonical support export under
  `artifacts/release/m5-execution-lifecycle-component-proof/`).
