# M5 execution-confidence primitive contract

This packet ships the reusable **execution-lane** confidence primitive that keeps
downgraded discovery or fallback results from masquerading as native, protocol-backed
truth once a target actually runs. Where the
[M5 build / run confidence primitive](./m5_build_confidence_primitive.md) narrows the
*static* build-confidence surfaces (badge, target-graph row, capability matrix,
raw-event drawer, fallback drawer), this packet narrows the surfaces those feed at
launch time: an **adapter-drift banner**, a run / test / debug **launcher state**, the
**launcher-state parity** fan-out onto problem surfaces / artifact views / follow-on
automation / AI, and a no-higher-confidence **overwrite guard**. A single
execution-target context resolves onto all four surfaces sharing one target identity
and one disclosed adapter source.

The primitive is minted and validated in
[`crates/aureline-infra`](../../crates/aureline-infra/src/implement_the_m5_adapter_drift_banner_launcher_state_and_no_higher_confidence_overwrite_primitive/mod.rs)
(`record_kind = m5_execution_confidence_primitive`, `schema_version = 1`). The Rust
resolver `resolve_execution_confidence()` and the builder
`seeded_m5_execution_confidence_packet()` are the source of truth; the checked-in
artifacts below are byte-for-byte emissions of the builder
(`current_stable_m5_execution_confidence_export()` re-reads the support export via
`include_str!`).

If this doc, the machine-readable schema, and the checked-in artifacts disagree, the
schema plus the Rust builder win and all companion artifacts update in the same change.

## The resolver

`resolve_execution_confidence(&M5ExecutionConfidenceInput)` projects one
execution-target context onto:

- **`M5ResolvedAdapterDriftBanner`** — the prior versus current adapter, whether the
  adapter changed, the per-verb capability delta (gained / retained / downgraded /
  lost), the gained / downgraded / lost verb lists, the affected targets, the
  divergence note, and the offered actions. `drift_detected` is true whenever the
  adapter changed or any verb dropped, and the banner is always visible before action.
- **`M5ResolvedExecutionLauncher`** — the current adapter source, confidence, and
  freshness, plus one affordance per verb (`available` / `narrowed` / `blocked`)
  derived from the delta, the blocked-verb list, and whether the launcher narrowed
  before launch.
- **`M5ResolvedParityConsumer`** (one per declared consumer) — the problem surface,
  artifact view, follow-on automation, or AI action, each carrying the same adapter
  source and confidence.
- **`M5ResolvedOverwriteGuard`** — the existing versus incoming adapter and
  confidence, the verdict (`promoted_higher_confidence` /
  `matched_existing_confidence` / `recorded_explicit_downgrade`), whether an explicit
  downgrade was recorded, the always-true preservation / never-silently-overwrite
  guarantees, the downgrade note, and the reconstructable downgrade trigger.

All four share one `target_id` and one disclosed `current_adapter`.

## Acceptance criteria

- **AC1 — execution lanes narrow affordances before launch when adapter capability
  drops.** The launcher derives each verb's affordance from the prior-versus-current
  delta: a lost verb is `blocked` and a downgraded verb is `narrowed` to inspect-only
  *before* any run / test / debug action. A supported verb is never claimed from an
  unknown-confidence source (`SupportedVerbUnknownConfidence`).
- **AC2 — adapter drift and affected targets are visible before action.** When drift
  is detected the banner must name the affected targets
  (`DriftWithoutAffectedTargets`), carry a precise divergence note
  (`DriftWithoutDivergenceDetail`), and offer recompute and open-diagnostics actions
  (`DriftWithoutRecoveryActions`), so users never trigger a failed rerun to discover
  the drift. Every affected target must be a stable identity (`AffectedTargetNotStable`).
- **AC3 — lower-confidence results never masquerade as native truth.** The overwrite
  guard refuses to replace existing higher-confidence truth
  (`SilentHigherConfidenceOverwrite`) or existing native truth
  (`SilentNativeMasquerade`) unless a downgrade is acknowledged, in which case the
  higher-confidence truth is preserved and the downgrade is named
  (`DowngradeWithoutNote`); a downgrade note without a downgrade is rejected
  (`DowngradeNoteWithoutDowngrade`). The adapter source and confidence ride into every
  launcher-state-parity consumer, and an export / copy action must be offered
  (`NoExportActionOffered`).

## Controlled vocabulary

The primitive mints `M5ExecutionSurfaceFamily` (5), `M5ExecutionParitySurface` (4),
`M5CapabilityDeltaKind` (4), `M5AffordanceState` (3), `M5OverwriteVerdict` (3),
`M5ExecutionActionKind` (6), and `M5ExecutionExportField` (7, five mandatory). It
reuses, without re-declaring, `M5BuildVerb` from the build-confidence primitive and the
frozen matrix vocabulary: `M5AdapterSourceKind`, `M5CapabilityState`,
`M5FallbackConfidenceState`, `TruthMode`, `M5ResourceFreshness`,
`M5DiscoveryConfidence`, and `M5ManifestBuildDowngradeTrigger`. The
`M5ExecutionConfidenceVocabularySet::canonical()` set is frozen so later M5 rows cannot
invent a parallel execution-confidence vocabulary.

## Redaction

Raw build output, event payloads, credentials, and endpoint data never cross this
boundary. The resolver carries only opaque refs, typed class tokens, booleans, and
redacted labels; `value_is_forbidden` rejects obvious secret material on input
(including inside affected-target identities) and the packet re-scans its own export
(`RawMaterialInExport`), so support and diagnostics exports reconstruct exactly what a
surface would have shown without leaking payloads.

## Checked-in artifacts

- Schema: [`schemas/ui/m5-execution-confidence-primitive.schema.json`](../../schemas/ui/m5-execution-confidence-primitive.schema.json)
- Support export (`include_str!` canonical):
  [`artifacts/release/m5-execution-confidence-primitive-proof/support_export.json`](../../artifacts/release/m5-execution-confidence-primitive-proof/support_export.json)
- Matrix CSV and Markdown report under the same proof directory.
- Fixtures: [`fixtures/ui/m5-execution-confidence-primitive/`](../../fixtures/ui/m5-execution-confidence-primitive/)
  (byte-identical copies of the support export and CSV).

The fixture-emitting bin is
`cargo run -p aureline-infra --bin emit_execution_confidence_primitive_fixture -- support|csv|summary`.
