# M5 orientation aids: multi-cursor, fold-state, breadcrumb, minimap, overview-ruler, and degraded-orientation truth

This contract makes orientation aids an explicit, reviewable product surface
across the new Milestone 5 editors, viewers, diffs, and browser-runtime overlays —
notebook fold gutters, data/API result minimaps, docs overview rulers, preview
breadcrumbs, review-diff minimaps, browser-runtime overlays, and provider-linked
companion breadcrumbs, plus the editor-core baseline — instead of aids that
silently disappear or show stale markers when a surface is constrained. It binds
the orientation-aid half of the frozen
[keyboard-continuity matrix](./freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md):
*orientation aids degrade honestly* — a collapse to no aids is a downgrade, never
a silent removal. Each aid names its kind, summarizes the markers it reflects,
aligns its identity with the same object IDs shown elsewhere, and discloses every
constraint that reduces, degrades, or disables it.

The canonical packet is built by
`aureline_shell::implement_multi_cursor_fold_state_breadcrumb_minimap_overview_ruler_and_degraded_orientati`.

## What each record binds

An `OrientationAidRecord` binds one claimed M5 surface (keyed by a
`KeyboardSurfaceKind` and a non-display `KeyboardSurfaceSubject`) to one
orientation aid:

- the `OrientationAidKind` — which aid the record renders: **multi-cursor** count,
  **fold-state** summary, **breadcrumb** path, **minimap**, or **overview-ruler**
  markers;
- the `OrientationMarkerSummary` — the object class, an opaque / workspace-relative
  object token aligned with the same object identity shown elsewhere, the marker
  count and the count actually rendered live, and a reviewable label, so the aid
  names *how many* cursors / folds / markers exist and *how many* are drawn rather
  than silently dropping the surplus;
- the `OrientationAidClass` posture from the frozen matrix vocabulary;
- a reopenable `AxisVerification` proof and the resolved
  `OrientationDisclosureClass`.

## Triggers and the minimum-disclosure floor

An aid is **never flattened into a fully-active claim when it is constrained**.
Each condition below fires an `OrientationContractTrigger` that imposes a
minimum-disclosure floor on the resolution, so the record can never resolve to a
flat `aid_fully_active` when a trigger fires:

| Trigger | Condition | Minimum resolution |
| --- | --- | --- |
| `high_cardinality_markers` | rendered markers < total markers | `count_summary_preserved` |
| `cross_surface_identity_shared` | the aid identity is shared across surfaces | `identity_aligned_across_surfaces` |
| `constrained_viewport` | a constrained viewport reduces detail | `reduced_detail_disclosed` |
| `reduced_motion_profile` | a reduced-motion profile suppresses animation | `motion_reduced_disclosed` |
| `large_or_unsafe_artifact` | the aid reflects a large / unsafe artifact | `degraded_disclosed` |
| `limited_capability_profile` | a limited capability profile disables the aid | `unavailable_disclosed` |
| `stale_or_missing_orientation_proof` | the orientation proof is stale / missing | `degraded_disclosed` |

The required floor is the maximum over all fired triggers. A record that flattens
a constrained aid into the bare fully-active baseline, or that resolves below its
required floor, fails `OrientationAidPacket::validate`.

## Disclosures stay distinct and honest

The `OrientationDisclosureClass` ladder keeps fully-active, count-summary,
identity-alignment, reduced-detail, motion-reduced, degraded, and unavailable
states **distinct**. Each non-baseline resolution must cite exactly the precise
label it requires — a generic non-answer (`degraded`, `reduced`, `hidden`,
`minimap`, …) is rejected. The advertised `OrientationAidClass` posture must match
the one the resolution implies, and no valid record ever advertises the
`orientation_aids_absent_downgraded` (silently-removed) posture: a degraded or
unavailable aid is disclosed in place with a reason an accessibility or support
surface can read back.

## Identity alignment and stale markers

A breadcrumb, minimap, or overview-ruler whose identity is shared across surfaces
resolves each segment / marker to the same underlying object ID the editor,
outline, and explorer use, so jumping from the aid lands on the identical object
rather than a re-derived guess. A degraded aid drops the markers it can no longer
keep current and says so rather than showing stale ones; the `stale_markers_shown`
guardrail is always false. A provider-linked surface is verified with imported
proof and never reads as a locally verified orientation truth.

## Boundary safety

Raw provider payloads, file contents, and absolute private paths never cross this
boundary. The packet carries only typed class tokens, booleans, opaque / relative
ids, fingerprint digests, and redaction-aware reviewable labels.

## Artifacts

- Schema:
  `schemas/interaction/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.schema.json`
- Support export:
  `artifacts/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati/support_export.json`
- Markdown summary:
  `artifacts/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.md`
- Protected fixtures:
  `fixtures/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati/`

Regenerate the checked artifacts with
`cargo run -p aureline-shell --example dump_orientation_aids [support|summary|fixture]`.
