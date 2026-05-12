# Boundary manifest — strawman

This document is the strawman for Aureline's open-source core versus
managed / commercial / service-plane boundary. It exists so the
boundary is explicit before implementation spreads across the repo, and
so no foundations-milestone task is left ambiguous about whether it
belongs in the open-core lane or a managed-only lane.

The strawman is **proposed, not yet ratified**. Every row below is
tagged `proposed`; the row stays in that state until an ADR or RFC
ratifies the classification (see
[`../governance/decision_backlog.md`](../governance/decision_backlog.md)
for the decision register that rows will link into). Rows are not
deleted; they are superseded.

## Scope

- Classify every product capability against the boundary.
- Record what remains usable when optional services are absent.
- Reserve the data-boundary, portability, deployment-profile,
  residual-dependency, and local-core-continuity slots each row will
  carry so later claim packets do not retrofit fields inconsistently.
- Point at decision rows and ownership lanes where the classification
  is gated.

## Out of scope

- Pricing, packaging, or commercial terms.
- The exact protocol or schema of any managed service (those live with
  the owning lane).
- Release-evidence claim manifests (those compose *over* the boundary
  manifest and live under `artifacts/release/`).

## Companion artifacts

- [`/schemas/product/boundary_manifest.schema.json`](../../schemas/product/boundary_manifest.schema.json)
  — machine-readable contract the manifest conforms to. A YAML
  instance under `artifacts/product/` is reserved as a future home; at
  this milestone the strawman lives in this document.
- [`/docs/governance/decision_backlog.md`](../governance/decision_backlog.md)
  — decisions that ratify individual rows land as ADRs closing
  rows in the decision register.
- [`/docs/governance/control_artifact_index.md`](../governance/control_artifact_index.md)
  — the control-artifact index carries a `boundary_manifest_strawman`
  row so this file has one canonical home and one owner.
