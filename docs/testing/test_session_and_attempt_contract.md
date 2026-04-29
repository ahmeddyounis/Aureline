# Test session, attempt, watch-state, and imported-CI projection contract

This contract freezes the testing-level session and attempt model that
connects durable test identity, execution run / attempt records, watch
controllers, imported provider evidence, inline markers, history rows,
debug-from-test launches, support exports, and release evidence packets.

A test session is the user-visible testing activity: run selected tests,
watch a scope, rerun failures, debug from a test row, import provider CI
evidence, follow a provider run, or reconstruct evidence for support or
release. A test attempt is one observed execution, import, follow update,
or debug launch inside that session. Attempts are append-only and keep
their own selector, target, environment, source, artifact, and time
lineage.

Machine-readable companions:

- [`/schemas/testing/test_session.schema.json`](../../schemas/testing/test_session.schema.json)
  - the `test_session_record`, including session purpose, trigger
  source, selector basis, execution context, target environment, watch
  state, imported-CI projection summary, raw event refs, mute /
  quarantine summary, artifact linkage, attempt refs, and surface
  mappings.
- [`/schemas/testing/test_attempt.schema.json`](../../schemas/testing/test_attempt.schema.json)
  - the `test_attempt_record`, including attempt purpose, trigger,
  predecessor / origin refs, selector basis, execution context, target
  environment, source drift, time basis, watch state at attempt,
  imported-CI projection class, mute / quarantine state, artifacts,
  raw event lineage, surface mappings, and reconstruction proof.
- [`/fixtures/testing/test_session_cases/`](../../fixtures/testing/test_session_cases/)
  - worked YAML fixtures covering a local watch session, a
  provider-imported partial artifact, a rerun-failed attempt after a
  source edit, and a debug-from-test attempt.
- [`/docs/testing/test_quarantine_and_mute_contract.md`](./test_quarantine_and_mute_contract.md),
  [`/schemas/testing/quarantine_record.schema.json`](../../schemas/testing/quarantine_record.schema.json),
  and
  [`/fixtures/testing/quarantine_cases/`](../../fixtures/testing/quarantine_cases/).
  Sessions and attempts cite mute / quarantine refs; the quarantine and
  mute contract defines owner, expiry, allowed surfaces, release
  visibility, review cadence, unblock, and waiver/debt treatment for
  those refs.

This contract composes with and does not replace:

- [`/docs/testing/test_item_identity_contract.md`](./test_item_identity_contract.md),
  [`/schemas/testing/test_item_identity.schema.json`](../../schemas/testing/test_item_identity.schema.json),
  and
  [`/schemas/testing/test_selector_grammar.schema.json`](../../schemas/testing/test_selector_grammar.schema.json).
  Sessions and attempts cite selector and identity records. They do not
  resolve display labels by themselves.
