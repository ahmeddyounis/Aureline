# Infrastructure-as-Code, Manifest, and Resource-Relationship Matrix

This document freezes how Aureline relates authored infrastructure
configuration, rendered outputs, plan/validation results, and observed live
resources so DevOps-class navigation surfaces stay explainable and
source-faithful.

The canonical M5 packet for later implementation rows now lives in
[`/docs/infra/source-intelligence-and-resource-relationships.md`](../infra/source-intelligence-and-resource-relationships.md)
with its matching schema and fixtures. This document remains useful background
and predecessor context, but later M5 infra rows should cite the infra packet
first.

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or Design
System Style Guide, those source documents win and this document plus its
companion artifacts must be updated in the same change.

## Companion artifacts

- [`/artifacts/devops/resource_relationship_matrix.yaml`](../../artifacts/devops/resource_relationship_matrix.yaml)
  — machine-readable matrix (source classes, required edges, cache metadata
  requirements, and rules).
- [`/fixtures/devops/resource_relationship_cases/`](../../fixtures/devops/resource_relationship_cases/)
  — worked relationship cases covering rendered manifests, policy overlays,
  plans, container/log navigation, and CI run→artifact linkages.
- [`/schemas/runtime/target_context.schema.json`](../../schemas/runtime/target_context.schema.json)
  — boundary schema for `resource_relationship` and `source_truth_class`
  used by action-safety and environment-context packets.
- [`/docs/runtime/environment_connector_action_safety_contract.md`](../runtime/environment_connector_action_safety_contract.md)
  — environment-context contract that consumes `resource_relationships[]`
  and requires explicit `no_source_match` when a mapping cannot be made.
- [`/docs/runtime/resource_drift_and_live_action_contract.md`](../runtime/resource_drift_and_live_action_contract.md)
  — drift and live-action contract defining `tool_identity` and
  cross-layer comparison semantics.

## Scope

In scope:

- source classes for Terraform/HCL, Kubernetes manifests, container and
  devcontainer descriptors, CI/environment descriptors, and policy/access
  configs;
- the minimum identity tokens each class must expose so formatting-only
  edits do not “re-key” objects;
- the required derived edges each class must support so navigation can move
  between authored intent, rendered output, plan/validation results, and
  observed live resources without inventing truth; and
- cache metadata requirements for generated/rendered objects so surfaces can
  explain provenance and invalidate stale outputs.

Out of scope:

- implementing cluster connectors, CI adapters, IaC analyzers, or provider
  integrations; and
- defining provider-specific request payloads or vendor-specific state
  snapshots.

## Shared vocabulary

### Truth labels

Truth labels are the minimum “what layer is this?” badges that every DevOps
surface must keep distinct.

- **`authored`** — repo-owned source configuration (workspace files).
- **`rendered`** — generated output produced deterministically from authored
  inputs (for example `helm template`, `kustomize build`, compose expansion).
- **`planned`** — plan/dry-run output produced from authored/rendered inputs
  (for example Terraform plan, server-side diff, admission validation).
- **`observed`** — live observation of a real runtime object.
- **`provider_overlay`** — provider-owned context (console metadata, rollout
  panels, CI checks) that is useful but not canonical repo truth.
- **`imported_evidence`** — imported snapshots or support bundles that may be
  inspectable but carry no live mutation authority.
- **`no source match`** — an explicit state stating that a live object cannot
  be traced back to source or plan. Runtime packets record this as
  `relationship_state = no_source_match`.

### Required generated-object metadata

Any generated, rendered, or cached object that is used for navigation or
diff/plan must retain:

- **Source set** — the set of input source refs (paths plus digests or other
  stable revision tokens) used to produce the object.
- **Tool identity/version** — the tool name and version (plus an optional
  digest) that produced the object.
- **Invalidation epoch** — an opaque epoch token that changes whenever any
  input that could change the output changes (inputs, tool version, render
  flags, environment selectors). Cached results must not be treated as
  equivalent across different epochs.

## Source-class matrix

Each source class below defines:

