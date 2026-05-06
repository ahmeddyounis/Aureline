# Public-interface stability labels, compatibility horizons, and version advertisement

This document makes Aureline’s stability promises explicit across the
public surfaces a downstream consumer can depend on: schemas, command
descriptors, CLI machine output, export artifacts, state bundles,
extension metadata, WIT host worlds, service APIs, event envelopes, and
docs/evidence packets.

It is the narrative companion to:

- [`/artifacts/governance/stability_label_rows.yaml`](../../artifacts/governance/stability_label_rows.yaml)
  — machine-readable stability-label definitions and their horizon
  floors.
- [`/schemas/governance/version_advertisement.schema.json`](../../schemas/governance/version_advertisement.schema.json)
  — machine-readable “what version is this surface speaking?” record
  that CLI payloads, service responses, and exported packets can embed.
- [`/fixtures/governance/public_interface_examples/`](../../fixtures/governance/public_interface_examples/)
  — worked examples for schema, CLI, and service/host boundaries.

This document does **not** implement runtime negotiation; mixed-version
negotiation is governed by the reusable envelope contract and skew
windows (see “Related mixed-version policy” below). The goal here is
labeling and advertisement: a reviewer can answer, for each surface
family, **what is stable, for how long, and how that is advertised**.

## Related inventories and policies

This document composes existing governance contracts rather than
replacing them:

- [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  and [`/docs/governance/interface_inventory.md`](./interface_inventory.md)
  — the canonical surface families and their packet-level contract
  posture.
- [`/artifacts/governance/compatibility_surfaces.yaml`](../../artifacts/governance/compatibility_surfaces.yaml)
  and [`/docs/governance/compatibility_surface_inventory.md`](./compatibility_surface_inventory.md)
  — the wider surface inventory, including outline-only families.
- [`/docs/governance/interface_lifecycle_policy.md`](./interface_lifecycle_policy.md)
  and [`/schemas/governance/deprecation_metadata.schema.json`](../../schemas/governance/deprecation_metadata.schema.json)
  — deprecation metadata, overlap windows, replacement rules, and
  notice-surface requirements for stable IDs and schema families once a
  surface leaves experimental state.
- [`/docs/automation/cli_surface_contract.md`](../automation/cli_surface_contract.md)
  and [`/schemas/automation/cli_output_registry_entry.schema.json`](../../schemas/automation/cli_output_registry_entry.schema.json)
  — CLI/headless machine-output stability classes and schema binding.

### Related mixed-version policy

Mixed-version negotiation and upgrade/rollback order are governed by:

- [`/docs/compat/upgrade_order_contract.md`](../compat/upgrade_order_contract.md)
- [`/schemas/compat/mixed_version_envelope.schema.json`](../../schemas/compat/mixed_version_envelope.schema.json)
- [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml)
- [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)

Stability labels below describe **compatibility promises for the surface
itself**. Mixed-version negotiation describes **what happens when two
participants are not on the same version line**.

## Stability label vocabulary (cross-surface)

The stability labels in `stability_label_rows.yaml` provide a
cross-surface vocabulary that maps onto:

- `maturity_lane` in surface-contract packets (`stable` / `beta` /
  `experimental` / `internal`);
- `interface_lifecycle_state` in deprecation metadata (`internal` /
  `experimental` / `beta` / `stable` / `lts_surface` / `deprecated` /
  `retired`);
- machine-output stability classes for CLI/headless output.

The labels are intentionally phrased for reviewer and docs/help
surfaces:

| Label | Meaning | Horizon floor | Allowed change types | Deprecation burden |
|---|---|---|---|---|
| **Internal** | Not a public contract. External consumers MUST NOT rely on it. | None | Breaking changes and removal allowed at any time. | None beyond “fail predictably” rules. |
| **Experimental** | Publicly visible, best-effort only. | None | Breaking changes allowed; removal allowed with explicit notice. | Notice MUST be explicit wherever the surface is discoverable. |
| **Provisional** | Intended for promotion; compatibility is expected within a release family, but additive growth is expected. | At least one minor release **or** 90 days of overlap before removal. | Additive fields and new enum members allowed; breaking removals require explicit migration guidance. | Deprecation metadata REQUIRED once stable IDs/schema families are claimed non-experimental. |
| **Stable** | General third-party contract. | At least two minor releases **and** 12 months of overlap before breaking removal. | Additive-only within a schema epoch; breaking changes require a version bump plus migration guidance. | Deprecation metadata rows, notice surfaces, and compatibility reporting are mandatory. |
| **Legacy** | Deprecated-but-supported. Consumers SHOULD migrate. | Inherits the Stable (or stronger) overlap window the deprecation metadata row declares. | No repurposing; removal only after the overlap window closes. | Requires deprecation metadata row + notice surfaces (`docs_help`, `release_notes`, `machine_readable_metadata`, plus any surface-local help). |

Notes:

- “Provisional” aligns with the packet lane `beta` and the lifecycle
  state `beta`: the surface is usable by external consumers but is
  explicitly still evolving under a published contract.
- “Legacy” is an **advertised** label for `interface_lifecycle_state =
  deprecated` rows that remain within their declared overlap window. The
  authoritative lifecycle state remains `deprecated`; “Legacy” is the
  compact reviewer-facing rendering used by docs/help and support
  surfaces.

## Version advertisement rules

Version advertisement is required on **machine-readable surfaces** so
support and mixed-version tooling can answer “what contract is this
payload speaking?” without scraping prose or guessing from context.

### Machine-readable rule

Any machine-readable surface that can leave the process boundary (export
packet, support bundle, CLI `--json` payload, service response, WIT host
contract binding, state bundle, docs pack manifest) MUST be able to
advertise:

1. the governing surface-contract reference (`stable_surface_inventory`
   row ref, or `compatibility_surfaces` row ref);
2. the current stability label (Internal / Experimental / Provisional /
   Stable / Legacy);
3. the primary contract version anchor(s) (schema file ref + version
   slot, OpenAPI version, WIT package version, or equivalent); and
4. the `running_build_identity_ref` of the producer when the payload is
   emitted by a running build.

The machine-readable shape for the advertisement block is frozen in
[`/schemas/governance/version_advertisement.schema.json`](../../schemas/governance/version_advertisement.schema.json).

### Human-surface rule

Human surfaces MUST render stability and version truth without implying
stronger guarantees than the machine contracts provide:

- CLI help MUST describe human output as non-contract by default; CLI
  machine output stability is shown via the registry row.
- Docs/help pages MUST render stability badges and the overlap horizon
  for deprecated/legacy interfaces, and MUST link to the machine-readable
  row refs that back the claim.
- Support/export manifests MUST carry the same surface-contract refs so
  field runbooks can trace from a payload to its policy sources.

## Public surface matrix (families)

This matrix answers, for each public surface family, what is stable, for
how long, and where that truth lives.

| Surface family | Canonical contract refs | Typical stability target | Compatibility / horizon source | Version advertisement anchor |
|---|---|---|---|---|
| Command descriptors | `artifacts/governance/stable_surface_inventory.yaml#command_plane.command_descriptor_and_invocation_session` | Provisional → Stable | `artifacts/compat/qualification_matrix_seed.yaml#compat_row:command_plane.command_descriptor_schema` + lifecycle overlap floors | Schema version field(s) + `command_revision_ref`; embed `version_advertisement_record` on machine exports. |
| CLI machine output | `artifacts/governance/compatibility_surfaces.yaml#automation.cli_structured_output_contract` | Stable for automation-facing verbs | CLI stability class + declared schema binding; lifecycle overlap for command IDs | Payload schema version field + registry schema version; optional embedded advertisement record. |
| Exported build/artifact identity | `artifacts/governance/stable_surface_inventory.yaml#build.exact_build_identity_fields` | Provisional → Stable | `artifacts/compat/qualification_matrix_seed.yaml#compat_row:release_identity.exact_build_propagation` | `running_build_identity_ref` (resolves to exact-build identity). |
| State bundles (profile/layout) | `artifacts/governance/stable_surface_inventory.yaml#state.portable_profile_and_layout_restore` | Provisional → Stable | `artifacts/compat/qualification_matrix_seed.yaml#compat_row:state.profile_layout_schema` + lifecycle overlap floors | Schema version fields on bundles + embedded advertisement record on exported bundles. |
| Extension manifests / offline bundles | `artifacts/governance/compatibility_surfaces.yaml#extensions.manifest_registry_and_offline_bundle` | Provisional → Stable | Extension host skew window + lifecycle overlap for extension IDs | Manifest schema version + advertised host contract identity; embed advertisement record on exports. |
| WIT host worlds | `artifacts/governance/stable_surface_inventory.yaml#extensions.wit_host_worlds` | Experimental → Provisional | `artifacts/compat/qualification_matrix_seed.yaml#compat_row:extension_host.sdk_wit_permission_window` | WIT package SemVer + declared permission vocabulary version; embed advertisement record in negotiation/inspection payloads. |
| Optional service APIs | `artifacts/governance/stable_surface_inventory.yaml#provider.service_api_family` | Experimental → Provisional | `artifacts/compat/qualification_matrix_seed.yaml#compat_row:provider.service_api_and_browser_handoff` + skew windows | OpenAPI `info.version` + explicit API-version header family; embed advertisement record in JSON responses where applicable. |
| Event envelopes (task/build/test) | `artifacts/governance/stable_surface_inventory.yaml#tooling.task_event_envelope` | Experimental → Provisional → Stable | `artifacts/compat/qualification_matrix_seed.yaml#compat_row:tooling.task_event_envelope` + skew windows | `task_event_envelope_schema_version` + envelope `record_kind`; embed advertisement record in replay bundles. |
| Docs packs / evidence packets | `artifacts/governance/stable_surface_inventory.yaml#docs.docs_pack_manifest` (and sibling evidence schemas) | Provisional → Stable | Docs-pack compat window + build identity; lifecycle overlap for schema families | Manifest schema version + `semver_version` / compat window; embed advertisement record on exported manifests. |

Worked examples of the advertisement record for a schema, a CLI command,
and a service boundary live in
`fixtures/governance/public_interface_examples/`.

