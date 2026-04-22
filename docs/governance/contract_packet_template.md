# Surface contract packet template

<!--
Machine-readable instances conform to
schemas/governance/contract_packet.schema.json.

The shared inventory lives at
artifacts/governance/stable_surface_inventory.yaml. Each row in that
inventory is a concrete packet instance and may be cited as
artifacts/governance/stable_surface_inventory.yaml#<surface_id>.
-->

This template is the narrative companion to the machine-readable
surface-contract packet schema. It exists so stable-facing and future
stable-facing surfaces publish one inspectable packet shape before
implementation hardens accidental contracts.

Rules:

- Every surface in `stable` or `beta` maturity MUST have a named owner,
  versioning rule, reader/writer semantics, downgrade behavior,
  support-window posture, publication artifact refs, and
  compatibility-window source row before broad implementation
  proceeds.
- `experimental` and `internal` surfaces still land the same packet
  shape. They may point at `not_yet_seeded` artifacts, but they do not
  skip versioning, downgrade, or support posture.
- Compatibility, docs, migration, deprecation, and release work cite
  the same `surface_id`. They do not mint parallel ids for the same
  boundary.
- WIT worlds, task-event envelopes, service APIs, JSON Schemas, field
  registries, and mixed-version envelopes all use this packet; no
  bespoke side document is required just because the wire format
  differs.

## Packet

- **Packet kind:** `surface_contract_packet`
- **Schema version:** `1`
- **Surface id:** `<surface-id>`
- **Surface title:** `<short title>`
- **Summary:** one or two sentences naming what the surface governs.
- **Maturity lane:** `stable` | `beta` | `experimental` | `internal`
- **Contract form:** `<json_schema_backed_contract_doc | wit_world_package | openapi_family | record_registry | field_set | event_envelope_schema | ...>`

## Ownership

- **Owner DRI:** `@handle`
- **Owning lane:** lane id from `artifacts/governance/ownership_matrix.yaml`
- **Backup owner:** `@handle` or `null`
- **Backup waiver:** waiver id when backup owner is `null`

## Publication

- **Publication artifact refs:** repo-relative paths or stable row refs
  that publish the contract.
- **Schema or interface directory refs:** directory paths that contain
  the governing machine-readable surface. Use `not_yet_seeded` when
  the contract directory does not exist yet.

## Versioning

- **Versioning rule:** how additive versus breaking changes are
  recognized.
- **Compatibility-window source row:** row ref that defines the
  currently claimed window or skew rule.

## Reader / writer semantics

- **Canonical writers:** named producers that emit the surface.
- **Canonical readers:** named consumers that parse or project the
  surface.
- **Reader semantics:** unknown-field handling, projection honesty, and
  fail-closed rules.
- **Writer semantics:** explicit version emission, field preservation,
  and mutation discipline.
- **Downgrade behavior:** what happens when the reader or writer falls
  outside the declared window.

## Compatibility

- **Compatibility window summary:** current skew or compatibility
  promise in plain language.
- **Support window posture:** declared support class or time/range
  promise.
- **Compatibility report refs:** paths or row refs compatibility work
  cites.
- **Migration guidance refs:** paths or row refs migration and
  import/export work cites.

## Lifecycle

- **Dependency markers:** explicit dependencies on preview, labs,
  policy-gated, or external surfaces.
- **Deprecation posture:** replacement path, alias map, removal horizon
  or review checkpoint, and required docs/help visibility.

## Review

- **Review cadence:** `each_change` | `per_milestone` | `each_release`
  | `on_promotion`
- **Docs/help touchpoint refs:** docs, help, or review artifacts that
  surface the contract to humans.

## Notes

Short free-form notes for edge cases the machine fields do not carry.
