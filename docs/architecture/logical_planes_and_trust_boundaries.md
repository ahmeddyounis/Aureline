# Logical planes, trust boundaries, and cross-plane interaction rules

This document freezes one runtime and architecture picture for the
interaction, knowledge, execution, extensibility, trust-and-control,
and optional-services planes. It exists so later implementation work
cannot invent side planes or hidden crossings, and so service
topology, authority, provider, transport, and boundary-manifest
tasks can cite one map instead of reconstructing boundary language.

## Companion artifacts

- [`/artifacts/architecture/plane_matrix.yaml`](../../artifacts/architecture/plane_matrix.yaml)
  — machine-readable plane map: primary logical planes, service
  sub-planes, seeded-crate and M0-artifact placement, allowed
  runtime and compile-time call directions, visibility
  expectations, and forbidden implicit crossings.
- [`/artifacts/architecture/trust_boundaries.yaml`](../../artifacts/architecture/trust_boundaries.yaml)
  — machine-readable trust-boundary matrix: eight primary axes,
  typed policy surfaces, allowed crossing forms, visible
  disclosure expectations, and cross-boundary rules.
- [`/artifacts/architecture/process_placement_map.yaml`](../../artifacts/architecture/process_placement_map.yaml)
  and
  [`/docs/architecture/service_topology_and_process_placement.md`](service_topology_and_process_placement.md)
  — authoritative service-plane identities, process placement,
  and inline-work policy.
- [`/artifacts/architecture/protected_path_dependency_rules.yaml`](../../artifacts/architecture/protected_path_dependency_rules.yaml)
  — compile-time dependency classes, forbidden directions, and
  sentinel patterns CI enforces.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — local-core, self-host-friendly, managed-convenience, and
  out-of-scope classifications for product capability rows.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  — source for the plane and trust-boundary language this
  document freezes.

## Why this exists

Three things are already true in the repo:

1. The service topology and process placement map names ten
   service planes, binds seeded crates to them, and forbids
   blocking I/O, network, or process-launch work on the
   interaction path.
2. The boundary manifest strawman classifies capabilities against
   the desktop-local versus managed-service lane.
3. The technical architecture document lists six top-level
   logical planes and eight trust boundaries in prose.

What was missing was the join point that answers three questions
at once:

- Which of the six logical planes does a crate, module, or M0
  artifact family belong to?
- Which trust boundaries does that plane help enforce, and which
  crossings are allowed only through a typed contract?
- What must be visible to the user or a support surface when a
  crossing happens?

This document and its two companion YAML files are that join
point. They do not invent planes or boundaries; they freeze the
vocabulary the TAD already names so later tasks stop reinventing
it.

## Primary logical planes

Six planes, frozen.

| Plane | Summary | Service sub-planes rolled up |
|---|---|---|
| `interaction_plane` | Shell, editor surfaces, renderer and accessibility tree; everything the user types into or focuses. | `shell_ui`, `renderer`, `text_buffer` |
| `knowledge_plane` | Workspace identity, canonical paths, watchers, index, search, semantic graph, freshness/confidence. | `vfs_watchers`, `index_search` |
| `execution_plane` | Task, test, debug, terminal, Git, execution-context resolution, and remote-helper transport. | `task_execution`, `remote_helper` |
| `extensibility_plane` | Extension runtime hosts, automation and recipe runtimes, AI tool brokers, VS Code compat bridge. | `ai_control_plane` |
| `trust_and_control_plane` | Workspace trust, restricted mode, policy, identity, transport governance, update/provenance, safe mode, observability, support export. | `updater_release`, `support_diagnostics` |
| `optional_services_plane` | Optional managed or self-hostable services: collab relay, workspace control plane, registry/mirror, AI gateway, profile sync. | none seeded |

The `remote_helper` sub-plane rolls up under `execution_plane`
because its responsibility is the typed RPC fabric and remote
connector transport that every execution target attaches through.
Transport *governance* (policy, egress, CA trust) is part of
`trust_and_control_plane`; the transport *path* is part of
`execution_plane`. The two must not collapse.

The `ai_control_plane` sub-plane rolls up under
`extensibility_plane` because its responsibility is the AI tool
runtime and provider-routing path, not the identity/policy
decisions that govern it. Policy, entitlements, and admission
stay in `trust_and_control_plane`.

Sub-plane runtime identity, process role, scheduling class, and
inline-work policy remain authoritative in
[`process_placement_map.yaml`](../../artifacts/architecture/process_placement_map.yaml).
Do not duplicate those fields here.

## Placing every seeded crate on one plane

The acceptance bar for this document is that every seeded crate
and every major M0 artifact family resolves to exactly one
primary plane. The machine-readable placements are in
[`plane_matrix.yaml`](../../artifacts/architecture/plane_matrix.yaml);
the summary:

