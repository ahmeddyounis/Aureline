# M5 debug qualification evidence

This set is the checked-in proof path for Aureline's typed M5 debugger qualification: the
canonical records every claim board, About / help / service-health surface, support packet,
and release packet reads to show whether a claimed debugger-facing row is currently certified
against the shared debug object model, what maturity the product is allowed to publish for it,
and why a claim narrowed. It binds each row to the debugger object families it claims and the
proof packets that keep it current, computes one qualification status from evidence freshness
and completeness, and derives the published maturity — narrowing automatically when debugger
evidence is stale, partial, or failing.

The published set is
[`fixtures/debug/m5_debug_qualification/canonical_set.json`](../../fixtures/debug/m5_debug_qualification/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_debug_qualification/mod.rs` by the gate at
`crates/aureline-debug/tests/m5_debug_qualification.rs`. The reviewer-facing contract is at
[`docs/debug/m5_debug_qualification.md`](../../docs/debug/m5_debug_qualification.md).

## Materialized qualification rows

| Row | Category | Status | Claimed | Published | Narrowed |
|---|---|---|---|---|---|
| `debug.qual:core_session_attach:0001` | core_runtime | certified | stable | stable | no |
| `debug.qual:core_breakpoints_frames:0002` | core_runtime | certified | stable | stable | no |
| `debug.qual:core_variables_evaluate:0003` | core_runtime | retest_pending | stable | retest_pending | yes |
| `debug.qual:notebook_debug_bridge:0004` | notebook | certified | preview | preview | no |
| `debug.qual:notebook_unsupported_kernel:0005` | notebook | stale | preview | retest_pending | yes |
| `debug.qual:profiler_replay_session:0006` | profiler_replay | certified | preview | preview | no |
| `debug.qual:profiler_replay_imported:0007` | profiler_replay | partial | stable | preview | yes |
| `debug.qual:incident_crash_symbolication:0008` | incident_support | failing | preview | withdrawn | yes |
| `debug.qual:incident_support_export:0009` | incident_support | policy_blocked | stable | withdrawn | yes |
| `debug.qual:profiler_replay_inspect_only:0010` | profiler_replay | certified | inspect_only | inspect_only | no |

The set materializes all four surface categories (core runtime, notebook, profiler/replay,
incident/support), all six qualification statuses (certified, retest-pending, stale, partial,
failing, policy-blocked), and all five published maturities (stable, preview, inspect-only,
retest-pending, withdrawn). Collectively the rows claim every one of the ten governed debugger
object families.

## Materialized claim publications

| Channel | Rows | Claimed | Published | Narrowed |
|---|---|---|---|---|
| `claim_board` | 3 | stable | retest_pending | yes |
| `about_help_service_health` | 4 | preview | retest_pending | yes |
| `support_export` | 3 | preview | withdrawn | yes |
| `release_packet` | 2 | stable | stable | no |

Every channel republishes the narrowest maturity across the rows it covers and shows the
qualification status and evidence refs backing the claim. The claim board narrows to
retest-pending because the core variable/evaluate evidence is aging; the support export
narrows to withdrawn because the incident symbolication has no current evidence and the
support export is policy-blocked; the release packet covers only ship-stable rows and stays
stable.

## Materialized downgrade rules

| Trigger | Floors to | Rows |
|---|---|---|
| `evidence_aging` | retest_pending | 1 |
| `evidence_stale` | retest_pending | 1 |
| `evidence_partial` | preview | 2 |
| `evidence_missing` | withdrawn | 1 |
| `support_class_degraded` | preview | 7 |
| `mapping_fidelity_degraded` | preview | 4 |
| `notebook_parity_lost` | preview | 2 |
| `replay_evidence_lost` | preview | 4 |
| `policy_blocked` | withdrawn | 1 |

Every active rule lists every row exhibiting its trigger and narrows each at least to its
resulting maturity, so the dashboard explanation of *why* a claim narrowed cannot drift from
the computed maturities.

## Frozen invariants

All ten invariants hold for the canonical set: every object family is claimed; every category
and status is materialized; every row's status/maturity/narrowing agrees with its evidence;
stable is earned, never asserted; every narrowed row carries a reason and every degraded row
publishes below stable; every channel is published once and republishes its floor; and every
active downgrade rule covers the rows it triggers.

## Safety

The record carries no source bodies, value bodies, raw paths, provider payloads, URLs,
hostnames, or credentials — only opaque object refs, stable tokens, and short reviewable
sentences — so it is safe for support export. The cross-tool boundary schema is at
[`schemas/debug/m5_debug_qualification.schema.json`](../../schemas/debug/m5_debug_qualification.schema.json).
