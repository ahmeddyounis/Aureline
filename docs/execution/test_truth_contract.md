# Test discovery, flaky-state, snapshot / golden, and rerun semantics contract

This document freezes one user-visible test model so the test runner
UI, the test tree panel, the explorer hover, the activity row, the
quick-action chip, the AI tool-call mediator, the support exporter,
the replay / import probe, and any "rerun this" affordance read the
same record family rather than inventing per-surface vocabularies for
what "discovered", "passed", "quarantined", "flaky", or "snapshot
updated" mean.

Every test surface (the test runner UI, the test tree panel, the
explorer hover, the activity center row, the AI tool-call mediator,
the support exporter, the replay / import probe, the durable activity
center, and any history / evidence export) reads the same record
family. There is one test discovery-state record family with typed
provenance, freshness, and scope; one test-item record family with a
canonical id and a typed eleven-value state vocabulary; one test
run-summary record per attempt with a typed result-source, rerun-
scope, and parity-validation row; one snapshot / golden review record
with typed baseline identity, file-count truth, and rollback path;
one flaky-history record family aggregating per-item attempt history
under a typed window; and one quarantine record family with typed
authority, reason, state, and expiry — not a separate hidden
discovery / flake / snapshot vocabulary per surface.

Machine-readable companions:

- [`/schemas/execution/test_discovery_state.schema.json`](../../schemas/execution/test_discovery_state.schema.json)
  — the `test_discovery_state_record` (the typed discovery header
  with provenance, freshness, scope, host boundary, and counts), the
  `test_item_record` (the per-node row with canonical id, kind,
  eleven-value state, parameterized lineage, and reserved
  flaky-history / quarantine cross-links), and the
  `coverage_handoff_record` (the typed coverage provenance,
  freshness, file / line counts, and artifact-event handoff).
- [`/schemas/execution/test_run_summary.schema.json`](../../schemas/execution/test_run_summary.schema.json)
  — the `test_run_summary_record` (the typed projection of how a
  test attempt finished, where its result came from, what rerun
  scope it executed under, and the per-state count truth) and the
  `snapshot_review_record` (the typed snapshot / golden update
  preview with baseline identity, change summary, file-count truth,
  and rollback path).
- [`/schemas/execution/flaky_history.schema.json`](../../schemas/execution/flaky_history.schema.json)
  — the `flaky_history_record` (typed classification, observed
  signals, history window, outcome tally) and the
  `quarantine_record` (typed authority, reason, state, expiry,
  lift / supersede paths).
- [`/fixtures/execution/test_cases/`](../../fixtures/execution/test_cases/)
  — worked YAML fixtures covering the required scenarios (cached
  discovery, quarantined flake, provider-imported result vs local
  parity run, failed-only rerun, snapshot update review).

This contract composes with and does not replace:

- [`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json),
  [`/schemas/execution/attempt.schema.json`](../../schemas/execution/attempt.schema.json),
  [`/schemas/execution/artifact_event.schema.json`](../../schemas/execution/artifact_event.schema.json),
  and
  [`/docs/execution/run_and_attempt_contract.md`](./run_and_attempt_contract.md)
  — run / attempt / artifact-event / outcome-event / rerun-comparison
  records. A test attempt rides a `run_record` with `run_kind_class =
  test_run`; a test run-summary cites the parent run, parent
  attempt, and outcome-event refs; the structured test report,
  coverage report, and snapshot / golden artifact bytes ride the
  per-attempt artifact rail; the typed rerun lineage uses the run /
  attempt rerun-kind vocabulary plus the test-specific
  `rerun_scope_class`.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  and
  [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
  — execution-context root, target identity, toolchain identity,
  workset / scope vocabulary, trust state, identity-mode envelope,
  policy epoch (ADR-0009, ADR-0018, ADR-0001). Every test discovery
  cites the snapshot the discovery resolved against; every test-run
  summary cites the parent attempt's snapshot.
- [`/schemas/execution/artifact_event.schema.json`](../../schemas/execution/artifact_event.schema.json)
  — the artifact rail. The structured test report, coverage report,
  snapshot / golden baseline, and snapshot / golden proposed payload
  ride `artifact_class = structured_test_report` /
  `coverage_report` / `support_bundle_segment` (snapshot bundles)
  on the per-attempt artifact rail. The test contract never re-mints
  a parallel test-artifact retention surface.
- [`/docs/execution/test_watch_and_environment_contract.md`](./test_watch_and_environment_contract.md),
  [`/schemas/execution/watch_controller_state.schema.json`](../../schemas/execution/watch_controller_state.schema.json),
  [`/schemas/execution/inline_test_result.schema.json`](../../schemas/execution/inline_test_result.schema.json),
  and
  [`/schemas/execution/environment_matrix_row.schema.json`](../../schemas/execution/environment_matrix_row.schema.json)
  — live watch-controller state, inline result markers, and
  environment-matrix rows. Watch degradation, imported-versus-live
  inline freshness, moved-test remaps, and local / container / remote /
  CI / notebook / provider-backed comparability are durable
  projections over the same discovery, run-summary, and artifact rails.
- [`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
  — the canonical lower-level task-event envelope for build / test /
  diagnostic / artifact-publication events. Adapter-level test
  events (a per-case start / pass / fail event from cargo-test,
  pytest, jest, etc.) ride the envelope; the test contract sits
  **above** the envelope and reads the per-attempt artifact rail
  rather than the raw event stream.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — shared truth-class and degraded-state vocabulary. Discovery and
  coverage freshness re-use the `authoritative_live` /
  `warm_cached` / `degraded_cached` / `stale_*` tokens; AI-inferred
  flaky signals re-use the `ai_inferred_truth` anchor-citation
  rule; provider-imported results re-use the
  `runtime_observed_truth` and `derived_indexed_truth` labeling
  discipline.
- [`/docs/commands/command_dispatch_contract.md`](../commands/command_dispatch_contract.md)
  — command-dispatch descriptor (ADR-0016). The descriptor mints
  rerun / cancel / quarantine / snapshot-apply / snapshot-rollback
  authority. A user-initiated rerun, a user-initiated quarantine, a
  user-initiated quarantine-lift, and a snapshot apply / rollback
  each cite a descriptor; the test surface never mints authority by
  itself, and a history row, a discovery refresh, or a flake
  classification never does.
- [`/docs/security/secret_broker_contract.md`](../security/secret_broker_contract.md)
  — credential-handle classes (ADR-0007). A test that depends on a
  credential-handle class not available in the current
  identity-mode resolves to `unsupported_on_this_host` with
  `unsupported_reason_class = missing_credential_class` rather than
  silently passing or failing.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  — quarantine, restart, and freshness floors. Session-quarantine
  inheritance is the typed authority class
  `quarantine_by_session_quarantine_inheritance` on the quarantine
  record; the `quarantine_by_supervisor_under_collapse_supersede`
  authority class re-projects the supervisor-collapse rule.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those documents
  disagree with this contract, those upstream documents win and this
  contract plus the companion schemas update in the same change.

## Why freeze this now

The test surface is one of the most overloaded shared vocabularies in
an IDE — the same word "passed" is used by the local cargo / pytest /
jest run, the cached discovery preview, the provider-imported result
chip, the parity-validated import row, the failed-only rerun
afford­ance, the snapshot / golden auto-update path, the AI tool-call
mediator, the support exporter, and the activity center digest.
Without one record family the failure modes are familiar:

- a "discovery" panel renders a tree that is actually two weeks
  cached, and the user runs a "rerun all" expecting a fresh
  collection — but the discovery never refreshed;
- a "passed" badge silently absorbs a quarantined case (the test was
  pinned under flake policy and never executed) or a skipped case
  (the test ran a `skip` directive and reported no assertion);
- a provider-imported result from a managed CI run is rendered as a
  local pass without a parity validation, and a developer who runs
  the same test locally against a drifted target sees a different
  result and cannot reconcile the two;
- a "rerun failed" affordance silently widens to "rerun all" when
  the prior run's failed subset is empty (because the prior run
  produced a partially-complete outcome with pending subsets), and
  the user pays the cost of a full rerun without realising;
- a snapshot / golden update is auto-applied without a baseline
  preview, and a reviewer cannot tell which files changed, what the
  prior baseline identity was, or how to roll back;
- a flake classification graduates from AI-suggested to "confirmed
  flake" without a paired observed signal, and a test is silently
  quarantined on AI inference alone;
- a quarantine expires and silently transitions to "lifted" with no
  re-evaluation run, and a developer trusts that a previously
  quarantined test has stabilised when the quarantine clock simply
  ran out;
- an admin-policy pack disables a test surface (the test depends on
  a credential class the current identity-mode does not grant, the
  test is marked managed-only running locally, the test would
  publish an irreversible side effect under workspace-trust
  restricted), and the renderer reports "skipped" — collapsing the
  typed `disabled_by_admin_policy` and `unsupported_on_this_host`
  states into the user's `skip` directive.

This contract makes those differences explicit before any test
runner, test tree panel, or AI test-selection assistant ships.
Discovery has typed provenance, freshness, and scope. Per-item state
is an eleven-value vocabulary that distinguishes passed from skipped,
filtered, quarantined, disabled-by-policy, unsupported-here, and
stale. Flaky history rides one record family with typed signals and
windows; quarantine rides one record family with typed authority,
reason, state, and expiry. Snapshot review requires a typed baseline-
identity preview with file-count truth and a typed rollback path
before the apply path is admitted.

## Scope

Frozen at this revision:

- the discovery record's discovery-provenance class (seven-value
  vocabulary covering local-authoritative / local-cached-no-fresh-
  run / provider-imported-only / provider-imported-with-local-
  parity / merged / ai-inferred-provisional / unknown-requires-
  review), the discovery-freshness class (six-value covering
  authoritative_live, warm_cached, degraded_cached,
  stale_partial_discovery, stale_full_discovery, and the unknown
  honesty row), the discovery-scope class (nine-value covering
  full_workspace, current_root, named_workset, sparse_slice,
  changed_files_only, failed_only_subset, single_test_id,
  policy_limited_view, and unknown), the host-boundary re-export
  (ten-value), the discovery-counts projection (eight typed counts
  including `quarantined_count` and `unsupported_here_count`), the
  freshness-floor invariant (`freshness_floor_breached = true`
  forces the freshness class to a stale or unknown value), and the
  closed denial / audit vocabularies for discovery records;
