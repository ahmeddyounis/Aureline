# Record-class governance

This document is the human-readable companion to the record-class
registry seed. It exists so Aureline names class-level retention,
export, delete, hold, and offboarding behavior *before* product,
support, policy, and schema-registry lanes hard-code those behaviors
under private labels.

Companion artifacts:

- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — machine-readable registry seed. Every seeded row conforms to the
  row schema below.
- [`/schemas/governance/record_class.schema.json`](../../schemas/governance/record_class.schema.json)
  — boundary schema for one `record_class_row`.
- [`./record_state_and_policy_simulation_models.md`](./record_state_and_policy_simulation_models.md)
  — per-record state machine and chronology model. The registry in this
  document is class-level posture; the state model is the per-record
  truth each instance moves through.
- [`../support/support_center_concept.md`](../support/support_center_concept.md)
  — supportability consumer of these rows. Support-bundle preview and
  issue handoff quote record classes rather than inventing their own
  retention labels.
- [`../product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — product-boundary consumer of these rows. Managed claims that move
  bytes off device, retain evidence, or promise exit packets should
  resolve to a record-class row in the same change.

## Why this registry exists

The governed-record state model already answers questions like "is this
record held?" and "is this delete request complete yet?" What it does
*not* answer is the class-level question surfaces keep re-inventing:

- is this class local-only, or does it have a managed copy;
- is the thing the user is looking at a live record, an export packet,
  or a destruction receipt;
- who owns retention locally versus in a managed copy;
- what kind of export manifest or redaction artifact constrains the
  class; and
- what an offboarding flow owes the user for this class.

Without one registry, support, policy, schema-registry exports, and
managed-boundary copy will each invent a private "retention class" or
"export safe" chip. The goal here is one canonical row shape that keeps
local copies, managed copies, export packets, receipts, holds, delete
completion, and offboarding roles separate.

## Seeded classes

The seed registry intentionally starts with the classes most likely to
drift into product behavior first:

| Record class | Local vs managed posture | Export / offboarding posture | Main consumers |
|---|---|---|---|
| `telemetry_contract_schema` | local authoritative descriptor, optional managed mirror | exportable schema/taxonomy row; cited by usage/offboarding packets | schema registry, telemetry inspection, support export |
| `crash_diagnostic_payload` | local authoritative payload, optional managed escalation copy | exportable on request; may require manual local capture | Doctor, support bundle, case export |
| `support_bundle_archive` | local authoritative bundle, optional managed case copy | the bundle is itself an export packet | Support Center, escalation, parity audit |
| `collaboration_evidence_packet` | managed archive authoritative, local replay/cache separate | exportable evidence packet, offboarding-visible | collaboration review, access export |
| `ai_retained_evidence_packet` | local and managed copies distinct, managed retention when enabled | the evidence packet is the export shape | review, support replay, AI audit |
| `entitlement_usage_export_packet` | managed aggregates projected into a user-visible packet | export packet required for access/export and entitlement review | identity/admin, billing, offboarding |
| `offboarding_exit_packet` | generated packet assembled for access end | exit packet itself; must stay open-format | boundary/offboarding flows |
| `destruction_receipt_record` | managed receipt store authoritative, local copy optional | receipt emitted by delete/redaction action; cited by exit or delete cases | delete honesty, admin, support |

The seed is intentionally narrow. It names the first classes later work
is most likely to treat as "just another export" even though they have
different delete, hold, and offboarding semantics.

## Row shape

Every `record_class_row` keeps the lifecycle axes separate:

- `scope_posture` answers where the class is authoritative, whether a
  local materialization exists, whether managed copies are forbidden,
  optional, required, or authoritative, and whether the class is merely
  exportable versus being the export packet or receipt itself.
- `retention_posture` names the local and managed retention owners
  separately plus the default retention trigger.
- `hold_posture` says whether the class is hold-eligible and which hold
  classes are admissible.
- `delete_posture` keeps delete-request support, local-versus-managed
  delete distinction, hold blocking, and completion evidence separate.
- `export_posture` names export availability, default formats, and
  whether a manifest is required before a surface can call the export
  complete.
- `offboarding_posture` names whether the class is inventory-only,
  included as an export, the exit packet itself, or merely a receipt
  reference inside the exit packet.
- `partial_result_cause_set` freezes the caveats a surface must
  disclose instead of silently reporting success.

These objects are deliberately not merged into one `lifecycle` field.
Delete, export, held, offboarded, and retained are distinct states or
actions, and the registry keeps them distinct.

## How other lanes use it

- **Policy and explainability** read this registry together with the
  governed-record state model. The registry answers "what kind of record
  is this and what default delete/export/offboarding behavior applies";
  the state model answers "what state is this instance in right now".
- **Supportability** uses the row to group bundle contents and issue
  handoff packets by record class, and to disclose whether an item is
  local-only, a managed reference, an export packet, or a receipt.
- **Boundary reviews** consult the row when a product capability claims
  managed retention, telemetry export, support evidence, AI retention,
  usage export, or exit-package behavior.
- **Schema-registry and generated-reference work** quote `record_class_id`
  when rendering or exporting class-bearing schemas, instead of inventing
  a second "retention class" vocabulary in generated docs.

## Change discipline

Adding or changing a record class requires all of the following in the
same change:

1. Add or update the row in
   [`record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml).
2. If the change introduces new vocabulary, extend
   [`record_class.schema.json`](../../schemas/governance/record_class.schema.json).
3. Link the row to at least one redaction artifact and one retention
   artifact. ADR 0007 and the governed-record state model are the floor;
   row-specific artifacts should be added when they exist.
4. Update the consumer docs when a lane begins depending on the row:
   supportability, boundary-manifest, policy/state, and
   schema-registry-adjacent docs must stay linked in the same change.
5. If the class changes product claims about managed copies, exit
   packets, or destruction receipts, update the corresponding
   boundary-manifest row in the same change.

## Versioning rules

- Adding a new row is additive.
- Adding a new enum value to the schema is additive-minor and requires a
  `record_class_schema_version` bump plus a doc update here.
- Repurposing an existing enum value or reusing a `record_class_id` for
  a meaningfully different class is breaking and requires a new decision
  row plus a superseding registry row.

## What this document is not

- It is **not** the per-record state machine. That remains in
  [`record_state_and_policy_simulation_models.md`](./record_state_and_policy_simulation_models.md).
- It is **not** the implementation of retention enforcement or legal
  hold tooling.
- It is **not** a substitute for the boundary manifest. The boundary
  manifest says *which capability* is local-core or managed; this
  registry says *which record classes* those capabilities create and how
  those records behave under export, deletion, and offboarding.
