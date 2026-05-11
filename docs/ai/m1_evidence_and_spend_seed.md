# M1 AI evidence-packet seed and route/spend truth-strip wedge

Reviewer-facing landing page for the bounded launch AI wedge that lands
the first-pass **AI evidence packet**, the **route/spend truth strip**,
and the **export-safe run metadata** action on the same launch row the
AI composer / context inspector already owns.

The seed is intentionally a **bounded prototype**:

- it carries no mutation authority and never dispatches a model;
- the truth strip never mints a billable spend amount and never quotes
  a raw provider URL, raw token count, raw cost amount, or raw
  credential body;
- every row carries a stable token the chrome quotes verbatim and a
  typed honesty marker rather than a free-form warning;
- the packet projects against the upstream composer draft (M01-116) and
  does not fork mention / attachment / route-placeholder vocabulary.

## Reuse, not reinvention

The seed projects against four upstream sources and does not fork their
vocabularies:

- [`crates/aureline-ai/src/composer/mod.rs`](../../crates/aureline-ai/src/composer/mod.rs)
  supplies the canonical [`ComposerDraft`] record plus the
  [`ProviderClass`], [`RoutePathClass`], [`DispatchTargetClass`],
  [`ComposerDraftState`], and [`BlockReason`] vocabularies. The
  evidence packet quotes those tokens verbatim.
- [`crates/aureline-shell/src/ai_context_inspector/mod.rs`](../../crates/aureline-shell/src/ai_context_inspector/mod.rs)
  is the sibling launch-wedge surface that renders mention /
  attachment / slash-command / route-placeholder rows. The truth strip
  surfaces alongside the inspector; both project the same draft and
  quote the same `composer_draft_id` / `composer_session_id` /
  `request_workspace_id`.
- [`docs/ai/spend_and_route_receipt_contract.md`](./spend_and_route_receipt_contract.md)
  and [`docs/ai/context_assembly_contract.md`](./context_assembly_contract.md)
  freeze the broader provider / route / spend / outcome vocabularies.
  The M1 seed covers a small, honest subset and grows additively
  without forking truth.
- [`crates/aureline-build-info/src/lib.rs`](../../crates/aureline-build-info/src/lib.rs)
  supplies the exact-build identity ref the packet pins. The wedge
  takes the ref as a caller-supplied string so unit tests can pass a
  fixture-stable value.

## What the seed owns

| Artifact | Role |
| --- | --- |
| `schemas/ai/m1_evidence_packet.schema.json` | Cross-tool boundary schema for the `ai_evidence_seed_record` and the `ai_truth_strip_snapshot_record`. Closed vocabularies for prototype label, run-state class, provider / route / dispatch-target class, local-vs-remote path class, spend-posture class, claim-limit row, and invariant-violation token. |
| `crates/aureline-shell/src/ai_truth_strip/mod.rs` | The live shell wedge. Mints one `AiEvidencePacketSeedRecord` plus the typed `RouteSpendTruthStripRow` list and the closed `AiTruthStripInvariantViolation` set. Exposes `AiTruthStripSnapshot::project`, `project_launch_wedge`, `export_safe_run_metadata`, and `render_plaintext`. |
| `crates/aureline-shell/src/ai_truth_strip/tests.rs` | Unit and fixture-driven tests covering the protected walk, the failure drill, and the named invariants. |
| `fixtures/ai/m1_evidence_and_spend_seed_cases/` | Replay fixtures owned by this lane and quoted by the proof packet. |
| `docs/ai/m1_evidence_and_spend_seed.md` | This reviewer landing page. |
| `artifacts/milestones/m1/proof_packets/evidence_and_spend_seed.md` | Proof packet anchored from the artifact index. |

## What the seed deliberately does **not** own

- Live model routing, dispatch, or any billable spend. The packet pins
  `provider_class = disabled_no_provider_in_m1_seed` /
  `route_path_class = denied_by_policy_in_m1_seed` /
  `dispatch_target_class = disabled_no_dispatch_in_m1_seed` /
  `local_or_remote_path_class = local_no_dispatch` /
  `spend_posture_class = no_spend_in_m1_seed` on every live shell row.
- Billing, chargeback, quota-control planes, or long-term AI run
  retention/analytics. Those land their own milestones; the M1 seed
  surfaces the `no_billing_or_quota_tracking` claim-limit row instead
  of fabricating them.
- The full `ai_evidence_packet_record` / `provider_route_receipt_record` /
  `ai_spend_receipt_record` shape frozen in the broader AI contracts.
  Those records land later milestones; the M1 seed is the cross-tool
  boundary that names what is *not* yet captured.
- Raw URLs, raw provider payloads, raw cost amounts in any currency,
  raw token counts, raw user identifiers, raw billing-account ids, and
  raw credential bodies. The packet is export-safe and never quotes
  those.

## Claim limits the chrome MUST quote

The snapshot always renders the canonical four-row claim-limit list in
this exact order:

1. `single_bounded_wedge_only`
2. `no_live_model_dispatch`
3. `no_billing_or_quota_tracking`
4. `no_raw_secrets_or_provider_urls`

Dropping a row, reordering them, or duplicating a row surfaces the typed
`claim_limits_missing_or_out_of_order` invariant.

## Protected walk

1. Open the launch AI wedge surface against a draft with one resolved
   workspace mention (`@editor.find`) and one trusted workspace-slice
   attachment.
