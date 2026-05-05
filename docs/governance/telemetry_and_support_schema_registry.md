# Telemetry, diagnostics, and support-export schema registry

This document is the human-readable companion to the telemetry,
crash/diagnostic, support-bundle, metering/usage-export,
offboarding-packet, and CLI/headless schema registry. It exists so
Aureline names each payload family — owner, version, consent class,
endpoint class, retention posture, build-flavor defaults,
compatibility horizon, downgrade rules, and UX-telemetry exclusions —
*before* shared plumbing (exporter, upload queue, redaction profile,
manifest writer, CLI stdout, offboarding assembler) starts hiding
policy differences between them.

Companion artifacts:

- [`/artifacts/governance/consent_ledger_seed.yaml`](../../artifacts/governance/consent_ledger_seed.yaml)
  — machine-readable registry and consent-ledger seed. Every seeded
  row conforms to the boundary schema below.
- [`/schemas/governance/schema_registry_entry.schema.json`](../../schemas/governance/schema_registry_entry.schema.json)
  — boundary schema for one `schema_registry_entry_row`.
- [`/docs/observability/signal_class_matrix.md`](../observability/signal_class_matrix.md)
  and
  [`/artifacts/observability/signal_classes.yaml`](../../artifacts/observability/signal_classes.yaml)
  — signal-class matrix used by privacy-history events, support-bundle
  previews, and schema-registry reviews to keep intent/risk categories
  consistent.
- [`./record_class_governance.md`](./record_class_governance.md) and
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — class-level retention, export, delete, hold, and offboarding
  posture. Registry rows inherit that posture via
  `schema_family_binding.record_class_id_refs`; they never redeclare
  retention semantics inline.