| Crate | Primary sub-plane | Primary plane | In production cone |
|---|---|---|---|
| `aureline-shell-spike` | `shell_ui` | interaction | no (throwaway) |
| `aureline-render` | `renderer` | interaction | yes |
| `aureline-text` | `text_buffer` | interaction | yes |
| `aureline-buffer` | `text_buffer` | interaction | yes |
| `aureline-vfs` | `vfs_watchers` | knowledge | yes |
| `aureline-rpc` | `remote_helper` | execution | yes |
| `aureline-telemetry` | `support_diagnostics` | trust-and-control | yes |
| `aureline-bench` | `support_diagnostics` | trust-and-control | no (off-cone) |
| `aureline-graph-proto` | `index_search` | knowledge | no (off-cone) |
| `aureline-largefile-proto` | `text_buffer` | interaction | no (off-cone) |
| `aureline-reactive-state` | `vfs_watchers` | knowledge | no (off-cone) |

Off-cone crates still place for review purposes, but protected
crates may not depend on them. That rule is enforced by the
[protected-path dependency rules](../../artifacts/architecture/protected_path_dependency_rules.yaml)
and
[package inventory](../../artifacts/governance/package_inventory.yaml).

## Trust-boundary matrix

Eight boundaries, each with one typed policy surface and one set
of allowed crossing forms. Full detail is in
[`trust_boundaries.yaml`](../../artifacts/architecture/trust_boundaries.yaml);
the summary:

| Boundary | Mediator (typed policy surface) | Visible disclosure |
|---|---|---|
| `local_shell_vs_external_systems` | transport governance + provider/browser handoff + remote-helper capability negotiation | service reachability, signer continuity |
| `trusted_vs_restricted_workspace` | workspace-trust state machine + restricted-mode capability gate | workspace-trust badge, preview/approval prompt |
| `core_vs_third_party_extensions` | extension manifest + publisher admission + host sandbox | capability/permission surface, signer continuity |
| `user_authored_vs_machine_derived_state` | durable-state vs cache classification + undo/reversibility contract | preview/approval prompt, support-bundle redaction |
| `local_vs_remote_execution` | execution-context model + origin/target route taxonomy + remote-connector negotiation | trust badge, service reachability, capability surface |
| `ordinary_vs_policy_defining_content` | settings resolver + policy admission + signed-artifact verification | policy allow/deny reason, signer continuity |
| `foreground_approved_vs_background_automation` | preview/approval ticket + scheduled-task admission + AI tool-use policy | preview/approval prompt, redaction posture |
| `desktop_only_vs_managed_flows` | boundary-manifest classification + identity modes + residency controls | identity-mode indicator, service reachability |

Cross-boundary rules that every crossing must respect:

- **No single prompt collapses two boundaries.** Trusting a
  workspace must not also admit a third-party extension or widen
  remote-execution trust. Each axis has its own approval.
- **Signed artifacts still pass admission.** Signature
  verification satisfies provenance, not policy. Policy-defining
  content still needs admission and audit.
- **No ambient trust inheritance.** Trust on one axis never
  widens another axis by default. Remote execution does not
  inherit local workspace trust; background automation does not
  inherit a prior foreground approval.
- **Every crossing has an audit event.** The
  support-diagnostics plane must be able to export a typed
  record for every allowed crossing. Missing events are a policy
  violation, not a logging gap.
- **Degraded state preserves visibility.** Safe mode, offline,
  mirror-only, restricted, and air-gapped modes keep the
  disclosure surfaces on every row. Degradation may narrow
  capability but never silences a badge, prompt, or reason
  string.
- **New crossing forms require an ADR.** The
  `crossing_form_vocabulary` in
  [`trust_boundaries.yaml`](../../artifacts/architecture/trust_boundaries.yaml)
  is closed. Adding a crossing form, a disclosure surface, or a
  boundary axis is an ADR-gated change.

## Cross-plane interaction rules

### Allowed runtime call directions

At the plane level, the allowed typed-request and event-stream
directions are:

| Source plane | Allowed target planes | Call form |
|---|---|---|
| `interaction_plane` | all other planes | typed request or subscription only |
| `knowledge_plane` | `execution_plane`, `trust_and_control_plane` | typed request or event stream |
| `execution_plane` | `knowledge_plane`, `extensibility_plane`, `trust_and_control_plane`, `optional_services_plane` | typed request or event stream |
| `extensibility_plane` | `knowledge_plane`, `execution_plane`, `trust_and_control_plane`, `optional_services_plane` | typed request under declared permissions |
| `trust_and_control_plane` | every plane | policy decision or audit event only |
| `optional_services_plane` | `trust_and_control_plane` | managed egress through transport governance only |

