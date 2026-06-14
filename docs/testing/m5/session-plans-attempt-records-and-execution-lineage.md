# M5 session plans, attempt-record histories, and execution lineage

This document is the contract for the **session plans** and **append-only
attempt-record histories** the M5 test-intelligence lane normalizes onto. Where
the scope-compatible selection contract makes durable discovery targets safely
selectable and re-runnable, this contract makes the *actual execution* of those
selections attributable: a run is a durable `SessionPlan` plus an ordered,
append-only history of `AttemptRecord`s — not a transient line of terminal
output.

A test session stays trustworthy only if its identity, attempt history, and
target / toolchain / env lineage survive across local, remote, notebook, and
imported-provider flows, and only if an imported / provider-backed verdict can
never masquerade as a local rerun. This contract makes both guarantees
structural.

## Source of truth

- Packet type: `SessionAttemptLedgerPacket`
  (`crates/aureline-runtime/src/session_plans_attempt_records_and_execution_lineage/`).
- Boundary schema:
  `schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json`.
- Checked support export:
  `artifacts/testing/m5/session-plans-attempt-records-and-execution-lineage/support_export.json`.
- Markdown summary:
  `artifacts/testing/m5/session-plans-attempt-records-and-execution-lineage.md`.
- Protected fixtures:
  `fixtures/testing/m5/session-plans-attempt-records-and-execution-lineage/`.

Regenerate the canonical export and summary after any shape change:

```bash
cargo run -p aureline-runtime --example dump_session_attempt_ledger
cargo run -p aureline-runtime --example dump_session_attempt_ledger summary
```

## Session plans

A `SessionPlan` ties a stable `session_id` and `plan_id` to:

- a `SessionFlow` (`local_workspace`, `remote_target`, `notebook_kernel`, or
  `imported_provider`) — the attributability axis;
- a `SessionPlanMode` (`run_selected`, `watch`, `rerun_all`, `rerun_failed`, or
  `import_provider_join`);
- the `selection_ref` and `snapshot_ref` it executes, so the originating
  selection and discovery snapshot stay reconstructable;
- an `ExecutionLineage` block (`runtime_token`, `toolchain_token`,
  `env_capsule_token`, `target_class`, `provenance_class`, and an `imported` flag);
- a `RetryPolicyClass` and `WatchPolicyClass`;
- a set of `LedgerTarget`s pinned at plan time.

The packet validation requires the four flows and the `run_selected`,
`rerun_failed`, and `import_provider_join` modes to each be represented, so the
parity across flows is exercised, not merely declared.

Identity rules every session obeys (`SessionPlan::is_valid`):

- **A target's fingerprint is never its bare id.** Each `LedgerTarget` carries a
  `target_fingerprint_token` distinct from its `target_id`
  (`fingerprint_substitutes_identity`).
- **Templates stay distinct from invocations.** A target carries a
  `DurableTestNodeKind`; the packet requires both `parameterized_template` and
  `concrete_invocation` to appear (`template_collapsed_with_invocation`).
- **Imported markers agree.** An imported plan (by flow, mode, lineage, or any
  imported target) must use the `imported_no_retry` retry policy and the
  `imported_not_watchable` watch policy; a local plan must use neither
  (`session_imported_markers_inconsistent`).
- The plan's `flow` agrees with its lineage's target class and provenance.

## Execution lineage

`ExecutionLineage` carries only non-display fingerprint tokens — never raw env
bodies, host names, or provider payloads. Its `imported` flag, `target_class`, and
`provenance_class` are mutually consistent (`ExecutionLineage::imported_consistent`):
an imported lineage is `provider_backend` / `imported_read_only` and carries a
`provider_token`; a non-imported lineage carries neither. The `target_class` and
`provenance_class` must agree on a single flow.

## Attempt-record histories

An `AttemptRecord` is one append-only attempt in a session. Each attempt carries
its own `ExecutionLineage`, so a single ledger can hold attempts from more than
one flow — a local initial run, a local failed-only rerun, an imported CI join,
and a local parity rerun — with per-attempt lineage keeping each honest.

| outcome | meaning |
| --- | --- |
| `queued` / `running` | local attempt in flight |
| `passed` / `failed` / `errored` | local / remote / notebook verdict |
| `imported` | imported / provider-backed result (never a local pass / fail) |
| `imported_stale` | imported evidence that must not roll up green |
| `unknown_requires_review` | unclassifiable; blocks an automatic green roll-up |

The imported outcome vocabulary (`imported`, `imported_stale`) is disjoint from
the local vocabulary so an imported verdict can never read as a local pass.

Append-only history rules (`AttemptRecord` + ledger validation):

- **Indices are unique and contiguous from one** within a session
  (`attempt_indices_not_contiguous`).
- **Every predecessor ref resolves to an earlier attempt in the same session**
  (`predecessor_chain_invalid`); a `rerun`, `rerun_failed`, or
  `local_parity_rerun` must record a predecessor.
- **Covered targets belong to the session** (`attempt_target_unresolved`).
- **An imported attempt cannot read as a local rerun**
  (`imported_attempt_reads_as_local`): an imported attempt (by flow, kind,
  lineage, or outcome) carries imported lineage, an imported outcome, and an
  `origin_provider_ref`; a local attempt carries none of these and is never an
  `imported_join` kind.
- **No green over stale / quarantine** (`green_over_stale_or_unknown`): an
  imported attempt may never carry a `passed` outcome.

## Reconstructability

Every attempt names its `session_ref`, and the packet validation requires it to
resolve to a session present in the export (`attempt_session_unresolved`). The
`LedgerConsumerProjection` block records that notifications, support exports,
review packets, and release gating all reopen the same session / attempt objects
instead of replaying the work or scraping UI text — so the exact session, attempt
history, and lineage used for any rerun, triage, or release decision can be
reconstructed from the packet alone.

## Boundary discipline

The packet carries only typed class tokens, booleans, opaque ids, fingerprint
digests, and redaction-aware reviewable labels. Raw test source, raw provider
payloads, provider cursors, credentials, host names, and raw artifact bodies never
cross this boundary.