- the test-item record's test-item-kind class (eleven-value
  covering test_suite, test_module, test_class, test_case,
  parameterized_case_root, parameterized_case_instance,
  fixture_or_setup_node, doc_test, snapshot_or_golden_node,
  benchmark_case, and unknown), the test-item-state class (twelve-
  value with the eleven distinct states the spec requires plus the
  unknown honesty row), the canonical test-item id reservation, the
  parameterized-case lineage (`parameterized_case_root_ref` and
  `parameterized_case_instance_index`), the reserved flaky-history
  and quarantine cross-link refs, the typed `unsupported_reason_
  class` (eight-value covering host-os, host-arch, missing-runtime-
  capability, missing-credential-class, missing-managed-target,
  policy-disabled, broadened-capture-required, unknown), and the
  typed `skip_reason_summary` / `display_label` short-sentence
  fields;
- the coverage-handoff record's coverage-provenance class (six-
  value covering local-instrumented / provider-imported / merged /
  cached-no-fresh-run / no-coverage-collected / unknown), the
  coverage-freshness class (seven-value with the `no_coverage_to_
  freshen` honesty row), the coverage-counts projection (six typed
  counts), the typed `coverage_artifact_event_ref` requirement on
  local / merged provenance (the contract pins the coverage payload
  to the artifact rail), and the freshness-floor invariant;
- the test run-summary record's test-result-source class (seven-
  value covering local-execution-authoritative / local-execution-
  cached / provider-imported-only / provider-imported-with-local-
  parity-validation / merged-local-and-provider-with-parity-
  validation / ai-assisted-filter-then-local-execution / unknown),
  the parity-validation class (seven-value with the typed
  parity_drift_detected_minor / _major and the typed pending /
  skipped-under-policy honesty rows), the rerun-scope class (nine-
  value covering not-a-rerun / all-tests / failed-only /
  quarantined-only / changed-files-only / named-workset / explicit-
  test-id-set / until-first-failure / unknown), the by-state counts
  projection (eleven typed counts mirroring the per-item state
  vocabulary), and the typed `subset_test_item_refs` requirement on
  every subset rerun;
- the snapshot / golden review record's review-kind class (five-
  value covering pre-apply-preview-required, post-apply-audit,
  rollback-preview, auto-update-blocked-no-preview, and unknown),
  the baseline-identity class (six-value covering workspace-
  authoritative, vcs-committed, managed-workspace, provider-
  imported, no-prior-baseline-first-capture, and unknown), the
  per-snapshot change class (eight-value covering added, removed,
  updated-minor, updated-major, updated-redaction-only, unchanged,
  unreviewable-redaction-limited, and unknown), the typed
  `snapshot_change_summary` (file-count truth: added, removed,
  updated_minor, updated_major, updated_redaction_only, unchanged,
  unreviewable_redaction_limited, total_file_count), the snapshot-
  review-decision class (seven-value covering pending / accepted-
  apply-now / accepted-apply-pending-user-action / rejected /
  rolled-back / blocked-under-policy / unknown), the typed
  `command_dispatch_descriptor_ref` requirement on apply / rollback
  decisions, the typed `rollback_path_ref` requirement on every
  apply except first capture, and the typed `proposed_artifact_
  event_ref` requirement (the contract pins the proposed bytes to
  the artifact rail);
- the flaky-history record's flaky-classification class (seven-
  value covering not-flaky / suspected / confirmed-threshold-met /
  quarantined-flaky / cleared-after-stable-streak / history-
  insufficient / unknown), the flaky-signal class (eleven-value
  covering intermittent-pass-fail-pass / fail-then-pass-under-
  retry / order-dependent / time-or-clock-dependent / concurrency-
  or-thread-dependent / external-dependency-unstable / host-or-
  environment-dependent / resource-pressure-dependent / network-
  or-io-dependent / ai-inferred-provisional / unknown), the
  history-window class (six-value covering last-n-attempts / last-
  n-days / last-main-branch-only / last-session-only / policy-
  defined-rolling-window / unknown), the typed outcome-tally
  projection (eight typed counts), the AI-inference anchor rule
  (an `ai_inferred_signal_provisional` MUST be paired with an
  authoritative anchor ref or with at least one non-AI observed
  signal), and the typed `quarantine_record_ref` requirement on
  the quarantined-flaky classification;
- the quarantine record's quarantine-authority class (seven-value
  covering not-quarantined / by-user / by-admin-policy / by-
  supervisor / by-automated-flake-threshold / by-session-
  quarantine-inheritance / unknown), the quarantine-reason class
  (nine-value covering flaky-threshold / known-failing / environ-
  ment-dependent / external-dependency-unstable / hardware-
  dependent / policy-required / broadened-capture-required / ai-
  recommendation-pending-human-review / unknown), the quarantine-
  state class (seven-value covering active / expired-pending-re-
  evaluation / lifted-after-stable-streak / lifted-by-user / lifted-
  by-policy-change / superseded / unknown), the typed actor / policy /
  threshold-rule / predecessor refs per authority, the typed
  `expires_at` requirement on every active quarantine, the typed
  silent-lift-after-expiry forbidden, and the typed
  `command_dispatch_descriptor_ref` requirement on user-initiated
  open / lift events;
