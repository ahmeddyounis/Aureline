# M5 mention-resolver and slash-command-row primitive contract

Task: **M05-886** — Ship mention resolvers and slash-command rows with exact-target
previews, ambiguity review, stable command IDs, and disabled-state explanations across the
claimed M5 composer surfaces.

This lane narrows the `mention_resolver` and `slash_command_row` families from the frozen
[prompt-composer component matrix](./freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md)
(M05-884) into two reusable primitives: a mention resolver, a slash-command-row resolver,
and one shared parity matrix. A user can tell — from the mention row or the command row
alone — which stable object an `@`-mention actually binds to, whether that binding is
unique, pinned, ambiguous, unresolved, out of scope, or deferred, what the exact target
preview is, and, for every slash command, its stable command id, capability class, help
path, availability posture, disabled-state explanation, and approval semantics — the same
truth the command graph projects to the palette, automation, and CLI surfaces.

## Primitives

- Mention resolver: `resolve_mention_resolver(&M5MentionResolverResolutionInput) -> Result<M5ResolvedMentionResolver, M5MentionResolverResolutionError>`.
- Slash-command-row resolver: `resolve_slash_command_row(&M5SlashCommandRowResolutionInput) -> Result<M5ResolvedSlashCommandRow, M5SlashCommandRowResolutionError>`.
- Parity matrix packet: `M5MentionSlashCommandPacket`, one row per claimed composer
  consumer, each carrying worked mention and command resolution cases.

### Mention-resolution ladder (exact-stable-first, never silently binds)

1. not `in_scope` → **out_of_scope_denied** (blocks send; offers reveal-scope).
2. `candidate_count == 0` → **unresolved_missing** (blocks send; needs explicit review).
3. `deferred` → **deferred_pending** (blocks send).
4. `has_exact_stable_target` → **resolved_pinned** when the target is pinned, else
   **resolved_unique** — an exact stable object is preferred even over several candidates.
5. `candidate_count == 1` → **resolved_unique** (single remaining candidate).
6. otherwise → **ambiguous_candidates** (blocks send; offers choose-candidate review).

The mention row always records `is_bound` (true only for resolved-unique / resolved-pinned),
`blocks_send` (true whenever not bound), `needs_explicit_review` (true for ambiguous /
unresolved), and `preserves_exact_target_preview`, and it always preserves the scope note.
A bound mention that does not carry both its target id and exact-target preview is a
`bound_mention_without_target` error, so a resolved mention always shows its exact target
before send. Remove is always offered.

### Slash-command-row posture ladder (blocking-first, approval-escalating)

1. `unknown_command` → **unknown_rejected** (blocked).
2. `policy_hidden` → **policy_hidden** (blocked; requires a disabled reason).
3. `disabled_unmet_precondition` → **disabled_explained** (requires a disabled reason).
4. `requires_approval` state → **approval_gated**.
5. `deprecated_aliased` → **deprecated_redirect** (invocable; requires a canonical alias),
   escalated to **approval_gated** when `requires_approval` is set.
6. `available` → **ready_invocable**, escalated to **approval_gated** when
   `requires_approval` is set.

The command row always records `is_invocable` (ready or deprecated-redirect), `is_blocked`
(policy-hidden or unknown), `requires_approval_before_run`, and `explains_disabled_state`.
A disabled or policy-hidden row without an explanation is a `disabled_without_explanation`
error; a deprecated command without a canonical target is a
`deprecated_without_canonical_target` error. Open-help is always offered so the help path is
never hidden.

## Parity matrix

The packet binds one row per claimed composer consumer:

- **Inline composer** — the AI composition surface.
- **Command palette** — the non-AI palette the same commands are reached from.
- **Automation recipe** — the automation surface.
- **CLI / headless** — the CLI / headless surface.
- **Support export** — the export a support reviewer reconstructs mention / command truth
  from.

Every row carries the shared mention and slash anatomy, the mention resolutions, the
slash-command states, the capability classes, the row postures, the bounded actions, the
export fields, and the non-visual accessibility routes, so the mention and slash-command
grammar stays identical across AI composition, palette, automation, CLI / headless, and
support exports rather than drifting into a separate AI-only grammar.

Each row also declares four hard invariants, all `false`: it never masks a command's
identity or capability (`masks_command_identity_or_capability`), never hides a mention's
resolution or ambiguity (`hides_mention_resolution_or_ambiguity`), never invents a private
command grammar (`invents_private_command_grammar`), and never bypasses the ambiguity or
approval gate (`bypasses_ambiguity_or_approval_gate`).

## Acceptance-criteria lints

`M5MentionSlashCommandPacket::validate` enforces, beyond the structural and vocabulary
checks:

- `mention_bind_coverage_unproven` — some worked mention binds and some blocks send.
- `mention_ambiguity_review_unproven` — some ambiguous mention blocks send with explicit
  review instead of binding to the wrong target.
- `mention_target_preview_unproven` — every bound mention preserves its exact-target
  preview.
- `slash_disabled_explanation_unproven` — some disabled command carries its explanation.
- `slash_approval_availability_coverage_unproven` — some command is approval-gated and some
  is ready-invocable, matching the command graph's approval semantics and availability.

## Boundary and provenance

The boundary schema is
[`schemas/ai/m5-mention-resolver-and-slash-command-row.schema.json`](../../../schemas/ai/m5-mention-resolver-and-slash-command-row.schema.json).
Rows reuse the stable command id, capability class, help path, availability, and approval
semantics from the command-graph
[`command_descriptor`](../../../schemas/commands/command_descriptor.schema.json) contract and
the mention provenance from the
[prompt-composer draft / mention-provenance](./ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts.md)
records, so AI composition uses the same object and command language as the palette,
automation, and CLI surfaces.

Raw prompts, mention query bodies, command bodies, raw argument values, raw paths, raw URLs,
credentials, and private endpoints never cross this boundary; every command id, mention
token, scope note, help path, and target label is carried only as an opaque, export-safe
representation.

## Artifacts

- Support export: `artifacts/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/support_export.json`
- Matrix CSV: `.../matrix.csv`
- Markdown report: `artifacts/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces.md`
- Narrowed fixtures: `fixtures/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/`

All are minted only by the headless emitter
`aureline_ai_mention_resolver_slash_command_row_primitive` from the seed builders, so the
in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.
