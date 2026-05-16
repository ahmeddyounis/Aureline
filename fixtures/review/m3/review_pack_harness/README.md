# Review-pack parity-harness alpha fixtures

These checked-in JSON fixtures drive the alpha review-pack parity-harness
family validated against
[`/schemas/review/review_pack_parity_harness.schema.json`](../../../../schemas/review/review_pack_parity_harness.schema.json)
and projected by `aureline_review::review_pack_parity_harness`. Every
fixture is one post-execution parity record: a local-lane and CI-lane
run of one upstream review-pack DSL record were compared and the
resulting per-check parity findings, drift downgrades, and overall
verdict were frozen as one row.

| Fixture | Upstream pack | Overall verdict | Row downgrade | What it proves |
| --- | --- | --- | --- | --- |
| `first_party_full_parity_run.json` | `review_pack_alpha:first_party:core_review` | `full_parity` | `no_downgrade` | First-party bundle runs identically in local and CI lanes. |
| `team_shared_mixed_parity_documented.json` | `review_pack_alpha:team_shared:mixed_parity` | `full_parity` | `no_downgrade` | Team-shared pack's local-only and CI-only-by-design checks match the documented parity claim. |
| `partner_signed_ci_only_documented.json` | `review_pack_alpha:partner:cosign_audit` | `full_parity` | `no_downgrade` | Partner-signed audit runs CI-only as documented while ownership review matches in both lanes. |
| `uncertified_community_drift_downgrade.json` | `review_pack_alpha:community:experimental_lints` | `drift_downgraded` | `downgraded_to_review_required` | Drift on a parity-unknown probe downgrades the row instead of preserving a green claim. |

Every fixture keeps raw paths, raw glob bodies, raw command lines, and
raw check outputs closed; only opaque ref labels, closed-vocabulary
tokens, and short reviewable sentences cross the boundary. Each check
finding references the upstream review-pack `check_id`, and a
`drift_detected` finding always pairs with a matching `drift_downgrades`
entry.
