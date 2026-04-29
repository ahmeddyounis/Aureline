# Test watch-mode, inline-result, and environment-matrix contract

This document freezes the live test-feedback contract shared by the
editor, notebooks, the test tree, CLI output, import/replay review,
support export, and release-evidence readers. It completes the test
object model by making watch fidelity, inline-result freshness, and
environment-labeled result truth durable records rather than transient
UI strings.

Machine-readable companions:

- [`/schemas/execution/watch_controller_state.schema.json`](../../schemas/execution/watch_controller_state.schema.json)
  — the `watch_controller_state_record`, including watch state,
  degradation reason, debounce policy, batch policy, backlog, power
  posture, remote posture, target health, pause state, and required
  disclosure where test results are consumed.
- [`/schemas/execution/inline_test_result.schema.json`](../../schemas/execution/inline_test_result.schema.json)
  — the `inline_test_result_record`, tying one visible editor or
  notebook location to the latest relevant attempt, evidence window,
  freshness, origin, remap posture, stability chip, and
  multi-environment divergence cues.
- [`/schemas/execution/environment_matrix_row.schema.json`](../../schemas/execution/environment_matrix_row.schema.json)
  — the `environment_matrix_row_record`, naming local, container,
  remote, CI, notebook, and provider-backed result rows with authority,
  comparability, runtime/toolchain/build refs, artifact refs, and rerun
  posture.
- [`/fixtures/execution/watch_mode_cases/`](../../fixtures/execution/watch_mode_cases/)
  — worked YAML fixtures covering live, polling, paused, backlog,
  unavailable, imported/stale inline markers, remapped notebook
  markers, and environment matrix rows across the supported target
  families.

This contract composes with and does not replace:

- [`/docs/testing/test_item_identity_contract.md`](../testing/test_item_identity_contract.md),
  [`/schemas/testing/test_item_identity.schema.json`](../../schemas/testing/test_item_identity.schema.json),
  and
  [`/schemas/testing/test_selector_grammar.schema.json`](../../schemas/testing/test_selector_grammar.schema.json)
  — durable test-item identity, selector grammar, parameterized
  expansion/collapse, and remap/drift records. Watch controllers and
  inline markers cite identity/remap refs; they do not infer runnable
  scope from display labels.
