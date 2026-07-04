# M5 Execution-Lifecycle Component Consumer Contract (M05-825)

This contract is the first-consumer **adoption** lane over the frozen M5
execution-lifecycle component matrix. Where
[`m5_execution_lifecycle_component_matrix.md`](m5_execution_lifecycle_component_matrix.md)
freezes the reusable run/attempt, input-request, artifact-publish, rerun-review,
and debug-hierarchy families, and the M05-821 – M05-824 primitives resolve their
per-surface truth, this lane proves the **five families are reusable primitives**
— not one task pane, one debug strip, or one provider-specific run view — by
adopting them across the five claimed M5 execution consumer classes.

- **Module:** `crates/aureline-runtime/src/add_shared_task_test_request_database_notebook_preview_ai_publish_and_support_execution_lifecycle_component_consumers/`
- **Schema:** [`schemas/ui/m5-execution-lifecycle-component-consumer.schema.json`](../../schemas/ui/m5-execution-lifecycle-component-consumer.schema.json)
- **Release proof:** [`artifacts/release/m5-execution-lifecycle-component-consumer-proof/`](../../artifacts/release/m5-execution-lifecycle-component-consumer-proof/)
- **Fixtures:** [`fixtures/ui/m5-execution-lifecycle-component-consumers/`](../../fixtures/ui/m5-execution-lifecycle-component-consumers/)

## Consumer classes

| Group | Surfaces |
| --- | --- |
| `task_test` | task-run pane, test explorer |
| `request_database` | request-run pane, database-execution pane |
| `notebook_preview` | notebook execution cell, preview-runtime lane |
| `ai_publish` | AI-mediated run, publish/deploy flow |
| `support_export` | support/export replay, run history / activity center, docs / help center |

## Component families and their canonical sources

Each consumer row points back to exactly **one** canonical family — the
primitive schema plus its release-proof packet — rather than cloning
surface-local run/debug prose.

| Family | Primitive | Canonical schema |
| --- | --- | --- |
| `run_attempt_header` | M05-821 | `schemas/ui/m5-run-attempt-header.schema.json` |
| `input_request_prompt` | M05-822 | `schemas/ui/m5-input-request-artifact-publish.schema.json` |
| `artifact_publish_row` | M05-822 | `schemas/ui/m5-input-request-artifact-publish.schema.json` |
| `rerun_review` | M05-823 | `schemas/ui/m5-rerun-comparison-sheet.schema.json` |
| `debug_hierarchy` | M05-824 | `schemas/ui/m5-debug-session-hierarchy.schema.json` |

The input-request prompt and artifact-publish row are the two halves of the same
M05-822 execution-interaction primitive, so they share one canonical schema and
one canonical packet.

## Preserved label families

Every consumer — full-interactive or narrowed — keeps these controlled label
families identical, and the union of all rows must cover the set:

- `run_attempt_identity` — run identity stays distinct from attempt identity.
- `outcome_state` — queued / preparing / running / waiting-input /
  partially-complete / passed / failed / cancelled / stale-output stay stable.
- `rerun_context_difference` — exact-vs-current-context differences before dispatch.
- `artifact_lineage_retention` — produced-run lineage and retention truth.
- `debug_control_posture` — launch / attach / core / replay / inspect-only and
  live-vs-captured control.

The same degraded-state vocabulary (`queued`, `waiting_input`,
`partially_complete`, `stale_output`, `cancelled`) is carried on every row.

## Acceptance criteria

- **AC1 — one canonical family, reused.** Every row's
  `canonical_family_schema_ref` and a `canonical_packet_ref` match the declared
  `component_family`, `references_canonical_not_local_prose` is true, and at
  least one family is adopted by two or more consumer groups
  (`families_reused_across_groups >= 1`). All five families and all five groups
  are covered.
- **AC2 — state parity across the primary UI and history / activity / support /
  export.** Every row preserves the controlled label families and degraded-state
  vocabulary; a narrower consumer (read-only, inspect-only, compare-only,
  export-only, policy-blocked) discloses the reduction with a reduced-capability
  banner whose `capability_state` matches its authority mode, and carries a
  handoff note when it punts to another surface. `label_parity` is
  `disclosed_narrowed` for narrowed rows, never `renamed_or_dropped`.
- **AC3 — AI, preview, publish, and docs / help cite the same primitives.** A
  docs / help consumer references the canonical families
  (`docs_help_reference_present`), and the AI, preview, and publish lanes adopt
  the same run/attempt, artifact, rerun, and debug primitives users saw in the
  original execution UI.

## Regenerating the artifacts

The seeded builder
(`seeded_m5_execution_lifecycle_component_consumers_packet`) is the one source
of truth shared by the tests, the example dump, and the checked-in support
export. Regenerate with:

```sh
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_component_consumers -- support \
  > artifacts/release/m5-execution-lifecycle-component-consumer-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_component_consumers -- csv \
  > artifacts/release/m5-execution-lifecycle-component-consumer-proof/matrix.csv
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_component_consumers -- summary \
  > artifacts/release/m5-execution-lifecycle-component-consumer-proof/report.md
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export
drifts from the builder.