- the closed denial-reason vocabularies per record family (15 test-
  discovery-state denial reasons, 7 test-item denial reasons, 7
  coverage-handoff denial reasons; 14 test-run-summary denial
  reasons, 13 snapshot-review denial reasons; 9 flaky-history
  denial reasons, 13 quarantine denial reasons);
- the matched audit-event vocabularies per record family
  (discovery-state published / revoked / provenance-updated /
  freshness-narrowed / scope-narrowed plus the silent-collapse-of-
  partial-into-full forbidden; test-item published / state-
  transitioned / quarantine-attached / quarantine-lifted plus the
  silent-collapse-of-skipped-or-quarantined-into-passed forbidden;
  coverage-handoff published / revoked / provenance-updated /
  freshness-narrowed; test-run-summary published / revoked /
  state-counts-updated / parity-validation-updated plus the silent-
  collapse-of-partially-complete-into-passed, silent-collapse-of-
  provider-imported-into-local-pass, and silent-admit-of-failed-
  only-subset-as-full-rerun forbiddens; snapshot-review published /
  decision-updated / apply-admitted / apply-denied / rolled-back
  plus the silent-baseline-overwrite forbidden; flaky-history
  published / revoked / classification-updated / signal-attached /
  quarantine-attached / quarantine-lifted plus the silent-
  graduation-from-ai-inference-to-confirmed-flake forbidden;
  quarantine published / revoked / state-transitioned / extended /
  lifted / superseded plus the silent-lift-after-expiry forbidden;
  and per-family audit-denial-emitted rows);
- the `additionalProperties = false` posture on every record; raw
  command lines, raw stdout / stderr byte streams, raw env bodies,
  raw API request / response bodies, raw absolute paths, raw URLs,
  raw secret values, raw test names, raw assertion bodies, raw
  snapshot bytes, raw artifact bytes, and raw stack traces MUST
  NOT cross any of these boundaries — records carry refs, hashes,
  counts, and class labels only.

Out of scope at this revision:

- implementing per-framework adapters (cargo test, pytest, jest,
  vitest, gtest, go test, junit, dotnet test, BSP / BEP test
  reports, jupyter test runners, etc.); those continue to ride the
  task-event envelope and emit `structured_test_report` artifacts on
  the per-attempt artifact rail;
- the test runner UI surface, the test tree panel, the failed-only
  rerun affordance, the snapshot / golden review modal, the flake-
  badge / quarantine-chip rendering, and the activity-center digest
  for test rows;
- the queue-engine integration that decides whether a failed-only
  rerun rides the same admission lane as the prior run; the queue
  admission contract on
  [`/schemas/runtime/background_job.schema.json`](../../schemas/runtime/background_job.schema.json)
  remains authoritative and the test summary cites it through the
  parent attempt rather than re-minting a parallel admission
  vocabulary;
- the AI tool-call mediator's test selection / triage assistant; AI-
  inferred flaky signals and AI-suggested test selection cite this
  contract's anchor rules but the AI surface itself is out of scope;
- the policy-pack authoring surface for admin-defined quarantine
  rules and admin-defined coverage thresholds.

## 1. The test discovery-state record

The discovery-state record is the typed header for a test-tree
collection. It is the row a reviewer reads to answer "what tests have
we discovered, where did the discovery come from, how fresh is it,
what scope did it cover, and how many are quarantined / disabled /
unsupported here".

### 1.1 Provenance and freshness

`discovery_provenance_class` is the load-bearing field. The seven-
value vocabulary covers local-authoritative discovery, local-cached-
no-fresh-run discovery, provider-imported-only discovery, provider-
imported-with-local-parity-check discovery, merged local + provider
discovery, AI-inferred provisional discovery, and the
`discovery_provenance_unknown_requires_review` honesty row.

`discovery_freshness_class` is the typed honest answer for "is this
tree current". The six-value vocabulary covers `authoritative_live`,
`warm_cached`, `degraded_cached`, `stale_partial_discovery`,
`stale_full_discovery`, and the unknown honesty row. The contract's
load-bearing rule is the freshness-floor invariant: when the
discovery's `discovery_collected_at` is older than the freshness
floor for its provenance class, `freshness_floor_breached` MUST be
true and `discovery_freshness_class` MUST narrow to one of
`stale_partial_discovery`, `stale_full_discovery`, or
`discovery_freshness_unknown_requires_review`. A surface that
renders the discovery MUST narrow rerun / run-selected affordances
when the freshness class is below `authoritative_live`.

### 1.2 Scope

`discovery_scope_class` is the nine-value vocabulary covering
`full_workspace_scope`, `current_root_scope`, `named_workset_scope`,
`sparse_slice_scope`, `changed_files_only_scope`, `failed_only_
subset_scope`, `single_test_id_scope`, `policy_limited_view_scope`,
and the unknown honesty row. `policy_limited_view_scope` is the
typed honest answer when an admin policy pack redacted parts of the
tree; consumers MUST surface the typed limitation rather than
presenting a complete tree.

### 1.3 Per-item state vocabulary

`test_item_state_class` on the per-item record is the eleven-value
state vocabulary the spec requires, plus the unknown honesty row:

