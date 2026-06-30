# M5 Community-Handoff Target Review

This document is the contract for the M5 community-handoff target review sheet
set: the canonical source for whether an outbound issue, security-disclosure,
docs-feedback, RFC/discussion, community-support, or official-support route is
typed, labeled, and reviewable before a browser or handoff target opens.
Help/About, support, ecosystem, and reporting surfaces ingest the checked-in set
rather than minting parallel route dialogs, so official and community routes stay
distinguishable in-product and in exported issue/support packets.

- Record kind: `m5_community_handoff_target_sheet_set`
- Schema: [`schemas/help/m5-handoff-target.schema.json`](../../schemas/help/m5-handoff-target.schema.json)
- Canonical support export: [`artifacts/help/m5-community-handoff-proof/target_set.json`](../../artifacts/help/m5-community-handoff-proof/target_set.json)
- Governance summary: [`artifacts/help/m5-community-handoff-governance.md`](../../artifacts/help/m5-community-handoff-governance.md)
- Matrix CSV: [`artifacts/help/m5-community-handoff-targets.csv`](../../artifacts/help/m5-community-handoff-targets.csv)
- Fixtures: [`fixtures/help/community-handoff/`](../../fixtures/help/community-handoff/)
- Producer: `aureline_shell::m5_community_handoff_targets::current_stable_m5_community_handoff_target_set`
- Headless emitter: `aureline_shell_m5_community_handoff_targets`

## Governed routes

One sheet is named per route. Each sheet pins a destination trust class, the
visibility boundary that applies once the report leaves, the auth expectation,
the data-exit boundary plus a reviewable note naming who receives the data, a
commitment-honesty block, and a mandatory local-safe fallback.

| Route | Trust class | Visibility | Commitment |
| --- | --- | --- | --- |
| `public_issue` | `official_public` | `world_readable_public` | No commitment (public forum) |
| `security_disclosure` | `private_security` | `private_security_channel` | Security handled privately |
| `docs_feedback` | `official_public` | `world_readable_public` | No commitment (public forum) |
| `rfc_discussion` | `community` | `community_visible` | Best-effort community |
| `community_support` | `community` | `community_visible` | Best-effort community |
| `official_support` | `official_authenticated` | `official_account_visible` | Official supported commitment |

## Controlled vocabularies

The five-class destination/trust vocabulary is reused verbatim across in-product
surfaces and exported packets so official and community routes never blur:

- **Trust class** — `official_public`, `official_authenticated`, `community`,
  `private_security`, `local_only`.
- **Visibility boundary** — `world_readable_public`, `official_account_visible`,
  `community_visible`, `private_security_channel`, `local_never_leaves`.
- **Auth expectation** — `no_account_needed`, `official_account_required`,
  `community_account_typical`, `security_channel_credential`, `local_no_network`.
- **Data-exit boundary** — reused from the About/help/community destination
  vocabulary: `no_payload_leaves_product`, `metadata_safe_object_refs`,
  `proposal_refs_only`, `redacted_support_packet`, `security_payloads_only`,
  `external_public_browse`, `vendor_or_third_party_outbound`.
- **Commitment class** — `official_supported_commitment`, `best_effort_community`,
  `no_commitment_public_forum`, `security_handled_privately`,
  `local_draft_no_delivery`.

## Invariants

The producer enforces, and the schema mirrors, the following:

- **No accidental public coercion.** A route may only target a trust class from
  its allowed set; a route that cannot be typed is denied at build time rather
  than collapsing to a public target.
- **Trust class pins the boundary.** The visibility boundary, auth expectation,
  and data-exit boundary must each be consistent with the trust class.
- **Commitments are honest.** `guaranteed_product_commitment` is true only for
  the `official_supported_commitment` class on an `official_authenticated`
  route. Community and public routes are labeled best-effort or no-commitment and
  never masquerade as guarantees.
- **World-readable routes are reviewed.** `official_public` and `community`
  routes set `requires_prior_review_before_open = true` and
  `auto_open_from_critical_alert_allowed = false`, so a critical alert never
  auto-opens a public or community target without prior review.
- **Private/security routes are labeled and degrade safely.** A
  `private_security` route sets `unsupported_profile_disclosure_required = true`,
  and every route carries a `local_safe_fallback` whose destination never leaves
  the product, so an unsupported profile or blocked handoff degrades to a labeled
  local path instead of dead-ending.
- **Object anchors and issue templates are preserved.** When Aureline can hand
  off richer context, the sheet names the exact object anchor and the
  issue-template / structured-intake block rather than a screenshot or a fuzzy
  description.

Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
tokens, and raw secret material never cross this boundary; the records carry
opaque refs and bounded reviewable sentences only.

## Versioning

Adding a new route, trust class, visibility boundary, auth expectation,
data-exit, or commitment class is additive-minor and bumps the relevant schema
version. Repurposing an existing value is breaking and requires a new decision
row.
