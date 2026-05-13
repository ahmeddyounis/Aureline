# Search Alpha Validation

Packet id: `search_alpha_validation.external_alpha`  
Owner: `@ahmeddyounis`  
Latest capture: `artifacts/milestones/m2/captures/search_alpha_validation_capture.json`  
Validator: `ci/check_m2_search_alpha.py`

## Purpose

This is the canonical alpha search validation packet for discoverability,
ranking-reason copy, and keyboard parity. It consumes the existing planner,
result-ID, ranking-card, palette discoverability, keyboard-audit, and
partial-index drill artifacts instead of restating a second search truth model.

## Evidence

| Evidence | Ref |
|---|---|
| Review packet | `docs/review/m2_search_alpha_review.md` |
| Runtime consumer | `crates/aureline-shell/src/search/alpha_validation.rs` |
| Combined Rust test | `crates/aureline-shell/tests/search_alpha_validation.rs` |
| Ranking card fixtures | `fixtures/search/ranking_reason_alpha/` |
| Keyboard fixture | `fixtures/accessibility/m2_search_keyboard/search_keyboard_parity.yaml` |
| Palette discoverability fixtures | `fixtures/commands/alpha_palette_queries/` |
| Partial-index drill | `artifacts/benchmarks/m2_partial_index_drill.md` |
| Known-limits packet | `artifacts/milestones/m2/known_limits_alpha.yaml` |

## Acceptance State

`accepted_with_known_limits`

The protected search lane passes the combined UX and accessibility review for
quick-open and symbol-search alpha paths. The accepted evidence proves stable
result IDs, structured ranking reasons, partial/readiness disclosure, same
surface result explanation routes, and keyboard access through the launch
keyboard audit.

## Known Limits

- `known_limit:external_alpha.search_alpha_synthetic_and_partial_index_only`
  keeps this proof scoped to synthetic protected fixtures and partial-index
  described-count drills. It does not widen to full partner-repository
  performance, all language bundles, or complete graph explainer coverage.

## Verification

```sh
cargo test -p aureline-shell --test search_alpha_validation
python3 ci/check_m2_search_alpha.py --repo-root . --report artifacts/milestones/m2/captures/search_alpha_validation_capture.json
```