2. The chrome mints one `AiEvidencePacketSeedRecord` with:
   - `prototype_label_token = m1_prototype_evidence_and_spend_seed`,
   - `provider_class = disabled_no_provider_in_m1_seed`,
   - `route_path_class = denied_by_policy_in_m1_seed`,
   - `dispatch_target_class = disabled_no_dispatch_in_m1_seed`,
   - `local_or_remote_path_class = local_no_dispatch`,
   - `spend_posture_class = no_spend_in_m1_seed`,
   - `run_state_class = dispatch_disabled_in_m1_seed`,
   - `draft_state_token = dispatch_disabled_in_m1_seed`.
3. The truth strip renders the eight canonical rows in this exact
   order: `provider`, `route`, `dispatch_target`,
   `local_or_remote_path`, `spend_posture`, `run_state`,
   `context_summary`, `build_identity`. The chrome quotes each
   `value_token` verbatim alongside the human-readable `value_label`.
4. The result-lineage section carries only the always-on
   `policy_blocked_route -> route:placeholder` row.
5. The claim-limit section renders the canonical four-row list in
   canonical order.
6. The invariant section reads "(all clear)".
7. The "export run metadata" action calls
   `AiTruthStripSnapshot::export_safe_run_metadata`, which emits a
   deterministic JSON payload containing only the typed packet fields.
   The export omits the raw intent text, the raw mention labels, and
   the raw attachment labels (those live on the upstream draft, not on
   the packet).

Evidence:
[`crates/aureline-shell/src/ai_truth_strip/tests.rs::protected_walk_local_no_dispatch_renders_clean_packet`](../../crates/aureline-shell/src/ai_truth_strip/tests.rs),
`truth_strip_renders_canonical_row_order`,
`claim_limits_render_in_canonical_order`,
`result_lineage_carries_one_row_per_block_reason`,
`export_safe_run_metadata_omits_raw_intent_and_carries_typed_tokens`,
`fixture_protected_walk_local_no_dispatch_replays_into_the_wedge`,
[`fixtures/ai/m1_evidence_and_spend_seed_cases/protected_walk_local_no_dispatch.json`](../../fixtures/ai/m1_evidence_and_spend_seed_cases/protected_walk_local_no_dispatch.json).

## Failure drill

Route an AI run through a different (mocked) provider with a different
spend class and confirm the strip and the packet expose the new route
and cost posture verbatim:

1. The fixture pins the upstream
   [`aureline_ai::RoutePlaceholder`] to the
   `RoutePlaceholder::mocked_for_fixtures()` variant (provider class
   `mocked_test_provider`, route path class `offline_cached_only`).
2. The fixture passes
   `AiRouteSpendPosture::mocked_alternative_for_failure_drill()` to
   the wedge — that is, `local_or_remote_path_class =
   remote_mocked_for_fixtures` plus `spend_posture_class =
   mocked_spend_for_fixtures` plus `run_state_class =
   preview_pre_dispatch_mocked`.
3. The truth strip surfaces those alternative tokens verbatim on the
   `provider`, `local_or_remote_path`, `spend_posture`, and `run_state`
   rows — distinctly from the live defaults — proving the chrome
   exposes the new route/cost posture instead of hiding it behind a
   generic "AI used" chip.
4. The snapshot still reports `has_invariant_violations = false`
   because the run-state class is one of the fixture-only `*_mocked`
   variants. The chrome's mocked-only chip ensures the user reads the
   strip as fixture / replay, not as a live dispatch.

Evidence:
`crates/aureline-shell/src/ai_truth_strip/tests.rs::failure_drill_alternate_route_surfaces_distinct_tokens`,
`fixture_failure_drill_alternate_route_and_spend_replays_into_the_wedge`,
[`fixtures/ai/m1_evidence_and_spend_seed_cases/failure_drill_alternate_route_and_spend.json`](../../fixtures/ai/m1_evidence_and_spend_seed_cases/failure_drill_alternate_route_and_spend.json).

Adjacent failure drills covered by the same suite:

- An unresolved mention bumps the run-state to
  `blocked_pending_resolution` and adds an
  `unresolved_mention -> mention:<id>` row to the result lineage
  (`unresolved_mention_routes_into_packet_lineage`).
- A tainted attachment surfaces a
  `tainted_attachment_outside_fenced_section -> attachment:<id>`
  row, bumps the run-state to `blocked_pending_resolution`, and
  increments the `tainted_attachment_count` in the context summary
  (`tainted_attachment_routes_into_packet_lineage_and_counts`).
- A buggy caller that swaps in the mocked spend posture but keeps the
  live run-state class surfaces both
  `route_and_path_class_disagree` and `spend_posture_contradicts_route`
  on the invariant set
  (`mocked_spend_against_live_run_state_surfaces_typed_invariant`).
- An empty `evidence_packet_id` or empty `exact_build_identity_ref`
  surfaces the typed `evidence_packet_id_missing` /
  `exact_build_identity_ref_missing` invariants.

## Validation command

```
cargo test -p aureline-shell --lib ai_truth_strip
```

## Closure recipe

The bounded wedge is live on the launch AI row; the truth strip is
visible alongside the AI context inspector; the evidence packet is
export-safe and never quotes raw secrets; the typed invariant set keeps
the chrome from minting a misleading packet; the fixture-driven failure
drill catches regressions without widening scope.

## Out of scope

Billing, chargeback, quota-control planes, and long-term AI run
retention or analytics systems. Live model routing, autonomous apply,
and the broad provider-route / spend-receipt / evidence-packet record
shapes frozen in the upstream AI contracts. The M1 seed's job is to
make the launch AI lane accountable — to attach a stub evidence packet,
a visible route/spend strip, and export-safe run metadata even before
M1 ships deep AI behavior — not to imply broad provider, billing, or
admin depth.
