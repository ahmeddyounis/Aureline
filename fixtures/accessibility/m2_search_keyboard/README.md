# Search Alpha Keyboard Fixtures

These fixtures protect the combined search UX and keyboard review lane. They
tie quick-open and symbol-search ranking cards to the launch keyboard audit so
`Why this result?` remains reachable without inventing a separate search
accessibility contract.

Run:

```sh
cargo test -p aureline-shell --test search_alpha_validation
python3 ci/check_m2_search_alpha.py --repo-root .
```
