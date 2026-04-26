# Run / attempt, artifact-event, queue / admission, and rerun semantics contract

This document freezes one user-visible execution model for tasks, runs,
tests, API sends, notebook actions, provider jobs, AI tool calls,
debug sessions, automation workflows, format / lint runs, support
exports, and other task-like flows before separate surfaces invent
incompatible lifecycle vocabularies.

Every execution-shaped surface (the task launcher, the test runner UI
and CLI, the build target launcher, the API client send button, the
notebook cell run action, the provider-job admin surface, the AI tool
call mediator, the debug session prep flow, the automation workflow
runner, the support export packager, the replay / import probes, the
durable activity center, and any history / evidence export) reads the
same record family. There is one run truth record, one attempt-record
family with typed input-request packets, one artifact-event record
family on the per-attempt artifact rail, one outcome-event record per
attempt outcome, and one rerun-comparison record between two attempts —
not a separate hidden lifecycle vocabulary per surface.

Machine-readable companions:

- [`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json)
  — the `run_record` (the stable identity for the user-visible
  request), the `outcome_event_record` (typed pass / fail / partially-
  complete / cancelled / stale-output result with side-effect summary),
  and the `rerun_comparison_record` (typed exact-replay vs current-
  context-replay vs failed-step-retry comparison with per-layer drift
  status).
- [`/schemas/execution/attempt.schema.json`](../../schemas/execution/attempt.schema.json)
  — the `attempt_record` (per-execution truth, append-only, one per
  attempt of the parent run) and the `attempt_input_request_record`
  (typed prompts for rerun-intent confirmation, irreversible-action
  confirmation, credential-handle selection, secret-mode entry, yes / no
  continuation, checkpoint resume / restart, and broadened-capture opt-
  in, with typed timeout / expiry).
- [`/schemas/execution/artifact_event.schema.json`](../../schemas/execution/artifact_event.schema.json)
  — the `artifact_event_record` for the per-attempt artifact rail
  (streaming chunks, one-shot emissions, finalize / withdraw /
  replace / fail-to-publish events, retention and redaction posture,
  and per-artifact side-effect attribution).
- [`/fixtures/execution/run_cases/`](../../fixtures/execution/run_cases/)
  — worked YAML fixtures covering the required scenarios (queued local
  run, managed job with artifact stream, waiting-input step, partially
  complete outcome, exact-vs-current-context rerun comparison).

This contract composes with and does not replace:

- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  and
  [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  — execution-context root, target identity, toolchain identity,
  workset / scope vocabulary, trust state, identity-mode envelope,
  policy epoch (ADR-0009, ADR-0018, ADR-0001). Every run cites this
  root through its `context_summary` and every attempt cites a
  context-snapshot resolved at attempt-open.
- [`/schemas/execution/context_snapshot.schema.json`](../../schemas/execution/context_snapshot.schema.json)
  and
  [`/docs/execution/context_inspector_packet.md`](./context_inspector_packet.md)
  — the inspectable execution-context snapshot, the structured
  snapshot diff, and the inspector-view projection. The run cites the
  snapshot the launching surface saw; the attempt cites the snapshot
  resolved at attempt-open. The rerun-comparison record's per-layer
  drift mirrors the snapshot diff's `diff_layer` set.
- [`/schemas/runtime/background_job.schema.json`](../../schemas/runtime/background_job.schema.json)
  and
  [`/docs/runtime/background_queue_contract.md`](../runtime/background_queue_contract.md)
  — queue lane, work class, budget domain, workload lane, collapse
  policy, checkpoint policy, staleness policy, and cancellation
  contract. The run / attempt admission summary re-projects these
  rows; queue admission decisions ride the typed
  `queue_admission_class` vocabulary frozen here.
- [`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json)
  and
  [`/docs/tooling/task_event_contract_seed.md`](../tooling/task_event_contract_seed.md)
  — the canonical lower-level task-event envelope for build / test /
  diagnostic / artifact-publication events. The run / attempt layer
  sits **above** the envelope: an attempt cites a `task_event_
  envelope_trace_id`, and an artifact-event MAY cite the matching
  `task_event_envelope_event_id` for an `artifact_publication`
  payload. The two layers do not collapse; the envelope retains
  source / adapter / confidence truth, and the run / attempt retains
  the user-visible request truth.
- [`/schemas/ux/activity_event_envelope.schema.json`](../../schemas/ux/activity_event_envelope.schema.json)
  and
  [`/docs/ux/attention_activity_taxonomy.md`](../ux/attention_activity_taxonomy.md)
  — durable job-row, activity-center, and quiet-hours rules. The
  run record's `history_row_ref` resolves to a durable activity row;
  toast / banner / digest deliveries cite the run's
  `event_lineage_ref`.
