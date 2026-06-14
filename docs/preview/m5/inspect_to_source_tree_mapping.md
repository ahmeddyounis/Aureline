# M5 inspect-to-source tree mapping

This document is the contract for the M5 inspect-to-source tree-node mapping. It
binds the **inspectable component, DOM, and widget-tree surfaces** onto a single
shared mapping packet so the mapping quality of every inspected node — and where
inspect-to-source actually lands — stops hiding inside provider-specific
extension chrome.

Where the
[source-first preview / browser-runtime inspection matrix](freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md)
freezes the *qualification* of each claimed preview/runtime surface and the
[preview-session descriptors](preview_session_descriptors.md) materialize the
*per-session* state each surface presents, this packet materializes the
*per-node* truth behind every inspectable tree node.

Source remains canonical; the mapping packet is derivative — never a second
writable truth model. Every node labels itself **exact**, **approximate**,
**generated-only**, or **runtime-only** before any jump-to-source or mutation
affordance appears, and a runtime-only node never masquerades as saved source
state.

## Source of truth

- Packet type: `InspectToSourceTreePacket`
  (`crates/aureline-preview/src/inspect_to_source_tree/`).
- Boundary schema:
  `schemas/preview/inspect_to_source_tree_mapping.schema.json`.
- Checked support export:
  `artifacts/preview/m5/inspect_to_source_tree_mapping/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/inspect_to_source_tree_mapping.md`.
- Protected fixtures:
  `fixtures/preview/m5/inspect_to_source_tree_mapping/`.
- Conformance dump: `cargo run -p aureline-preview --example dump_m5_inspect_to_source_tree [support|summary]`.

## Inspectable tree kinds

Each inspectable tree kind carries at least one node mapping descriptor:
`component`, `dom_element`, and `widget_tree_node`. Component, DOM, and
widget-tree inspection normalize onto the same mapping packet rather than bespoke
per-framework chrome.

## Mapping-quality labels

Each node carries exactly one mapping-quality label, resolved and presented
before any source-navigation or mutation affordance:

| Label | Meaning | Continuity route |
| --- | --- | --- |
| `exact` | Maps unambiguously to a canonical-source span | `exact_jump` |
| `approximate` | Maps to source heuristically; a jump lands near, not exactly | `approximate_jump_with_disclosure` |
| `generated_only` | Generated output with no hand-authored span | `source_only_fallback` |
| `runtime_only` | Exists only in the live runtime tree, no source backing | `runtime_only_explanation` |

- The **continuity route** is fully determined by the mapping quality, so the
  chrome can never advertise a stronger jump than the mapping supports.
- The **source anchor** is the opaque pointer at the canonical span the node maps
  to; it is required for source-backed (`exact` / `approximate`) quality, may
  point at a generator input for `generated_only`, and is absent for
  `runtime_only`.

## Label-before-affordance and continuity gate

A node may not offer navigation or mutation it cannot back:

- A node's `mapping_label_resolved` flag is true before `source_navigation_offered`
  or `mutation_offered` is true.
- A node downgraded through a runtime reconnect, provider loss, or mapping
  downgrade records a `downgrade_trigger` and a precise, non-generic
  `degraded_label`, and never silently re-upgrades into a source-backed mapping.
  A node with no trigger carries no degraded label.

`InspectToSourceTreePacket::validate` rejects a packet that:

- omits a required inspectable tree kind, omits a required mapping-quality class,
  or demonstrates no continuity-preserving downgrade node;
- lets a node's continuity route disagree with its mapping quality;
- offers navigation or mutation before the mapping label is resolved;
- declares a source-anchor presence inconsistent with the mapping quality;
- lets a `runtime_only` node claim saved source state or carry a canonical source
  anchor, or lets any non-source-backed node claim saved source state;
- offers a mutation on a node that is not source-backed, lacks a source anchor, or
  skips previewing the real source diff before commit;
- lets a node carry a downgrade trigger without a precise label, or a precise
  label without a trigger;
- carries a node without evidence;
- fails any guardrail or consumer-projection invariant; or
- carries raw boundary material in the export.

## Guardrails

- Source remains canonical; the mapping packet is derivative — never a second
  writable truth model.
- Runtime state never hides source-mapping uncertainty behind a node label.
- Inspect-only nodes are never auto-upgraded into write-capable designer flows.
- Embedded preview/browser boundaries are not blurred into product authority.
- The mapping-quality label is shown before any navigation or mutation.
- Continuity is preserved without silently upgrading a runtime-only node into a
  source-backed one.

## Consumers

Product, docs/help, diagnostics, support export, and release-control surfaces
ingest these node packets directly instead of cloning mapping terminology by
hand, and support / diagnostics exports can reconstruct exactly what mapping
quality the user saw for each inspected node.
