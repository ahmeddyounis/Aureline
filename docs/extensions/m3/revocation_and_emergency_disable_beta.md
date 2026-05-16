# Revocation and emergency disable beta

This document is the reviewer-facing contract for extension advisory,
emergency-disable, quarantine, and revocation communication. The
machine-readable source is
[`schemas/extensions/revocation_and_emergency_disable.schema.json`](../../../schemas/extensions/revocation_and_emergency_disable.schema.json),
the Rust model is
[`crates/aureline-extensions/src/revocation/`](../../../crates/aureline-extensions/src/revocation/),
the fixture suite is
[`fixtures/extensions/m3/revocation_and_emergency_disable/`](../../../fixtures/extensions/m3/revocation_and_emergency_disable/),
and the checked packets live under
[`artifacts/extensions/m3/advisory_templates/`](../../../artifacts/extensions/m3/advisory_templates/).

The lane is intentionally narrow. It does not add marketplace growth,
recommendation, appeal, or publisher self-service features. It makes
emergency ecosystem incidents actionable and explainable for installed
extensions, primary registries, approved mirrors, support exports, and
CLI/headless consumers.

## Required Incident Truth

Every incident communication packet carries:

| Field family | Required truth |
|---|---|
| Advisory identity | copy-safe advisory ID, incident ID, publish/update timestamps, disclosure refs |
| Affected extension | extension identity, package ID, version, publisher, registry manifest, catalog descriptor, runtime contract, installed-state refs |
| Severity and reason | controlled severity and reason-code vocabulary |
| Source and actor | source class, signer ref, actor class, policy refs, audit event refs |
| Registry and mirror state | primary registry lane, mirror lane, freshness, signer continuity, mirror continuity, import requirement |
| Blocked operations | new install, update, auto-update, activation, execution, or mirror import |
| Lifecycle state | `advisory_active`, `disabled`, `quarantined`, `revoked`, `mitigated_locally`, or `resolved_retained` |
| Recovery guidance | last-known-good version, rollback manifest, pin/remove/refresh/admin actions, safe-mode ref |
| Support export parity | same incident ID, advisory ID, lifecycle state, revocation state, and blocked operation set |

Packets are refused when incident identity is missing, affected extension
identity is missing, required disclosures are not rendered, primary or
mirror trust state is ambiguous, forced actions omit blocked operations, or
forced actions omit rollback/recovery guidance.

## Decision Vocabulary

| Decision | Typical reason | Meaning |
|---|---|---|
| `advisory_published` | `advisory_no_forced_action` | Advisory is active, installed items remain visible, and no forced disablement is applied. |
| `disable_engaged` | `emergency_disable_engaged` | Extension remains installed but blocked from the declared operations until the incident clears or recovery is applied. |
| `quarantine_engaged` | `quarantine_engaged` | Extension remains visible for review and cannot reactivate until a user, admin, runtime, or signed policy path clears it. |
| `revocation_engaged` | `revocation_engaged` | Artifact or publisher-backed state is revoked; install/update/activation/execution remain blocked with rollback guidance attached. |
| `awaiting_mirror_import` | `awaiting_mirror_import` | Primary incident metadata is known, but the mirror lane must import signed advisory or revocation metadata before acting. |
| `refused` | typed refused reason | Packet is incomplete or unsafe to apply; support export can still explain the refusal if generated. |

## Mirror Rules

Primary registry and mirror lanes must both name a trust state. A mirror is
never treated as implicitly current just because the primary registry has an
advisory. Accepted mirror states are explicit:

| Mirror state | Meaning |
|---|---|
| `verified_current` | Mirror has current signed incident metadata. |
| `verified_stale` | Mirror metadata verifies but dependent claims must narrow. |
| `pending_mirror_import` | Mirror must import signed advisory or revocation metadata before acting. |
| `mirror_continuity_broken` | Mirror promotion or continuity failed; mirror import is blocked. |
| `signature_or_digest_mismatch` | Mirror verification failed; mirror import is blocked. |

`unknown_refused` and `revocation_snapshot_missing` are not actionable
states for this lane.

## Fixture Drills

| Fixture | Expected result |
|---|---|
| `primary_registry_emergency_disable.json` | forced disable from a signed emergency bundle with current primary and mirror state |
| `mirror_quarantine_pending_reverify.json` | quarantine from an approved mirror with broken mirror continuity and explicit primary registry state |
| `artifact_revoked_mirror_parity.json` | artifact revocation current on both primary registry and approved mirror |

## Headless Usage

Run the Rust fixture suite:

```text
cargo test -p aureline-extensions revocation
```

Dump incident and support-export records:

```text
cargo run -q -p aureline-extensions --example dump_revocation_records
cargo run -q -p aureline-extensions --example dump_revocation_records -- incident primary_registry_emergency_disable
cargo run -q -p aureline-extensions --example dump_revocation_records -- support-export primary_registry_emergency_disable
```

Validate checked examples and schema drift:

```text
python3 tools/check_schema_example_drift.py
```

## Checked Outputs

| Output | Purpose |
|---|---|
| `emergency_disable_incident_packet.json` | canonical forced-disable packet |
| `emergency_disable_support_export.json` | metadata-safe support/export projection |
| `mirror_quarantine_incident_packet.json` | mirror quarantine packet with explicit trust state |
| `artifact_revocation_incident_packet.json` | revocation packet with primary/mirror parity |
| `security_advisory_notice_template.md` | human-readable notice template aligned to packet fields |

Support exports are the first consuming surface. They repeat only
metadata-safe refs and state classes, preserve the same incident and
lifecycle identifiers as the packet, and offer actions that map back to
advisory detail, recovery guidance, rollback/pin, removal/disablement,
mirror refresh, admin review, and packet export.
