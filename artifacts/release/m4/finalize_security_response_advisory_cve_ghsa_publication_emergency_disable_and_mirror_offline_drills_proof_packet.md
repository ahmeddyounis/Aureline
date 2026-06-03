# Security-response packet — proof packet

Reviewer-facing proof packet for the gated security-response, advisory, CVE/GHSA
publication, emergency disable, and mirror/offline drill packet for the release
line's security-response lanes.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Packet: [`/artifacts/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.json`](../finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.json)
- Schema: [`/schemas/release/security_response_packet.schema.json`](../../../schemas/release/security_response_packet.schema.json)
- Companion doc: [`/docs/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.md`](../../../docs/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.md)
- Typed consumer: `aureline_release::finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills`

## What this packet proves

1. **Each security-response lane binds a response packet to a public claim.**
   Every row binds one lane (`response_kind`, `lane_ref`) to the response packet
   that grounds it (`response_packet`), the emergency controls it must satisfy
   (`emergency_controls`), the mirror drill checkpoints it must verify
   (`mirror_drill_checkpoints`), the waiver that holds it provisionally
   (`waiver`), and the public claim whose lifecycle label it backs (`claim_ref`,
   `claim_label`). The packet reuses the stable claim level vocabulary rather than
   minting per-lane labels, so docs, Help/About, the release center, and support
   exports render one label per lane.

2. **The packet ingests the stable claim manifest as a hard ceiling.** The CI
   gate reads the stable claim manifest named by `claim_manifest_ref` and fails
   when a row's `claim_label` is not the label that manifest publishes for the
   entry named by `claim_ref`, when a row names an entry the manifest does not
   carry, or when a response is backed wider than the public claim's canonical
   label. A lane's effective label can never outrun the public claim it backs.

3. **The packet-freshness, waiver-expiry, emergency-control, and mirror-drill
   automations narrow unready lanes before publication.** Each row's response
   packet carries a freshness SLO and a recorded `slo_state`; each emergency
   disable lane carries an `emergency_controls` set; each mirror/offline-drill
   lane carries a `mirror_drill_checkpoints` set. The CI gate recomputes the
   freshness state and the waiver-expiry state against the packet `as_of` date
   and fails when a declared state overstates the clock, when a ready lane rides
   a stale packet or an expired waiver, or when a lane that lost its waiver
   still claims readiness.

4. **The five response kinds and the release-blocking lane set stay covered.**
   The gate fails if any of `security_response`, `advisory_publication`,
   `cve_ghsa_publication`, `emergency_disable`, or `mirror_offline_drill` has no
   row, if a declared release-blocking lane has no covering row, if a
   release-blocking row is not declared, or if a `lane_ref` repeats.

5. **The publication verdict is recomputed, not asserted.** The gate recomputes
   the `hold`/`proceed` decision and the blocking rule/lane sets from the firing
   response rules and fails on any drift. With `--require-proceed` it exits
   non-zero on `hold`, so shiproom and release tooling block security-response
   publication directly from this artifact.

## Current snapshot (as of 2026-06-02)

The checked-in packet holds publication. Of eight rows across two public claims,
four carry labels at or above the cutline:

- the **core security response process** is ready, backed by a current packet
  and owner sign-off;
- the **advisory publication** lane is ready on an active waiver covering a
  pending full-cadence rehearsal;
- the **emergency disable capability** is ready, with both required emergency
  controls satisfied; and
- the **advisory template seed** is ready, with a current packet and owner
  sign-off.

Four lanes are narrowed below the cutline:

- the **CVE/GHSA publication** lane narrowed to beta because its response packet
  breached its freshness SLO;
- the **mirror/offline drill** lane narrowed to beta because its provisional
  waiver expired before the quarterly full-drill rehearsal landed;
- the **offline bundle verification** lane narrowed to beta because it has no
  captured response packet and an unverified mirror drill checkpoint; and
- the **security response alpha rehearsal** inherits a beta ceiling from its
  upstream public claim.

Three of the narrowed lanes — CVE/GHSA publication, mirror/offline drill, and
offline bundle verification — back public claims still published Stable, so they
fire four blocking response rules and hold the
`security_response_packet_publication` gate. The packet narrows the optimistic
Stable security-response promises automatically instead of letting them ride;
publication clears once the CVE/GHSA packet is refreshed, the mirror/offline
rehearsal lands, and the offline bundle verification packet is captured.

## How to re-verify

```
cargo test -p aureline-release --test finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills
```

The protected tests parse the checked-in packet, confirm every response kind is
covered, verify the release-blocking lane set, cross-check the frozen validation
capture, and exercise the negative fixtures to prove that narrowing failures,
stale packets, unsatisfied emergency controls, unverified mirror checkpoints,
and inconsistent publication decisions are all rejected by the typed model.
