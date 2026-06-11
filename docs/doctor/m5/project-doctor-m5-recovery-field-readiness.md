# Project Doctor M5 recovery field-readiness

This document describes the canonical packet that proves the M5 **field-readiness
and supportability** lane for Project Doctor and guided-repair flows. It is the
user-facing companion to the governed artifact at
`artifacts/doctor/m5/project-doctor-m5-recovery-field-readiness.json`, the
boundary schema at
`schemas/doctor/project-doctor-m5-recovery-field-readiness.schema.json`, and the
typed model in the `aureline-doctor` crate
(`m5_diagnosis_latency_recovery_ladders_and_support_parity`).

It builds on the feature-lane probe packet
(`artifacts/doctor/m5/project-doctor-feature-lane-probes.json`, *which finding*)
and the repair-transaction-receipts packet
(`artifacts/doctor/m5/project-doctor-repair-transaction-receipts.json`, *what a
repair does*). This packet answers a third question: **when a real user is
blocked in an M5 lane, can we diagnose them fast enough, take the right recovery
rung, and hand the case off with an export-safe packet — and does a stale or slow
corpus narrow the promotion automatically?**

## What each scenario pins

Every scenario seeds one blocked-user situation in one M5 recovery lane and
records, in a single rerunnable record:

- **Which lane and finding?** A `lane` (one of `notebook_kernel`, `request_api`,
  `database_target`, `profiler_replay`, `preview_route`, `sync_device_registry`,
  `companion_handoff`, `incident_packet`) and one or more `initiating_findings`,
  each lane-scoped under `doctor.finding.<lane>.` so support can reconstruct
  exactly what was diagnosed.
- **Which recovery rung?** A `recovery_rung` on the blocked-user recovery
  ladder — `safe_mode`, `quarantine`, `open_without_restore`,
  `cache_index_repair`, `restricted_reopen`, or `typed_repair`. A `typed_repair`
  rung (and only that rung) carries a `repair.`-prefixed `repair_id`.
- **How fast was the first actionable diagnosis?** A
  `first_actionable_latency_budgets` array with a `p50`/`p90`/`p95` budget
  (`target_ms` < `yellow_ms` < `red_ms`) and the corpus's `observed_latencies`.
  The **p90 first-actionable-diagnosis target** is the headline; an observed p90
  over the red threshold is a regression.
- **Is the corpus fresh?** A `corpus_run_at` date, a `corpus_age_days`, and a
  `freshness_window_days`; a corpus older than its window is stale.
- **Can the case be handed off?** A `drill_outcome` of
  `diagnosed_and_handed_off`, `diagnosed_not_handed_off`, or `not_diagnosed`, and
  a `support_linkage` describing the support-bundle manifest and escalation
  packet.

## Support-bundle and escalation parity

The `support_linkage` block is the supportability contract. It preserves stable
identity — `bundle_manifest_ref`, `escalation_packet_ref`,
`preserved_finding_ids` (`doctor.finding.`-prefixed), `preserved_repair_ids`
(`repair.`-prefixed), and `preserved_scope_refs` — so support and release review
can reconstruct the failure and the chosen ladder rung without screen sharing or
tribal knowledge. It carries `durable_evidence_refs` for reconstruction, and it
is metadata-safe by construction: `redaction_class` is always
`metadata_safe_default`, `raw_private_material_excluded` is always `true`, and
`overcapture_excluded` is always `true` — identity and opaque refs only, never
credential bodies or mount/port/tunnel secrets.

Stable identity (`bundle_manifest_ref`, `escalation_packet_ref`,
`preserved_finding_ids`, `preserved_scope_refs`) is preserved even when a
scenario is narrowed or blocked, so a case is always reconstructable. The piece
that may legitimately be missing is `durable_evidence_refs`; when it is absent
the gate narrows the scenario for `evidence_missing` rather than publishing it as
field-ready.

## The non-inheriting promotion gate

Each scenario publishes a `published_promotion_action`
(`publish_full`, `narrow_to_advisory`, or `block_promotion`) and a
`published_narrowing_reason`. Those values are **not asserted by hand** — they are
validated against the decision the gate recomputes from the scenario's own state,
in this precedence:

1. drill `not_diagnosed` → `block_promotion` / `drill_not_diagnosed`
2. stale corpus → `narrow_to_advisory` / `stale_corpus`
3. p90 latency breached → `narrow_to_advisory` / `latency_breached`
4. escalation incomplete (no durable evidence, or a typed repair with no
   preserved repair id) → `narrow_to_advisory` / `evidence_missing`
5. drill `diagnosed_not_handed_off` → `narrow_to_advisory` / `drill_not_handed_off`
6. otherwise → `publish_full` / `none`

Because the decision is recomputed per scenario, no scenario inherits "ready"
from an adjacent one: a stale or slow corpus, a missing evidence packet, or an
unhanded-off drill narrows or blocks *that* scenario automatically. A
`publish_full` scenario is independently guarded to require a fresh corpus, an
unbreached p90 budget, a complete escalation, and a `diagnosed_and_handed_off`
drill.

## Cross-surface parity

Every scenario renders on all six `parity_surfaces` — `desktop_pane`, `cli_row`,
`headless_json`, `support_export`, `incident_packet`, and `public_truth` — and
carries the locale-invariant `machine_meaning_keys` (`scenario_id`, `lane`,
`recovery_rung`, `drill_outcome`, `promotion_action`) so localized prose can
never change what a surface means.

## Reading the packet from code

```rust
use aureline_doctor::m5_diagnosis_latency_recovery_ladders_and_support_parity::{
    current_project_doctor_m5_recovery_field_readiness, PromotionAction,
};

let packet = current_project_doctor_m5_recovery_field_readiness()?;
assert!(packet.validate().is_empty());

for scenario in &packet.scenarios {
    // The published gate decision always equals the recomputed one.
    assert_eq!(
        (scenario.published_promotion_action, scenario.published_narrowing_reason),
        scenario.recompute_gate(),
    );
}
# Ok::<(), serde_json::Error>(())
```

The corpus is checked in and rerunnable: the typed model embeds the artifact via
`include_str!`, so the crate's tests and any CI gate validate the same rows
without re-deriving them.
