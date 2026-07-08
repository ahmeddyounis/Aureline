# M5 Structured-Artifact Review-Component Surface Certification fixtures

Golden fixtures for the closing certification capstone in
`crates/aureline-review/src/certify_structured_artifact_review_component_truth_on_every_claimed_m5_structured_artifact_review_surface`.

Each fixture is a full `StructuredArtifactCertificationPacket` that must pass
`validate()` and conform to
[`schemas/ui/m5-structured-artifact-review-component-certification.schema.json`](../../../schemas/ui/m5-structured-artifact-review-component-certification.schema.json).

- `render_trust_stale_auto_narrowed.json` — the canonical eight-surface packet after
  `apply_downgrade_automation` narrows the diff-toolbar surface because its render /
  parser trust went stale: the claim drops to a disclosed raw fallback, the
  `structured_fidelity_provenance` axis narrows, and the render-trust trigger is
  disclosed.
- `merge_sheet_and_cli_narrowed.json` — two surfaces (merge sheet and headless CLI)
  auto-narrowed together, proving the automation narrows each stale-fidelity surface
  independently while preserving component truth everywhere.

Regenerate with:

```
GEN_STRUCTURED_ARTIFACT_CERTIFICATION_ARTIFACTS=1 \
  cargo test -p aureline-review --lib regenerate_structured_artifact_certification_artifacts
```
