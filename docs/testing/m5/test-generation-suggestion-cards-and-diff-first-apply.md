# M5 test-generation suggestion cards and diff-first apply

This document is the contract for the **AI-assisted test-generation suggestion
cards** the M5 test-intelligence lane uses to take test generation from a
feature-level prototype into governed, evidence-bound truth. Where the coverage /
snapshot-review contract governs whether the *evidence* drawn over the editor is
trustworthy, this contract governs whether an *AI-assisted test proposal* is
trustworthy enough to flow through the same preview / diff / apply / revert pipeline
the rest of the lane uses.

A generated test is not a free-text suggestion and an apply is not a blind write.
A suggestion card names its target symbols and files, the uncovered-path or bug
source that motivated it, its assumptions, the files it would write, and a
sandbox-validation posture. It binds to reopenable discovery / session / coverage
evidence objects rather than free-text justification. It cannot be applied without an
isolated sandbox pass, cannot bypass preview-first, cannot silently widen beyond its
evidenced scope, and an imported proposal is held read-only instead of reading as a
local apply.

## Source of truth

- Packet type: `TestGenerationProposalPacket`
  (`crates/aureline-runtime/src/test_generation_suggestion_cards_and_diff_first_apply/`).
- Boundary schema:
  `schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json`.
- Checked support export:
  `artifacts/testing/m5/test-generation-suggestion-cards-and-diff-first-apply/support_export.json`.
- Markdown summary:
  `artifacts/testing/m5/test-generation-suggestion-cards-and-diff-first-apply.md`.
- Protected fixtures:
  `fixtures/testing/m5/test-generation-suggestion-cards-and-diff-first-apply/`.

Regenerate the canonical export, summary, and fixture after any shape change:

```bash
cargo run -p aureline-runtime --example dump_test_generation_suggestion_cards
cargo run -p aureline-runtime --example dump_test_generation_suggestion_cards summary
cargo run -p aureline-runtime --example dump_test_generation_suggestion_cards fixture
```

## Suggestion cards

A `TestGenerationSuggestionCard` ties a stable `card_id` and a durable
`GeneratedTestSubject` to:

- a `GenerationSourceKind` — `uncovered_coverage_path`, `uncovered_branch_path`,
  `bug_reproduction`, `regression_guard`, `changed_code_gap`, or
  `unknown_requires_review` — the uncovered-path / bug provenance, so a proposal is
  always tied to a concrete gap;
- a `ProposalProvenance` (`locally_generated`, `imported_proposal`,
  `unknown_requires_review`);
- a list of `TargetReference` rows naming the target symbols and files the generated
  test exercises;
- a list of `EvidenceReference` rows over the `EvidenceObjectKind` vocabulary
  (`coverage_overlay`, `discovery_snapshot`, `session_plan`, `attempt_record`,
  `stability_verdict`, `diagnostic_record`, `bug_report`);
- a list of named `AssumptionEntry` rows;
- a list of `GeneratedFileEntry` rows over the `GeneratedFileKind` vocabulary
  (`new_test_file`, `appended_test_file`, `new_fixture_file`, `new_snapshot_baseline`);
- a `SandboxValidation` block and an `ApplyPosture` block.

The packet validation requires both an uncovered-path source and a bug source to be
represented (`source_kind_case_missing`), both a `sandbox_validated_pass` and an
unvalidated posture (`validation_posture_case_missing`), both an `applied` and a
`blocked_needs_validation` apply state (`apply_state_case_missing`), at least one
imported proposal held read-only (`imported_proposal_case_missing`), at least one
card that exercises the no-silent-widening guard (`widening_guard_case_missing`), and
both `parameterized_template` and `concrete_invocation` subject kinds
(`template_collapsed_with_invocation`), so the vocabulary is exercised, not merely
declared.

## Disclosure before any apply path

A proposal must name its assumptions, evidence basis, affected files, and a
determinate validation posture before any apply path appears
(`apply_path_without_disclosure`). An apply path "appears" once the card reaches a
`previewed`, `applied`, or `reverted` state; at that point `assumptions`,
`evidence_basis`, `targets`, and `generated_files` must be non-empty and the
`SandboxValidation` posture must be determinate (not `not_validated` and not
`sandbox_validation_pending`).

## Evidence-bound, not free-text

Every card carries at least one `EvidenceReference`, and each reference is reopenable:
it carries a non-empty `evidence_ref` and an `evidence_fingerprint_token` distinct
from that ref, so generated-test review reopens the exact discovery / session /
coverage object that motivated the proposal rather than reading a free-text
justification (`evidence_not_reopenable`). The same fingerprint discipline applies to
the `GeneratedTestSubject` (`fingerprint_substitutes_identity`) and to each
`TargetReference`.

## Diff-first apply parity

An `ApplyPosture` carries an `ApplyState` (`pending_preview`, `previewed`, `applied`,
`reverted`, `rejected`, `blocked_needs_validation`), a `preview_first` flag, a
`diff_ref`, an optional `revert_ref`, and a `widens_beyond_evidence` flag. A generated
test is applied like any other change, not written blindly:

- **Sandbox-validated before apply.** An `applied` card must carry a
  `sandbox_validated_pass` posture backed by an isolated sandbox run that executed and
  passed every generated case (`applied_without_sandbox_validation`). A not-yet-validated
  proposal stays `blocked_needs_validation`.
- **Preview-first.** An `applied` card must have been presented preview-first
  (`applied_bypasses_preview`).
- **No silent widening.** A card that would widen the test scope beyond its evidenced
  basis sets `widens_beyond_evidence` and may never be `applied`; it routes to review
  (`applied_widens_beyond_evidence`).
- **Follow-on rerun linkage.** An `applied` card carries a `follow_on_rerun_ref` so
  the generated test is diagnosed like an ordinary change
  (`applied_without_rerun_linkage`).

## Imported never reads as local

An imported proposal carries an `origin_provider_ref`, an `imported_read_only` subject
identity, and an `imported_unvalidated` posture, and is never `applied` as a local
result (`imported_reads_as_local`). A non-imported proposal carries none of those
markers.

## Consumer projection and boundary discipline

The `TestGenerationConsumerProjection` block records that the suggestion-card UI, the
preview / diff / apply / revert pipeline, generated-test review (which reopens the
evidence objects), the follow-on rerun / diagnose flows, and release / support exports
all normalize onto these records instead of re-deriving test-generation truth.

The packet carries only typed class tokens, booleans, counts, opaque ids, fingerprint
digests, and redaction-aware reviewable labels. Generated source bodies, raw model
prompts / completions, sandbox stdout, diff bytes, raw provider payloads, provider
cursors, credentials, and host names never cross this boundary.
