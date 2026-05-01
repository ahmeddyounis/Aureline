# Claim-publication binding fixtures

Worked fixtures for the claim-publication automation, evidence-binding,
and stale-claim fail-gate contract frozen in
[`/docs/release/claim_publication_automation_contract.md`](../../../docs/release/claim_publication_automation_contract.md),
the boundary schema
[`/schemas/governance/claim_publication_binding.schema.json`](../../../schemas/governance/claim_publication_binding.schema.json),
and the CI gate policy
[`/ci/claim_publication_gate.yaml`](../../../ci/claim_publication_gate.yaml).

Each case ships as a complete `claim_publication_binding_record`
covering one of the typed gate verdicts, blocking-failure classes,
diff-kind classes, channel categories, and audience / redaction-profile
pairings. The fixtures exercise the schema-enforced declared-to-
effective posture narrowing, the badge-channel surface-pairing rule,
the `not_published_this_run` ↔ suppressed pairing, the
`narrowed_from_declared` ↔ `narrowing_action_class` consistency
constraint, and the audience ↔ redaction-profile pairing.

## Index

| Case | Fixture | Posture |
| --- | --- | --- |
| Docs freshness — passed | `passed_docs_freshness_truth_publication.yaml` | `claim_bearing` declared and effective; `gate_state_class = passed`; `diff_kind_class = no_change`; `release_readiness` audience |
| Exact-build identity — narrowed_pass | `narrowed_pass_exact_build_identity_evidence_narrower_than_claim.yaml` | declared `claim_bearing` → effective `limited`; `evidence_narrower_than_claim` permissible under narrowed_pass; `narrowed_to_limited`; `diff_kind_class = claim_row_narrowed_in_run` |
| Workflow-bundle badge — blocked widening | `blocked_widening_workflow_bundle_badge_stale.yaml` | `gate_state_class = blocked_widening`; `stale_workflow_bundle_badge` blocking failure; `diff_kind_class = badge_widened_blocked`; `workflow_bundle_badge` channel surface pairing |
| Release notes — fail-closed missing caveat | `fail_closed_release_notes_missing_caveat.yaml` | `gate_state_class = fail_closed`; `destination_missing_required_caveat` not permissible under narrowed_pass; `diff_kind_class = claim_row_widening_blocked`; `support_handoff` audience |
| Marketplace orphan card — fail-closed orphan claim | `fail_closed_orphan_claim_text_marketplace_card.yaml` | `gate_state_class = fail_closed`; `orphan_claim_text` and `claim_row_not_in_baseline` failures; `diff_kind_class = claim_row_widening_blocked`; `engineering_internal` audience |
| Certification badge — fail-closed revoked | `fail_closed_certification_badge_revoked.yaml` | declared `claim_bearing` → effective `replacement_grade`; `badge_revoked_for_path` not permissible under narrowed_pass; `narrowed_to_replacement_grade`; `diff_kind_class = badge_downgraded_in_run`; `public_proof_safe` audience |

## Intended usage

- **First-class governed object conformance.** Every fixture carries a
  stable `binding_id`, a stable `publication_run_id`, a stable
  `claim_row_ref` resolving into the claim-manifest baseline, the typed
  `evidence_binding_rules` envelope (freshness floor × support-class
  alignment × known-limit coverage × badge-downgrade × narrowing
  action), the typed `evidence_resolution_rows[]` (with per-row
  freshness, scope, and result-status state), the typed
  `destination_publication_rows[]` (with projection-kind ×
  projected-copy-state × badge-state × narrowing-action consistency),
  the typed `publication_diff` envelope with per-channel change rows,
  the typed `gate_verdict` envelope with closed blocking-failure
  vocabulary, the typed audience and redaction-profile pair, the typed
  `linked_artifact_families` block, and the typed
  `consuming_surface_parity` floor. A surface that renders the binding
  as a free-text status note is non-conforming.
- **Audience and redaction conformance.** The schema enforces that
  `audience_class` matches `redaction_profile_class` per the contract's
  pairing table. The fixtures cover engineering-internal,
  support-handoff, release-readiness, and public-proof-safe audiences
  with the matching redaction profiles.
