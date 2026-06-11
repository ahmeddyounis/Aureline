# Freeze the M5 dependency-intelligence, package-health, and code-quality parity matrix

This document is the human-readable companion to the canonical M5 dependency/package and code-quality parity matrix checked in at `artifacts/release/m5/freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix.json`.

## Purpose

The parity matrix freezes the canonical control surface for every marketed M5 dependency-intelligence, package-health, and code-quality lane. It binds each lane to an explicit claim class, deployment-profile posture, and compatibility/downgrade rule, plus a scorecard, proof packet, and owner, so dependency/package and quality/scanner depth cannot widen through implicit inheritance from the M4 stable line. A lane that loses fresh proof, a compatibility report, an admin/policy story, a rollback path, owner sign-off, or whose upstream hard dependency narrows must drop below the Stable cutline rather than inherit an adjacent green lane.

## Structure

The matrix contains:

- **Lane rows** — one per dependency/quality lane (`dependency_intelligence`, `package_mutation`, `package_health`, `quality_profile`, `scanner_import`, `live_quality`, `cli_headless_parity`).
- **Claim class** — the explicit class a lane is marketed under (`marketed_depth`, `foundation_parity`, `imported_visibility`, `advisory_only`).
- **Deployment-profile posture** — `local_only`, `mirrored`, `managed`, or `browser_handoff`, so registry/mirror side effects and managed gates are inspectable.
- **Compatibility/downgrade rule** — a per-row ref naming exactly how the lane narrows when its evidence lapses.
- **Scorecards** — per-lane proof packet, compatibility report, admin/policy story, and rollback path refs.
- **Dependency graph** — hard and soft edges between lanes. A hard dependency means the downstream lane must narrow if the upstream lane narrows.
- **Stop rules** — closed conditions that gate promotion. Every gap reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from firing stop rules so stale, unqualified, or policy-blocked rows narrow automatically instead of depending on manual doc edits.

## Imported-versus-live truth

The `scanner_import` lane carries the `imported_visibility` claim class so SARIF/scanner imported findings stay distinct from `live_quality` findings produced against the working tree. Imported visibility never inherits the live lane's authority.

## Consumption

Downstream docs, Help/About, CLI inspection, release/public-truth automation, and support export surfaces should ingest `support_export_projection()` from the typed model (`aureline_release::freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix`) rather than cloning status text. The export projection carries the claim class and deployment-profile posture for each lane.

## Regeneration

`tools/regenerate_freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix.py` is the single source of truth for the matrix, its CI validation capture, and the negative fixtures. It derives the summary counts and promotion verdict exactly as the typed Rust consumer does. After editing the row set, run the regenerator and then `cargo test -p aureline-release --test freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix`.

## Freshness

The matrix is current as of the `as_of` date embedded in the JSON artifact. The CI gate recomputes the promotion verdict and summary against the rows and stop rules and fails if the matrix is stale, underqualified, or policy-blocked.
