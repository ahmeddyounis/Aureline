# Shiproom claim packet — M5 supportability qualification

This packet is the shiproom- and release-center-facing view of the M5
supportability qualification family. It does not maintain its own summary: the
claim scope below is derived from the canonical qualification index and narrows
automatically when a surface/profile row goes stale, missing, or red.

## Canonical inputs

- Qualification index: `fixtures/support/m5/m5-supportability-qualification/packet.json`
- Review artifact: `artifacts/support/m5/m5-supportability-qualification.md`
- Boundary schema: `schemas/support/m5-supportability-qualification.schema.json`
- Companion doc: `docs/help/support/m5-supportability-qualification.md`
- Crate consumer: `crates/aureline-support/src/m5_supportability_qualification/mod.rs`

- Claim publishable: **yes**
- Qualified rows: `30`
- Narrowed rows: `0`
- Blocked rows: `0`
- Claimed profiles: `5` (`desktop_local_first`, `hybrid_remote_attach`, `managed_cloud`, `self_hosted_sovereign`, `air_gapped_mirror_only`)
- Deployment modes: `2` (`desktop`, `cli_headless`)

## Claim scope

Each supportability surface is qualified on all five claimed profiles across
both deployment modes. Each row stands on its own backing lane proof.

| Surface | Backing lane | Claim | Local-save first |
| ------- | ------------ | ----- | ---------------- |
| `support_center` | `m5_support_center_matrix` | **Published** (5/5 profiles) | n/a |
| `environment_explainability` | `m5_environment_status_strips` | **Published** (5/5 profiles) | n/a |
| `precedence_inspection` | `m5_precedence_inspectors` | **Published** (5/5 profiles) | n/a |
| `support_bundle_consent` | `m5_support_bundle_consent` | **Published** (5/5 profiles) | yes |
| `crash_intake_recovery` | `m5_crash_intake_and_recovery` | **Published** (5/5 profiles) | yes |
| `supportability_handoff` | `m5_supportability_handoff_packets` | **Published** (5/5 profiles) | yes |

## Sign-off gate

Promotion of the supportability claim holds unless all of the following are true
on the current qualification index:

1. Every claimed supportability surface binds one row on every claimed profile
   across both deployment modes (30 rows), and no row carries a stale-proof
   token (`published_state == qualified`).
2. No surface borrows another surface's proof: each row cites its own boundary
   schema, canonical artifact, fixture corpus, and record kind
   (`surface_evidence_stale` is not triggered by a borrowed lane).
3. No bound supportability drill is stale (`supportability_drill_stale` is not
   open for diagnosis latency, consent-sheet accuracy, export-mode parity,
   crash-loop recovery, or exact-build intake).
4. Every send-capable surface keeps local-save first-class
   (`local_save_first == true`) so no narrowing demotes local-save below a send
   path.
5. Help/About, the desktop Support Center, CLI/headless, support export, this
   shiproom packet, and the release manifest still ingest the index by reference
   (`consumer_binding_missing` is not open).

A surface that loses its own evidence or a bound drill narrows to
`limited_profile_scoped`, `local_self_diagnosis_only`, or `blocked_unverified`
on its affected rows, and this packet's qualified/narrowed/blocked counts move
with it. Support claims do not widen on profiles or deployment modes whose
supportability surfaces are stale, missing, or policy-blocked.

## Regenerating this packet

This packet is checked in alongside the index it derives from. When the
qualification contract changes, regenerate the index and re-run the crate tests
before re-reviewing:

```sh
cargo run -q -p aureline-support --example dump_m5_supportability_qualification_packet -- canonical \
  > fixtures/support/m5/m5-supportability-qualification/packet.json
cargo test -p aureline-support m5_supportability_qualification
```
