# M5 Learning Component Surface Certification (M05-1011)

This is the **closing surface-certification capstone** for the B119 learning-component lane. Where
the frozen matrix (`schemas/ui/m5-learning-component-matrix.schema.json`) defines the six reusable
**learning-mode-toggle**, **tip-card**, **guided-exercise-step**, **glossary-chip-or-card**,
**safe-explanation-banner**, and **progress-marker** components, the M05-1005..1007 primitive lanes
narrow each one, the M05-1009 consumer lane proves they are reusable across the claimed onboarding /
migration / contextual-help / docs-browser / tour / companion-handoff / support-export consumers,
and the M05-1010 accessibility / auto-narrowing capstone certifies keyboard / screen-reader /
localization / export parity per family, this capstone **certifies that the shared
learning-component truth holds on every claimed M5 learnability surface** — and auto-narrows any
surface that cannot sustain it.

- **Module:**
  `crates/aureline-learning/src/certify_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_truth_on_every_claimed_m5_learnability_surface/`
- **Boundary schema:** `schemas/ui/m5-learning-component-certification.schema.json`
- **Release proof:** `artifacts/release/m5-learning-component-certification/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-learning-component-certification/`
- **Canonical bundle every row cites:** `artifacts/release/m5-learning-component-proof/support_export.json`
  (the frozen learning-component matrix release proof — the canonical M5 evidence index entry for
  this lane)

## What is certified

The packet is keyed on the **surface** a user learns on — not on component family or primitive lane.
Eight claimed surfaces are certified exactly once:

| Surface | Meaning |
| --- | --- |
| `first_run_onboarding` | The first-run onboarding flow. |
| `feature_family_tour` | The feature-family tour. |
| `docs_glossary_browser` | The docs / glossary browser. |
| `support_export` | The support / export bundle. |
| `guided_exercise_practice` | The guided-exercise practice surface. |
| `contextual_help` | The contextual-help surface. |
| `educational_ai_companion` | The educational-AI companion surface. |
| `cli_headless` | The CLI / headless surface. |

Each surface is scored on **six truth axes**: `visual`, `keyboard`, `screen_reader`, `export`
(always-on), `degraded_state`, and `learning_boundary_provenance`. Every one of the six frozen
component families is certified on at least one surface.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps an
`exact_learning` / `reviewable_guidance` claim while one of its truth axes is not current — the
glossary citation is stale, the exercise pack has drifted, the explain-versus-do boundary cannot be
proven, or progress portability is blocked — is over-claiming and **blocks (red)**. A surface that
discloses the reduction by narrowing its learning claim (with a bound reason and a frozen downgrade
trigger) is honestly **yellow**. A surface with full parity delivers its claim (**green**).

The learning-claim ladder, strongest first: `exact_learning` (7) > `reviewable_guidance` (6) >
`paused_mode_projection` (5) > `snoozed_tip_projection` (4) > `stale_pack_projection` (3) >
`uncited_glossary_projection` (2) > `unprovable_boundary_projection` (1) >
`blocked_progress_projection` (0). Certification may only **narrow** a claim, never strengthen it.

### Learning-lineage preservation

Learning truth never loses lineage: a narrowed surface always preserves its **cited-source /
command-binding / progress-ownership / explain-versus-do** lineage continuity rather than dropping
it between a tip, a glossary chip, and an exported progress record. Dropping lineage blocks the
surface (`LineageDropped`).

### No widened authority

No certified surface may widen trust or mutating authority: learnability stays opt-in,
citation-backed, command-backed, and privacy-bounded, and **explain stays separate from do**. A
surface that widens authority blocks (`LearningAuthorityWidened`).

### Always-on export parity

The `export` axis must always stay certified, so support and automation can reconstruct the same
learning-mode / tip / exercise / citation / explanation / progress truth from the same component
identity the user saw. Export must offer text / JSON / Markdown reconstruction and prohibit a
screenshot-only export.

## The four auto-narrow conditions

The seed packet certifies four green surfaces (full parity, claim delivered) and four yellow
surfaces — one for each spec auto-narrow condition (missing cited glossary/source truth, drifted
exercise pack, unprovable explain-versus-do boundary, or blocked progress privacy/portability):

| Surface | Claimed → Certified | Binding axis | Trigger |
| --- | --- | --- | --- |
| `guided_exercise_practice` | `exact_learning` → `stale_pack_projection` | `degraded_state` | `exercise_step_state_unstated` |
| `contextual_help` | `exact_learning` → `uncited_glossary_projection` | `learning_boundary_provenance` | `glossary_citation_severed` |
| `educational_ai_companion` | `reviewable_guidance` → `unprovable_boundary_projection` | `learning_boundary_provenance` | `explanation_apply_boundary_unstated` |
| `cli_headless` | `exact_learning` → `blocked_progress_projection` | `learning_boundary_provenance` | `progress_ownership_unstated` |

No surface hides drift (red), no surface widens authority, and no surface drops lineage.

## Metadata-only boundary

The packet is metadata-only: typed class tokens, opaque refs, booleans, and redacted labels. Raw
learning copy, captured glossary bodies, exercise payloads, imported progress state, and
credential-bearing material never cross this boundary (`RawLearningMaterialInExport`).

## Regenerating the artifacts

The checked-in export is byte-aligned with the in-code seed builder
(`seeded_m5_learning_component_certification_packet`). A drift test fails if they diverge. To
regenerate after an intentional change:

```
GEN_LEARNING_CERT_ARTIFACTS=1 cargo test -p aureline-learning --lib \
  -- certify_learning_mode_toggle generate_artifacts
```

Then re-run the suite:

```
cargo test -p aureline-learning --lib -- certify_learning_mode_toggle
```
