# M5 approver matrices and review-pack summaries

Two reusable M5 protected-path governance components — the **approver matrix** and the **review-pack
summary** — so a user can tell *which* approvers are required, *where* that requirement came from,
whether each approval is satisfied, pending, waived, or expired, whether a review pack was evaluated
locally, is provider-authoritative, is CI-only, was not evaluated here, or is stale relative to
base/head, and which checks or waivers are suppressed, before they trust, merge, or release a
governed change.

- Implementation: `crates/aureline-review/src/implement_approver_matrices_and_review_pack_summaries_with_requirement_source_approval_state_local_versus_provider_evaluation_parity_suppressed_check_visibility_and_freshness_truth`
- Boundary schema: [`schemas/ui/m5-approver-review-pack-controls.schema.json`](../../../schemas/ui/m5-approver-review-pack-controls.schema.json)
- Checked support export: `artifacts/release/m5-approver-review-pack-controls-proof/support_export.json`
- Narrowed fixtures: `fixtures/ui/m5-approver-review-pack-controls/`

This lane *implements* two of the families frozen by the
[protected-path governance component matrix](freeze_the_m5_protected_path_governance_component_matrix.md).
It reuses that matrix's frozen governance-state vocabulary — `advisory`, `authoritative`, `covered`,
`backup_missing`, `waived`, `expired`, `stale`, `provider_authoritative`, `local_estimate` — verbatim
rather than minting a drifted lexicon. The states this lane owns — `waived`, `expired`, `stale`,
`provider_authoritative`, and `local_estimate` — always render under their frozen tokens. Honest
states the frozen lexicon does not name (`satisfied`, `pending`, `ci_only`, `not_evaluated_here`)
carry **no** governance token and never borrow another state's label.

## Derived resolvers

The module derives every honesty axis from an honest input, so a component can never *assert* a
posture it did not earn:

- `resolve_evaluation_locus(source)` maps an evaluation-locus source to the exact locus posture
  (`provider_authoritative`, `local_only`, `ci_only`, `not_evaluated_here`, or
  `stale_relative_to_head`) and, where the frozen lexicon names one, to the governance-state token it
  renders under. Both the approver matrix and the review-pack summary call it, so their
  local-versus-provider parity is one truth. A `provider_enforced_gate` or `provider_reported_status`
  source is provider-authoritative; a `local_evaluation_only` source is local-only; a
  `ci_reported_only` source is CI-only; a `not_evaluated_here` source was not evaluated here; and a
  `stale_against_base_head` source is stale relative to base/head. A CI-only or local-only evaluation
  can never claim provider-authoritative, and a not-evaluated-here pack can never claim it was
  evaluated.
- `resolve_approver_state(source)` maps an approver-state source to the exact approver state
  (`satisfied`, `pending`, `waived`, or `expired`). Only `satisfied` is a clean satisfied approval; a
  `waived` or `expired` approval degrades explicitly under its own `waived` / `expired` token and
  never collapses into generic `approved` language.

## Components

- **Approver matrix row** — names its approver role (as an **export-safe role alias**, never
  person-specific contact detail), requirement source, satisfied/pending/waived/expired state,
  local-versus-provider evaluation parity, evidence link, and expiry where relevant.
  `OpenEvidenceLink`, `InspectRequirementSource`, and `ReviewApproverState` are always offered, so the
  evidence, the requirement source, and the approver state stay inspectable before a user trusts the
  sign-off.
- **Review-pack summary** — names its pack digest, base/head identity, capability set,
  local-versus-provider parity, evaluation freshness, and suppressed checks or waivers.
  `InspectEvaluationParity`, `ReviewSuppressedChecks`, and `OpenPackDigest` are always offered, so the
  parity, the suppressed checks, and the pack digest stay inspectable before a user trusts the pack.

## Acceptance criteria

- **A review pack's evaluation locus is always distinguishable.** The review-pack summaries alone
  cover all five locus postures — `local_only`, `provider_authoritative`, `ci_only`,
  `not_evaluated_here`, and `stale_relative_to_head` — so a user can tell whether a pack is local-only,
  provider authoritative, CI-only, not evaluated here, or stale relative to base/head without opening
  raw logs. A CI-only or local-only summary that claims provider-authoritative, or a
  not-evaluated-here summary that claims it was evaluated, fails validation.
- **Approver state never collapses waived or expired into generic `approved` language.** A waived or
  expired approver row that claims a satisfied approval fails validation, and each waived / expired
  row must carry its `waived` / `expired` governance token, its explaining note, and its expiry label.

## Coverage and invariants

The controls packet's validator enforces the honesty invariants directly:

- The union of both vectors covers every evaluation-locus source and posture; the approver rows cover
  every approver-state source and posture, every evidence-link kind, and every requirement-source
  class; the review-pack summaries cover every locus posture, every pack capability, and every
  suppression class.
- Every component's four hard invariants (`hides_requirement_source_or_state` /
  `hides_parity_or_freshness`, `lets_ci_or_local_read_as_provider_authoritative`,
  `lets_waived_or_expired_read_as_satisfied` / `hides_suppressed_checks_or_waivers`, and
  `invents_alternate_state_label`) must be `false`.
- Raw approval logs, raw provider payloads, raw review-pack bodies, person-specific contact detail,
  credentials, and secrets stay outside the export boundary; every approver is an export-safe role
  alias and every evidence and pack reference is an opaque export-safe reference.

## Regenerating artifacts

The checked support export, Markdown summary, and narrowed fixtures are emitted by the gated
generator test:

```
GEN_APPROVER_REVIEW_PACK_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review \
  implement_approver_matrices_and_review_pack_summaries_with_requirement_source_approval_state_local_versus_provider_evaluation_parity_suppressed_check_visibility_and_freshness_truth::tests::generate_artifacts \
  -- --exact --ignored
```
