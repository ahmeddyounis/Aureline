# Shiproom — M5 public-contract certification

Single-screen certification status for the M5 public-contract publication lane, rendered from `artifacts/certification/m5-public-contract-certification.json` by `tools/regenerate_m5_public_contract_certification.py`. The certification packet is the closeout proof that every claimed M5 public artifact family has the right published contract form, lifecycle metadata, example corpus, validator coverage, compatibility report, and release-graph linkage — or has narrowed below the cutline.

**Certification decision: HOLD**

Certification is held: one or more release-blocking M5 public artifact families have a missing required contract pillar (published contract form, lifecycle metadata, example corpus, validator coverage, compatibility report, or release-graph linkage). Publishing the missing contract evidence and rerunning the gate clears the hold.

## At a glance

- Families: **16** (8 release-blocking)
- Certified: **15**
- Narrowed: **0**
- Withheld: **1**
- Held (promotion): **1**
- Pillars: **95** current / **0** stale / **1** missing

## Blockers

| Family | Claim | Certified | Missing pillars | Reasons | Stop actions |
| --- | --- | --- | --- | --- | --- |
| `task_event_envelope` | stable | beta | `compatibility_report` | `row_downgraded`, `compatibility_report_missing`, `evidence_missing`, `retest_pending`, `mirror_parity_incomplete` | `hold_certification`, `hold_promotion`, `narrow_claim`, `publish_compatibility_report`, `schedule_retest`, `republish_mirror_bundle` |

## Narrowed below the marketed claim

- `task_event_envelope`: marketed `stable` → certified `beta` (withheld).

## Sources

- Certification report: `artifacts/certification/m5-public-contract-certification.md`
- Contract-health register: `artifacts/release/m5-contract-health.json`
- Publication matrix: `artifacts/contracts/m5-stability-lifecycle-map.json`
- Help-center page: `docs/help/m5-public-contract-certification.md`
