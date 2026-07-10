# M5 Framework-Component Accessibility & Auto-Narrowing (M05-1042)

This lane is the accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone over the frozen M5 framework-component matrix
(`freeze_the_m5_framework_pack_header_...`). Where the freeze matrix defines the reusable framework
pack header, route / endpoint row, component / service tree node, convention-diagnostic row,
generator preview sheet, run-config scaffold card, and derived-relationship banner primitives — and
the 1037–1041 implementation / consumer lanes resolve their per-surface truth — this lane certifies,
per component family, that framework-aware claims stay **keyboard-complete,
assistive-tech-reachable, CLI/export-safe, and self-narrowing**.

## What it guarantees

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and CLI/headless-reachable path into the same pack identity, support
  class, exact-versus-heuristic-versus-runtime-confirmed certainty, proving-source linkage,
  local-versus-remote execution boundary, file / dependency / config impact, and rollback or
  regenerate recovery boundary the rich component shows — never a hover-only chip. The
  hierarchy-heavy component / service tree node (nested component / service topology) additionally
  binds its tree to a flat list / textual path.
- **Export parity.** The support / release / CLI export reconstructs each component's meaning from
  typed tokens and opaque refs **without a raw value** — never a raw credential or raw generated
  file payload — preserving the stable component identity, pack / certainty posture, execution
  boundary, impact disclosure, proving-source linkage, recovery boundary, and narrowing reasons — so
  support, docs, and release proof can reconstruct exactly what the user was actually shown.
- **Honest auto-narrowing.** When a framework pack's health cannot be proven, a supported version
  range cannot be proven for the active project, a proving-source linkage is missing, a relationship
  is only heuristically inferred, or a generator-effect truth is only partial, the component's
  exactness claim auto-narrows from `exact_framework_truth` to an `unverified_pack_projection` /
  `unproven_version_range_projection` / `unlinked_source_projection` / `heuristic_inference_projection`
  / `partial_generator_effect_projection`, discloses the narrowing with a precise trigger and binding
  dimension, and preserves the canonical pack / certainty source, proving-source linkage, execution
  boundary, and rollback / regenerate recovery boundary. An unproven-version-range / unlinked-source
  / heuristic-inference / partial-generator-effect state can never keep an exact framework claim —
  incomplete evidence never invents exact certainty, a heuristic route never reads as exact, and a
  generator never implies a safe or no-op write.
- **Cross-surface disclosure.** The same narrowed state surfaces in the framework-pack, route /
  topology explorer, diagnostic-center, generator-review, run-config, editor-gutter, CLI, and
  support-export surfaces so product, docs, and release publication stay aligned on downgrade
  behavior.

## Model

- **Exactness claim tiers** (strongest first): `exact_framework_truth`,
  `unproven_version_range_projection`, `unverified_pack_projection`, `unlinked_source_projection`,
  `heuristic_inference_projection`, `partial_generator_effect_projection`.
- **Claim dimensions** (the five spec axes): `pack_health_integrity`, `supported_version_range`,
  `proving_source_linkage`, `heuristic_inference_boundary`, `generator_effect_evidence`. The seven
  frozen families fold onto these five axes; the framework pack header additionally carries its
  supported-version-range dimension as a secondary condition.
- **Condition states**: `framework_verified_exact` (baseline) plus the operational / support state
  `pack_health_unproven`, and the four "cannot-be-proven" incomplete-evidence narrowing axes
  `version_range_unproven`, `source_linkage_unproven`, `heuristic_inference_only`, and
  `generator_effect_partial`.

Each condition state maps 1:1 to a permitted claim ceiling and names an on-topic frozen downgrade
trigger (`support_class_unstated`, `pack_identity_unstated`, `proving_source_omitted`,
`exact_versus_heuristic_unstated`, `impact_undisclosed`) so certified reasons stay byte-identical to
the freeze matrix. Only the four cannot-be-proven incomplete-evidence states can never keep an exact
framework claim; an unproven pack health is an honest support / operational disclosure, not an
exactness overstatement.

## Artifacts

- Schema: `schemas/ui/m5-framework-component-accessibility-fallback.schema.json`
- Support export (canonical): `artifacts/release/m5-framework-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-framework-component-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-framework-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-framework-component-accessibility-fallback/`

Regenerate the checked-in artifacts with:

```
GEN_FRAMEWORK_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-templates generate_artifacts
```
