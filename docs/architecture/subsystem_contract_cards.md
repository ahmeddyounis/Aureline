# Subsystem Contract Cards

This pack turns the architecture-freeze prose into a reviewable card set.
Each card is a bounded implementation contract: what the subsystem owns,
what it may call, what it must never bypass, how it degrades, which proof
packets are required, and which gaps remain explicit.

Machine-readable cards live in
`artifacts/architecture/subsystem_contract_cards/`. The dependency overview
is in `docs/architecture/subsystem_dependency_overview.md`.

## How To Cite

Use the card id plus a field name when planning or implementing a downstream
slice. For example:

- `card:shell_renderer owned_objects`
- `card:workspace_vfs_persistence failure_modes`
- `card:release_build_identity shared_contract_refs`

Do not cite a card as a replacement for an ADR, schema, benchmark packet, or
scorecard row. The card is the router and reviewer checklist. The underlying
ADR, schema, fixture, or proof packet remains authoritative for detailed API
shape and exact validation.

## Card Shape

Every YAML card uses these required fields:

| Field | Meaning |
| --- | --- |
| `card_id` | Stable citation id for the subsystem card. |
| `title` | Human-readable subsystem name. |
| `freeze_status` | Current implementation posture: `frozen`, `provisional`, `seeded`, or `blocked`. Mixed cards use the narrowest honest status and list frozen obligations separately. |
| `purpose` | Why the subsystem exists and which product truth it protects. |
| `owned_objects` | Objects, records, services, state, or artifacts this subsystem owns. |
| `non_owned_objects` | Adjacent objects this subsystem must not silently own or mutate. |
| `entry_points` | Command ids, event frames, schemas, route rows, or packet families that enter the subsystem. |
| `dependencies` | Allowed direct dependencies, required indirect routing, and forbidden shortcuts. |
| `budgets` | Latency, resource, restart, evidence, or review budgets the card must preserve. |
| `failure_modes` | Named failure or degraded states plus required behavior. |
| `proof_packets` | Artifacts, fixtures, schemas, benchmark rows, or checkers required before claims widen. |
| `owners` | DRI, lane, backup posture, and review forums. |
| `shared_contract_refs` | Cross-card references for command ids, route truth, exact-build identity, record classes, settings ids, and lifecycle/deprecation posture. |
| `explicit_gaps` | Known gaps. A blank gap list means the card claims no open gaps at this review level. |

## Card Index

| Card | Scope | Freeze status | Primary refs | Current explicit gaps |
| --- | --- | --- | --- | --- |
| `shell_renderer` | Shell, renderer, hot interaction path | `provisional` | renderer ADR, process placement, protected-path ledgers | permanent shell home, keyboard-complete evidence, native-window closure |
| `editor_buffer_text` | Text primitives, buffer, undo, large-file behavior | `frozen` | buffer ADR, undo rows, source-fidelity fixtures | product compare/restore surface proof is later scope |
| `workspace_vfs_persistence` | Workspace identity, VFS, saves, watchers, restore | `provisional` | VFS ADR, mutation-lineage model, save examples | local-history surfaces and release-grade recovery corpus |
| `command_plane` | Command descriptors, invocation sessions, UI/CLI/AI routing | `frozen` | command contract, command schemas, command registry seed | keyboard-complete graph and stable runtime parity |
| `rpc_reactive_truth` | Typed RPC, event envelopes, subscription invalidation | `frozen` | RPC ADR, subscription ADR, schemas | production transport implementation still follows the frozen envelopes |
| `search_navigation_graph` | Search truth, ranking, indexing, semantic graph | `frozen` | search ADR, result-truth labels, graph seeds | graph implementation and full relevance evidence |
| `execution_context_tooling` | Execution context, tasks, terminal, debug, target truth | `provisional` | execution-context ADR, runtime vocabulary, context examples | task/test/debug event packets are not fully unified yet |
| `settings_effective_configuration` | Setting ids, precedence, effective values | `frozen` | settings ADR, settings schemas, scope rows | UI/CLI/support projections must still implement the contract |
| `docs_public_truth` | Docs packs, Help/About, service health, public truth | `provisional` | docs-truth ADR, docs-pack contract, stale-example rules | exact-build joins and stale-example enforcement remain yellow |
| `release_build_identity` | Exact-build identity, provenance, release evidence | `provisional` | exact-build model, release evidence template, artifact graph | clean-room parity and symbolication closure |
| `quality_benchmark_claims` | Benchmark lab, protected metrics, certification, claim proof | `provisional` | protected metrics, fitness catalog, protected paths | council-approved hardware baseline |
| `design_attention_embedded` | Design tokens, component states, durable attention, embedded boundaries | `frozen` | design vocabulary, attention taxonomy, embedded-boundary ADR | accessibility/locale claim proof remains separate and blocked from green |
| `security_policy_support` | Trust, policy, secret posture, security intake, support export | `provisional` | severity matrix, support bundle, object handoff, secret ADR | live incident routing and monitored contact path |
| `governance_ownership_lifecycle` | Ownership, protected dependency rules, lifecycle/deprecation policy | `provisional` | ownership matrix, package inventory, lifecycle policy | backup-owner waiver and permanent shell home |
| `record_class_governance` | Record classes, retention, export, delete, hold posture | `frozen` | record-class registry, governance docs, schemas | downstream managed-copy rows must keep citing registry ids |
| `ai_context_runtime` | AI context, provider routing, evidence, budget, taint fences | `provisional` | AI context contract, provider registry, model budget docs | language/provider arbitration and broad agent policy |
| `extension_ecosystem_lifecycle` | Extension permissions, host worlds, lifecycle, budgets | `provisional` | extension ADRs, capability worlds, runtime budget rows | publication pipeline, SDK support window, registry moderation |
| `remote_collaboration_runtime` | Remote attach, route truth, collaboration session authority | `provisional` | remote-agent ADR, route taxonomy, collaboration contracts | relay/service implementation and hosted review policy |

