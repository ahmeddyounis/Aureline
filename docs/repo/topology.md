# Repository topology

This document is the authoritative map of where things live in the Aureline
repository. It is normative for path expectations: future tooling, governance
checks, and CI gates may consume it. Move-don't-fork: when a directory needs
to change, update this map in the same change.

## Top-level layout

| Path           | Purpose                                                                                              |
|----------------|------------------------------------------------------------------------------------------------------|
| `Cargo.toml`   | Root Cargo workspace manifest. Lists every internal crate.                                           |
| `CODEOWNERS`   | Pull-request review routing. Paired with `artifacts/governance/ownership_matrix.yaml` for ownership. |
| `crates/`      | All Rust crates. One directory per crate; crate name matches directory name.                        |
| `docs/`        | Design and governance docs that ship with the repository (not external product docs).                |
| `schemas/`     | Machine-readable schemas (JSON Schema, protobuf, etc.) consumed by tooling and runtime.              |
| `fixtures/`    | Reusable test inputs and golden artifacts. Subtrees grow per protected-path corpus.                  |
| `tools/`       | Repository-local tooling (lint helpers, codegen scripts, governance checkers).                       |
| `ci/`          | CI configuration shared across pipelines (job definitions, gate scripts).                            |
| `artifacts/`   | Checked-in evidence and governance outputs. Subtrees: `architecture/`, `governance/`, `compat/`, `evidence/`, `io/`, `release/`, `platform/`, `qa/`, `ux/`, `accessibility/`, `support/`, `startup/`.  |

## Reserved subtrees inside `artifacts/`

| Path                   | Purpose                                                                              |
|------------------------|--------------------------------------------------------------------------------------|
| `artifacts/governance/`| Package inventory, ownership matrix, requirement/waiver registers, scorecard/packet templates, and public-truth claim/parity seeds. |
| `artifacts/architecture/` | Service-topology maps, protected-path dependency rules, process-placement seeds, and tradeoff registers. |
| `artifacts/compat/`    | Compatibility qualification matrix, version-skew register, and boundary-row seeds.   |
| `artifacts/evidence/`  | Shared evidence metadata catalogs and freshness field vocabularies used by release-facing packets. |
| `artifacts/io/`        | Save-path, source-fidelity, rewrite-class, and root-capability artifacts for editor and VFS truth contracts. |
| `artifacts/release/`   | Release-artifact graph rules, provenance, SBOMs, compatibility reports, claim manifests, rollback packets. |
| `artifacts/platform/`  | Claimed desktop profile registry and platform-owned primitive matrices that bind desktop claims to named OS/profile rows. |
| `artifacts/qa/`        | Seed QA verification matrices and claimed-profile continuity drill vocabularies. |
| `artifacts/ux/`        | Design-system snapshots and UX review packets.                    |
| `artifacts/accessibility/` | Accessibility and platform-input matrices, shell-conformance checklists, and accessibility-tree coverage rows. |
| `artifacts/support/`   | Support-bundle templates, recovery drill outputs, Project Doctor seeded scenarios.   |
| `artifacts/startup/`   | Startup ordering packets, admission-order seeds, and shell-ready-before-full-graph proof inputs. |

The `artifacts/compat/` subtree is the pre-release compatibility seed
home. Later release-time compatibility reports still land under
`artifacts/release/`; they extend the seeded row ids rather than
replacing them.

Other tasks may land additional subtrees; they extend this map rather
than relocating it.

## Workspace crates

This table is generated from the Cargo workspace and the reviewed governance
classification map. The package inventory remains the machine-readable source
for protected posture, work-package ownership, and exact allowed edges.

