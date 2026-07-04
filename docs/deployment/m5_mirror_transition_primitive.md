# M5 mirror-transition primitive

The **mirror-transition primitive** is the reusable set of mirror/offline artifact
rows, a mode-change / disconnect review sheet, and a channel-association review row that
update-center, mirror-manager, admin, diagnostics, support, and docs surfaces ingest
instead of cloning a bespoke offline banner, a diagnostics pane, or an admin-only
dashboard. One transition context resolves into all three surfaces and they share one
transition identity, so mirror freshness, signature verification, cache reuse /
invalidation, rollback truth, and handler ownership never blur across them.

It **narrows** the remaining three operational families of the frozen
[deployment/continuity component matrix](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
— `mirror_offline_artifact_row`, `mode_change_review_sheet`, and
`channel_association_review_row` — into one working resolver
(`resolve_mirror_transition`) rather than restating install / deployment truth in
feature-local prose. It reuses the frozen matrix's operating-mode, provenance /
freshness, mirror-signature, boundary-change, and downgrade-trigger vocabulary; it adds
only the minted vocabulary the resolver needs (artifact class, mirror source class,
mirror/offline continuity state, artifact action, cache disposition, rollback path,
export field, and the parity surface families).

- **Boundary schema:**
  [`schemas/ui/m5-mirror-transition-primitive.schema.json`](../../schemas/ui/m5-mirror-transition-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-deployment-continuity-component-matrix.schema.json`](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-mirror-transition-primitive-proof/support_export.json`](../../artifacts/release/m5-mirror-transition-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-mirror-transition-primitive/`](../../fixtures/ui/m5-mirror-transition-primitive/)
- **Implementation:**
  `crates/aureline-install/src/implement_the_m5_mirror_offline_mode_change_and_channel_association_primitive/`

## What the resolver projects

`resolve_mirror_transition(&M5MirrorTransitionInput)` returns a
`M5ResolvedMirrorTransition` with three surfaces that all carry the same
`transition_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Mirror/offline artifact rows | `Vec<M5ResolvedMirrorArtifactRow>` | each artifact's source class, artifact class, signature / digest verification, freshness, one shared continuity state, pin/offline note, and verify / open-manifest actions for docs, extensions, models, updates, and policy bundles |
| Mode-change / disconnect review sheet | `M5ResolvedModeChangeReviewSheet` | preserved local state, affected managed features, cache reuse / invalidation, overall artifact posture, the rollback path, and the export-before-change action |
| Channel-association review row | `M5ResolvedChannelAssociationRow` | the channel / handler association, the disclosed current owner, and the reviewed-before-apply guarantee |

## One shared continuity vocabulary

The primitive keeps one vocabulary for the mirror/offline states across UI, docs / help,
and support exports — `Mirror unavailable`, `Offline cache only`, `Verification failed`,
`Needs refresh`, plus `Current verified` and `Pinned offline`. Every artifact row derives
its `M5MirrorContinuityState` from the same `derive(freshness, signature,
mirror_reachable, pinned_offline, needs_refresh)` function, so the same artifact renders
the same state on every surface.

## Acceptance criteria the resolver proves

- **AC1 — offline and mirror transitions never read like generic warnings.** Every
  artifact row names its source class, artifact class, verification state, and shared
  continuity state, and the mode-change sheet names exactly what will stale, what remains
  usable, and how to reverse the change. Mirrored / cached / imported content shown as a
  current live source is rejected as `StaleShownAsCurrent`.
- **AC2 — artifact verification / manifests remain accessible from the same component
  family across deployment profiles.** Every artifact row keeps a verify-signature and an
  open-manifest action reachable regardless of deployment mode; a hidden verification or
  manifest is rejected as `VerificationNotAccessible` / `ManifestNotAccessible`.
- **AC3 — mode changes preserve export-before-change and rollback truth.** The review
  sheet keeps a preserved-local-state ref, an export-before-change action, and a rollback
  path; a change forced without review or without an export path is rejected as
  `ChangeNotReviewed` / `BlindSwitchWithoutExport`, and a channel association that
  silently captures a default handler is rejected as `LastWriterWinsCapture`.

## Honesty guarantees

- Raw config bytes, credentials, license keys, mirror URLs, and device identifiers never
  cross this boundary; the resolver carries only opaque refs, typed class tokens,
  booleans, and redacted labels.
- A degraded input must carry a precise, non-generic label; a generic non-answer
  (`unavailable`, `error`, `offline`, …) is rejected.
- The support / export packet reconstructs exactly what each surface would have shown:
  every worked case stores both its input and its resolved projection, and validation
  re-runs the resolver so a stored projection can never drift from the live resolver.

## Parity matrix

The `M5MirrorTransitionPrimitivePacket` binds each of the six mirror surface families
(update center, mirror manager, admin deployment console, diagnostics mirror, support /
export replay, docs mirror reference) to the shared contract with worked resolution
cases, a frozen controlled-vocabulary set, governance-review and consumer-projection
blocks, and a release / support parity posture. See the
[matrix CSV](../../artifacts/release/m5-mirror-transition-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-mirror-transition-primitive-proof/report.md)
for the per-surface summary.