- `not_discovered_yet` — the typed honest answer for an item the
  discovery has not yet collected. A discovery in progress carries
  these rows; rendering them as `passed` is forbidden;
- `filtered_out_by_selection` — the typed honest answer for an item
  the user's selection (named workset, sparse slice, single-id
  scope) excluded;
- `skipped_by_test_directive` — the typed honest answer for an
  in-source `skip` / `xfail` / `ignore` directive;
- `quarantined_under_flake_policy` — the typed honest answer for an
  item currently quarantined. MUST cite a `quarantine_record_ref`;
- `disabled_by_admin_policy` — the typed honest answer for an item
  disabled by an admin policy pack (workspace-trust restricted,
  secret-class intersection, irreversible-action policy);
- `unsupported_on_this_host` — the typed honest answer for an item
  the current host boundary does not support. MUST cite a typed
  `unsupported_reason_class` (host-os mismatch, host-arch mismatch,
  missing runtime capability, missing credential class, missing
  managed target, policy disabled on this host, broadened-capture
  required not granted, or unknown);
- `passed`, `failed`, `errored_outside_assertion`, `timed_out` — the
  typed terminal pass / fail / errored / timed-out states.
  `errored_outside_assertion` is distinct from `failed` — it is the
  typed honest answer for a fixture / setup / teardown error that
  prevented the test body from running;
- `stale_partial_discovery_no_recent_run` — the typed honest answer
  for an item that was previously discovered but whose surrounding
  tree is now stale; rendering it as `passed` is forbidden;
- `test_item_state_unknown_requires_review` — the typed honesty row.

The contract guarantee is that all eleven distinct states remain
distinct: collapsing any of them into `passed` (in particular
collapsing `skipped_by_test_directive` or
`quarantined_under_flake_policy` or `disabled_by_admin_policy` or
`unsupported_on_this_host` into `passed`) is a typed denial under
`test_item_record_must_not_collapse_skipped_or_quarantined_into_
passed` paired with the matching audit-denial event.

### 1.4 Canonical test-item id and parameterized lineage

`canonical_test_item_id` is the stable cross-record join key for
testing extensions (failure history, evidence packets, coverage
overlays, AI suggestions). The canonical id MUST NOT encode a raw
absolute path or a raw URL; it is an opaque token resolved through
the owning registry. The reserved id is the contract's evidence path
that flake history, quarantine state, and coverage overlay records
all key off the same canonical row across discovery refreshes.

Parameterized cases ride a two-record split: the
`parameterized_case_root` carries the unexpanded canonical id, and
each `parameterized_case_instance` cites
`parameterized_case_root_ref` plus an ordinal
`parameterized_case_instance_index`. Per-instance flake history and
per-instance quarantine state are admissible; per-root flake history
is admissible when the policy aggregates across instances.

### 1.5 Reserved cross-links

`flaky_history_ref` and `quarantine_record_ref` on the test-item
record are reserved cross-links so later testing contracts (failure
history, evidence packets, coverage overlays, AI test selection)
extend one base model rather than fork it. The contract requires
that every `quarantined_under_flake_policy` item cite a
`quarantine_record_ref`; the typed denial is
`test_item_record_must_cite_quarantine_record_ref_when_quarantined`.

## 2. The coverage handoff

`coverage_handoff_record` is the typed bridge from a test attempt to
its coverage payload. It is per-attempt and rides the artifact rail.

### 2.1 Provenance and freshness

`coverage_provenance_class` covers local-instrumented, provider-
imported, merged, cached-no-fresh-run, no-coverage-collected, and the
unknown honesty row. The `no_coverage_collected` value is the typed
honest answer when the run produced no coverage payload; the typed
denial is `coverage_handoff_record_must_carry_typed_provenance_
class`. A reviewer MUST NOT see a cached prior coverage row
attributed to a run that itself produced no coverage.

`coverage_freshness_class` mirrors the discovery freshness vocabulary
with one addition (`no_coverage_to_freshen` for the typed honest
answer when no coverage payload exists).

### 2.2 Artifact-event handoff

`coverage_artifact_event_ref` is the typed handoff to the artifact
rail. The contract requires it on every `local_instrumented_run` and
`merged_local_and_provider_report` provenance — the artifact rail
remains the authoritative durable surface for the coverage bytes,
and the handoff record only projects provenance / freshness / counts
on top of it.

## 3. The test run-summary record

The run-summary record is the typed projection of how a test attempt
finished. It is one record per attempt and cites the parent run,
parent attempt, outcome event, and matched discovery state.

### 3.1 Result source

`test_result_source_class` is the seven-value vocabulary that names
where the result came from. The contract's load-bearing rules:

- `local_execution_authoritative` is the canonical typed value for a
  fresh local adapter run;
- `provider_imported_only` MUST cite a typed `parity_validation_
  class`. Rendering a provider-imported result as a local pass
  without a typed parity row is a typed denial under
  `test_run_summary_record_must_not_collapse_provider_imported_into_
  local_pass`;
- `provider_imported_with_local_parity_validation` is the typed
  value when a local re-run validated the provider import; the
  typed `parity_validation_class` resolves to one of `parity_
  validated_local_matches_provider`, `parity_drift_detected_minor`,
  `parity_drift_detected_major`, `parity_validation_pending_local_
  rerun`, `parity_validation_skipped_under_policy`, or the unknown
  honesty row;
