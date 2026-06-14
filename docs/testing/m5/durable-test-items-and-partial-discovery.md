# M5 durable test items, template / invocation identities, notebook-linked tests, and partial-discovery truth

This document is the contract for the durable test-item discovery objects the M5
framework, notebook, and test-tree lanes normalize onto. Where the
test-intelligence qualification matrix gates *whether a surface's claim is backed
by identified objects*, this contract lands the **objects themselves**: the
durable nodes and discovery snapshots that make a discovered test tree a set of
stable, separately identified product objects rather than a list of display rows.

A discovered test tree only stays trustworthy if every item carries durable
identity, a parameterized template stays distinct from its concrete invocations,
a notebook-linked test stays distinct from an ordinary file-backed test, and
partial or heuristic discovery stays visible rather than collapsing into an empty
or overconfident tree. This contract makes those guarantees structural.

## Source of truth

- Packet type: `DurableTestDiscoveryPacket`
  (`crates/aureline-runtime/src/durable_test_items_and_partial_discovery/`).
- Boundary schema:
  `schemas/testing/durable-test-items-and-partial-discovery.schema.json`.
- Checked support export:
  `artifacts/testing/m5/durable-test-items-and-partial-discovery/support_export.json`.
- Markdown summary:
  `artifacts/testing/m5/durable-test-items-and-partial-discovery.md`.
- Protected fixtures:
  `fixtures/testing/m5/durable-test-items-and-partial-discovery/`.

Regenerate the canonical export and summary after any shape change:

```bash
cargo run -p aureline-runtime --example dump_durable_test_discovery
cargo run -p aureline-runtime --example dump_durable_test_discovery summary
```

## Durable test nodes

Each `DurableTestNode` carries its own stable `node_id` and a distinct
`node_kind`. The five kinds are deliberately separate so no two collapse into one
row identity:

| `node_kind` | meaning |
| --- | --- |
| `suite` | a grouping suite / module / class that contains other nodes |
| `concrete_case` | a single concrete, file-backed runnable case |
| `parameterized_template` | a template (family root); never itself run as one case |
| `concrete_invocation` | one concrete invocation of a template |
| `notebook_linked_test` | a test bound to a notebook cell, not an ordinary file |

Identity rules every node obeys (`DurableTestNode::is_valid`):

- **Display labels are never identity.** `identity_basis_token` is a non-display
  stable basis and must differ from `display_label`. A node whose basis equals
  its label is rejected (`display_label_substitutes_identity`).
- **Templates stay distinct from invocations.** A `concrete_invocation` carries a
  `template_node_id` that points to a *different* node — a
  `parameterized_template` present in the same snapshot — and an `invocation_key`.
  An invocation aliased onto its template's id, or pointing at a missing template,
  is rejected (`template_collapsed_with_invocation`).
- **Notebook-linked tests stay distinct from file tests.** A
  `notebook_linked_test` carries a `notebook_linkage` (opaque notebook and cell
  ids, never raw cell source); no other kind may carry one
  (`notebook_test_collapsed_with_file_test`).
- **Fail closed on display-only identity.** A node whose `identity_class` is
  `display_text_only_denied` (or whose `source_state` is `display_text_only`) is
  not a valid durable node.

The existing canonical taxonomy bridges onto these nodes through
`DurableTestNode::from_canonical_item`, which maps `CanonicalTestItemKind`
(`case`, `parameterized_family`, `parameterized_instance`,
`imported_provider_instance`) onto durable node kinds — so the first real
framework-pack / test-tree consumer normalizes onto these objects instead of
brittle display rows.

## Identity remap and source moves

When a node's source moves, `DurableTestNode::degrade_to_needs_remap` degrades it
to `remap_review_required` and `source_moved_needs_remap` **and appends the
superseded identity basis to `prior_identity_chain`**. A node that needs remap
but has lost its prior chain is rejected (`remap_chain_lost`). The durable
identity chain therefore survives a source move; review can reattach the node
without inventing a brand-new identity.

## Discovery snapshots and partial-discovery truth

Each `DiscoverySnapshot` belongs to one consumer (`framework_pack`, `notebook`,
`test_tree`, or `imported_ci`) and records:

- a `partiality` (`complete`, `partial_visible`, `heuristic`, `streaming`, or
  `provider_imported`);
- `omitted_scopes`, each with a closed `reason` and a precise label, so partial /
  heuristic / streaming / imported discovery stays visible;
- an export-safe `mapping_support_class` so the snapshot's support disposition
  survives reopen, support export, and release evidence.

Partial-visibility rule (`DiscoverySnapshot::partial_visibility_ok`): a
non-complete snapshot **must** record at least one omitted scope, and a complete
snapshot **must** record none. A partial snapshot with no recorded omission, or a
complete snapshot that hides an omission, is rejected
(`partial_discovery_hidden`). This is what keeps partial discovery from collapsing
into an empty or overconfident tree.

Mapping-support consistency (`DiscoverySnapshot::mapping_support_consistent`)
binds the export-safe class to the truth: `fully_mapped_local` requires a complete
local snapshot with no needs-remap node; `partially_mapped_visible` requires a
non-complete local snapshot; `imported_read_only_mapped` requires imported /
provider-owned discovery; `needs_remap_preserved` requires at least one
needs-remap node. An overstated class is rejected
(`mapping_support_inconsistent`).

## Imported / provider separation

When a snapshot's consumer, partiality, or any node is imported / provider-owned,
`imported_not_shown_as_local` must hold — imported discovery never masquerades as
a live local rerun (`imported_shown_as_local`).

## Validation coverage

`DurableTestDiscoveryPacket::validate` additionally requires, across the packet:

- the `framework_pack`, `notebook`, and `test_tree` consumers are each
  represented (`required_consumer_missing`);
- both `parameterized_template` + `concrete_invocation` and
  `notebook_linked_test` + `concrete_case` node kinds are present, proving the
  taxonomy is exercised, not merely declared;
- at least one snapshot demonstrates partial-but-visible discovery
  (`partial_discovery_case_missing`);
- at least one node demonstrates a needs-remap chain preserved across a source
  move (`remap_chain_case_missing`);
- the guardrail and consumer-projection blocks hold, and the export carries no raw
  boundary material.

## Boundary discipline

The packet carries only typed class tokens, booleans, opaque ids, and
redaction-aware reviewable labels. Raw test source, raw provider payloads, raw log
bytes, provider cursors, credentials, and raw artifact bodies never cross this
boundary.
