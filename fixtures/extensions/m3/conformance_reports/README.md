# Conformance, compatibility, and bundle review fixtures

Replayable fixtures for `aureline_extensions::conformance_reports`. Each file
carries a `__fixture__` block of expectations and an `input` block consumed by
`build_conformance_report` or `build_mirror_bundle_review`. The crate tests
(`crates/aureline-extensions/src/conformance_reports/tests.rs`) replay every
fixture, assert the expected decision, reason, and counts, validate the produced
record, render the Markdown, and round-trip the JSON.

The produced records conform to
[`/schemas/extensions/conformance_report.schema.json`](../../../../schemas/extensions/conformance_report.schema.json)
and
[`/schemas/extensions/mirror_bundle_review.schema.json`](../../../../schemas/extensions/mirror_bundle_review.schema.json).

## Conformance + compatibility reports

- `conformance_publish_ready.json` — all checks pass, no compatibility blockers.
- `conformance_recommendations_only.json` — no blockers, but a warning check and
  a non-blocking deprecation remain.
- `conformance_blockers_present.json` — a failed blocker check plus a removed
  deprecated API and a required ABI shim block publication.

## Mirror / offline bundle reviews

- `bundle_offline_ready.json` — sealed offline bundle, identity/signing/
  provenance/dependencies/reproducibility all preserved.
- `bundle_mirror_downgraded_ready.json` — approved mirror; identity and signing
  preserved but one trust claim downgraded and only partially reproducible.
- `bundle_signing_gap_refused.json` — manual side-load that is unsigned with
  missing provenance even though compatibility passes, so the bundle is refused.
