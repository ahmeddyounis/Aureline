# Stable telemetry, support-export, and usage-export schema registry

This document is the human-readable companion to
[`artifacts/governance/telemetry_support_usage_schema_registry.json`](../../artifacts/governance/telemetry_support_usage_schema_registry.json),
which stabilizes the telemetry, diagnostics, support-export, and
usage-export payload families as first-class product contracts.

It differs from the consent-ledger seed companion
([`./telemetry_and_support_schema_registry.md`](./telemetry_and_support_schema_registry.md))
in that it adds the governance dimensions needed before any stable
managed or support-sensitive claim may depend on these payloads:

- **Endpoint-policy truth per deployment context** — for each of the
  three deployment contexts (`oss_local`, `self_hosted`,
  `managed_enterprise`), each row declares whether its data stays
  local, is queued for manual export, flows to a managed endpoint, or
  is disabled by policy or build flavor.
- **OSS telemetry posture enforcement** — telemetry-family rows must
  carry `opt_in_disabled_until_user_consent` as their OSS default, or
  reference a signed exception packet that explicitly authorizes a
  different posture.
- **Redaction profile references** — every row cites the redaction ADR
  that governs what bytes the family excludes at boundary crossings.
- **Deprecated-field handling policy** — the closed policy for what
  readers must do when they encounter a field that has been deprecated
  or is from an unknown schema version.
- **Partial-outcome markers** — whether packets in the family can be
  partial due to policy suppression, offboarding-window bounds, or
  manual reconciliation.
- **Offboarding compatibility notes** — reviewable sentences stating
  what each family does at account exit, so offboarding, portability,
  and deletion flows can cite truth rather than invent it.
- **Schema diff report references** — before stable promotion whenever
  a schema version advances, a diff report artifact must be cited here.

If this document and the JSON artifact disagree, the JSON artifact wins
and this document must be updated in the same change.

## Registry rows

The registry covers the six stable-emitted payload families from the
governed schema registry:

| Entry id | Family class | Consent class | Endpoint class | OSS default |
|---|---|---|---|---|
| `telemetry.ux_product_event` | `telemetry_payload` | explicit opt-in | local + optional upload | `opt_in_disabled_until_user_consent` |
| `diagnostics.crash_payload` | `diagnostic_payload` | explicit opt-in | local + optional upload | `local_capture_no_upload_by_default` |
| `support.bundle_manifest` | `support_export_payload` | export-only on user request | export-only user-initiated | `user_initiated_export_only` |
| `usage.metering_export_packet` | `usage_export_payload` | admin policy gated | managed-authoritative | `not_available_without_managed_policy` |
| `offboarding.exit_packet` | `offboarding_packet` | admin policy gated | deletion/offboarding channel | `not_available_without_managed_policy` |
| `cli.headless_diagnostic_payload` | `cli_headless_packet` | implied local | CLI stdio local only | `local_stdio_only` |

## Endpoint-policy truth matrix

The endpoint-policy truth answers the most common release and shiproom
question: "for a user in context X, does this payload leave their
device?" The answers are fixed per family and context:

| Entry id | OSS/local | Self-hosted | Managed enterprise |
|---|---|---|---|
| `telemetry.ux_product_event` | `local_only` | `local_only` | `disabled_by_policy_or_flavor` |
| `diagnostics.crash_payload` | `local_only` | `local_only` | `local_only` |
| `support.bundle_manifest` | `queued_for_manual_export` | `queued_for_manual_export` | `queued_for_manual_export` |
| `usage.metering_export_packet` | `disabled_by_policy_or_flavor` | `disabled_by_policy_or_flavor` | `managed_endpoint` |
| `offboarding.exit_packet` | `disabled_by_policy_or_flavor` | `disabled_by_policy_or_flavor` | `queued_for_manual_export` |
| `cli.headless_diagnostic_payload` | `local_only` | `local_only` | `local_only` |

`local_only` means the data never leaves the device by default.
`queued_for_manual_export` means the data is held locally until an
explicit user or admin action releases it. `managed_endpoint` means
the data flows to the vendor-operated managed plane under admin policy.
`disabled_by_policy_or_flavor` means the family is not active on that
lane and produces nothing.

## OSS telemetry posture

Open-source desktop builds treat telemetry as opt-in by default. The
registry enforces this by requiring that every `telemetry_payload`
family carry `opt_in_disabled_until_user_consent` as its
`open_source_default_posture`, or cite a narrower signed exception
packet in `oss_telemetry_exception_packet_ref`.

