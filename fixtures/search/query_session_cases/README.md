# Search query-session cases

Worked cases for the canonical search query-session, result-identity, and
explanation-capture contracts:

- `partial_index.yaml` shows a global search served before the full index is
  ready.
- `policy_limited.yaml` shows a docs search narrowed by admin policy and
  captured for support without hidden content.
- `provider_limited.yaml` shows AI context retrieval when a provider-overlay
  lane is unavailable or unauthorized.
- `hidden_by_filter.yaml` shows user-filtered hidden counts that remain
  distinct from zero matches.
- `recentness_boosted.yaml` shows quick-open recentness ranking while raw query
  text stays local-only.

Each case contains one `query_session` record, one or more stable
`result_identities`, and one or more `explanation_captures`. These fixtures are
intended to be read as cross-schema examples; the top-level case wrapper is not
itself a boundary schema.
