# Constrained-file and degraded-provider assist-narrowing model

## Release evidence

This artifact documents the one canonical, frozen, export-safe constrained-file and
degraded-provider assist-narrowing model produced by
`crates/aureline-editor/src/m5_constrained_assist/`. It states how every editor
assist micro-surface narrows, downgrades, blocks, or routes elsewhere when file
state or provider certainty means Aureline cannot safely offer the same completion /
hint / hover / refactor behavior it offers on an ordinary source file. Editor,
CLI/headless, support-export, and AI-evidence consumers render this model rather
than inventing per-pane constrained-file behavior.

The model is the constrained-file assist-honesty lane: for every constrained state
and channel it makes the editor truthful about **what is narrowed** (a degraded
verdict that reuses the shared `AssistDegradeClass`), **why** (a closed reason and a
non-empty disabled-state diagnostic, never silently hidden), **what to do next** (a
closed next-safe-action route with a command), and **that it stays reachable** (an
offered cell is always keyboard-reachable). A set of degraded-provider cases proves
the same source-labeled, routed narrowing on otherwise-ordinary files, and a set of
consumer-surface proofs binds the claimed surfaces back to the shared vocabulary.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `ConstrainedAssistModel` | `m5_constrained_assist_model` | `schemas/editor/m5-constrained-assist.schema.json` | 1 |
| `ConstrainedStateProfile` | `m5_constrained_state_profile` | (nested) | 1 |
| `AssistNarrowingCell` | (nested) | (nested) | 1 |
| `DegradedProviderCase` | `m5_degraded_provider_case` | (nested) | 1 |
| `ConsumerSurfaceProof` | `m5_consumer_surface_proof` | (nested) | 1 |

