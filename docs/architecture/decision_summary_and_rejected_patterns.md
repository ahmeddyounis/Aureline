# Architecture decision summary, rejected patterns, and revisit triggers

This document is a reviewer-facing entrypoint for the current architecture posture: what is **selected**, what is **explicitly rejected**, and what concrete evidence is required to **reopen** a decision.

It is intentionally shorter than the full source documents; when in doubt, defer to the canonical sources linked below.

## Source of truth

Primary sources:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — decision summary (4.2) and explicitly rejected patterns (4.4).
- `artifacts/governance/decision_index.yaml` — decision status, owners, forums, and linked ADRs (authoritative for governance metadata).

Companion machine artifacts introduced by this change:

- `artifacts/architecture/rejected_pattern_rows.yaml` — rejected-pattern ledger with governing refs and reopen evidence requirements.
- `artifacts/architecture/revisit_trigger_matrix.yaml` — concrete revisit triggers and the required evidence packet to reopen a decision.

## How to use this page (review workflow)

1. **Start from the proposal.** Identify which decision area it touches (renderer, buffer, VFS, transport, extension runtime, remote, AI, release, etc.).
2. **Check the decision summary** below for the selected posture and the governing ADR/contract refs.
3. **If the proposal conflicts with an explicitly rejected pattern**, locate the matching `rp.*` row in `artifacts/architecture/rejected_pattern_rows.yaml` and use it as the review anchor.
4. **If the proposer claims “we can revisit later,”** require a concrete trigger from `artifacts/architecture/revisit_trigger_matrix.yaml` (or add one in the same change).
5. **To actually change posture**, reopen through the decision forum and artifact class named by the relevant revisit trigger row (typically an ADR plus a benchmark/security/compat packet).

## Decision summary (selected posture + governing refs)

The table below summarizes the baseline posture. Each row includes at least one governing ADR, contract, or register row so the “chosen” posture is never an unlinked slogan.

| Area | Selected posture | Governing refs (non-exhaustive) |
|---|---|---|
| Shell & renderer | Native desktop shell and renderer; shell interaction path stays local and must not block on filesystem/network/process work. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2, 4.4), `docs/adr/0002-renderer-text-stack-and-shaping-fallback.md` (D-0001), `docs/adr/0016-shell-windowing-input-accessibility-boundary.md` (D-0022), `artifacts/architecture/renderer_tradeoff_rows.yaml` |
| Editor core (text model) | Piece-table / piece-tree core text model with explicit undo classes and a separate large-file path. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2), `docs/adr/0003-buffer-undo-large-file.md` (D-0002), `artifacts/architecture/undo_class_rows.yaml` |
| Save & source fidelity | Durable state is human-readable and recoverable; derived state is disposable; structured formats must preserve unknown/vendor metadata on round-trip. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2, 4.4), `docs/adr/0006-vfs-save-cache-identity.md` (D-0003), `docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`, `docs/verification/text_fidelity_packet.md` |
| Workspace VFS & identity | Stable, inspectable workspace + path identity; watcher model is explicit; “mutable stable ids” are out of bounds for durable objects and deep links. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (3.x, 4.2), `docs/adr/0006-vfs-save-cache-identity.md` (D-0003), `artifacts/architecture/vfs_tradeoff_rows.yaml` |
| Process/service topology | Local supervisor + supervised worker/host topology; isolation and restart classes are explicit; optional helpers and managed boundaries must not block core editing. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (8.x), `docs/architecture/process_topology.md`, `artifacts/architecture/runtime_host_classes.yaml`, `artifacts/architecture/process_placement_map.yaml` |
| Transport & contracts | Typed binary RPC + event streams internally; JSON-RPC where required for ecosystem compatibility; schema/tooling is contract-first. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2), `docs/adr/0004-rpc-transport-and-schema-toolchain.md` (D-0004), `artifacts/architecture/rpc_tradeoff_rows.yaml` |
| Syntax & language routing | Tree-sitter is the default parser substrate; language tooling routes through one typed router with explicit provider provenance and quarantine. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2), `docs/architecture/parser_substrate_adr.md`, `docs/architecture/language_protocol_router_adr.md` |
| Extension runtime | Capability-sandboxed Wasm runtime plus isolated external hosts; no ambient privilege for extensions or AI tools. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2, 4.4), `docs/adr/0012-extension-manifest-permission-publisher-policy.md` (D-0018), `docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md` (D-0024), `docs/adr/0011-capability-lifecycle-and-dependency-markers.md` (D-0017) |
| Remote model | Same logical contracts locally and remotely; remote unavailability narrows features but must not strand local workflows. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2), `docs/adr/0020-remote-agent-contract.md` (D-0025), `artifacts/architecture/process_placement_map.yaml` |
| AI routing & evidence | AI is a routed platform plane with policy/evidence/provenance; background AI work is bounded and cannot starve input or bypass review/undo checkpoints. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2, 4.4), `docs/ai/context_assembly_contract.md`, `docs/ai/spend_and_route_receipt_contract.md`, `docs/ai/evidence_replayability_contract.md`, `artifacts/architecture/process_placement_map.yaml` |
| Release, provenance, and artifact trust | Signed, mirrorable, revocable artifacts with provenance and rollback; packaging is desktop-first and enterprise/self-host friendly. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2), `docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md` (D-0010), `docs/release/install_topology_plan.md`, `artifacts/release/install_topology_matrix.yaml` |
| Deployment profiles & continuity | Deployment profiles and continuity posture are explicit (local-only, self-hosted, enterprise, managed, air-gapped); optional services must degrade without breaking local-core workflows. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (9.x), `docs/deployment/locality_and_continuity_seed.md`, `artifacts/deployment/locality_matrix.yaml` |
| Optional service-plane posture | Optional services are additive and must not become hidden prerequisites; service APIs, retention, and offline behavior are contract-first and inspectable. | `.t2/docs/Aureline_Technical_Architecture_Document.md` (9.x), `docs/service/managed_service_seed.md`, `docs/service/api_inventory_seed.md`, `artifacts/service/slo_rows.yaml`, `artifacts/service/retention_rows.yaml` |
| Policy / flags / schemas | Policy, feature flags, and schema governance is explicit and inspectable (OpenFeature-shaped flags; Rego-shaped policy evaluation; JSON Schema + OpenAPI where applicable). | `.t2/docs/Aureline_Technical_Architecture_Document.md` (4.2), `docs/governance/policy_flag_schema_stack.md`, `docs/architecture/standards_interchange_matrix.md` |

## Rejected patterns (ledger + examples)

The canonical rejected-pattern ledger is `artifacts/architecture/rejected_pattern_rows.yaml`.

Worked “rejected proposal → rejection anchor → reopen packet” examples live in:

- `fixtures/architecture/rejected_pattern_examples/`

## Revisit triggers (when and how to reopen)

Reopening a decision is not a “maybe later” note. A proposal that wants to reopen posture must:

1. name a concrete trigger row in `artifacts/architecture/revisit_trigger_matrix.yaml`,
2. provide the required evidence packet(s) listed there, and
3. route through the named decision forum(s).
