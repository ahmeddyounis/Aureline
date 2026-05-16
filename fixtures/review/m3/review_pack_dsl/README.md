# Review-pack DSL alpha fixtures

These checked-in JSON fixtures drive the alpha review-pack DSL family
validated against
[`/schemas/review/review_pack.schema.json`](../../../../schemas/review/review_pack.schema.json)
and projected by `aureline_review::review_pack_dsl`. Every fixture is a
versioned, repo-defined declaration of one review pack: which checks
would run, who owns the affected scopes, how local and CI parity is
claimed, and which fields are unsupported on the current DSL version.

| Fixture | `pack_authority_class` | Parity classes covered | Why it exists |
| --- | --- | --- | --- |
| `first_party_local_and_ci_parity.json` | `repo_first_party` | `local_and_ci_parity` | Bundled schema, doc, and ownership checks share one validator across local and CI. |
| `team_shared_mixed_parity.json` | `repo_team_shared` | `local_and_ci_parity`, `local_only_documented`, `ci_only_documented` | Team-shared pack with a local-only format check and a CI-only deploy gate, both documented. |
| `partner_signed_ci_only_lane.json` | `repo_partner_signed` | `ci_only_documented`, `local_and_ci_parity` | Partner-signed pack whose audit gate runs only in CI; local lane previews it. |
| `uncertified_community_local_only_lane.json` | `repo_uncertified_community` | `local_only_documented`, `parity_unknown_requires_review` | Uncertified community pack where CI declines to engage and one probe is parity-unknown. |

Every fixture keeps raw paths, raw glob bodies, raw command lines, and
raw check outputs closed; only opaque ref labels, closed-vocabulary
tokens, and short reviewable sentences cross the boundary. Each check
that names an `ownership_scope_refs` entry must back it with an
`ownership_hints` row in the same fixture, and every check `parity_class`
must be recorded in at least one matching `parity_observations` entry.