- [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  and
  [`/docs/ux/notification_delivery_contract.md`](../ux/notification_delivery_contract.md)
  — canonical event-id and notification-routing model. The run /
  attempt / artifact-event rails cite the same lineage so transient
  deliveries (toast, banner, OS notification, digest) and durable
  history (activity center, support export, evidence packet) share
  one event id rather than duplicating object identity.
- [`/docs/commands/command_dispatch_contract.md`](../commands/command_dispatch_contract.md)
  — command-dispatch descriptor (ADR-0016). The descriptor mints
  cancel / rerun / withdraw authority. A run, an attempt, an
  irreversible-action confirmation, and a withdraw event each cite a
  descriptor; the run record never mints authority by itself, and a
  history row, an artifact id, or a rerun affordance never does.
- [`/docs/security/secret_broker_contract.md`](../security/secret_broker_contract.md)
  — credential-handle classes (ADR-0007). Secret-mode prompts,
  credential-handle selection prompts, broadened-capture opt-in
  prompts, and any artifact event that intersects credential-handle
  classes cite class refs from this boundary.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  — quarantine, restart, and freshness floors. The
  `cancellation_authority_class` re-projects supervisor / quarantine /
  policy / disconnect rows; the
  `admission_denied_by_session_quarantine` and
  `cancellation_by_workspace_trust_revocation` values read directly
  from those rules.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — shared `Limited`, `Stale`, `Blocked`, and downgrade language; the
  `partially_complete` / `stale_output` / `*_unknown_requires_review`
  honesty values reuse it.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those documents
  disagree with this contract, those upstream documents win and this
  contract plus the companion schemas update in the same change.

## Why freeze this now

The run / attempt model is the most overloaded surface in an IDE — the
same word "run" is used by the task launcher, the test runner, the
build target launcher, the debugger, the API client, the notebook, the
provider job admin, the AI tool call mediator, the automation flow,
the format / lint pass, the support export packager, and any "rerun
this" affordance the user ever taps. Without a single record family
the failure modes are familiar:

- a "rerun" button silently switches between exact replay (frozen
  inputs / target / runtime) and current-context replay (resolved at
  rerun time), and the user cannot tell which one happened until the
  result diverges from the prior run;
- a partially-complete result (some test cases passed, some still
  pending, some failed) is collapsed into a misleading single "last
  result" badge;
- a stale-output run (the prior output's inputs / target / runtime /
  profile have drifted) is presented as a fresh pass / fail without a
  drift cue;
- an attempt that reached a terminal failure has its logs and
  artifacts overwritten by the next attempt's logs and artifacts, and
  the prior attempt's evidence is no longer reviewable;
- a queue-admission denial (workspace-trust restricted, admin policy
  pack, session quarantine, secret-class intersection, irreversible
  action without explicit confirmation) is presented as "running" or
  "queued" with no typed reason;
- a cancellation authority is implied rather than typed, and a
  reviewer cannot tell whether the user, the supervisor, the policy
  pack, or a remote disconnect cancelled an attempt;
- an artifact rail (streaming logs, structured test reports, build
  binaries, AI response chunks, network response bodies) flows
  through ad-hoc per-surface objects, and a finalized artifact is
  silently overwritten when a rerun produces a new one;
- an irreversible side effect (a deploy, a published artifact, a sent
  message) is published, and the run still surfaces a "rerun" button
  that would re-publish or re-send without an explicit confirmation;
- a notification (toast, banner, OS notification, digest) routes
  through its own per-surface event vocabulary and dismissing it
  silently mutates the durable history row.

This contract makes those differences explicit before any apply path
ships. A run is one stable user-visible request; each attempt is one
typed execution of it; the artifact rail is per-attempt and append-
only; the rerun-comparison is typed; queue admission is a closed
vocabulary with typed denial reasons; cancellation authority is typed
and bound to an explicit actor; outcome events are append-only and
the partially-complete / stale-output rows are first-class.

## Scope

Frozen at this revision:

- the run record's run-kind class (fourteen-value vocabulary covering
  task / test / build-target / format-or-lint / debug-session /
  notebook-action / api-call-send / provider-job / ai-tool-call /
  automation-workflow / support-export / replay-probe / import-probe
  runs plus `run_kind_unknown_requires_review`), the host-boundary
  class (ten-value), the lifecycle-state class (ten-value covering
  `queued`, `preparing`, `running`, `waiting_input`,
  `partially_complete`, `passed`, `failed`, `cancelled`,
  `stale_output`, and `lifecycle_state_unknown_requires_review`), the
  queue-admission class (thirteen-value with typed denial reasons),
  the rerun-kind class (seven-value with the four typed rerun shapes
  the contract supports), the side-effect summary class (ten-value
  including `side_effects_irreversible_action_published` as the typed
  honest answer that narrows the rerun affordance), the rerun-drift
  layer / status pair (eleven-value layer × six-value status), and
  the typed `auto_rerun_admitted_on_this_record` boolean that is
  always false when an irreversible side effect has been published;
