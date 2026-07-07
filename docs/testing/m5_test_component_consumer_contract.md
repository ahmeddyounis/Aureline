# M5 Test-Explorer / Watch / Triage Component Consumer Contract (M05-913)

This contract is the consumer-adoption lane over the frozen M5 test-explorer /
watch / triage component matrix
([`m5-test-explorer-watch-triage-component-matrix.schema.json`](../../schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json),
frozen by M05-908) and the 909–912 primitive resolvers. Where the freeze matrix
defines the seven reusable component families and 909–912 resolve their
per-surface truth, this lane proves the families are reusable **primitives**
rather than one test-tree pane by adopting them across the claimed M5 test
consumer surfaces beyond the primary test tree.

- Boundary schema: [`schemas/ui/m5-test-component-consumer.schema.json`](../../schemas/ui/m5-test-component-consumer.schema.json)
- Release proof: [`artifacts/release/m5-test-component-consumer-proof/`](../../artifacts/release/m5-test-component-consumer-proof/)
- Emitter: `cargo run -p aureline-runtime --bin aureline_runtime_test_component_consumers -- support-export`

## The seven canonical families

Each consumer row points back to exactly one canonical component family — its
primitive schema **and** its release-proof packet — instead of cloning
surface-local test vocabulary:

| Family | Canonical schema | Resolver |
| --- | --- | --- |
| `test_tree_row` | `m5-test-tree-row.schema.json` | M05-909 |
| `inline_result_marker` | `m5-inline-result-marker.schema.json` | M05-910 |
| `session_summary_bar` | `m5-test-session-summary-bar.schema.json` | M05-911 |
| `watch_mode_banner` | `m5-test-watch-mode-banner.schema.json` | M05-911 |
| `failure_triage_panel` | `m5-test-failure-triage-panel.schema.json` | M05-912 |
| `quarantine_review_sheet` | `m5-test-quarantine-review-sheet.schema.json` | M05-912 |
| `environment_matrix_card` | `m5-test-environment-matrix-card.schema.json` | M05-912 |

The session/watch families share the 911 release packet, and the
triage/quarantine/environment families share the 912 release packet, because
those primitives are twin- and triple-resolvers.

## The four consumer classes and eight surfaces

| Class | Surfaces |
| --- | --- |
| `day_to_day_editor` | `status_bar_summary`, `activity_center` |
| `quality_intelligence` | `coverage_intelligence`, `flaky_intelligence`, `snapshot_review` |
| `pipeline_imported` | `pipeline_overlay`, `imported_ci_view` |
| `support_export` | `support_packet` |

All four classes and all eight surfaces must be adopted, every family must be
adopted by at least one consumer, and at least one family must be adopted across
two or more classes (the strongest evidence the family is a reusable primitive).

## One truth across surfaces (AC1)

Every consumer keeps the identical controlled label families —
`result_freshness`, `target_class`, `watch_state`, `quarantine_semantics`, and
imported-versus-live `result_origin` — and one **shared state lexicon**
(`failed`, `rerun_failed`, `quarantined`) verbatim. The union of preserved label
families across the packet must cover all five, and every row must carry the
identical lexicon so a red mark, a re-run of only the failing selection, and a
suppressed test read the same on every surface. A row that renames, flattens, or
drops a governed label is rejected.

## Auto-narrowed claim language (AC2)

When a consumer's evidence is weaker than a full local-live claim, it
**auto-narrows** its visible claim language and discloses the reduction with an
auto-narrow banner naming the reason(s) and a recovery hint. The four spec-named
conditions are:

| Reason | Meaning | Recovery |
| --- | --- | --- |
| `results_imported` | imported result — not a local rerun | rerun locally to produce a live result |
| `target_compatibility_drift` | target compatibility drifted from this result | rerun on the matching target to re-verify |
| `watch_fidelity_degraded` | watch fidelity degraded — not live | recover the watch session for live fidelity |
| `quarantine_visibility_restricted` | quarantine visibility restricted by policy | open the quarantine-review sheet with scope |

Every one of the four conditions is demonstrated by at least one consumer. A
narrowed row must carry a banner whose `reasons` exactly match the row's
`claim_narrow_reasons`, carry a non-empty recovery hint, and use a
`disclosed_narrowed` label parity — never a generic label and never a spurious
banner on a full-claim row.

### Imported-CI and local-live stop diverging

An imported-origin consumer (`imported_ci`, `imported_teammate`, or
`replayed_snapshot`) must carry the `results_imported` narrow reason and may
never claim a full local-live parity; a `live_local` consumer may never claim its
result was imported. The packet must carry **both** an imported auto-narrowing
consumer and a local-live consumer, so the two surfaces resolve to one meaning of
`failed`, `rerun failed`, and `quarantined` instead of diverging.

## Metadata-only boundary

The packet carries only typed class tokens, opaque summary / evidence refs,
booleans, and redacted labels. Raw runner output, assertion diffs, stack frames,
credentials, and provider payloads never cross this boundary; the validator
rejects any export that contains obviously forbidden material.