- `merged_local_and_provider_with_parity_validation` is the typed
  value when both surfaces contributed and parity was validated;
- `ai_assisted_filter_then_local_execution` is the typed value for
  AI-suggested test selection that nonetheless ran the selected
  tests locally — the AI surface filters; it never substitutes for
  a local execution.

### 3.2 Rerun scope

`rerun_scope_class` is the typed answer to "what subset did this
rerun execute against". Composes with the `rerun_kind_class` on the
run record (exact replay vs current-context replay vs failed-step
retry vs automated / manual retry); the rerun scope is a test-
specific narrowing that names the subset selection.

The contract's load-bearing rule is the failed-only-rerun honesty:
`rerun_failed_only_subset` MUST carry at least one
`subset_test_item_refs` entry, and rendering it as
`rerun_all_tests_in_scope` is a typed denial under
`test_run_summary_record_must_not_admit_silent_collapse_of_failed_
only_subset_into_full_rerun`. The same rule applies to
`rerun_quarantined_only_subset`, `rerun_changed_files_only_subset`,
`rerun_named_workset_subset`, `rerun_explicit_test_id_set`, and
`rerun_until_first_failure_subset`.

### 3.3 By-state counts

`by_state_counts` mirrors the eleven-state per-item vocabulary so a
reviewer can read the summary and the discovery counts with the same
eyes. The counts pin partially-complete results: the typed denial
`test_run_summary_record_must_not_collapse_partially_complete_into_
passed` blocks any roll-up that hides a `failed_count` or
`errored_count` or `timed_out_count` behind a `lifecycle_state_class
= passed`.

## 4. The snapshot / golden review record

The snapshot / golden review record is the typed bridge from "an
adapter wants to update a baseline" to "a reviewer accepted (or
rejected, or rolled back) the update". Snapshot bytes ride the
artifact rail; the review record projects the typed metadata on top.

### 4.1 Review kind and baseline identity

`snapshot_review_kind_class` is the five-value vocabulary covering
`pre_apply_preview_required`, `post_apply_audit`, `rollback_preview_
for_local_history`, `auto_update_blocked_no_preview`, and the unknown
honesty row. `auto_update_blocked_no_preview` is the typed honest
refusal class when an adapter would have silently overwritten a
baseline; the matching denial reason is
`snapshot_review_record_must_not_admit_silent_baseline_overwrite`.

`snapshot_baseline_identity_class` names where the prior baseline
lived. `workspace_authoritative_baseline` is the typed value for a
durable user-authored row on the local workspace; `vcs_committed_
baseline` for a baseline committed to the VCS; `managed_workspace_
baseline` for a baseline on a managed workspace; `provider_imported_
baseline` for a baseline that rode a provider import;
`no_prior_baseline_first_capture` for a first capture (no rollback
target). The contract requires the typed `baseline_artifact_event_
ref` on every non-first-capture baseline so a reviewer can audit the
prior bytes alongside the proposed bytes; first capture nulls the
ref.

### 4.2 File-count truth

`snapshot_change_summary` carries the file-count truth: `added_
count`, `removed_count`, `updated_minor_count`, `updated_major_
count`, `updated_redaction_only_count`, `unchanged_count`,
`unreviewable_redaction_limited_count`, and `total_file_count`. The
contract requires every snapshot review to publish these counts so
a reviewer cannot accept an update without seeing how many files
would change. Per-snapshot change rides the
`snapshot_change_class` vocabulary.

### 4.3 Decision and rollback path

`snapshot_review_decision_class` is the seven-value vocabulary
covering `review_pending_no_decision`, `review_accepted_apply_now`,
`review_accepted_apply_pending_user_action`, `review_rejected_
keep_prior_baseline`, `review_rolled_back_to_prior_baseline`,
`review_blocked_under_policy`, and the unknown honesty row. The
contract's load-bearing rules:

- `review_accepted_apply_now` and `review_rolled_back_to_prior_
  baseline` MUST cite a `command_dispatch_descriptor_ref`;
- `review_accepted_apply_now` and `review_accepted_apply_pending_
  user_action` MUST cite a `rollback_path_ref` (the local-history /
  VCS / managed-workspace rollback target) unless the baseline is
  `no_prior_baseline_first_capture`;
- `review_blocked_under_policy` MUST cite a `denial_reason_class`
  on the record.

## 5. The flaky-history record

The flaky-history record aggregates per-test-item attempt outcomes
across a typed history window. It is per-canonical-test-item — flake
history persists across discovery refreshes by keying off
`canonical_test_item_id` rather than `test_item_record_id`.

### 5.1 Classification

`flaky_classification_class` is the seven-value vocabulary. The
contract's load-bearing rules:

- `suspected_flaky_recent_intermittent`, `confirmed_flaky_threshold_
  met`, and `quarantined_flaky_under_active_quarantine` MUST cite at
  least one `observed_signals` entry. An empty signal set on these
  classifications is a typed denial under `flaky_history_record_
  must_cite_observed_signals_when_classified_flaky`;
- `quarantined_flaky_under_active_quarantine` MUST cite a typed
  `quarantine_record_ref`;
