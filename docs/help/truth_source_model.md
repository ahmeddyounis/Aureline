# Docs/help/About/service-health truth-source model

This page is the reviewer-facing entry point for the canonical
machine-readable seed that pins the provenance, freshness, client-scope,
install-mode, service-health, and source-class badge vocabulary every
in-product Help, About, service-health, and docs/help-browser surface
projects. The seed exists so the four surfaces and the support / export
copy that quotes them never invent a private badge string and never
silently blank out a row when upstream truth is missing.

The seed is the M1 slice of the wider truth-source contract. It owns
the closed token vocabulary the four named surfaces MUST render
verbatim, the explicit honesty-fallback token they render when upstream
truth is missing or unverified, the consuming-surface parity floor that
forbids any surface from forking a private vocabulary, and one named
runtime consumer per row.

## Companion artifacts

- [`/schemas/help/provenance_badge_vocabulary.schema.json`](../../schemas/help/provenance_badge_vocabulary.schema.json)
  — boundary schema for `m1_truth_source_badge_vocabulary_seed`.
- [`/artifacts/help/m1_truth_source_examples.yaml`](../../artifacts/help/m1_truth_source_examples.yaml)
  — canonical seed rows.
- [`/fixtures/help/m1_truth_source_examples/`](../../fixtures/help/m1_truth_source_examples/)
  — per-row canonical example payloads the validation lane reparses
  end-to-end.
- [`/docs/help/badge_vocabulary_draft.md`](badge_vocabulary_draft.md)
  — named runtime consumer that quotes every row verbatim so the
  docs/help / About / service-health / docs-browser surfaces and the
  support-export copy read the same closed vocabulary.
- [`/tests/help/m1_truth_source_seed_lane/run_m1_truth_source_seed_lane.py`](../../tests/help/m1_truth_source_seed_lane/run_m1_truth_source_seed_lane.py)
  — unattended Python validation lane.

Upstream contracts this seed projects from rather than restating:

- [`/docs/help/help_about_truth_source.md`](help_about_truth_source.md)
  — Help/About/provenance/service-health seed surface (M01-086) the
  live shell consumer already renders.
- [`/docs/help/docs_browser_contract.md`](docs_browser_contract.md)
  — docs/help browser skeleton (M01-071) that mints the source class,
  version-match state, and freshness class tokens this seed pins.
- [`/docs/governance/m1_boundary_manifest.md`](../governance/m1_boundary_manifest.md)
  — internal boundary manifest and open/local capability matrix draft
  (M01-099) the client-scope row anchors to.

## Goal

One canonical source of truth for the M1 docs/help/About/service-health
badge vocabulary, frozen so:

- Help, About, service-health, and the docs/help browser cannot fork
  surface-local copy for source class, version-match state, freshness
  class, client scope, install mode, provenance row state, or service-
  health state;
- support / export copy quotes the same tokens the four surfaces
  render, so a copy-context dump and an export packet are not allowed
  to drift from the in-product wording;
- every row carries an explicit honesty-fallback token so a surface
  cannot silently blank out a row when upstream truth is missing or
  unverified;
- the chrome cannot fabricate "all green" by promoting seed placeholders
  before the live verifier / aggregator lands;
- the seed can be extended in M2 without breaking its M1 consumers —
  adding a new badge family is additive-minor, repurposing an existing
  token is breaking.

## Row anatomy

Every row in [`/artifacts/help/m1_truth_source_examples.yaml`](../../artifacts/help/m1_truth_source_examples.yaml)
freezes one badge family with the following fields. The boundary schema
[`/schemas/help/provenance_badge_vocabulary.schema.json`](../../schemas/help/provenance_badge_vocabulary.schema.json)
enforces every constraint listed here.