- [`/docs/execution/run_and_attempt_contract.md`](./run_and_attempt_contract.md)
  and
  [`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json)
  — watch cycles and rerun/debug actions are still runs and attempts.
  A watch controller cites the session/run/attempt lineage rather than
  mutating one status row in place.
- [`/docs/execution/test_truth_contract.md`](./test_truth_contract.md),
  [`/schemas/execution/test_discovery_state.schema.json`](../../schemas/execution/test_discovery_state.schema.json),
  [`/schemas/execution/test_run_summary.schema.json`](../../schemas/execution/test_run_summary.schema.json),
  and
  [`/schemas/execution/flaky_history.schema.json`](../../schemas/execution/flaky_history.schema.json)
  — discovery, per-item state, run summary, snapshot/golden review,
  flaky verdict, and quarantine truth remain the base test model.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md),
  [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json),
  and
  [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
  — every watch, inline marker, and environment row cites the
  execution context and target snapshot it is describing.
- [`/docs/execution/task_event_and_evidence_contract.md`](./task_event_and_evidence_contract.md)
  and
  [`/schemas/execution/artifact_event.schema.json`](../../schemas/execution/artifact_event.schema.json)
  — logs, structured reports, coverage payloads, support bundles, and
  imported provider artifacts ride the existing artifact/evidence rail.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — freshness and degraded-state labels reuse the shared honesty
  vocabulary. Cached, imported, stale, partial, and unknown rows remain
  visible typed states.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Architecture_Document.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those documents
  disagree with this contract, those upstream documents win and this
  contract plus the companion schemas update in the same change.

Raw command lines, raw stdout/stderr byte streams, raw environment
bodies, raw absolute paths, raw URLs, raw secret values, raw test names,
raw assertion bodies, raw snapshot bytes, raw artifact bytes, and raw
stack traces MUST NOT cross these boundaries. Records carry opaque refs,
digests, counts, timestamps, and class labels only.

## Why freeze this now

Live test feedback is where stale truth most often looks useful enough
to trust. A green gutter marker can come from a fresh local run, a
cached watch cycle, an imported CI artifact, a notebook kernel that no
longer matches the file, or a provider-backed target that is not
rerunnable locally. If those states collapse into one badge, users make
wrong rerun, debug, review, and release decisions.

The failure modes this contract prevents:

- watch mode degrades to polling, power-throttled batches, or remote
  latency, while inline markers continue to look live;
- a paused or unavailable controller leaves the last pass/fail badge in
  the editor with no stale label;
- a provider-imported CI result appears identical to a fresh local run;
- a moved test, stale discovery snapshot, or changed execution target
  makes an inline marker disappear instead of narrowing into a typed
  remap/stale state;
- a parent test-tree row hides filtered, quarantined, unsupported, or
  unknown children while rendering a clean aggregate pass;
- local, container, remote, CI, notebook, and provider-backed runs are
  merged into one generic pass/fail row without authority,
  comparability, or artifact refs.

## Scope

Frozen at this revision:

- the watch controller state vocabulary:
  `live`, `reduced`, `polling`, `paused`, `degraded`, `backlog`,
  `unavailable`, and `watch_state_unknown_requires_review`;
- the watch degradation reason vocabulary, including watcher polling
  fallback, event loss, watcher overflow/rescan, adapter crash, remote
  latency/disconnect, target-context drift, power/thermal throttling,
  policy-limited scope, debounce/batch extension, backlog budget breach,
  notebook-kernel loss, provider read-only import, and unknown;
- the required watch fields for debounce policy, batch policy, backlog,
  power summary, remote summary, target health, pause summary, last
  successful cycle, and result-surface disclosure;
- the inline-result location vocabulary for editor gutter, editor
  inline overlay, editor line-end badge, notebook cell gutter, notebook
  output header, test-tree mirror, generated-artifact mapping, and
  unknown;
- the inline-result freshness vocabulary, including current,
  live-pending, warm cached, stale evidence-window expired, stale
  discovery snapshot, stale target changed, no evidence yet, and
  unknown;
- the inline-result origin vocabulary separating live local, live
  container, live remote, live notebook, provider-imported CI,
  provider-imported non-CI, cached local, cached provider, read-only
  import, and unknown;
- the inline-result remap vocabulary for exact anchors, moved-test
  exact remaps, moved-test approximate remaps, stale markers retained
  because a source anchor is missing, notebook cell identity changes,
  generated-source mapping degradation, and unknown;
- the divergence cue vocabulary covering no known divergence, local vs
  container, local vs remote, minor/major local vs CI drift, notebook vs
  file drift, provider/local pending parity, incompatible comparison
  blocked, and unknown;
- the environment row target vocabulary for local, container, remote,
  CI, notebook, provider-backed, and unknown rows;
- the environment row authority vocabulary separating authoritative
  live rows from cached rows, read-only CI/provider imports, notebook
  kernel rows, merged authority, and unknown;
- the comparability vocabulary: `exact_target`, `compatible_rerun`,
  `comparable_with_caution`, `manual_re_resolve_required`,
  `comparison_blocked_incompatible`, `comparison_pending`, and
  `comparability_unknown_requires_review`.

## Watch Controller Rules

A watch controller is durable. It has one controller id and cites the
test session, selection, discovery snapshot, execution-context snapshot,
latest run, and latest attempt it is coordinating. Watch loops append
attempt history through the run/attempt rail; they do not overwrite the
prior attempt in place.

Every controller MUST publish:

- `state_class`, one of the frozen watch states;
- `degradation_reason_class`, even when the value is `none`;
- `debounce_policy` with the debounce class and bounded timing values;
- `batch_policy` with the batch class and bounded batch/cycle values;
- `backlog_summary` with pending change/test counts and the oldest
  pending age;
- `power_summary`, including whether power saver or thermal throttling
  narrowed the controller;
- `remote_summary`, including host boundary and remote health posture;
- `target_health_summary`, including watcher, adapter, and
  target-context health;
- `pause_summary`, even when the controller is not paused;
- `last_successful_cycle`, or `null` when there has never been one;
- `result_surface_disclosure`, proving that degradation is visible
  where results are consumed.

The controller MUST surface degradation in editor inline results, the
test tree or summary, CLI/export rows, and support views whenever those
surfaces show any result that depends on the controller. A controller
MUST NOT store degradation only in logs, diagnostics, or private adapter
state.

State semantics:

- `live` means event-driven watch is armed and latest visible results
  are inside their evidence window.
- `reduced` means watch continues but one fidelity axis is narrowed
  while freshness remains bounded and visible.
- `polling` means event notifications are not authoritative; cycles are
  driven by an interval or explicit scan and visible results need a
  polling chip.
- `paused` means the controller is intentionally stopped and no result
  may appear current after its evidence window expires.
- `degraded` means the controller continues with known target,
  adapter, remote, policy, or health limitations.
- `backlog` means pending changes or tests exceed the batch/cycle
  budget; inline rows may show live-pending or stale labels but not a
  clean current result unless their own evidence window is current.
- `unavailable` means the controller cannot produce new cycles for the
  target; prior evidence remains visible only as cached/imported/stale
  truth.
- `watch_state_unknown_requires_review` fails closed and forbids fresh
  pass/fail rendering.

## Inline Result Rules

An inline result row is a projection, not a source of authority. It ties
one visible location to the latest relevant attempt, run summary, test
item, discovery snapshot, and environment rows. It never carries raw
source paths or raw test names.

Every inline row MUST publish:

- `visible_location`, identifying editor or notebook placement through
  opaque document/cell/mapping refs;
- `test_item_ref`, `parent_discovery_state_ref`, and
  `latest_relevant_attempt_ref`;
- `result_state_class`, using the same outcome distinctions as the test
  item and run-summary contracts;
- `freshness_class` and an explicit `evidence_window`;
- `result_origin_class`, separating live, cached, imported, notebook,
  and provider-backed evidence;
- `anchor_remap_state_class`, so moved tests, stale discovery
  snapshots, changed targets, notebook cell identity changes, and
  generated-source mapping degradation stay visible;
- `divergence_summary`, even when the value is no known divergence;
- `environment_matrix_row_refs`, so the inline marker can reveal where
  the result came from;
- `badge_disclosure`, including whether origin, stale, divergence, or
  watch-degradation chips are required.

Inline-result badges MUST NOT outlive their evidence window. Once
`expires_at` is in the past, the row narrows to a stale freshness class
or disappears only after a successor row or typed tombstone is visible.
Imported CI/provider evidence MUST show an imported/read-only origin
chip unless a fresh local parity attempt has replaced it as the latest
relevant evidence. Cached local evidence MUST show cached or stale
posture; it may not impersonate a fresh local watch cycle.

Moved tests and stale discovery snapshots are represented as typed
states:

- exact moved-test remaps may preserve the badge with a remap chip;
- approximate moved-test remaps may preserve only stale/cached state and
  must offer a review action before rerun/debug shortcuts;
- missing anchors keep a stale marker at the prior visible location or
  move to a test-tree/history row with the same record id lineage;
- changed target or environment fingerprints keep the inline row
  visible as stale-target-changed until a compatible rerun or manual
  re-resolve produces new evidence.

## Environment Matrix Rules

An environment matrix row names one result-bearing environment, not a
generic status bucket. Local, container, remote, CI, notebook, and
provider-backed rows remain separate even when their result states
match.

Every environment row MUST publish:

- target class and target identity refs;
- execution context, context snapshot, runtime, toolchain, build, and
  environment-fingerprint refs;
- authority class and freshness class;
- result source and parity-validation class;
- comparability class;
- result state and by-state counts;
- structured test report, coverage, log, support bundle, and additional
  artifact refs where available;
- comparison summary naming compared rows, divergence posture, and
  caution/blocking posture;
- rerun/debug action refs when admitted, or a manual re-resolve flag
  when the row cannot be rerun safely in its current target.

Comparability semantics:

- `exact_target` means same target identity, runtime/toolchain/build,
  policy epoch, and relevant environment fingerprint.
- `compatible_rerun` means a rerun is expected to be semantically
  comparable after normal resolver work, but not byte-for-byte exact.
- `comparable_with_caution` means the UI/export must show the caution
  sentence before comparison, release claims, or rerun/debug actions.
- `manual_re_resolve_required` means the user or automation must review
  a target/environment resolver sheet before rerun/debug.
- `comparison_blocked_incompatible` means aggregate pass/fail
  comparison is blocked.
- `comparison_pending` means comparison is not yet resolved and may not
  be rendered as agreement.
- `comparability_unknown_requires_review` fails closed.

Users and support MUST be able to compare environments without losing
authority or artifact lineage. A green local row and a green CI row are
two rows until a comparability rule explicitly admits aggregation.

## Downgrade And Remap Rules

Downgrades preserve mental-model continuity. A moved test, stale
snapshot, changed target, remote disconnect, provider-only import, or
adapter loss narrows the record; it does not silently remove it.

Required downgrade behavior:

- Watch controller downgrade is visible through
  `result_surface_disclosure` and propagated to inline rows that depend
  on the controller.
- Inline rows keep `anchor_remap_state_class`, `freshness_class`,
  `result_origin_class`, and `divergence_summary` until a successor row
  supersedes them.
- Environment rows keep `authority_class`, `comparability_class`,
  `freshness_class`, and artifact refs even when a row is blocked,
  stale, read-only, or manually re-resolve-required.
- Rerun/debug shortcuts cite the originating selection and environment
  row. If scope or target widens, a review action is required before
  execution.
- Unknown states fail closed. They may be exported and reviewed, but
  they may not render as fresh pass/fail or authorize mutating actions.

## Release And Support Publication

Support bundles, CLI output, release packets, certification claims, and
provider-import review surfaces consume the same records:

- watch-mode support claims cite state-class coverage and downgrade
  reasons by target family;
- inline-result audits count current, cached, imported, stale,
  divergent, remapped, and unknown badges separately;
- environment matrices preserve row identity, authority,
  comparability, and artifact refs;
- imported-only or stale rows may contribute to review context, but
  they do not count as fresh local validation unless a matching
  environment row records compatible or exact local parity.

## Pre-implementation Note

There is no live watch-mode implementation, test UI, notebook test
runner, or environment matrix surface wired up yet. These records are
pre-implementation governance artifacts so later execution surfaces
share one vocabulary from the start.
