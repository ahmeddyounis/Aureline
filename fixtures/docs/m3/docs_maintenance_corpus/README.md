# Docs preview / maintenance integrity corpus

This corpus is the regression-gated proof lane for the M3 docs-maintenance beta
surfaces. Every drill reuses the canonical docs-maintenance records and their
validation owned by `aureline-docs::maintenance` — `DocsPreviewHeader`,
`DocsSuggestionCard`, `DocsExampleFindingRow`, `DocsMaintenanceRow`,
`DocsMaintenanceContract`, and the exported `DocsMaintenanceReviewPacket`. The
runner in [`crates/aureline-qe/src/docs_maintenance/`](../../../../crates/aureline-qe/src/docs_maintenance)
parses each fixture into the real record type, runs the canonical ruleset, and
compares the result against the per-drill truth pinned in `manifest.json`. The
ruleset is never re-implemented in the corpus.

## What it proves

The corpus guards the M3 docs-maintenance exit-gate:

> Claimed beta docs-authoring/help rows have current proof that
> preview/rendering is safe and labeled, suggestions remain evidence-backed
> diffs, stale-example and broken-link findings stay attributable, and
> README/changelog maintenance preserves branch/channel/audience truth across
> desktop, CLI, and exported review packets.

- **Positive drills** MUST validate cleanly (zero findings), match every pinned
  `expected_*` token (preview mode, sanitization posture, CommonMark baseline,
  suggestion trigger / apply posture, finding class / detection / validation
  mode, suppression attribution, and branch / release / channel / audience
  scope), and carry no raw URL or raw-body export.
- **Negative drills** MUST FAIL validation with a finding whose `check_id`
  contains the recorded `expected_violation_check_id`. Review-packet drift is
  checked against the seeded contract, so a wrong-channel export that tries to
  pass beta notes off as stable docs is caught as drift.

## Coverage

Positive drills cover CommonMark-only / extension-enabled / blocked-content
previews; release-note-drift and failing-snippet suggestions; broken-link,
stale-example (suppressed with attribution), and version-mismatch findings;
local-only / review-handoff / publish-handoff maintenance rows across
public-reader, end-user, and release-manager audiences; the full seeded
contract; and an export-safe review packet.

Negative drills pin the conditions the lane MUST catch: a hidden (undeclared)
renderer extension, raw HTML allowed without disclosure, a silently-applied
suggestion, an unscoped changelog publish, a blocked-unscoped row that leaks an
apply action, a suppressed stale-example finding that dropped its attribution, a
proven-broken finding marked not-validated, and a wrong-channel review-packet
export.

## Regenerate

The whole-contract and review-packet fixtures are generated from the seeded
records so they never drift:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- manifest \
  > fixtures/docs/m3/docs_maintenance_corpus/positive/contract_seeded_full.json
cargo run -q -p aureline-docs --bin aureline_docs_preview_and_maintenance_beta -- review-packet \
  > fixtures/docs/m3/docs_maintenance_corpus/positive/review_packet_export_safe.json
```

The remaining per-record fixtures are hand-authored (the negatives are
intentionally invalid).

## Validate

```sh
cargo test -p aureline-qe --test docs_maintenance_conformance
```