| Crate | Path | Layer | Dependency class | Protected | Role |
|---|---|---|---|---|---|
| `aureline-activity` | `crates/aureline-activity/` | L0 | `shell_ui` | yes | Notification-envelope, activity-object, badge-aggregate, fanout-receipt, and attention-routing contracts. |
| `aureline-ai` | `crates/aureline-ai/` | L10 | `ai_control_plane` | yes | AI composer and context-inspector seed for the bounded launch AI wedge. |
| `aureline-api` | `crates/aureline-api/` | L6 | `remote_helper` | yes | Versioned request-workspace documents, environment sets, and auth-source inspector contracts. |
| `aureline-auth` | `crates/aureline-auth/` | L5 | `ai_control_plane` | yes | System-browser auth callback seed and local-versus-managed shell vocabulary. |
| `aureline-bench` | `crates/aureline-bench/` | LX | `off_cone` | no | Benchmark harness and protected-path trace fixtures. |
| `aureline-buffer` | `crates/aureline-buffer/` | L1 | `text_buffer` | yes | Editor buffer core: piece-tree storage, selections, undo/redo. |
| `aureline-build-farm` | `crates/aureline-build-farm/` | L0 | `support_diagnostics` | yes | Metadata-only build-farm trust-domain and provenance-chain model. |
| `aureline-build-info` | `crates/aureline-build-info/` | L0 | `support_diagnostics` | yes | Build identity and exact-build identity helpers for runtime, support exports, and provenance-facing stubs. |
| `aureline-capabilities` | `crates/aureline-capabilities/` | L0 | `shell_ui` | yes | Capability records and artifact dependency markers for settings, profiles, workflow bundles, portable-state packages, recipes, saved views, migration packets, support exports, and sync artifacts. |
| `aureline-change-objects` | `crates/aureline-change-objects/` | L0 | `support_diagnostics` | yes | Portable change-object contracts for offline review and support handoff. |
| `aureline-chronology` | `crates/aureline-chronology/` | L0 | `support_diagnostics` | yes | Canonical chronology grammar and export-safe history rows. |
| `aureline-cli` | `crates/aureline-cli/` | L13 | `shell_ui` | yes | CLI/headless schema stabilization, machine-readable output contracts, and support/export compatibility promises. |
| `aureline-collab` | `crates/aureline-collab/` | L0 | `remote_helper` | yes | Collaboration session envelopes, role admission truth, and retention/export qualification contracts. |
| `aureline-collections` | `crates/aureline-collections/` | L10 | `index_search` | yes | Stable dense-collection contracts for filters, saved views, scope counters, and batch review. |
| `aureline-commands` | `crates/aureline-commands/` | L0 | `shell_ui` | yes | Canonical command descriptor schema and runtime registry. |
| `aureline-companion` | `crates/aureline-companion/` | L9 | `remote_helper` | yes | Frozen M5 companion, incident, sync, residency, and offboarding matrix truth packet with staged rollout lanes for companion, incident, support, diagnostics, and Help/About surfaces. |
| `aureline-config` | `crates/aureline-config/` | L0 | `shell_ui` | yes | Structured config, manifest, environment-file, and effective/live truth contracts. |
| `aureline-content-safety` | `crates/aureline-content-safety/` | L0 | `text_buffer` | yes | Shared suspicious-content detector and representation-labeled transfer records for safe preview. |
| `aureline-continuity` | `crates/aureline-continuity/` | L0 | `support_diagnostics` | yes | Connectivity state, deferred intent, and reconciliation contracts. |
| `aureline-crash` | `crates/aureline-crash/` | L0 | `support_diagnostics` | yes | Crash incident trails joining crash envelopes, exact-build symbolication, and support bundle refs. |
| `aureline-data` | `crates/aureline-data/` | L6 | `ai_control_plane` | yes | Local-first data and experiment provenance contracts for notebook-adjacent result workflows. |
| `aureline-debug` | `crates/aureline-debug/` | L0 | `task_execution` | yes | Debug-session chronology, replay support class truth, and capability descriptor contracts for local, remote/helper, container, and notebook-bridge debug lanes. |
| `aureline-deps` | `crates/aureline-deps/` | L0 | `support_diagnostics` | yes | Dependency, security, compliance, and export-truth types for advisory, license, suppression, SBOM, and lockfile-risk surfaces. |
| `aureline-design-system` | `crates/aureline-design-system/` | L1 | `shell_ui` | yes | Governed design-system beta contracts for component state, appearance, screenshot diff, and token conformance. |
| `aureline-docs` | `crates/aureline-docs/` | L0 | `index_search` | yes | Docs-node identity and citation evidence primitives for docs, help, explainers, onboarding, and AI. |
| `aureline-doctor` | `crates/aureline-doctor/` | L1 | `support_diagnostics` | yes | Read-only Project Doctor alpha probes and support/export projections. |
| `aureline-ecosystem` | `crates/aureline-ecosystem/` | L0 | `updater_release` | yes | Shared ecosystem compatibility scorecards and cross-surface claim projections. |
| `aureline-editor` | `crates/aureline-editor/` | L13 | `shell_ui` | yes | Editor viewport model, compositor, and paint pipeline. |
| `aureline-env` | `crates/aureline-env/` | LX | `off_cone` | no | Environment-capsule, workspace-template, prebuild-fingerprint, and runtime-materialization governance. Freezes one typed, inspectable matrix that certifies environment-capsule truth per claimed M5 template/starter/prebuild/devcontainer/remote/managed profile, with one narrowing engine that downgrades stale or partial evidence, narrows warm-start reuse when a prebuild outruns its source digest, and withholds unproven dimensions. |
| `aureline-execution` | `crates/aureline-execution/` | L0 | `task_execution` | yes | Canonical M5 build-intelligence, host-boundary, and managed-workspace execution-truth matrix. |
| `aureline-extensions` | `crates/aureline-extensions/` | L10 | `updater_release` | yes | Extension-manifest baseline, effective-permission summary, and install / review decision validator for the first ecosystem-bearing lane. |
| `aureline-framework` | `crates/aureline-framework/` | L0 | `support_diagnostics` | yes | Framework-aware tooling support strips, exact-vs-heuristic row certainty, convention-diagnostic rows, and review-first generator/codemod previews. |
| `aureline-generated` | `crates/aureline-generated/` | L0 | `support_diagnostics` | yes | Generated-artifact provenance, regeneration, writable-boundary, and reversible-checkpoint governance. Freezes one typed, inspectable matrix that certifies generated-artifact truth per claimed M5 artifact class, with one narrowing engine that downgrades stale or partial evidence, narrows the writable-boundary posture toward a reviewed override or regenerate-only when canonical-source or boundary evidence outruns proof, and withholds claims whose canonical source, regeneration route, drift state, or checkpoint lineage cannot be proven. |
| `aureline-git` | `crates/aureline-git/` | L3 | `task_execution` | yes | Git service alpha for repository status, branch identity, and launch-wedge change discovery. |
| `aureline-governance` | `crates/aureline-governance/` | L0 | `support_diagnostics` | yes | Governed schema and record-class registry access. |
| `aureline-graph` | `crates/aureline-graph/` | L1 | `index_search` | yes | Semantic graph storage and alpha query-family runtime for launch-wedge navigation. |
| `aureline-graph-proto` | `crates/aureline-graph-proto/` | L0 | `index_search` | yes | Semantic-workspace-graph seed prototype. Mirrors the node-class, edge-class, evidence-state, provenance, freshness, confidence, query-family, shard-affinity, invalidation-producer, topology-edge, impact-reason, and explainer-citation vocabularies frozen in docs/graph/workspace_graph_seed.md and schemas/graph/workspace_graph_seed.schema.json, and enforces the identity / label rules the doc names against the fixtures under fixtures/graph/example_workspace_graphs/. |
| `aureline-graph-ui` | `crates/aureline-graph-ui/` | L2 | `index_search` | yes | Graph understanding surface projections for topology, impact, and cited explainers. |
| `aureline-history` | `crates/aureline-history/` | L2 | `text_buffer` | yes | Local-history checkpoints and unified mutation-journal persistence. |
| `aureline-i18n` | `crates/aureline-i18n/` | L0 | `support_diagnostics` | yes | Localization message identity, locale-pack governance, fallback inspection, and support-export projections. |
| `aureline-incident` | `crates/aureline-incident/` | L9 | `support_diagnostics` | yes | Incident workspace and runbook packet alpha state projected through redacted support exports. |
| `aureline-infra` | `crates/aureline-infra/` | L6 | `ai_control_plane` | yes | Infrastructure target-context, connector-class, and control-plane boundary qualification packets. |
| `aureline-input` | `crates/aureline-input/` | L1 | `shell_ui` | yes | Input modeling and deterministic keybinding resolution for shell surfaces. |
| `aureline-install` | `crates/aureline-install/` | L0 | `updater_release` | yes | Install-topology alpha contract for channel, state-root, handler, silent deployment, and support-export truth. |
| `aureline-language` | `crates/aureline-language/` | L2 | `index_search` | yes | Tree-sitter grammar registry and parser lifecycle runtime for launch-language syntax. |
| `aureline-largefile-proto` | `crates/aureline-largefile-proto/` | LX | `off_cone` | no | Prototype large-file path: paged reader, classification, and limited-mode capability split. Validates the ADR 0003 large-file mode without poisoning the normal piece-tree buffer. |
| `aureline-learning` | `crates/aureline-learning/` | L1 | `shell_ui` | yes | Qualification layer for learning-mode surfaces, guided tours, exercise rails, glossary packs, and teaching-session flows. |
| `aureline-navigation` | `crates/aureline-navigation/` | L0 | `index_search` | yes | Typed navigation target, reference, hierarchy, rename-preview, and continuity contracts. |
| `aureline-notebook` | `crates/aureline-notebook/` | L0 | `task_execution` | yes | Retained notebook preview runtime-truth model: kernel/session/output truth, restart/reconnect review, variable-explorer freshness, rich-output trust classes, debugger-bridge state, round-trip fixtures, heavy-output corpora, share and handoff sheets with scope separation, and canonical support packet. |
| `aureline-notices` | `crates/aureline-notices/` | L0 | `support_diagnostics` | yes | Typed notice, SBOM, and critical-upstream projections for repository compliance. |
| `aureline-policy` | `crates/aureline-policy/` | L6 | `ai_control_plane` | yes | Policy simulation, exception lifecycle, and remembered-decision contracts. |
| `aureline-preview` | `crates/aureline-preview/` | L1 | `shell_ui` | yes | Representation-labeled safe-preview and copy/export wedge for risky text, oversized artifacts, and generated content. |
| `aureline-profiler` | `crates/aureline-profiler/` | L8 | `support_diagnostics` | yes | Profile launcher, attach sheets, capture-mode descriptors, and storage-location truth for profiler and trace surfaces. |
| `aureline-provider` | `crates/aureline-provider/` | L8 | `ai_control_plane` | yes | Connected-provider registry alpha for external provider descriptors, publish-later queue truth, and CI overlay disclosures. |
| `aureline-qe` | `crates/aureline-qe/` | L15 | `support_diagnostics` | yes | Conformance and failure / recovery drill harnesses for Aureline beta surfaces. |
| `aureline-reactive-state` | `crates/aureline-reactive-state/` | L0 | `index_search` | yes | Reactive state and subscription-envelope prototype. Validates the ADR 0005 subscription envelope, lifecycle, freshness / completeness / stale-reason vocabulary, authority / derivation split, materialized-view classes, and protected-hot-path hooks against a frozen scenario table with byte-stable invalidation traces. |
| `aureline-records` | `crates/aureline-records/` | L0 | `support_diagnostics` | yes | Typed record-class registry loader and record-kind validation. |
| `aureline-recovery` | `crates/aureline-recovery/` | L12 | `support_diagnostics` | yes | Crash journals, dirty-buffer recovery, and session-restore skeleton persistence. |
| `aureline-release` | `crates/aureline-release/` | L12 | `updater_release` | yes | Release-center object model for candidates, publish targets, promotion history, rollback, and revocation. |
| `aureline-remote` | `crates/aureline-remote/` | L6 | `remote_helper` | yes | Governed route objects, exposure-review sheets, and revocation truth for port-forward, tunnel, preview-route, and exposed-service rows. |
| `aureline-render` | `crates/aureline-render/` | L1 | `renderer` | yes | GPU-accelerated rendering primitives for the desktop shell. |
| `aureline-review` | `crates/aureline-review/` | L10 | `task_execution` | yes | Review and diff surface contracts for launch-wedge local Git review. |
| `aureline-rpc` | `crates/aureline-rpc/` | L0 | `remote_helper` | yes | In-process and cross-process RPC transport for the supervisor and service fabric. |
| `aureline-runbooks` | `crates/aureline-runbooks/` | L0 | `support_diagnostics` | yes | Governed runbook object model freezing source classes, executable step classes, deviation lineage, control-plane handoff, and archival/export truth across claimed incident/operator surfaces. |
| `aureline-runtime` | `crates/aureline-runtime/` | L4 | `task_execution` | yes | Execution-context object model and resolver seed shared by terminal, task, and debug-prep lanes. |
| `aureline-scaffold` | `crates/aureline-scaffold/` | L0 | `updater_release` | yes | Stable scaffold manifest, preflight, health, and generated-project lineage contracts. |
| `aureline-search` | `crates/aureline-search/` | L9 | `index_search` | yes | Workspace search foundations: lexical filename/path search shell with scope and partiality truth. |
| `aureline-service` | `crates/aureline-service/` | LX | `off_cone` | no | Frozen commercial-control-plane truth for managed lanes: meter families, entitlement and managed-state vocabulary, chargeback-scope ownership, forecast thresholds, grace-period rights, org-switch semantics, and the local-safe baseline every managed lane preserves. |
| `aureline-service-health` | `crates/aureline-service-health/` | L9 | `support_diagnostics` | yes | Scheduled maintenance, read-only/drain windows, tenant migration/failover communication, and publish-later/local-draft continuity for managed and provider-linked surfaces. |
| `aureline-service-health-feed` | `crates/aureline-service-health-feed/` | L0 | `support_diagnostics` | yes | Shared service-health feed contract for desktop, headless, Help/About, diagnostics, and support export surfaces. |
| `aureline-settings` | `crates/aureline-settings/` | L11 | `shell_ui` | yes | Effective-settings schema registry, precedence engine, and locked-write flow. |
| `aureline-shell` | `crates/aureline-shell/` | L14 | `shell_ui` | yes | Desktop shell: canonical zone registry and live desktop frame. |
| `aureline-shell-spike` | `crates/aureline-shell-spike/` | LX | `off_cone` | no | Throwaway integration spike for the desktop shell, renderer, and input loop. Replaced before stable. |
| `aureline-support` | `crates/aureline-support/` | L7 | `support_diagnostics` | yes | Support-bundle manifest, redaction defaults, local preview, and exact-build capture for the live shell. |
| `aureline-telemetry` | `crates/aureline-telemetry/` | L0 | `support_diagnostics` | yes | Hot-path instrumentation, tracing, and metrics primitives. |
| `aureline-templates` | `crates/aureline-templates/` | L0 | `updater_release` | yes | Signed template registry, provenance/mirror, and template-health truth packets for the template gallery and scaffold preflight. |
| `aureline-terminal` | `crates/aureline-terminal/` | L4 | `task_execution` | yes | Terminal foundation: PTY host abstraction and session-header truth. |
| `aureline-text` | `crates/aureline-text/` | L0 | `text_buffer` | yes | Foundational text primitives: encoding, segmentation, shaping inputs. |
| `aureline-ui` | `crates/aureline-ui/` | L0 | `shell_ui` | yes | Shared UI primitives: semantic tokens, appearance state, and design-system contracts. |
| `aureline-vfs` | `crates/aureline-vfs/` | L0 | `vfs_watchers` | yes | Virtual filesystem, watcher, canonical-path, alias-set, and save-target prototype. Validates the ADR 0006 filesystem-identity, watcher, and save-pipeline contracts against a synthetic-root fixture table. |
| `aureline-workspace` | `crates/aureline-workspace/` | L3 | `shell_ui` | yes | Workspace entry vocabulary and recent-work registry. |

## Layering at a glance

Active crates occupy topological layers `L0` through the current workspace
maximum; every production/build edge points to a lower-numbered layer. `LX`
is reserved for off-cone crates and may never become a production dependency
without an explicit promotion that updates all governance artifacts together.

Exact edges and layer membership are published in
[`dependency_rules.md`](./dependency_rules.md). Service-plane placement and
class-level direction are published in
[`protected_path_dependency_rules.yaml`](../../artifacts/architecture/protected_path_dependency_rules.yaml).
## Product boundary

Every crate above is on the local-core side of the open-source core versus
managed / service-plane boundary. The boundary is drawn explicitly in
[`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
and conforms to
[`/schemas/product/boundary_manifest.schema.json`](../../schemas/product/boundary_manifest.schema.json).
When a new crate, service, or managed dependency is added, it must map to an
existing boundary-manifest row or land a new row in the same change;
introducing a capability without a boundary row is a governance error.
