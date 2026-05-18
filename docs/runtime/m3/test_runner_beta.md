# Beta Test Runner

This document is the reviewer-facing landing page for the beta promote of the
test runner: tree views, inline editor results, rerun-last parity, and
structured artifact identity. The machine-readable boundary lives at
[`/schemas/testing/test_runner_beta.schema.json`](../../../schemas/testing/test_runner_beta.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/testing/`](../../../crates/aureline-runtime/src/testing/).
The promoted item/session/attempt ledger lives at
[`test_identity_beta.md`](test_identity_beta.md) and
[`/crates/aureline-runtime/src/testing_identity/`](../../../crates/aureline-runtime/src/testing_identity/).
The shell consumer lives at
[`/crates/aureline-shell/src/test_runner_beta/`](../../../crates/aureline-shell/src/test_runner_beta/).

The beta promise:

- discovery, run, rerun, and inspection share **one** typed identity model
  built on the canonical alpha test-attempt records;
- the **test tree** (workspace → file → case rows), the **inline editor
  result** marker, and the **rerun-last** command all quote the same
  `canonical_test_item_ref`, the same `selector_ref`, and the same
  `latest_attempt_ref` for any given case;
- a [`TestRunnerBetaCoverageManifest`](../../../crates/aureline-runtime/src/testing/mod.rs)
  pins the closed framework set the beta lane claims today (pytest only).
  Beta rows that touch a framework outside the manifest are reported as
  unclaimed instead of being silently rendered;
- artifacts produced by an attempt carry a typed
  [`TestArtifactKind`](../../../crates/aureline-runtime/src/testing/mod.rs)
  and quote the same identity, so support exports can correlate evidence
  end to end.

## Coverage manifest

| Framework | Wedge | Rerun lane | Rerun-last command | Surfaces |
| --- | --- | --- | --- | --- |
| `pytest` | `test` | `test` | `cmd:test.rerun_last` | `launch_wedge`, `test_tree`, `editor_inline`, `cli_output`, `support_export` |

The closed artifact-kind vocabulary every framework is allowed to publish:

| Kind | Use |
| --- | --- |
| `run_report` | Structured run report (xunit / pytest report style) |
| `coverage_report` | Structured coverage report |
| `snapshot_diff` | Snapshot or golden diff produced by the attempt |
| `log_slice` | Captured stdout / stderr slice retained on a governed artifact rail |
| `raw_event_envelope` | Retained raw adapter envelope reference |
| `debug_trace` | Debug trace recorded from a test attempt |
| `ai_suggestion` | AI-generated suggestion attached to the attempt |

Adding a framework or an artifact kind is a vocabulary change that MUST update
the canonical schema, this doc, and the coverage manifest fixture together.

## Tree, inline, and rerun parity

For each pytest case the beta layer projects three rows, each carrying the
same canonical test-item ref and the same test session ref:

1. a `test_runner_beta_tree_row_record` of kind `test_case`, parented at the
   `test_file` row that owns the source path;
2. a `test_runner_beta_inline_row_record` carrying the editor decoration the
   shell renders next to the case;
3. a `test_runner_beta_rerun_parity_record` binding the tree row id, the
   inline row id, the latest attempt ref, and the rerun-last lane.

The parity record carries a closed `agreement_state` vocabulary:

| State | Meaning |
| --- | --- |
| `rows_agree` | Tree row, inline row, and the rerun-last command all quote the same canonical test-item identity, the same session ref, and the same lane. |
| `rerun_lane_unset` | Tree and inline agree on identity, but `RerunLastLoop` has no remembered launch for this lane yet. The `cmd:test.rerun_last` command is still keyboard-reachable and reports `unavailable` until a first attempt has been remembered. |
| `surface_disagreement_requires_review` | At least one consumer surface disagrees on identity (different canonical id, different session ref, drifted tree-row pointer, or a rerun-last launch on the wrong lane). Dispatch MUST be re-authorised. |

A beta row may only render an "auto-rerun" affordance when its parity row is
in `rows_agree`. `rerun_lane_unset` is a soft state — it is normal on a
freshly opened workspace.

## Structured artifact identity

Every artifact ref produced by an alpha attempt is republished as a
`test_runner_beta_artifact_identity_record`:

- `artifact_kind` is a closed token (see the table above);
- `test_session_ref`, `test_attempt_ref`, `canonical_test_item_refs`, and
  `selector_ref` quote the same identity the tree row, the inline row, and
  the rerun-last contract use;
- `producing_run_ref` and `producing_execution_attempt_ref` link back to the
  generic execution rail so support exports can replay the attempt without
  parsing adapter payloads;
- `identity_stability_token` carries the same stability label the test-item
  identity projection used (so a row imported from CI is visibly read-only
  even on the artifact rail).

The shell consumer renders these as a deterministic plaintext block in the
support-export clipboard action; they are never derived from a freeform run
log.

## Support exports

The support-export packet
[`TestRunnerBetaSupportExport`](../../../crates/aureline-runtime/src/testing/mod.rs)
bundles the coverage manifest, the tree-projection ids, the inline-projection
ids, the artifact identity rows, the parity rows, and the underlying alpha
test-attempt packet ids. A reviewer reading the export can verify, without
accessing live workspace state, that:

- every claimed beta row belongs to a framework the manifest claims;
- every parity row carries a matching tree / inline pair;
- every artifact ref is bound to a typed kind, a session, and a canonical
  test-item id.
- when attached, the `test_trust_packet_ref` points to the release-facing
  watch/flaky/snapshot/quarantine debt packet for the same beta rows.

## Cross-references

- Alpha test-attempt model — [`/crates/aureline-runtime/src/tests/`](../../../crates/aureline-runtime/src/tests/)
  and [`/schemas/testing/test_attempt.schema.json`](../../../schemas/testing/test_attempt.schema.json)
- Beta task-event model — [`task_event_model_beta.md`](task_event_model_beta.md)
- Beta run / debug profile model — [`run_debug_profiles_beta.md`](run_debug_profiles_beta.md)
- Rerun-last task and test loop —
  [`/crates/aureline-runtime/src/rerun/`](../../../crates/aureline-runtime/src/rerun/)
- Promoted identity/session/attempt ledger — [`test_identity_beta.md`](test_identity_beta.md)
- Pytest discovery —
  [`/crates/aureline-runtime/src/discovery/pytest/`](../../../crates/aureline-runtime/src/discovery/pytest/)
- Test triage trust packets — [`test_triage_trust_beta.md`](test_triage_trust_beta.md)

## Out of scope for this beta

- Frameworks beyond pytest (jest / vitest / cargo-test do not yet ship a
  framework row in the coverage manifest).
- Full M5 notebook-kernel test depth.
- Cross-workspace test-tree merging.
- Collaboration or multi-user test-tree productization.
