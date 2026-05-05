# Framework-pack descriptor, package-manager identity, and lineage contract

This document freezes how Aureline framework packs declare identity, reuse
shared graph/search/build/docs/execution contracts, and disclose package-manager
and generated-source lineage truth so framework depth cannot accumulate private
metadata or hidden state.

Framework packs are overlays: they may add framework-aware facts and navigation,
but they MUST do so by emitting and consuming the same cross-surface records the
rest of the product uses (graph nodes/edges, search truth packets, build-target
events, docs-pack rows, execution-context truth, generated-artifact lineage, and
debug artifact manifests). If a framework pack cannot bind a claim back to these
contracts, the claim is non-conforming and MUST degrade.

Machine-readable companions:

- [`/schemas/language/framework_pack_descriptor.schema.json`](../../schemas/language/framework_pack_descriptor.schema.json)
  — boundary schema for `framework_pack_descriptor_record`, the descriptor every
  framework pack publishes for inspection, support export, and cross-surface
  provenance joins.
- [`/fixtures/language/framework_pack_cases/`](../../fixtures/language/framework_pack_cases/)
  — worked YAML cases exercising required overlay, identity, and lineage
  scenarios.

Composes with and does not replace:

- [`/docs/language/provider_graph_and_arbitration_contract.md`](./provider_graph_and_arbitration_contract.md)
  — provider-family taxonomy; framework packs MUST appear as attributable
  `framework_pack` providers rather than private “internal” lanes.
- [`/docs/framework/framework_certainty_and_source_sync_contract.md`](../framework/framework_certainty_and_source_sync_contract.md)
  — framework certainty row + source-sync chip; framework packs MUST emit
  evidence that can satisfy `framework_proven` without folklore.
- [`/docs/language/language_router_contract.md`](./language_router_contract.md)
  — package/workspace root binding and toolchain identity disclosure required
  before protected surfaces render results.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  — toolchain/target/scope vocabulary; framework packs MUST not mint a private
  “which Node/runtime/toolchain is in effect?” explanation format.