- `cleared_after_stable_run_streak` MUST cite a typed
  `stable_streak_length`;
- `history_insufficient_to_classify` is the typed honest answer for
  a too-small sample; rendering it as `not_flaky_stable_history`
  is a typed denial.

### 5.2 AI inference anchor rule

`flaky_signal_class` includes an `ai_inferred_signal_provisional`
value for AI-derived intermittency signals (a model that observed a
suspicious pattern in a transcript / artifact). The contract's anchor
rule is that an `ai_inferred_signal_provisional` entry MUST be
paired with at least one of:

- a non-AI observed signal in the same `observed_signals` array
  (`intermittent_pass_then_fail_then_pass`, `fail_then_pass_under_
  retry_only`, `order_dependent_pass_only_in_isolation`, etc.); or
- a non-empty `ai_inference_anchor_refs` list (each ref pointing at
  an authoritative anchor row from one of the other truth classes
  per the truth-class vocabulary).

The typed denial is `flaky_history_record_must_not_admit_silent_
graduation_from_ai_inference_to_confirmed_flake`. The contract's
evidence path is that a confirmed flake classification can never
ride only on AI inference.

### 5.3 History window

`history_window_class` covers `last_n_attempts`, `last_n_days`,
`last_main_branch_only`, `last_session_only`, `policy_defined_
rolling_window`, and the unknown honesty row. The companion
`history_window_size` pins the sample size so a reviewer can
reproduce the threshold computation. `outcome_tally` projects the
per-state attempt counts; the typed `total_attempt_count` MUST
equal the sum of the seven typed per-state counts.

## 6. The quarantine record

The quarantine record is the typed authority record that pins a test
under flake / known-failing / environment-dependent / under-
investigation policy. It is per-canonical-test-item and append-only:
state transitions (active → expired-pending-re-evaluation → lifted /
superseded) mint new audit-event refs on the same record; lift /
supersede paths are typed; raw expiry timer races are forbidden.

### 6.1 Authority and actor binding

`quarantine_authority_class` is the seven-value vocabulary covering:

- `not_quarantined` — the honest answer when no quarantine is in
  force;
- `quarantine_by_user_via_command_dispatch` — MUST cite a
  `command_dispatch_descriptor_ref`;
- `quarantine_by_admin_policy_pack` — MUST cite an
  `admin_policy_epoch_ref`;
- `quarantine_by_supervisor_under_collapse_supersede` — re-projects
  the supervisor-collapse rule from the run / attempt contract;
- `quarantine_by_automated_flake_threshold_policy` — MUST cite an
  `automated_threshold_rule_ref`;
- `quarantine_by_session_quarantine_inheritance` — re-projects the
  session-quarantine row from the fault-domains contract;
- `quarantine_authority_unknown_requires_review` — the typed
  honesty row.

The typed actor binding is the contract's evidence that a reviewer
can tell who quarantined a test and under what authority.

### 6.2 State and expiry

`quarantine_state_class` is the seven-value vocabulary covering
`active_quarantine`, `expired_pending_re_evaluation`,
`lifted_after_stable_streak`, `lifted_by_user_via_command_dispatch`,
`lifted_by_policy_change`, `superseded_by_newer_quarantine`, and
the unknown honesty row.

The contract's load-bearing rule is the silent-lift-after-expiry
forbidden: every `active_quarantine` MUST carry a typed `expires_at`
deadline; past the deadline, the typed state MUST narrow to
`expired_pending_re_evaluation` rather than silently transitioning
to `lifted_*`. A re-evaluation run that confirms stability transitions
to `lifted_after_stable_streak` (with a typed `stable_streak_length`
on the matched flaky-history record); a re-evaluation run that
confirms continued flake mints a fresh active quarantine. The typed
denial is `quarantine_record_must_not_admit_silent_lift_after_
expiry`.

### 6.3 Reason

`quarantine_reason_class` is the nine-value vocabulary. `ai_assist_
recommendation_pending_human_review` is the typed honest answer for
an AI-suggested quarantine that has not yet been confirmed by a
human; the reason MUST NOT silently graduate to `flaky_threshold_
exceeded` without a human review (the matching audit-denial pair on
the flaky-history record is `flaky_history_record_silent_graduation_
from_ai_inference_to_confirmed_flake_forbidden_denial`).

## 7. Invariants the contract guarantees

The eight invariants the contract guarantees on the discovery /
test-item / coverage / run-summary / snapshot / flaky-history /
quarantine rail:

