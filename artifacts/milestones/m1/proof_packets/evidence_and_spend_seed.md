# Proof packet: AI evidence-packet seed and route/spend truth strip

Purpose: anchor proof captures for the M1 bounded prototype wedge that
lands the first-pass AI evidence packet, the visible route/spend truth
strip, and the export-safe run-metadata action on the launch AI wedge.
The seed is read-only for mutation, never dispatches a model, and
projects its addressable axes through the upstream
[`aureline_ai::ComposerDraft`] record rather than forking AI-only truth.

Reviewer landing page:
[`docs/ai/m1_evidence_and_spend_seed.md`](../../../../docs/ai/m1_evidence_and_spend_seed.md).

Canonical sources:

- Schema:
  - `schemas/ai/m1_evidence_packet.schema.json` — cross-tool boundary
    schema for the `ai_evidence_seed_record` and the
    `ai_truth_strip_snapshot_record`, closed vocabularies for
    prototype label, run-state class, provider / route / dispatch-
    target class, local-vs-remote path class, spend-posture class,
    claim-limit row, and invariant-violation token.
- Crate (live wedge): `crates/aureline-shell/`
  - `src/ai_truth_strip/mod.rs` — `AiEvidencePacketSeedRecord`,
    `RouteSpendTruthStripRow`, `AiTruthStripSnapshot`,
    `AiRouteSpendPosture`, the closed
    `AiTruthStripInvariantViolation` set, deterministic plaintext
    render, and the export-safe `export_safe_run_metadata` action.
  - `src/ai_truth_strip/tests.rs` — unit and fixture-driven coverage.
- Crate (upstream truth source): `crates/aureline-ai/`
  - `src/composer/mod.rs` — `ComposerDraft`, route-placeholder
    vocabulary, and `BlockReason` enum the packet quotes verbatim.
- Reviewer landing page:
  `docs/ai/m1_evidence_and_spend_seed.md`
- Fixtures:
  - `fixtures/ai/m1_evidence_and_spend_seed_cases/protected_walk_local_no_dispatch.json`
  - `fixtures/ai/m1_evidence_and_spend_seed_cases/failure_drill_alternate_route_and_spend.json`

Upstream contracts the seed projects against (without forking):

- `docs/ai/spend_and_route_receipt_contract.md` /
  `schemas/ai/provider_route_receipt.schema.json` /
  `schemas/ai/spend_receipt.schema.json` — the broader provider-route /
  spend-receipt vocabularies. The M1 seed names what is *not* yet
  captured and surfaces typed claim-limit rows so a downstream surface
  cannot imply broad provider / billing / quota depth.
- `docs/ai/context_assembly_contract.md` /
  `schemas/ai/evidence_packet.schema.json` — the broader AI evidence-
  packet record shape that lands later milestones. The M1 seed covers
  a small, honest subset and grows additively without forking.
- `docs/ai/prompt_composer_contract.md` /
  `crates/aureline-ai/src/composer/` — the upstream composer draft the
  packet reads verbatim.

## Protected walk

Open the launch AI wedge against a draft with one resolved workspace
mention and one trusted workspace-slice attachment. Confirm:

- the prototype label chip reads `m1_prototype_evidence_and_spend_seed`
  and the chrome quotes the human-readable label "M1 prototype — AI
  evidence packet and route/spend truth strip, no model dispatch";
- the evidence packet pins
  `provider_class = disabled_no_provider_in_m1_seed`,
  `route_path_class = denied_by_policy_in_m1_seed`,
  `dispatch_target_class = disabled_no_dispatch_in_m1_seed`,
  `local_or_remote_path_class = local_no_dispatch`,
  `spend_posture_class = no_spend_in_m1_seed`,
  `run_state_class = dispatch_disabled_in_m1_seed`, and
  `draft_state_token = dispatch_disabled_in_m1_seed`;
- the truth strip renders the eight canonical rows in canonical order:
  `provider`, `route`, `dispatch_target`, `local_or_remote_path`,
  `spend_posture`, `run_state`, `context_summary`, `build_identity`;
- the claim-limit list renders in canonical order
  (`single_bounded_wedge_only`, `no_live_model_dispatch`,
  `no_billing_or_quota_tracking`, `no_raw_secrets_or_provider_urls`);
- the result-lineage section carries only the always-on
  `policy_blocked_route -> route:placeholder` row;
