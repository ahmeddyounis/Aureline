# Local-CI review-pack parity report (alpha)

This report is the deterministic, support-and-partner-facing summary of
the alpha review-pack parity-harness runs. It is generated from the
checked-in parity-harness fixtures under
[`fixtures/review/m3/review_pack_harness/`](../../../fixtures/review/m3/review_pack_harness/)
and validated against
[`schemas/review/review_pack_parity_harness.schema.json`](../../../schemas/review/review_pack_parity_harness.schema.json)
by
[`ci/check_review_pack_parity_harness_alpha.py`](../../../ci/check_review_pack_parity_harness_alpha.py).
The full vocabulary is frozen in
[`docs/review/m3/review_pack_parity_alpha.md`](../../../docs/review/m3/review_pack_parity_alpha.md).

The report exists to prove that the review-pack DSL is more than
syntax: every claimed review-pack feature family has at least one
harness run that reports parity or drift explicitly, and any
drift_detected finding downgrades the affected row instead of silently
preserving a green claim.

## 1 Runs by pack-authority class

| Parity-harness id | Upstream review pack | Authority | Overall verdict | Row downgrade |
| --- | --- | --- | --- | --- |
| `review_pack_parity_harness_alpha:first_party:core_review` | `review_pack_alpha:first_party:core_review` | `repo_first_party` | `full_parity` | `no_downgrade` |
| `review_pack_parity_harness_alpha:team_shared:mixed_parity` | `review_pack_alpha:team_shared:mixed_parity` | `repo_team_shared` | `full_parity` | `no_downgrade` |
| `review_pack_parity_harness_alpha:partner:cosign_audit` | `review_pack_alpha:partner:cosign_audit` | `repo_partner_signed` | `full_parity` | `no_downgrade` |
| `review_pack_parity_harness_alpha:community:drift_downgrade` | `review_pack_alpha:community:experimental_lints` | `repo_uncertified_community` | `drift_downgraded` | `downgraded_to_review_required` |

## 2 Per-check parity findings

### 2.1 `repo_first_party` — `review_pack_alpha:first_party:core_review`

| Check ref | Expected parity | Local outcome | CI outcome | Finding |
| --- | --- | --- | --- | --- |
| `review_pack_check:first_party:schema_validation` | `local_and_ci_parity` | `passed_parity` | `passed_parity` | `full_parity` |
| `review_pack_check:first_party:doc_freshness` | `local_and_ci_parity` | `passed_parity` | `passed_parity` | `full_parity` |
| `review_pack_check:first_party:ownership_review` | `local_and_ci_parity` | `passed_parity` | `passed_parity` | `full_parity` |

Both lanes engaged the bundled validator and the same fixture corpus;
no drift, no downgrade.

### 2.2 `repo_team_shared` — `review_pack_alpha:team_shared:mixed_parity`

| Check ref | Expected parity | Local outcome | CI outcome | Finding |
| --- | --- | --- | --- | --- |
| `review_pack_check:team_shared:policy_lint` | `local_and_ci_parity` | `passed_parity` | `passed_parity` | `full_parity` |
| `review_pack_check:team_shared:format_check_local` | `local_only_documented` | `passed_parity` | `skipped_by_lane_documented` | `local_only_documented_match` |
| `review_pack_check:team_shared:deploy_gate_ci_only` | `ci_only_documented` | `skipped_by_lane_documented` | `passed_parity` | `ci_only_documented_match` |

Both documented divergences match the pack's parity claim; the local
lane never silently engaged the CI-only deploy gate and the CI lane
never silently re-ran the local-only format check.

### 2.3 `repo_partner_signed` — `review_pack_alpha:partner:cosign_audit`

| Check ref | Expected parity | Local outcome | CI outcome | Finding |
| --- | --- | --- | --- | --- |
| `review_pack_check:partner:cosign_audit` | `ci_only_documented` | `skipped_by_lane_documented` | `passed_parity` | `ci_only_documented_match` |
| `review_pack_check:partner:ownership_review` | `local_and_ci_parity` | `passed_parity` | `passed_parity` | `full_parity` |

The partner-signed audit ran only in CI as documented; the local lane
previewed the pack and ran the ownership review with full parity.

### 2.4 `repo_uncertified_community` — `review_pack_alpha:community:experimental_lints`

| Check ref | Expected parity | Local outcome | CI outcome | Finding |
| --- | --- | --- | --- | --- |
| `review_pack_check:community:experimental_lint` | `local_only_documented` | `passed_parity` | `declined_by_lane_documented` | `local_only_documented_match` |
| `review_pack_check:community:parity_unknown_probe` | `parity_unknown_requires_review` | `passed_with_drift_note` | `declined_by_lane_documented` | `drift_detected` |

The parity-unknown probe surfaced a drift note locally; the CI lane
documented-declined the uncertified bundle. The drift_detected finding
pairs with a `drift_downgrades` entry of
`downgraded_to_review_required`, the row's `row_downgrade_class` is
`downgraded_to_review_required`, and the overall verdict is
`drift_downgraded`. The harness can never silently preserve a green
claim under drift.

## 3 Acceptance summary

- At least one harness exists for each claimed review-pack feature
  family. The four parity-harness fixtures cover `repo_first_party`,
  `repo_team_shared`, `repo_partner_signed`, and
  `repo_uncertified_community` packs.
- Paritization failures downgrade the affected row instead of silently
  preserving a green claim. The community fixture is the worked
  example: a drift_detected finding forces a non-`no_downgrade`
  `row_downgrade_class` and a `drift_downgraded` overall verdict, both
  enforced by the schema, the validator, the integration test, and the
  shell consumer.
- The parity report is consumable by docs, support, and partner
  packets. Every parity-harness fixture wires `support_export`,
  `docs_review`, and `cli_headless_entry` consumer surfaces; raw paths,
  raw glob bodies, raw command lines, and raw check outputs stay
  closed.

## 4 Wiring

- Schema — [`schemas/review/review_pack_parity_harness.schema.json`](../../../schemas/review/review_pack_parity_harness.schema.json)
- Fixtures — [`fixtures/review/m3/review_pack_harness/`](../../../fixtures/review/m3/review_pack_harness/)
- Validator — [`ci/check_review_pack_parity_harness_alpha.py`](../../../ci/check_review_pack_parity_harness_alpha.py)
- Reviewer doc — [`docs/review/m3/review_pack_parity_alpha.md`](../../../docs/review/m3/review_pack_parity_alpha.md)
- Rust crate module — [`crates/aureline-review/src/review_pack_parity_harness/mod.rs`](../../../crates/aureline-review/src/review_pack_parity_harness/mod.rs)
- Integration test — [`crates/aureline-review/tests/review_pack_parity_harness_alpha.rs`](../../../crates/aureline-review/tests/review_pack_parity_harness_alpha.rs)
- Shell consumer — [`crates/aureline-shell/src/review/parity_harness_inspector/mod.rs`](../../../crates/aureline-shell/src/review/parity_harness_inspector/mod.rs)