| Field | Meaning |
|---|---|
| `row_id` | Stable dot-prefixed id (`truth_source.<family>`). |
| `badge_family_class` | Closed family vocabulary, one of `docs_help_source_class`, `docs_help_version_match_state`, `docs_help_freshness_class`, `client_scope_badge_family`, `install_mode_class`, `provenance_row_state`, `service_health_state`. |
| `vocabulary_tokens` | Closed token vocabulary the consuming surfaces render verbatim. Each token names a `role`: `live_state_token`, `degraded_state_token`, `seed_placeholder_token`, or `honesty_fallback_token`. |
| `frozen_token_count` | The locked count of tokens in `vocabulary_tokens`. Adding or removing a token without bumping this count fails `truth_source.vocabulary_token_count_mismatch`. |
| `honesty_fallback_token` | The single token a consuming surface renders when upstream truth is missing or unverified. MUST also appear in `vocabulary_tokens` with role `honesty_fallback_token`. |
| `consuming_surface_classes` | Surfaces that MUST consume the row's vocabulary. `help_pane`, `about_pane`, `service_health_pane`, and `docs_browser_pane` MUST appear on every row. |
| `support_export_compatible` | Structural constant `true`: every badge family in the M1 seed MUST be safe for support / export copy without surface-local rewrites. |
| `named_runtime_consumer` | One real consumer (the live shell module, a docs page, support-export copy, or a CI validator) with `consumer_ref`, `consumer_class`, and `consumed_fields`. |
| `example_payload_ref` | Canonical example payload under `fixtures/help/m1_truth_source_examples/` the validation lane reparses end-to-end. |
| `failure_drill` | One named drift the lane can replay under `--force-drill`, naming the `expected_check_id` it MUST reproduce. |

## Row coverage

The M1 seed lands seven rows covering every required badge family:

| Row | Family class | Honesty fallback |
|---|---|---|
| `truth_source.docs_help_source_class` | `docs_help_source_class` | `unknown_source` |
| `truth_source.docs_help_version_match_state` | `docs_help_version_match_state` | `unknown_target_build` |
| `truth_source.docs_help_freshness_class` | `docs_help_freshness_class` | `unverified` |
| `truth_source.client_scope_badge_family` | `client_scope_badge_family` | `degraded_trust` |
| `truth_source.install_mode_class` | `install_mode_class` | `unknown_install_mode` |
| `truth_source.provenance_row_state` | `provenance_row_state` | `not_verified_this_seed` |
| `truth_source.service_health_state` | `service_health_state` | `stale_snapshot` |

Every row binds `help_pane`, `about_pane`, `service_health_pane`, and
`docs_browser_pane` on `consuming_surface_classes` so the four surfaces
read the same vocabulary. The `support_export_card` surface appears on
every row as well so support / export copy quoting the rows is parity-
consistent.

## Consuming-surface parity

The seed's parity floor is the schema's
`required_consuming_surface_class_coverage` field: every row MUST
declare all four of `help_pane`, `about_pane`, `service_health_pane`,
and `docs_browser_pane`. The validation lane fails loudly with
`truth_source.required_consuming_surface_missing` if any row drops one
of the four surfaces. The schema rejects unknown surfaces structurally.

## Honesty-fallback invariant

Every row's `honesty_fallback_token` MUST also appear in
`vocabulary_tokens` with role `honesty_fallback_token`. The validation
lane fails loudly with `truth_source.honesty_fallback_token_missing`
when the field is empty and with
`truth_source.honesty_fallback_token_not_in_vocabulary` when the token
is not in the vocabulary. This is what prevents a surface from rendering
a blank row when upstream truth is missing or unverified.

## Seed-placeholder honesty

The `provenance_row_state` and `service_health_state` rows include the
`seed_placeholder_awaiting_wiring` token with role
`seed_placeholder_token`. The seed is honest about the fact that the
live signature verifier and the live service-health aggregator are
owned by later milestones; the placeholder is the row the chrome
renders in M1 so it cannot fabricate "all green" before the verifier
or aggregator lands. The lane fails loudly with
`truth_source.seed_placeholder_token_required` if either row drops the
placeholder and with `truth_source.seed_placeholder_role_widened` if a
row promotes the placeholder to a live state.