Fine-grained sub-plane call rules remain authoritative in
[`process_placement_map.yaml`](../../artifacts/architecture/process_placement_map.yaml)
under `service_planes.*.allowed_runtime_targets`. The plane-level
rows above are a summary; they do not replace the sub-plane map.

### Compile-time direction at the plane level

| Source plane | Allowed compile-time target planes |
|---|---|
| `interaction_plane` | `knowledge_plane`, `execution_plane`, `trust_and_control_plane` |
| `knowledge_plane` | `trust_and_control_plane` |
| `execution_plane` | `trust_and_control_plane` |
| `extensibility_plane` | `knowledge_plane`, `execution_plane`, `trust_and_control_plane` |
| `trust_and_control_plane` | none |
| `optional_services_plane` | `trust_and_control_plane` |

Exact crate edges still resolve through
[`package_inventory.yaml`](../../artifacts/governance/package_inventory.yaml).
Plane-class direction and hot-path sentinels resolve through the
[protected-path dependency rules](../../artifacts/architecture/protected_path_dependency_rules.yaml).

### Visibility expectations

Every plane must make certain state visible to the user or the
support-export surface when it acts. Rows are deliberately coarse
so downstream badges, banners, and support bundles compose
without inventing parallel language:

- **Interaction plane** — focus/input state, command target
  disambiguation, degraded-decoration or reduced-motion notice,
  safe-mode or restricted-workspace banner.
- **Knowledge plane** — indexing/rebuilding badge, stale/partial/
  verified truth lane, watcher or external-change state, ignore
  rule explainability.
- **Execution plane** — execution context and scope disclosure,
  remote vs local target origin, task/debug session lifecycle
  state, transport disconnect posture.
- **Extensibility plane** — extension permission scope and trust
  posture, AI provider route and policy posture, recipe or
  automation preview/approval state, ambient-privilege denial
  disclosure.
- **Trust and control plane** — workspace-trust state, policy
  allow/deny reason, update and provenance state, safe-mode
  entry reason, support-bundle redaction posture.
- **Optional services plane** — service reachability and mirror
  state, residency/region posture, narrowed claim when service
  absent.

### Forbidden implicit crossings

The following crossings must never ship silently. They are
examples, not an exhaustive list; the machine-checked rule set
lives in
[`plane_matrix.yaml`](../../artifacts/architecture/plane_matrix.yaml)
under `forbidden_implicit_crossings`:

- Shell or renderer opening files, sockets, or subprocesses
  inline instead of routing through VFS, remote-helper, or
  trust-and-control contracts.
- Any plane minting its own canonical path, file identity, save
  lane, or ignore rule outside `vfs_watchers`.
- Extension, recipe, or AI-tool runtime reaching the network
  outside `remote_helper` and transport governance.
- Extension inheriting ambient shell, workspace, or identity
  privilege that was not declared in an extension manifest or
  approval ticket.
- Any plane silently widening trust, permissions, entitlements,
  or AI/network egress through import, sync, restore, or
  extension install.
- Update check, download, install, or rollback running on the
  interaction-plane inline path.
- Archive creation, symbolication, or support-bundle assembly
  running synchronously on the shell or renderer.
- A capability classed as `local_core` or `self_host_friendly`
  hard-failing when the optional-services plane is absent.
- A plane mutating another plane's authoritative state through
  shared memory, globals, or side channels instead of the typed
  RPC, subscription, or command contract.

## How later tasks cite this map

Service-topology, authority, provider, transport, and
boundary-manifest tasks reference the files in this document set
rather than re-deriving the vocabulary:

- **Service topology and process placement** — cite
  `plane_matrix.yaml` and `trust_boundaries.yaml` for plane
  placement and crossing rules; keep inline-work policy and
  process roles authoritative in
  [`service_topology_and_process_placement.md`](service_topology_and_process_placement.md).
- **Authority projection** — cite `plane_matrix.yaml` for which
  plane owns which authoritative state. New authority rows name
  a primary sub-plane.
- **Provider and browser handoff** — cite
  `trust_boundaries.yaml` for the allowed crossing forms on the
  `local_shell_vs_external_systems` and
  `ordinary_vs_policy_defining_content` axes before minting
  provider or callback contracts.
- **Transport governance** — cite both files for the
  boundary, the enforcing service planes, and the visibility
  surfaces that every egress path must preserve.
- **Boundary manifest** — cite `trust_boundaries.yaml` for the
  `desktop_only_vs_managed_flows` axis when a row moves between
  local_core and managed-service lanes; cite `plane_matrix.yaml`
  for the primary-plane placement that the classification
  implies.

## Changing this map

Adding a primary plane, a service sub-plane, a trust-boundary
axis, a crossing form, or a visibility surface is an ADR-gated
change. Sub-plane identity, process placement, and inline-work
policy continue to evolve through the existing
service-topology/process-placement contract; this document freezes
the next-level-up vocabulary those rows compose into.