## Shared Contract Map

| Shared contract | Owning card | Consumers |
| --- | --- | --- |
| Exact-build identity fields | `release_build_identity` | `docs_public_truth`, `quality_benchmark_claims`, `security_policy_support`, `remote_collaboration_runtime` |
| Route, origin, target, exposure truth | `execution_context_tooling` | `command_plane`, `search_navigation_graph`, `remote_collaboration_runtime`, `ai_context_runtime`, `security_policy_support` |
| Record classes and retention/export posture | `record_class_governance` | `security_policy_support`, `docs_public_truth`, `ai_context_runtime`, `remote_collaboration_runtime` |
| Stable setting ids and effective values | `settings_effective_configuration` | `shell_renderer`, `command_plane`, `security_policy_support`, `extension_ecosystem_lifecycle` |
| Command ids and invocation sessions | `command_plane` | `shell_renderer`, `execution_context_tooling`, `ai_context_runtime`, `docs_public_truth` |
| Lifecycle and deprecation posture | `governance_ownership_lifecycle` | all cards that publish stable ids, schemas, command ids, or support windows |
| RPC and subscription envelopes | `rpc_reactive_truth` | `shell_renderer`, `workspace_vfs_persistence`, `search_navigation_graph`, `execution_context_tooling`, `remote_collaboration_runtime` |

## Reviewer Guide

Review cards in this order:

1. Read `docs/architecture/subsystem_dependency_overview.md` to see allowed
   cross-card edges and failure-containment boundaries.
2. For an implementation slice, open the relevant YAML card and check
   `owned_objects`, `dependencies`, `failure_modes`, and `proof_packets`.
3. Follow `freeze_basis_refs` to the ADRs, schemas, fixtures, and benchmark
   rows that carry the normative detail.
4. Check `explicit_gaps` before widening any claim. A gap means the card is
   still usable only inside the named narrow posture.
5. Compare `owners` against `artifacts/governance/ownership_matrix.yaml`
   and current scorecard rows before treating the subsystem as green.

Cards relate to other artifacts as follows:

- ADRs explain the accepted decision and tradeoff. Cards summarize the
  implementation contract that downstream work should cite.
- Schemas define exact packet or record shape. Cards name which schemas cross
  the subsystem boundary and what happens when a reader cannot satisfy them.
- Benchmark and proof packets show whether the card is currently evidenced.
  Cards do not convert seed proof into a stable claim.
- Milestone scorecards report current review posture. Cards make the scope
  behind each scorecard lane inspectable.
- Frozen-surface and lifecycle policies control breaking changes after a
  stable-facing contract exists. Cards point to those policies and must not
  carry private deprecation rules.

