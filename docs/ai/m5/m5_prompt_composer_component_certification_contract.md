# M5 Prompt-Composer Component Surface Certification (M05-891)

This is the closing capstone of the B104 prompt-composer component lane. Where the freeze
matrix
(`freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`)
froze the nine reusable prompt-composer components, the M05-885..888 primitive lanes
narrowed each one, the M05-889 consumer lane proved they are reusable across the claimed
AI composition consumers, and the M05-890 accessibility / auto-narrowing capstone
certified keyboard / screen-reader / CLI / export parity per family, this lane
**certifies** that the shared component truth holds on every claimed M5 AI composition
surface — and auto-narrows any surface that cannot sustain it.

- **Module:** `crates/aureline-ai/src/certify_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_truth_on_every_claimed_m5_ai_surface`
- **Schema:** `schemas/ai/m5-prompt-composer-component-certification.schema.json`
- **Support export (canonical):** `artifacts/ai/m5/m5-prompt-composer-component-certification/support_export.json`
- **Matrix CSV / report:** same directory (`matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ai/m5/m5-prompt-composer-component-certification/`

## What is certified

The packet is keyed on the claimed **surface** a user composes, attaches, reviews, or
sends an AI request through — **not** on component family or primitive lane. The eight
certified surfaces are:

`inline_composer`, `assistant_panel`, `patch_review`, `branch_agent_queue`,
`docs_help_console`, `companion_app`, `cli_headless`, `support_export`.

Each surface is scored across six truth axes:

1. `visual` — composer mode, scope, route/provider/model, attachment identity,
   freshness/trust/taint, omitted/truncated context, draft locality, and send gate shown
   on the primary surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color/glyph-only.
4. `cli_export` — **always-on**; the surface state is reconstructable as text / JSON /
   Markdown from the same draft identity. This axis must stay certified on every row.
5. `degraded_state` — an offline/local-only draft, an unreachable route, or a stale
   attachment honestly downgrades a `ready_to_send` / `reviewable_composition` claim.
6. `composition_and_send_provenance` — composer mode/scope, route/provider/model,
   attachment identity, freshness/trust/taint, omitted/truncated context, draft locality,
   and the send/review gate stay explicit before send, never inheriting a healthier
   surface's composition truth or masking an unresolved mention, stale attachment,
   over-budget composition, tainted paste, or policy-blocked route as ready-to-send.

## Support-claim ladder

Every surface asserts a `claimed_claim` and is certified down to a `certified_claim` on
this six-tier ladder (strongest first), reused from the M05-890 accessibility capstone:
`ready_to_send` (5) → `reviewable_composition` (4) → `narrowed_composition` (3) →
`local_only_composition` (2) → `unresolved_composition` (1) →
`policy_blocked_composition` (0). Certification may only ever *narrow* a claim (lower its
rank), never raise it.

## The invariant

**A degraded truth axis must produce a visible claim narrowing.** The derived status is
recomputed, never asserted:

- **green** — every axis certified and the claimed tier is delivered.
- **yellow** — an axis is not current, the claim narrows to the weakest supported tier,
  and the reduction binds to that axis with a non-generic label and a frozen downgrade
  trigger.
- **red** — a degraded axis is hidden behind a full claim (undisclosed drift), the
  always-on `cli_export` axis drops, a claim is strengthened, or the narrowing is
  inconsistent. A red surface blocks the release.

Compatibility notes are captured for the paths the spec names — unsupported consumer
scopes, policy-owned route narrowing, offline/local-only fallbacks, stale attachments,
and unresolved mentions — and each yellow surface auto-narrows visibly rather than
inheriting a stronger label from a healthier AI lane.

## Canonical bundle

Every row cites exactly one canonical proof bundle — the frozen prompt-composer component
matrix release proof
(`artifacts/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/support_export.json`) —
plus the M05-889 consumer-adoption support export and the M05-890 accessibility support
export as supporting evidence. The packet is metadata-only; raw draft bodies, pasted
external text, provider tokens, and attachment contents never cross this boundary.

## Seeded certification

Four surfaces deliver their claim (green): `inline_composer`, `assistant_panel`,
`patch_review`, `support_export`. Four auto-narrow a not-current axis (yellow):

- `branch_agent_queue` — unreachable route holds the queued draft offline →
  `local_only_composition` (`draft_locality_masked`, degraded-state axis).
- `docs_help_console` — policy-blocked route → `policy_blocked_composition`
  (`route_or_provider_masked`).
- `companion_app` — mirrored, narrowed attachment scope → `narrowed_composition`
  (`attachment_freshness_masked`).
- `cli_headless` — unresolved mention with no exact target → `unresolved_composition`
  (`mention_left_unresolved`).

No surface hides drift (0 red). All nine frozen component families are certified on at
least one surface.

## Regenerating the artifacts

```
GEN_COMPOSER_CERT_ARTIFACTS=1 cargo test -p aureline-ai \
  certify_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_truth_on_every_claimed_m5_ai_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the checked-in support export
drifts from the seeded builder.
