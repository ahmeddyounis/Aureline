# M5 Source-Round-Trip Honesty Primitive: Sync Chip, Conflict Banner, Unsupported Card, and Boundary Notice

This contract implements the source-sync-chip, round-trip-conflict-banner, and
unsupported-construct-card families frozen in the M5 visual-designer component
matrix (tracked under `docs/preview/m5/m5_visual_designer_component_matrix.md`),
plus a generated-or-protected-file boundary notice, as **one reusable
source-round-trip honesty primitive**. Aureline resolves a single designer
target's round-trip situation once and projects it onto a source-sync chip, an
optional conflict banner or unsupported-construct card, and an optional boundary
notice that share a single target identity, so a canvas edit never implies broader
or safer write authority than the source model actually supports.

Where the [selected-node primitive](m5_visual_designer_selected_node_primitive_contract.md)
implements the canvas / tree / inspector families, this primitive implements the
round-trip-honesty families that keep writeback honest.

- **Module:** `crates/aureline-preview/src/implement_the_m5_source_sync_chip_round_trip_conflict_and_generated_or_protected_boundary_primitive`
- **Boundary schema:** [`schemas/ui/m5-source-round-trip-honesty-primitive.schema.json`](../../schemas/ui/m5-source-round-trip-honesty-primitive.schema.json)
- **Checked support export:** `artifacts/release/m5-source-round-trip-honesty-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-source-round-trip-honesty-proof/matrix.csv`
- **Report:** `artifacts/components/m5-source-round-trip-honesty-primitive.md`
- **Fixtures:** [`fixtures/ui/m5-source-round-trip-honesty-primitive/`](../../fixtures/ui/m5-source-round-trip-honesty-primitive/)

## The two halves

### 1. The resolver

`resolve_round_trip_status(&M5RoundTripStatusInput) -> Result<M5ResolvedRoundTripStatus, M5RoundTripResolutionError>`
takes one designer target — its identity, node / file labels, source-sync class,
round-trip capability, source-boundary class, protected-path posture, an unsaved
flag, an optional source span, an optional round-trip conflict, and an optional
unsupported construct — and produces:

- **Source-sync chip** (`M5ResolvedSourceSyncChip`): the chip state
  (`in_sync` / `unsaved` / `needs_refresh` / `unsupported_construct` / `conflict`),
  a recovery route consistent with the sync class, and paired open-source /
  open-diff actions when a source or a diff is available.
- **Round-trip conflict banner** (`M5ResolvedConflictBanner`, when a conflict is
  open): the affected node and file, the conflict class, the conflict resolution
  route, an exact source-first fallback, refresh (reload-source) and compare
  (open-diff) actions, and the always-true `never_silent_writeback` /
  `preserves_selection_context` guards.
- **Unsupported-construct card** (`M5ResolvedUnsupportedCard`, when the target
  cannot round-trip): the affected node and file, the reason class, an exact
  source-first fallback, an open-source action, and a preserved selection context.
- **Boundary notice** (`M5ResolvedBoundaryNotice`, when the file is not
  author-owned source): the boundary class, whether the designer may write at all,
  whether an owner / managed-file flow is required, and the always-true
  `refuses_silent_widening` guard.
- **Honest write authority** (`M5WriteAuthority`): `writable`,
  `writable_with_review`, `source_only_fallback`, or `read_only` — the exact
  authority the source model supports.

### 2. The parity matrix

`M5RoundTripHonestyPacket` binds one row per claimed M5 visual-design surface
family — desktop designer, source-first preview, browser-runtime inspector,
framework-pack preview, embedded shell designer, and support-export replay — and
carries worked resolution cases whose stored resolution must equal a fresh resolve
of its input. The Rust validator is the authoritative gate.

## Acceptance criteria and how they are enforced

| Acceptance criterion | Mechanism |
| --- | --- |
| Unsupported constructs, manual source edits, generated sections, and protected files cannot be silently normalized away by the designer. | Whenever a hard block is present (a conflict banner, an unsupported-construct card, or a non-writable boundary notice) the resolved write authority is narrowed to a source-first fallback or read-only — never a plain writable surface. `refuses_silent_normalization` proves it, and the `SilentNormalizationUnproven` lint requires a worked blocked case and that every case refuses silent normalization. |
| Users get exact source-first fallbacks instead of best-effort writeback when round-trip support drops. | When the write authority does not write back, the resolver names an exact `M5SourceFirstFallback` route (reload-then-reapply, keep-source-discard-visual, open-source-directly, open-managed-file-owner-flow, or inspect-only). `offers_source_first_fallback` proves it, and the `SourceFirstFallbackUnproven` lint requires a worked fallback case and that every case offers one. |
| Support and release packets can explain why a visual-designer surface narrowed or went read-only. | Every narrowing of a round-trip that could otherwise write back carries a typed `M5VisualDesignerDowngradeTrigger` and a precise label. `narrowing_is_explained` proves it, and the `NarrowingExplanationUnproven` lint requires a worked narrowed case with a trigger and that every case is explained and identity-consistent. |

## Honesty guards enforced by the resolver

- A round-trip that writes back to source with no source span is refused with
  `MissingSpanForSourceRoundTrip` — a surface cannot claim a write-back it has no
  span for.
- A runtime-only-no-source surface carrying a source span is refused with
  `ContradictoryRuntimeSpan`.
- A generated / managed, protected read-only, or external / vendored boundary is
  never designer-writable; the write authority is read-only and the notice routes
  through an owner / managed-file flow.

## Boundary hygiene

Raw source bodies, diff hunks, file contents, credentials, and raw provider
payloads never cross this boundary. Node and file labels and the serialized packet
are screened for forbidden material (`api_key`, `password`, `secret`, bearer
tokens, URLs, PEM blocks); a violation fails resolution or packet validation.

## Reused vocabulary

The source-sync class (`SourceSyncClass`), round-trip capability class
(`RoundTripCapabilityClass`), sync-recovery route (`M5SyncRecoveryRoute`),
round-trip conflict class (`M5RoundTripConflictClass`), conflict resolution route
(`M5ConflictResolutionRoute`), unsupported-construct reason
(`UnsupportedConstructReason`), protected-path posture (`ProtectedPathPosture`),
downgrade triggers (`M5VisualDesignerDowngradeTrigger`), preview surface
(`PreviewSurface`), and visual-design surface families
(`M5VisualDesignSurfaceFamily`) are reused verbatim from the frozen visual-designer
component matrix and the sibling primitives. This module mints new vocabulary only
for the source-sync chip state (`M5SourceSyncChipState`), the source-boundary class
(`M5SourceBoundaryClass`), the honest write authority (`M5WriteAuthority`), the
source-first fallback route (`M5SourceFirstFallback`), and the export fields
(`M5RoundTripExportField`).
