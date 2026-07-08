# M5 Structured-Artifact Review-Component Surface Certification

Closing certification capstone for batch **B114**. It certifies that the nine
shared M5 structured-artifact review components — artifact-identity-bar,
diff-mode-switcher, structure-row, merge-decision-row, generated-artifact-notice,
rendered-compare-viewer, media-metadata-rail, redaction-or-trust-badge-set, and
compare-summary-card — present the **same controlled component truth on every
claimed M5 diff, merge, and compare surface**, with no hidden parser/schema,
render-trust, write-back, or metadata drift.

- Module: `crates/aureline-review/src/certify_structured_artifact_review_component_truth_on_every_claimed_m5_structured_artifact_review_surface`
- Boundary schema: [`schemas/ui/m5-structured-artifact-review-component-certification.schema.json`](../../../schemas/ui/m5-structured-artifact-review-component-certification.schema.json)
- Checked export: `artifacts/review/m5/…/support_export.json`
- Release proof: `artifacts/release/m5-structured-artifact-review-certification-proof/`
- Fixtures: `fixtures/ui/m5-structured-artifact-review-component-certification/`

## What it builds on

| Lane | Contract |
| --- | --- |
| Component matrix (M05-964) | `schemas/ui/m5-structured-artifact-review-component-matrix.schema.json` |
| Shared consumers (M05-969) | `schemas/ui/m5-structured-artifact-review-component-consumer.schema.json` |
| A11y / headless / export parity (M05-970) | `schemas/ui/m5-structured-artifact-review-component-accessibility-parity.schema.json` |
| artifact-identity-bar / diff-mode-switcher | `schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json` |
| structure-row / compare-summary-card | `schemas/ui/m5-structure-compare-summary-controls.schema.json` |
| merge-decision-row / generated-artifact-notice | `schemas/ui/m5-merge-decision-generated-notice-controls.schema.json` |
| rendered-compare-viewer / media-metadata-rail / redaction-or-trust-badge-set | `schemas/ui/m5-rendered-compare-media-trust-controls.schema.json` |

It reuses `M5ArtifactComponent` (the nine frozen components) and
`ArtifactReviewClaimTier` (the five-tier full-structured-fidelity →
metadata-withheld claim ladder) directly; it does not re-mint them.

## Certified surfaces

Eight claimed M5 structured-artifact review surfaces are certified:
`diff_toolbar_surface`, `merge_sheet_surface`, `review_workspace_surface`,
`help_artifact_surface`, `support_export`, `exported_artifact_packet`,
`cli_headless`, and `diagnostics`.

## Certification axes

Each surface row scores six axes:

- **visual**, **keyboard**, **screen_reader**, **cli_export** — always-on parity
  axes every claimed component must pass on every surface.
- **degraded_state** — narrows the claim honestly when parser/schema state, render
  trust, write-back safety, or metadata availability weakens.
- **structured_fidelity_provenance** — the certification-specific separation axis.
  It keeps the structured-vs-raw and render-trust distinctions explicit so a
  **certified surface never implies its structured fidelity is full, its render is
  trusted, or its write-back is safe**.

## Status ladder

`derive_structured_artifact_surface_claim_status` scores each surface:

- **`certified_parity`** (green): certified claim equals the claimed claim, no axis
  narrows, and component truth is preserved.
- **`narrowed_parity`** (yellow): a claim narrowed or an axis narrowed, but the
  component's meaning is preserved and the narrowing is disclosed with a trigger.
- **`parity_blocked`** (red): the component's artifact class, canonical source, diff
  mode, parser/schema state, compare-only / write-back safety, render trust,
  generated-from relation, metadata visibility, or redaction posture was flattened
  out of the surface. This is the delta of the capstone — certification may narrow a
  claim, but it may never drop the component's meaning.

## Acceptance criteria

- **AC1 — no hidden fidelity drift.** Every claimed surface presents the same
  controlled component truth. Enforced by the trust-review invariants, the
  per-surface axis coverage, the `raw_artifact_material_in_export` guard, and the
  `all_surfaces_covered` / `all_components_covered` summary flags.
- **AC2 — parity, not just workflow maturity.** The certified claim may never exceed
  the claimed one (`certified_claim_exceeds_claimed`), status must match the derived
  status (`status_mismatch`), and `apply_downgrade_automation` narrows a surface the
  moment its structured fidelity (parser/schema and render trust) goes stale —
  proving the release evidence tracks component parity, not earlier workflow-level
  artifact rows. The `certified_never_implies_full_fidelity` theme is proven by the
  orthogonal `structured_fidelity_provenance` axis.

## Regenerating artifacts

```
GEN_STRUCTURED_ARTIFACT_CERTIFICATION_ARTIFACTS=1 \
  cargo test -p aureline-review --lib regenerate_structured_artifact_certification_artifacts
```

Then rebuild and run the module suite:

```
cargo test -p aureline-review --lib certify_structured_artifact_review_component_truth
```
