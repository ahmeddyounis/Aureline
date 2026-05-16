# Extension mirror and manual-import beta baseline

This document is the reviewer-facing contract for extension artifacts that
arrive through the primary catalog, approved mirrors, offline bundles, or a
manual artifact import. The machine-readable source is
[`schemas/extensions/mirror_import_baseline.schema.json`](../../../schemas/extensions/mirror_import_baseline.schema.json),
the Rust model is
[`crates/aureline-extensions/src/mirror_import/`](../../../crates/aureline-extensions/src/mirror_import/),
the fixture suite is
[`fixtures/extensions/m3/mirror_import/`](../../../fixtures/extensions/m3/mirror_import/),
and the checked packet lives under
[`artifacts/extensions/m3/mirror_import_baseline/`](../../../artifacts/extensions/m3/mirror_import_baseline/).

The baseline is intentionally narrow. It does not add a registry service,
marketplace ranking, or a new install UI. It gives install review, support
exports, docs/help, and headless validation one record that says where the
artifact came from and whether catalog, permission, lifecycle, compatibility,
publisher-continuity, and trust-claim metadata survived import.

## Required Baseline Truth

Every mirror/manual import baseline carries:

| Field family | Required truth |
|---|---|
| Source visibility | route class, delivered source class, origin source class, and a safe source label |
| Artifact identity | delivered and origin content address, with import refused on digest drift |
| Publisher continuity | publisher id, continuity ref, trust tier, signing-key refs, lifecycle events, freshness |
| Permission metadata | permission manifest ref, declared refs, effective refs, delta ref, freshness |
| Compatibility metadata | compatibility report ref, host-contract refs, capability-world refs, target platforms, rendered label |
| Lifecycle metadata | lifecycle metadata ref, lifecycle state, source endpoint ref, channel class, support class |
| Trust claims | one row per claim, with verified, preserved, downgraded, missing, or refused state |
| Disclosures | publisher continuity, source lane, artifact identity, permission manifest, compatibility metadata, lifecycle metadata, trust claims, revocation snapshot, and native install-review handoff |

Rows are refused when identity is missing, required disclosures are absent,
artifact identity changes, publisher continuity is missing, permission,
compatibility, or lifecycle metadata is missing, or a trust claim is refused.
Rows can continue to native install review with downgraded trust claims when
the semantic metadata is preserved and the downgraded claim is named.

## Decision Vocabulary

| Decision | Typical reason | Meaning |
|---|---|---|
| `ready_for_import` | `ready_primary_catalog_baseline` or `ready_mirror_semantic_parity` | The row preserves source, artifact identity, publisher continuity, permissions, compatibility, lifecycle, and trust metadata without route-specific downgrades. |
| `ready_with_downgraded_trust_claims` | `limited_by_mirror_trust_downgrade` or `ready_manual_import_unverified` | The install lane can continue, but users and admins see exactly which trust claim was downgraded. |
| `awaiting_admin_review` | `awaiting_admin_out_of_band_verification` | A manual artifact needs an admin or mirror-operator verification receipt before install review. |
| `refused` | typed refused reason | Import cannot continue to install review, but support export still explains the block. |

## Fixture Drills

| Fixture | Expected result |
|---|---|
| `primary_catalog_baseline_ready.json` | primary catalog import with no trust downgrades |
| `approved_mirror_degraded_trust_claim_ready.json` | approved mirror import where only the revocation-snapshot freshness claim is downgraded |
| `manual_artifact_import_preserves_metadata.json` | manual archive import with preserved semantic metadata and capped publisher/signature trust |

## Headless Usage

Run the Rust fixture suite:

```text
cargo test -p aureline-extensions mirror_import::tests
```

Dump baseline and support-export records for schema validation:

```text
cargo run --example dump_mirror_import_baseline_records -p aureline-extensions
```

Validate checked artifact shape with the schema drift tool:

```text
python3 tools/check_schema_example_drift.py
```

## Checked Outputs

| Output | Purpose |
|---|---|
| `primary_catalog_baseline_record.json` | canonical primary catalog baseline |
| `approved_mirror_baseline_record.json` | canonical approved mirror baseline with a named trust downgrade |
| `manual_artifact_baseline_record.json` | canonical manual artifact baseline with capped trust claims |
| `support_export.json` | metadata-safe support/export projection for the approved mirror baseline |

The support export is the first consuming surface. It repeats only
metadata-safe refs and state classes, preserves source visibility, and offers
actions that map back to source details, publisher continuity, permission
manifest, compatibility report, lifecycle metadata, trust-claim details,
native install review, admin review, and packet export.