| Invariant | Where it lives | Failure mode when missing |
|---|---|---|
| Discovery provenance and scope are first-class | `test_discovery_state_record.discovery_provenance_class` and `discovery_scope_class` are required; `freshness_floor_breached = true` forces freshness to a stale or unknown value; the typed denial is `test_discovery_state_record_must_not_admit_silent_collapse_of_partial_into_full_discovery` | A two-week-old cached tree is rendered as authoritative live truth and a "rerun all" silently runs against the stale tree |
| Per-item state distinctions are preserved | `test_item_record.test_item_state_class` is the eleven-value vocabulary plus the unknown row; collapsing skipped, quarantined, disabled-by-policy, unsupported-here, or stale into passed is a typed denial under `test_item_record_must_not_collapse_skipped_or_quarantined_into_passed` | A quarantined or skipped case is rolled up into a `passed` badge and a developer cannot tell that the test never ran |
| Provider-imported results require a typed parity row | `test_run_summary_record.test_result_source_class` of provider-imported / merged MUST cite a `parity_validation_class`; the typed denial is `test_run_summary_record_must_not_collapse_provider_imported_into_local_pass` | A managed-CI pass is rendered as a local pass without parity validation and the developer cannot reconcile a divergent local re-run |
| Failed-only rerun is named, not assumed | `test_run_summary_record.rerun_scope_class = rerun_failed_only_subset` MUST carry a non-empty `subset_test_item_refs`; the typed denial is `test_run_summary_record_must_not_admit_silent_collapse_of_failed_only_subset_into_full_rerun` | A "rerun failed" affordance silently widens to "rerun all" when the failed subset is empty, paying full-rerun cost |
| Snapshot updates require a typed preview with baseline identity, file-count truth, and rollback path | `snapshot_review_record` requires `snapshot_review_kind_class`, `snapshot_baseline_identity_class`, `snapshot_change_summary` with `total_file_count`, and `rollback_path_ref` on every apply except first capture; the typed denial is `snapshot_review_record_must_not_admit_silent_baseline_overwrite` | A snapshot / golden auto-update silently overwrites a baseline with no preview, no file-count truth, and no rollback target |
| Flake classification cannot ride only on AI inference | `flaky_history_record` requires `ai_inferred_signal_provisional` to be paired with a non-AI observed signal or a non-empty `ai_inference_anchor_refs` list; the typed denial is `flaky_history_record_must_not_admit_silent_graduation_from_ai_inference_to_confirmed_flake` | A test is silently classified as confirmed flake (and quarantined) on a single AI suggestion with no observed signal |
| Quarantine has typed authority, reason, state, and expiry; expiry never silently lifts | `quarantine_record` requires `quarantine_authority_class` + actor ref, `quarantine_reason_class`, `quarantine_state_class`, and `expires_at` on active state; `expired_pending_re_evaluation` is the typed mid-state past expiry; the typed denial is `quarantine_record_must_not_admit_silent_lift_after_expiry` | An expired quarantine silently transitions to lifted with no re-evaluation, and a developer trusts a still-flaky test |
| Coverage freshness and provenance are first-class; counts ride the typed handoff | `coverage_handoff_record.coverage_provenance_class` is required; `coverage_artifact_event_ref` is required for local / merged provenance (the artifact rail is authoritative for bytes); the freshness-floor invariant narrows freshness to stale on breach | Stale or imported coverage is rendered as authoritative-live and a developer trusts a coverage gauge that is two weeks old |

## 8. Out-of-band attestations and audit events

Every record family carries a closed `audit_event_id` vocabulary so a
reviewer can read the chronology without raw bodies.

The discovery / item / coverage family covers discovery-state
published / revoked / provenance-updated / freshness-narrowed /
scope-narrowed plus the silent-collapse-of-partial-into-full
forbidden; test-item published / state-transitioned / quarantine-
attached / quarantine-lifted plus the silent-collapse-of-skipped-or-
quarantined-into-passed forbidden; coverage-handoff published /
revoked / provenance-updated / freshness-narrowed; and per-family
audit-denial-emitted rows.

The test-run-summary / snapshot family covers test-run-summary
published / revoked / state-counts-updated / parity-validation-
updated plus the silent-collapse-of-partially-complete-into-passed,
silent-collapse-of-provider-imported-into-local-pass, and silent-
admit-of-failed-only-subset-as-full-rerun forbiddens; snapshot-
review published / decision-updated / apply-admitted / apply-denied /
rolled-back plus the silent-baseline-overwrite forbidden; and per-
family audit-denial-emitted rows.

The flaky-history / quarantine family covers flaky-history published /
revoked / classification-updated / signal-attached / quarantine-
attached / quarantine-lifted plus the silent-graduation-from-ai-
inference-to-confirmed-flake forbidden; quarantine published /
revoked / state-transitioned / extended / lifted / superseded plus
the silent-lift-after-expiry forbidden; and per-family audit-
denial-emitted rows.

Raw command lines, raw stdout / stderr byte streams, raw env bodies,
raw API request / response bodies, raw absolute paths, raw URLs, raw
secret values, raw test names, raw assertion bodies, raw snapshot
bytes, and raw stack traces MUST NOT appear on any audit event.

## 9. Versioning

Each schema declares its own `*_schema_version` const at value `1`:

- `test_discovery_state.schema.json` — `test_discovery_state_schema_
  version = 1`;
- `test_run_summary.schema.json` — `test_run_summary_schema_
  version = 1`;
- `flaky_history.schema.json` — `flaky_history_schema_version = 1`.

Adding a new enum member, a new optional field, or a new sub-record
kind is additive-minor and bumps the matching version const.
Repurposing an existing enum member or removing one is breaking and
requires a new decision row. Adding a new record kind to the
top-level `oneOf` of a schema is additive-minor as long as it does
not change the semantics of an existing record kind. Adding a new
schema file under `schemas/execution/` is additive-minor under the
execution-family rules in
[`/artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml).
