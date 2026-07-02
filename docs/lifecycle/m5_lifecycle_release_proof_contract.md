# M5 lifecycle release proof contract

This lane is the **release-evidence capstone** on top of the frozen
[M5 lifecycle-state and journey-checkpoint matrix](m5_lifecycle_matrix_contract.md). The matrix
freezes, for every long-lived M5 object family, an explicit state machine, one visible primary
status surface, one exportable status code, one controlled last-failure reason, one named recovery
affordance, and an ordered inventory of milestone checkpoints. This lane produces the release-grade
evidence that, for every one of those thirteen object families, its **lifecycle-state truth**,
**checkpoint truth**, and **recovery-affordance truth** hold across **all six claimed M5 desktop
profiles** and survive **every exported truth surface** — UI, CLI/headless, docs/help, diagnostics,
support exports, telemetry, and claim publication — with the same state-truth vocabulary preserved
in headless and companion-adjacent execution.

The lane exists so M5 cannot honestly ship its growing mix of notebook, data/API, AI, remote,
preview, operator, docs, and release surfaces while object state, checkpoint boundaries, or recovery
vocabulary still drift by surface or disappear in export paths. Any family that still collapses its
state into generic loading or error behavior is **automatically narrowed or blocked from stable
promotion** rather than shipping an over-claim.

## Governed object families

The proof covers exactly the thirteen governed object families the matrix freezes, and refuses to
ship if any is missing:

- `workspace` — Workspace / window session
- `extension` — Installed extension
- `remote_session` — Remote / tunnel session
- `collaboration_session` — Collaboration session
- `ai_action` — AI assistant action
- `update_rollback` — Update / rollback
- `notebook_runtime` — Notebook runtime
- `request_api_run` — Request / API run
- `preview_session` — Preview / live-server session
- `pipeline_run` — Pipeline / task run
- `data_session` — Data / database session
- `profiler_capture` — Profiler / trace capture
- `companion_session` — Companion / paired-device session

Every binding a row certifies — the driving matrix journey, the explicit state machine (admitted
states), the primary status surface, the status-code export field, the last-failure-reason field,
the named recovery affordance, the checkpoint lineage, the declared consumer surfaces, the
applicable downgrade triggers, and the controlled last-failure reason classes — is pulled straight
from the frozen matrix's seeded packet, so this lane mints no parallel lifecycle vocabulary and
cannot certify a family the matrix does not freeze.

## Certified proof dimensions

Each row is certified across the four proof dimensions the acceptance criteria and implementation
requirements demand:

- **lifecycle-state truth** — `explicit_state_truth_certified` (green), a disclosed
  `disclosed_reduced_state_truth` where a handful of intermediate states are grouped into one
  disclosed grouped state while the terminal controlled state is still named (yellow), or
  `state_collapsed_into_generic_loading_or_error` (red: the object hid its controlled state behind a
  generic loading or error behavior).
- **checkpoint truth** — `named_checkpoint_truth_certified` (green), a disclosed
  `disclosed_compacted_checkpoint_truth` where two adjacent milestones are folded into one disclosed
  compacted milestone while each terminal checkpoint is still named (yellow), or
  `checkpoints_collapsed_to_anonymous_spinner` (red: the object collapsed its ordered milestones into
  one anonymous spinner).
- **recovery-affordance truth** — `named_recovery_and_reason_certified` (green), a disclosed,
  **waivered** `disclosed_reduced_recovery_truth` where the named recovery affordance is deferred to a
  linked action while the controlled last-failure reason is still named (yellow), or
  `recovery_or_reason_truth_missing` (red: the object dropped its recovery affordance or reason).
- **exported-proof parity** — `exported_surfaces_reflect_current_proof` (green), a disclosed
  `disclosed_partial_export_refresh` where one legacy export refreshes on a slower cadence while still
  disclosing the lag (yellow), or `exported_proof_stale_or_divergent` (red: an exported surface
  overclaims relative to the current lifecycle truth).

## Auto-narrowing and the stable-promotion gate

Row status is **derived, never asserted**. A row is `green` only when all four proof dimensions hold
at full standing, headless/companion-adjacent parity is preserved, and the row certifies its truth
across all six claimed desktop profiles, keeps all three truth pillars, and certifies every declared
consumer surface. It drops to `yellow` on any single disclosed narrowing, and to `red` on any hard
blocker: collapsed state, collapsed checkpoints, missing recovery affordance or reason, a stale or
divergent exported surface, a headless-parity loss, an incomplete profile set, an incomplete
truth-pillar set, or an uncertified consumer surface. A disclosed reduced recovery truth may stay
`yellow` only when an active waiver discloses it. `all_rows_publishable` is the stable-promotion
gate: it is `true` only when no row is blocked.

## Published evidence

The headless emitter `aureline_shell_m5_lifecycle_release_proof` is the single mint-from-truth path:

- schema — `schemas/lifecycle/m5-lifecycle-release-proof.schema.json`
- markdown report — `artifacts/lifecycle/m5-lifecycle-release-proof.md`
- certification packet — `artifacts/release/m5-lifecycle-release-proof-proof/packet.json`
- certification dashboard — `artifacts/release/m5-lifecycle-release-proof-proof/dashboard.json`
- support export — `artifacts/release/m5-lifecycle-release-proof-proof/support_export.json`
- matrix CSV — `artifacts/release/m5-lifecycle-release-proof-proof/matrix.csv`
- protected fixtures — `fixtures/state/m5-lifecycle-release-proof/packet.json`,
  `fixtures/state/m5-lifecycle-release-proof/dashboard.json`,
  `fixtures/state/m5-lifecycle-release-proof/support_export.json`, and
  `fixtures/state/m5-lifecycle-release-proof/compact.txt`

Product UI, CLI, diagnostics, Support Center, telemetry, Shiproom, the release center, and docs/help
all resolve the same certification rows — green/yellow/red status, active waivers, and the exact
conformance causes — through this one proof rather than restating lifecycle-state, checkpoint, or
recovery posture by hand.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_release_proof -- validate
cargo test -p aureline-shell --test m5_lifecycle_release_proof_fixtures
```

The Rust validator in `crates/aureline-shell` is the authoritative gate; this contract documents the
shape and the controlled vocabulary.
