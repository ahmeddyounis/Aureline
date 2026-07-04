# M5 Manifest / Build Component Qualification Contract (M05-819)

This contract is the **one-bundle qualification capstone** that CLOSES the B95
manifest / build component lane. It proves that every claim-bearing infrastructure
and execution surface either passes the shared component parity check on every
dimension or **narrows automatically and discloses it**, and that release, help,
and support packets can cite a **single certification bundle** for the whole lane.
It sits on top of the frozen
[M5 manifest / build component matrix](m5_manifest_build_component_matrix.md)
(M05-812) and its 813–818 implementation, execution, consumer, and accessibility
lanes.

- **Rust module:** `crates/aureline-infra/src/qualify_shared_manifest_build_components_across_every_claimed_consumer_with_one_certification_bundle/`
- **Boundary schema:** [`schemas/ui/m5-manifest-build-component-qualification.schema.json`](../../schemas/ui/m5-manifest-build-component-qualification.schema.json)
- **Support export / certification bundle (canonical):** [`artifacts/release/m5-manifest-build-component-qualification-proof/support_export.json`](../../artifacts/release/m5-manifest-build-component-qualification-proof/support_export.json)
- **Matrix CSV / report:** `artifacts/release/m5-manifest-build-component-qualification-proof/{matrix.csv,report.md}`
- **Fixtures:** [`fixtures/ui/m5-manifest-build-component-qualification/`](../../fixtures/ui/m5-manifest-build-component-qualification/)
- **Emit bin:** `cargo run -p aureline-infra --bin emit_manifest_build_component_qualification_fixture -- {support|csv|summary}`

## Scope

The packet carries one `ManifestBuildQualificationRow` per claimed
`M5QualifiedManifestBuildConsumer` (all 8: infrastructure surface, live-resource
surface, execution launcher, incident support, handoff consumer, support packet,
help center, release evidence). Each row keys on the claim-bearing **consumer** and
reuses the frozen matrix vocabulary — required labels and downgrade triggers —
rather than minting synonyms, so the qualified labels stay byte-identical to the
matrix and the sibling primitive packets. The packet is **metadata-only**: raw
manifests, adapter payloads, credentials, and provider bodies never cross the
boundary; only typed class tokens, opaque summary / evidence refs, booleans, target
IDs, and redacted labels are recorded.

## Parity dimensions

Each consumer is qualified against five parity dimensions
(`M5ManifestBuildQualificationDimension`), each carrying a
`M5ManifestBuildParityState` of `certified`, `disclosed_narrowed`, or
`undisclosed_drift`:

1. **Target context** (`target_context`). The consumer keeps the target context
   visible on every read- or mutate-capable surface; it never blurs which target it
   acts on. `target_context_ref` is non-empty on every row.
2. **Schema freshness** (`schema_freshness`). Schema freshness stays explicit; a
   stale schema is never presented as current.
3. **Truth-layer labels** (`truth_layer_labels`). Authored / rendered / planned /
   live / cached / provider-overlay truth stays distinct rather than collapsing into
   one label.
4. **Adapter source kind** (`adapter_source_kind`). Native-vs-fallback adapter
   provenance stays explicit; lower-confidence discovery never overwrites
   higher-confidence truth silently.
5. **Accessibility / export behavior** (`accessibility_export_behavior`). A
   non-visual fallback and a text / JSON / Markdown export are preserved (never a
   screenshot alone), and the export preserves the same target IDs, adapter kinds,
   and freshness / confidence states shown in-product.

A `disclosed_narrowed` dimension carries a frozen
`M5ManifestBuildDowngradeTrigger` and a precise, non-generic reason label. An
`undisclosed_drift` dimension is never honest.

## Verdict

`ManifestBuildQualificationRow::verdict()` derives:

- **`blocked`** (red) — the consumer forks its own components, drops the target
  context, hides drift, narrows without a trigger / precise reason, omits a parity
  dimension, drops a mandatory export field, or (when narrowed) drops the narrowed
  reason from the export. Blocked rows may not promote.
- **`qualified_with_narrowing`** (yellow) — every dimension is honest and at least
  one disclosed a narrowing.
- **`qualified`** (green) — every parity dimension is certified.

## Acceptance criteria

- **AC1** — Every claim-bearing surface uses the same shared components and tracks
  the same target-context and confidence truth (`uses_shared_components`,
  `preserves_target_context`, no hidden drift, honest narrowings).
- **AC2** — Release / help / support packets reference **one** certification bundle.
  Every row cites `certification_bundle_ref` (== the packet bundle) and draws from
  the consolidated `certified_component_packets`, which must contain every
  `canonical_component_packet_refs()` entry — the frozen matrix, the three primitive
  resolvers, the execution-confidence primitive, the consumer adoption, and the
  accessibility fallback. The support, help, and release evidence consumers are all
  qualified.
- **AC3** — The export preserves the same per-dimension parity, including the
  narrowed reason when the consumer narrowed, so support / release exports can
  reconstruct exactly why a consumer narrowed.

## Consolidated bundle

The one certification bundle (`certified_component_packets`) consolidates the seven
canonical B95 component packets:

| # | Lane | Packet |
| - | ---- | ------ |
| 1 | M05-812 freeze matrix | `artifacts/infra/m5-manifest-build-component-matrix/support_export.json` |
| 2 | M05-813 manifest authoring | `artifacts/release/m5-manifest-authoring-primitive-proof/support_export.json` |
| 3 | M05-814 live-resource navigation | `artifacts/release/m5-live-resource-navigation-primitive-proof/support_export.json` |
| 4 | M05-815 build confidence | `artifacts/release/m5-build-confidence-primitive-proof/support_export.json` |
| 5 | M05-816 execution confidence | `artifacts/release/m5-execution-confidence-primitive-proof/support_export.json` |
| 6 | M05-817 component consumers | `artifacts/release/m5-manifest-build-component-consumer-proof/support_export.json` |
| 7 | M05-818 accessibility fallback | `artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/support_export.json` |

## Seeded status

The checked-in packet is **5 green / 3 yellow / 0 red**. The three yellow rows each
demonstrate one honest, disclosed narrowing on a distinct dimension:

- Live-resource surface — rendered-vs-live divergence narrows `truth_layer_labels`
  (`drift_from_source`).
- Execution launcher — heuristic adapter fallback narrows `adapter_source_kind`
  (`adapter_unavailable`).
- Incident support — a stale manifest schema mirror narrows `schema_freshness`
  (`schema_stale`).

## Regeneration

The `seeded_m5_manifest_build_component_qualification_packet()` builder is the one
source of truth shared by the tests, the emit bin, and the on-disk support export.
Regenerate the artifacts with the emit bin (`support` → `support_export.json`, `csv`
→ `matrix.csv`, `summary` → `report.md`) and copy `support_export.json` / `matrix.csv`
into the fixtures directory. The `on_disk_export_matches_builder` test fails if the
checked-in export drifts from the builder.
