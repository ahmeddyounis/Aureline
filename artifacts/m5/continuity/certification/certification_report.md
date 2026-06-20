# Continuity certification report

Human-readable companion to the canonical certified-row registry checked in at
`artifacts/m5/continuity/certification/certified_rows.json`. Both are produced by
the typed model in `crates/aureline-continuity/src/m5_continuity_certification/`
(`cargo run -p aureline-continuity --example
dump_m5_continuity_certification_fixtures -- page`).

## Purpose

This report turns continuity truth into a certifiable lane rather than a
best-effort deployment appendix. It folds the upstream continuity lanes —
locality/tenant/key disclosure, typed control-plane/data-plane degradation,
backup/restore/failover drills, restore-identity/partial-loss semantics,
mirror/offline continuity, and continuity-proof freshness — into **one
certification verdict per claimed managed, self-hosted, or sovereign row**.
Release packets, Help/About truth, service-health summaries, support exports, and
partner qualification packets read this one verdict instead of re-deriving "is
this continuity claim certified?" by hand.

## Certification rule

A certification-scope row (any managed, self-hosted, or sovereign surface, or a
row carrying a claimed managed dependency) stays **certified** only when every
required continuity dimension is `current`:

| Dimension | What it certifies |
| --- | --- |
| `locality_tenant_key` | Processing/storage locality, tenant boundary, and key-mode disclosure |
| `control_data_plane_degradation` | Typed control-plane vs data-plane degraded fallback |
| `backup_restore_failover` | Current backup/restore/failover drill evidence |
| `restore_identity_partial_loss` | Restore identity reproduced and partial-loss disclosure |
| `mirror_offline_continuity` | Mirror-only / air-gapped offline continuity (air-gapped rows only) |
| `drill_freshness_slo` | The backing continuity proof packet is within its freshness SLO |

When any required dimension is **stale or partial** the claim narrows to `beta`;
when it is **missing** the claim narrows to `preview`; when the evidence
**contradicts the claimed profile** the claim is **withdrawn**. Narrowing is
automatic: a row may never keep enterprise/managed language broader than the
evidence supports.

## Guardrails

- **The local-core lane is never narrowed.** A pure local-only row with no
  claimed managed dependency rides the local-core continuity lane and stays
  certified even when a managed row goes stale; it is never conflated with the
  managed lane.
- **No shared reference drill.** A single reference-environment
  backup/restore/failover drill may not stand in for more than one claimed
  profile row; when two certification-scope rows reuse the same drill evidence
  ref, both narrow.
- **Verdict reuse.** Each row's verdict must reach every required surface (About,
  Help, service-health, support exports, docs/public-truth, and partner
  qualification for scope rows; the in-product and public-truth surfaces for the
  local-core lane).

## Current verdict

Computed as of `2026-06-19` — overall decision **certified**; 5 of 5 rows hold
their claim, 0 narrowed, 0 withdrawn.

| Row | Profile | Lane | Claimed | Verdict | Effective |
| --- | --- | --- | --- | --- | --- |
| Managed cloud workspace sync and backup | managed | managed_lane | stable | certified | stable |
| Managed relay and collaboration failover | managed | managed_lane | stable | certified | stable |
| Customer self-hosted restore and rebuild | self_hosted | managed_lane | stable | certified | stable |
| Sovereign air-gapped snapshot and replication | sovereign | managed_lane | stable | certified | stable |
| Local desktop core continuity | local_only | local_core | stable | certified | stable |

The sovereign air-gapped row additionally certifies the `mirror_offline_continuity`
dimension; the local desktop row is out of managed certification scope and is
held only to local-core continuity.

## Narrowing in action

The stale-, missing-, mismatched-, and shared-drill cases under
`fixtures/continuity/certification_cases/` prove that each dimension narrows the
claim automatically and that the local-core lane stays certified throughout. See
the companion `drill_freshness_report.md` for the per-row drill and freshness
posture.

## Consumption

Downstream release, docs/Help/About, service-health, CLI inspection
(`aureline_continuity_certification_inspect`), and support-export surfaces ingest
the support-export projection
(`artifacts/m5/continuity/certification/certification_support_export.json`) —
including the per-row certification verdict, effective label, and narrow-reason
tokens — rather than cloning status text.
