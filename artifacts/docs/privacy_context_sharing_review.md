# Docs-browser privacy and context-sharing review

Reviewer-facing privacy review for the docs-browser baseline search
flows and the boundary beyond which broader context sharing
requires a separately-approved higher-trust surface. Companion to
[`/docs/verification/docs_browser_packet.md`](../../docs/verification/docs_browser_packet.md)
and
[`/fixtures/docs/symbol_link_validation_manifest.yaml`](../../fixtures/docs/symbol_link_validation_manifest.yaml).
If this review and the packet doc / ADR-0013 truth-source contract /
ADR-0007 redaction contract disagree, the packet and the ADRs win
and this review MUST update in the same change.

The frozen vocabulary tokens used below
(`privacy_context_sharing_posture`, `redaction_class`,
`browser_handoff_explanation_class`,
`docs_browser_cache_state_class`, `source_class`,
`version_match_state`) live in the docs-browser packet, the docs /
help / service-health truth-source schema
([`/schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)),
the ADR-0010 browser-handoff packet
([`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)),
and the ADR-0007 redaction-class export
([`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)).
This review does not mint parallel tokens; it freezes the posture
rules.

## Scope

Frozen at this revision:

- The baseline privacy posture for every docs-search flow the docs
  browser MAY perform without an explicit opt-in: exact symbol
  lookup, fuzzy docs search by title, pack navigation, mirror
  refresh, offline-bundle resolution, vendored-local resolution,
  generated-reference search, curated-knowledge-pack search, and
  support-runbook search.
- The explicit prohibition against baseline flows silently
  uploading surrounding code, private docs, workspace identity,
  file paths, symbol-graph neighbourhoods, or clipboard context to
  a remote docs provider.
- The boundary beyond which broader context sharing requires a
  separately-approved higher-trust surface, including the denial
  posture the docs browser renders until that approval fires.
- The export / screenshot posture required to preserve citation
  reconstructability on docs rows that travel through support-
  export, release-evidence, or claim-manifest channels.

Out of scope until a superseding decision row opens:

- The higher-trust context-sharing approval surface itself (its
  approval packet, its explicit-consent dialog, its scoped upload
  envelope, and its operator audit rows). This review names only
  where baseline flows MUST deny and route to it.
- The AI-explanation overlay's prompt-engineering rules. The
  derived-explanation reuse boundary is covered here; prompt
  formatting and model selection are not.
- The provider-overlay prefetch strategy (which vendor docs the
  overlay fetches proactively). This review freezes only the
  minimum-metadata discipline such overlays MUST follow.

## Posture tokens (frozen — mirrors the docs-browser packet)

- `local_only_no_remote_transmission` — the baseline.
- `mirror_fetch_metadata_minimum` — the only acceptable remote
  call shape for a docs-pack mirror refresh from a baseline flow.
- `vendor_overlay_inspect_only_minimum` — the only acceptable
  remote call shape for a vendor / provider overlay row from a
  baseline flow.
- `higher_trust_context_sharing_required` — baseline denial; row
  routes to the separately-approved surface.
- `refused_policy_blocked_remote` — admin-policy denial; row
  refuses the remote call and cites the policy-pack ref.

## What baseline docs-search flows MAY transmit

Baseline flows MAY transmit the following metadata when the row
resolves against a mirror, a vendor / provider overlay, or an
external status feed; never otherwise.

1. **Pack identity metadata.**
   - `pack_id` (opaque id, safe to log).
   - `pack_revision_ref` (opaque id, safe to log).
   - `source_class` token from the ADR-0013 vocabulary.
2. **Coarse locale metadata.**
   - A single BCP-47 locale tag (e.g. `en`, `en-US`) the user's
     session has selected.
3. **Refresh-shape metadata.**
   - `mirror_endpoint_ref` or `destination_descriptor_ref`
     (opaque id).
   - The last-known mirror-snapshot digest the product is
     requesting a refresh above.
4. **User-typed query strings** (only when the user has typed
   them).
   - The literal docs-search query string the user typed into the
     docs-browser input field.
   - A vendor / provider overlay MAY receive the literal query
     string; it MUST NOT receive any other workspace context.
5. **Browser-handoff envelope fields** (only when the user
   initiates an out-of-product action).
   - The ADR-0010 `browser_handoff_packet` envelope fields
     (destination class, reason code, disclosure summary). The
     `browser_handoff_packet` body itself MAY include a link
     target; the docs browser does NOT reuse it for further
     transmission.

Everything above rides the `mirror_fetch_metadata_minimum` or
`vendor_overlay_inspect_only_minimum` posture on the projected
`help_status_badge_record` and the `docs_browser_result_record`.
The typed minimum-metadata disclosure MUST be available on the
primary surface (not tooltip-only) on any row that transmits
anything to a remote provider.

## What baseline docs-search flows MAY NOT transmit

Baseline flows MUST NOT transmit any of the following to any remote
docs / vendor / provider. This list is closed; additions are
additive-minor and bump this review's schema version.

- **Surrounding code** — source text from any file in the
  workspace, even when the row's query originated from a symbol
  lookup in that file. The docs browser uses only the symbol id,
  pack id, and pack revision ref to resolve the row locally; the
  surrounding code stays local.
- **Symbol-graph neighbourhood** — adjacent symbols, call graph,
  type graph, or reverse-reference lists derived from the project
  index. These stay local.
- **File paths** — absolute paths, workspace-relative paths, or
  directory listings. The docs browser resolves rows by pack id /
  pack revision / symbol id only; it never attaches paths to
  remote calls.
- **Private docs pack bodies** — page content of
  `project_docs`, `curated_knowledge_pack`, or `support_runbook`
  packs whose `redaction_class` is
  `internal_support_restricted`, `operator_only_restricted`, or
  `signing_evidence_only`. Private pack bodies stay local; the
  docs browser MAY render them locally but MAY NOT forward their
  bytes to a vendor or mirror refresh endpoint.
- **Workspace identity** — workspace id, workspace trust state,
  managed-workspace instance id, execution-context id, or any
  ADR-0001 identity-mode attribute. None of these reach remote
  providers from a baseline flow.
- **User identity / account credentials** — the ADR-0007 secret
  broker redacts these before any persistent or exportable sink
  sees them. Baseline docs-search flows MUST NOT attach them to
  any remote call.
- **Clipboard or selection context** — the user's current text
  selection, clipboard contents, or active editor content. The
  docs browser does not read these as input to a baseline flow.
- **Hover or keyboard-driven context** — the symbol under the
  cursor outside of an explicit user-typed query. The docs
  browser resolves a symbol-linked reference only when the user
  explicitly requests it (click on a symbol, command-palette
  "Go to docs for ..."); hover-driven prefetch is NOT a baseline
  flow.
- **Telemetry payloads beyond the ADR-0013 audit-event envelope** —
  the `docs_help_service_health` audit stream carries id-only
  records. Baseline flows MUST NOT layer additional telemetry on
  top.

Any flow that would need one of the above to resolve a row MUST
deny the baseline resolution and route to the higher-trust context-
sharing surface. Silent escalation is non-conforming.

## Higher-trust context sharing (where it lives, not what it does)

Broader context sharing (e.g. "use the surrounding function body to
rank candidate docs pages", "use the symbol-graph neighbourhood to
disambiguate an overloaded symbol", "send the private internal
runbook body to the AI overlay for summarisation") is not a
baseline flow. It lives on a separately-approved higher-trust
surface whose design is out of scope for this review. What this
review freezes is the boundary:

- A docs-browser row that would require broader context to resolve
  MUST set `derived_explanation_reuse_state =
  refused_vendor_overlay_requires_higher_trust` or the general
  `privacy_context_sharing_posture =
  higher_trust_context_sharing_required`.
- The row MUST render the typed higher-trust-approval-required
  disclosure on the primary surface.
- The docs browser MUST NOT auto-transition a baseline flow into a
  higher-trust flow mid-render. The user (or an admin policy with
  a typed approval) must cross the boundary explicitly.
- The higher-trust surface, when landed, MUST carry its own
  ADR-0010 browser-handoff packet, its own approval-ticket
  record, and its own scoped upload envelope. It MAY NOT reuse
  the docs-browser row as implicit approval.

## Admin-policy boundary

Admin policy MAY narrow baseline flows further (e.g. block a named
vendor provider, disable the offline-bundle mechanism, forbid
mirror refresh on a specific client scope). When policy narrows a
flow, the docs-browser row MUST project
`privacy_context_sharing_posture = refused_policy_blocked_remote`
and MUST cite the policy-pack ref on the projected badge record's
`policy_context`.

Admin policy MAY NOT silently widen baseline flows beyond what this
review permits. A policy that attempts to auto-enable broader
context sharing without the higher-trust approval surface is
non-conforming and the docs browser denies render with the
ADR-0013 `policy_blocked` denial reason until the higher-trust
approval path is used.

## Export and screenshot posture

Docs-browser rows travel through support-export, release-evidence,
claim-manifest, and object-handoff channels. The citation-anchor
reconstructability contract comes from ADR-0013 (anchors are ids
only; bodies are fetched on demand). This review freezes the
additional export posture required to preserve reconstructability.

1. **Citation anchors retained.** Every exported docs-browser row
   MUST include its `citation_anchor_refs` list; dropping the refs
   on export is non-conforming. A support bundle that strips
   anchors for brevity is non-conforming; anchors are the
   reconstruction key.
2. **Source / version / cache state retained.** The exported row
   MUST include `source_class`, `version_match_state`,
   `freshness_class`, and `cache_state_class` so a reviewer
   reading the export can tell at a glance whether the row was
   live, cached, mirrored, or offline-bundled at the time the row
   was minted.
3. **No body bytes exported.** The export payload MUST NOT
   include page bodies, symbol definitions, runbook bodies, or
   raw URLs. Anchors + pack revision refs + target-build-identity
   refs are sufficient; anything more violates ADR-0013.
4. **Redaction class honoured.** The `redaction_class` on the
   row governs which sinks the row reaches. Rows with
   `redaction_class` of `internal_support_restricted`,
   `operator_only_restricted`, or `signing_evidence_only` MUST NOT
   reach screenshot-safe / public-export channels; the
   support-export pipeline enforces this via the ADR-0007
   redaction pass.
5. **Screenshot safety on browser-handoff rows.** A row whose
   `browser_handoff_explanation_class` is non-null MUST carry a
   screenshot-safe `disclosure_summary` (ADR-0010). The
   disclosure is what a reader sees in a support screenshot; it
   MUST describe the destination class and reason without
   including raw URLs or user identity.

## Derived-explanation reuse privacy

The docs browser's `derived_explanation_reuse_state` tokens (see
the packet doc) are the privacy contract for AI-overlay reuse.
Additional privacy rules here:

- An AI overlay that quotes a docs-browser row MUST treat the row
  as the minimum sufficient context. The overlay MAY NOT layer
  additional surrounding-code or symbol-graph context on top
  without the higher-trust approval path.
- A derived-explanation row whose
  `derived_explanation_reuse_state` is
  `refused_vendor_overlay_requires_higher_trust` is a denial; the
  AI overlay MUST NOT render a degraded-quality explanation in
  lieu.
- An AI overlay whose prompt would need to include any of the
  prohibited payloads (surrounding code, symbol-graph
  neighbourhood, file paths, private pack bodies, workspace
  identity, clipboard context) MUST deny render and route to the
  higher-trust surface. The ADR-0013
  `derived_explanation_uncited_refused` denial applies when the
  overlay cannot resolve a compliant explanation under the
  baseline boundary.

## Conformance checklist

A surface conforms to this review when all of the following hold:

1. Every docs-browser row carries exactly one
   `privacy_context_sharing_posture` token from the frozen
   vocabulary.
2. Rows with posture `local_only_no_remote_transmission` emit no
   outbound network calls to docs / vendor / AI providers from
   baseline flows.
3. Rows with posture `mirror_fetch_metadata_minimum` or
   `vendor_overlay_inspect_only_minimum` limit outbound payload to
   the fields enumerated in §What baseline docs-search flows MAY
   transmit.
4. Rows with posture `higher_trust_context_sharing_required` deny
   render in baseline flows and route to the higher-trust
   approval surface.
5. Rows with posture `refused_policy_blocked_remote` cite the
   policy-pack ref that blocked the flow.
6. Exports preserve citation anchors, source / version / cache
   fields, and redaction class; never body bytes.
7. AI-overlay reuse follows the `derived_explanation_reuse_state`
   tokens and never layers additional context on top of a row
   without the higher-trust approval path.
8. Admin policy that attempts to silently widen baseline flows
   beyond this review is denied with ADR-0013 `policy_blocked`.

## Known gaps and follow-ups

- **Higher-trust surface pending.** The approval packet, consent
  dialog, scoped upload envelope, and operator audit rows for the
  higher-trust context-sharing surface are not frozen here; a
  later milestone will land them and this review will update to
  name the surface explicitly.
- **AI-overlay prompt rules pending.** The specific prompt-
  construction rules for the AI-explanation overlay are not
  covered; only the reuse-state / privacy boundary is.
- **Provider prefetch strategy pending.** Whether the vendor /
  provider overlay may proactively warm-fetch rows (and how many
  minimum-metadata calls that implies) is not frozen; current
  posture assumes one minimum-metadata call per user-initiated
  lookup. A later review may relax this with an explicit
  budget.
- **Offline docs-pack provenance note.** Offline bundles imported
  via signed distribution currently carry an
  `air_gapped_origin_label`; that field is an operator-supplied
  label and never a URL or path, but the review notes it here so a
  later policy pass can decide whether to further narrow its
  content.

## Reviewer signoff

- **Reviewer / forum:** `@ahmeddyounis`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:**
  `packet_row:docs_browser.privacy_context_sharing`,
  `packet_row:docs_browser.derived_explanation_reuse`,
  `packet_row:docs_browser.browser_handoff_explanation`
- **Blocking refs:** `none`
