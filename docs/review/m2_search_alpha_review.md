# Search Alpha Review Packet

This packet is the reviewer entry point for the protected alpha search lane.
It validates that quick-open and symbol-search rows are discoverable,
explainable, and keyboard reachable while broader indexing and graph work are
still partial.

## Scope

Covered surfaces:

- quick open with a hot-set lexical file row;
- symbol search with structural fallback while graph proof is warming;
- palette discoverability for command, symbol, and file rows;
- keyboard access to the same-surface `Why this result?` route.

Out of scope: full codebase explainer depth, replacement-grade ranking,
long-tail language graph completeness, and partner-repository performance
measurements.

## Canonical Inputs

- Planner contract: `docs/search/query_planner_contract_seed.md`
- Result ID and ranking contract: `docs/search/result_identity_and_ranking.md`
- Explainability contract: `docs/search/search_explainability_contract.md`
- Ranking fixtures: `fixtures/search/ranking_reason_alpha/`
- Palette discoverability contract: `docs/ux/alpha_discoverability.md`
- Keyboard audit: `docs/accessibility/m2_keyboard_gap_audit.md`
- Search keyboard fixture: `fixtures/accessibility/m2_search_keyboard/search_keyboard_parity.yaml`
- Partial-index drill: `artifacts/benchmarks/m2_partial_index_drill.md`
- Milestone validation packet: `artifacts/milestones/m2/search_alpha_validation.md`

## Review Result

The combined packet accepts the alpha search lane with known limits:

- quick-open and symbol-search rows carry stable result IDs;
- ranking-reason classes are present and support-exportable;
- partial and structural-fallback states remain visible;
- palette search exposes command, symbol, and file rows from one entry route;
- the keyboard audit covers the palette diagnostics route used to reach result
  explanation detail; and
- synthetic/partial-index proof limits are recorded in
  `known_limit:external_alpha.search_alpha_synthetic_and_partial_index_only`.

## Verification

```sh
cargo test -p aureline-shell --test ranking_reason_card_cases
cargo test -p aureline-shell --test keyboard_gap_audit
cargo test -p aureline-shell --test search_alpha_validation
python3 ci/check_m2_search_alpha.py --repo-root .
```
