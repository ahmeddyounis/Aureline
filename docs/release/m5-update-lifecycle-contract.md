# M5 update / support-lifecycle governance contract

This contract freezes the one governed baseline every claimed M5 update-center,
release-note evidence, change-impact, migration, service-health, support-window,
compatibility-window, and end-of-support surface qualifies against. It exists so that
update impact, migration work, support windows, and end-of-support state stop living as
fragmented release-center prose, docs, and support notes, and instead resolve to one
inspectable, machine-readable matrix.

It does **not** invent new channel families, add managed products, or rewrite the existing
release-publication objects. It is the missing governance layer those objects bind to.

- Packet schema: [`schemas/release/m5-update-center-summary.schema.json`](../../schemas/release/m5-update-center-summary.schema.json)
- Published inventory: [`artifacts/release/m5-update-lifecycle-summary.json`](../../artifacts/release/m5-update-lifecycle-summary.json)
- Rendered governance matrix: [`artifacts/release/m5-update-lifecycle-governance.md`](../../artifacts/release/m5-update-lifecycle-governance.md)
- Machine-readable matrix: [`artifacts/release/m5-update-lifecycle-matrix.csv`](../../artifacts/release/m5-update-lifecycle-matrix.csv)
- Release-grade parity proof: `artifacts/release-proof/m5-update-lifecycle/update-lifecycle-matrix.json` (+ `.md`)
- Per-state fixtures: `fixtures/release/m5-update-center/`
- Producer crate / module: `crates/aureline-release` → `m5_update_lifecycle`
- Headless emitter: `aureline_release_m5_update_lifecycle`

## What the matrix governs

The matrix has three parts, all minted from one source by the headless emitter so the
in-code packet, the published artifacts, and the fixtures can never drift.

### 1. Canonical lifecycle state families

Five ordered, gate-bound vocabularies. Every token binds to a gate posture
(`governed` / `narrowed` / `blocked`) drawn from the shared descriptor / badge runtime, and
to the effective qualification floor that posture implies (`stable` / `beta` / `unavailable`).
Surfaces reuse these tokens instead of restating lifecycle state as ad hoc labels.

| Family | Governed | Narrowed | Blocked |
|--------|----------|----------|---------|
| `update` | `up_to_date`, `update_offered` | `update_recommended`, `update_required` | `update_blocked` |
| `readiness` | `ready_no_restart`, `restart_required`, `rollback_available` | `action_required` | `not_ready` |
| `migration` | `no_migration`, `automatic_migration` | `assisted_migration`, `manual_migration` | `blocking_migration` |
| `support_window` | `full_support` | `maintenance_support`, `security_support`, `grace_window` | `out_of_support` |
| `end_of_support` | `supported` | `sunset_announced`, `deprecated` | `retired`, `removed` |

### 2. Governed facets

The eight product surfaces the source set treats as governed update / support-lifecycle
truth. Each facet owns exactly one proof path and an accountable owner role, names the
state family that governs it, and discloses the artifact classes it touches, the claimed
channels and deployment profiles it scopes to, and how it behaves under stale / mirrored /
no-live-data conditions.

| Facet | Dimension | State family | Owner role |
|-------|-----------|--------------|------------|
| `update_availability` | `change_disclosure` | `update` | `release_update_center_owner` |
| `change_impact` | `change_disclosure` | `readiness` | `release_update_center_owner` |
| `release_note_evidence` | `change_disclosure` | `readiness` | `release_notes_owner` |
| `migration_assistant` | `migration_continuity` | `migration` | `migration_continuity_owner` |
| `service_health` | `migration_continuity` | `readiness` | `migration_continuity_owner` |
| `support_window` | `support_lifecycle` | `support_window` | `support_lifecycle_owner` |
| `compatibility_window` | `support_lifecycle` | `support_window` | `support_lifecycle_owner` |
| `end_of_support` | `support_lifecycle` | `end_of_support` | `support_lifecycle_owner` |

Channels are a subset of the frozen release-channel vocabulary (`stable`, `beta`,
`preview`, `nightly`, `lts`); profiles are `managed` and `self_hosted`. Stale-data behavior
is one of `live_verified`, `mirrored_labelled`, `offline_cached`, `stale_banner_shown`, or
`local_only_no_live_data` — every behavior keeps the surface local-safe by labelling the
weaker state rather than dropping it.

### 3. Claimed consumers

The eight claimed M5 surfaces that must ingest the matrix rather than keep parallel
inventories: `release_center`, `update_center`, `help_about`, `docs_help`, `diagnostics`,
`support_export`, `shiproom`, and `companion_handoff`. Each binds the facets it reads; the
matrix derives, per consumer, the union of disclosed artifact classes / channels / profiles,
the proof paths backing it, the exact coverage gaps, a gate decision, and an effective
qualification.

## Gate semantics — proof *or* lifecycle coverage

A consumer's claim is derived, never hand-maintained. For each facet a consumer reads:

- a **stale** proof, or a facet whose current lifecycle state is itself **narrowing**,
  records a narrowing gap and narrows the consumer to at most `beta`;
- an **expired** / **missing** proof, a facet the matrix does not govern, or a facet whose
  current lifecycle state is **blocking**, records a blocking gap and blocks the consumer
  from Stable promotion (effective qualification `unavailable`);
- a **current** proof and a **governed** state record no gap.

Every gap names its facet, drifted dimension, gap kind, and a routable message id, so a
drift report says *which* of change disclosure, migration continuity, or support lifecycle
aged out rather than collapsing the cause into one flag. Gaps in proof **or** lifecycle
coverage therefore fail the matrix rather than remaining implied — the acceptance bar for
this lane.

The packet-level release gate aggregates the per-consumer gates: it lists the blocked,
narrowed, and certified consumers and the drifted dimensions, and holds Stable promotion
while any consumer is blocked.

## Drills

Two checked-in drills exercise the auto-narrowing deterministically:

- `lifecycle_stale_proof_narrowed` marks the `change_impact` facet's proof stale; exactly
  the five consumers that read it narrow to `beta`, the other three stay certified, and the
  release gate stays a pass.
- `lifecycle_missing_proof_blocked` marks the `service_health` facet's proof missing;
  exactly the six consumers that read it block, the other two stay certified, and the
  release gate blocks Stable promotion.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- registry   > artifacts/release/m5-update-lifecycle-summary.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- governance > artifacts/release/m5-update-lifecycle-governance.md
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- csv        > artifacts/release/m5-update-lifecycle-matrix.csv
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- variant canonical > artifacts/release-proof/m5-update-lifecycle/update-lifecycle-matrix.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- markdown   > artifacts/release-proof/m5-update-lifecycle/update-lifecycle-matrix.md
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- variant canonical > fixtures/release/m5-update-center/lifecycle_all_current.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- variant stale     > fixtures/release/m5-update-center/lifecycle_stale_proof_narrowed.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle -- variant missing   > fixtures/release/m5-update-center/lifecycle_missing_proof_blocked.json
```

The packet carries metadata and refs only: no credential bodies or raw provider payloads.
The export is scanned for forbidden material on every validation.