- Model id: `m5-constrained-assist:model:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 8 constrained-file states × 9 assist channels, 3 degraded-provider cases, 5 consumer-surface proofs
- Overall: all 17 invariants hold

## Reused canonical packets

The model does not fork the assist, surface, or provider contracts. Each profile
**embeds** the canonical `AssistSourceDescriptor` (provider id, support, freshness,
locality, scope, degraded state) for provider provenance and **reuses** the
`AssistChannelClass`, `AssistDegradeClass`, and `EditorSurfaceClass` catalogs from
the editor-assist matrix and the language router's `DegradedStateClass` for the
provider axis. The eight constrained-file state classes each **project** a canonical
landed state class — the writable-boundary, generated-artifact, managed-zone,
projection, captured-evidence, partial-index, restricted-mode, and
large-file-posture classes — and document it in `canonical_class_note`; they are not
redefined here.

## Honesty invariants (all must pass)

1. `every_state_resolves_one_cell_per_channel` — each constrained-file state resolves exactly one narrowing cell per assist channel.
2. `narrowed_reasons_are_inspectable` — every channel narrowed below full fidelity carries a non-empty disabled-state diagnostic.
3. `blocked_apply_offers_next_safe_action` — every cell that blocks apply offers a concrete next-safe-action route with a command.
4. `no_silently_hidden_side_effectful_assist` — no apply-capable channel is blocked or unavailable without marking apply blocked and disclosing why.
5. `offered_cells_stay_keyboard_reachable` — every offered (non-blocked) cell stays keyboard-reachable.
6. `large_file_suppresses_semantic_and_apply` — on large-file every semantic and apply-capable channel is suppressed and disclosed.
7. `partial_index_narrows_semantic_to_pending` — on partial-index every semantic channel narrows to a labeled pending state.
8. `generated_and_managed_route_writes_to_source` — generated and managed states block apply and route to open-generator-source / regenerate-from-source.
9. `restricted_routes_writes_to_approval` — the restricted state blocks apply and routes to request-approval.
10. `captured_evidence_is_inspect_only` — captured evidence makes editing channels unavailable and routes to inspect-only while hover and peek still read.
11. `read_only_and_projection_allow_read_block_write` — read-only and projection keep reads full fidelity and block writes with a duplicate / edit-source route.
12. `decoration_truth_preserved_except_large_file` — editing-truth decorations stay full fidelity except in large-file mode.
13. `degraded_provider_cases_source_labeled_not_silent` — every degraded-provider case is source-labeled, routed, and disclosed.
14. `degraded_provider_posture_narrows_assist` — every profile with a degraded provider posture narrows at least one channel.
15. `consumer_surfaces_reuse_shared_vocabulary` — every consumer surface reuses a constrained state and a catalogued degrade class, and its asserted narrowing matches the resolved model.
16. `claimed_consumer_surfaces_present` — the notebook, generated, request-artifact, docs-code, and protected surfaces each prove the shared vocabulary.
17. `every_profile_screen_reader_meaningful` — every constrained-state profile carries a non-empty screen-reader summary.

## State coverage

Generated and pinned in `fixtures/editor/m5-constrained-assist/canonical_model.json`.

| State | Primary route | Apply blocked | Provider posture |
|---|---|---|---|
| read_only_boundary | duplicate_editable_copy | yes | authoritative (live) |
| generated_artifact | open_generator_source | yes | generated-source bridge (warm cached) |
| managed_region | regenerate_from_source | yes | managed-zone bridge (warm cached) |
| projection_view | edit_underlying_source | yes | projection bridge (live) |
| captured_evidence | view_only_no_action | yes | captured-evidence snapshot (advisory) |
| partial_index | wait_for_index | no | language server, indexing (scope narrowed) |
| restricted_mode | request_approval_review | yes | policy schema pack (live) |
| large_file | open_in_full_editor | yes | lexical fallback (unsupported, scope narrowed) |

The **generated** and **managed** states are the worked proof that an apply-capable
channel is shown read-only and routed to its generator; **captured_evidence** that
editing assist is unavailable on an immutable snapshot while reading remains;
**partial_index** that semantics narrow to a labeled pending state without blocking
apply; **restricted_mode** that writes route to staged review; **large_file** that
every semantic and apply channel is suppressed but decorations survive as a labeled
lexical fallback; and **read_only_boundary** / **projection_view** that reads stay
full fidelity while writes route to a duplicate or the underlying source.

## Degraded-provider coverage

| Case | Channel | Verdict | Route |
|---|---|---|---|
| degraded-provider:provider_unavailable | completion | source_labeled_fallback | reconnect_provider |
| degraded-provider:scope_narrowed | hover | pending_partial_index | wait_for_index |
| degraded-provider:stale_awaiting_refresh | signature_help | source_labeled_fallback | reconnect_provider |

## Consumer-surface coverage

| Surface | State | Channel | Verdict | Route |
|---|---|---|---|---|
| notebook_cell | partial_index | completion | pending_partial_index | wait_for_index |
| generated_file | generated_artifact | completion | read_only_no_apply | open_generator_source |
| request_editor (artifact) | captured_evidence | completion | blocked_unavailable | view_only_no_action |
| docs_code_block | projection_view | completion | read_only_no_apply | edit_underlying_source |
| protected_file | restricted_mode | completion | read_only_no_apply | request_approval_review |

## Verification

Emit the canonical model:

```sh
cargo run --bin aureline_m5_constrained_assist
cargo run --bin aureline_m5_constrained_assist -- --lines
```

Run the freeze gate (rebuilds the model and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_constrained_assist_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_constrained_assist
```

## Risks and follow-ups

- **The model is a contract, not a live binding.** The resolved profiles are the
  declared policy; wiring each live assist surface (notebook, generated, request /
  SQL, docs-code, protected, large-file) to render the narrowing and its route is
  incremental follow-up.
- **Postures are illustrative for the corpus.** Each state pins one representative
  provider posture; the live router and writable-boundary manager decide the exact
  posture per file from the same provider arbitration and state classes this model
  reuses.
- **State classes and provider vocabulary are reused, not re-proved here.** The
  canonical writable-boundary, generated-artifact, managed-zone, projection,
  captured-evidence, partial-index, restricted-mode, large-file-posture, and router
  degraded-state contracts remain the source of truth; this model carries their
  refs and labels.
- **Scoped to assist narrowing.** The model deliberately stays out of generic
  file-state redesign and Project Doctor / repair work.
