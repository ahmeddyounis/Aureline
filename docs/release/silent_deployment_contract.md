# Silent deployment contract

This contract freezes the unattended install, update, pin, rollback, and
uninstall result model so enterprise deployment, release engineering, and
support can reason about outcomes **without** reading installer code or
reverse-engineering platform-specific exit-code folklore.

The goal is simple: every silent deployment resolves into a **machine-readable
result record** with:

- a stable return-code family and numeric exit code;
- a closed failure reason and remediation pointer (or explicit `none`);
- stable references back to the install-topology card and state-root rows that
  determine what is safe to touch; and
- reserved linkage slots for later support/export surfaces (support bundles,
  rollback evidence, offline-bundle signature refs, and managed-package
  inventory).

This is a contract layer, not an installer implementation.

## Companion artifacts

- [`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml)
  - authoritative vocabulary for result kinds/statuses, return-code families and
    numeric codes, failure-reason classes, remediation-pointer classes, and the
    base `unattended_deployment_result_record` shape.
- [`/schemas/release/silent_deployment_result.schema.json`](../../schemas/release/silent_deployment_result.schema.json)
  - boundary schema for result records and multi-record packets used by support
    and fleet export.
- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  - install-profile card ids and the per-row policy, rollback, diagnostics, and
    managed-package-report slots that silent deployment results must reference.
- [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  - stable state-root ids and collision rules used for state-root audit linkage.
- [`/artifacts/release/managed_package_report_seed.yaml`](../../artifacts/release/managed_package_report_seed.yaml)
  - managed-package inventory report shape for fleet/enterprise rows where the
    install-topology matrix declares the report slot `available` or `reserved`.
- [`/fixtures/release/silent_deployment_cases/`](../../fixtures/release/silent_deployment_cases/)
  - worked cases that join unattended result records to install-topology cards,
    state-root ids, mirror/offline posture, and uninstall preservation.

If this document and the machine-readable companions disagree, the YAML and
JSON Schema are the tooling source and this document must be corrected in the
same change.

## Scope

Frozen by this contract:

- Return-code family ids and numeric exit codes for unattended operations.
- The `unattended_deployment_result_record` shape emitted by silent install,
  update, pin, rollback, uninstall, and verify operations.
- A packet wrapper (`unattended_deployment_result_packet`) used for support and
  fleet export when multiple result records must travel together.

Out of scope:

- Shipping enterprise deployment tooling, MDM templates, or package-manager
  adapters.
- Defining every platform-specific installer log format.

## Record kinds

### `unattended_deployment_result_record`

Every silent operation emits one `unattended_deployment_result_record`.

Minimum required fields:

- `silent_deployment_result_schema_version` — schema version for the
  boundary contract (currently `1`).
- `record_kind` — the constant `unattended_deployment_result_record`.
- `result_id` — stable id for correlation across logs, support bundles, and
  fleet inventory.
- `result_kind` — `install`, `update`, `pin`, `rollback`, `uninstall`, `verify`.
- `result_status` — `success`, `partial_success`, `failed`, `rolled_back`,
  `verify_failed`, `reboot_required`.
- `return_code_family` and `return_code_numeric` — stable family and numeric
  code from the silent-deployment seed.
- `install_profile_card_ref` — install-topology card id the outcome applies to.
- `failure_reason_class` — closed failure reason (or explicit `null` on success).
- `remediation_pointer_class` — closed remediation pointer (or explicit `none`).
- `redaction_class` — redaction / secret posture consistent with
  [`docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md).

Linkage fields:

- `state_root_refs` — required for state-root collisions, portable spill, or
  side-by-side marker corruption; references stable state-root ids from
  `state_root_map.yaml`.
- `managed_package_report_ref` — present for managed/fleet rows when a managed
  package report slot exists; references a report record id from
  `managed_package_report_seed.yaml` or a later produced report artifact.

Reserved linkage slots (present as `null` or a stable ref):

- `running_build_identity_ref`
- `policy_injection_context_ref`
- `rollback_evidence_ref`
- `fleet_ring_context_ref`
- `offline_bundle_signature_ref`
- `support_bundle_ref`

Silent deployment results MUST NOT rely on free-text-only failure strings as
the primary machine contract. Human-facing prose is allowed, but it is not the
automation boundary.

### `unattended_deployment_result_packet`

When a scenario produces multiple result records (for example an update that
returns `rollback_required` followed by a rollback attempt, or a side-by-side
audit that records both Stable and Preview rows), the records travel together
in an `unattended_deployment_result_packet` so support and fleet tooling can
correlate the full chain without log scraping.

Packets are used in worked fixtures and later support/export surfaces. They are
not a new vocabulary: they only wrap `unattended_deployment_result_record`
entries plus stable references to the rows those records depend on.

Packets also carry `silent_deployment_result_schema_version` and a constant
`record_kind: unattended_deployment_result_packet` so downstream tooling can
discriminate packet payloads without relying on file naming conventions.

## Return-code contract

Return-code families are **channel-agnostic**; the channel travels on
`install_profile_card_ref`, not on the return code.

The authoritative mapping of:

- `return_code_family -> return_code_numeric`
- admissible `(result_kind, result_status)` sets per family

is in [`silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml).

Numeric exit codes align with the stable exit-code model declared in
`.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix B.2. Silent
deployment reserves additional codes for rollback-required, verification-failed,
and admin-required outcomes so callers do not have to parse stderr.

## Linkage requirements

Silent deployment is only supportable when result records are joinable to the
same truth sources every other surface uses:

- `install_profile_card_ref` MUST reference a card id in
  `install_topology_matrix.yaml`.
- State-root-related failures (`state_root_collision`,
  `side_by_side_marker_corruption`, `portable_spill_detected`) MUST carry
  `state_root_refs` pointing at the stable ids from `state_root_map.yaml`.
- Managed/fleet rows SHOULD emit `managed_package_report_ref` so enterprise
  inventory can correlate silent deployment outcomes to fleet package state.
- When a support bundle is captured for a failure, the record MUST set
  `support_bundle_ref` and set `redaction_class` appropriately.

## Change control

- Adding a new failure-reason class, remediation-pointer class, or return-code
  family is additive-minor: bump the schema version in the seed/schema and add
  at least one worked fixture demonstrating the new value.
- Repurposing an existing family, numeric code, or reason/pointer value is
  breaking: open a release-governance decision row before any installer work
  depends on the changed meaning.
