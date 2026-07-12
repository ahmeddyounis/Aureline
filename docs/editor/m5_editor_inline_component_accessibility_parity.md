# M5 Editor-Inline-Component Accessibility & Auto-Narrowing (M05-1122)

This contract governs the **keyboard / screen-reader / high-zoom / reduced-motion / CLI / export
parity and honest automatic claim narrowing** capstone over the frozen M5 editor-inline component
matrix (`schemas/ui/m5-editor-inline-component-matrix.schema.json`, workstream **B133**). It is the
accessibility / export / narrowing sibling of the implementation lanes (M05-1117 … M05-1120) that
resolve per-surface truth for the editor tab, gutter, diagnostic decoration, code-action chip, diff
view, review thread, AI message card, and evidence timeline primitives, and of the consumer-adoption
capstone (M05-1121).

- **Schema:** `schemas/ui/m5-editor-inline-component-accessibility-parity.schema.json`
- **Support export (canonical):**
  `artifacts/release/m5-editor-inline-component-accessibility-parity/support_export.json`
- **Matrix CSV:**
  `artifacts/release/m5-editor-inline-component-accessibility-parity/matrix.csv`
- **Report:** `artifacts/release/m5-editor-inline-component-accessibility-parity.md`
- **Mirror fixtures:**
  `fixtures/ui/m5-editor-inline-component-accessibility-parity/`
- **Rust module:** `aureline-editor` →
  `m5_editor_inline_accessibility_parity_and_narrowing_when_evidence_truth_is_stale`
- **Regeneration:** `GEN_EDITOR_INLINE_A11Y_ARTIFACTS=1 cargo test -p aureline-editor --lib m5_editor_inline_accessibility_parity_and_narrowing`

## What each row certifies

Each row keys on one frozen `M5EditorInlineComponentFamily` and reuses the frozen required labels,
downgrade triggers, and consumer surfaces from the matrix (no parallel synonyms are minted). A row
certifies that the family:

1. **Reaches canonical truth via assistive tech.** A keyboard-complete, screen-reader-reachable,
   high-zoom-legible, reduced-motion-safe, and CLI/headless-reachable path exposes the same inline
   identity, state / disposition, anchor durability and freshness, severity / source, fix posture,
   confidence, approval state, and evidence lineage the rich component shows — never a color-only badge,
   a hover-only chip, or a motion-only cue. Structure-heavy families (the gutter's stacked markers, the
   diff view's hunks, the evidence timeline's lineage) additionally bind their structured layout to a
   flat list / textual path.
2. **Exports without a raw payload.** The support / release / CLI export reconstructs the component's
   meaning from typed tokens and opaque refs, copyable as text / JSON / Markdown; a raw document body,
   diff hunk, message transcript, or evidence blob is never the only export.
3. **Auto-narrows honestly.** When an editor / review / AI dimension weakens, the component's claim
   auto-narrows from `trusted_inline_result` / `reviewable_inline_result` to the exact permitted
   projection, names the binding dimension and the frozen downgrade trigger, and preserves the canonical
   component identity / last-known state. The underlying editor / review / AI truth is never dropped
   opaquely.

## The claim ladder and honest narrowing

`M5EditorInlineComponentClaim` ranks the postures a surface may present, strongest first:

| Claim | Meaning |
| --- | --- |
| `trusted_inline_result` | Fully current, durably anchored, attributed, confidence-clear, approval-clear, evidence-complete inline surface. |
| `reviewable_inline_result` | A reviewable read-only structure (gutter / diff / evidence timeline), not a trusted apply surface. |
| `anchor_unverified_projection` | Anchor durability stale / drifted — last-known identity preserved. |
| `severity_unverified_projection` | Severity / source attribution stale — last-known severity preserved. |
| `fix_posture_unverified_projection` | Fix posture only inferred — named an inferred fix requiring review. |
| `confidence_unverified_projection` | Confidence / source context stale — last-known confidence disclosed. |
| `approval_unverified_projection` | Review approval / outdated-versus-resolved unverified — last-known thread state preserved. |
| `evidence_lineage_projection` | Evidence lineage only partial / redacted — partial / redacted lineage disclosed. |

`M5EditorInlineComponentConditionState::cannot_be_shown_trusted` flags the five overclaim-risk states —
`anchor_durability_stale`, `severity_source_stale`, `fix_posture_inferred`, `confidence_stale`, and
`approval_unverified` — that must never keep a `trusted_inline_result` claim. `evidence_lineage_partial`
is deliberately **excluded**: a partial / redacted evidence lineage shown honestly with inspectable
structure is a disclosed-absence operation, not a truth overstatement, so it still auto-narrows to
`evidence_lineage_projection` but does not trip the `weak_state_shown_as_trusted` guardrail.

## Guardrails (mirroring the frozen matrix)

- Tab / marker / diagnostic state is never encoded by color alone.
- Comment anchors and AI evidence pointers never silently drift.
- Outdated and resolved review state are never blurred together.
- An inferred fix is never presented as an exact one.
- An evidence timeline is never hidden in an opaque log without inspectable structure.
- Every narrowed rendering surface discloses its reduction and preserves its labels, so editor, diff,
  review, notebook, AI, diagnostics, CLI-export, support-export, and product consumers stay aligned on
  the same narrowed state.

## Acceptance-criteria coverage

- **Every B133 component has non-visual and exported representations that preserve state, anchor, and
  evidence truth.** Eight rows cover the eight frozen families one-to-one; each offers keyboard /
  screen-reader / high-zoom / reduced-motion / CLI reach and an export-safe summary with no raw payload.
- **Stale or partial inline evidence causes visible narrowing rather than silent optimistic copy.** Six
  families auto-narrow to their permitted projection with a precise, non-generic label and the frozen
  trigger; the editor tab stays green and the gutter stays a disclosed-reduced reviewable structure.
- **Accessibility, export, and narrowing behaviors are proven in the first claimed B133 consumers.**
  Each row ships to at least the support export and product UI plus its two most relevant surfaces, and
  the full nine-surface consumer set is exercised across the packet.