- the attempt record's attempt-kind class (seven-value covering
  initial / exact-replay / current-context-replay / failed-step-retry /
  automated-retry / manual-retry / unknown), the host-boundary class
  re-export, the queue-admission class re-export, the lifecycle-state
  class re-export, the cancellation-authority class (ten-value with
  typed actor binding), the typed predecessor / failed-step-checkpoint
  refs, the typed `task_event_envelope_trace_id` cross-link, and the
  invariant that an attempt is append-only — a new attempt mints a new
  attempt_record with its own `attempt_record_id`;
- the attempt-input-request record's input-request kind class (ten-
  value covering rerun-intent confirmation, irreversible-action
  confirmation, credential-handle selection, visible-echo line entry,
  secret / password modes, yes / no continuation, checkpoint resume /
  restart, and broadened-capture opt-in), the typed result class (ten-
  value), the typed result-summary class that resolves to
  `no_summary_secret_or_redacted` on every secret / credential prompt,
  the typed `expires_at` deadline, the typed
  `irreversible_action_summary_class` that resolves on every
  irreversible-action confirmation, and the typed
  `command_dispatch_descriptor_ref` requirement on rerun-intent and
  irreversible-action confirmations;
- the artifact-event record's artifact-event kind class (eleven-value
  covering streaming chunk, one-shot, finalize, withdraw-by-{newer,
  user, policy, quarantine}, fail-to-publish, retained-under-
  redaction, pending-finalization, and unknown), the artifact class
  (nineteen-value covering log / console / test report / diagnostic /
  coverage / build binary / archive / doc bundle / AI response chunk /
  AI tool invocation / network request and response / notebook cell
  output / provider job artifact / support bundle segment / transcript
  export segment / exact-build-identity attestation / side-effect
  audit / unknown), the finalization class (seven-value), the
  retention class (six-value re-export), the media class (eighteen-
  value re-export with binary classes added), the side-effect
  attribution class (seven-value), the typed
  `replacement_artifact_ref` requirement on `artifact_withdrawn_
  replaced_by_newer`, the typed `withdrawal_actor_ref` requirement on
  `artifact_withdrawn_by_{user,policy,quarantine}`, and the invariant
  that a finalized artifact MUST NOT be silently overwritten;
- the outcome-event record's outcome-kind class (six-value), the
  failure class (sixteen-value), the typed
  `subset_pass_count` / `subset_fail_count` / `subset_pending_count`
  trio required on every `outcome_partially_complete`, the typed
  `stale_output_drift_layer` required on every `outcome_stale_output`,
  and the typed `side_effect_summary_class` required on every outcome;
- the rerun-comparison record's typed two-attempt scope, the per-
  layer drift entries (with the side-effect layer required), the
  typed `rerun_kind_class` (which MUST NOT be
  `not_a_rerun_initial_attempt` or unknown on a comparison record),
  the typed redaction-limited reason vocabulary, and the typed
  side-effect summaries on both sides;
- the closed denial-reason vocabularies per record family (twelve
  run-record denial reasons, six outcome-event denial reasons, six
  rerun-comparison denial reasons; eighteen attempt-record denial
  reasons, eight attempt-input-request denial reasons; twelve
  artifact-event denial reasons);
- the matched audit-event vocabularies per record family (run-record
  published / revoked / attempt-added / lifecycle-transitioned /
  queue-admission-decided / cancellation-admitted / cancellation-
  denied plus the silent-collapse-of-attempts / silent-overwrite-of-
  attempt-history / silent-rerun-after-irreversible-side-effect
  forbiddens; outcome-event published plus the silent-collapse-of-
  partially-complete-into-passed and silent-collapse-of-outcome-
  history forbiddens; rerun-comparison published plus the silent-
  collapse-of-layers and silent-omission-of-side-effect-layer
  forbiddens; attempt-record published / lifecycle-transitioned /
  input-request-opened / input-request-closed-with-typed-result /
  artifact-event-attached / outcome-event-attached / cancellation-
  admitted / cancellation-denied plus the silent-overwrite / silent-
  collapse / silent-admit-under-quarantine forbiddens; attempt-input-
  request opened / closed-with-typed-result / timed-out plus the
  silent-secret-capture and silent-irreversible-admit forbiddens;
  artifact-event published / streaming-chunk-attached / finalized /
  withdrawn / retention-posture-narrowed / retention-posture-
  broadened plus the silent-overwrite-of-finalized-artifact / silent-
  admit-under-secret-intersection / silent-admit-under-broadened-
  capture forbiddens; and per-family audit-denial-emitted rows);
