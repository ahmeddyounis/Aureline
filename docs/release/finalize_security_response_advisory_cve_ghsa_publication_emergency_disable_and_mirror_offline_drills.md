# Security-response, advisory, CVE/GHSA publication, emergency disable, and mirror/offline drill packet

This document is the human-readable companion to the checked-in security-response
packet. The canonical machine source is the JSON packet; this doc explains the
model, the vocabulary, and how to update the packet.

## Packet location

- Checked-in packet: `artifacts/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.json`
- Typed consumer: `aureline_release::finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills`
- Proof packet: `artifacts/release/m4/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills_proof_packet.md`

## Response lanes

The packet governs five response lanes. Each lane is one row in the packet.

| Lane | Kind | Release-blocking | What it governs |
|---|---|---|---|
| Core security response process | `security_response` | Yes | Triage, incident declaration, evidence preservation, and coordination |
| Security advisory publication | `advisory_publication` | Yes | Drafting, review, and release of customer-facing security advisories |
| CVE/GHSA publication | `cve_ghsa_publication` | Yes | Assignment and publication to external vulnerability databases |
| Emergency disable capability | `emergency_disable` | Yes | Kill-switch, feature-flag disable, and rapid-response circuit breakers |
| Mirror/offline drill | `mirror_offline_drill` | Yes | Verification that approved mirrors and offline bundles can be imported and verified |

## Lifecycle labels

The packet reuses the stable claim level vocabulary rather than minting
per-response labels:

- `lts` — long-term-support stable
- `stable` — broad stable
- `beta` — narrowed below the stable cutline
- `preview` — narrowed below the stable cutline
- `withdrawn` — claim withdrawn

## Response states

| State | Meaning |
|---|---|
| `ready` | The lane is ready: current packet, satisfied controls / verified checkpoints, owner-signed |
| `ready_on_waiver` | Ready only because an active, unexpired waiver covers a residual gap |
| `not_ready_unbacked` | Incomplete evidence, unsatisfied control, unverified checkpoint, or missing sign-off |
| `not_ready_claim_narrowed` | The backing public claim is itself below the cutline |
| `not_ready_stale` | The response packet breached its freshness SLO or is missing |
| `not_ready_waiver_expired` | The lane relied on a waiver that has expired |

## Gap reasons

| Reason | When it fires |
|---|---|
| `claim_label_narrowed` | The backing public claim narrowed below the cutline |
| `response_evidence_incomplete` | Required response evidence is incomplete |
| `response_packet_freshness_breached` | The response packet breached its freshness SLO |
| `response_packet_missing` | No response packet has been captured |
| `waiver_expired` | A waiver the lane relied on has expired |
| `owner_signoff_missing` | The required lane owner sign-off is missing |
| `emergency_control_unsatisfied` | One or more required emergency controls are unsatisfied |
| `mirror_drill_unverified` | One or more required mirror drill checkpoints are unverified |

## Emergency controls

An emergency-disable lane must carry at least one [`EmergencyControl`]. Each
control records:

- `control_id` — stable control id
- `title` — human-readable title
- `control_ref` — ref to the artifact or policy the control checks
- `satisfied` — whether the control is satisfied

For a lane to be `ready`, every emergency control must be `satisfied`.

## Mirror drill checkpoints

A mirror/offline-drill lane must carry at least one [`MirrorDrillCheckpoint`].
Each checkpoint records:

- `checkpoint_id` — stable checkpoint id
- `title` — human-readable title
- `restore_point_ref` — ref to the restore point or artifact the checkpoint verifies
- `verified` — whether the checkpoint was verified

For a lane to be `ready`, every mirror drill checkpoint must be `verified`.

## Response rules

Each gap reason has a corresponding response rule. A rule fires when a watched
row carries its trigger reason. Rules that `blocks_publication: true` can hold
the security-response packet publication gate.

## How to update the packet

1. Edit `artifacts/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.json`.
2. Update `as_of` to the current UTC date.
3. Ensure every response kind has at least one row.
4. Ensure every declared release-blocking lane has a covering row.
5. Ensure the `effective_label` of every row is at or below its `claim_label`.
6. Ensure ready rows ride a captured within-SLO packet, have no active gap reasons,
   satisfy every emergency control (if any), verify every mirror checkpoint (if any),
   and are owner-signed.
7. Ensure narrowing rows carry at least one active gap reason and are below the cutline.
8. Ensure the `publication` block matches the computed decision and blocking sets.
9. Ensure the `summary` block matches the computed counts.
10. Run the tests to confirm the packet validates cleanly.

## Verification

```sh
cargo test -p aureline-release --test finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills
```

## Response-packet freshness SLO

Each response packet carries a freshness SLO:

- `target_max_age_days` — the packet may be at most this many days old
- `warn_within_days` — days-remaining threshold at or below which the packet is `due_for_refresh`
- `slo_register_ref` — ref into the freshness-SLO register that defines this target

The standard target for security-response packets is 45 days with a 10-day warn
window. Emergency-disable and mirror/offline-drill lanes may use shorter targets
if their rehearsal cadence demands it.
