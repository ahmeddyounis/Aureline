# Locality, tenant/key-mode, and continuity-drill matrix

This contract freezes one canonical packet that turns region/residency, tenant
scope, key mode, backup/restore/failover evidence, restore identity, partial
loss, and local-core continuity into typed continuity-claim rows instead of
deployment footnotes. The packet is produced by
`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix` and is the
single fact source About, Help, service-health, support exports, docs and
public-truth pages, and partner qualification packets reuse when they need to
answer the same continuity questions the same way:

1. Where does processing and storage actually happen, which tenant boundary
   applies, and which key mode protects durable state for a claimed surface?
2. Does the surface sit on the local-core continuity lane or the managed
   continuity lane, and does its degraded fallback distinguish control-plane
   impairment from data-plane impairment?
3. Which named backup, restore, failover, or snapshot continuity packet family
   backs the claim, what restore identity does a recovery reproduce, and what
   partial loss is disclosed?
4. On what cadence is that continuity packet drilled, who owns the drill now and
   next, and is the drill evidence current, reconstructable, stale, or never run?

## Track invariant

No claimed managed, self-hosted, or sovereign row reaches stable or beta truth
without explicit locality/tenant/key disclosure, typed control-plane versus
data-plane degradation, current or reconstructable backup/restore/failover drill
evidence, restore-identity and partial-loss semantics, and automatic claim
narrowing when continuity evidence is stale, partial, or profile-mismatched.

## Stable conditions

The matrix qualifies `stable` only when all of the following hold at once:

1. Every claimed row discloses processing locality, storage locality, and a
   residency label, plus its partial-loss behavior on recovery.
2. Every managed-scope row (managed, self-hosted, sovereign, or any row carrying
   a claimed managed dependency) declares an explicit tenant scope and key mode.
3. Every managed-lane row names a backup, restore, failover, or snapshot
   continuity packet family and references the continuity packet backing it.
4. Every managed-scope row names a drill cadence and both a current and future
   drill owner, with drill evidence that is current, within grace, or
   reconstructable from a verified snapshot.
5. Every row backed by a managed continuity packet declares the restore identity
   a successful recovery reproduces.
6. The matrix as a whole classifies at least one control-plane impairment row and
   one data-plane impairment row, and at least one local-core lane row and one
   managed-lane row.
7. Every managed-scope row's continuity facts are reused by About, Help,
   service-health, support exports, docs/public-truth, and partner qualification
   packets.

## Claim narrowing

Rows lacking current or reconstructable continuity evidence are pre-marked to
narrow rather than silently inheriting green enterprise or managed language.
Each row carries a computed outcome with its own qualification and reasons:

- `drill_evidence_stale` or `drill_never_run` — drill evidence is stale (beta) or
  has never been run (preview).
- `locality_undisclosed`, `tenant_key_posture_missing`,
  `continuity_packet_family_missing`, `drill_cadence_or_owner_missing`,
  `restore_identity_undeclared`, `partial_loss_undisclosed`,
  `surface_reuse_incomplete` — a required disclosure is missing (beta).
- `degraded_fallback_class_missing`, `continuity_lane_distinction_missing` — the
  matrix fails to distinguish the planes or the lanes (beta).
- `profile_mismatch` — the claimed profile is inconsistent with its own posture,
  e.g. a sovereign row claiming shared multi-tenancy or vendor-managed keys, or a
  managed row claiming purely local-core restore (preview).

## Hard guardrails

Two conditions are special:

- A self-hosted or sovereign row that hides a vendor-operated restore or failover
  lane reports `sovereign_continuity_overclaimed` and the row's claim is
  **withdrawn** rather than silently weakened.
- A local-only desktop row is never redefined as managed continuity scope. If one
  is marketed as managed continuity (a managed lane or a managed packet family)
  without a claimed managed or self-hosted dependency, it reports
  `local_only_overclaimed_as_managed` and is held at preview. Correctly scoped
  local-core rows stay out of managed continuity requirements entirely.

## Output shape

The packet contains:

- continuity-claim rows with profile class, continuity lane, locality posture,
  tenant scope, key mode, degraded-fallback plane, continuity packet family,
  restore/failover hosting, restore identity, partial loss, and a drill block
- per-row outcomes joining each row to its computed qualification, narrow
  reasons, and managed-scope flag
- a consolidated drill schedule grouped by continuity packet family
- a summary and typed defects, plus a support-export wrapper

The packet is metadata-only. It intentionally excludes raw hostnames, raw tenant
identifiers, raw KMS handles, raw trust roots, raw backup payloads, and all
secret material.

## Canonical paths

- Doc: `docs/m5/continuity/locality_tenant_keymode_and_drill_matrix.md`
- Artifact: `artifacts/m5/continuity/claim_rows_and_drill_schedule.md`
- Schema: `schemas/continuity/m5-continuity-claim-row.schema.json`
- Fixtures: `fixtures/continuity/m5-continuity-profile-cases/`

This checked-in matrix, schema, fixtures, and drill schedule are the canonical
source for locality and continuity truth on claimed managed, self-hosted, and
sovereign rows. Later docs, help, support, and shiproom surfaces consume them
instead of restating continuity claims by hand.

## Verify

```sh
cargo test -p aureline-continuity m5_locality --locked
cargo run -q -p aureline-continuity --example dump_m5_continuity_claim_matrix_fixtures -- page
python3 tools/validate_m5_continuity_claim_matrix_fixtures.py
```
