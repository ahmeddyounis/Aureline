# M5 lifecycle transition-safety contract

This lane is the **transition-safety capstone** on top of the frozen
[M5 lifecycle-state and journey-checkpoint matrix](m5_lifecycle_matrix_contract.md). The matrix
freezes, for every long-lived M5 object family, an explicit state machine drawn from the controlled
lifecycle vocabulary. This lane certifies that each of those state machines is **safe to move
through**: that the object exposes safe retry / cancel / rollback / compensation transition rules,
attributes every transition to a controlled actor or subsystem, cannot skip a required review /
checkpoint / rollback state behind an anonymous spinner, and keeps local editing as the protected
fallback when a managed lane degrades — and that the same state-truth vocabulary survives a headless
or companion-adjacent execution.

The lane exists so that M5 can honestly ship its growing mix of notebook, data/API, AI, remote,
preview, operator, docs, and release surfaces without a workflow hiding an ambiguous half-ready or
maybe-applied state behind one generic spinner or success banner, and without a managed,
collaborative, AI, or remote lane stranding the user with no safe local path when it degrades.

## Governed object families

The certification covers exactly the thirteen governed object families the matrix freezes, and
refuses to ship if any is missing:

- `workspace` — Workspace / window session
- `extension` — Installed extension / capability
- `remote_session` — Remote / tunnel session
- `collaboration_session` — Live collaboration session
- `ai_action` — AI assistant action
- `update_rollback` — Update / rollback lifecycle
- `notebook_runtime` — Notebook kernel runtime
- `request_api_run` — Request / API run
- `preview_session` — Preview / live-server session
- `pipeline_run` — Pipeline / task run
- `data_session` — Data / database session
- `profiler_capture` — Profiler / trace capture
- `companion_session` — Companion / paired device session

Every attribute a row certifies over — the explicit state machine (the admitted controlled states,
always including `ready`), the named recovery affordance the local fallback anchors on, the declared
consumer surfaces, and the applicable downgrade triggers — is pulled straight from the frozen
matrix's seeded packet, so this lane mints no parallel lifecycle vocabulary and cannot certify a
family, or a transition, the matrix does not freeze.

## Certified transition dimensions

Each row is certified across the four transition-safety dimensions the spec requires every
long-lived M5 object's state machine to hold (`safe_transition`, `transition_attribution`,
`checkpoint_sequencing`, `local_fallback`):

- **safe transition** — `safe_retry_cancel_rollback_rules` (green: every transition is restartable
  or compensatable — a retry cannot double-apply, a cancel cannot strand a half-applied change, and
  a rollback/compensation path is always reachable), a disclosed `disclosed_reduced_transition_set`
  where a reduced set of safe transitions is exposed on a subset of surfaces — e.g. deferring cancel
  until a reconnect resolves (yellow), or `unsafe_or_missing_transition_rules` (red: the object
  allowed an unsafe or missing transition that could double-apply, strand, or skip its
  rollback/compensation).
- **transition attribution** — `actor_subsystem_attributed` (green), a disclosed
  `disclosed_coarse_attribution` where a transition is attributed to a coarse subsystem group rather
  than the exact actor until the specific attribution resolves (yellow), or
  `attribution_missing_on_transition` (red: the object stopped attributing a transition to any actor
  or subsystem).
- **checkpoint sequencing** — `required_checkpoints_enforced` (green), a disclosed
  `disclosed_compacted_checkpoints` where the required checkpoints are presented compacted while
  still named individually (yellow), or `required_checkpoint_skipped` (red: a protected journey
  skipped a required review/checkpoint/rollback state or fell back to an anonymous spinner).
- **local fallback** — `local_editing_protected_fallback` (green), a disclosed
  `disclosed_reduced_fallback` where a reduced local-editing fallback is kept — e.g. read-only local
  edits until the lane rejoins (yellow), or `local_fallback_lost` (red: the object lost its
  protected local-editing fallback).

In addition, every row carries `headless_parity_preserved`: a hard invariant that the same
state-truth vocabulary survives a headless or companion-adjacent execution. A row that loses it
**blocks** (a `state_vocabulary_drift` cause), because a headless run must not report a different
transition and state language than the in-product surface.

## Derived status and the certification lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when any of the
four transition dimensions takes a disclosed narrowing. It drops to `red` when a transition is
unsafe or missing, a transition attribution goes missing, a required review/checkpoint/rollback
state is skipped, the protected local-editing fallback is lost, headless/companion-adjacent parity
is lost, or the row fails to certify every declared consumer surface. The consumer-surface
completeness check is the lint that prevents a certification from silently regressing into a partial,
single-surface view — the exact regression that would let a protected flow hide a half-ready or
maybe-applied state behind one generic spinner on the surfaces it did not certify. The Rust
validator in `crates/aureline-shell/src/m5_lifecycle_transition_safety` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_reduced_fallback` narrowing — the
sensitive narrowing that reduces the protected local-editing fallback — must additionally carry an
active, matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact transition causes, and the blocking findings
  the lane refuses to ship with.
- **Certification dashboard** — a light projection the product UI / CLI / diagnostics / support /
  telemetry automation reads to auto-narrow a governed object family's transition-safety claim when
  its certification falls out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix ref, build
  id, each object family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels — never raw
URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety`) is the
only mint-from-truth path for:

- `artifacts/release/m5-lifecycle-transition-safety-proof/packet.json`
- `artifacts/release/m5-lifecycle-transition-safety-proof/dashboard.json`
- `artifacts/release/m5-lifecycle-transition-safety-proof/support_export.json`
- `artifacts/release/m5-lifecycle-transition-safety-proof/matrix.csv`
- `artifacts/lifecycle/m5-lifecycle-transition-safety.md` (this report's rendered companion)
- `fixtures/state/m5-lifecycle-transition-safety/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/lifecycle/m5-lifecycle-transition-safety.schema.json`](../../schemas/lifecycle/m5-lifecycle-transition-safety.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety -- validate
cargo test -p aureline-shell --test m5_lifecycle_transition_safety_fixtures
cargo test -p aureline-shell m5_lifecycle_transition_safety
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety --"
$BIN packet         > artifacts/release/m5-lifecycle-transition-safety-proof/packet.json
$BIN dashboard      > artifacts/release/m5-lifecycle-transition-safety-proof/dashboard.json
$BIN support-export > artifacts/release/m5-lifecycle-transition-safety-proof/support_export.json
$BIN csv            > artifacts/release/m5-lifecycle-transition-safety-proof/matrix.csv
$BIN markdown       > artifacts/lifecycle/m5-lifecycle-transition-safety.md
$BIN packet         > fixtures/state/m5-lifecycle-transition-safety/packet.json
$BIN dashboard      > fixtures/state/m5-lifecycle-transition-safety/dashboard.json
$BIN support-export > fixtures/state/m5-lifecycle-transition-safety/support_export.json
$BIN compact        > fixtures/state/m5-lifecycle-transition-safety/compact.txt
```
