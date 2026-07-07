# M5 Prompt-Composer-Component Consumer Contract

Row **M05-889** (batch **B104**) is the closing consumer-adoption lane over the
frozen M5 prompt-composer component matrix
([`freeze_the_m5_prompt_composer_header_..._and_draft_state_component_matrix`](freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md)).
Four sibling `implement_*` / `ship_*` lanes narrowed the nine frozen families into
working primitives; this lane proves those families are reusable **components** — not
one inline composer plus a few isolated design objects — by binding every claimed M5
composer consumer to the same canonical component schemas and the one shared
descriptor vocabulary.

## Consumers (`M5ComposerComponentConsumer`)

| Consumer | Adopts |
| --- | --- |
| `inline_panel` | header, attachment pill, mention resolver, split-send review control |
| `patch_review` | header, attachment pill, budget strip, split-send review control |
| `branch_agent` | header, slash-command row, draft-state row, split-send review control |
| `docs_help` | mention resolver, slash-command row, tainted-context warning, attachment-stale banner |
| `companion` | attachment pill, budget strip, tainted-context warning, draft-state row, attachment-stale banner |

`docs_help` is the surface the acceptance criteria single out (`is_docs_or_help`):
every family it adopts must point at the canonical component schema so its prose can
never drift from the product truth.

## Component families (`M5PromptComposerComponentFamily`) → canonical primitive

Each family maps to exactly one narrowed primitive's canonical schema / doc /
support-export artifact. A consumer that adopts a family **references** those
canonical refs; it never re-words the facts in local prose.

| Family | Owning primitive lane |
| --- | --- |
| `prompt_composer_header`, `context_attachment_pill` | prompt-composer header / context-attachment pill |
| `mention_resolver`, `slash_command_row` | mention resolver / slash-command row |
| `budget_size_strip`, `tainted_context_warning` | budget / size strip / tainted-context warning |
| `draft_state_row`, `attachment_stale_banner`, `send_review_control` | draft-state row / attachment-stale banner / split-send review control |

Every family is adopted by at least two distinct consumers (`validate_family_reuse`).

## Shared descriptor vocabulary (`M5ComposerParityDescriptor`, all required)

`locality`, `route`, `approval`, `taint` — the track invariant that draft
locality/retention, route/provider/model, approval / send behaviour, and trust / taint
stay explicit on every surface. A binding that drops one is rejected.

## Auto-narrowing on a degraded workflow

`resolve_composer_binding` derives the claim-parity state from the parity-health mode:

| `M5ComposerParityHealth` | narrowing reason | recovery action |
| --- | --- | --- |
| `full_parity` | — (claims preserved, **no** banner) | — |
| `review_only_narrowed` | `review_only_workflow` | `return_to_live_composer_to_send` |
| `handoff_only_narrowed` | `handoff_only_workflow` | `resume_in_originating_composer` |
| `offline_mirror_narrowed` | `offline_or_mirror_scope` | `reconnect_to_live_route` |
| `companion_scope_narrowed` | `companion_scope_limited` | `open_in_full_composer` |

Any weakened mode emits a self-contained `M5ComposerAutoNarrowBanner` naming the exact
reason, the preserved descriptors, the export caveats, and the recovery action — never
a generic "degraded" note. The descriptor vocabulary stays intact under the narrowing,
so a review-only, handoff-only, offline / mirrored, or companion-scoped surface narrows
the claim **visibly** instead of inheriting stronger labels from a healthier surface.

## Invariants enforced by `validate`

- **Canonical reference** — each binding's schema/artifact ref equals the family's
  canonical ref and `references_canonical_not_local_prose` is `true`
  (`CanonicalRefMismatch`).
- **Family reuse** — every family adopted by ≥2 consumers
  (`ComponentFamilyReuseUnproven`).
- **Narrowing disclosure** — at least one worked binding proves a narrowed rendering
  with a self-contained banner (`NarrowingDisclosureUnproven`); at least one proves a
  full-parity rendering with preserved parity and no banner (`ScopePreservedUnproven`).
- **Docs/help reference** — every family a docs/help consumer adopts references the
  canonical schema (`DocsHelpReferenceMissing`).
- **Mandatory anatomy / export / descriptors**, accessibility route
  (`keyboard_focusable`), consumer surfaces, downgrade triggers, worked-binding
  self-consistency, stable-consumer proof refs, and four per-row hard invariants
  (`rewords_claims_per_surface`, `invents_new_composer_grammar`,
  `drops_locality_route_approval_or_taint_when_narrowed`,
  `inherits_stronger_label_from_healthier_surface`, all MUST be `false`).
- **Governance / projection / proof-freshness / release posture** blocks, plus a
  raw-material export guard (no `://`, tokens, or credentials in the packet).

## Artifacts

- Boundary schema: [`schemas/ai/m5-prompt-composer-component-consumer.schema.json`](../../../schemas/ai/m5-prompt-composer-component-consumer.schema.json)
- Support export / matrix CSV / report: `artifacts/ai/m5/m5-prompt-composer-component-consumer-proof/`
- Narrowed fixtures: `fixtures/ai/m5/m5-prompt-composer-component-consumers/`

The seed builders in `seed.rs` are the single producer of the checked-in export and
fixtures; the headless emitter
(`cargo run -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers`)
mints them and `checked_support_export_validates_and_matches_seed` asserts the disk
copy never drifts from the in-code matrix.
