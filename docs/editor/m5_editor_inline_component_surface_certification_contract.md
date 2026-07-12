# M5 editor-inline component surface certification contract (M05-1123)

This is the closing B133 capstone over the frozen M5 editor-inline component matrix
(`schemas/ui/m5-editor-inline-component-matrix.schema.json`). Where the freeze matrix defines
the eight reusable **editor tab**, **gutter**, **diagnostic decoration**, **code-action chip**,
**diff view**, **review thread**, **AI message card**, and **evidence timeline** components,
the M05-1117..1120 implement lanes narrow each one, the M05-1121 shared consumer lane aligns
their vocabulary, and the M05-1122 accessibility lane proves keyboard / screen-reader /
high-zoom / reduced-motion / CLI-export parity plus per-family auto-narrowing, this capstone
**certifies** that the shared inline component truth holds on every claimed M5 editor / review
/ AI operating profile — and auto-narrows any profile that cannot sustain it.

- Boundary schema:
  [`schemas/ui/m5-editor-inline-component-surface-certification.schema.json`](../../schemas/ui/m5-editor-inline-component-surface-certification.schema.json)
- Canonical proof bundle (release):
  `artifacts/release/m5-editor-inline-component-surface-certification-proof/`
- Fixtures mirror:
  `fixtures/ui/m5-editor-inline-component-surface-certification/`
- Implementing module (aureline-editor):
  `m5_editor_inline_component_surface_certification`

## What it certifies

The packet is keyed on the claimed **profile** a user, reviewer, or support engineer reads
inline editor / review / AI truth through — not on component family or implement lane. Eight
profiles are certified:

| Profile | Claim | Verdict | Families |
| --- | --- | --- | --- |
| `live_trusted_inline_surface` | `trusted_inline_result` | green | editor tab, code-action chip |
| `reviewable_inline_structure` | `reviewable_inline_result` | green | diff view, gutter |
| `drifted_anchor_surface` | narrows to `anchor_unverified_projection` | yellow | diagnostic decoration, review thread |
| `stale_severity_decoration` | narrows to `severity_unverified_projection` | yellow | diagnostic decoration, gutter |
| `inferred_fix_chip` | narrows to `fix_posture_unverified_projection` | yellow | code-action chip |
| `stale_confidence_message` | narrows to `confidence_unverified_projection` | yellow | AI message card |
| `unverified_approval_thread` | narrows to `approval_unverified_projection` | yellow | review thread |
| `partial_evidence_timeline` | narrows to `evidence_lineage_projection` | yellow | evidence timeline |

Every one of the eight frozen component families is certified on at least one profile, so the
editor, diff, review, notebook, AI, and support/export lanes all trace back to the B133
component family.

Each row is scored on eight truth axes, each appearing exactly once:

1. **visual** — inline state, anchor durability, severity / source, fix posture, confidence /
   source context, approval / resolution, and evidence lineage on the primary surface, never
   by color alone.
2. **keyboard** — the same truth and its bounded local actions reachable without a pointer,
   never hover-only.
3. **screen_reader** — the same truth announced non-visually, never color/motion/glyph-only.
4. **high_zoom_reflow** — the same truth reflows legibly at high zoom.
5. **reduced_motion** — the same truth legible and usable with reduced motion.
6. **cli_export** *(always-on)* — the profile state reconstructable as text / JSON / Markdown.
7. **degraded_state** — a drifted anchor, stale severity, inferred fix, stale confidence,
   unverified approval, or partial evidence lineage honestly downgrades the claim rather than
   reading as a fresh authoritative inline result.
8. **inline_component_truth** — inline state, anchor durability, severity / source, fix
   posture, confidence / source context, approval / outdated-versus-resolved state, and
   evidence lineage stay explicit and never collapse into generic chrome.

## Invariants

- **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
  `trusted_inline_result` / `reviewable_inline_result` claim while a truth axis is not current
  is over-claiming and **blocks (red)**. A profile that discloses the reduction by narrowing
  its claim (with a bound, non-generic reason and a frozen downgrade trigger) is honestly
  **yellow**.
- **Only a live first-party trusted inline profile may certify `trusted_inline_result`.** Any
  reviewable, drifted, stale, inferred, unverified, or partial profile that keeps a trusted
  claim blocks.
- **CLI/export parity is always-on** and must stay certified so support and automation can
  reconstruct the inline state, anchor durability, severity / source, fix posture, confidence
  / source context, approval / outdated-versus-resolved state, and evidence lineage from the
  same component identity the user saw.
- **Certification may only narrow a claim, never strengthen it.**
- **All five B133 guardrails must hold** on every row (a breach blocks):
  1. tab / marker / diagnostic state must not be encoded by color alone;
  2. comment anchors and AI evidence pointers must not silently drift;
  3. outdated and resolved review state must not be blurred together;
  4. an inferred fix must not be presented as exact;
  5. an evidence timeline must not be hidden in an opaque log.

## Metadata-only boundary

Raw editor buffers, diff bodies, comment payloads, AI message bodies, credentials, secrets,
and endpoint refs never cross this boundary. The packet carries only typed class tokens,
opaque component refs, booleans, and controlled labels so support, release, and diagnostics
exports can reconstruct exactly what an accessible fallback would have shown without leaking
sensitive material.

## Regenerating the proof

The seed builder is the single source of truth shared by the tests and the on-disk export.
Regenerate the checked-in artifacts and fixtures with:

```sh
GEN_EDITOR_INLINE_CERT_ARTIFACTS=1 cargo test -p aureline-editor \
  m5_editor_inline_component_surface_certification::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the checked-in export drifts from
the seeded builder.
