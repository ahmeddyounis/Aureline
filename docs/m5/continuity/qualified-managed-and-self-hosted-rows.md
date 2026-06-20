# Qualified managed and self-hosted continuity rows

Reviewer-facing companion to the continuity certification lane in
`crates/aureline-continuity/src/m5_continuity_certification/`. It documents how
every claimed managed, self-hosted, and sovereign continuity row earns one
certification verdict — and how that verdict narrows automatically when the
backing evidence is stale, partial, missing, or profile-mismatched.

## What this lane certifies

The upstream continuity lanes each freeze one slice of continuity truth:

- locality, tenant, and key-mode disclosure
  (`docs/m5/continuity/locality-and-tenant-boundary-surfaces.md`,
  `docs/m5/continuity/key-mode-and-storage-posture.md`,
  `docs/m5/continuity/locality_tenant_keymode_and_drill_matrix.md`);
- typed control-plane versus data-plane degradation
  (`docs/m5/continuity/control-plane-vs-data-plane-degradation.md`);
- backup, restore, and failover drills
  (`docs/m5/continuity/backup-restore-failover-packets.md`);
- restore identity and partial-loss semantics
  (`docs/m5/continuity/post-restore-truth-and-replay-fences.md`);
- mirror-only and air-gapped offline continuity
  (`docs/m5/continuity/mirror-airgap-continuity.md`); and
- continuity-proof freshness against an SLO
  (`docs/release/m5-continuity-shiproom-gates.md`).

This certification lane **folds them into one verdict per row** so release
packets, Help/About truth, service-health summaries, support exports, and partner
qualification packets read a single certified/non-certified continuity verdict
instead of re-deriving it from six separate packets.

## Certification verdict

A certification-scope row (any managed, self-hosted, or sovereign surface, or a
row carrying a claimed managed dependency) stays **certified** only when every
required continuity dimension is `current`. Otherwise the claim narrows
automatically:

| Evidence state | Effective label |
| --- | --- |
| `current` / `not_applicable` | holds its claim |
| `stale` / `partial` | narrows to `beta` |
| `missing` | narrows to `preview` |
| `profile_mismatched` | `withdrawn` |

Required dimensions are `locality_tenant_key`,
`control_data_plane_degradation`, `backup_restore_failover`,
`restore_identity_partial_loss`, and `drill_freshness_slo`; air-gapped and
mirror-only rows additionally owe `mirror_offline_continuity`. The effective
label is a hard ceiling — a row may never publish a label above the one its
evidence supports, so enterprise/managed language can never run broader than the
proof.

## Guardrails

- **The local-core lane is never narrowed.** A pure local-only row with no
  claimed managed dependency rides the local-core continuity lane and stays
  certified even when a managed row goes stale.
- **No shared reference drill.** A single reference-environment
  backup/restore/failover drill may not stand in for more than one claimed
  profile row; reusing a drill ref narrows every row that shares it.
- **One verdict, every surface.** A scope row's verdict must reach About, Help,
  service-health, support exports, docs/public-truth, and partner qualification;
  the local-core lane reaches the in-product and public-truth surfaces.
- **No claim fresher than the clock.** A row may not certify continuity-proof
  freshness fresher than the freshness-SLO dashboard
  (`artifacts/m5/continuity/freshness_slo_dashboard.json`) records.

## Artifacts

- `artifacts/m5/continuity/certification/certified_rows.json` — canonical
  certified-row registry (the full report packet)
- `artifacts/m5/continuity/certification/certification_support_export.json` —
  redaction-safe support-export projection
- `artifacts/m5/continuity/certification/certification_report.md` — human report
- `artifacts/m5/continuity/certification/drill_freshness_report.md` — per-row
  drill and freshness posture
- `schemas/continuity/continuity_certification_report.schema.json` — schema
- `fixtures/continuity/certification_cases/` — narrowing/withdrawal/local-core
  fixtures

## Verification

```sh
cargo test -p aureline-continuity --locked m5_continuity_certification
python3 tools/validate_m5_continuity_certification_fixtures.py
python3 tools/check_m5_continuity_certification.py
```

CLI/headless inspection of any report:

```sh
cargo run -q -p aureline-continuity --bin aureline_continuity_certification_inspect -- \
  artifacts/m5/continuity/certification/certified_rows.json
```

This certification report, certified-row registry, and drill-freshness report are
the canonical M5 source for continuity qualification truth; they are linked from
the canonical M5 evidence index.
