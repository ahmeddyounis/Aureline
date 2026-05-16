# Registry moderation beta

This document is the reviewer-facing contract for moderated extension
catalog descriptors. The machine-readable source is
[`schemas/extensions/catalog_descriptor.schema.json`](../../../schemas/extensions/catalog_descriptor.schema.json),
the Rust model is
[`crates/aureline-extensions/src/registry/`](../../../crates/aureline-extensions/src/registry/),
the fixture suite is
[`fixtures/extensions/m3/registry_moderation/`](../../../fixtures/extensions/m3/registry_moderation/),
and the checked packet lives under
[`artifacts/extensions/m3/registry_moderation/`](../../../artifacts/extensions/m3/registry_moderation/).

The descriptor is intentionally narrow. It does not add marketplace
recommendations, ranking UI, vanity listing fields, publisher self-service,
or a registry service. It makes each catalog row explainable, mirrorable,
and support-exportable from one metadata source.

## Required Descriptor Truth

Every catalog descriptor carries:

| Field family | Required truth |
|---|---|
| Package identity | descriptor id, extension identity, package id, version, display name, publication ref, manifest refs |
| Publisher continuity | publisher id, continuity ref, trust tier, continuity state, signing-key refs, lifecycle event refs, freshness |
| Lifecycle | catalog lifecycle state, source registry class, source endpoint ref, channel class, support class |
| Moderation | pending/admitted/limited/revoked/quarantined state, review ref, reason refs, anti-abuse refs, primary and backup operator refs |
| Revocation readiness | revocation state, snapshot ref and age, last-known-good version, rollback manifest, emergency-disable refs |
| Mirror metadata | mirrorability class, content address, signature ref, trust-inheritance rule, parity assertion refs |
| Compatibility | claim class, bridge state, rendered label, host-contract refs, capability-world refs, target platforms, caveats |
| Disclosures | publisher continuity, lifecycle, moderation, revocation, compatibility, mirror metadata, permission manifest, rollback posture |

Rows are refused when identity is missing, publisher continuity is missing,
required disclosures are absent, mirror metadata is incomplete or blocked,
revocation snapshots are stale, rollback metadata is missing, or the row is
revoked or quarantined. Rows can remain staged or limited without losing the
same mirror and support metadata.

## Decision Vocabulary

| Decision | Typical reason | Meaning |
|---|---|---|
| `ready_for_catalog` | `ready_mirrorable_trust_complete` | Moderation admitted the row; publisher continuity, revocation, compatibility, and mirror metadata are complete. |
| `staged_for_review` | `staged_pending_moderation` | The row is structurally valid and mirrorable, but moderation or channel state is still staged. |
| `limited` | `limited_by_policy_or_compatibility` | The row remains inspectable with a compatibility, policy, source, or bridge limitation. |
| `refused` | typed refused reason | Install/update catalog mutation is blocked, but support export still explains why. |

Support exports preserve the same `lifecycle_state_class`,
`moderation_state_class`, `revocation_state_class`, and
`mirrorability_class` as the catalog descriptor. Support and docs should not
invent a separate explanation for staged, limited, revoked, or quarantined
rows.

## Fixture Drills

| Fixture | Expected result |
|---|---|
| `mirrorable_catalog_approved.json` | admitted descriptor with active publisher continuity, fresh revocation metadata, rollback metadata, and mirror parity refs |
| `staged_pending_moderation.json` | staged descriptor that remains mirrorable while pending registry review |
| `limited_compatibility_catalog.json` | limited descriptor caused by bridge-backed compatibility and mirror re-verification |
| `revoked_catalog_refused.json` | refused descriptor with signed revocation and last-known-good rollback metadata |
| `quarantined_publisher_refused.json` | refused descriptor caused by publisher quarantine with operator handoff refs |

## Headless Usage

Run the Rust fixture suite:

```text
cargo test -p aureline-extensions registry::tests
```

Dump the descriptor and support-export records for schema validation:

```text
cargo run --example dump_registry_moderation_records -p aureline-extensions
```

Validate the checked artifact shape with the schema drift tool:

```text
python3 tools/check_schema_example_drift.py
```

## Checked Outputs

| Output | Purpose |
|---|---|
| `catalog_descriptor_record.json` | canonical moderated catalog descriptor for the sample extension |
| `catalog_descriptor_support_export.json` | metadata-safe support/export projection |

The support export is the first consuming surface. It repeats only
metadata-safe refs and state classes, and it offers actions that map back to
moderation review, publisher continuity, revocation notice, mirror details,
compatibility report, pinning, admin review, removal/disablement, and packet
export.
