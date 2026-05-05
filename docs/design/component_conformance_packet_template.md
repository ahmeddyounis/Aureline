# Component conformance packet template

This document defines the **component conformance packet**: a small, auditable
record that ties a reusable UI component to the contracts and evidence that make
it safe to ship across themes, densities, accessibility postures, and extension
or embedded surfaces.

The conformance packet exists so component quality can be reviewed from one
repeatable artifact family (packets + CI results) rather than inferred from
scattered screenshots, ad hoc notes, and implementation details.

Companion contracts and artifacts:

- [`/docs/ux/component_contract_template.md`](../ux/component_contract_template.md) and
  [`/schemas/design/component_contract.schema.json`](../../schemas/design/component_contract.schema.json)
  — the canonical reusable component contract packet shape. Conformance packets
  reference a component contract packet by id; they do not replace it.
- [`/docs/design/component_state_taxonomy.md`](./component_state_taxonomy.md)
  — the shared state taxonomy reusable components must map to.
- [`/docs/design/component_metrics_contract.md`](./component_metrics_contract.md),
  [`/artifacts/design/component_metrics_ledger.yaml`](../../artifacts/design/component_metrics_ledger.yaml),
  and [`/schemas/design/component_metrics.schema.json`](../../schemas/design/component_metrics.schema.json)
  — the frozen micro-metrics ledger reusable controls and rows must cite.
- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md),
  [`/docs/design/theme_support_and_inheritance_contract.md`](./theme_support_and_inheritance_contract.md),
  and [`/docs/design/token_conformance_audit.md`](./token_conformance_audit.md)
  — the token, theme, inheritance, and drift-gate vocabulary conformance packets
  must cite.
- [`/artifacts/design/token_drift_rules.yaml`](../../artifacts/design/token_drift_rules.yaml)
  — closed `allowed_inheritance_gap_class` vocabulary and gap policy used when a
  component declares extension/embedded inheritance exceptions.
- [`/docs/design/appearance_evidence_packet_template.md`](./appearance_evidence_packet_template.md) and
  [`/artifacts/design/appearance_row_coverage_matrix.yaml`](../../artifacts/design/appearance_row_coverage_matrix.yaml)
  — appearance evidence packet family conformance packets may link to when the
  component participates in launch-critical appearance rows.

Machine-readable boundaries:

- [`/schemas/design/component_conformance_packet.schema.json`](../../schemas/design/component_conformance_packet.schema.json)
  validates packet structure.
- [`/artifacts/design/component_conformance_matrix.yaml`](../../artifacts/design/component_conformance_matrix.yaml)
  publishes the component-family coverage matrix and the CI fail-gate contract.

## Packet rules (contract)

1. **Packets link, they do not duplicate.** Conformance packets point at the
   component contract, state taxonomy, metrics ledger, token inheritance
   contract, and evidence refs; they do not restate those contracts in prose.
2. **Evidence travels by stable ref.** Packets MUST NOT embed raw screenshots,
   raw URLs, raw token bytes, or raw user content. They carry ids and refs only.
3. **State meaning is taxonomy-bound.** A component’s required states must be
   expressed using the shared taxonomy classes; local lifecycle labels map to
   those classes in the component contract.
4. **Metrics cite the ledger.** Components MUST cite metric ids from the
   component metrics ledger; exceptions must be explicit and justified.
5. **Token inheritance is explicit.** Components cite the token vocabulary and
   drift gate contract, and declare any extension/embedded inheritance gaps as
   explicit exception records using the closed allowed-gap vocabulary.
6. **Launch-critical components fail closed.** When a component is marked
   launch-critical, CI rejects packets that omit metric ids or the minimum
   keyboard, accessibility, and screenshot-baseline evidence hooks.

## Packet outline

A packet is a single YAML record with the sections below. Empty arrays are
allowed structurally, but CI may require them to be non-empty depending on the
launch priority class.

### 1) Identity and scope

- `packet_id` — stable id for the conformance packet.
- `component_id`, `component_title`, `component_family_class`
- `launch_priority_class` — launch-critical vs supporting vs optional.
- `component_contract_ref` — id or ref for the corresponding component contract
  packet.
- `conformance_posture_class` — fully conformant vs conformant with disclosed
  gaps.

### 2) State taxonomy coverage

- `state_taxonomy.taxonomy_ref` — ref to the shared taxonomy document.
- `state_taxonomy.required_taxonomy_state_classes[]` — the taxonomy classes the
  component must support and preserve semantics for.

### 3) Metric references

- `metric_requirements.ledger_ref` — ref to the component metrics ledger.
- `metric_requirements.metric_ids[]` — metric ids the component binds to.
- `metric_requirements.component_metric_case_refs[]` — optional case ids binding
  the component to density/input baselines.

### 4) Token inheritance and drift-gate routing

- `token_inheritance.token_vocabulary_ref`
- `token_inheritance.theme_inheritance_contract_ref`
- `token_inheritance.token_conformance_contract_ref`
- `token_inheritance.token_drift_rules_ref`
- `token_inheritance.token_drift_check_refs[]` — evidence ids for drift checks.

### 5) Appearance and screenshot baselines

- `appearance_evidence.screenshot_baseline_refs[]` — baseline capture ids across
  relevant themes/densities/postures.
- `appearance_evidence.appearance_notes[]` — short notes; no raw values.

### 6) Keyboard and accessibility evidence

- `keyboard_evidence.keyboard_journey_refs[]`
- `accessibility_evidence.assistive_technology_refs[]`
- `accessibility_evidence.accessible_name_notes[]`

### 7) Extension/embedded inheritance exceptions

When a component may appear in extension or embedded surfaces, any partial theme
or token inheritance posture must be declared explicitly:

- `extension_embedded_exceptions[]` — each record pins one
  `allowed_inheritance_gap_class`, a rationale, and evidence refs.

