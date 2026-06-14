# M5 Inspect-to-Source Tree Mapping

- Packet: `m5-inspect-to-source-tree:stable:0001`
- Label: `M5 Inspect-to-Source Tree Mapping`
- Nodes: 4 (1 downgraded)
- Tree kinds: 3 / 3
- Mapping qualities: 4 / 4

## Nodes

- **inspect-node:component:0001** (component)
  - Component node mapped exactly to its canonical-source span with a write-back round-trip
  - tree=component mapping=exact continuity=exact_jump runtime_backed=true
  - source_anchor=`source_anchor:inspect-node:component:0001` source_nav=true mutation=true
- **inspect-node:dom:0001** (dom_element)
  - DOM element mapped approximately to source; jump-to-source lands near the span with disclosure
  - tree=dom_element mapping=approximate continuity=approximate_jump_with_disclosure runtime_backed=true
  - source_anchor=`source_anchor:inspect-node:dom:0001` source_nav=true mutation=false
- **inspect-node:widget:0001** (widget_tree_node)
  - Generated widget node with no hand-authored span; inspect-to-source falls back to the generator input
  - tree=widget_tree_node mapping=generated_only continuity=source_only_fallback runtime_backed=false
  - source_anchor=`source_anchor:generator-input:widget:0001` source_nav=true mutation=false
- **inspect-node:dom:0002** (dom_element)
  - Runtime-only DOM node whose source map was lost on provider loss; explained as having no source to jump to
  - tree=dom_element mapping=runtime_only continuity=runtime_only_explanation runtime_backed=true
  - source_anchor=`none` source_nav=false mutation=false
  - Downgraded: Source map provider was lost on reconnect; this node is runtime-only and has no canonical source span to open
