# M5 manifest / build component consumer contract (M05-817)

This is the first-consumer **adoption** contract over the frozen M5 manifest /
build-confidence component matrix (M05-812) and the three narrowing primitives
that implement it (M05-813 manifest-authoring, M05-814 live-resource navigation,
M05-815 build-confidence). It proves the ten reusable component families are
genuine **primitives** — not one infra page and one launcher page — by adopting
them across the four claimed M5 handoff consumer classes and holding every
consumer to the same target-context, freshness, schema-source, adapter-source,
confidence, and degraded-state language.

- Module: `crates/aureline-infra/src/add_shared_container_devcontainer_request_incident_support_and_ai_manifest_build_component_consumers/`
- Schema: [`schemas/ui/m5-manifest-build-component-consumer.schema.json`](../../schemas/ui/m5-manifest-build-component-consumer.schema.json)
- Support export (`include_str!` canonical): [`artifacts/release/m5-manifest-build-component-consumer-proof/support_export.json`](../../artifacts/release/m5-manifest-build-component-consumer-proof/support_export.json)
- Matrix CSV / report: `matrix.csv`, `report.md` in the same proof directory
- Protected fixtures: [`fixtures/ui/m5-manifest-build-component-consumers/`](../../fixtures/ui/m5-manifest-build-component-consumers)

## Consumer classes

Each row is one consumer adopting one canonical family on one surface. The four
consumer groups (`ConsumerGroup`) must all be present:

| Group | Surfaces |
| --- | --- |
| `container_devcontainer` | `devcontainer_manifest_panel`, `container_target_graph_inspector` |
| `request_live_resource_handoff` | `request_resource_link_handoff`, `live_resource_explorer_handoff` |
| `incident_support` | `incident_support_bundle`, `support_export`, `release_proof` |
| `ai_explanation` | `ai_execution_explainer`, `ai_confidence_narrative`, `docs_help` |

## Canonical family bindings

Consumers never invent a surface-local schema. `canonical_schema_ref_for` /
`canonical_packet_ref_for` map each of the ten `M5ManifestBuildComponentFamily`
back to the single primitive that owns it:

- **manifest-authoring** — `manifest_editor_header`, `schema_validator_row`, `target_context_chip_group`
- **live-resource navigation** — `resource_link_row`, `resource_explorer_row`
- **build-confidence** — `adapter_source_badge`, `target_graph_row`, `capability_matrix`, `raw_event_drawer`, `fallback_confidence_drawer`

## Enforced invariants (`ManifestBuildConsumerPacket::validate`)

- **AC1 — one canonical family:** every row's `canonical_family_schema_ref` and
  a `canonical_packet_ref` equal the owning primitive's, with
  `references_canonical_not_local_prose = true`. All four consumer groups and all
  ten families are adopted, and at least one family is reused across two distinct
  groups.
- **AC2 — label & state parity:** target-context identity stays visible on every
  surface (`target_context_ref` non-empty); badges, degraded-state vocabulary,
  and the mandatory required labels are preserved; adapter source and confidence
  never contradict (a heuristic / imported / overlay source cannot claim high
  confidence). A narrower consumer discloses the reduction with a
  reduced-capability banner whose `capability_state` matches its authority mode,
  and carries a companion / browser / handoff note whenever it hands off. A
  full-interactive consumer carries no spurious banner.
- **AC3 — help/support/release parity:** at least one `docs_help`,
  `support_export`, or `release_proof` surface references the canonical
  component families, so AI/explainer and incident/support lanes cite the same
  target-context and confidence primitives users saw in the original UI.
- **Copy/export parity:** every row keeps text / JSON / Markdown carriers and
  forbids screenshot-only export.
- The stored `summary` must equal the recomputed summary.

## Regenerating the artifacts

```sh
B=artifacts/release/m5-manifest-build-component-consumer-proof
cargo run -p aureline-infra --bin emit_manifest_build_component_consumers_fixture -- support > $B/support_export.json
cargo run -p aureline-infra --bin emit_manifest_build_component_consumers_fixture -- csv     > $B/matrix.csv
cargo run -p aureline-infra --bin emit_manifest_build_component_consumers_fixture -- summary > $B/report.md
cp $B/support_export.json $B/matrix.csv fixtures/ui/m5-manifest-build-component-consumers/
```

`checked_support_export_matches_builder` pins the on-disk artifact byte-for-byte
to `seeded_m5_manifest_build_consumer_packet()`.