- the `additionalProperties = false` posture on every record; raw
  command lines, raw stdout / stderr byte streams, raw env bodies,
  raw API request / response bodies, raw absolute paths, raw URLs,
  raw secret values, raw artifact bytes, raw provider responses, and
  raw stack traces MUST NOT cross any of these boundaries — records
  carry refs, hashes, counts, and class labels only.

Out of scope at this revision:

- implementing the full activity-center surface, the run launcher
  surface, the rerun comparison view, the artifact rail viewer, or
  the support-export packager;
- implementing the queue engine, the supervisor, the cancellation
  broker, the credential broker, or the approval-ticket issuer;
- implementing per-source adapters (cargo, pytest, bazel, BSP, BEP,
  Jupyter kernels, AI tool runtimes, provider jobs, network sends,
  etc.); those continue to ride the task-event envelope;
- the final UI affordances for chips, badges, drift indicators,
  rerun-comparison diff views, irreversible-action confirmation
  modals, or notification routing;
- the conflict-resolution policy when a rerun races with a still-in-
  flight prior attempt (named here through the queue-admission and
  cancellation vocabularies but the reconciler is its own decision
  row).

## 1. The run record

The run record is the stable identity for the user-visible request.
It is the row a reviewer reads to answer "what was asked, where, and
under what queue / admission rules". It does **not** carry the per-
execution artifacts, logs, or outcome events — those ride attempts,
the artifact rail, and outcome events.

### 1.1 Run kind and host boundary

`run_kind_class` is the load-bearing field. It decides which
adapters MAY emit task events under the run, which artifact classes
are admissible on its rail, and which queue lane the run defaults to.
The fourteen-value vocabulary covers task / test / build-target /
format-or-lint / debug-session / notebook-action / api-call-send /
provider-job / ai-tool-call / automation-workflow / support-export /
replay-probe / import-probe runs and the `run_kind_unknown_requires_
review` honesty row.

`host_boundary_class` is the ten-value vocabulary that decides which
target-identity witness the run MUST cite, which boundary cue MUST be
visible in the launcher, and which capability limits a renderer MUST
surface. Every non-local boundary requires a remote target-identity
witness ref; container / devcontainer boundaries require the
container target-identity ref; `host_boundary_unknown_requires_review`
fails closed and forbids any mutating attempt.

### 1.2 Context summary

`context_summary` is the compact projection of the snapshot the run
launcher saw. It carries `execution_context_id`,
`context_snapshot_ref`, `target_class`, `canonical_target_id`,
`toolchain_class`, `toolchain_id`, `scope_class`, `trust_state`,
`identity_mode`, `policy_epoch`, and a typed `secret_class_count`.
The summary never re-mints the snapshot vocabulary; consumers that
need the canonical record resolve the snapshot ref.

### 1.3 Queue admission summary

`queue_admission_summary` is the typed projection of the queue's
decision: the admission class (admitted / queued-pending-* /
admission-deferred-by-quiet-hours / admission-held-pending-high-risk-
review / admission-denied-by-{trust,policy,quarantine,secret-class,
irreversible-without-explicit-confirmation} / unknown), the queue
lane id, the work class id, the budget domain id, and an optional
ref to the powering `background_job_record`. Held / denied admission
classes fail closed and MUST cite a denial-reason class on the
record.

### 1.4 Lifecycle roll-up

`lifecycle_state_class` on the run is a roll-up from the latest
attempt's state and the outcome-event log. The roll-up MUST NOT
collapse `partially_complete` or `stale_output` into `passed`; doing
so is a typed denial under
`run_record_must_not_admit_silent_collapse_of_outcome_events` and
emits the matching audit-denial event.

### 1.5 Rerun lineage

`rerun_kind_class` on the run is one of the seven-value rerun-kind
labels. The four typed rerun shapes the contract supports are:

- **exact replay** (`exact_replay_same_inputs_same_target_same_
  runtime`) — the prior run's frozen inputs, target, and runtime are
  reused as-is. Useful for reproducing a result against a known-good
  context;
- **current-context replay** (`current_context_replay_resolved_at_
  rerun_time`) — the inputs are resolved at rerun time against the
  current execution context. The canonical reason a rerun produces a
  different result; the rerun-comparison record names which layers
  drifted;
- **failed-step retry** (`failed_step_retry_only_resume_from_last_
  checkpoint`) — only the failed step is retried, not the whole
  run. Requires a failed-step checkpoint on the prior attempt;
- **automated retry** / **manual retry** — the run was retried by a
  policy-bound supervisor (automated) or by an explicit user action
  (manual). Manual retries are the typed answer when no automated
  retry policy applies.

A run that is itself a rerun MUST cite `original_run_ref` and SHOULD
cite `rerun_chain_origin_run_ref` so a reviewer can walk the chain
back to its origin without resolving each link.

