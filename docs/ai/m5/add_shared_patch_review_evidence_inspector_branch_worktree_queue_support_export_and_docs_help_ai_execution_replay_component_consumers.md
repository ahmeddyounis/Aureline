# M5 AI Execution/Replay-Component Consumer Contract

Row **M05-882** (batch **B103**) is the closing consumer-adoption lane over the
frozen M5 AI execution/replay component matrix
([`freeze_the_m5_ai_action_state_banner_..._and_agent_status_component_matrix`](freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md)).
Five sibling `implement_*` / `ship_*` lanes narrowed the eight frozen families into
working primitives; this lane proves those families are reusable **components** — not
one assistant panel plus a few admin pages — by binding every claimed M5
execution/replay consumer to the same canonical component schemas and the one shared
descriptor vocabulary.

## Consumers (`M5AiExecutionReplayConsumer`)

| Consumer | Adopts |
| --- | --- |
| `patch_review` | action-state banner, approval sheet, tool-call timeline, replay/rerun sheet |
| `evidence_inspector` | connector row, run-history row, replay sheet, local-model card |
| `branch_worktree_queue` | agent-status card, action-state banner, approval sheet, run-history row |
| `support_export` | run-history row, replay sheet, agent-status card, tool-call timeline |
| `docs_help` | connector row, action-state banner, agent-status card, local-model card |

`docs_help` is the surface the acceptance criteria single out (`is_docs_or_help`):
every family it adopts must point at the canonical component schema so its prose can
never drift from the product truth.

## Component families (`M5AiSharedComponent`) → canonical primitive

Each family maps to exactly one narrowed primitive's canonical schema / doc /
support-export artifact. A consumer that adopts a family **references** those
canonical refs; it never re-words the facts in local prose.

| Family | Owning primitive lane |
| --- | --- |
| `ai_action_state_banner` | action-state / boundary-blocked banner |
| `connector_detail_row`, `local_model_pack_card` | connector detail row / local-model pack card |
| `approval_sheet`, `tool_call_timeline_row` | high-friction approval sheet / tool-call timeline row |
| `run_history_row` | run-history row / approval-timeline entry / evidence-export summary |
| `replay_review`, `agent_status` | rerun-review sheet / incomplete-replay banner / agent-status card |

Every family is adopted by at least two distinct consumers (`validate_family_reuse`).

## Shared descriptor vocabulary (`M5AiReplayDescriptor`, all required)

`route`, `approval_gate`, `checkpoint_lineage`, `replay_completeness` — the track
invariant that route/provider/model, approval, checkpoint lineage, and replay
completeness stay explicit on every surface. A binding that drops one is rejected.

## Auto-narrowing on weakened replayability

`resolve_replay_binding` derives the claim-parity state from the replay-health mode:

| `M5AiReplayHealth` | narrowing reason | recovery action |
| --- | --- | --- |
| `full_replay` | — (claims preserved, **no** banner) | — |
| `route_provider_model_drift` | `route_provider_model_drift` | `reroute_to_declared_provider` |
| `missing_connector_output` | `missing_connector_output` | `reattach_connector_evidence` |
| `redaction_fenced` | `redaction_fence` | `replay_within_redaction_scope` |
| `stale_approval` | `stale_approval` | `renew_approval_then_rerun` |

Any weakened mode emits a self-contained `M5AiAutoNarrowBanner` naming the exact
reason, the preserved descriptors, the export caveats, and the recovery action —
never a generic "degraded" note. The descriptor vocabulary stays intact under the
narrowing, so missing evidence or drift narrows the claim **visibly** instead of
inheriting full replay/resume language from a healthier run.

## Invariants enforced by `validate`

- **Canonical reference** — each binding's schema/artifact ref equals the family's
  canonical ref and `references_canonical_not_local_prose` is `true`
  (`CanonicalRefMismatch`).
- **Family reuse** — every family adopted by ≥2 consumers
  (`ComponentFamilyReuseUnproven`).
- **Narrowing disclosure** — at least one worked binding proves a narrowed rendering
  with a self-contained banner (`NarrowingDisclosureUnproven`); at least one proves a
  full-replay rendering with preserved parity and no banner (`ScopePreservedUnproven`).
- **Docs/help reference** — every family a docs/help consumer adopts references the
  canonical schema (`DocsHelpReferenceMissing`).
- **Mandatory anatomy / export / descriptors**, accessibility route
  (`keyboard_focusable`), consumer surfaces, downgrade triggers, worked-binding
  self-consistency, stable-consumer proof refs, and four per-row hard invariants
  (`rewords_claims_per_surface`, `invents_new_execution_grammar`,
  `drops_route_or_approval_when_narrowed`, `hides_drift_reason_or_takeover_path`, all
  MUST be `false`).
- **Governance / projection / proof-freshness / release posture** blocks, plus a
  raw-material export guard (no `://`, tokens, or credentials in the packet).

## Artifacts

- Boundary schema: [`schemas/ai/m5-ai-execution-replay-component-consumer.schema.json`](../../../schemas/ai/m5-ai-execution-replay-component-consumer.schema.json)
- Support export / matrix CSV / report: `artifacts/ai/m5/m5-ai-execution-replay-component-consumer-proof/`
- Narrowed fixtures: `fixtures/ai/m5/m5-ai-execution-replay-component-consumers/`

The seed builders in `seed.rs` are the single producer of the checked-in export and
fixtures; the headless emitter
(`cargo run -p aureline-ai --bin aureline_ai_execution_replay_component_consumers`)
mints them and `checked_support_export_validates_and_matches_seed` asserts the disk
copy never drifts from the in-code matrix.
