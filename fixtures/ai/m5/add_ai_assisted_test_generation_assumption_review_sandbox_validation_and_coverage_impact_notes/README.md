# AI Test Generation, Assumption Review, Sandbox Validation, and Coverage-Impact Fixtures

This directory contains fixture files for the AI test-generation lane, which binds
read-only generated test proposals — each anchored to a location by a durable
anchor that survives edits and never auto-applied — together with an
assumption-review sheet that flags unvalidated assumptions, sandbox validation that
stays isolated and never counts as release coverage truth, and coverage-impact
notes that label estimates as estimates rather than measured.

## Files

- `valid_packet.json` — A fully valid generated-test-review packet that passes all
  validation invariants. Mirrors the checked-in support export.
- `sandbox_treated_as_release_truth.json` — A packet whose sandbox block sets
  `sandbox_is_not_release_truth` false, triggering `sandbox_treated_as_release_truth`.
