# Collection Truth Alpha Fixtures

These fixtures protect the shared dense-collection contract in
`aureline-search`.

- `query_backed_scope_truth.json` proves a search-like result collection keeps
  user filters separate from hidden workset, policy, and provider narrowing;
  carries versioned saved-view state without transient selection; and exposes
  visible, loaded, all-matching, selected, blocked, and hidden counters.

