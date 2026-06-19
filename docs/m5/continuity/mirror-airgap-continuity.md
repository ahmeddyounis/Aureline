# Mirror-only and air-gapped continuity packets

This contract makes mirror-only, air-gapped, and self-hosted-restricted
continuity a first-class product truth instead of assuming a public-network
rescue path is always available. Every claimed offline-leaning surface that
carries mirror, sovereign, or air-gapped continuity language must point to one
typed **mirror/air-gap continuity packet** that a person — in shiproom, support,
docs, or a partner qualification — can read directly.

For each claimed offline row it produces one **descriptor** that answers the same
questions everywhere:

1. What trust-root continuity backs the boundary — which trust-root posture
   anchors it, and can that trust survive and renew offline without a public
   reissue?
2. How fresh is the mirror or offline bundle the boundary depends on, when was it
   last synced, and when does that freshness age out?
3. What offline import and export paths move content across the boundary without
   touching the public network?
4. Where do security advisories and revocation data come from — a signed offline
   bundle, the approved mirror, or (forbidden on an isolated row) a live public
   fetch?
5. Is public fallback **prohibited**, **unavailable**, or **policy-gated** —
   rather than silently attempted?

The packet is produced by
`aureline_continuity::m5_mirror_airgap_continuity_packets`. It binds trust-root
continuity to the same [`TrustRootPostureClass`] vocabulary the key-mode and
storage-posture surface uses
(`aureline_continuity::m5_key_mode_and_storage_posture`), and reuses the
deployment-profile and qualification vocabulary from the frozen continuity-claim
matrix (`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`), so
there is exactly one continuity vocabulary across the product rather than a
separate mirror-only dialect. The descriptor is then projected identically onto
the release-center, shiproom, support-center, partner-qualification, and public
claim-manifest surfaces.

[`TrustRootPostureClass`]: ../../../crates/aureline-continuity/src/m5_key_mode_and_storage_posture/mod.rs

## Connectivity posture

A row declares an explicit connectivity posture so an isolated boundary is never
confused with a connected one:

| Posture | Meaning | Public fetch | Live mirror |
|---|---|---|---|
| `mirror_only` | Served from an approved mirror, no public-network path | Forbidden | Required |
| `air_gapped` | Fully isolated; reached only by offline exchange | Forbidden | Not required (uses offline bundles) |
| `self_hosted_restricted` | Self-hosted with restricted, controlled egress | Allowed (governed) | Required |
| `local_only` | Pure local desktop surface, no managed or mirror lane | n/a | n/a (exempt) |

## What every surface answers the same way

- Which trust-root posture anchors the boundary, and does it survive offline?
- How fresh is the mirror or offline bundle, and when does it age out?
- What are the offline import and export paths?
- Where do advisories and revocation data come from?
- Is public fallback prohibited, unavailable, or policy-gated?

## Stable conditions

A page qualifies `stable` only when all of the following hold at once:

1. Every offline row declares its trust-root posture and how that trust root
   renews, and an isolated row's trust root renews without a public reissue.
2. Every mirror-backed row names a mirror and a current freshness window
   (last-synced timestamp and a freshness expiry).
3. Every offline row discloses its offline import and export paths and its
   advisory/revocation source.
4. Every offline row states its public-fallback policy as prohibited,
   unavailable, or policy-gated.
5. Every claimed offline row points to a current packet (no row carries
   offline-continuity language without one).
6. Every packet is projected onto all five surfaces, and the trust-root, mirror,
   offline-exchange, advisory, and public-fallback vocabulary is identical across
   every projection.

## Fail-closed guardrails: no quiet public fallback

Three load-bearing guardrails **fail closed** — the claim is withdrawn:

- A mirror-only or air-gapped row may not **silently** fall back to public
  endpoints (`silent_public_fallback`).
- Advisory or revocation language may not imply a **live public fetch** on a
  mirror-only or air-gapped row (`advisory_implies_live_public_fetch`).
- An isolated row's trust root may not require a **public reissue** to renew
  (`trust_root_breaks_offline`).

A self-hosted-restricted row is *not* isolated, so a governed live public
advisory fetch is allowed there and does not narrow.

## Automatic claim narrowing

The `OfflineContinuityRegistry` is the typed consumer the release-center,
shiproom, support-center, partner-qualification, and public claim-manifest
surfaces read. It reports, per claimed offline row, whether a current packet
backs the claim. A row narrows automatically when its packet is missing, stale,
or withheld:

| Condition | Coverage | Qualification |
|---|---|---|
| A current packet backs the claim | `current_packet` | `stable` |
| The packet exists but its trust-root, mirror, advisory, or fallback evidence is missing or stale | `stale_packet_needs_refresh` | `beta` / `preview` |
| The packet silently falls back to public, fetches advisories live on an isolated row, or breaks trust offline | `packet_withheld` | `withdrawn` |
| No packet backs the claimed offline row | `no_packet` | `preview` |

## Export safety

The packet is metadata-only. Trust-root and public-fallback fields are
export-safe by default and remain visible in operator and support surfaces. It
carries closed-vocabulary tokens, export-safe plain-language labels, UTC
timestamps, and opaque refs only. Raw mirror bytes, raw provider payloads, raw
endpoint hostnames, raw trust-root key material, and secret bodies never cross
this boundary.

## Schema, artifact, and fixtures

- Schema: `schemas/continuity/mirror_airgap_packet.schema.json`
- Artifact summary: `artifacts/m5/continuity/mirror_airgap_continuity_packets.md`
- Canonical evidence packets: `artifacts/m5/continuity/mirror_airgap/`
- Fixtures: `fixtures/continuity/mirror_airgap_cases/`
- Validator: `python3 tools/validate_m5_mirror_airgap_continuity_fixtures.py`
- CLI inspect: `aureline_mirror_airgap_continuity_inspect`