No exception packet currently overrides this posture for any family.
Any proposed override must be reviewed as a boundary change and
produce a signed exception artifact before the registry row is updated.

## Separation contract

Support-bundle manifests, analytics/usage payloads, and
customer-visible usage-export schemas are distinct contracts. Even
when they share transport code, redaction profiles, or upload queues:

- **Telemetry** (`telemetry.ux_product_event`) is never a support
  manifest, usage export, or offboarding packet.
- **Support-bundle manifest** (`support.bundle_manifest`) is a
  user-initiated export artifact, not an analytics event or billing
  record.
- **Usage export** (`usage.metering_export_packet`) is a
  customer-visible managed record, not telemetry or an exit packet.
- **Offboarding exit packet** (`offboarding.exit_packet`) cites
  usage exports, portability records, and destruction receipts by
  opaque reference; it never embeds them.

These separation rules are enforced by the base schema registry
(`schemas/registry/schema_registry.json`) `separation` fields; this
registry inherits and requires them.

## Promotion gates

A payload family may not be promoted to stable unless the registry row:

1. Has a non-empty `owner_ref`, `schema_version`, `consent_class`,
   `endpoint_class`, `retention_note`, `redaction_profile_ref`, and
   `offboarding_compatibility_note`.
2. Declares `endpoint_policy_truth_by_context` for all three required
   contexts: `oss_local`, `self_hosted`, and `managed_enterprise`.
3. For `telemetry_payload` families: carries
   `opt_in_disabled_until_user_consent` or a non-null
   `oss_telemetry_exception_packet_ref`.
4. When `schema_version` has advanced from a prior stable: carries a
   non-null `schema_diff_report_ref`.

The typed Rust validator in
`crates/aureline-governance/src/telemetry_support_usage_registry/` returns
a `Vec<RegistryViolation>` that CI and shiproom gates must treat as empty
before allowing a stable promotion claim.

## Redaction profile

Every row cites
[`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
as its redaction profile. This is the floor; family-specific
exclusions defined in the consent-ledger seed companion supplement it.
Adding a family-specific widening (e.g. permitting raw hostnames in a
diagnostic payload) requires a separate ADR or waiver, and the
`redaction_profile_ref` must then point to that artifact rather than
only to ADR 0007.

## Deprecated-field handling

All six rows use `preserve_as_unknown` as the deprecated-field
handling policy. This is the safe default: readers that encounter a
deprecated field preserve its value under an `unknown` marker rather
than dropping it or refusing the read. It is consistent with the
`deprecated_version_policy: deprecated_with_warning` downgrade rule
in the base schema registry.

Changing this policy to `refuse_read` requires a decision row because
it changes the observable behavior of existing consumers.

## Partial-outcome markers

| Entry id | Marker | Meaning |
|---|---|---|
| `telemetry.ux_product_event` | `none` | Telemetry events are atomic; no partial path exists |
| `diagnostics.crash_payload` | `partial_suppressed_by_policy` | Fields may be redacted under policy |
| `support.bundle_manifest` | `partial_suppressed_by_policy` | Bundles explicitly redact by profile |
| `usage.metering_export_packet` | `partial_offboarding_window_bounded` | Completeness bounded by entitlement retention window |
| `offboarding.exit_packet` | `partial_offboarding_window_bounded` | Completeness depends on what sibling families are still available |
| `cli.headless_diagnostic_payload` | `none` | CLI stdio output is always complete |

## Fixtures

Worked cases are in
[`fixtures/governance/telemetry_support_usage_schema_registry_cases/`](../../fixtures/governance/telemetry_support_usage_schema_registry_cases/).
They cover:

- OSS telemetry staying local by default.
- Support export queued for manual export in all contexts.
- Usage export active only in managed-enterprise context.
- Diagnostics local-only across all contexts.
- Promotion gate blocking an unlabeled row.

## Change discipline

Changes to this registry follow the same rules as the base governed
schema registry. In addition:

- Any change to `endpoint_policy_truth_by_context` requires an update
  to the endpoint-policy truth matrix table in this document.
- Any widening of OSS telemetry posture requires a signed exception
  packet artifact and a citation in `oss_telemetry_exception_packet_ref`.
- Any schema version advance requires a `schema_diff_report_ref` before
  stable promotion.
- Changing `deprecated_field_handling` from `preserve_as_unknown` to
  any other value requires a decision row.