- [`../adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — redaction defaults every row inherits. The registry row names the
  payload family; ADR 0007 still fixes what any bytes look like when
  they cross a boundary.
- [`../support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — support-bundle packet family. Registry row
  `support.bundle_manifest` points here.
- [`../product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — product-boundary consumer of these rows. Managed claims that move
  bytes off device, retain evidence, or promise exit packets resolve
  to a registry row in the same change.

If this document and the boundary schema disagree, the schema wins and
this document must be updated in the same change.

## Why a separate telemetry/support schema registry

The record-class registry answers class-level retention, delete,
export, hold, and offboarding questions. What it does *not* answer,
and what surfaces keep re-inventing, is the **payload-family level**
contract:

- who owns the family;
- which schema file and schema version are authoritative;
- what consent class applies;
- which endpoint class (local-only, optional upload, export-only,
  managed-mirror, managed-authoritative, admin-broker-gated,
  deletion/offboarding-channel-only, or CLI-stdio-local-only) the
  family lives on;
- which retention note applies inside the record-class posture;
- which default posture each build flavor ships with;
- what compatibility horizon and downgrade rules bind readers and
  writers;
- which raw or sensitive UX payloads the family excludes by default;
  and
- which sibling families it must cross-reference without embedding.

Without one registry, telemetry, crash, diagnostics, support-bundle,
metering/usage-export, offboarding, CLI/headless, review parity,
voice/dictation privacy, and assurance/compliance schemas all invent
their own consent language. The registry closes that gap before any
of those families are implemented.

## Seeded payload families

The seed intentionally starts with the families most likely to drift
into product behavior first, plus the linkage row that forces sibling
families to cross-reference without embedding:

| Entry id | Family class | Consent class | Endpoint class | Build-flavor summary |
|---|---|---|---|---|
| `telemetry.ux_product_event` | `ux_product_telemetry` | explicit opt-in | local-authoritative, optional upload | opt-in on stable/lts/preview/beta/nightly; disabled on dev, hotfix, portable; admin-forced-disabled on managed |
| `telemetry.learning_onboarding_event` | `learning_onboarding_telemetry` | explicit opt-in | local-authoritative, optional upload | opt-in on stable/lts/preview/beta/nightly; disabled on dev/hotfix/portable; admin-forced-disabled on managed |
| `diagnostics.crash_payload` | `crash_diagnostic` | explicit opt-in | local-authoritative, optional upload | local capture always on; upload is opt-in; admin-policy-gated on managed |
| `diagnostics.repair_transaction_packet` | `repair_transaction_packet` | implied for local authoritative | local-authoritative, optional upload | local by default everywhere; admin-policy-gated on managed |
| `support.bundle_manifest` | `support_bundle_manifest` | export only on user request | export-only, user-initiated | user-initiated on every flavor; admin-policy-gated on managed |
| `support.bundle_redaction_profile` | `support_bundle_redaction_profile` | admin policy gated | export-only, user-initiated | always available; admin-forced-enabled on managed |
| `privacy.voice_dictation_event` | `voice_dictation_privacy_event` | explicit opt-in | local device only | off by default; opt-in on product flavors; admin-policy-gated on managed |
| `review.hosted_parity_record` | `hosted_review_parity_record` | admin policy gated | managed mirror when enabled | not available on dev/portable; opt-in on product flavors; admin-policy-gated on managed |
| `export.portability_record` | `portability_export_record` | export only on user request | export-only, user-initiated | user-initiated on every flavor; admin-forced-enabled on managed |
| `usage.metering_export_packet` | `metering_usage_export` | admin policy gated | managed-authoritative when enabled | not available outside stable/lts managed paths; admin-forced-enabled on managed |
| `offboarding.exit_packet` | `offboarding_exit_packet` | admin policy gated | deletion/offboarding export channel only | not available outside managed stable/lts; admin-forced-enabled on managed |
| `cli.headless_diagnostic_payload` | `cli_headless_diagnostic` | implied for local authoritative | CLI stdio local only | user-initiated on every flavor; admin-forced-enabled on managed |
| `assurance.compliance_packet` | `assurance_compliance_packet` | admin policy gated | admin-broker-gated | not available outside managed beta/stable/lts/hotfix; admin-forced-enabled on managed |
| `linkage.usage_export_offboarding` | `usage_export_offboarding_linkage` | not applicable (local mechanics) | local device only | governance contract; enabled on every flavor |

The seed is intentionally narrow. It names the families product work
is most likely to treat as "just another schema", even though they
have different consent, endpoint, retention, build-flavor, and
compatibility obligations.

## Row shape

Every `schema_registry_entry_row` keeps the governance axes separate:

- `family_class` names which of the fourteen seeded families this row
  is canonical for. Telemetry/usage payloads, crash/diagnostic
  payloads, support bundles, metering/usage exports, offboarding
  packets, and CLI/headless schemas stay separate rows even when they
  share transport or redaction plumbing.
- `owner_ref` names the governing team, role, or forum. A family
  without an owner is a governance bug.
- `schema_family_binding` pins the schema path, schema version, stable
  `$id`-style version URI, and the record-class ids (from
  `record_class_registry.yaml`) the family inherits. Retention,
  delete, hold, and offboarding posture come from those record
  classes, not from this row.
- `consent_class` names how consent is established. The closed set
  distinguishes local mechanics, implied-for-local-authoritative,
  explicit opt-in (default off), explicit opt-in (default on under
  review), admin policy gated, export-only-on-user-request,
  delete-governed-only, and deny-by-default.
- `endpoint_class` names where the payload may travel. The closed set
  distinguishes local-only, local-authoritative with optional upload,
  export-only user-initiated, managed mirror when enabled,
  managed-authoritative when enabled, admin-broker-gated,
  deletion-or-offboarding-export-channel-only, and CLI stdio
  local-only.
- `retention_note` is a short reviewable sentence naming the
  retention window and what closes it. The underlying record class
  fixes the full posture; this field is a human-readable inheritance
  note.
- `build_flavor_default_posture` names the default posture for every
  build flavor the product ships under (dev_local, nightly, preview,
  beta, stable, lts, hotfix, portable_stable, portable_preview,
  managed_enterprise). Every row lists every flavor so a reviewer
  can spot posture drift at a glance. The closed posture set
  distinguishes `not_available_on_flavor`, `disabled_by_default`,
  `opt_in_only`, `opt_in_required_each_session`,
  `enabled_metadata_only`, `enabled_by_default`,
  `admin_policy_gated`, `admin_forced_enabled`,
  `admin_forced_disabled`, `preview_gated`, and `export_only`.
- `compatibility_horizon` names `min_readable_version`,
  `min_writable_version`, a deprecation-window note, and a
  sunset-window note.
- `downgrade_rules` list the typed actions a reader or writer must
  take when it encounters a schema_version outside the active
  horizon: drop-on-read, preserve-as-unknown, refuse-read,
  refuse-export, refuse-upload, degrade-to-local-only, or
  require-manual-review. Every row lists at least one rule.
- `default_ux_payload_exclusions` name the raw or sensitive payloads
  the family excludes by default: raw code bodies, AI prompts and
  responses, repo names, file paths, directory names, branch names,
  clipboard contents, full search queries, free-text input, terminal
  command payloads, raw environment variables, raw hostnames, raw
  user identifiers, voice audio, voice transcripts, notebook cell
  contents, and secret/token material. Every exclusion carries an
  `override_policy` in `{never_permitted,
  separately_reviewed_opt_in_required,
  separately_reviewed_opt_out_receipt_required,
  redacted_placeholder_only}`. Anything other than `never_permitted`
  requires at least one `permitted_by_separate_review_refs` entry.
- `separation_rule` names the canonical family, the families this row
  must never be conflated with even when plumbing is shared, and a
  short shared-plumbing note.
- `sibling_family_links` name the sibling entries this family must
  reference (or must be kept from embedding). Linkage classes include
  `references_sibling_in_manifest`,
  `cited_by_sibling_offboarding_packet`,
  `cited_by_sibling_support_bundle`, `receipt_for_sibling_action`,
  `mutual_cross_reference_required`, and
  `sibling_must_never_embed_this`.
- `local_only_lane_posture` and `local_only_lane_note` let a reviewer
  spot any family whose default posture would become coercive or
  ambiguous on a local-only lane. The closed set distinguishes
  `local_only_safe_default`, `local_only_requires_explicit_opt_in`,
  `local_only_not_available_requires_managed`,
  `local_only_export_only_when_user_initiates`, and
  `local_only_ambiguous_requires_review`. Anything other than the
  safe default must carry a justification sentence.
- `governance_links` carry redaction, retention, source, boundary,
  supportability, and related-schema references so a reviewer can
  trace posture to its authoritative artifacts.

These objects are deliberately not merged. Consent, endpoint,
retention, build-flavor posture, compatibility horizon, downgrade,
exclusion, separation, sibling linkage, and local-only-lane posture
are distinct axes, and the registry keeps them distinct.

## Separation rule for support manifests vs analytics, usage, and offboarding

Several families share transport code, redaction profiles, or upload
queues. The registry keeps them separate:

- **Support bundle manifest** (`support.bundle_manifest`) remains its
  own family even though it may embed or reference crash payloads,
  repair packets, portability records, and metering exports. The
  bundle is never an analytics event, never a billing snapshot, and
  never the offboarding exit packet.
- **Analytics/usage telemetry** (`telemetry.ux_product_event`,
  `telemetry.learning_onboarding_event`) remains separate from
  support bundles even if upload plumbing is shared. Telemetry
  posture changes are reviewed against telemetry exclusions, not
  bundle profiles.
- **Metering usage export** (`usage.metering_export_packet`) remains
  separate from portability and offboarding even though all three
  are user-visible packets. Billing posture is reviewed against
  `entitlement_usage_export_packet` retention; portability is
  reviewed as a user right; offboarding is reviewed as an access-end
  promise.
- **Offboarding exit packet** (`offboarding.exit_packet`) cites
  metering, portability, support-bundle, and assurance rows by
  reference and is never the thing any of those rows produce.

When two families must reference each other, they do so via
`sibling_family_links` with `mutual_cross_reference_required`. The
dedicated `linkage.usage_export_offboarding` row exists to freeze the
metering/offboarding cross-reference contract so neither family
silently embeds the other.

## Default UX-telemetry exclusions

Every registry row lists the default exclusion posture for raw code,
AI prompts and responses, repo names, file paths, directory names,
branch names, clipboard contents, full search queries, free-text
input, terminal command payloads, raw environment variables, raw
hostnames, raw user identifiers, voice audio, voice transcripts,
notebook cell contents, and secret/token material. A row may widen
capture for an exclusion only when it cites a separately reviewed
schema, ADR, or waiver in `permitted_by_separate_review_refs` under
`override_policy: separately_reviewed_opt_in_required` or
`separately_reviewed_opt_out_receipt_required`.

The registry deliberately forbids an ambient "telemetry permits
everything" posture. Widening is per-exclusion and per-row, and it
lands on the same boundary review as the schema the family ships.

## How other lanes use this registry

- **Boundary and privacy reviews** read registry rows to audit
  consent, endpoint, build-flavor posture, and local-only-lane
  posture side by side. A proposed payload family that cannot be
  placed in the registry without ambiguity is a review finding.
- **Support-bundle assembly** quotes `support.bundle_manifest` and
  the cited redaction profile row when it decides which crash,
  repair, CLI, or review-parity rows to embed, reference, or omit.
- **Delete, export, and offboarding flows** quote registry rows to
  distinguish what is local-only, optional upload, export-only,
  managed-mirror, managed-authoritative, admin-broker-gated,
  or deletion/offboarding-export-channel-only, without inventing
  parallel privacy vocabularies.
- **CLI/headless surfaces** quote `cli.headless_diagnostic_payload`
  when they emit diagnostic output so the CLI is never repurposed as
  an analytics pipe behind the user's back.
- **Managed/enterprise lanes** read the `managed_enterprise`
  build-flavor posture row to confirm an admin posture before
  enabling a family.
- **AI, voice, review, compliance, and billing lanes** each consult
  the family row that governs them (`ai_retained_evidence_packet` in
  the record-class registry plus the related schema rows here,
  `privacy.voice_dictation_event`, `review.hosted_parity_record`,
  `assurance.compliance_packet`, and `usage.metering_export_packet`)
  rather than inventing private consent classes.

## Change discipline

Adding, changing, or retiring a payload family requires all of the
following in the same change:

1. Add or update the row in
   [`consent_ledger_seed.yaml`](../../artifacts/governance/consent_ledger_seed.yaml).
2. If the change introduces new vocabulary, extend
   [`schema_registry_entry.schema.json`](../../schemas/governance/schema_registry_entry.schema.json)
   (additive-minor) and bump `schema_registry_schema_version` when
   the addition is breaking.
3. Cite at least one redaction artifact and one retention artifact in
   `governance_links`. ADR 0007 and the governed-record state model
   are the floor; family-specific artifacts are added when they
   exist.
4. Ensure `schema_family_binding.record_class_id_refs` points at the
   record-class rows whose retention, hold, delete, export, and
   offboarding posture the family inherits. If no existing class
   applies, land the record-class row in the same change.
5. If the change touches a sibling family (support bundle manifest,
   portability record, metering export, offboarding exit packet,
   or assurance packet), update both `separation_rule` and
   `sibling_family_links` on both sides so cross-references stay
   balanced.
6. If the change widens a `default_ux_payload_exclusions` entry,
   cite the separately reviewed schema, ADR, or waiver in
   `permitted_by_separate_review_refs`. A widening without that
   citation is a review finding.
7. If the change alters `local_only_lane_posture` or any
   `build_flavor_default_posture` entry, update the
   `local_only_lane_note` or per-flavor `note` so the posture stays
   reviewable.

## Versioning rules

- Adding a new row is additive.
- Adding a new enum value to the schema (family class, consent class,
  endpoint class, build flavor, build-flavor posture, downgrade
  action, telemetry-exclusion class, exclusion-override policy,
  sibling-linkage class, local-only-lane posture) is additive-minor
  and requires a `schema_registry_schema_version` bump plus a doc
  update here.
- Repurposing an existing enum value, changing the meaning of a
  field, or reusing an `entry_id` for a meaningfully different family
  is breaking and requires a new decision row plus a superseding
  registry row.

## What this document is not

- It is **not** a telemetry backend design. Running a telemetry
  pipeline, storing events, billing usage, or auto-uploading crash
  dumps is out of scope. The registry fixes payload-family policy
  before any of that is implemented.
- It is **not** the support-bundle contract. That remains
  [`../support/support_bundle_contract.md`](../support/support_bundle_contract.md).
  The registry quotes it.
- It is **not** a substitute for the record-class registry. That
  remains
  [`./record_class_governance.md`](./record_class_governance.md) and
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml).
  The schema-registry binds into that registry; it does not replace
  it.
- It is **not** the secret-broker ADR. That remains
  [`../adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md).
  Redaction defaults come from the ADR; this registry names the
  families the defaults apply to.
