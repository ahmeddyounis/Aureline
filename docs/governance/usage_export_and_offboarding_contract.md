# Usage export and offboarding compatibility contract

This contract freezes the packet-level schema and posture vocabulary
for usage export and offboarding exit surfaces. It exists so:

- customer-visible usage export stays **honest and scriptable** (scope,
  time basis, quota family, authority class, and caveats are explicit);
- offboarding and account-exit flows can **reference** usage export
  packets without embedding them, so schemas do not collapse into one
  coercive “billing + exit” blob; and
- build-flavor and deployment-profile defaults remain reviewable so a
  user or admin can tell whether a given lane uploads, stores, exports,
  or omits usage records.

This contract composes over (and does not replace) the managed metering
row contract. Where this document disagrees with the normative product
spec sources, the source specs win and this document and the companion
schemas update in the same change.

## Companion artifacts

- [`/schemas/governance/usage_export_record.schema.json`](../../schemas/governance/usage_export_record.schema.json)
  — boundary schema for one `usage_export_packet_record`.
- [`/schemas/governance/offboarding_exit_packet.schema.json`](../../schemas/governance/offboarding_exit_packet.schema.json)
  — boundary schema for one `offboarding_exit_packet_record`.
- [`/artifacts/governance/usage_export_posture_matrix.yaml`](../../artifacts/governance/usage_export_posture_matrix.yaml)
  — default-posture matrix by build flavor and deployment profile.
- [`/fixtures/governance/usage_export_cases/`](../../fixtures/governance/usage_export_cases/)
  — worked cases covering managed export, entitlement-loss review,
  renewal window, local-only absence, and narrowed downgrade posture.
- [`/fixtures/governance/offboarding_exit_packet_cases/`](../../fixtures/governance/offboarding_exit_packet_cases/)
  — worked cases showing exit packets referencing usage export packets by
  opaque id (no embedding) with an explicit access-end window.

## Inherited contracts and registries

This contract is a packet-level narrowing layer. It reuses the
vocabulary and obligations from:

- [`/docs/managed/metering_and_usage_export_contract.md`](../managed/metering_and_usage_export_contract.md)
  and [`/schemas/managed/usage_export_row.schema.json`](../../schemas/managed/usage_export_row.schema.json)
  — row-level truth (scope, time basis, quota family, authority/source,
  caveats, suppression) and the rule that metering is not a billing
  engine.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](./telemetry_and_support_schema_registry.md)
  and [`/artifacts/governance/consent_ledger_seed.yaml`](../../artifacts/governance/consent_ledger_seed.yaml)
  — consent class, endpoint class, and build-flavor default posture for
  the usage-export and offboarding families.
- [`/docs/governance/record_class_governance.md`](./record_class_governance.md)
  and [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — retention/hold/delete/export posture inheritance for
  `entitlement_usage_export_packet` and `offboarding_exit_packet`.
- [`/docs/governance/data_portability_and_exit_matrix.md`](./data_portability_and_exit_matrix.md)
  — the per-domain exit floor that offboarding packets cite.
- [`/docs/managed/account_seat_plan_and_exit_contract.md`](../managed/account_seat_plan_and_exit_contract.md)
  and [`/schemas/managed/account_exit_packet.schema.json`](../../schemas/managed/account_exit_packet.schema.json)
  — entitlement-loss and access-end posture that exit surfaces cite
  instead of inventing “access revoked” copy.
- [`/artifacts/managed/local_baseline_proof.yaml`](../../artifacts/managed/local_baseline_proof.yaml)
  — surviving local-only baseline language (local edit/save/search/Git
  and read of already exported local artifacts remain admissible).

## Scope

Frozen at this revision:

- the `usage_export_packet_record` envelope and its customer-visible
  schema identity fields;
- the explicit `availability_class` that distinguishes present rows,
  local-only “absent” posture, and policy withholding;
- the requirement that packets and exit surfaces reference sibling
  families via opaque ids and the linkage contract, not by embedding;
- the build-flavor × deployment-profile default posture matrix.

Out of scope:

- a billing engine, pricing model, taxation, invoicing, or rating
  ledger;
- vendor invoice payloads, payment processor records, or raw provider
  account identifiers; and
- managed control-plane implementation details.

## Core rules

1. **Usage export is not hidden billing logic.** It is a customer-visible
   export contract with explicit scope, time basis, quota family, and
   authority/caveat disclosure.
2. **Absence is explicit.** When usage export is not applicable on a
   lane, the posture is represented as `availability_class =
   absent_no_applicable_metering` rather than by implying “zero usage”.
3. **Narrowing is non-coercive.** Losing a managed entitlement may
   narrow fields (policy suppression) or narrow freshness/authority
   claims, but it must not coerce account retention by hiding the last
   contractually promised export.
4. **Offboarding references, it does not embed.** Offboarding exit
   packets cite usage export packet ids, portability export ids, and
   destruction receipt ids. They do not embed raw sibling payloads.
5. **Default posture is reviewable.** The posture matrix must be
   sufficient for a user or admin to determine whether a lane uploads,
   stores, exports, or omits usage records.

## Linkage requirements

Every usage-export packet and offboarding exit packet MUST link to:

- `artifacts/governance/consent_ledger_seed.yaml#usage.metering_export_packet`
  and `#offboarding.exit_packet` (default posture and consent);
- `artifacts/governance/consent_ledger_seed.yaml#linkage.usage_export_offboarding`
  (the “reference without embedding” rule); and
- the record-class registry rows for retention/export posture.

The packet schemas enforce these link refs via constant fields under
`links`.

## Default posture matrix

The posture matrix is the machine-readable source for “does this lane
upload, store, export, or omit usage exports?” It binds each build
flavor and deployment profile to:

- availability (present vs absent);
- storage locus (local/export-only vs customer-operated control plane vs
  vendor-operated managed plane); and
- customer visibility and offboarding inclusion posture.

The matrix must include local-only and non-vendor-hosted rows so
reviewers can challenge coercive defaults early.

## Fixtures (worked cases)

The fixture corpus demonstrates:

- **Managed lane export:** a present packet with explicit scope/time
  basis, quota families, and authoritative rows.
- **Entitlement-loss review:** a present packet that narrows fields
  (policy-suppressed) instead of disappearing.
- **Renewal window:** a packet whose offboarding availability window is
  explicit (available until a typed cutover timestamp or `null` when
  not bounded).
- **Local-only surviving baseline:** an explicit “absent” packet for a
  local-only lane, with a reviewable summary that the local baseline
  remains intact.
- **Downgrade narrowing:** a packet whose visibility narrows by policy
  suppression rather than by forcing account retention.

## Evolution rules

- Adding a new optional field or a new additive enum value is
  additive-minor and bumps the packet schema const version(s) plus a
  fixture refresh.
- Repurposing an existing enum value or turning an “absent” posture
  into a silent omission is breaking and requires a decision row plus a
  migration note for offboarding and support consumers.
