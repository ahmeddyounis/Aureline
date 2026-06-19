# Continuity-proof freshness SLOs and shiproom gates

This document is the reviewer-facing companion for the gated continuity-proof
freshness dashboard:

- artifact: [`/artifacts/m5/continuity/freshness_slo_dashboard.json`](../../artifacts/m5/continuity/freshness_slo_dashboard.json)
- schema: [`/schemas/continuity/continuity_freshness_slo_dashboard.schema.json`](../../schemas/continuity/continuity_freshness_slo_dashboard.schema.json)
- typed model: [`crates/aureline-continuity/src/m5_continuity_freshness_slo`](../../crates/aureline-continuity/src/m5_continuity_freshness_slo/mod.rs)
- rerun tool: [`/tools/continuity/run_drill_packets.py`](../../tools/continuity/run_drill_packets.py)
- CI gate: [`/tools/check_m5_continuity_freshness.py`](../../tools/check_m5_continuity_freshness.py)
- stale-evidence fixtures: [`/fixtures/continuity/stale_evidence_cases`](../../fixtures/continuity/stale_evidence_cases)

It is the canonical M5 source for continuity-proof freshness truth. The
shiproom gate, release center, docs/public-truth publication, and support
exports read this one freshness signal instead of re-deriving staleness for
every claimed continuity row.

## Why this lane exists

The continuity-claim matrix already states *what* each claimed managed,
self-hosted, or sovereign row discloses: locality, tenant scope, key mode,
continuity packet family, restore identity, partial loss, and drill
cadence/owner. What it cannot state by itself is *how fresh* the evidence behind
that claim still is. A backup, restore, failover, or snapshot packet that was
current at qualification ages. Without an explicit freshness SLO, a stale
locality / tenant / key / failover row keeps inheriting green enterprise language
long after its proof expired — an ops footnote masquerading as a release truth.

This lane closes that gap. It moves continuity truth from one-time packet
generation to ongoing evidence freshness with visible ownership and expiry, and
makes stale continuity evidence a release and public-truth concern.

## The freshness SLO

Every tracked continuity row points to one continuity proof packet with a
freshness SLO:

- `target_max_age_days` — the packet may be at most this many days old.
- `warn_within_days` — once this few days of life remain, the packet is
  `due_for_refresh`.

Against the dashboard `as_of` date, each packet earns one freshness-SLO state,
using the same closed vocabulary the release claim manifest and shiproom
dashboard publish:

| State | Meaning | Within SLO? |
| --- | --- | --- |
| `current` | captured or drilled well within the SLO | yes |
| `due_for_refresh` | within the SLO but inside the warn window | yes |
| `breached` | older than the target; stale | no — narrows |
| `missing` | no proof packet captured | no — narrows |

`breached` and `missing` force the claim to narrow below the stable cutline. The
date arithmetic that turns `captured_at` + `freshness_slo` into a state lives in
the rerun tool and the CI gate — the typed Rust model carries the declared state
and enforces every invariant that holds regardless of the clock.

## What narrows, and what never does

A release-scope row (any claimed managed, self-hosted, or sovereign row) narrows
when its evidence is stale, missing, unattested, or unrefreshable:

| Condition | Stop reason | Row narrows to |
| --- | --- | --- |
| packet breached its SLO | `continuity_packet_freshness_breached` | beta |
| no packet captured | `continuity_packet_missing` | preview |
| no current drill-owner sign-off | `drill_owner_signoff_missing` | beta |
| no rerun path to refresh evidence | `rerun_path_unavailable` | beta |
| backing evidence unqualified / mismatched | `continuity_evidence_unqualified` | preview |

The **guardrail** is explicit and tested: the local-core continuity lane keeps
working without any managed lane, so a local-core row never narrows or holds
promotion because a managed continuity row went stale. When a managed row
breaches, the dashboard narrows that managed claim and holds promotion — the
local-core claim stays green. (`fixtures/continuity/stale_evidence_cases/case_local_core_stays_green.json`.)

## Shiproom stop rules and the promotion verdict

The dashboard enumerates one stop rule per stop reason. A rule fires when a row
at or above the stable/beta cutline carries its trigger reason. The promotion
verdict is `hold` when any release-scope row holds promotion and `proceed`
otherwise. The verdict, the firing rule ids, and the blocked row ids are recorded
on the dashboard so the shiproom session reads a decision, not a spreadsheet.

## The rerun rehearsal path

Freshness is only honest if evidence can be regenerated without manual artifact
surgery. Every release-scope row declares a rerun path
(`automated_rerun`, `scripted_refresh`, `manual_runbook_only`, or `no_rerun_path`)
and the tool that refreshes it. `tools/continuity/run_drill_packets.py` is that
path:

```sh
# Regenerate the dashboard and fixtures from the Rust source of truth
python3 tools/continuity/run_drill_packets.py --regenerate

# Recompute every packet's freshness against an explicit clock and report drift
python3 tools/continuity/run_drill_packets.py --check --as-of 2026-09-01

# Record a fresh drill for one row and re-derive the whole dashboard
python3 tools/continuity/run_drill_packets.py \
    --rerun continuity-row:managed-cloud-sync --captured-at 2026-09-01 --write
```

Recording a fresh drill updates one packet's `captured_at` and re-derives the
narrowing, the promotion verdict, and the summary in place — no hand-editing of
the JSON. A row whose packet had breached returns to `current` and its claim
de-narrows automatically.

## The CI gate

`tools/check_m5_continuity_freshness.py` is the proof-expiry automation for this
lane. Over the canonical artifact and every fixture it:

1. validates each record against the schema;
2. recomputes every packet's freshness state from `captured_at` against the
   dashboard `as_of` and **fails when a declared state is fresher than the clock
   allows** — a continuity claim may not ride evidence fresher than its capture;
3. reuses the rerun tool's freshness engine and **fails on any drift** between the
   checked-in dashboard and the recompute, so the typed Rust model and the Python
   automation cannot diverge and the artifact cannot quietly outlive its proof;
4. enforces the structural shiproom invariants (local-core never blocks, every
   release-scope row names a rerun path, every stop reason is watched by a rule);
   and
5. asserts each stale-evidence fixture narrows and holds promotion as declared.

It writes a validation capture to
`artifacts/governance/captures/m5-continuity-freshness-slo_validation_capture.json`.
The crate audit (`cargo test -p aureline-continuity m5_continuity_freshness`) and
the schema validator
(`python3 tools/validate_m5_continuity_freshness_slo_fixtures.py`) run alongside
it in `.github/workflows/check_m5_continuity_freshness.yml`.

## Update rules

- Change a row's posture, packet family, or SLO target in the typed model, then
  run `python3 tools/continuity/run_drill_packets.py --regenerate` to refresh the
  artifact and fixtures together.
- When the supporting topology, profile, or deployment row changes, do not leave
  the continuity evidence green: refresh `captured_at` (or let the clock age the
  packet) so the affected managed claim narrows rather than overclaiming.
- Never block a local-core claim because a managed continuity row went stale;
  narrow the affected managed claim instead.
