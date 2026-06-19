# Backup, restore, and failover continuity packets

This contract turns backup/restore/failover proof into first-class continuity
evidence instead of implicit ops lore. Every claimed managed, self-hosted, or
sovereign surface that carries resilience language must point to one typed
**backup/restore/failover packet** that a person — in shiproom, support, docs, or
a partner qualification — can read directly.

For each claimed continuity row it produces one **descriptor** that answers the
same questions everywhere:

1. Which continuity family backs the claim — backup, restore, failover,
   snapshot/replication, or local-core continuity — and which claim row does it
   back?
2. What did the most recent drill actually exercise, and — when the drill was
   only partial — what restored *narrower than normal* or was not exercised at
   all?
3. What restore identity does a recovery reproduce, and what partial loss is
   disclosed on recovery?
4. On what cadence is the packet drilled, who owns the drill now and next, when
   was it last drilled, and when does its evidence age out under the freshness
   SLO?

The packet is produced by
`aureline_continuity::m5_backup_restore_failover_packets`. It reuses the
continuity packet-family, restore-identity, partial-loss, restore/failover
hosting, drill-cadence, drill-evidence, and qualification vocabulary from the
frozen continuity-claim matrix
(`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`) so there is
exactly one continuity vocabulary across the product. The descriptor is then
projected identically onto the release-center, shiproom, support-center, partner
qualification, and public claim-manifest surfaces.

## What every surface answers the same way

- Which continuity family backs the claim, and which claim row?
- What did the last drill exercise, and what restored narrower than normal?
- What restore identity does recovery reproduce, and what partial loss is
  disclosed?
- On what cadence is it drilled, who owns it now and next, and when does the
  evidence age out?

## Stable conditions

A page qualifies `stable` only when all of the following hold at once:

1. Every managed-family packet records the typed operations its most recent
   drill exercised (an exercised scope, not narrative text).
2. A partially-exercised drill discloses what restored narrower than normal or
   was not exercised.
3. Every managed-family packet declares the restore identity recovery reproduces
   and discloses its partial-loss behavior on recovery.
4. Every managed-family packet names a recurring drill cadence, a current and a
   future drill owner, and — for current or graced evidence — a last-drill
   timestamp and a freshness-SLO expiry.
5. Every claimed resilience row points to a current packet (no row carries
   resilience language without one).
6. Every packet is projected onto all five surfaces, and the restore-identity,
   partial-loss, scope, and drill vocabulary is identical across every
   projection.

## Fail-closed guardrail: no generic "DR tested" text

The load-bearing guardrail is that a backup/restore/failover claim may never rest
on generic "DR tested" text. A packet **fails closed** — its claim is withdrawn —
when it sets `generic_dr_text_only`, and a self-hosted or sovereign packet that
hides a vendor-operated restore or failover lane is likewise withdrawn. A managed
packet that exercised nothing in its most recent drill cannot exceed preview.

## Automatic claim narrowing

The `DrillPacketRegistry` is the typed consumer the release-center, shiproom,
support-center, partner-qualification, and public claim-manifest surfaces read.
It reports, per claimed resilience row, whether a current packet backs the claim.
A row narrows automatically when its packet is missing, stale (aged out under the
freshness SLO), or withheld:

| Condition | Coverage | Qualification |
|---|---|---|
| A current packet backs the claim | `current_packet` | `stable` |
| The packet exists but its drill evidence is stale | `stale_packet_needs_refresh` | `beta` / `preview` |
| The packet relies on generic "DR tested" text or hides a vendor lane | `packet_withheld` | `withdrawn` |
| No packet backs the claimed resilience row | `no_packet` | `preview` |

## Export safety

The packet is metadata-only. Restore-identity and partial-loss fields are
export-safe by default and remain visible in operator and support surfaces. It
carries closed-vocabulary tokens, export-safe plain-language labels, UTC
timestamps, and opaque refs only. Raw backup bytes, raw provider payloads, raw
hostnames, raw KMS handles, and secret material never cross this boundary.

## Schema, artifact, and fixtures

- Schema: `schemas/continuity/backup_restore_failover_packet.schema.json`
- Artifact summary: `artifacts/m5/continuity/backup_restore_failover_packets.md`
- Canonical evidence packets: `artifacts/m5/continuity/drill_packets/`
- Fixtures: `fixtures/continuity/restore_identity_cases/`
- Validator: `python3 tools/validate_m5_backup_restore_failover_fixtures.py`
- CLI inspect: `aureline_backup_restore_failover_inspect`
