# M5 Structured-Artifact Review Component Matrix (Design QA)

Batch B114 · Row M05-964 · Track: structured-artifact diff/merge and compare-viewer component lane.

This design record freezes nine reusable components for structured-artifact
review so Design, release, help, and support packets reference one canonical
matrix instead of local widget descriptions. Source refs: UI/UX Spec §17.29
(binary/media/design-snapshot and structured-artifact review UX) with the
structured merge/trust rules in §§11, 13, 17, 18; UX Design System §16.42
(structured diff toolbars, merge rows, artifact badges, compare viewers); TDD
§7.4 (structured review and merge safety) and §8.34 (package/lockfile safety).

## Component anatomy

1. **Artifact identity bar** — artifact class, canonical source, parser/schema
   state. Read-only.
2. **Diff-mode switcher** — available structured diff modes and the active one;
   never silently collapses to raw.
3. **Structure row** — one structured path (cell/node/key/symbol) and its change
   class; nested structure is never flattened.
4. **Merge-decision row** — base/ours/theirs pick with explicit write-back
   safety; compare-only artifacts are never promoted to writable.
5. **Generated-artifact notice** — names the generating source of truth and
   marks the artifact regenerate-only.
6. **Rendered compare viewer** (Beta) — side-by-side render with an explicit
   render-trust class and export-safe raw fallback.
7. **Media-metadata rail** (Preview) — dimensions/encoding/provenance for
   media-like artifacts; missing metadata shown as missing.
8. **Redaction/trust badge set** — redaction, export, and safe-preview posture.
9. **Compare-summary card** — rolls up added/removed/changed structure, render
   trust, and safety without flattening classes into a single verdict.

## Fidelity narrowing vocabulary

`structured_faithful` → `structured_partial` → `schema_unrecognized` →
`render_untrusted` → `raw_fallback` → `redacted_or_withheld`. Every narrowing
step stays explicit and offers an export-safe fallback rather than a silent
downgrade.

## Accessibility / validation notes

- Every component exposes CLI/headless and support-export projections so the
  same truth is available without a rendered surface.
- Downgrade narrows the claim (Stable → Beta → Preview) rather than hiding a
  component; stale proof auto-narrows (`auto_narrow_on_stale`).
- Two components are intentionally narrowed: `rendered_compare_viewer` (Beta) and
  `media_metadata_rail` (Preview).

See the frozen packet at
`artifacts/release/m5-structured-artifact-review-proof/support_export.json` and
the contract doc at
`docs/review/m5/freeze_the_m5_structured_artifact_review_component_matrix.md`.
