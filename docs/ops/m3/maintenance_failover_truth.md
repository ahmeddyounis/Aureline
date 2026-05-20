# Maintenance & failover continuity-notice truth (beta)

## Why this lane exists

Service-health truth answers "is a service degraded?". Real operational change
asks something far more specific:

- Is this a **maintenance**, **drain / read-only**, **failover**, or
  **tenant-migration** window?
- *Exactly* when does it start and end, in which timezone and offset, and how
  fresh is the status I am reading?
- Which deployment profiles, tenants, regions, and write classes are affected?
- Did my queued **publish-later** or **local-draft** work survive — and is it
  visibly separate from work that actually landed?
- Did a **tenant / region / residency / key-ownership / endpoint boundary**
  change, and is that change still visible after the system recovered?
- What stays **local-safe** right now, and is **postponing or exporting** safer
  than retrying?

Before this lane, a precise planned window or an emergency failover could be
flattened into a generic "something is offline" banner, queued work could vanish
into the same bucket as successful mutations, a recovered state could silently
hide a boundary that actually changed, and a stale or superseded notice could
keep reading as current operational truth.

This lane closes that gap with **one governed view** every surface reads
verbatim — it does **not** fork a per-surface banner.

## The governed view

`continuity_notice_view_record` is minted by
`crates/aureline-shell/src/continuity_notices` and frozen at the boundary by
`schemas/ops/continuity_notice_view.schema.json`. The desktop shell, the
activity center / durable history, CLI / headless inspect, diagnostics, and
support exports all read this single record, so they cannot drift on the window,
scope, write classes, boundary, or freshness for the same notice.

The view **composes** the upstream boundary records — it reuses their
vocabularies rather than minting a parallel one:

- `maintenance_notice_record` and `tenant_migration_event_record` —
  `docs/ops/maintenance_migration_failover_contract.md`
- `failover_banner_record` and `local_safe_baseline_record` —
  `docs/ops/failover_continuity_banner_contract.md`

### What every view carries

- **Kind, category, and plan.** A closed `notice_kind` (nine kinds) projects to
  a coarse `category` (`maintenance` / `drain` / `failover` /
  `tenant_migration`) and a `plan_class` (`planned` / `emergency`), so the user
  can always tell what kind of window this is.
- **Exact schedule.** `starts_at`, `expected_or_actual_ends_at`, `completed_at`,
  the IANA `timezone_id`, the `utc_offset_at_start`, and the `latest_refresh_at`,
  plus a derived `refresh_age` bucket.
- **Affected scope.** Deployment profiles, opaque tenant / region refs,
  residency scopes, and control-plane service classes.
- **Blocked write classes.** Per managed action: its `block_state_class`, its
  `continuity_posture` (queued publish-later, local-draft preserved, retryable,
  draining, blocked-pending-reconnect, blocked-pending-boundary-recheck,
  blocked-no-safe-retry, requires-manual-rerun), its `safer_guidance` (retry vs
  export-now vs postpone vs manual-rerun vs escalate), a canonical queue / intent
  ref for preserved work, and its `resume_trigger`.
- **Successful hosted mutations.** A *separate* list of mutations that already
  landed, so survived queued work is never collapsed into work that succeeded.
- **Boundary change.** Per axis (tenant, region, residency, key ownership,
  endpoint identity): its state, a canonical `current_ref` when changed, and a
  summary. The view derives `boundary_change_unresolved`.
- **Local-safe continuity.** The local-core status and the retained local-safe
  capability sentences.
- **Lifecycle + derived freshness.** The declared lifecycle freshness plus the
  honest `effective_freshness`, the `downgrade_reasons`, the
  `honesty_marker_present` bit, and durable `history_ref` / `support_export_ref`
  routing.

## Invariants

### No silent current

`effective_freshness == current` **only** when the declared lifecycle freshness
is `active_current` **and** the last refresh is `fresh` or `recent` relative to
`as_of`. A superseded, completed, imported, or refresh-aged-out notice
downgrades, names a `downgrade_reason`, lights the honesty marker, and carries a
non-null stale label. A stale or cached continuity notice can never masquerade as
current operational truth.

### Queued work survives and stays separate

Every queued publish-later and local-draft write is marked `intent_preserved`
and carries a canonical queue / intent ref; successful hosted mutations live in a
separate list. `queued_and_succeeded_collapsed` stays false, so survived queued
work is always visibly distinct from work that landed.

### Boundary preserved after recovery

A changed or unknown-recheck boundary axis carries a canonical `current_ref` and
stays visible even on a completed / recovered notice. `boundary_change_hidden`
stays false, and `boundary_change_unresolved` lights the honesty marker until the
boundary is reviewed.

### No generic-degraded collapse

Every notice names its category, and the display-copy invariants stay false:
`all_work_broken_implied`, `incident_language_for_planned_used`,
`generic_degraded_banner_used`, `queued_and_succeeded_collapsed`,
`stale_presented_as_current`, and `boundary_change_hidden`.

## Drill corpus and gate

The deterministic corpus lives in
`crates/aureline-shell/src/continuity_notices/corpus.rs` and is pinned on disk
under `fixtures/ops/m3/maintenance_and_failover_notices/` (see that directory's
README for the per-drill table). It exercises every notice kind, category,
effective freshness, write-continuity posture, downgrade reason, and
boundary-axis state.

- **Fixture-replay** (`crates/aureline-shell/tests/continuity_notices_fixtures.rs`):
  asserts the on-disk fixtures are a bit-for-bit projection of the in-code
  corpus and re-checks the invariants above.
- **Audit validator** (`ci/check_continuity_notices.py`): schema-validates every
  fixture and independently re-derives the refresh age, the no-silent-current
  downgrade, the boundary-unresolved flag, the honesty marker, and the summary
  counts — a second implementation of the model — then checks corpus coverage.
- **Lane entry point** (`scripts/ci/run_continuity_notices.sh`, wired by
  `.github/workflows/check_continuity_notices.yml`): runs the validator always,
  and re-emits + replays the fixtures when a Cargo toolchain is present.

Regenerate the fixtures after any change to the corpus or model:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_continuity_notices_corpus -- emit-fixtures \
  fixtures/ops/m3/maintenance_and_failover_notices
```

## Scope

This lane is truthful user / operator communication on claimed beta managed and
hybrid rows. It does **not** add incident-management or control-plane
administration workflows, and it does **not** add billing, usage, or
organizational reconfiguration — only the notices and continuity states needed
for current beta claims.
