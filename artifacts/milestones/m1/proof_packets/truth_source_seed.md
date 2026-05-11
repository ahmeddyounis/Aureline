# Proof packet: M1 docs/help/About/service-health truth-source seed

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical badge-vocabulary seed for the provenance,
freshness, client-scope, install-mode, service-health, and
source-class badges that Help, About, service-health, and the docs/
help browser project. The lane proves the seed is consumable by the
live shell modules and the support-export copy without re-encoding
the badge vocabulary on each surface.

Reviewer entry point:
[`/docs/help/truth_source_model.md`](../../../docs/help/truth_source_model.md).

Canonical sources:

- `artifacts/help/m1_truth_source_examples.yaml` — seed rows the
  runner consumes; one row per badge family
  (`docs_help_source_class`, `docs_help_version_match_state`,
  `docs_help_freshness_class`, `client_scope_badge_family`,
  `install_mode_class`, `provenance_row_state`,
  `service_health_state`) with a closed `vocabulary_tokens` list, a
  `frozen_token_count` lock, an `honesty_fallback_token`, the four
  required `consuming_surface_classes`, one named runtime consumer,
  one canonical example payload ref, and one named failure drill.
- `schemas/help/provenance_badge_vocabulary.schema.json` — boundary
  schema for the seed; freezes closed vocabularies for
  `badge_family_class`, `vocabulary_role_class`,
  `consuming_surface_class`, `consumer_class`, and the structural
  invariants (every row carries all four required surfaces, the
  honesty-fallback token lives in vocabulary_tokens with the
  matching role, support_export_compatible is the constant `true`,
  and the `provenance_row_state` and `service_health_state` rows
  declare the seed placeholder explicitly).
- `fixtures/help/m1_truth_source_examples/` — canonical example
  payloads (one per row) that the runner reparses end-to-end so the
  seed's tokens and the example surfaces stay parity-consistent.
- `tests/help/m1_truth_source_seed_lane/run_m1_truth_source_seed_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

Named runtime consumers:

- `docs/help/badge_vocabulary_draft.md` — consumer-facing draft that
  quotes every row's vocabulary verbatim so docs / support / export
  copy reads the same tokens as the live shell.
- `crates/aureline-shell/src/help_about/mod.rs` — live Help / About /
  provenance / service-health surface that already renders the
  client-scope, install-mode, provenance, and service-health
  vocabularies the seed pins.
- `crates/aureline-shell/src/docs_browser/state.rs` — live docs/help-
  browser projection that already renders the source class,
  version-match state, and freshness class tokens the seed pins.

Live runtime consumers (read-only):

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

Validation captures:

- `artifacts/milestones/m1/captures/truth_source_seed_validation_capture.json`

Refresh: re-run the validation lane after a change to the seed YAML,
the boundary schema, the badge vocabulary draft, the canonical
example payloads, the reviewer-facing landing page, or any of the
named runtime consumers.

Closure rule: the lane stays open until the latest capture lands
under the governed proof root and every row reports PASS for closed-
vocabulary membership (`badge_family_class`, `consuming_surface_class`,
`consumer_class`, `vocabulary_role_class`, `failure_drill_id`), the
`frozen_token_count` lock, the honesty-fallback invariant
(`honesty_fallback_token` non-empty, present in `vocabulary_tokens`
with role `honesty_fallback_token`, and unique on the row), the
degraded-state honesty rule (no widening of canonical degraded tokens
to live), the seed-placeholder honesty rule (the
`provenance_row_state` and `service_health_state` rows must carry the
`seed_placeholder_awaiting_wiring` token with role
`seed_placeholder_token`), the required consuming-surface coverage
(every row binds `help_pane`, `about_pane`, `service_health_pane`,
`docs_browser_pane`), named-runtime-consumer existence, example-
payload agreement (kind / row_id / badge_family_class / honesty
fallback / surface-class membership / rendered_token membership),
required badge-family coverage (all seven families seeded), and its
named failure drill.

Failure-drill coverage (seven named drills, all reproducible under
`--force-drill <row_id>:<drill_id>`):

| Row | Drill | Expected check id |
|---|---|---|
| `truth_source.docs_help_source_class` | `truth_source_drill.honesty_fallback_token_dropped` | `truth_source.honesty_fallback_token_missing` |
| `truth_source.docs_help_version_match_state` | `truth_source_drill.version_match_token_count_drifted` | `truth_source.vocabulary_token_count_mismatch` |
| `truth_source.docs_help_freshness_class` | `truth_source_drill.freshness_degraded_pair_widened` | `truth_source.degraded_state_token_widened` |
| `truth_source.client_scope_badge_family` | `truth_source_drill.required_consuming_surface_help_pane_dropped` | `truth_source.required_consuming_surface_missing` |
| `truth_source.install_mode_class` | `truth_source_drill.install_mode_unknown_token_dropped` | `truth_source.honesty_fallback_token_not_in_vocabulary` |
| `truth_source.provenance_row_state` | `truth_source_drill.provenance_seed_placeholder_dropped` | `truth_source.seed_placeholder_role_widened` |
| `truth_source.service_health_state` | `truth_source_drill.service_health_seed_placeholder_dropped` | `truth_source.seed_placeholder_token_required` |
