# Subsystem Dependency Overview

This overview summarizes the allowed dependency shape behind the subsystem
contract cards. The machine-readable dependency and placement rules remain in
`artifacts/architecture/protected_path_dependency_rules.yaml`,
`artifacts/architecture/process_placement_map.yaml`, and
`artifacts/architecture/service_ownership_matrix.yaml`.

## Core Direction

| Source card | May call or consume directly | Must route through | Forbidden shortcuts |
| --- | --- | --- | --- |
| `shell_renderer` | `command_plane`, `editor_buffer_text`, `settings_effective_configuration`, bounded diagnostics | `workspace_vfs_persistence`, `search_navigation_graph`, `execution_context_tooling`, `ai_context_runtime`, `remote_collaboration_runtime` | raw filesystem, network, process launch, hidden AI/provider calls |
| `editor_buffer_text` | text primitives, renderer layout inputs, undo/source-fidelity artifacts | VFS saves, search/index updates, diagnostics, Git, task/test/debug | direct provider calls, direct VCS/build/test actions, direct remote transport |
| `workspace_vfs_persistence` | filesystem adapters, watcher feeds, save envelopes, policy/trust overlays | shell rendering, search/index, execution helpers, support export | feature-local path canonicalization, hidden writes, shell-inline watch bootstrap |
| `command_plane` | settings resolver, policy broker, route truth, service registry | task/test/debug, Git, AI, remote, updater, support actions | feature-private command ids or invocation packets |
| `rpc_reactive_truth` | schemas, trace/cancel/deadline vocabulary, subscription envelopes | service-specific payload contracts | untyped shared mutable state or packet-local event envelopes |
| `search_navigation_graph` | VFS identity, graph/index shards, docs/index feeds | shell UI, task execution, AI context, support export | direct UI mutation or language-server-owned ranking truth |
| `execution_context_tooling` | environment resolver, terminal/debug/Git helpers, target discovery | command plane, policy broker, route truth, support export | ad hoc shell process launch or task state without provenance |
| `settings_effective_configuration` | setting registry, precedence rows, policy locks | shell UI, command plane, support export | silently widening permissions through user/workspace settings |
| `docs_public_truth` | docs-pack records, Help/About route rows, stale-example policy | exact-build identity, command ids, record classes | rendering stale or blocked docs as authoritative |
| `release_build_identity` | provenance, artifact graph, clean-room rebuild, signing posture | docs/help, support, benchmark, security, release evidence | version-only fallback where an exact identity ref is required |
| `quality_benchmark_claims` | protected paths, metrics, corpus, evidence freshness | implementation card owners, release evidence, claim manifest | claim broadening from seed dashboards or stale evidence |
| `design_attention_embedded` | tokens, component states, activity/job rows, embedded-boundary schemas | command ids, route truth, accessibility review lanes | toast-only truth for durable work or hidden owner/origin chrome |
| `security_policy_support` | policy/trust, support bundle, object handoff, redaction posture | record classes, exact build, execution context, route truth | hidden privilege escalation, silent telemetry widening, raw secret export |
| `governance_ownership_lifecycle` | ownership matrix, package inventory, lifecycle/deprecation policy | all stable-facing cards | ownerless protected work or private deprecation rules |
| `record_class_governance` | record-class registry, retention/export/delete/hold vocabulary | support, AI, collaboration, release, docs | private retention/export labels |
| `ai_context_runtime` | context broker, tool broker, provider policy, evidence packets | command plane, graph/search, route truth, record classes | raw workspace crawl, hidden edits, direct credential access |
| `extension_ecosystem_lifecycle` | capability broker, permission records, host worlds, budget rows | settings, policy, route truth, lifecycle policy | ambient file/network powers or hidden first-party privilege |
| `remote_collaboration_runtime` | transport governance, capability negotiation, session authority | VFS/tooling contracts, route truth, support export | alternate file/network semantics exposed to the shell |

## Failure Containment

| Failure family | First card to contain it | Required visible outcome |
| --- | --- | --- |
| Shell hot-path stall | `shell_renderer` | Defer non-critical panes, preserve typing and command entry, emit protected-path evidence. |
| Text corruption or undo drift | `editor_buffer_text` | Stop widening edits, preserve recovery journal, require source-fidelity proof before claim broadening. |
| Watcher loss or save conflict | `workspace_vfs_persistence` | Mark watcher degraded, use save-token or merge-review path, never silently overwrite. |
| Missing command or disabled action | `command_plane` | Show typed disabled reason or lifecycle/deprecation state instead of hiding the action. |
| Lost RPC continuity | `rpc_reactive_truth` | Cancel, resync, or fail closed with trace id and subscription state. |
| Stale search or graph answer | `search_navigation_graph` | Surface freshness/confidence labels; do not claim exactness. |
| Wrong target or environment | `execution_context_tooling` | Block or reapprove with route/target context and support-exportable evidence. |
| Policy or setting conflict | `settings_effective_configuration` | Resolve through precedence, lock, alias, or denial reason; never mutate around policy. |
| Stale docs or build mismatch | `docs_public_truth` and `release_build_identity` | Downgrade Help/About/service-health truth until exact identity and freshness join. |
| Benchmark evidence stale | `quality_benchmark_claims` | Mark claim yellow or seed-only; rerun or narrow before promotion. |
| Record retention/export ambiguity | `record_class_governance` | Refuse export/delete/support claim until registry id and posture resolve. |
| AI egress or hidden mutation risk | `ai_context_runtime` | Degrade to explain-only, local-only, or disabled with audit reason. |
| Extension crash or capability mismatch | `extension_ecosystem_lifecycle` | Quarantine or disable the package/host without destabilizing the shell. |
| Remote disconnect or collaboration partition | `remote_collaboration_runtime` | Keep local shell and editing usable; mark remote/session state degraded and resumable. |

## Review Joins

| Review question | Primary artifacts |
| --- | --- |
| Is this dependency allowed? | `artifacts/architecture/protected_path_dependency_rules.yaml`, card `dependencies` fields |
| Where does this work run? | `artifacts/architecture/process_placement_map.yaml`, `artifacts/architecture/service_ownership_matrix.yaml` |
| Who owns it? | `artifacts/governance/ownership_matrix.yaml`, card `owners` fields |
| Is the surface frozen or provisional? | `artifacts/governance/interface_freeze_matrix.yaml`, card `freeze_status` fields |
| What proof is required? | card `proof_packets`, `artifacts/perf/protected_path_ledger.yaml`, `artifacts/bench/fitness_function_catalog.yaml`, signoff packet evidence rows |
| What changes with a stable-facing break? | `docs/governance/interface_lifecycle_policy.md`, `artifacts/contracts/frozen_surface_manifest.yaml` |