- [`/docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
  — boundary rows that introduce managed copies, support exports, AI
  evidence, usage exports, exit packets, or destruction receipts should
  land or extend a record-class row in the same change so boundary
  claims do not hide record behavior.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — every row's `linked_lanes` entry resolves to an id in
  `scorecard_lane_index` there.
- [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  — shared continuity and impairment drill catalog seed. Boundary rows
  with optional-service dependencies should map
  `local_core_continuity` and `absence_narrows_to` back to drill ids
  there rather than inventing ad hoc outage language.

## Classifications

Each capability row carries exactly one classification:

- **`local_core`** — MUST work with no network, no sign-in, no hosted
  service, on a user's device. This is the desktop-local truth. A
  row in this class is a hard commitment: the capability cannot
  migrate out of local_core without an ADR and a superseding row.
- **`self_host_friendly`** — optional service whose protocol and
  reference implementation are designed for customer or third-party
  operation. A managed form may exist alongside, but the capability
  is reachable from a self-hosted control plane, a private mirror,
  or an air-gapped deployment. Local-core workflows keep working
  when this service is absent; the capability's own hosted workflows
  narrow until a self-hosted or managed instance is reachable.
- **`managed_convenience`** — hosted-only value-add layered on top of
  a self-hostable protocol. The underlying workflow remains reachable
  via the self-hostable path; the managed form adds convenience
  (dashboards, admin UX, cross-tenant scale) that is out of scope for
  the local-core lane. Removing the managed form narrows claims but
  does not reclassify the row.
- **`out_of_scope`** — explicitly not in the open-core or managed
  roadmap. Recording the row prevents implicit drift.

## Deployment profiles

The manifest reserves a closed vocabulary of deployment profiles so
rows can be read row-by-profile without ambiguity:

| Profile id | Summary | Sign-in expected | Public internet expected | Control plane operator |
|-----------------------|------------------------------------------------|------------------|--------------------------|------------------------|
| `individual_local`    | Single user on a personal device. No account required. | no  | optional | none |
| `self_hosted`         | Customer-operated control plane, mirror, relay, or registry. | yes (customer IdP) | optional | customer |
| `enterprise_online`   | Enterprise online with vendor-managed or customer-federated services. | yes | yes | vendor or customer |
| `air_gapped`          | No public internet. Offline bundles and local/admin policy only. | optional (local or federated) | no | customer |
| `managed_cloud`       | Vendor-operated SaaS control plane. | yes | yes | vendor |

The profiles are **operational stances**, not pricing tiers. The
`individual_local` profile is the baseline: every `local_core` row
MUST be available in it without degradation.

## Residual-dependency, data-boundary, portability slots

Every row reserves the following slots so later claim packets do not
retrofit them inconsistently:

- **`residual_dependencies`** — external systems, services, or
  credentials the capability still depends on in its normal form.
  Each residual-dependency entry carries a `self_hostable` flag and
  an `absence_impact` string so later claim packets can narrow
  claims without reclassifying the capability.
- **`data_boundary.primary_store`** — where the capability's primary
  state lives (local disk, workspace repo, customer control plane,
  vendor control plane, none, or a named combination).
- **`data_boundary.crosses_device_boundary`** — whether normal use
  sends user data off the device it was authored on.
- **`data_boundary.residency_controls`** — region / residency /
  BYO-key controls available to the operator.
- **`data_boundary.export_safe`** — whether state can be exported in
  an inspectable, redaction-aware form.
- **`portability.export_format`** — the format exports use. File-
  based and text-based formats are strongly preferred.
- **`portability.offboarding`** — how a user or organization leaves
  the capability without data loss. For `managed_convenience` rows
  this MUST describe the path back to the self-hostable or local
  form.
- **`portability.migration_story`** — how users move between
  deployment profiles (for example, from `managed_cloud` to
  `self_hosted`).
- **`local_core_continuity`** — explicit statement of what still
  works locally when every optional service this row touches is
  absent. No silent blanks; `managed_convenience` rows MUST state
  the local-core behaviour that persists when the managed form is
  unavailable.
- **`absence_narrows_to`** — narrowed claim the capability carries
  when sign-in is absent or the hosted control plane is unreachable.
  This is how the manifest encodes *narrowing instead of
  reclassifying*: a collaboration session narrows to "local edits
  continue; presence is unavailable" without becoming out-of-scope.

The deployment drill catalog seed consumes these two fields directly.
Rows that depend on optional services should make the retained
local-safe baseline specific enough that a drill row can restate it
without inventing new semantics.

## Capability rows

The rows below are the initial strawman. Every row resolves against
the fields defined in
[`boundary_manifest.schema.json`](../../schemas/product/boundary_manifest.schema.json).
Source anchors point into the product requirement and architecture
documents under `.t2/docs/`; quotes are illustrative, not exhaustive.

### Local core

#### `editor_core` — Editor and buffer

- **Classification:** `local_core`
- **Description:** In-process editor surface backed by the piece-tree
  buffer, save coordination, undo history, and edit primitives. The
  capability is the product's hero surface.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Editing, saving, undo, and navigation
  continue with no network and no account. This is the floor the
  product guarantees.
- **Residual dependencies:** none. (Reserved slot for a future
  local file-system capability if sandboxing policy requires one.)
- **Data boundary:** primary store is the workspace repository on
  local disk. State never crosses the device boundary. Export format
  is the source files themselves.
- **Portability:** no lock-in; users leave by closing the product.
- **Absence narrows to:** no narrowing required; the row is the
  baseline.
- **Linked decisions:** `D-0002` (buffer and editor-core persistence
  model).
- **Linked lanes:** `aureline-buffer`, `aureline-text`.
- **Status:** `proposed`.

#### `renderer_shell` — Renderer and shell surface

- **Classification:** `local_core`
- **Description:** Desktop shell window, rendering primitives, input
  plumbing, and accessibility bridge plumbing that let the editor
  appear on screen.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Window, shell, rendering, and
  accessibility surface continue with no network.
- **Residual dependencies:** OS compositor / window server only.
- **Data boundary:** primary store is local process state; no data
  leaves the device.
- **Portability:** visual state is ephemeral; layout preferences live
  in the configuration file covered by `configuration_profiles`.
- **Absence narrows to:** baseline; no narrowing.
- **Linked decisions:** `D-0001` (renderer stack), `D-0008`
  (accessibility bridge).
- **Linked lanes:** `aureline-render`, `aureline-text`,
  `accessibility_input_review`.
- **Status:** `proposed`.

#### `workspace_vfs` — Workspace file system

- **Classification:** `local_core`
- **Description:** Workspace root resolution, canonical path identity,
  file watching, and ignore resolution.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** File open, file save, and file watch
  continue with no network.
- **Residual dependencies:** OS file system and watcher APIs only.
- **Data boundary:** primary store is local disk; no data leaves the
  device.
- **Portability:** files stay in the repository; nothing to export.
- **Absence narrows to:** baseline; no narrowing.
- **Linked decisions:** `D-0003` (workspace VFS path identity and
  watcher model).
- **Linked lanes:** `aureline-vfs`.
- **Status:** `proposed`.

#### `command_plane` — Command identity and execution plane

- **Classification:** `local_core`
- **Description:** Shared command and execution contract that powers
  keybindings, command palette, CLI/headless flows, automation
  recipes, and (later) AI tool invocations. One plane, many callers.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Every command invokable from the palette
  is invokable from the CLI form, with no network.
- **Residual dependencies:** none.
- **Data boundary:** primary store is local disk (command registry is
  compiled or manifest-driven); no data leaves the device.
- **Portability:** command graph is inspectable via the CLI; export
  is reserved for route-map and build-truth artifacts
  (`route_build_truth` control-artifact row).
- **Absence narrows to:** baseline; no narrowing.
- **Linked decisions:** `D-0006` (shell / command-system contract),
  `D-0007` (keyboard-complete command graph).
- **Linked lanes:** `aureline-shell-spike`, `shell_command_system`.
- **Status:** `proposed`.

#### `local_git` — Local version control integration

- **Classification:** `local_core`
- **Description:** Local Git operations: status, stage, commit,
  branch, blame, diff, merge tools, integrated against the workspace
  VFS.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** All local Git operations work offline.
  Remote-hosting integrations are out of this row's scope and live
  under extensions.
- **Residual dependencies:** Git binary on the host, SSH client where
  remotes are used.
- **Data boundary:** primary store is the local Git repository. No
  data leaves the device unless the user pushes.
- **Portability:** standard Git repository; nothing proprietary.
- **Absence narrows to:** baseline; no narrowing.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `aureline-vfs`, `docs_public_truth`.
- **Status:** `proposed`.

#### `extension_runtime_local` — Local extension runtime

- **Classification:** `local_core`
- **Description:** In-process / sandboxed runtime for extensions
  (WASM or equivalent), with a capability-scoped permission model
  and a local install path.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Previously installed extensions load
  from the local cache with no network; no new installs are required
  to open the product.
- **Residual dependencies:** locally installed extension bundles.
- **Data boundary:** primary store is local disk; cross-device only
  when extensions themselves cross the boundary, which is a separate
  per-extension concern.
- **Portability:** extension bundles are file-based; they can be
  copied between hosts.
- **Absence narrows to:** baseline; no narrowing for already-installed
  extensions. New extension discovery narrows via
  `extension_registry_mirror`.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `aureline-rpc` (host contract),
  `docs_public_truth`.
- **Status:** `proposed`.

#### `local_ai_byok` — Local AI and BYOK providers

- **Classification:** `local_core`
- **Description:** Local model execution and bring-your-own-key AI
  providers, invoked through the same command plane as other
  workflows. The product's AI trust story is local-first and
  provider-neutral.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Search, edit, test, and Git never depend
  on any AI provider. The local and BYOK paths are reachable without
  a hosted model gateway.
- **Residual dependencies:** user-supplied credentials for BYOK
  providers; optional local model weights for the local path.
- **Data boundary:** primary store is local disk (credentials live in
  OS keychains; model weights live on local disk). Cross-device only
  when the user explicitly configures a remote BYOK provider.
- **Portability:** credentials round-trip via the configuration
  profile; model weights are file-based.
- **Absence narrows to:** if no BYOK provider is configured and no
  local model is present, AI-assisted workflows become unavailable;
  every non-AI workflow remains available.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `aureline-rpc`, `docs_public_truth`.
- **Status:** `proposed`.

#### `accessibility_surface` — Accessibility and semantic surface

- **Classification:** `local_core`
- **Description:** Keyboard-complete command graph, semantic surface
  for assistive tech, reduced-motion and contrast guarantees.
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Accessibility surface is available in
  every profile with no network.
- **Residual dependencies:** OS accessibility APIs.
- **Data boundary:** primary store is local process state; no data
  leaves the device.
- **Portability:** accessibility artifacts are exported through the
  `accessibility_review_packets` control-artifact row.
- **Absence narrows to:** baseline; no narrowing.
- **Linked decisions:** `D-0007` (input model), `D-0008`
  (accessibility bridge).
- **Linked lanes:** `accessibility_input_review`, `aureline-render`.
- **Status:** `proposed`.

#### `configuration_profiles` — Layered configuration and profiles

- **Classification:** `local_core`
- **Description:** Layered configuration model (embedded defaults,
  signed local admin bundle, user/workspace configuration) and
  profiles. Sync is a separate capability (`managed_sync_profile`).
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Configuration reads and writes work
  offline. Admin bundles are cached locally.
- **Residual dependencies:** optional signed admin bundle source.
- **Data boundary:** primary store is local disk. Admin bundles may
  be fetched from the organization's chosen source; they are cached
  locally.
- **Portability:** configuration files are JSON/YAML; fully
  inspectable and copyable.
- **Absence narrows to:** if the admin bundle source is unavailable,
  the last cached bundle continues to apply.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `docs_public_truth`, `support_export`.
- **Status:** `proposed`.

#### `local_support_bundle` — Local / offline support bundles

- **Classification:** `local_core`
- **Description:** Redaction-aware support bundle generation, doctor
  probes, and crash-diagnostics corpus produced on the user's device.
  Telemetry upload is a separate capability
  (`telemetry_support_pipeline`).
- **Deployment profiles:** `individual_local`, `self_hosted`,
  `enterprise_online`, `air_gapped`, `managed_cloud`.
- **Local-core continuity:** Bundles are producible and inspectable
  with no network. Users can attach them to issues or share them
  privately without any hosted service.
- **Residual dependencies:** none.
- **Data boundary:** primary store is local disk. Bundles follow the
  export-safe packet schema.
- **Portability:** bundles are file-based archives.
- **Absence narrows to:** baseline; no narrowing.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `support_export`.
- **Status:** `proposed`.

### Self-host-friendly

#### `identity_policy_service` — Identity and policy distribution

- **Classification:** `self_host_friendly`
- **Description:** OpenID-Connect-compatible sign-in and policy-
  distribution service. Self-hostable reference implementation,
  file-based policy ingestion fallback, and optional vendor-managed
  form.
- **Deployment profiles:** `self_hosted`, `enterprise_online`,
  `air_gapped` (local or federated IdP), `managed_cloud`.
- **Local-core continuity:** Local editing, search, Git, tasks, and
  local AI never require a sign-in. Loss of control-plane
  connectivity pauses managed actions only.
- **Residual dependencies:** identity provider (`identity_provider`,
  self-hostable, absence narrows hosted actions), policy service
  (`policy_service`, self-hostable, absence narrows policy refresh).
- **Data boundary:** primary store is the operator's chosen control
  plane (customer-hostable or vendor-hosted). Region pinning is an
  operator concern and is recorded per deployment.
- **Portability:** policy bundles are signed JSON/YAML; exportable.
  Offboarding is a policy-bundle export and revocation.
- **Absence narrows to:** sign-in is unavailable, but cached
  sessions and last-known policy keep the local product working;
  only new protected operations pause until a session refreshes.
  **This is the canonical narrowing example: absence of sign-in
  narrows the row's claim from "managed identity available" to
  "local product fully usable under the last-known policy" without
  reclassifying identity as out-of-scope.**
- **Linked decisions:** `D-0009` (identity modes and workspace-trust
  posture), closed by `docs/adr/0001-identity-modes.md`.
- **Linked lanes:** `aureline-vfs`, `aureline-rpc`,
  `release_evidence`.
- **Status:** `accepted`.

#### `collaboration_relay` — Presence and session relay

- **Classification:** `self_host_friendly`
- **Description:** Presence, session discovery, NAT traversal, and
  permission-checked relay for collaborative editing. Self-hostable
  reference implementation; vendor-managed scale relay is the
  convenience form.
- **Deployment profiles:** `self_hosted`, `enterprise_online`,
  `managed_cloud`. (Not normally reachable in `air_gapped` unless the
  customer operates a private relay; `individual_local` reaches this
  row only when opting into a hosted or peer session.)
- **Local-core continuity:** Local editing continues with no relay.
  Reconnect and local replay are visible when the relay drops.
- **Residual dependencies:** relay service
  (`relay`, self-hostable, absence narrows to local-only editing),
  network egress to the relay.
- **Data boundary:** ephemeral session data crosses the device
  boundary; durable work lives in the local workspace and Git.
- **Portability:** invitations are revocable; session links are
  rotatable; customers can move to a self-hosted relay.
- **Absence narrows to:** local edits continue; presence is
  unavailable; session reconnects when a relay becomes reachable.
- **Linked decisions:** none at this milestone; collaboration
  contracts follow first-beta-milestone RFC work.
- **Linked lanes:** `aureline-rpc`, `docs_public_truth`.
- **Status:** `proposed`.

#### `workspace_control_plane` — Remote workspace orchestration

- **Classification:** `self_host_friendly`
- **Description:** Template catalog, lifecycle, provisioning, and
  suspend/resume for SSH/container/cloud-VM workspaces. Self-hostable
  core with managed convenience layered on top.
- **Deployment profiles:** `self_hosted`, `enterprise_online`,
  `managed_cloud`.
- **Local-core continuity:** Direct SSH or container attach remains
  available where configured, independently of the control plane.
- **Residual dependencies:** control plane (`control_plane`,
  self-hostable, absence narrows to direct attach), orchestrator
  back-end, network egress.
- **Data boundary:** workspace state lives in the operator's chosen
  store; region pinning and customer-managed keys are operator
  concerns recorded per deployment.
- **Portability:** templates and prebuild definitions are OCI
  references plus YAML/JSON manifests; importable to a self-hosted
  or alternate managed deployment.
- **Absence narrows to:** orchestrated workspace lifecycle becomes
  unavailable; direct SSH/container attach and local editing
  continue.
- **Linked decisions:** `D-0004` (RPC transport) for cross-process
  contracts the workspace control plane will speak.
- **Linked lanes:** `aureline-rpc`, `release_evidence`.
- **Status:** `proposed`.

#### `model_gateway` — AI provider routing and policy gateway

- **Classification:** `self_host_friendly`
- **Description:** Provider routing, rate limits, audit, redaction,
  and policy for AI requests. Local and BYOK providers remain
  reachable without this service; the gateway adds managed keys,
  caching, and org quotas.
- **Deployment profiles:** `self_hosted`, `enterprise_online`,
  `managed_cloud`.
- **Local-core continuity:** `local_ai_byok` remains usable when the
  gateway is absent; AI outages never block search, edit, test, or
  Git.
- **Residual dependencies:** model gateway (`model_gateway`,
  self-hostable, absence narrows to direct BYOK), model providers,
  network egress.
- **Data boundary:** prompts and completions may cross the device
  boundary. Redaction policy is enforced at the gateway when present
  and at the client otherwise.
- **Portability:** policy bundles are exportable; provider keys are
  portable.
- **Absence narrows to:** AI requests fall back to BYOK or local
  models; managed quotas and audit are unavailable.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `aureline-rpc`, `docs_public_truth`.
- **Status:** `proposed`.

#### `extension_registry_mirror` — Extension registry and mirror

- **Classification:** `self_host_friendly`
- **Description:** Discovery, provenance, revocation, and private-feed
  distribution of extensions. Vendor-neutral protocol with a local
  mirror option and an optional vendor-hosted marketplace.
- **Deployment profiles:** `self_hosted`, `enterprise_online`,
  `air_gapped` (via offline bundle or internal mirror),
  `managed_cloud`.
- **Local-core continuity:** Installed extensions keep running when
  the registry is unreachable; offline bundles cover
  air-gapped installs.
- **Residual dependencies:** registry (`registry`, self-hostable,
  absence narrows to mirror or offline bundle), mirror
  (`mirror`, self-hostable), signing material for provenance.
- **Data boundary:** catalog metadata may cross the device boundary;
  customers can operate private mirrors to keep it internal.
- **Portability:** registry content is OCI-compatible; mirror content
  can be exported and re-imported.
- **Absence narrows to:** discovery of new extensions pauses; already
  installed extensions continue to run; installs retry cleanly when
  a registry or mirror becomes reachable.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `release_evidence`, `docs_public_truth`.
- **Status:** `proposed`.

#### `telemetry_support_pipeline` — Telemetry and support pipeline

- **Classification:** `self_host_friendly`
- **Description:** Opt-in metrics, crash reports, and support-bundle
  delivery. Local and offline bundles exist in the local-core lane
  (`local_support_bundle`); the pipeline adds aggregation, cohorts,
  and support tooling.
- **Deployment profiles:** `self_hosted`, `enterprise_online`,
  `managed_cloud`.
- **Local-core continuity:** Bundles can be produced locally with no
  network. The pipeline is opt-in; declining it never degrades the
  local product.
- **Residual dependencies:** telemetry sink
  (`telemetry_sink`, self-hostable, absence narrows to local bundles
  only), network egress.
- **Data boundary:** aggregated telemetry crosses the device
  boundary; redaction rules are documented per packet family.
- **Portability:** telemetry payloads are OpenTelemetry-compatible;
  sinks are swappable.
- **Absence narrows to:** aggregated dashboards are unavailable;
  local bundles remain the canonical supportability artefact.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `support_export`, `release_evidence`.
- **Status:** `proposed`.

### Managed convenience

#### `managed_sync_profile` — Managed settings sync

- **Classification:** `managed_convenience`
- **Description:** Vendor-hosted (or customer-managed) settings-sync
  service over an end-to-end encrypted or customer-managed-storage
  protocol. The underlying configuration capability is local-core.
- **Deployment profiles:** `enterprise_online`, `managed_cloud`.
  (Self-hostable protocol is recorded under `configuration_profiles`'
  admin-bundle source.)
- **Local-core continuity:** Configuration files continue to load and
  save locally without sync; sync is strictly additive.
- **Residual dependencies:** sync sink
  (`control_plane`, self-hostable at the protocol level, absence
  narrows to local configuration only), network egress.
- **Data boundary:** configuration crosses the device boundary when
  sync is active. End-to-end-encrypted payloads or customer-managed
  storage are required.
- **Portability:** configuration files are the ground truth; sync is
  additive. Offboarding is turning sync off and keeping local files.
- **Absence narrows to:** settings do not sync across devices; each
  device keeps its local configuration.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `docs_public_truth`.
- **Status:** `proposed`.

#### `hosted_marketplace_ui` — Hosted marketplace UI

- **Classification:** `managed_convenience`
- **Description:** Vendor-hosted marketplace front end and enterprise
  curation UI. The underlying registry protocol is recorded under
  `extension_registry_mirror`.
- **Deployment profiles:** `enterprise_online`, `managed_cloud`.
- **Local-core continuity:** Local extension install and execution
  work without the hosted UI; CLI and private-mirror install paths
  remain canonical.
- **Residual dependencies:** registry (`registry`, self-hostable),
  network egress, hosted UI.
- **Data boundary:** browse telemetry and curation metadata cross the
  device boundary; private-mirror operators can keep all of it
  internal.
- **Portability:** marketplace catalog content is OCI-compatible.
- **Absence narrows to:** browse UI is unavailable; CLI and mirror
  installs remain available.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `docs_public_truth`.
- **Status:** `proposed`.

#### `managed_ai_quota_billing` — Managed AI quota, billing, and audit UI

- **Classification:** `managed_convenience`
- **Description:** Hosted dashboards, quota enforcement, and audit
  trails for AI usage, layered on the model gateway protocol.
- **Deployment profiles:** `enterprise_online`, `managed_cloud`.
- **Local-core continuity:** `local_ai_byok` and self-hosted
  `model_gateway` remain reachable without the managed UI.
- **Residual dependencies:** model gateway (`model_gateway`,
  self-hostable), billing sink (`telemetry_sink`).
- **Data boundary:** usage aggregates cross the device boundary;
  customer-managed keys and region pinning are recorded per
  deployment.
- **Portability:** quota and audit exports are file-based.
- **Absence narrows to:** quota and audit UI are unavailable; AI
  requests fall back to BYOK or local models.
- **Linked decisions:** none at this milestone.
- **Linked lanes:** `docs_public_truth`, `release_evidence`.
- **Status:** `proposed`.

#### `fleet_admin_ui_scim` — Fleet admin UI, SCIM provisioning, audit dashboards

- **Classification:** `managed_convenience`
- **Description:** Hosted admin UX for fleet reporting, SCIM-driven
  provisioning, and audit dashboards. The identity and policy protocol
  is covered by `identity_policy_service`; this row covers the
  convenience UI layered on top.
- **Deployment profiles:** `enterprise_online`, `managed_cloud`.
- **Local-core continuity:** Sign-in, policy distribution, and
  deprovisioning continue via the self-hostable identity and policy
  protocols when the admin UI is absent.
- **Residual dependencies:** identity provider (`identity_provider`,
  self-hostable), policy service (`policy_service`, self-hostable),
  admin UI.
- **Data boundary:** fleet metadata crosses the device boundary;
  region pinning is an operator concern.
- **Portability:** SCIM is standards-based; policy bundles are
  file-based and exportable.
- **Absence narrows to:** fleet dashboards are unavailable; SCIM and
  policy distribution remain reachable via the self-hostable
  protocol; local product remains fully usable under the
  last-known policy. **Second narrowing example: absence of the
  hosted control plane narrows fleet observability without
  reclassifying identity or policy as out-of-scope.**
- **Linked decisions:** `D-0009` (identity modes and workspace-trust
  posture), closed by `docs/adr/0001-identity-modes.md`.
- **Linked lanes:** `release_evidence`, `docs_public_truth`.
- **Status:** `accepted`.

### Out of scope

#### `legacy_os_support_32bit` — 32-bit and legacy desktop OS support

- **Classification:** `out_of_scope`
- **Description:** Support for 32-bit systems and pre-supported legacy
  desktop OS versions.
- **Deployment profiles:** `individual_local` (nominal only; the
  capability does not exist in any profile).
- **Local-core continuity:** n/a; the capability is out of scope.
- **Residual dependencies:** n/a.
- **Data boundary:** n/a.
- **Portability:** n/a.
- **Absence narrows to:** the row exists to prevent implicit drift;
  absence is the baseline.
- **Linked decisions:** none.
- **Linked lanes:** none.
- **Status:** `proposed`.

#### `mandatory_vendor_hosted_auth` — Mandatory vendor-hosted sign-in for local use

- **Classification:** `out_of_scope`
- **Description:** A sign-in wall that blocks local editing, search,
  Git, tasks, or local AI behind a vendor-hosted account. Recorded
  as out-of-scope so the account-free local mode cannot drift into
  requiring a sign-in wall.
- **Deployment profiles:** `individual_local` (nominal only).
- **Local-core continuity:** by construction, this row is the inverse
  of local-core continuity; it is out of scope.
- **Residual dependencies:** n/a.
- **Data boundary:** n/a.
- **Portability:** n/a.
- **Absence narrows to:** absence is the required state. A decision
  to flip this to in-scope would supersede the row rather than edit
  it.
- **Linked decisions:** `D-0009` (identity modes and workspace-trust
  posture), closed by `docs/adr/0001-identity-modes.md`.
- **Linked lanes:** none.
- **Status:** `accepted`.

#### `general_ci_platform` — General CI / cloud control plane replacement

- **Classification:** `out_of_scope`
- **Description:** Replacing dedicated CI platforms, terminal
  multiplexers, or cloud control planes with first-party equivalents.
- **Deployment profiles:** `individual_local` (nominal only).
- **Local-core continuity:** n/a.
- **Residual dependencies:** n/a.
- **Data boundary:** n/a.
- **Portability:** n/a.
- **Absence narrows to:** baseline; absence is the required state.
- **Linked decisions:** none.
- **Linked lanes:** none.
- **Status:** `proposed`.

## Unambiguous foundations-milestone coverage

The pre-implementation foundations milestone (the "foundations"
milestone referenced in
[`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml))
produces only governance artefacts: none of those artefacts are
product capabilities, so none of them are ambiguous about the
boundary. The table below maps each foundations task family to the
boundary row (or rows) it later reaches into, so no foundations task
is left ambiguous about whether it belongs in open-core or a
managed-only lane.

