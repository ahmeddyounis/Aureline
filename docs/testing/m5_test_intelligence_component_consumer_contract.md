# M5 Test-Intelligence Component Consumer Contract (M05-1033)

This contract is the consumer-adoption lane over the frozen M5 test-intelligence
component matrix
([`m5-test-intelligence-component-matrix.schema.json`](../../schemas/ui/m5-test-intelligence-component-matrix.schema.json),
frozen by M05-1028) and the M05-1029..1032 primitive resolvers. Where the freeze
matrix defines the seven reusable quality-evidence component families and
1029-1032 resolve their per-surface truth, this lane proves the families are
reusable **primitives** by adopting them across the claimed M5 quality surfaces
users actually inspect before trusting a green bar, a flaky verdict, or a
generated test.

- Boundary schema: [`schemas/ui/m5-test-intelligence-component-consumer.schema.json`](../../schemas/ui/m5-test-intelligence-component-consumer.schema.json)
- Release proof: [`artifacts/release/m5-test-intelligence-component-consumer-proof/`](../../artifacts/release/m5-test-intelligence-component-consumer-proof/)
- Emitter: `cargo run -p aureline-runtime --bin aureline_runtime_test_intelligence_component_consumers -- support-export`

## The seven canonical families

Each consumer row points back to exactly one canonical component family — its
primitive schema **and** its release-proof packet — instead of cloning
surface-local test vocabulary:

| Family | Canonical schema | Resolver |
| --- | --- | --- |
| `coverage_summary_bar` | `m5-coverage-summary-bar.schema.json` | M05-1029 |
| `coverage_overlay_marker` | `m5-coverage-overlay-marker.schema.json` | M05-1029 |
| `flaky_state_badge` | `m5-flaky-state-badge.schema.json` | M05-1030 |
| `retry_history_row` | `m5-retry-history-row.schema.json` | M05-1030 |
| `snapshot_review_card` | `m5-snapshot-review-card.schema.json` | M05-1031 |
| `coverage_import_merge_sheet` | `m5-coverage-import-merge-sheet.schema.json` | M05-1031 |
| `test_generation_suggestion_card` | `m5-test-generation-suggestion-card.schema.json` | M05-1032 |

The coverage summary/overlay families share the 1029 release packet, the
flaky/retry families share the 1030 packet, and the snapshot/merge families
share the 1031 packet, because those primitives are twin-resolvers.

## The six consumer classes and eight surfaces

| Class | Surfaces |
| --- | --- |
| `editor_surface` | `editor_gutter_overlay`, `editor_coverage_summary` |
| `test_tree` | `test_tree_panel` |
| `review_surface` | `review_coverage_diff`, `review_snapshot_card` |
| `cli_summary` | `cli_quality_summary` |
| `imported_ci_detail` | `imported_ci_detail_view` |
| `support_export` | `support_export_packet` |

All six classes and all eight surfaces must be adopted, every family must be
adopted by at least one consumer, and at least one family must be adopted across
two or more classes (the strongest evidence the family is a reusable primitive).

## One truth across surfaces (AC1)

Every consumer keeps the identical controlled label families —
`provenance_and_freshness`, `included_run_scope`, `baseline_identity`,
`raw_or_text_fallback`, and `assumption_boundary` — and one **shared state
lexicon** (`imported_not_local`, `suspected_not_confirmed`,
`generated_review_first`) verbatim. The union of preserved label families across
the packet must cover all five, and every row must carry the identical lexicon so
an imported result, a merely-suspected flaky test, and a generated test read the
same on every surface. A row that renames, flattens, or drops a governed label is
rejected.

Each consumer also keeps the family-appropriate explicit actions reachable
(spec guardrail: keep raw/text fallback and rerun/open-logs actions explicit):

| Family | Required actions |
| --- | --- |
| coverage summary / overlay / merge sheet | `rerun`, `open_report` |
| flaky badge / retry row | `rerun`, `open_logs` |
| snapshot review card | `open_raw_or_text_fallback`, `rerun` |
| test-generation suggestion card | `open_diff_preview`, `rollback` |

## Auto-narrowed claim language (AC2)

When a consumer's evidence is weaker than a verified current-run claim, it
**auto-narrows** its visible claim language and discloses the reduction with an
auto-narrow banner naming the reason(s) and a recovery hint. The five spec-named
conditions are:

| Reason | Meaning | Recovery |
| --- | --- | --- |
| `evidence_imported` | imported CI evidence — not a verified local run | rerun locally to produce a verified current-run result |
| `shard_scope_omitted` | a shard is omitted from the included run set | include the omitted shard, or open the merge sheet for scope |
| `provenance_stale` | cached or stale result — not the current source | rerun to refresh the provenance to the current source |
| `flakiness_unconfirmed` | suspected flaky — not reproduced across attempts | rerun to reproduce before confirming the flaky verdict |
| `generated_assumptions_unverified` | generated test carries unverified assumptions | open the diff preview and review the assumptions before applying |

Every one of the five conditions is demonstrated by at least one consumer. A
narrowed row must carry a banner whose `reasons` exactly match the row's
`claim_narrow_reasons`, carry a non-empty recovery hint, and use a
`disclosed_narrowed` label parity — never a generic label and never a spurious
banner on a full-claim row.

### Imported-CI and verified current-run stop diverging

A weaker-than-current provenance forces its narrow reason: an
`imported_ci_artifact` result carries `evidence_imported`, a `cached_local_result`
or `stale_prior_result` carries `provenance_stale`, and a `suspected_flaky`
result carries `flakiness_unconfirmed` — none may claim a full verified
current-run parity. A `verified_current_run` result may never claim imported or
stale provenance. The packet must carry **both** an imported auto-narrowing
consumer and a verified current-run consumer, so the two surfaces resolve to one
meaning of `imported`, `suspected flaky`, and `generated` instead of diverging.

## Hard invariants (guardrails)

Every row carries five hard invariants that must all be `false`; a `true` value
is rejected:

- `collapses_shard_omission_into_single_percentage` — a single percentage may not
  hide a shard omission or stale provenance.
- `labels_intermittent_as_confirmed_flaky` — one intermittent failure may not read
  as confirmed flakiness.
- `bundles_generated_changes_into_opaque_apply` — generated assertion, fixture,
  and snapshot changes may not collapse into one opaque apply path.
- `rewords_scope_freshness_or_baseline_per_surface` — no surface may reword the
  shared scope / freshness / baseline language.
- `invents_alternate_state_label` — no surface may invent a state label outside
  the frozen vocabulary.

## Metadata-only boundary

The packet carries only typed class tokens, opaque summary / evidence refs,
booleans, and redacted labels. Raw runner output, coverage line data, assertion
diffs, snapshot bytes, stack frames, credentials, and provider payloads never
cross this boundary; the validator rejects any export that contains obviously
forbidden material.
