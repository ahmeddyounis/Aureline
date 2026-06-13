# M5 Content-Integrity Certification Fixtures

These fixtures are valid, export-safe certification packets that exercise the
auto-narrow behavior the canonical support export keeps green. Each one keeps
every artifact family present, the review and consumer-projection invariants
satisfied, and proof freshness valid — the difference is which family is narrowed
and why. Each fixture's `certified_qualification`, `narrowing_reasons`, and
`summary` counts are consistent with its own per-dimension proofs, so validation
recomputes them and confirms the row narrowed instead of silently keeping its
claim.

## notebook_safe_preview_proof_missing.json

The notebook rich-output family loses its safe-preview trust-class proof
(`safe_preview_trust_class` is `missing`). A missing required proof narrows the
family to `experimental`, demonstrating that a claimed family auto-narrows when a
required content-integrity proof is absent rather than publishing its prior claim.

## marketplace_strict_identity_stale.json

The marketplace install/update family's strong-decision strict-identity proof is
stale (`strong_decision_display` is `stale_pass`). A stale required proof narrows
the family one rung from `stable` to `beta`, demonstrating proof-freshness
enforcement on a strong-decision surface.