| Foundations task family | Artefact produced this milestone | Boundary row(s) it gates |
|---------------------------------------------------|------------------------------------------------|--------------------------------------------|
| Workspace and topology baseline                   | Cargo workspace, crate topology, topology doc  | `editor_core`, `renderer_shell`, `workspace_vfs`, `command_plane` |
| Reproducible-build baseline                       | Pinned toolchain, bootstrap, build-identity    | All rows (build identity anchors release evidence across every row) |
| Ownership matrix and CODEOWNERS                   | DRI map and ownership matrix                   | All rows (every row links into lane ids)   |
| ADR / RFC / decision backlog                      | Decision register and templates                | Every row's `linked_decisions`             |
| Contribution, provenance, and compliance baseline | CONTRIBUTING, provenance baseline, checklist   | All rows (provenance anchors every release claim) |
| Control-artifact index and issue routing          | Control-artifact index and issue routing       | All rows (this strawman is one control asset under that index) |
| Boundary manifest strawman (this document)        | Strawman + schema                              | Every capability row                       |

All foundations tasks map to `local_core` or governance/public-truth
lanes; none introduces a `managed_convenience`-only capability at
this milestone. When a foundations task later does reach a managed
lane (for example, when release-evidence signing moves onto hosted
infrastructure), the change lands as a decision row that supersedes
or refines the affected capability row, not as a reclassification of
the foundations artefact itself.