- the invariant section reads "(all clear)";
- `export_safe_run_metadata` emits a deterministic JSON payload that
  carries the typed tokens and omits the raw intent text, the raw
  mention labels, and the raw attachment labels.

Evidence:
`crates/aureline-shell/src/ai_truth_strip/tests.rs::protected_walk_local_no_dispatch_renders_clean_packet`,
`crates/aureline-shell/src/ai_truth_strip/tests.rs::truth_strip_renders_canonical_row_order`,
`crates/aureline-shell/src/ai_truth_strip/tests.rs::claim_limits_render_in_canonical_order`,
`crates/aureline-shell/src/ai_truth_strip/tests.rs::result_lineage_carries_one_row_per_block_reason`,
`crates/aureline-shell/src/ai_truth_strip/tests.rs::export_safe_run_metadata_omits_raw_intent_and_carries_typed_tokens`,
`crates/aureline-shell/src/ai_truth_strip/tests.rs::render_plaintext_quotes_every_section_in_stable_order`,
`crates/aureline-shell/src/ai_truth_strip/tests.rs::fixture_protected_walk_local_no_dispatch_replays_into_the_wedge`,
`fixtures/ai/m1_evidence_and_spend_seed_cases/protected_walk_local_no_dispatch.json`.

## Failure drill

Route an AI run through a different (mocked) provider with a different
spend class and confirm the strip and the packet expose the new route
and cost posture verbatim. The fixture pins the upstream
`aureline_ai::RoutePlaceholder` to the
`RoutePlaceholder::mocked_for_fixtures()` variant (provider class
`mocked_test_provider`, route path class `offline_cached_only`) and
the truth-strip posture to the
`AiRouteSpendPosture::mocked_alternative_for_failure_drill()` variant
(`local_or_remote_path_class = remote_mocked_for_fixtures`,
`spend_posture_class = mocked_spend_for_fixtures`,
`run_state_class = preview_pre_dispatch_mocked`). The truth strip MUST
surface those alternative tokens distinctly from the live defaults.

Evidence:
`crates/aureline-shell/src/ai_truth_strip/tests.rs::failure_drill_alternate_route_surfaces_distinct_tokens`,
`crates/aureline-shell/src/ai_truth_strip/tests.rs::fixture_failure_drill_alternate_route_and_spend_replays_into_the_wedge`,
`fixtures/ai/m1_evidence_and_spend_seed_cases/failure_drill_alternate_route_and_spend.json`.

Adjacent failure drills covered by the same suite:

- `crates/aureline-shell/src/ai_truth_strip/tests.rs::mocked_spend_against_live_run_state_surfaces_typed_invariant`
  — a buggy caller that swaps in the mocked spend posture but keeps
  the live run-state class surfaces both
  `route_and_path_class_disagree` and
  `spend_posture_contradicts_route` on the invariant set.
- `crates/aureline-shell/src/ai_truth_strip/tests.rs::unresolved_mention_routes_into_packet_lineage`
  — the result lineage gains an
  `unresolved_mention -> mention:<id>` row and the run-state class
  bumps to `blocked_pending_resolution`.
- `crates/aureline-shell/src/ai_truth_strip/tests.rs::tainted_attachment_routes_into_packet_lineage_and_counts`
  — a tainted attachment surfaces a typed
  `tainted_attachment_outside_fenced_section -> attachment:<id>` row,
  increments `tainted_attachment_count`, and bumps the run-state to
  `blocked_pending_resolution`.
- `crates/aureline-shell/src/ai_truth_strip/tests.rs::missing_evidence_packet_id_surfaces_typed_invariant`
  and `missing_exact_build_identity_ref_surfaces_typed_invariant`
  — empty caller-supplied identifiers surface the typed
  `evidence_packet_id_missing` / `exact_build_identity_ref_missing`
  invariants instead of letting the chrome render a misleading packet.

## Validation command

```
cargo test -p aureline-shell --lib ai_truth_strip
```

## Evidence storage

- Schema: `schemas/ai/m1_evidence_packet.schema.json`
- Crate source: `crates/aureline-shell/src/ai_truth_strip/`
- Reviewer doc: `docs/ai/m1_evidence_and_spend_seed.md`
- Fixtures: `fixtures/ai/m1_evidence_and_spend_seed_cases/`
- Tests: `crates/aureline-shell/src/ai_truth_strip/tests.rs`
