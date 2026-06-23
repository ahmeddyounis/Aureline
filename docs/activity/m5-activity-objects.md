# Activity-objects contract

This document describes the working **durable activity object model** every
long-running, retryable, or reviewable M5 job becomes, and the deterministic
projection that renders it as an activity-center row. Where the
[attention-routing matrix](./m5-attention-routing.md) *names and freezes the
object model* — including the durable activity object, its required fields, its
applicable states, its reopen targets, and its `durable_until_archived` retention
rule — this lane *implements that object family*: one typed activity object per
job family, rendered once into a row every surface consumes.

The track invariant this lane protects: **attention is routed, typed,
privacy-aware, and reopen-safe.** No long-running or reviewable work lives only in
a spinner or toast; completion and failure history is preserved durably until
archived or expired by policy; the archive state is one shared truth across
desktop, support export, companion, and operator; privacy never widens on a
surface; and badges derive from durable items.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## What every job family becomes

Each claimed M5 job family — notebook runs, task / CI jobs, AI / agent runs,
preview routes, pipeline actions, continuity sync, offboarding, operator handoffs,
and managed alerts — becomes one typed
[`ActivityObject`](../../crates/aureline-activity/src/m5_activity_objects/mod.rs)
instead of a transient spinner or completion toast. The object carries the fields
the spec makes the contract:

- a stable **job id** and the **actor subsystem** that owns the work,
- a coarse **phase** and a typed **progress state** (the matrix vocabulary:
  `running`, `queued_waiting`, `partially_completed`, `failed`, `completed`,
  `resolved`, `unknown_requires_review`),
- **cancel / retry / open-details** affordances, derived from the family and
  state — cancel only while in flight, retry only for failed or partial work of a
  retryable family, open-details always,
- **evidence links** to the run,
- **cost / trust / policy-impact** flags,
- **created / updated** stamps, and
- a frozen **retention policy** and the **archive / expiry state** derived from it.

Title and action copy is carried as **localizable keys** (`title_key`,
`reopen_anchor.label_key`), never raw bodies, so copy stays revisable while the
stable enums, ids, and reopen anchor are the actual contract.

## Archive and expiry

[`archive_state_for`](../../crates/aureline-activity/src/m5_activity_objects/mod.rs)
is a pure function of the progress state, the
[`ActivityRetentionPolicy`](../../crates/aureline-activity/src/m5_activity_objects/mod.rs),
and the object's `age_days` (the retention clock — days between `updated_at` and
the bundle `as_of`):

- Non-terminal work (running, queued, partial, awaiting review) is always
  **active**.
- A terminal state (completed, failed, resolved) ages from **active** to
  **archived** (past `archive_after_days`) to **expired** (past
  `expire_after_days`).
- Default families archive after 30 days and expire after 180; managed families
  (operator handoffs, managed alerts) are retained for compliance — 90 / 365.

Completion and failure history is therefore retained durably — a recent failure
stays active and reviewable, not dropped into transient chrome — and archive /
expiry behavior is testable rather than implicit.

## How a row is rendered

[`render_row`](../../crates/aureline-activity/src/m5_activity_objects/mod.rs) is a
pure function of an activity object. It produces one
[`ActivityRowProjection`](../../crates/aureline-activity/src/m5_activity_objects/mod.rs)
per consumer surface (shell activity center, CLI / headless, support export,
companion, operator dashboard), so a row is **reproducible byte-for-byte** in
support export and CLI / headless diagnostics. The projection rules:

1. The **shell activity center** and **CLI / headless** always render the durable
   row; the shell holds the full affordance set.
2. **Support export** always includes the row — active, archived, or as an expired
   tombstone — so completion and failure history is reviewable.
3. The **companion** mirrors a redacted summary, except managed-sensitive and
   expired rows, which stay in-product.
4. The **operator dashboard** renders org-scoped and managed rows as a read-only
   managed view; workspace-private rows are not shown.

Every surface other than the shell offers an **open-details (reopen)** affordance
— it reopens the authoritative object rather than reissuing a blind side effect.
The **archive state is identical on every surface**: archive / expiry is one
shared truth, not a per-client decision.

## The honesty rules, enforced

The canonical
[`activity_objects_bundle`](../../crates/aureline-activity/src/m5_activity_objects/mod.rs)
computes each invariant's `holds` flag from the built families, objects, and rows,
so an inconsistent edit flips an invariant and fails the freeze gate:

- `activity.every_family_has_durable_object` — every claimed family has a registry
  entry and a durable object; none is spinner-or-toast-only.
- `activity.durable_never_toast_only` — every object is a durable record that
  survives focus change and is never reduced to a spinner or toast.
- `activity.reopen_target_authoritative` — every object reopens an authoritative
  object the matrix admits.
- `activity.archive_expiry_deterministic` — every archive state recomputes from
  its retention policy and age.
- `activity.failure_completion_history_retained` — recent terminal work stays
  active, and the corpus exercises active, archived, and expired retention.
- `activity.archive_state_shared_across_surfaces` — every row reports the same
  archive state on every surface.
- `activity.privacy_never_widens_on_surface` — each surface applies a redaction at
  least as strong as the object default; managed-sensitive rows never reach the
  companion.
- `activity.badge_from_durable_items` — only durable, active, attention-pending
  rows count toward the badge.
- `activity.matrix_bound` — every retention class, progress state, and reopen
  target binds back to the attention-routing matrix's activity object.

## Companion artifacts

- [`/schemas/activity/m5-activity-objects.schema.json`](../../schemas/activity/m5-activity-objects.schema.json)
  — boundary schema for `m5_activity_objects_bundle`.
- [`/fixtures/activity/m5-activity-objects/canonical_bundle.json`](../../fixtures/activity/m5-activity-objects/canonical_bundle.json)
  — the published canonical bundle; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/activity/m5-activity-objects.md`](../../artifacts/activity/m5-activity-objects.md)
  — the human-readable companion (family, object, row, and invariant tables).
- `crates/aureline-activity/src/m5_activity_objects/` — the activity object record,
  the row renderer, the retention policy, the invariants, and the canonical
  builder.
- `crates/aureline-activity/tests/m5_activity_objects.rs` — the freeze gate.
- `cargo run -p aureline-activity --example dump_m5_activity_objects` — the
  headless emitter that regenerates the fixture (`-- --lines` for the
  human-readable projection).