## Boundary posture under absence

Two canonical narrowing examples are carried in the row bodies above
and are summarised here:

1. **Sign-in absent.** `identity_policy_service` narrows from
   "managed identity available" to "local product fully usable under
   the last-known policy". Identity and policy are not reclassified
   as out-of-scope; the hosted and self-hostable forms simply pause.
   Cached sessions, last-known policy, and local editing continue.
2. **Hosted control plane absent.** `fleet_admin_ui_scim` narrows
   fleet observability (dashboards unavailable) without
   reclassifying the underlying identity and policy protocols,
   which remain reachable via their self-hostable reference
   implementation. SCIM and policy distribution continue; only the
   convenience UI pauses.

Every other row's `absence_narrows_to` field encodes the same shape
of narrowing — claim contraction, never reclassification.

## Adding or updating a row

1. Append the row to this document under the correct classification
   heading, or open a successor row pointing at a superseded one.
2. Fill every required field. `local_core_continuity` and
   `absence_narrows_to` are not optional.
3. Reserve the deployment-profile, residual-dependency,
   data-boundary, and portability slots even if the first
   implementation fills them later.
4. If the row introduces or changes a retained/exported/offboarding
   artifact class, add or update the corresponding record-class row in
   [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
   and, for alpha-facing support/export or managed-copy claims, the
   schema and record alpha registries
   [`/artifacts/governance/schema_registry_alpha.yaml`](../../artifacts/governance/schema_registry_alpha.yaml)
   and
   [`/artifacts/governance/record_class_registry_alpha.yaml`](../../artifacts/governance/record_class_registry_alpha.yaml)
   in the same change.
5. When an ADR ratifies the classification, set the row's status to
   `accepted` and link the ADR from the row's `linked_decisions`
   entry (and from the decision register row itself).
6. When this document and the machine form disagree, the machine
   form wins for tooling and this document must be updated in the
   same change. (The machine form is reserved under
   `artifacts/product/boundary_manifest.yaml`; it is not yet
   populated at this milestone.)

## What this manifest is not

- It is **not** a pricing or packaging document. The rows describe
  the technical and product boundary only.
- It is **not** a claim manifest. The claim-manifest packet family
  lives under
  [`/artifacts/governance/governance_packet_template.yaml`](../../artifacts/governance/governance_packet_template.yaml)
  and cites this manifest for the underlying boundary.
- It is **not** a substitute for the decision register. Decisions
  that gate boundary rows live in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml);
  this manifest only links into them.
- It is **not** a release-evidence pack. Release-evidence packets
  live under `artifacts/release/` and compose over this manifest.
