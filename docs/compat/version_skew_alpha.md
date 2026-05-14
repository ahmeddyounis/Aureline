# Version-Skew And Drift Truth Alpha

This alpha slice makes compatibility drift visible before a helper, provider,
or saved-artifact surface can imply live authority.

Companion artifacts:

- `crates/aureline-shell/src/drift_truth/mod.rs` defines the typed shell
  projection.
- `fixtures/compat/version_skew_alpha/manifest.yaml` protects the four
  acceptance states.
- `docs/remote/helper_negotiation_alpha.md` remains the helper capability
  source for unsupported and retry-required agent states.
- `artifacts/compat/qualification_matrix_seed.yaml`,
  `artifacts/compat/version_skew_register.yaml`, and
  `artifacts/compat/skew_windows.yaml` remain the compatibility sources.

## Contract

Each drift row quotes the same compatibility refs:

- boundary family
- compatibility row
- version-skew register
- concrete skew case
- skew-window declaration

Rows do not restate the compatibility contract locally. The shell projection
adds only the visible state, mutation posture, next action, blocked action refs,
preserved artifact refs, and export refs needed by product surfaces.

## States

| State | Meaning | Mutation posture |
|---|---|---|
| `unsupported_skew` | Producer and consumer are outside the declared support window or required vocabulary is missing. | Block remote or provider mutation. |
| `retry_required` | A probe, refresh, or reattach must run before authority can widen. | Inspect-only until retry completes. |
| `stale_snapshot` | The row is reading cached provider or runtime data after the freshness floor crossed. | Read-only and labeled stale. |
| `migration_review_needed` | Saved state or an artifact could not safely migrate without review. | Review-only with preserved compare/export refs. |

## Consumers

The first shell consumer is `SupportSeedSurface::drift_truth_preview`. It turns a
`DriftTruthSnapshot` into a support-bundle preview item whose manifest row
carries metadata only. The same snapshot also emits a review packet via
`DriftTruthSnapshot::export_packet(DriftTruthExportAudience::Review)`.

Both packets set `raw_payloads_excluded = true`. They preserve row ids,
compatibility refs, state classes, blocked actions, repair actions, source refs,
and preserved artifact refs, but they do not carry raw provider payloads, target
paths, logs, secrets, or artifact bodies.

## Fixture Coverage

The fixture snapshot protects these rows:

- helper unsupported skew from the helper capability envelope
- helper retry-required drift probe
- provider cached read-only stale snapshot
- saved support recovery artifact needing migration review

The test path validates row invariants, required state coverage, required
surface coverage, display labels, support export safety, review export safety,
and support-preview integration.
