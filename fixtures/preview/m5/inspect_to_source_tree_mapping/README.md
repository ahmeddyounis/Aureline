# M5 Inspect-to-Source Tree Mapping Fixtures

## node_set_labels_quality_and_preserves_continuity_on_provider_loss.json

A label-before-affordance and continuity-preservation drill fixture for the
inspect-to-source tree mapping packet. The inspectable tree kinds — component,
DOM element, and widget-tree node — each carry at least one node descriptor that
resolves and presents its mapping-quality label before any jump-to-source or
mutation affordance.

The packet demonstrates all four mapping-quality labels: an `exact` component
node that maps unambiguously to its canonical-source span and offers a write-back
mutation that previews the real source diff before commit; an `approximate` DOM
node whose jump-to-source lands near the span with an explicit disclosure; a
`generated_only` widget node with no hand-authored span whose inspect-to-source
falls back to the generator input; and a `runtime_only` DOM node whose source map
was lost on provider loss. The runtime-only node is `runtime_backed`, never claims
to be saved source state, carries no canonical source anchor, and records a
`provider_loss` downgrade trigger with a precise non-generic degraded label — so
continuity is preserved without silently turning a runtime-only node into a
source-backed one. Every non-source-backed node stays inspect-only.

The fixture validates against
`schemas/preview/inspect_to_source_tree_mapping.schema.json` and is byte-aligned
with the in-crate builder via
`cargo run -p aureline-preview --example dump_m5_inspect_to_source_tree`.