### 1.6 Append-only attempts

`attempt_refs` is an ordered list. The contract guarantee is that
attempts are append-only: a per-step retry inside the same run mints
a new attempt with `attempt_index = prior + 1`; a rerun of the run as
a whole mints a new run record with its own `attempt_index = 0`.
Collapsing two attempts into one — silently overwriting the prior
attempt's context, logs, or artifacts — is a typed denial under
`run_record_must_not_admit_silent_overwrite_of_attempt_history`.

### 1.7 Auto-rerun gate after irreversible side effects

`auto_rerun_admitted_on_this_record` MUST be `false` on every run
whose latest outcome event carries `side_effect_summary_class =
side_effects_irreversible_action_published`. The contract forbids
silent rerun on a run that has already published an irreversible
side effect (a deploy, a published artifact, a sent message); an
explicit user-initiated fresh run under a fresh command-dispatch
descriptor is the only path back to a new attempt. The matching
denial reason is
`run_record_must_narrow_rerun_when_irreversible_side_effect_
published`.

## 2. The attempt record and the typed input-request packet

The attempt is the per-execution truth. Every attempt carries its own
context snapshot, its own host boundary, its own queue admission
posture, its own input-request packets, its own outcome events, and
its own artifact-event refs. The attempt history is append-only and
MUST NOT be collapsed into a misleading single "last result".

### 2.1 Attempt kind and predecessor

`attempt_kind_class` is one of seven values. The `failed_step_retry_
attempt`, `automated_retry_after_transient_failure_attempt`, and
`manual_retry_user_initiated_attempt` kinds MUST cite a
`predecessor_attempt_ref`; `failed_step_retry_attempt` MUST also cite
a `failed_step_checkpoint_ref`. `initial_attempt` MUST null both.

The `attempt_kind_class` of an attempt MAY differ from its parent
run's `rerun_kind_class`: a rerun of a run as a whole mints a new
run record (with the run-level rerun-kind set) whose first attempt is
`initial_attempt` from the attempt's perspective. This separation
keeps run-level rerun shape distinct from per-attempt retry shape.

### 2.2 Per-attempt context snapshot

`context_snapshot_ref_at_attempt` is the typed evidence that two
attempts of the same run may resolve different contexts. An exact-
replay attempt's `execution_context_id_at_attempt` equals the parent
run's `context_summary.execution_context_id`; a current-context-
replay attempt's may differ — that is the canonical reason a rerun
produces a different result, and the rerun-comparison record's per-
layer drift names which layers drifted.

### 2.3 Per-attempt queue admission

`queue_admission_summary` on the attempt is the typed projection of
the queue's admission decision for this attempt — not for the run as
a whole. A run may sit in `queued_pending_dependency` while one
attempt is `admitted_into_lane` and another is `queued_pending_
capacity`. Each attempt mints its own queue-admission summary;
collapsing them is a typed denial under
`run_record_must_not_admit_silent_collapse_of_outcome_events`.

### 2.4 Lifecycle, cancellation authority, and waiting input

The lifecycle vocabulary is the same ten-value set as the run record.
`waiting_input` is the typed honest mid-state — an attempt in
`waiting_input` MUST carry at least one open
`attempt_input_request_record` whose result class is
`result_pending_no_typed_result_yet`.

`cancellation_authority_class` is one of ten values. Every value
outside `not_cancelled` and `cancellation_authority_unknown_requires_
review` MUST cite a `cancellation_actor_ref` and pin
`lifecycle_state_class = cancelled`. The typed actor binding is the
contract's evidence that a reviewer can tell whether the user, the
supervisor, the admin policy pack, the session quarantine, the
workspace-trust revocation, the timeout, the remote disconnect, or
an irreversible-side-effect block initiated the cancellation.

### 2.5 Outcome events on terminal states

Every attempt that reaches a terminal lifecycle state (`passed`,
`failed`, `cancelled`, `stale_output`, `partially_complete`) MUST
carry at least one `outcome_event_record_ref` on its `outcome_event_
refs` list. An attempt without an outcome event is non-conforming.
The typed denial is
`attempt_record_must_carry_outcome_event_when_completed`.

### 2.6 Cross-link to the task-event envelope

