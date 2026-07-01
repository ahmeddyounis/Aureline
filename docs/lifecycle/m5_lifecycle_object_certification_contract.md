# M5 lifecycle-object certification contract

This lane is the **lifecycle-object capstone** on top of the frozen
[M5 lifecycle-state and journey-checkpoint matrix](m5_lifecycle_matrix_contract.md). The matrix
freezes, for every long-lived M5 object family, an explicit state machine, one visible primary
status surface, one exportable status code, one controlled last-failure reason, and one named
recovery affordance. This lane certifies that those four user-facing lifecycle **bindings**
actually hold across every consumer surface for every governed object family, and that the same
state-truth vocabulary survives a headless or companion-adjacent execution.

The lane exists so that M5 can honestly ship its growing mix of notebook, data/API, AI, remote,
preview, operator, docs, and release surfaces without object state, status codes, last-failure
reasons, or recovery vocabulary drifting by surface or disappearing in export paths: support,
docs, CLI/headless, and telemetry all describe the same state.

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

Every binding a row certifies — the primary status surface, the status-code field, the
last-failure-reason field, the named recovery affordance, the declared consumer surfaces, and the
applicable downgrade triggers — is pulled straight from the frozen matrix's seeded packet, so this
lane mints no parallel lifecycle vocabulary and cannot certify a family, or a binding, the matrix
does not freeze.

## Certified lifecycle bindings

Each row is certified across the four lifecycle bindings the spec requires every long-lived M5
object to expose (`primary_status_surface`, `exportable_status_code`, `last_failure_reason`,
`named_recovery_affordance`):

- **primary status surface** — `bound_to_one_primary_surface` (green), a disclosed
  `disclosed_surface_relocation` where the object's canonical surface is unavailable and its state
  is relocated to a disclosed, waivered still-visible fallback surface (yellow), or
  `status_surface_missing_or_split` (red: the object lost or split its single primary surface).
- **exportable status code** — `stable_code_exports_everywhere` (green), a disclosed
  `disclosed_partial_export` where the code exports in a reduced form on a subset of surfaces while
  still naming the same controlled state (yellow), or `status_code_unexportable` (red: the code
  stopped exporting on an export path).
- **last-failure reason** — `controlled_reason_reported` (green), a disclosed
  `disclosed_generic_reason` where the object falls back to a generic but still-controlled reason
  class until the specific class is available (yellow), or `last_failure_reason_missing_or_raw`
  (red: the object dropped its controlled reason or reported raw text).
- **named recovery affordance** — `named_recovery_present` (green), a disclosed
  `disclosed_reduced_recovery` where the object offers a reduced recovery while still naming a path
  forward (yellow), or `recovery_affordance_missing` (red: the object lost its named recovery
  affordance).

In addition, every row carries `headless_parity_preserved`: a hard invariant that the same
state-truth vocabulary survives a headless or companion-adjacent execution. A row that loses it
**blocks** (a `state_vocabulary_drift` cause), because a headless run must not report a different
state language than the in-product surface.

## Derived status and the certification lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when any of the
four bindings takes a disclosed narrowing. It drops to `red` when a primary status surface is lost
or split, a status code stops exporting, a last-failure reason goes missing or raw, a named
recovery affordance disappears, headless/companion-adjacent parity is lost, or the row fails to
certify every declared consumer surface. The consumer-surface completeness check is the lint that
prevents a certification from silently regressing into a partial, single-surface view — the exact
regression that would force support and diagnostics back onto surface-specific heuristics. The Rust
validator in `crates/aureline-shell/src/m5_lifecycle_object_certification` is the authoritative
gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_surface_relocation` narrowing must
additionally carry an active, matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact object causes, and the blocking findings the
  lane refuses to ship with.
- **Certification dashboard** — a light projection the product UI / CLI / diagnostics / support /
  telemetry automation reads to auto-narrow a governed object family's lifecycle claim when its
  certification falls out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix ref, build
  id, each object family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels — never raw
URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_object_certification`) is the
only mint-from-truth path for:

- `artifacts/release/m5-lifecycle-object-certification-proof/packet.json`
- `artifacts/release/m5-lifecycle-object-certification-proof/dashboard.json`
- `artifacts/release/m5-lifecycle-object-certification-proof/support_export.json`
- `artifacts/release/m5-lifecycle-object-certification-proof/matrix.csv`
- `artifacts/lifecycle/m5-lifecycle-object-certification.md` (this report's rendered companion)
- `fixtures/state/m5-lifecycle-object-certification/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/lifecycle/m5-lifecycle-object-certification.schema.json`](../../schemas/lifecycle/m5-lifecycle-object-certification.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_object_certification -- validate
cargo test -p aureline-shell --test m5_lifecycle_object_certification_fixtures
cargo test -p aureline-shell m5_lifecycle_object_certification
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_object_certification --"
$BIN packet         > artifacts/release/m5-lifecycle-object-certification-proof/packet.json
$BIN dashboard      > artifacts/release/m5-lifecycle-object-certification-proof/dashboard.json
$BIN support-export > artifacts/release/m5-lifecycle-object-certification-proof/support_export.json
$BIN csv            > artifacts/release/m5-lifecycle-object-certification-proof/matrix.csv
$BIN markdown       > artifacts/lifecycle/m5-lifecycle-object-certification.md
$BIN packet         > fixtures/state/m5-lifecycle-object-certification/packet.json
$BIN dashboard      > fixtures/state/m5-lifecycle-object-certification/dashboard.json
$BIN support-export > fixtures/state/m5-lifecycle-object-certification/support_export.json
$BIN compact        > fixtures/state/m5-lifecycle-object-certification/compact.txt
```
