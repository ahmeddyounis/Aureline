# M5 prompt-composer-header and context-attachment-pill primitive contract

Task: **M05-885** — Implement prompt-composer headers and context-attachment pills with
mode / scope / route / budget / freshness / trust truth across the claimed M5 AI
composition surfaces.

This lane narrows the `prompt_composer_header` and `context_attachment_pill` families from
the frozen [prompt-composer component matrix](./freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md)
(M05-884) into two reusable primitives: a header resolver, an attachment-pill resolver, and
one shared parity matrix. A user can tell — from the header or the pill alone — what mode,
scope, and route a request composes under, what budget band applies, whether the route
stays local-only or is blocked, and, for every attached object, its exact identity, its
freshness / trust state, and the remove / open behavior available before send.

## Primitives

- Header resolver: `resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput) -> Result<M5ResolvedPromptComposerHeader, M5PromptComposerHeaderResolutionError>`.
- Attachment-pill resolver: `resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput) -> Result<M5ResolvedContextAttachmentPill, M5ContextAttachmentPillResolutionError>`.
- Parity matrix packet: `M5PromptComposerHeaderPillPacket`, one row per claimed composition
  consumer, each carrying worked header and pill resolution cases.

### Header posture ladder (blocking-first)

1. `route_blocked` → **route_blocked** — a policy-blocked route never reads as ready.
2. `hard_blocked` budget → **budget_blocked**.
3. `review_first` mode → **review_before_send**.
4. `near_limit` / `over_budget` / `truncation_pending` budget → **budget_constrained**
   (still sendable).
5. route stays on the local device (`local_model`) → **local_only_composing**.
6. otherwise → **ready_composing**.

The header always records `is_sendable` (false only when route- or budget-blocked),
`route_stays_local` / `route_leaves_shell`, and `requires_review_before_send` (true in
`review_first` mode), so a local-only route and a blocked route are never hidden.

### Pill posture ladder (honesty-first)

1. `tainted_external` trust → **tainted**.
2. not `in_scope`, or `out_of_scope` trust → **out_of_scope**.
3. `unverified_source` trust → **unverified**.
4. `is_stale`, or `trusted_stale` trust → **stale**.
5. `redacted_scope` trust → **redacted**.
6. otherwise (`trusted_fresh`) → **fresh_trusted**.

The pill preserves the exact object identity, is openable only when the source has not been
removed, always offers **remove** before send, and offers **refresh** / **review_trust** /
**reveal_scope** follow-ups matched to the posture.

### Bounded actions

| Condition | Action offered |
| --- | --- |
| source not removed | `open` |
| always | `remove` |
| stale posture / `is_stale` | `refresh` |
| tainted / unverified posture | `review_trust` |
| out-of-scope / redacted posture | `reveal_scope` |

### Resolver errors

- Header: `empty_provider_model_label`, `forbidden_header_material`.
- Pill: `empty_attachment_id`, `empty_attachment_label`, `stale_attachment_without_reason`
  (a stale attachment must name why), `forbidden_attachment_material`.

## Claimed consumer surfaces

`inline_assistant`, `side_panel`, `patch_draft`, `handoff_surface`, and
`cli_support_export`. Every row reuses the shared header and pill anatomy, the same modes /
scopes / routes / budget postures / kinds / trust states / postures / bounded actions, the
same mandatory export fields, and a non-visual accessibility route, so the mode / route /
budget / freshness / trust vocabulary stays identical across inline, side-panel,
patch-draft, handoff, and CLI / support exports.

## Hard invariants (per row, all must be false)

- `masks_mode_or_route`
- `hides_attachment_freshness_or_trust`
- `invents_private_composer_grammar`
- `bypasses_review_before_send`

## Acceptance-criterion lints

- `header_sendability_coverage_unproven` — at least one header resolution is sendable and at
  least one is non-sendable (route- or budget-blocked).
- `header_local_only_disclosure_unproven` — at least one header resolution proves a route
  that stays on the local device.
- `attachment_identity_preservation_unproven` — every pill resolution preserves its exact
  object identity.
- `attachment_trust_coverage_unproven` — at least one pill resolution is fresh-trusted and
  at least one needs attention (tainted / unverified / stale / out-of-scope).
- `attachment_open_remove_coverage_unproven` — at least one pill resolution is openable with
  an `open` action and at least one has a removed source but still offers `remove`.

## Reused vocabulary (frozen in M05-884)

`M5ComposerMode`, `M5ComposerScope`, `M5ComposerRouteClass`, `M5AttachmentKind`,
`M5AttachmentTrustState`, `M5BudgetPosture`, `M5StalenessReason`, `M5ComposerSurfaceFamily`,
`M5ComposerDeploymentLine`, `M5ComposerConsumerSurface`, `M5ComposerAccessibilityRoute`,
`M5ComposerQualificationClass`, and `M5ComposerDowngradeTrigger`.

## Artifacts

- Boundary schema: `schemas/ai/m5-prompt-composer-header-and-context-attachment-pill.schema.json`.
- Support export (canonical): `artifacts/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/support_export.json`.
- Matrix CSV and Markdown report alongside the support export.
- Narrowed fixtures under `fixtures/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/`.
- Headless emitter: `cargo run -p aureline-ai --bin aureline_ai_prompt_composer_header_context_attachment_pill_primitive -- <support-export|report|csv|validate|fixture-...>`.
