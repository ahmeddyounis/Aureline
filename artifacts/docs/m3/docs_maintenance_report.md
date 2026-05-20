# Docs preview / maintenance integrity conformance evidence

This artifact is the release-consumable conformance evidence for the M3 docs
preview / maintenance beta lane. Every claimed beta docs-authoring / help row
reads exactly one governed docs-maintenance record assembled by
`aureline-docs::maintenance` — a `DocsPreviewHeader`, `DocsSuggestionCard`,
`DocsExampleFindingRow`, `DocsMaintenanceRow`, the holding
`DocsMaintenanceContract`, or the exported `DocsMaintenanceReviewPacket`. Every
record is exercised by at least one drill in
[`fixtures/docs/m3/docs_maintenance_corpus/`](../../../fixtures/docs/m3/docs_maintenance_corpus/);
the drills are executed by
[`crates/aureline-qe/src/docs_maintenance/`](../../../crates/aureline-qe/src/docs_maintenance/)
and replayed by
`cargo test -p aureline-qe --test docs_maintenance_conformance`.

The corpus is owned by the QE crate so the same fixture matrix can gate the
desktop preview/maintenance surfaces, the CLI / headless docs-maintenance
output, support-export parity reviews, and release evidence reviews from one
shared truth. The runner reuses the canonical validation owned by
`aureline-docs::maintenance`; it never re-implements the ruleset, so the lane
cannot drift into a separate ad hoc rule set.

The exit-gate condition the corpus guards is the M3 docs-maintenance anchor:

> Claimed beta docs-authoring/help rows have current proof that preview/
> rendering is safe and labeled, suggestions remain evidence-backed diffs,
> stale-example and broken-link findings stay attributable, and README/
> changelog maintenance preserves branch/channel/audience truth across
> desktop, CLI, and exported review packets.

## Result

`cargo test -p aureline-qe --test docs_maintenance_conformance` — **7 tests,
all passing** (1 corpus replay + 6 transverse invariants). The corpus
publishes **13 positive** drills and **8 negative** drills. The in-library
replay `aureline_qe::docs_maintenance::run_corpus_from_repo_root` returns a
`CorpusReport` with no `failures()`.

## Coverage matrix

| Axis | Drill id | Outcome anchored |
| --- | --- | --- |
| CommonMark-only preview | `preview.commonmark_only` | `source` mode, CommonMark baseline, no extensions, sanitization `not_applicable`, local-only. |
| Extension-enabled preview | `preview.extension_enabled` | `split` mode, declared extensions only, `sanitized_safe`, canonical-not-proof disclosure. |
| Blocked-content preview | `preview.blocked_content` | `rendered` mode, `raw_html_blocked`, canonical-not-proof disclosure. |
| Release-note-drift suggestion | `suggestion.release_note_drift` | `release_note_drift` trigger, `apply_after_review`, evidence-backed, review diff present. |
| Failing code example | `suggestion.failing_snippet_blocked` | `failing_snippet` trigger, `blocked_pending_evidence`, no review diff, local-only. |
| Broken-link finding | `finding.broken_link` | `broken_link`, `proven_broken`, `rendered` validation, active. |
| Stale-example finding | `finding.stale_example_suppressed` | `stale_example`, `suspected_stale`, `stale` validation, `suppressed_until_reviewed` with actor/reason/expiry/evidence attribution. |
| Version-mismatch finding | `finding.version_mismatch` | `version_mismatch`, `unchanged_unverified`, `skipped` validation, active. |
| Changelog scoped publish | `maintenance.changelog_scoped_publish` | `changelog`, `publish_handoff_scoped`, `release_manager`, branch/release/channel scope kept (beta). |
| README local-only | `maintenance.readme_local_only` | `readme`, `local_only`, `public_reader`, no scope required. |
| Onboarding review handoff | `maintenance.onboarding_review_handoff` | `onboarding_note`, `review_handoff_scoped`, `end_user`, branch/channel scope kept (beta). |
| Full contract | `contract.seeded_full` | All preview modes, all validation modes, all triggers, all boundary states, cross-refs resolve. |
| Export-safe review packet | `export.review_packet_export_safe` | Screenshot-free, no raw bodies, reconstructs from the seeded contract. |
| Negative — hidden renderer extension | `negative.preview_hidden_extension` | Validation rejects with `preview_header.hidden_extension` (active extension not declared). |
| Negative — raw HTML, no disclosure | `negative.preview_raw_html_no_disclosure` | Validation rejects with `preview_header.sanitization_note`. |
| Negative — silent suggestion | `negative.suggestion_silent_rewrite` | Validation rejects with `suggestion_card.silent_rewrite`. |
| Negative — unscoped changelog publish | `negative.maintenance_unscoped_publish` | Validation rejects with `maintenance_row.publish_scope`. |
| Negative — blocked row leaks action | `negative.maintenance_blocked_with_action` | Validation rejects with `maintenance_row.blocked_action`. |
| Negative — dropped stale-example attribution | `negative.finding_stale_example_missing_attribution` | Validation rejects with `finding_row.suppression_attribution`. |
| Negative — proven-broken, not validated | `negative.finding_proven_broken_not_validated` | Validation rejects with `finding_row.proven_broken_validation`. |
| Negative — wrong-channel export | `negative.review_packet_wrong_channel_drift` | Packet flattens the changelog row from `beta` to `stable`; rejected as `review_packet.row_drift` against the seeded contract. |

## Exit-gate mapping

| Acceptance criterion | Evidence |
| --- | --- |
| The corpus can prove which preview mode, validation mode, branch/channel target, and suggestion trigger produced each finding. | Each drill pins the exact tokens in `manifest.json`; the runner asserts them against the parsed record. |
| Hidden renderer extensions, silent suggestion application, or unscoped README/changelog updates fail the lane. | `negative.preview_hidden_extension`, `negative.suggestion_silent_rewrite`, `negative.maintenance_unscoped_publish`. |
| Stale-example and broken-link findings stay specific, attributable, and export-safe. | `finding.broken_link`, `finding.stale_example_suppressed` (attribution), `negative.finding_stale_example_missing_attribution`; positive payloads scanned for raw URL / raw-body leaks. |
| Desktop, CLI, and exported packets agree on docs-maintenance truth. | `desktop_cli_and_exported_packets_agree_on_truth` asserts the contract, surface projection, and review packet carry identical rows; `negative.review_packet_wrong_channel_drift` proves divergence is caught. |

## Replay

```sh
cargo test -p aureline-qe --test docs_maintenance_conformance
cargo test -p aureline-docs --test docs_preview_and_maintenance_beta
```