- [`/docs/execution/run_and_attempt_contract.md`](../execution/run_and_attempt_contract.md),
  [`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json),
  and
  [`/schemas/execution/attempt.schema.json`](../../schemas/execution/attempt.schema.json).
  A test attempt is a testing projection over the generic execution
  attempt rail. The execution attempt remains the process / queue /
  artifact authority; the testing attempt owns selector, target, test
  source, provider-import, watch, and surface reconstruction truth.
- [`/docs/execution/test_truth_contract.md`](../execution/test_truth_contract.md),
  [`/schemas/execution/test_discovery_state.schema.json`](../../schemas/execution/test_discovery_state.schema.json),
  [`/schemas/execution/test_run_summary.schema.json`](../../schemas/execution/test_run_summary.schema.json),
  and
  [`/schemas/execution/flaky_history.schema.json`](../../schemas/execution/flaky_history.schema.json).
  Discovery state, run summaries, snapshot review, flaky history, and
  quarantine remain the result-state authorities that sessions and
  attempts cite.
- [`/docs/execution/test_watch_and_environment_contract.md`](../execution/test_watch_and_environment_contract.md),
  [`/schemas/execution/watch_controller_state.schema.json`](../../schemas/execution/watch_controller_state.schema.json),
  [`/schemas/execution/inline_test_result.schema.json`](../../schemas/execution/inline_test_result.schema.json),
  and
  [`/schemas/execution/environment_matrix_row.schema.json`](../../schemas/execution/environment_matrix_row.schema.json).
  Watch controllers, inline rows, and environment rows cite session and
  attempt refs rather than owning separate history.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  and
  [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json).
  Every session and attempt names the execution context and target
  snapshot being described.
- [`/docs/execution/task_event_and_evidence_contract.md`](../execution/task_event_and_evidence_contract.md)
  and
  [`/schemas/execution/artifact_event.schema.json`](../../schemas/execution/artifact_event.schema.json).
  Raw runner events, logs, structured reports, coverage, mirrored
  provider artifacts, support bundles, and release packets stay on the
  evidence / artifact rail.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Architecture_Document.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those documents
  disagree with this contract, those upstream documents win and this
  contract plus the companion schemas update in the same change.

Raw command lines, raw stdout or stderr byte streams, raw environment
bodies, raw absolute paths, raw URLs, raw secret values, raw test names,
raw assertion bodies, raw source excerpts, raw artifact bytes, raw
provider payloads, and raw stack traces MUST NOT cross these
boundaries. Records carry opaque refs, digests, counts, class labels,
bounded summaries, and timestamps only.

## Why freeze this now

Test evidence becomes misleading when each surface stores only its
latest status. The same visible test row can be backed by a fresh local
attempt, a watch cycle, a failed-only rerun, a debug launch, a provider
CI import, a mirrored artifact, or an old provider result that is no
longer comparable to the current source. Flattening those into one
status erases the target and environment that produced the evidence.

The failure modes this contract prevents:

- rerun failed starts from an old failed subset after the source changed
  and the history row no longer shows whether the attempt was an exact
  replay or current-context rerun;
- a provider-imported green CI result appears as the current local
  truth even though no local parity attempt ran;
- a mirrored CI artifact in a support bundle is treated as the
  provider's authoritative current state instead of a captured copy;
- a watch controller enters debounce, partial import, stale import, or
  disconnected follow while inline markers still look live;
- debug-from-test loses the originating selector and target, making the
  debug session impossible to relate back to release evidence;
- a quarantined or muted test vanishes from counts in a tree aggregate,
  support export, or release evidence packet.
- a local mute is treated as shared release truth and removes a debt row
  from scorecards or claim manifests.

## Scope

Frozen at this revision:

- the `test_session_record` field set for session purpose, trigger
  source, lifecycle, selector basis, execution context, target
  environment, watch state, imported-CI projection summary, raw event
  lineage, mute / quarantine summary, artifact linkage, surface
  mapping, and ordered attempt refs;
- the `test_attempt_record` field set for attempt purpose, trigger
  source, predecessor / origin refs, execution attempt refs, selector
  basis, execution context, target environment, source drift, time
  basis, result projection, watch state at attempt, mute / quarantine
  state, artifact linkage, raw event lineage, surface mapping, and
  reconstruction proof;
- the watch-state taxonomy:
  `idle`, `initial_discovery`, `watching`, `debounce_pending`,
  `rerun_failed`, `partial_import`, `provider_live_follow`,
  `stale_imported`, `disconnected_follow`, and the fail-closed
  `watch_state_unknown_requires_review`;
- the imported-CI projection taxonomy:
  `not_imported_ci`, `authoritative_provider_result`,
  `mirrored_artifact`, `local_rerun`, `incomparable_stale_prior_result`,
  and the fail-closed `imported_ci_projection_unknown_requires_review`;
- the time-basis taxonomy that keeps provider observation time,
  artifact mirror import time, local execution time, local rerun time,
  and stale prior result time distinct;
- the inline / editor / tree / history / export mapping rule that a
  surface row must cite the same session and attempt refs needed to
  reconstruct the selector, target, environment, source revision,
  artifacts, raw-event refs, and time basis.

## Session Record Rules

A session is a durable container for one testing activity. It answers:
what the user or provider triggered, which selector or import scope was
in force, what target was requested, which environment was resolved,
which watch state applies, whether the evidence came from local
execution or imported CI, and which attempts belong to the session.

Every session MUST publish:

- `session_purpose_class`;
- `trigger_source_class`;
- `lifecycle_state_class`;
- `selection_basis`, including selector refs, canonical test item refs,
  predecessor attempt refs when the selection came from prior failures,
  and the widening rule;
- `execution_context`, including execution-context and context-snapshot
  refs plus the policy and identity refs that affected resolution;
- `target_environment`, including target class, target identity,
  fingerprint, runtime, toolchain, build, and environment matrix refs;
- `watch_state`, even for non-watch sessions, where the state is
  `idle`;
- `imported_ci_projection`, even when it is `not_imported_ci`;
- `raw_event_lineage`, so normalized records can be traced back to the
  governed event / artifact refs without exposing raw payloads;
- `muting_and_quarantine_summary`, including visible muted /
  quarantined counts and refs;
- `artifact_linkage`, including structured report, coverage, log,
  mirrored provider artifact, support bundle, release evidence, and
  additional artifact refs where present;
- `surface_mapping`, proving editor, tree, history, export, release,
  debug, and CLI projections cite this session consistently;
- ordered `attempt_refs`.

Sessions MUST NOT:

- represent an imported provider row as a live local session;
- omit target/environment identity from a rerun-capable session;
- collapse multiple target environments into one status;
- hide muted or quarantined tests from aggregate counts;
- claim provider-live follow after the provider disconnects without
  entering `disconnected_follow` or `stale_imported`.

## Attempt Record Rules

An attempt is one observed unit inside a session. It may be a local test
execution, a watch cycle, a failed-only rerun, a debug-from-test launch,
a provider import, a provider follow update, a local parity rerun, or a
support / release reconstruction. Attempts are append-only. A newer
attempt may supersede what surfaces show as "latest", but it never
overwrites the older attempt's context, artifacts, source snapshot, or
time basis.

Every attempt MUST publish:

- `parent_test_session_ref`;
- `parent_run_ref` and `parent_execution_attempt_ref` when the attempt
  executed or debug-launched through the execution rail;
- `attempt_purpose_class` and `attempt_trigger_class`;
- `attempt_index`;
- predecessor or origin refs when the attempt is a rerun, parity rerun,
  provider follow update, or debug-from-test launch;
- the same selector-basis, execution-context, and target-environment
  object families used by sessions, resolved at attempt-open;
- `source_lineage`, including whether source changed since the
  predecessor attempt;
- `time_basis`, naming the observed and captured times and their
  authority;
- `result_projection`, including imported-CI projection, result source,
  parity validation, comparability, run summary, environment rows, and
  outcome refs;
- `watch_state_at_attempt`;
- `muting_quarantine_state`;
- `artifact_linkage`;
- `raw_event_lineage`;
- `surface_mapping`;
- `reconstruction_contract`.

Attempts MUST NOT:

- silently widen a failed-only rerun to the full scope;
- claim exact replay when source, selector, target, runtime, policy, or
  environment fingerprint drifted;
- treat a provider import or mirrored artifact as local current truth;
- launch debug-from-test without citing the originating attempt,
  selector, target, and debug-session ref;
- omit artifact or raw-event refs needed for release evidence or
  support reconstruction.

## Watch-State Taxonomy

| State | Meaning | Required posture |
|---|---|---|
| `idle` | No watch cycle is armed for the session. | Rerun/debug may still be available through explicit actions, but watch freshness is not implied. |
| `initial_discovery` | The session is collecting or importing the first discovery snapshot. | Results are pending or cached; no current pass/fail may be claimed from this state alone. |
| `watching` | The watch controller is armed and current evidence remains inside its window. | Inline/tree/history rows may render current only when their linked attempt is current. |
| `debounce_pending` | Changes have been observed and are waiting for a bounded debounce or batch cycle. | Affected rows are live-pending or stale; they may not remain clean current. |
| `rerun_failed` | The session is executing the failed subset from a prior attempt. | The attempt must cite the prior failed subset and any widening review. |
| `partial_import` | A structured provider report or artifact was only partially imported. | Imported rows are partial/read-only; missing shards and omitted scopes remain visible. |
| `provider_live_follow` | The session is following a provider run that is still live in the provider environment. | Provider authority and provider time basis are shown; local truth is not implied. |
| `stale_imported` | Imported evidence is outside its freshness window or source/target changed. | Evidence remains visible as history but cannot count as current validation. |
| `disconnected_follow` | A provider or remote follow lost connectivity. | Last-known evidence is cached/stale; rerun/cancel/follow actions require fresh authority. |
| `watch_state_unknown_requires_review` | The watch state cannot be classified. | Fail closed. No fresh pass/fail or mutating action may be inferred. |

## Imported-CI Projection Rules

Imported CI evidence is a projection, not a replacement for local
attempt history.

| Projection class | Meaning | Current-truth rule |
|---|---|---|
| `not_imported_ci` | The session or attempt did not depend on imported CI evidence. | Result truth comes from the cited local/container/remote/notebook execution rows. |
| `authoritative_provider_result` | The provider result is authoritative for the provider environment named by its provider run, job, matrix, commit, runner, and timestamp refs. | It may be current for that provider row only. It is not local current truth without a separate parity attempt. |
| `mirrored_artifact` | Aureline has a retained copy of a provider report, log, annotation, or artifact. | The mirror preserves what was captured and when; it is not proof that the provider still reports the same state. |
| `local_rerun` | A local/container/remote attempt re-executed a selector to compare with imported provider evidence. | It creates new local evidence and a parity row; it does not erase the provider row. |
| `incomparable_stale_prior_result` | Prior evidence is stale because source, target, runtime, policy, provider freshness, or environment fingerprint changed beyond comparability. | It may remain visible in history/export only and may not satisfy current validation. |
| `imported_ci_projection_unknown_requires_review` | Projection cannot be classified. | Fail closed; do not aggregate into pass/fail truth. |

Imported CI projections MUST preserve:

- provider event refs and provider artifact refs;
- provider observation time, provider import time, and local capture
  time as distinct time bases;
- provider target identity, runner/environment identity, matrix axis,
  commit or source snapshot refs, and artifact digest refs;
- mirror linkage when an artifact was copied locally or into support /
  release evidence;
- local parity attempt refs when a local rerun exists;
- comparability class when a local row and provider row are compared;
- omitted scope and partial import summaries.

## Surface Mapping Rules

Inline markers, editor gutters, test-tree rows, history rows, exports,
debug sessions, CLI rows, support bundles, and release evidence packets
are projections. They must be able to reconstruct one attempt by
following refs rather than by copying status text.

Every projection that names an attempt MUST be able to recover:

- `test_session_record_id`;
- `test_attempt_record_id`;
- selector refs and canonical test item refs;
- discovery snapshot and remap refs;
- execution context and target environment refs;
- source snapshot / source drift refs;
- imported-CI projection class and provider lineage refs when relevant;
- time basis;
- artifact refs;
- raw event refs or artifact refs sufficient for governed replay /
  support review;
- mute / quarantine state refs;
- predecessor, origin, or debug-session refs when the attempt came from
  rerun, provider follow, local parity, or debug-from-test.

Mapping guardrails:

- inline/editor rows MAY show latest-attempt status, but history/export
  rows MUST keep older attempts reachable;
- tree aggregates MUST count muted, quarantined, disabled, unsupported,
  unknown, imported, and stale children separately;
- debug-from-test mints a debug attempt linked to the originating test
  attempt and debug session; it does not rewrite the test attempt;
- release evidence packets cite sealed attempt refs and artifact refs;
  they do not recalculate "latest status" at export time;
- support exports may redact raw payloads, but they must preserve the
  refs, digests, classes, and time basis needed to explain lineage.

## Required Invariants

| Invariant | Contract surface |
|---|---|
| A session preserves exact selector, target, and environment lineage across local, remote, watched, rerun, debug, imported, and release flows. | `test_session_record.selection_basis`, `execution_context`, `target_environment`, `attempt_refs`. |
| Attempts are append-only and reconstructable. | `test_attempt_record.attempt_index`, predecessor / origin refs, artifacts, raw-event lineage, and `reconstruction_contract`. |
| Imported CI never masquerades as local current truth. | `imported_ci_projection_class`, provider lineage refs, time basis, and comparability class. |
| Mirrored artifacts remain captured copies, not provider-live status. | `mirrored_provider_artifact_ref`, artifact digest refs, import/capture times. |
| Rerun failed preserves failed-subset and source-drift truth. | `selector_basis_class = failed_only_from_prior_attempt`, `source_lineage.source_drift_class`, predecessor refs. |
| Debug-from-test keeps the originating test attempt reachable. | `attempt_purpose_class = debug_from_test_attempt`, `originating_attempt_ref`, `debug_session_ref`. |
| Muted or quarantined tests remain counted and linked. | `muting_and_quarantine_summary` and `muting_quarantine_state`. |
| Unknown watch, projection, or comparability states fail closed. | `*_unknown_requires_review` enum values and denial reasons. |

## Versioning

The two schema files declare schema version `1`.

Adding a new optional field, enum member, projection class, watch state,
or surface mapping class is additive-minor and bumps the relevant schema
version. Removing or repurposing an existing value is breaking and
requires a decision row. Any schema change that changes session,
attempt, imported-CI, or reconstruction semantics must update this
document and the fixture corpus in the same change.

## Pre-implementation Note

There is no live test session store, test runner UI, provider CI
connector, or debug-from-test integration wired up yet. These records
are pre-implementation governance artifacts so later execution surfaces
share one lineage-preserving vocabulary from the start.
