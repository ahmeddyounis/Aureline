# M5 efficiency certification

This is the normative policy companion for the M5 efficiency certification lane.
The lane certifies power, thermal, battery-efficiency, and hidden-work-shedding
truth on every **claimed laptop-or-desktop profile** and every **long-running M5
surface family**, and is the **single source of truth** for whether a low-power
claim may be promoted. Release, support, docs, and help consume the proof packet
instead of cloning a low-power claim.

- Machine-readable proof packet: [`artifacts/efficiency/m5-efficiency-proof-packet.json`](../../artifacts/efficiency/m5-efficiency-proof-packet.json)
- Human-readable rendering: [`artifacts/efficiency/m5-efficiency-certification.md`](../../artifacts/efficiency/m5-efficiency-certification.md)
- Boundary schema: [`schemas/efficiency/m5-efficiency-certification.schema.json`](../../schemas/efficiency/m5-efficiency-certification.schema.json)
- Enforcing gate: [`ci/check_efficiency_certification.py`](../../ci/check_efficiency_certification.py)
- Regenerator: [`tools/regenerate_efficiency_certification.py`](../../tools/regenerate_efficiency_certification.py)
- Shell binding: [`crates/aureline-shell/src/efficiency/certification/`](../../crates/aureline-shell/src/efficiency/certification/)
- Aligned governance matrix: [`docs/efficiency/m5-efficiency-governance.md`](./m5-efficiency-governance.md)

## Why this lane exists

The efficiency-state governance matrix already freezes a typed low-power contract
per M5 surface, and the energy/thermal lab and session-pressure modules already
capture the evidence. What was still implicit was the **certification** step: a
single, inspectable claim that, on each *claimed hardware profile* and surface
family, that evidence is **current** and actually backs the posture being
advertised. Without it, a claimed low-power row could stay green on one good
manual battery test while its current efficiency evidence was stale or
incomplete. This lane closes that gap so:

- efficiency governance is a **named certification lane** with current proof
  packets and **automatic claim narrowing**;
- stable/beta claims for laptop/desktop profiles and long-running surface families
  **cannot outrun their efficiency evidence**; and
- docs, help, support, and release derive from the **same certification packet**.

## What a row certifies

Each row covers one claimed subject along one of two axes the spec keeps separate:

- a **laptop-or-desktop profile** — a battery ultrabook, a thermal workstation, a
  policy-managed fleet machine, or a critical-battery field laptop; or
- a **long-running M5 surface family** — notebooks, previews, docs/browser panes,
  traces, pipelines, remote sessions, support exports, or companion-adjacent
  views.

Every row runs the fixed **drill set** — efficiency-state behaviour, hidden-work
suppression, protected-path preservation, session-aware shedding, and staged
recovery — against the canonical evidence (energy/thermal traces, hidden-pane
audits, session-pressure postures). A row records one result per drill, the
freshness of the evidence behind it, the narrowing reasons that fired, the
narrowed effective posture, and whether it holds promotion.

## Evidence freshness

Bound evidence is **current** within 30 days of the packet `as_of`. Older
evidence is **stale**; an absent required-drill binding is **partial** when the
subject still has other evidence and **missing** otherwise. The freshness rule is
the heart of the guardrail: stale, partial, and missing evidence each
auto-narrow the claim, so a posture can never advertise more than its *current*
evidence supports.

## Claim narrowing and the promotion gate

A row publishes at most its **published claim ceiling**. Each fired narrowing
reason has a posture floor; the **effective posture** is the lowest-ranked of the
ceiling and every fired floor. The certification state follows: `certified` when
nothing fired, `quarantined` when the row narrowed all the way to the
undeclared-badge floor, and `narrowed` otherwise.

Promotion **holds** when any row whose published ceiling is claim-bearing narrows
below that ceiling — a claim-bearing posture that cannot prove current behaviour
fails the gate. A row that asserts no claim (an undeclared badge) is retained only
for diagnosis and does not hold promotion. The gate is recomputed by
[`ci/check_efficiency_certification.py`](../../ci/check_efficiency_certification.py)
from the drill results, so the stored verdict can never be hand-edited, and
negative drills prove the recompute fails closed when evidence goes stale, a drill
regresses, or a stored verdict is inflated.

## Alignment with the governance matrix

Every surface-family row cites the governance matrix row it certifies and may
**narrow** but never **inflate** that surface's governed posture. This keeps the
certification lane aligned with desktop continuity, scheduler fairness, and the M5
surface-family claim publication objects: a surface can never certify a stronger
low-power claim than the governance matrix permits for it.

## Export safety

The proof packet is export-safe by construction: it carries efficiency-state
tokens, posture levels, drill outcomes, freshness grades, labels, and evidence
references — never raw energy/power/thermal/battery telemetry, raw logs, provider
payloads, secrets, file paths, or user content. The gate scans for and rejects any
such field crossing the boundary.

## Regenerating

The proof packet is produced from the shell certification lane and dumped by the
conformance example, so it never drifts from the code:

```bash
python3 tools/regenerate_efficiency_certification.py
python3 ci/check_efficiency_certification.py --repo-root .
cargo test -p aureline-shell efficiency_certification
```
