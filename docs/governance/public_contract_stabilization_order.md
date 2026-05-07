# Public-contract stabilization order, maturity gates, and promotion prerequisites

This document converts Aureline’s growing list of machine-readable public
contracts into a concrete stabilization order so the highest-cost surfaces
harden first and lower-risk surfaces do not block them by accident.

The goal is not to predict feature scope. The goal is to make “which contracts
must stabilize first, and why?” a mechanical answer during hardening.

Companion artifacts:

- [`/artifacts/governance/public_contract_priority.yaml`](../../artifacts/governance/public_contract_priority.yaml)
  — machine-readable stabilization order and risk rationale.
- [`/artifacts/governance/public_contract_gates.yaml`](../../artifacts/governance/public_contract_gates.yaml)
  — maturity-gate profiles (“what must exist before Beta/Stable claims are
  permitted?”) and per-surface prerequisites.

Authoritative inputs (do not duplicate here):

- [`/artifacts/governance/compatibility_surfaces.yaml`](../../artifacts/governance/compatibility_surfaces.yaml)
  — compatibility-bearing surface inventory (source-of-truth list).
- [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  — packet-level surface contracts once a surface is stable-facing.
- [`/docs/governance/interface_lifecycle_policy.md`](./interface_lifecycle_policy.md)
  — deprecation/alias/no-repurpose policy once a surface leaves experimental.
- [`/docs/governance/public_interface_stability_matrix.md`](./public_interface_stability_matrix.md)
  and
  [`/artifacts/governance/stability_label_rows.yaml`](../../artifacts/governance/stability_label_rows.yaml)
  — stability labels and horizon floors across surfaces.
- [`/artifacts/governance/standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  — which surfaces must be standard-shaped (JSON Schema, OpenAPI, WIT, etc).
- [`/artifacts/contracts/example_pack_index.yaml`](../../artifacts/contracts/example_pack_index.yaml)
  — example-pack coverage and CI-required sample payloads.
- [`/artifacts/contracts/frozen_surface_manifest.yaml`](../../artifacts/contracts/frozen_surface_manifest.yaml)
  and [`/docs/governance/frozen_surface_ci_policy.md`](./frozen_surface_ci_policy.md)
  — which already-frozen surfaces must carry same-train diff + companion
  updates.

## Definitions

**Public contract**
Any machine-readable surface whose payloads, schemas, or identifiers are
expected to be consumed outside the producing component and therefore must
survive independent upgrades. The canonical inventory is
`compatibility_surfaces.yaml`.

**Stabilize**
Publish the minimum set of contract artifacts needed so downstream consumers
can implement against the surface without inferring shape from prose:

- schema or interface definition (JSON Schema / OpenAPI / WIT / etc),
- contract narrative with explicit versioning + downgrade posture,
- at least one reference example payload (ideally CI-checked),
- compatibility window source row(s), and
- lifecycle/deprecation posture once non-experimental.

**Hardening order**
An execution order for contract stabilization work, not an implementation order
for features. A surface can be stabilized early even if the runtime is not yet
feature-complete, as long as the claims remain honest (Experimental/Provisional
labels where appropriate).

## Risk model (why some contracts must go first)

The priority register uses five factors to rank a surface:

1. **User-owned data impact** — how much durable user/workspace data is at
   risk if the contract drifts.
2. **Interoperability load** — how many independent producers/consumers (or
   external ecosystems) must agree on the contract.
3. **Deprecation cost** — how expensive it is to fix mistakes once the surface
   is broadly depended on (aliases, migrations, rollback constraints).
4. **Support burden** — how directly the surface affects diagnosis, support
   export, and “what happened?” explainability.
5. **Dependency fan-out** — how many other surface families cite the surface
   ids, schemas, or vocabulary.

Scores are intentionally conservative: they bias toward stabilizing surfaces
that can create lock-in, silent corruption, or ecosystem breakage.

## Stabilization order (launch-critical surfaces first)

The table below lists the launch-critical contract surfaces that should
stabilize first. The full ranked register lives in
`artifacts/governance/public_contract_priority.yaml`.

| Rank | Surface | Canonical inventory refs | Why this is early | Gate profile |
|---:|---|---|---|---|
| 1 | Command graph / descriptors / UI slots | `artifacts/governance/compatibility_surfaces.yaml#command_plane.command_graph_and_ui_slot_schema`, `artifacts/governance/stable_surface_inventory.yaml#command_plane.command_descriptor_and_invocation_session` | Highest fan-out: palette, menu, CLI, automation, AI-tool, replay. Stable ids and help anchors make later fixes costly. | `launch_critical.stable_claim` |
| 2 | Settings + profile JSON | `artifacts/governance/compatibility_surfaces.yaml#settings.setting_definition_and_effective_value_json`, `artifacts/governance/stable_surface_inventory.yaml#settings.setting_ids_and_effective_values` | User-owned durable data with high deprecation and migration cost; drift silently corrupts portability and support exports. | `user_owned_state.stable_claim` |
| 3 | Workspace manifests + pane tree | `artifacts/governance/compatibility_surfaces.yaml#workspace.manifest_and_pane_tree` | Durable workspace state in VCS; migration mistakes are expensive and user-visible; many flows depend on restore fidelity classes. | `user_owned_state.stable_claim` |
| 4 | Entry/restore + migration result packets | `artifacts/governance/compatibility_surfaces.yaml#workspace.entry_restore_and_migration_result` | Drives restore prompts, migration reporting, and support narratives; used as the “truth bridge” for portability. | `user_owned_state.provisional_claim` |
| 5 | CLI structured output | `artifacts/governance/compatibility_surfaces.yaml#automation.cli_structured_output_contract` | Automation and CI must not scrape prose; once scripts depend on JSON shapes, breaking changes are high-cost. | `automation_surface.provisional_claim` |
| 6 | Extension manifests + offline bundle registry | `artifacts/governance/compatibility_surfaces.yaml#extensions.manifest_registry_and_offline_bundle` | Ecosystem interoperability and policy enforcement; mistakes cause lock-in, trust failures, and brittle distribution. | `ecosystem_surface.provisional_claim` |
| 7 | WIT host worlds + bindings | `artifacts/governance/compatibility_surfaces.yaml#extensions.wit_host_worlds_and_bindings`, `artifacts/governance/stable_surface_inventory.yaml#extensions.wit_host_worlds` | Binary compatibility surface with expensive rollback; drift breaks extension loading/quarantine and forces coordinated upgrades. | `ecosystem_surface.provisional_claim` |
| 8 | Optional service API family | `artifacts/governance/compatibility_surfaces.yaml#service.optional_api_family`, `artifacts/governance/stable_surface_inventory.yaml#provider.service_api_family` | External interoperability plus mixed-version windows; requires standard-shaped OpenAPI story and explicit degradation posture. | `service_api.provisional_claim` |
| 9 | Evidence + support bundle manifests | `artifacts/governance/compatibility_surfaces.yaml#support.bundle_and_evidence_packets` | Support/export is the “court record” for trust; drift makes incidents un-debuggable and breaks offboarding/forensics. | `support_export.provisional_claim` |
| 10 | Notification envelopes + attention taxonomy | `artifacts/governance/compatibility_surfaces.yaml#notification.attention_and_activity_envelope` | Cross-surface UX truth; drift silently widens urgency or breaks explainability for activity feeds and support exports. | `notification_surface.provisional_claim` |
| 11 | Docs-pack manifest + citation anchors | `artifacts/governance/compatibility_surfaces.yaml#docs.docs_pack_manifest`, `artifacts/governance/stable_surface_inventory.yaml#docs.docs_pack_manifest` | Public-truth surface: docs/help, service health, and support must agree on freshness/version truth to avoid overclaim. | `public_truth_surface.stable_claim` |
| 12 | Compatibility/certification report formats | `artifacts/governance/compatibility_surfaces.yaml#certification.reference_workspace_and_archetype_report` | Governs “is this compatible?” evidence; without it, promotion decisions regress to ad hoc judgement. | `qualification_surface.provisional_claim` |

## Maturity gates (what must exist before promotion claims are allowed)

The maturity-gate map in `public_contract_gates.yaml` answers two questions:

1. **For this surface, what is the minimum evidence set before claiming**
   Provisional/Beta or Stable?
2. **Which surfaces are allowed to remain Experimental/Internal longer** without
   blocking stabilization of the launch-critical surfaces above?

Promotion claims must remain consistent across:

- the compatibility-surface row (`compatibility_surfaces.yaml`),
- the stability-label vocabulary (`stability_label_rows.yaml`),
- surface-contract packets (`stable_surface_inventory.yaml`) when stable-facing,
- docs/help and CLI publication surfaces, and
- the example pack’s CI-required fixtures when a surface is promoted.

## How to use this order during hardening

1. **Pick the next highest-ranked surface** whose promotion prerequisites are
   incomplete.
2. **Do not block a higher-ranked stabilization** on a lower-ranked surface
   unless the dependency is explicit in the gate prerequisites.
3. **When a surface becomes “frozen enough to build against,”** add it to the
   frozen-surface manifest so CI enforces same-train diff metadata.
4. **When a surface leaves experimental state,** apply the lifecycle/deprecation
   policy (no repurpose, alias rows, support windows).

## Updating the register

Changes to stabilization order must update the machine-readable register and
this narrative in the same change:

- `artifacts/governance/public_contract_priority.yaml`
- `docs/governance/public_contract_stabilization_order.md`

If a gate profile changes, update:

- `artifacts/governance/public_contract_gates.yaml`
- and any affected stability-label, standards, or example-pack rows the gate
  cites.
