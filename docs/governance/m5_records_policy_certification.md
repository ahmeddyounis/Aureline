# M5 Record-Governance Certification

The M5 record-governance certification packet is the single proof that a claimed
M5 managed or enterprise record-governance row is actually truthful and
supportable. It does not introduce new record behaviour; it certifies that the
behaviour already shipped by the lane's truth packets is present, current, and
honest for every governed family before a claim hardens.

## What it certifies

For each governed M5 family it binds one proof cell per lane dimension:

| Dimension | Backing truth | Contract |
| --- | --- | --- |
| Record class | `records_policy_simulation_matrix` + record-class registry | `governance:records_delete_chronology_policy:v1` |
| Hold / delete | `m5_records_policy` hold/retention packet | `records:m5_hold_retention_truth:v1` |
| Chronology | `m5_evidence_chronology_packet` | `chronology:m5_evidence_time_lineage:v1` |
| Policy simulation | `m5_policy_impact_simulation_packet` | `records:m5_policy_impact_simulation:v1` |
| Exception / expiry | `m5_exception_expiry_packet` | `policy:m5_exception_expiry_truth:v1` |

A family is **certified** only when all five cells are observed and current.

## How a row narrows

A family **narrows** automatically when any cell is missing or stale, or when a
local-only family claims a managed hold, export, or delete it cannot prove. A
narrowed row carries explicit narrow reasons (`proof_missing`, `proof_stale`,
`upstream_validation_failed`, `managed_claim_unproven`) and its shiproom and
public-claim labels must say so — the packet rejects a cosmetically clean
"certified" label over a narrowed claim. When a narrowed family is
release-blocking, the shiproom promotion gate moves to **hold**.

## Shiproom and public claims read the same result

Shiproom gate review and public status surfaces ingest the packet's projections
verbatim:

- `shiproom_projection()` — entry, verdict, label, narrow reasons; a `hold`
  promotion decision blocks the M5 record-governance gate.
- `public_claim_projection()` — the public label per family, never widened past
  the verdict.
- `cli_headless_projection()` — per-dimension freshness and narrow reasons.
- `support_export_projection()` — metadata-safe proof refs and verdicts.

No surface clones its own status text; they all render this certification.

## Freshness and regression

`verify_against_live_packets()` re-checks the certification against current
same-crate truth (the records/policy matrix and the seeded hold/retention and
policy-simulation packets) and confirms each proof source matches. If an upstream
packet regresses or a proof source is tampered, the cross-check surfaces a
finding so the affected claim re-certifies rather than holding stale proof.

## Guardrails

- No row implies remote delete, remote export, or remote legal hold for a family
  the platform only knows locally — local-only managed claims narrow.
- Remembered decisions and exceptions never widen authority across actor,
  object, target, policy epoch, or environment drift; the exception/expiry proof
  cell tracks the upstream `m5_exception_expiry` revalidation contract.
- Delete/export honesty outranks cosmetically simple "done" copy.

## Validating locally

```bash
cargo test -p aureline-records m5_records_policy_certification
python3 tools/check_m5_records_policy_certification.py
```

Regenerate the canonical fixture after changing the seeded packet:

```bash
cargo run -p aureline-records --example dump_m5_records_policy_certification \
  > fixtures/governance/m5_records_policy_certification/canonical_packet.yaml
```