## Degraded-state honesty

The `docs_help_freshness_class` row pairs `degraded_cached`, `stale`,
and `unverified` with the `degraded_state_token` or
`honesty_fallback_token` role. The lane fails loudly with
`truth_source.degraded_state_token_widened` if any of those tokens is
promoted to a live state. This is what prevents a chrome from silently
rendering a stale snapshot as `authoritative_live`.

## Named runtime consumers

Every row names one real runtime consumer that already reads the
vocabulary:

- Three rows (`docs_help_source_class`, `docs_help_version_match_state`,
  `docs_help_freshness_class`) point at
  [`/crates/aureline-shell/src/docs_browser/state.rs`](../../crates/aureline-shell/src/docs_browser/state.rs)
  — the live docs/help-browser projection.
- Four rows (`client_scope_badge_family`, `install_mode_class`,
  `provenance_row_state`, `service_health_state`) point at
  [`/crates/aureline-shell/src/help_about/mod.rs`](../../crates/aureline-shell/src/help_about/mod.rs)
  — the live Help/About/provenance/service-health surface.

[`/docs/help/badge_vocabulary_draft.md`](badge_vocabulary_draft.md) is
the consumer-facing draft that quotes every row verbatim so support /
export copy and docs reviewers read the same vocabulary as the live
shell.

## Failure-drill coverage

The validation lane runs without arguments to assert every row passes
under the structural invariants. Each row carries a typed
`failure_drill` the lane replays under
`--force-drill <row_id>:<drill_id>` to reproduce the named drift:

| Drill | Targets |
|---|---|
| `truth_source_drill.honesty_fallback_token_dropped` | Source class row drops its honesty fallback. |
| `truth_source_drill.version_match_token_count_drifted` | Version-match row injects an unsanctioned token, drifting frozen_token_count. |
| `truth_source_drill.freshness_degraded_pair_widened` | Freshness row widens a degraded token to live. |
| `truth_source_drill.required_consuming_surface_help_pane_dropped` | Client-scope row drops `help_pane` from consuming surfaces. |
| `truth_source_drill.install_mode_unknown_token_dropped` | Install-mode row drops its `unknown_install_mode` fallback. |
| `truth_source_drill.provenance_seed_placeholder_dropped` | Provenance row widens the seed placeholder to live. |
| `truth_source_drill.service_health_seed_placeholder_dropped` | Service-health row drops the seed placeholder entirely. |

Each drill exits 0 only when the runner reproduces the row's declared
`expected_check_id`; un-forced rows are still validated and the lane
fails if any of them drifts.

## How to run the lane

```
python3 tests/help/m1_truth_source_seed_lane/run_m1_truth_source_seed_lane.py --repo-root .
```

Replay one named drill:

```
python3 tests/help/m1_truth_source_seed_lane/run_m1_truth_source_seed_lane.py \
    --repo-root . \
    --force-drill truth_source.docs_help_source_class:truth_source_drill.honesty_fallback_token_dropped
```

## Out of scope

The seed deliberately does **not** own:

- the live signature / attestation / SBOM / advisory verifier (the
  provenance row stays at `seed_placeholder_awaiting_wiring` until the
  verifier lands);
- the live service-health aggregator (the service-health row stays at
  `seed_placeholder_awaiting_wiring` until the aggregator lands);
- the docs / help publishing pipeline, the service-status backend, or
  the incident-communications portal;
- the eventual public-truth audit packet that drift-detects the four
  surfaces against the claim manifest. That packet is owned by
  [`/docs/public_truth/help_about_service_health_audit_packet.md`](../public_truth/help_about_service_health_audit_packet.md);
  this seed is one of the canonical owner artifacts that audit packet
  reads from.
