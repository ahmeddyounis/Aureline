# Browser / Provider-Console Handoff Objects

This document is the contract for the handoff objects that turn every claimed M5
external documentation or provider-console exit into a typed, reviewable record
rather than a raw URL jump. A handoff is any moment the product leaves a governed
surface to open a browser or a provider console: a docs-browser open, a
help/about portal link, an AI-answer citation jump, or a provider-console pivot.
Each one must route through exactly one `BrowserHandoff` object, so the product
can always disclose *what* it opens, *why* in-product viewing was insufficient,
*what context is or is not shared*, and *how the reader returns*.

- Record kind: `browser_provider_console_handoff_objects_packet`
- Support-export record kind: `browser_provider_console_handoff_objects_support_export`
- Schema: [`schemas/docs/implement-browser-provider-console-handoff-objects-with-destination-reason-privacy-consequence-and-return-anchor.schema.json`](../../../schemas/docs/implement-browser-provider-console-handoff-objects-with-destination-reason-privacy-consequence-and-return-anchor.schema.json)
- Canonical support export: [`artifacts/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/support_export.json`](../../../artifacts/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/support_export.json)
- Summary artifact: [`artifacts/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor.md`](../../../artifacts/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor.md)
- Fixtures: [`fixtures/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/`](../../../fixtures/docs/m5/implement_browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor/)
- Producer: `aureline_docs::current_stable_browser_handoff_export`
- Headless emitter: `aureline_docs_browser_provider_console_handoff_objects`

## The exits

`HandoffSourceSurface` names the governed surface a handoff leaves. The
docs/help/AI/provider-console exits must each route at least one handoff
(`HandoffSourceSurface::REQUIRED_EXITS`):

| Source surface | Meaning |
| --- | --- |
| `docs_browser` | The docs browser / reader surface. |
| `help_about` | The help / about surface. |
| `ai_answer` | An AI answer / citation surface. |
| `provider_console_pivot` | A pivot into a provider console (AI or admin). |
| `review_surface` | A hosted review surface. |
| `support_history` | A reopened docs-history / support surface replaying a prior handoff. |

## The handoff object

A `BrowserHandoff` makes a boundary crossing a typed record:

- **Source identity** — `source_surface`, `source_identity_ref` (a stable ref,
  never a raw body), and `source_class` name the governed surface the reader is
  leaving. `source_class` reuses the canonical docs-contracts source-class
  vocabulary so project docs never masquerade as vendor docs.
- **Destination** — `destination_class` and the opaque `destination_ref` (never a
  raw URL) name *what* is opened. The destination-class tokens stay aligned with
  the integration-level browser-handoff packet contract.
- **Destination reason** — `destination_reason` (the canonical browser-handoff
  reason vocabulary) plus `destination_reason_note` say *why* in-product viewing
  was insufficient.
- **Privacy consequence** — `privacy_consequence` (the canonical vocabulary) plus
  the structured `shared_context` say *exactly what does or does not cross the
  boundary*.
- **Trust / policy posture** — `trust_class` (the canonical vocabulary) and
  `policy_posture` disclose how trustworthy the destination is and whether the
  handoff is allowed, needs confirmation, is blocked, or is unavailable.
- **Return anchor** — `return_anchor` (a kind, a stable `anchor_ref`, a label, and
  an optional `follow_up_note`) is the return-path guarantee: how the reader gets
  back.

## No hidden context share

`SharedContext` is the heart of the boundary-honesty guarantee. It records
exactly what crosses:

- `shares_resolved_destination_ref` — only the opaque resolved destination ref or
  anchor crosses.
- `shares_user_query_terms` — the user's query terms cross. Allowed **only** on an
  explicitly user-initiated, disclosed handoff.
- `shares_raw_code_selection`, `shares_private_readme_text`,
  `shares_unpublished_adr_content`, `shares_prompt_context` — the four named raw
  exfiltration vectors. They must **always** be false; a handoff that sets one is
  blocked.

The declared `privacy_consequence` must agree with the `shared_context`:
`no_context_shared` crosses nothing, `scoped_url_only` carries only the resolved
ref, `query_terms_disclosed` carries the user's query terms on a user-initiated
handoff, `isolated_session` opens an isolated session sharing no prior state, and
`shared_context_blocked` records a context share the product prevented.

Ordinary docs navigation (`ordinary_navigation`) must not share even the query
terms: a handoff that is part of ordinary navigation may carry only the resolved
destination ref. Query-term sharing requires an explicitly user-initiated
handoff.

## Every exit is reviewed

`routed_through_handoff_review` and `policy_posture` keep raw opens honest:

- A raw browser open, provider-console pivot, or docs fallback that did not go
  through explicit handoff review (`routed_through_handoff_review = false`) is
  blocked.
- A `blocked_by_policy` or `unavailable_disclosed` destination that is still
  `offered_as_actionable` is blocked — a blocked destination may not be presented
  as available. When honestly disclosed and **not** offered as actionable, it
  narrows below stable instead.

## Identity survives the round trip

`HandoffConsumerProjection` records that each consumer surface reuses the *same*
handoff object and can reconstruct the prior handoff reason and return anchor.
Each projection names the surface, the packet it belongs to, the handoffs it
reconstructs, and four preservation flags (`reuses_shared_handoff_object`,
`preserves_destination_reason`, `preserves_return_anchor`,
`preserves_privacy_consequence`). The `help_about`, `support_export`, and
`docs_history` surfaces must each carry a projection
(`BrowserHandoffConsumerSurface::REQUIRED_RECONSTRUCTION`), and the
`support_export` and `docs_history` projections must reconstruct **every** handoff
(`BrowserHandoffConsumerSurface::FULL_COVERAGE`) so an export or a reopened
history never silently flattens a handoff into ordinary navigation.

## Promotion and validation

`BrowserHandoffPacket::materialize` computes the validation findings and the
promotion state from the input:

- `stable` — all invariants hold.
- `narrowed_below_stable` — a non-fatal narrowing applies: a handoff is honestly
  disclosed as blocked / unavailable (and not offered as actionable), or a
  context share was honestly blocked.
- `blocks_stable` — a blocking invariant failed: a handoff leaks raw context,
  ordinary navigation shares context, a handoff bypasses review, a blocked
  destination is presented as available, a return anchor is dropped, a required
  exit lacks a handoff, or a support / history projection drops a handoff.

The packet is metadata-only: it carries no raw URLs, raw callback bodies, raw page
bodies, raw code selections, prompt text, raw provider payloads, or credentials.

## Consumers

The browser companion, help/about, support-export, docs-history, diagnostics, and
extension surfaces consume this packet directly. They project the shared handoff
object rather than re-deriving a private navigation state, so any claimed M5
external exit can be traced back to one handoff object naming its destination
class, the reason it left a governed surface, what context crossed the boundary,
and how the reader returns.