- primary identities (stable identity tokens);
- required derived edges (minimum navigation edges the graph must support);
- live overlay support (whether provider/connector overlays may attach);
- minimum truth labels (labels surfaces must preserve); and
- cache metadata requirements (for rendered/generated objects).

### Terraform / HCL

- **Example artifacts:** module files, variables/outputs, state references,
  plan files.
- **Primary identities:** module path, resource address, workspace/environment
  selector.
- **Required derived edges:** module-to-module, variable/output,
  resource-to-provider, plan-to-resource, resource-to-runbook.
- **Live overlay support:** yes (CLI/agent/provider overlays when claimed).
- **Minimum truth labels:** `authored`, `planned`, `observed`,
  `provider_overlay`.
- **Cache metadata requirements:** any plan output, plan summary, or cached
  “planned graph” must retain source set, tool identity/version, and
  invalidation epoch.

### Kubernetes manifests

- **Example artifacts:** YAML objects, Kustomize layers, Helm values/templates,
  admission-policy outputs.
- **Primary identities:** group/version/kind + name + namespace plus source
  path.
- **Required derived edges:** source-to-rendered object, object-to-live
  resource, object-to-log/event stream, object-to-runbook.
- **Live overlay support:** yes (on claimed clusters/connectors).
- **Minimum truth labels:** `authored`, `rendered`, `planned`, `observed`.
- **Cache metadata requirements:** any rendered output or validation snapshot
  must retain source set, tool identity/version, and invalidation epoch.

### Container and devcontainer descriptors

- **Example artifacts:** `Dockerfile`, compose files, devcontainer configs,
  OCI metadata.
- **Primary identities:** image ref, service name, container/workspace
  identity.
- **Required derived edges:** source-to-built image, service-to-port,
  service-to-log stream, service-to-attach target.
- **Live overlay support:** yes (on claimed container runtimes).
- **Minimum truth labels:** `authored`, `rendered`, `observed`.
- **Cache metadata requirements:** image build outputs, devcontainer-resolved
  configs, and cached service expansion must retain source set, tool
  identity/version, and invalidation epoch.

### CI / environment descriptors

- **Example artifacts:** workflow files, env templates, deployment descriptors,
  preview-environment configs.
- **Primary identities:** pipeline id, environment name, rollout target.
- **Required derived edges:** source-to-run, run-to-artifact,
  env-to-service/resource slice.
- **Live overlay support:** yes (on claimed CI/provider overlays).
- **Minimum truth labels:** `authored`, `observed`, `provider_overlay`.
- **Cache metadata requirements:** any cached run summary or artifact index
  derived from CI must retain source set, tool identity/version, and
  invalidation epoch.

### Policy and access configs

- **Example artifacts:** admission policies, RBAC-like files, network policy,
  secret/identity binding descriptors.
- **Primary identities:** policy id, scope selector, principal/resource class.
- **Required derived edges:** policy-to-target resource,
  policy-to-enforcement result, policy-to-runbook.
- **Live overlay support:** yes (where enforcement and audit feeds are
  integrated).
- **Minimum truth labels:** `authored`, `planned`, `observed`,
  `provider_overlay`.
- **Cache metadata requirements:** compiled policies, validation results, and
  cached enforcement summaries must retain source set, tool identity/version,
  and invalidation epoch.

## Rules

1. **Stable identities under formatting-only changes.** Primary identities
   must remain stable under whitespace, ordering, comment, or reflow edits.
2. **Live overlays must link back or say no source match.** Live overlays
   must point back to source or plan objects when the relationship is
   knowable, and must state **no source match** explicitly when it is not.
3. **Rendered/generated objects retain provenance.** Rendered or generated
   objects may be cached, but cache metadata must retain the source set, tool
   identity/version, and invalidation epoch.
4. **Overlays may enrich but not replace.** Provider overlays may enrich
   navigation but may never silently replace repo-owned truth or local diff
   state.

## Worked examples

The worked fixture cases are under
[`/fixtures/devops/resource_relationship_cases/`](../../fixtures/devops/resource_relationship_cases/).
Each case includes an explicit source set, tool identity/version, and
invalidation epoch for any generated objects, plus explicit `no_source_match`
when applicable.