`task_event_envelope_trace_id` is the optional ref to the lower-level
trace id under
[`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json).
Stable consumers (support exports, replay harnesses, benchmark
harnesses) cite the trace id so per-attempt task events are not
flattened across attempts. The two layers do not collapse: the
envelope keeps source / adapter / confidence truth, and the attempt
keeps the user-visible execution truth.

### 2.7 The attempt-input-request packet

The `attempt_input_request_record` is the typed prompt the host
opens against the user when an attempt needs a value, a confirmation,
a credential-handle selection, or a broadened-capture opt-in. It is
distinct from the terminal `input_request_record` in
[`/schemas/execution/command_boundary_marker.schema.json`](../../schemas/execution/command_boundary_marker.schema.json):
terminal input requests serve a running shell; attempt input
requests serve an attempt that needs a typed value before it can
advance to the next lifecycle state.

The contract's load-bearing rules:

- **Secret-mode prompts** (`input_value_secret_no_echo`,
  `input_value_password_managed_echo_mask`, `credential_handle_
  selection`) MUST cite at least one `credential_handle_class_refs`
  entry, MUST resolve `result_summary_class` to
  `no_summary_secret_or_redacted`, and MUST null
  `result_summary_text`. The typed denial when these rules are
  violated is `attempt_input_request_record_must_resolve_secret_
  summary_to_no_summary` (paired with the silent-secret-capture
  forbidden audit denial).
- **Irreversible-action confirmations** (`irreversible_action_
  confirmation`) MUST cite both `irreversible_action_summary_class`
  and `command_dispatch_descriptor_ref`. The typed denial is
  `attempt_input_request_record_must_cite_irreversible_action_
  summary_when_irreversible`.
- **Rerun-intent confirmations** (`rerun_intent_confirmation`) MUST
  cite a `command_dispatch_descriptor_ref`. The typed denial is
  `attempt_input_request_record_must_carry_typed_result_class` paired
  with the silent-irreversible-admit forbidden audit denial.
- **Broadened-capture opt-in confirmations** MUST cite an
  `approval_ticket_ref`. The typed denial is
  `attempt_input_request_record_must_cite_approval_ticket_ref_when_
  broadened_capture`.
- **Timeouts**: every input request carries a typed `expires_at`
  deadline. Past the deadline, an open prompt MUST close with
  `result_class = result_timed_out_no_value_provided`; reopening
  requires a fresh `attempt_input_request_record` (no implicit
  authority extension).

## 3. The artifact-event record and the per-attempt artifact rail

The artifact rail is the durable history of what an attempt
produced. It is per-attempt and append-only.

### 3.1 Streaming vs one-shot

`artifact_event_kind_class` separates streaming (`artifact_emitted_
streaming_chunk`) from one-shot (`artifact_emitted_complete_one_
shot`) emissions. Streaming chunks accumulate under one
`artifact_ref` with monotonic `chunk_index`; the artifact does not
become finalized until a separate `artifact_finalized_published`
event is emitted. The intermediate state is the typed
`finalization_pending_streaming_in_progress`.

### 3.2 Finalize, withdraw, replace

`artifact_finalized_published` is the typed event that marks an
artifact as immutable. Once finalized, the artifact MUST NOT be
silently overwritten; the typed denial is `artifact_event_record_
must_not_silently_overwrite_finalized_artifact`. A replacement
mints a new `artifact_event_record` with `artifact_event_kind_class
= artifact_withdrawn_replaced_by_newer` on the prior artifact (which
MUST cite `replacement_artifact_ref`) and `artifact_finalized_
published` on the replacement.

The withdraw events come in three typed authority flavours:
`artifact_withdrawn_by_user` (cite the command-dispatch descriptor
in `withdrawal_actor_ref`), `artifact_withdrawn_by_policy` (cite the
admin-policy epoch), and `artifact_withdrawn_by_quarantine` (cite
the quarantine row). Each resolves `finalization_class` to
`finalization_withdrawn_no_artifact_remains`.

### 3.3 Retention and redaction

`retention_class` re-exports the `raw_payload_retention_class`
vocabulary from the task-event envelope:
`not_retained_ephemeral` / `retained_local_support_bundle_only` /
`retained_local_with_replay_opt_in` / `retained_managed_with_
redaction` / `retained_managed_with_broadened_capture` /
`retention_class_unknown_requires_review`. The retained-* values
require a non-null `raw_artifact_body_ref`; `not_retained_ephemeral`
MUST null both `raw_artifact_body_ref` and `raw_artifact_hash`.
`retained_managed_with_broadened_capture` and `redaction_class =
broadened_capture` both require an `approval_ticket_ref`.

### 3.4 Per-artifact side-effect attribution

`side_effect_attribution_class` lets the artifact rail roll up into
the run-level side-effect summary. The typed `side_effect_irreversible_
action_published_attributable_to_artifact` value MUST narrow the
parent run record's `auto_rerun_admitted_on_this_record` to `false`;
that is the contract's evidence path from "this artifact represented
a deploy / publication / send" to "this run cannot be silently
rerun".

### 3.5 Cross-link to the task-event envelope

`task_event_envelope_event_id` is the optional ref to the matching
task-event envelope event (typically an `artifact_publication`
payload). Stable consumers correlate the run-level artifact rail
with the lower-level task-event stream without flattening either
model.

## 4. Outcome events and the partially-complete / stale-output rules

The outcome event is the typed result of one attempt. Outcome events
are append-only — a later attempt mints additional outcome events on
its own attempt-record; the prior outcome events are never overwritten.

### 4.1 The six-value outcome-kind vocabulary

- `outcome_passed` — typed clean pass. MUST resolve lifecycle to
  `passed` and MUST null all subset counts;
- `outcome_failed` — typed clean fail. MUST cite a `failure_class`
  (one of sixteen values: tool_exit_nonzero, tool_crash, timeout,
  cancelled_by_*, precondition_failed, input_request_timed_out,
  environment_drift, host_boundary_unreachable, queue_admission_denied,
  side_effect_irreversible_block, policy_denied, adapter_error, or
  unknown);
- `outcome_partially_complete` — typed mid-result. MUST cite the
  three subset counts (`subset_pass_count`, `subset_fail_count`,
  `subset_pending_count`). Collapsing this into `outcome_passed` is
  a typed denial under
  `outcome_event_record_must_not_collapse_partially_complete_into_
  passed`;
- `outcome_cancelled` — typed cancellation outcome. MUST cite a
  `failure_class`;
- `outcome_stale_output` — the typed honest answer when an attempt's
  prior output no longer maps to the current context. MUST cite
  `stale_output_drift_layer` (one of the eleven `rerun_drift_layer_
  class` values: inputs / target / runtime / profile / capsule /
  secret / policy / trust / queue / host / side-effect);
- `outcome_event_kind_unknown_requires_review` — the typed honesty
  row that fails closed.

### 4.2 The side-effect summary

Every outcome event carries a `side_effect_summary_class` (one of
ten typed values). The contract's load-bearing rule is that a
`side_effects_irreversible_action_published` outcome locks the run
record's `auto_rerun_admitted_on_this_record` to `false` until a
fresh user-initiated run mints a new run record with a fresh
command-dispatch descriptor.

## 5. The rerun-comparison record

The `rerun_comparison_record` is the typed bridge from "the prior
attempt produced X" to "the rerun produced Y because layer Z drifted".
It compares two attempts of two runs (not just two runs) so per-
attempt context, logs, and artifacts cannot be flattened across an
attempt boundary.

### 5.1 The eleven-layer × six-status grid

The eleven layers (`inputs_layer`, `target_identity_layer`, `runtime_
identity_layer`, `profile_layer`, `environment_capsule_layer`,
`secret_posture_layer`, `policy_epoch_layer`, `trust_state_layer`,
`queue_admission_layer`, `host_boundary_layer`, `side_effect_layer`)
mirror the snapshot diff's `diff_layer` set with two additions
(queue / host / side-effect) the snapshot does not carry. Each
layer entry resolves a typed status (`preserved_no_drift`, `drift_
detected_minor`, `drift_detected_major`, `drift_detected_irreversible_
side_effect`, `drift_status_redaction_limited`, `drift_status_
unknown_requires_review`) plus an optional class-label / hash / count
summary on each side.

### 5.2 The side-effect layer is required

Every rerun-comparison record MUST include a layer entry whose
`layer = side_effect_layer`. The typed denial is `rerun_comparison_
record_must_carry_side_effect_summary` (paired with the silent-
omission-of-side-effect-layer forbidden audit denial). The two
side-effect summary class fields (`side_effect_summary_class_a`,
`side_effect_summary_class_b`) project the typed summary on each
side without re-deriving it from the layer entry.

### 5.3 Rerun kind on the comparison

`rerun_kind_class` on a comparison record MUST NOT be `not_a_rerun_
initial_attempt` or `rerun_kind_unknown_requires_review`. Comparing
two attempts of the same run (a per-step retry against the prior
attempt) and comparing two attempts of two runs (a rerun against the
prior run) both ride this record; the four typed rerun shapes (exact
replay, current-context replay, failed-step retry, automated /
manual retry) are the same vocabulary.

## 6. Invariants the contract guarantees

The eight invariants the contract guarantees on the run / attempt /
artifact rail:

| Invariant | Where it lives | Failure mode when missing |
|---|---|---|
| Attempt history is append-only | `run_record.attempt_refs` is ordered; attempt_record_id is opaque-stable per attempt; the typed denial is `run_record_must_not_admit_silent_overwrite_of_attempt_history` | A user (or an AI assist) sees the latest attempt's logs / artifacts and assumes they reflect the entire run; the prior attempt's evidence is invisibly gone |
| Per-attempt context, logs, and artifacts cannot be collapsed into a misleading single "last result" | `run_record.attempt_refs` carries every attempt; outcome events cite their parent_attempt_ref; artifact events cite their parent_attempt_ref; the typed denial is `run_record_must_not_collapse_attempts_into_single_last_result` | A "last result" badge silently elides a partially-complete or stale-output mid-result |
| Partially-complete outcomes never collapse into passed | `outcome_event_record.outcome_event_kind_class = outcome_partially_complete` carries the three subset counts; the typed denial is `outcome_event_record_must_not_collapse_partially_complete_into_passed` | A test run with five passes, two pending, and one fail surfaces as "passed" |
| Stale-output outcomes name the typed drift layer | `outcome_event_record.stale_output_drift_layer` is required when `outcome_event_kind_class = outcome_stale_output`; the typed denial is `outcome_event_record_must_cite_stale_output_drift_layer_when_stale` | A rerun against a drifted target / runtime / profile reports "passed" without naming what drifted |
| A finalized artifact is never silently overwritten | `artifact_event_record` finalize event resolves `finalization_class = finalization_published_immutable`; replacement requires a paired withdraw + new finalize event with `replacement_artifact_ref`; the typed denial is `artifact_event_record_must_not_silently_overwrite_finalized_artifact` | A rerun's new build binary silently replaces the prior run's binary under the same artifact id; the prior evidence is gone |
| Cancellation authority is typed and bound to an explicit actor | `attempt_record.cancellation_authority_class` outside `not_cancelled` / unknown MUST cite `cancellation_actor_ref`; the typed denial is `attempt_record_must_cite_cancellation_authority_when_cancelled` | A reviewer cannot tell whether the user, the supervisor, the admin policy pack, or a remote disconnect cancelled an attempt |
| Auto-rerun is forbidden after an irreversible side effect | `run_record.auto_rerun_admitted_on_this_record = false` whenever the latest outcome carries `side_effects_irreversible_action_published`; the typed denial is `run_record_must_narrow_rerun_when_irreversible_side_effect_published` | A "rerun" button silently re-publishes a deploy or re-sends an external message |
| Open / queued / running / waiting-input / partially-complete / stale-output / passed / failed / cancelled remain distinct | `lifecycle_state_class` is shared between run and attempt; `lifecycle_state_unknown_requires_review` fails closed; the typed audit denial is `run_record_silent_collapse_of_attempts_forbidden_denial` | The lifecycle silently collapses waiting-input / partially-complete / stale-output into running / passed without a typed reason |

## 7. Out-of-band attestations and audit events

Every record family carries a closed `audit_event_id` vocabulary so a
reviewer can read the chronology without raw bodies.

The run / outcome-event / rerun-comparison family covers run published
/ revoked / attempt-added / lifecycle-transitioned / queue-admission-
decided / cancellation-admitted / cancellation-denied, plus the
silent-collapse-of-attempts / silent-overwrite-of-attempt-history /
silent-rerun-after-irreversible-side-effect / audit-denial denials;
outcome-event published plus the silent-collapse-of-partially-
complete-into-passed and silent-collapse-of-outcome-history denials;
rerun-comparison published plus the silent-collapse-of-layers and
silent-omission-of-side-effect-layer denials.

The attempt / attempt-input-request family covers attempt published /
lifecycle-transitioned / input-request-opened / input-request-closed-
with-typed-result / artifact-event-attached / outcome-event-attached /
cancellation-admitted / cancellation-denied, plus the silent-overwrite-
of-attempt-history / silent-collapse-with-other-attempt / silent-
admit-under-quarantine / audit-denial denials; attempt-input-request
opened / closed-with-typed-result / timed-out, plus the silent-
secret-capture and silent-irreversible-admit denials.

The artifact-event family covers artifact-event published / streaming-
chunk-attached / finalized / withdrawn / retention-posture-narrowed /
retention-posture-broadened-with-approval-ticket, plus the silent-
overwrite-of-finalized-artifact / silent-admit-under-secret-
intersection / silent-admit-under-broadened-capture / audit-denial
denials.

Raw command lines, raw stdout / stderr byte streams, raw env bodies,
raw API request / response bodies, raw absolute paths, raw URLs, raw
secret values, raw artifact bytes, and raw stack traces MUST NOT
appear on any audit event.

## 8. Versioning

Each schema declares its own `*_schema_version` const at value `1`:

- `run.schema.json` — `run_schema_version = 1`;
- `attempt.schema.json` — `attempt_schema_version = 1`;
- `artifact_event.schema.json` — `artifact_event_schema_version = 1`.

Adding a new enum member, a new optional field, or a new sub-record
kind is additive-minor and bumps the matching version const.
Repurposing an existing enum member or removing one is breaking and
requires a new decision row. Adding a new record kind to the
top-level `oneOf` of a schema is additive-minor as long as it does
not change the semantics of an existing record kind. Adding a new
schema file under `schemas/execution/` is additive-minor under the
execution-family rules in
[`/artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml).
