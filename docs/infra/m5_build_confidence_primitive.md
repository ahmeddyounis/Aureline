# M5 build / run confidence primitive contract

This packet narrows the five remaining build / run families of the frozen
[M5 manifest / build component matrix](./m5_manifest_build_component_matrix.md)
— the adapter-source badge, the target-graph row, the capability matrix, the
raw-event drawer, and the fallback-confidence drawer — into one working
primitive with a real **resolver**. A single build-target context resolves onto
five surfaces that share one target identity and one disclosed adapter source,
so native / protocol-backed truth and heuristic / imported fallback truth stay
honest and **inspectable before** any target is trusted or executed.

The primitive is minted and validated in
[`crates/aureline-infra`](../../crates/aureline-infra/src/implement_the_m5_adapter_source_badge_target_graph_capability_matrix_raw_event_and_fallback_confidence_primitive/mod.rs)
(`record_kind = m5_build_confidence_primitive`, `schema_version = 1`). The Rust
resolver `resolve_build_confidence()` and the builder
`seeded_m5_build_confidence_packet()` are the source of truth; the checked-in
artifacts below are byte-for-byte emissions of the builder
(`current_stable_m5_build_confidence_export()` re-reads the support export via
`include_str!`).

If this doc, the machine-readable schema, and the checked-in artifacts disagree,
the schema plus the Rust builder win and all companion artifacts update in the
same change.

## The resolver

`resolve_build_confidence(&M5BuildConfidenceInput)` projects one build-target
context onto:

- **`M5ResolvedAdapterSourceBadge`** — the adapter source kind, its confidence
  chip, whether the source is native, and the always-true guarantees that the
  source kind is rendered explicitly and the confidence chip is consistent with
  the source.
- **`M5ResolvedTargetGraphRow`** — the typed target identity (node kind, stable
  id, owning module, workspace root), the truth class, edge confidence,
  freshness, the supported verbs, the required environment, and that target
  context stays visible.
- **`M5ResolvedCapabilityMatrix`** — the adapter source, confidence, one resolved
  cell per requested verb (each flagged when downgraded below full support), the
  list of downgraded verbs, and that the sheet discloses its source and
  confidence.
- **`M5ResolvedRawEventDrawer`** — the raw-event provenance channel, the adapter
  version, the payload-lineage chain, the always-true redaction /
  identity-preservation guarantees, and the export / copy actions.
- **`M5ResolvedFallbackConfidenceDrawer`** — the structured-versus-heuristic
  confidence state, the fallback reason (present only for a fallback), the
  recovery route, the fallback note, the never-silently-overwrite guarantee, and
  the reconstructable downgrade trigger.

All five share one `target_id` and one disclosed `adapter_source`, so native and
fallback lanes never blur across the surfaces.

## Acceptance criteria

- **AC1 — adapter provenance is never hidden.** The badge always renders its
  adapter source kind explicitly and keeps its confidence chip consistent with
  that source, so a heuristic parse or imported snapshot can never claim native
  authority (`AdapterConfidenceInconsistent`), and a native lane can never
  masquerade as a fallback (`AdapterFallbackMismatch`). The fallback drawer
  discloses the structured-versus-heuristic state in lockstep with the badge.
- **AC2 — target identity and confidence stay inspectable before action.** The
  target-graph row preserves the stable target id, owning module / root,
  freshness, supported verbs, and required environment; the capability matrix
  explains supported verbs and downgraded actions; a supported verb is never
  claimed from an unknown-confidence source
  (`SupportedCapabilityUnknownConfidence`) — all before any run / test / debug
  action is offered.
- **AC3 — support and AI consumers reuse the same component truth.** The
  raw-event drawer redacts payloads to typed tokens, preserves stable event
  identity and payload lineage, names the adapter version, and must offer an
  export / copy action (`NoExportActionOffered`), so support and AI surfaces
  reconstruct the same target ids, adapter kinds, and freshness / confidence
  states shown in-product instead of re-deriving them from logs.

## Controlled vocabulary

The primitive mints `M5BuildConfidenceSurfaceFamily` (6), `M5BuildVerb` (6),
`M5BuildActionKind` (5), and `M5BuildConfidenceExportField` (7, five mandatory).
It reuses, without re-declaring, the frozen matrix vocabulary:
`M5AdapterSourceKind`, `M5CapabilityState`, `M5RawEventChannel`,
`M5TargetGraphNodeKind`, `M5FallbackConfidenceState`, `M5FallbackReason`,
`M5FallbackRecoveryRoute`, `TruthMode`, `M5ResourceFreshness`,
`M5DiscoveryConfidence`, and `M5ManifestBuildDowngradeTrigger`. The
`M5BuildConfidenceVocabularySet::canonical()` set is frozen so later M5 rows
cannot invent a parallel build-confidence vocabulary.

## Redaction

Raw build output, event payloads, credentials, and endpoint data never cross
this boundary. The resolver carries only opaque refs, typed class tokens,
booleans, and redacted labels; `value_is_forbidden` rejects obvious secret
material on input and the packet re-scans its own export
(`RawMaterialInExport`), so support and diagnostics exports reconstruct exactly
what a surface would have shown without leaking payloads.

## Checked-in artifacts

- Schema: [`schemas/ui/m5-build-confidence-primitive.schema.json`](../../schemas/ui/m5-build-confidence-primitive.schema.json)
- Support export (`include_str!` canonical):
  [`artifacts/release/m5-build-confidence-primitive-proof/support_export.json`](../../artifacts/release/m5-build-confidence-primitive-proof/support_export.json)
- Matrix CSV and Markdown report under the same proof directory.
- Fixtures: [`fixtures/ui/m5-build-confidence-primitive/`](../../fixtures/ui/m5-build-confidence-primitive/)
  (byte-identical copies of the support export and CSV).

The fixture-emitting bin is
`cargo run -p aureline-infra --bin emit_build_confidence_primitive_fixture -- support|csv|summary`.
