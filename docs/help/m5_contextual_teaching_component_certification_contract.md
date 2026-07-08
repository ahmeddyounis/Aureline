# M5 Contextual-Teaching Component Surface Certification (M05-931)

This is the **closing surface-certification capstone** for the B109 contextual-teaching
component lane. Where the frozen matrix
(`schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json`) defines the
five reusable **contextual-tip-card**, **migration-bridge-card**, **sequence-help-strip**,
**why-unavailable-explanation-row**, and **source-language-fallback** components, the M05-925..928
primitive lanes narrow each one, the M05-929 consumer lane proves they are reusable across the
claimed onboarding / importer / keybinding / command-doc / help / localized-support consumers, and
the M05-930 accessibility / auto-narrowing capstone certifies keyboard / screen-reader / CLI /
export parity per family, this capstone **certifies that the shared contextual-teaching component
truth holds on every claimed M5 onboarding, migration, and command-help surface** — and
auto-narrows any surface that cannot sustain it.

- **Module:**
  `crates/aureline-learning/src/certify_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_truth_on_every_claimed_m5_onboarding_migration_and_command_help_surface/`
- **Boundary schema:** `schemas/ui/m5-contextual-teaching-component-certification.schema.json`
- **Release proof:** `artifacts/release/m5-contextual-teaching-component-certification/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-contextual-teaching-component-certification/`

## What is certified

The packet is keyed on the **surface** a user learns, switches, or hits a blocked state on — not
on component family or primitive lane. Eight claimed surfaces are certified exactly once:

| Surface | Meaning |
| --- | --- |
| `first_run_onboarding` | The first-run onboarding flow. |
| `migration_importer_review` | The migration importer review. |
| `command_palette_docs` | The command palette / command-docs surface. |
| `keybinding_help` | The keybinding / leader-overlay help surface. |
| `modal_sequence_overlay` | The modal command-sequence overlay. |
| `localized_support` | The localized support / help packet. |
| `support_export` | The support / export bundle. |
| `cli_headless` | The CLI / headless surface. |

Each surface is scored on **six truth axes**: `visual`, `keyboard`, `screen_reader`, `cli_export`
(always-on), `degraded_state`, and `teaching_boundary_provenance`. Every one of the five frozen
component families is certified on at least one surface.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps an
`exact_teaching` / `reviewable_guidance` claim while one of its truth axes is not current — the
contextual tip is snoozed, the migration bridge is only partial, the command sequence is
unsupported in the current context, the localized fallback content is stale, or the
command-binding / migration-mapping / blocked-action / source-language boundary is unstated — is
over-claiming and **blocks (red)**. A surface that discloses the reduction by narrowing its
teaching claim (with a bound reason and a frozen downgrade trigger) is honestly **yellow**. A
surface with full parity delivers its claim (**green**).

The teaching-claim ladder, strongest first: `exact_teaching` (5) > `reviewable_guidance` (4) >
`snoozed_tip_projection` (3) > `partial_bridge_projection` (2) > `unsupported_sequence_projection`
(1) > `stale_fallback_projection` (0). Certification may only **narrow** a claim, never strengthen
it.

### Teaching-lineage preservation

Teaching truth never loses lineage: a narrowed surface always preserves its **command-binding /
migration-mapping / blocked-action / source-language** lineage continuity rather than dropping it
between an in-place tip, a migration bridge, and a localized help fallback. Dropping lineage
blocks the surface (`LineageDropped`).

### Always-on CLI / export parity

The `cli_export` axis must always stay certified, so support and automation can reconstruct the
same tip-trigger / command-binding / migration-mapping / sequence-state / blocked-action /
source-language-citation truth from the same command identity the user saw. Export must offer
text / JSON / Markdown reconstruction and prohibit a screenshot-only export.

## The four auto-narrow conditions

The seed packet certifies four green surfaces (full parity, claim delivered) and four yellow
surfaces — one for each spec auto-narrow condition:

| Surface | Claimed → Certified | Binding axis | Trigger |
| --- | --- | --- | --- |
| `migration_importer_review` | `exact_teaching` → `partial_bridge_projection` | `teaching_boundary_provenance` | `migration_mapping_unstated` |
| `modal_sequence_overlay` | `exact_teaching` → `unsupported_sequence_projection` | `teaching_boundary_provenance` | `sequence_help_state_unstated` |
| `localized_support` | `exact_teaching` → `stale_fallback_projection` | `teaching_boundary_provenance` | `source_language_fallback_unstated` |
| `cli_headless` | `exact_teaching` → `snoozed_tip_projection` | `degraded_state` | `tip_command_binding_unstated` |

No surface hides drift (red), and no surface drops lineage.

## Metadata-only boundary

The packet is metadata-only: typed class tokens, opaque refs, booleans, and redacted labels. Raw
teaching copy, captured source-language bodies, imported migration payloads, and credential-bearing
material never cross this boundary (`RawTeachingMaterialInExport`).

## Regenerating the artifacts

The checked-in export is byte-aligned with the in-code seed builder
(`seeded_m5_contextual_teaching_component_certification_packet`). A drift test fails if they
diverge. To regenerate after an intentional change:

```
GEN_TEACHING_CERT_ARTIFACTS=1 cargo test -p aureline-learning --lib \
  -- certify_contextual_tip_card generate_artifacts
```

Then re-run the suite:

```
cargo test -p aureline-learning --lib -- certify_contextual_tip_card
```