- [`/docs/execution/package_manager_and_lockfile_safety_contract.md`](../execution/package_manager_and_lockfile_safety_contract.md)
  — package-manager family vocabulary and no-hidden-mutation rules used by
  install/update flows; framework packs cite the same family ids and provenance
  posture.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  and [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
  — generated artifact lineage and row-level hints; framework packs must attach
  lineage truth rather than treating generated outputs as ordinary source.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  and [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — exact-build identity model; source maps and debug artifacts must bind to one
  exact build identity rather than ad hoc “build-ish” folklore.
- [`/docs/debug/artifact_resolution_seed.md`](../debug/artifact_resolution_seed.md)
  and [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  — debug artifact manifest binding symbols/source maps/generated-source mapping
  back to workspace/target/build identity.

If this document disagrees with the PRD, technical architecture, technical
design, or the linked contracts above, those documents win and this document
plus its companion schema and fixtures update in the same change.

## Why freeze this now

Framework-aware features are a high-risk place for accidental product truth:

- a framework pack can “know” routes from a dev-server side channel without
  disclosing which workspace root, package root, or toolchain identity produced
  the claim;
- a framework pack can treat generated artifacts (route manifests, typed route
  helpers, bundled preview outputs) as canonical source unless lineage truth is
  carried across explorer/search/review/AI surfaces; and
- a framework pack can silently pick one package manager (or one workspace root)
  while other flows use a different root/manager, creating contradictory
  semantics that look authoritative.

Freezing the descriptor, identity fields, and lineage obligations now prevents
later framework and ecosystem work from growing private metadata and “it works
on my machine” folklore into product truth.

## Scope

Frozen at this revision:

- one descriptor record shape (`framework_pack_descriptor_record`) and its
  identity fields;
- required contract bindings that keep packs compositional (graph/search/build/
  docs/execution/lineage/debug);
- package-manager and workspace-manager identity disclosure: root resolution,
  manifest authority, and toolchain-selection provenance;
- lineage obligations for generated outputs, preview artifacts, and source maps
  so derived bytes never masquerade as canonical source on protected surfaces;
- example cases for (a) route/config/test overlays, (b) mixed generated + authored
  source, and (c) package-manager identity drift.

Out of scope:

- implementing any framework-specific analyzer, bundler integration, or web
  runtime behavior; and
- prescribing UI copy (the vocabulary and record fields are frozen; final copy
  is owned by shell truth contracts).

## Definitions

### Framework pack

A **framework pack** is a provider-lane overlay that enriches language and graph
truth with framework-aware facts (routes, components, config, runtime bindings,
tests, and framework diagnostics). It is not a private subsystem: it is a
producer/consumer of the same records other surfaces use.

### Package-manager identity vs workspace-manager identity

- **Package-manager identity** names which package manager family (and version
  when known) is treated as authoritative for dependency and workspace
  semantics.
- **Workspace-manager identity** names the workspace boundary model (workspaces
  feature, monorepo manager, or ecosystem-specific workspace system) that defines
  package roots and member boundaries.

These two may coincide (for example, one tool implements both), but they are
recorded separately so root selection and authority can be inspected.

### Generated-source lineage

**Generated-source lineage** is the explicit binding between a derived artifact
and:

1. its canonical source inputs (with identity + digests),
2. the generator/resolver identity and version,
3. the regeneration path, and
4. the writable-boundary/edit policy.

Framework packs MUST treat generated artifacts as first-class and must carry
lineage refs into explorer/search/review/AI/debug flows rather than relying on
implicit “everyone knows this file is generated” heuristics.

## 1. Descriptor and shared-contract reuse

Every framework pack MUST publish a `framework_pack_descriptor_record` conforming
to the companion schema. The descriptor is the stable record other surfaces use
to answer:

- what the pack is (identity + version),
- what it claims to support (overlay surface families),
- what contracts it binds to (graph/search/build/docs/execution/lineage/debug),
  and
- what package-manager/workspace-manager identity it will surface.

### 1.1 The “no private metadata” rule

Framework packs MUST NOT introduce a parallel graph, parallel workspace-root
resolver, parallel package-manager identity model, parallel lineage model, or
parallel debug artifact catalog.

Allowed:

- caching or internal indices for performance *as long as* every claim exposed
  to a protected surface can be reconstructed from, or at least explained by,
  the shared records cited by the descriptor (graph nodes/edges, execution
  context, lineage, debug manifest entries).

Forbidden:

- pack-private ids that are not linkable to graph ids, lineage ids, or execution
  context ids;
- pack-private truth fields that can change the meaning of a claim without
  appearing in a router decision, result provenance record, or lineage record.

### 1.2 Required contract bindings

Every descriptor MUST declare bindings for the following contract classes:

| Contract class | Purpose |
|---|---|
| `workspace_graph` | Framework facts must project into the shared workspace graph rather than a private model. |
| `search_truth` | Search rows must keep readiness/degraded-state truth; framework facts are attributable and do not silently re-rank or rewrite truth. |
| `build_target_events` | Framework-aware build/test/run facts must bind to build-target vocabulary and event envelopes rather than pack-private “tasks”. |
| `docs_pack` | Any framework docs projection must reuse docs-pack identity, publication posture, and redaction rules. |
| `execution_context` | Toolchain/runtime/target identity must reuse execution-context truth so “which runtime produced this?” is answerable everywhere. |
| `generated_artifact_lineage` | Generated artifacts produced/consumed by packs must carry lineage refs and safe-edit policy, not folklore. |
| `debug_artifact_manifest` | Source maps, symbols, crash artifacts, and generated-source mappings must bind to debug artifact manifests and exact-build identity. |

The contract-binding rows in the descriptor are declarations: they do not embed
the full records, but they freeze which shared schemas and docs are referenced
so pack authors cannot quietly swap dialects.

## 2. Package-manager and workspace-manager identity disclosure

Framework packs MUST disclose which package/workspace manager identity influenced
their results so reviewers can distinguish:

- “this route mapping was computed under pnpm workspace root A” vs
- “this mapping was computed under yarn workspace root B” vs
- “workspace root was ambiguous; mapping is downgraded”.

### 2.1 Root resolution and manifest authority

Every descriptor MUST include a `package_manager_identity` block that can be
rendered by:

- execution-context inspectors,
- router-decision explainers,
- support exports, and
- AI evidence citations.

Minimum truth required:

- selected package-manager family (closed vocabulary shared with the package
  change plan),
- selected workspace-manager family (closed vocabulary),
- workspace-root resolution class and anchor ref,
- manifest authority: which manifest(s) are treated as canonical, and
- toolchain-selection provenance refs that tie the decision back to
  execution-context resolution.

### 2.2 Identity drift is explicit

If multiple package managers appear plausible for a workspace (multiple lockfiles,
conflicting workspace manifests, explicit overrides, or imported/support-only
evidence), a pack MUST:

1. select exactly one `selected_identity`, or select the `unknown_requires_review`
   posture, and
2. record an `identity_drift` block naming what conflicted.

A pack MUST NOT silently switch identities between runs without:

- a new execution-context id/provenance record, and
- an updated identity snapshot in the pack’s emitted provenance packets.

## 3. Overlay families (routes, config, tests) and evidence rules

The descriptor declares which overlay families the pack can emit. Overlay
families are not framework-specific; they are categories of evidence a pack may
project.

### 3.1 Route overlays

A route overlay claim MUST be attributable and provable:

- It MUST cite a proving artifact class (manifest, build output, runtime index,
  or other typed evidence source).
- It MUST join to a workspace-graph projection (nodes/edges) so other surfaces
  can point back to the same anchor.
- If the overlay is inferred (not proven), it MUST degrade to an inferred class
  per the framework certainty contract and MUST NOT claim proven authority.

### 3.2 Config overlays

Config overlays (environment files, framework config, runtime config, test
config) MUST disclose:

- which config root was selected (opaque ref),
- which manifest authority governed the selection, and
- whether the config is canonical source, imported snapshot, or derived
  projection.

Config overlays MUST NOT widen scope silently (for example, reading outside the
 selected workspace root) and MUST fail closed under policy/trust narrowing.

### 3.3 Test overlays

When a framework pack contributes test discovery or test run intent, it MUST:

- bind test identity to the build/test target vocabulary (target refs, profile
  refs, run context refs), and
- disclose the package/workspace root and toolchain identity that governed test
  semantics.

Test overlays MUST NOT execute code or mutate state; they only publish discovery
and provenance records. Execution uses the execution and package-action contracts.

## 4. Generated-source and source-map lineage rules

Framework packs frequently touch derived artifacts:

- generated route manifests and typed route helpers,
- bundled preview outputs,
- framework caches and resolved manifests, and
- source maps and debug artifacts.

The lineage and exact-build identity contracts already freeze how these are
represented. Framework packs MUST reuse them and MUST NOT introduce a
pack-private lineage dialect.

### 4.1 Generated artifacts never masquerade as canonical source

Rules:

1. Any artifact a framework pack causes to appear in explorer/search/review/AI
   surfaces that is not canonical handwritten source MUST carry a
   `generated_artifact_lineage_ref` and a row-level lineage hint.
2. A pack MAY provide richer presentation, but it MUST preserve the shared
   provenance tokens (artifact class, drift state, default edit posture).
3. If lineage cannot be resolved, the pack MUST degrade to `unknown_lineage`
   posture and MUST NOT claim an edit-safe or authoritative posture.

### 4.2 Source maps and debug artifacts bind to exact build identity

If a framework pack produces, consumes, or references:

- JS/TS/CSS source maps,
- crash artifacts, symbols, or profiles, or
- generated-source mapping packets,

then the pack MUST surface those artifacts only through debug artifact manifest
entries that bind to one exact build identity. A pack MUST NOT attach a source
map or debug artifact to a result without:

- a target/build-target ref, and
- an exact-build identity ref (or an explicit pending/unlinked posture).

### 4.3 Bundle outputs, previews, and “stable surfaces”

Any framework pack claim that crosses a stable/protected surface boundary (route
explorer, component tree, debug resolution, test/run scaffolding, AI citations,
support export) MUST be backed by one or more of:

- workspace-graph ids/edges,
- execution-context ids/provenance ids,
- generated-artifact lineage refs, and/or
- debug artifact manifest entries that bind to exact build identity.

If none are available, the pack MUST degrade its claim and must not emit a
stable-looking fact.

## Change management

- Adding a new contract binding class, a new overlay family, or a new identity
  resolution class is additive-minor: bump
  `framework_pack_descriptor_schema_version` and extend fixtures.
- Repurposing an existing token or changing the meaning of an existing field is
  breaking and requires a new schema version plus updated fixtures and docs.