- **Declared → effective posture narrowing conformance.** The schema
  enforces that when `declared_claim_posture = claim_bearing` and
  `effective_claim_posture` is one of `{experimental, limited,
  policy_disabled, replacement_grade, seed_only, withdrawn}`, the
  binding's `gate_verdict.automatic_narrowing_applied` MUST be `true`.
  The narrowed_pass and certified-badge-revoked fixtures exercise this.
- **Badge-channel surface-pairing conformance.** The schema enforces
  that `channel_id` in `{certification_badge, workflow_bundle_badge}`
  requires `projection_kind = badge_only` and a non-`not_applicable`
  `badge_state_class`. The blocked-widening and certification-revoked
  fixtures exercise both badge channels.
- **`not_published_this_run` conformance.** The schema enforces that
  `projection_kind = not_published_this_run` pairs with
  `projected_copy_state_class` in the `suppressed_*` family. The
  release-notes-missing-caveat and certification-revoked fixtures
  exercise this with `suppressed_pending_caveat` and
  `suppressed_replacement_grade` respectively.
- **`narrowed_from_declared` consistency.** The schema enforces that
  `narrowed_from_declared = true` requires
  `narrowing_action_class != no_narrowing_required` and conversely.
  The narrowed_pass fixture sets every destination row to
  `narrowed_from_declared = true` paired with `narrowed_to_limited`;
  the passed and blocked-widening fixtures set every row to
  `narrowed_from_declared = false` paired with `no_narrowing_required`.
- **Gate-verdict conformance.** The schema enforces that
  `gate_state_class = passed` requires `blocking_failure_rows = []`,
  `automatic_narrowing_applied = false`, and
  `narrowed_to_posture = null`; that `gate_state_class = narrowed_pass`
  requires `automatic_narrowing_applied = true` and a non-null
  `narrowed_to_posture`; and that `gate_state_class` in
  `{blocked_widening, fail_closed}` requires non-empty
  `blocking_failure_rows[]`. The fixtures exercise all four classes.
- **Diff-kind conformance.** The schema enforces that
  `diff_kind_class = new_claim_row_published` requires
  `previous_publication_snapshot_ref = null`. The passed,
  narrowed_pass, blocked-widening, fail-closed-caveat,
  fail-closed-orphan, and certified-revoked fixtures all carry
  non-null previous snapshot refs paired with non-`new_claim_row_published`
  diff classes.

## Acceptance coverage

The acceptance criteria from
[`/.plans/M00-525.md`](../../../.plans/M00-525.md) are covered as
follows:

- **"New or changed claims cannot publish without a bound evidence
  row, current destination list, and matching support/known-limit
  posture."** — the fail-closed-orphan fixture pins
  `claim_row_not_in_baseline` and `orphan_claim_text` against a draft
  marketplace-discovery card whose claim_row_ref does not resolve in
  the manifest baseline; the fail-closed-missing-caveat fixture pins
  `destination_missing_required_caveat` against a release-notes card
  that dropped the version-skew-alias caveat.
- **"Reviewers can compare old and new published claim sets from one
  generated diff instead of reading every destination manually."** —
  every fixture pins a `publication_diff` envelope with the previous
  and current publication-snapshot refs and one `per_channel_diff_row`
  per destination, so the diff is reproducible from the binding alone.
  The narrowed_pass fixture renders four `copy_narrowed` rows; the
  certified-revoked fixture renders one `badge_state_changed` row plus
  two `copy_narrowed` rows plus one `removed_via_narrowing` row.
- **"Broken, stale, or over-broad claim bindings fail closed rather
  than allowing silent public overclaim."** — the fail-closed-orphan
  fixture pins `gate_state_class = fail_closed`; the
  fail-closed-missing-caveat fixture pins `gate_state_class =
  fail_closed`; the certified-revoked fixture pins `gate_state_class =
  fail_closed`; the blocked-widening fixture pins `gate_state_class =
  blocked_widening`. Every fail-closed and blocked-widening fixture
  emits at least one typed `blocking_failure_row` so the verdict is
  inspectable.

## Out of scope

Documentation publishing, release-note rendering pipelines, Help/About
or service-health renderers, workflow-bundle or certification badge
renderers, public-proof publishing services, and live publication
generators are explicitly out of scope per
[`.plans/M00-525.md`](../../../.plans/M00-525.md). The contract
defines the source object; integrations consume it.
