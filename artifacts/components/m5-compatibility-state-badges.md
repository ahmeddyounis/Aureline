# M5 Compatibility State Badge Primitive

- Packet: `m5-compatibility-state-badge-primitive:stable:0001`
- Label: `M5 compatibility-state badge primitive: exact-match/compatible/limited/mismatch parity as one distinct, composable cue with reconciliation, repair, and compare detail preserved before install/import/apply/reopen`
- Badge consumers: 6 (6 stable)
- State values: exact_match, compatible, limited, mismatch
- Compatibility postures: full_parity, compatible_within_range, reduced_capability, incompatible_as_claimed
- Gap classes: capability_subset_reduced, version_or_schema_mismatch
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Badge consumers

- **Workspace Reopen Card**: `stable`
  - Owner: Workspace reopen compatibility badge owner
  - State: The workspace reopen card renders the shared compatibility-state badge so an exact-match portable-state artifact reads as full parity and safe to reopen, and a mismatched artifact reads as incompatible-as-claimed with its version/schema gap, residual capability, and repair action preserved — proving the compatibility state is its own axis, presented before the reopen proceeds, and never collapses into support class, lifecycle, or channel
  - Worked resolutions: 2
    - state `exact_match` → posture `full_parity` (gap `no_reconciliation_gap`)
    - state `mismatch` → posture `incompatible_as_claimed` (gap `version_or_schema_mismatch`)
- **Toolchain Install Row**: `stable`
  - Owner: Toolchain install compatibility badge owner
  - State: The toolchain install row renders the shared compatibility-state badge so a compatible toolchain reads as compatible-within-range and installs without reconciliation, and a limited toolchain reads as reduced-capability with the exact reduced subset and compare/review action disclosed before install — the same compatibility vocabulary an install reviewer reads elsewhere
  - Worked resolutions: 2
    - state `compatible` → posture `compatible_within_range` (gap `no_reconciliation_gap`)
    - state `limited` → posture `reduced_capability` (gap `capability_subset_reduced`)
- **Extension Import Row**: `stable`
  - Owner: Extension import compatibility badge owner
  - State: The extension import row renders the shared compatibility-state badge so a limited extension reads as reduced-capability and continues with a reduced scope — disclosing exactly which capabilities are narrowed before import — and an exact-match extension reads as full parity, so a Limited reading is a reviewable narrowing rather than a silent exclusion
  - Worked resolutions: 2
    - state `limited` → posture `reduced_capability` (gap `capability_subset_reduced`)
    - state `exact_match` → posture `full_parity` (gap `no_reconciliation_gap`)
- **Workflow Bundle Apply Card**: `stable`
  - Owner: Workflow bundle apply compatibility badge owner
  - State: The workflow-bundle apply card renders the shared compatibility-state badge so a mismatched bundle reads as incompatible-as-claimed and is blocked-until-reconciled — preserving the version/schema gap and a repair-before-apply entrypoint — and a compatible bundle reads as compatible-within-range, so a risky apply is gated on an explicit posture instead of a generic warning
  - Worked resolutions: 2
    - state `mismatch` → posture `incompatible_as_claimed` (gap `version_or_schema_mismatch`)
    - state `compatible` → posture `compatible_within_range` (gap `no_reconciliation_gap`)
- **Compare / Review Panel**: `stable`
  - Owner: Compare review compatibility badge owner
  - State: The compare / review panel renders the shared compatibility-state badge so a limited artifact reads as reduced-capability with a compare-and-review action and a mismatched artifact reads as incompatible-as-claimed with a repair-before-apply action — the two non-parity readings stay distinct, detail-preserving cues a reviewer can compare directly rather than one collapsed warning
  - Worked resolutions: 2
    - state `limited` → posture `reduced_capability` (gap `capability_subset_reduced`)
    - state `mismatch` → posture `incompatible_as_claimed` (gap `version_or_schema_mismatch`)
- **Support Export Row**: `stable`
  - Owner: Support export compatibility badge owner
  - State: The support-export row renders the shared compatibility-state badge so a mismatched artifact carries its state, posture, gap class, reconciliation detail, and residual capability as separate fields into exported evidence — enough to repair, compare, and narrow the claim later — and an exact-match artifact reads as full parity, so exported evidence never loses the state's meaning
  - Worked resolutions: 2
    - state `mismatch` → posture `incompatible_as_claimed` (gap `version_or_schema_mismatch`)
    - state `exact_match` → posture `full_parity` (gap `no_reconciliation_gap`)
