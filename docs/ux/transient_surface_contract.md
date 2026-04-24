# Transient-surface contract: tooltip, hovercard, popover, peek panel — previews, pinning, freshness

This document is the **cross-surface transient-preview contract**
for Aureline. It exists so one preview primitive, one set of
trigger / escalation rules, one freshness-and-stale-state
posture, one pinning-and-promotion rule, and one keyboard- /
touch-accessibility fallback serve every surface that shows a
transient preview of a canonical object — symbol / reference
tooltips, documentation hovercards, AI-derivation hovercards,
provider-bearing popovers, diff-peek panels, run / job / evidence
peeks, search-result peeks — without minting parallel preview
models, private freshness vocabularies, or hidden side channels
that hide consequential instructions behind pointer hover.

The contract is normative. Where this document disagrees with the
UI / UX Spec sections it quotes, the source spec wins and this
document MUST be updated in the same change. Where this document
disagrees with a downstream surface's private preview, hovercard,
popover, or peek story, this document wins and the surface is
non-conforming.

The companion artifacts are:

- [`/schemas/ux/transient_surface.schema.json`](../../schemas/ux/transient_surface.schema.json)
  — boundary schema every non-owning surface reads. Freezes the
  `transient_preview_record`, the `preview_escalation_rule_record`,
  and the `pinned_preview_record` shapes along with the closed
  vocabularies this document binds (preview-surface kind, trigger
  kind, content-consequence class, freshness class, stale-state
  class, mapping-quality class, representation-label class,
  attribution-source class, promote-action kind, keyboard-route
  kind, denial reason).
- [`/fixtures/ux/preview_surfaces/`](../../fixtures/ux/preview_surfaces/)
  — worked examples covering routine tooltips, AI-assisted
  hovercards with confidence disclosure, approval / safety-
  critical content that cannot live in a tooltip, vanished
  targets, stale cached remote previews, pinning that preserves
  identity, and a pointer-hover-independent keyboard route.

This contract rides alongside — it does **not** re-mint — the
vocabularies frozen in:

- [`/docs/ux/chronology_row_contract.md`](./chronology_row_contract.md)
  and [`/schemas/ux/history_row.schema.json`](../../schemas/ux/history_row.schema.json)
  — canonical object target ref, provenance-badge class, importance
  class, detail-link vocabulary, time-presentation policy. A
  preview whose `canonical_object_target_ref` is also a history
  row SHOULD preserve that identity on promotion rather than
  minting a parallel id.
- [`/docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md)
  and [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  — canonical event id and durable linkback rules. A preview that
  surfaces lineage-bearing work SHOULD cite the event-lineage
  record rather than renaming the fields.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — authority, consequence, revert, and representation
  vocabularies. A preview that carries a consequence-bearing
  interaction quotes its `interaction_safety_packet_record` by
  ref.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — source subsystem, client scope, redaction class. Re-exported
  here verbatim.
- `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
  — freshness and basis-drift invalidation semantics.
- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
  — redaction pass runs before bytes reach any persistent or
  exportable sink, including preview `summary_label` text.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  — `freshness_class`, `client_scope`, `redaction_class` are
  re-exported without modification.

## Who reads this document

- **Editor, palette / search, review-and-diff, docs / help, AI
  apply, provider-bearing, terminal, task-runner, and support
  surfaces** — to emit previews with the same required metadata
  rather than minting per-surface tooltip / hovercard models.
- **Shell, a11y, and accessibility pipelines** — to verify that
  every preview has a keyboard route and a touch fallback; that
  no safety-critical instruction lives only behind pointer
  hover; and that pinned previews preserve object identity and
  provenance.
- **Support, admin, and release evidence pipelines** — to consume
  pinned-preview exports with a single structured vocabulary
  whose object identity, provider attribution, and freshness at
  pin time survive round-tripping.

## What problem this contract solves

Tooltips and hovercards tend to become hidden mini-applications:
an AI summary without a confidence badge, a cached docs snippet
without an age label, a safety-critical recovery hint that only
appears on mouse hover, a "preview" whose canonical target has
long since moved, a pin that lands on a generic home screen
because the preview forgot its provider. Every one of those is a
product failure: a user takes action on content whose provenance,
freshness, or mapping quality was undisclosed.

This contract forbids those failures by declaring:

1. Four canonical preview-surface kinds (`tooltip`, `hovercard`,
   `popover`, `peek_panel`) plus a named escalation target
   (`full_sheet_or_tab`) — no private aliases.
2. A shared metadata contract every preview MUST carry — canonical
   target ref, provenance, representation label, freshness,
   mapping quality, AI-confidence disclosure, stale state,
   keyboard route, and the promote-action set.
3. A content-consequence ladder — **safety-critical recovery
   instructions and approval-required instructions MUST NOT live
   in tooltip-only or hovercard-only deliveries**; they escalate
   to a peek panel or a full sheet and remain reachable without
   pointer hover.
4. A pinning-and-promotion rule — pinning or promoting preserves
   canonical object identity, provider attribution, freshness
   cues, and AI-confidence disclosure; a pinned preview is never
   a contextless orphan view.
5. A keyboard- and touch-accessibility rule — every preview has at
   least one keyboard route and a touch fallback; pointer-hover-
   only previews are non-conforming for tier_actionable or
   higher content.

## 1. Three transient-preview primitives

### 1.1 `transient_preview_record`

One structured record per **(canonical object, preview surface
kind, trigger kind, presented_at)** tuple. Minted by the surface
rendering the preview. A preview that renders without a canonical
target ref and without a well-typed `mapping_quality_class`
naming the degradation is non-conforming
(`denial_reason = preview_without_canonical_target_or_mapping_quality`).

Every record carries:

- **canonical target identity** — `canonical_object_target_ref`
  (opaque id).
- **source / provider attribution** — `source_subsystem`,
  `attribution_source_class`, optional
  `provider_attribution_label` when the content came from an
  extension, remote service, or AI provider.
- **representation label** —
  `representation_label_class` names what kind of render this
  preview is (full canonical object, summary, type signature,
  remote snippet, cached snippet, generated narrative,
  AI-derivation with confidence, approximate preview).
- **freshness and stale state** — `freshness_class`,
  `stale_state_class`, `freshness_observed_at`, and (when
  applicable) `age_label`.
- **mapping quality** — `mapping_quality_class` names whether the
  preview resolves to the exact canonical target, a near match,
  a type-only fallback, or an unavailable (moved / missing /
  policy-blocked) target.
- **AI-confidence disclosure** — `ai_confidence_class` when
  attribution is `ai_assisted` or `ai_generated`.
- **content consequence** — `content_consequence_class` names how
  consequential the content is; the ladder below binds this
  class to the minimum preview-surface tier.
- **keyboard route** — `keyboard_route_kinds_available` (minItems
  1) names the keyboard routes this preview exposes.
- **pointer-hover dependency** — `pointer_hover_dependent` is a
  boolean that MUST be false on tier_actionable or higher
  content.
- **promote actions** — `allowed_promote_actions` names every
  promote / pin / open target the surface exposes. Promoting to
  a generic home screen is forbidden
  (`denial_reason = promote_to_generic_home_screen`).

A transient preview is **never authoritative for a consequential
action**. The authoritative surface is the promotion target
(peek, full sheet, review sheet, diff view, canonical target).
The preview names the promotion target; the user's action binds
to the promoted surface, not to the preview.

### 1.2 `preview_escalation_rule_record`

One row per **preview-surface kind** naming default trigger
kinds, allowed escalation tiers, forbidden surface kinds, the
minimum-tier mapping per `content_consequence_class`, required
keyboard-route kinds, and whether a touch fallback is required.
Rows are frozen; a surface that mints a local escalation rule for
one of the four transient-surface kinds is non-conforming.

### 1.3 `pinned_preview_record`

One structured record per **user pin or promotion** that
persists. A pin or promotion MUST preserve:

- `canonical_object_target_ref` (identity) identical to the
  source preview;
- `attribution_source_class` and `provider_attribution_label`;
- `freshness_at_pin_time`, `stale_state_at_pin`, and
  `freshness_observed_at_pin` so the pin records the freshness
  at the moment of pinning — the pinned view does not later
  silently present stale content as live;
- `ai_confidence_class` when attribution is AI-assisted or
  AI-generated;
- a `promote_target_record` naming the durable destination.

A pin or promotion that drops any of `canonical_object_target_ref`,
`attribution_source_class`, `freshness_at_pin_time`, or the
AI-confidence disclosure is non-conforming
(`denial_reason ∈ {promote_lost_canonical_identity,
promote_lost_provider_attribution, promote_lost_freshness_cue,
promote_ai_without_confidence_disclosure}`).

## 2. Four transient-surface kinds plus one escalation target

The `preview_surface_kind` vocabulary is closed at five values.
The first four are transient in-product surfaces; the fifth is
the named escalation target that promotion lands on.

| Kind                     | Intent                                                                                                               | Typical triggers                                                   |
|--------------------------|----------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------|
| `tooltip`                | Single-line identification / name / type signature. Routine, non-actionable.                                        | `pointer_dwell`, `focus_dwell`, `keyboard_invocation`              |
| `hovercard`              | Multi-field summary: identity + type + short description + one or two metadata fields + one or two promote actions. | `pointer_dwell`, `focus_dwell`, `keyboard_invocation`              |
| `popover`                | In-surface actionable panel attached to an anchor. MAY carry a primary action when the action is reversible.        | `keyboard_invocation`, `explicit_invoke`, `touch_long_press`       |
| `peek_panel`             | In-surface panel opening a full preview (diff peek, evidence peek, run peek, docs peek). Carries full metadata and at least one promote action to the canonical target.| `keyboard_invocation`, `explicit_invoke`                           |
| `full_sheet_or_tab`      | Named escalation target. Not a transient surface; a preview promotes to this target rather than expanding in place.| Invoked via `allowed_promote_actions` on the transient preview.    |

Distinctions frozen by this contract:

- **Tooltip is not a hovercard.** A tooltip renders a single line
  of identification; it does not carry a promote action.
  Surfaces that mint a "rich tooltip" with embedded actions are
  non-conforming — use `hovercard` or `popover`.
- **Hovercard is not a popover.** A hovercard is a passive
  summary surface that disappears on pointer leave; a popover is
  anchored to the surface and persists until dismissed
  explicitly. Popovers MAY carry a primary action when the
  action is reversible; hovercards MAY NOT mint the same
  primary action.
- **Popover is not a peek.** A popover is bound to its anchor; a
  peek is an in-surface full panel that replaces in-place
  content while the user decides whether to promote to
  `full_sheet_or_tab`.
- **Peek is not a full sheet.** A peek is transient; the full
  sheet / tab is durable. Promotion crosses that boundary and
  MUST mint a `pinned_preview_record` (or a durable detail row)
  that preserves identity.

## 3. Triggers, keyboard routes, and touch fallback

The `trigger_kind` vocabulary is closed:

- `pointer_dwell` — pointer remains over the anchor past a dwell
  threshold.
- `focus_dwell` — keyboard or assistive-tech focus lands on the
  anchor and the surface exposes the preview on focus.
- `keyboard_invocation` — user presses the preview-invocation
  shortcut while focus is on the anchor.
- `touch_long_press` — long-press on a touch surface invokes the
  preview.
- `explicit_invoke` — user explicitly invokes via a menu item, a
  command-palette entry, or a peek / pin action.

**Keyboard route rule.** Every preview MUST expose at least one
`keyboard_route_kind` drawn from the closed set:

- `show_details_focus_ring` — expose the preview on the focused
  anchor without pointer hover.
- `invoke_primary_promote` — keyboard invocation of the primary
  promote action.
- `pin_current_preview` — keyboard-driven pinning.
- `escalate_next_tier` — keyboard-driven escalation to the next
  preview-surface tier (hovercard → popover / peek, peek →
  full_sheet_or_tab).
- `dismiss_preview` — keyboard-driven dismiss that MUST NOT
  mutate the underlying object.
- `open_canonical_target` — keyboard-driven open of the canonical
  target.

A preview whose `keyboard_route_kinds_available` is empty is
non-conforming (`denial_reason = preview_missing_keyboard_route`).

**Touch fallback rule.** Every preview kind MUST name how it is
reachable on a touch surface. For `tooltip` and `hovercard`, the
touch-fallback is `touch_long_press`; for `popover` and
`peek_panel`, the touch-fallback is `explicit_invoke` (a
dedicated touch target). A surface that renders a hovercard on
desktop and nothing on touch is non-conforming
(`denial_reason = preview_missing_touch_fallback`).

**Pointer-hover dependency rule.** For any preview whose
`content_consequence_class` is `non_reversible_consequential`,
`safety_critical_recovery_instruction`, or
`approval_required_instruction`, `pointer_hover_dependent` MUST
be `false`. The content must remain reachable via focus,
keyboard invocation, and touch without pointer dwell.

## 4. The content-consequence ladder and the escalation rule

The `content_consequence_class` vocabulary is closed at four
values and binds each class to a minimum preview-surface tier:

| `content_consequence_class`                 | Minimum preview-surface kind                              | Rationale                                                                                                        |
|---------------------------------------------|-----------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| `routine_informational`                     | `tooltip`                                                 | A label, a type signature, a short identity blurb. Pointer dwell is acceptable; keyboard route still required.  |
| `non_reversible_consequential`              | `popover` (with primary action) or `peek_panel`           | Actionable content that survives pointer leave; hover alone is not enough.                                      |
| `safety_critical_recovery_instruction`      | `peek_panel` plus a durable promotion target             | Recovery content must remain readable without hover; the user reaches full detail via promotion or pin.         |
| `approval_required_instruction`             | `peek_panel` plus a durable promotion target             | Approval steps cannot be hidden behind hover; a keyboard route MUST reach the preview.                          |

Rules (frozen):

1. **Safety-critical recovery instructions MAY NOT live in a
   `tooltip` or a `hovercard` alone.** A preview whose
   `content_consequence_class = safety_critical_recovery_instruction`
   and whose `preview_surface_kind ∈ {tooltip, hovercard}` is
   non-conforming
   (`denial_reason = safety_critical_content_in_tooltip_only`).
2. **Approval-required instructions MAY NOT live behind pointer
   hover alone.** A preview whose
   `content_consequence_class = approval_required_instruction`
   and whose `pointer_hover_dependent = true` is non-conforming
   (`denial_reason = approval_instruction_in_hover_only`).
3. **Tier regression is forbidden without a reason.** A preview
   surface that opens at `peek_panel` for consequential content
   and later renders the same canonical target as a
   `tooltip`-only without a typed reason is non-conforming
   (`denial_reason = escalation_tier_regressed_without_reason`).
4. **Escalation names a target.** A transient preview MUST name
   at least one entry in `allowed_promote_actions` that lands on
   a durable destination (`open_canonical_target_exact`,
   `open_review_sheet`, `open_diff_view`, `open_full_detail_sheet`,
   `open_history_row`, `open_durable_job_row`, or
   `pin_to_surface`). A preview with zero promote targets is
   non-conforming
   (`denial_reason = transient_surface_missing_escalation_path`).
5. **Promotion to a generic home screen is forbidden.** A
   promote action whose target is a home screen, a search box,
   or an external URL is non-conforming
   (`denial_reason = promote_to_generic_home_screen`).

## 5. Freshness, stale state, and vanished targets

The `freshness_class` vocabulary is closed:

- `live_authoritative` — first-party live observation, current.
- `cached_last_known_good` — cached from a live source that is
  currently reachable; cache age within TTL.
- `stale_past_ttl` — cached content whose TTL has expired; the
  source may or may not be reachable.
- `remote_unreachable_cached` — cached content whose live source
  is presently unreachable (offline, mirror-only, network
  partition).
- `generated_synthesized` — content generated / synthesized from
  other first-party sources (not AI).
- `ai_assisted_derivation` — AI-assisted content with human
  anchors; `ai_confidence_class` MUST be disclosed.
- `approximate_best_effort` — approximate rendering when exact
  resolution is unavailable (near-match, fuzzy, type-only).

The `stale_state_class` vocabulary is closed:

- `live_current` — source is live and the render reflects the
  current state.
- `stale_ttl_exceeded` — cache TTL has expired; content may have
  drifted.
- `stale_basis_drifted` — subscription basis has changed
  (invalidation fired) and the preview has not re-derived yet.
- `stale_source_unreachable` — cached but the live source is
  presently unreachable.
- `vanished_target_missing` — the canonical target has been
  removed.
- `vanished_target_moved` — the canonical target has moved /
  renamed; a typed link-forward MAY be offered.
- `vanished_target_policy_blocked` — the canonical target is no
  longer reachable under current policy.

Rules (frozen):

1. **A preview MUST name its freshness.** Every preview carries a
   `freshness_class` and a `stale_state_class`. A preview whose
   `freshness_class != live_authoritative` MUST also carry a
   non-null `age_label` describing how old the cached / stale /
   remote content is (`denial_reason =
   stale_preview_presented_as_live` when a non-live preview
   claims live freshness).
2. **Vanished targets disclose the reason.** Any preview whose
   `stale_state_class ∈ {vanished_target_missing,
   vanished_target_moved, vanished_target_policy_blocked}` MUST
   set `mapping_quality_class ∈ {unavailable_target_missing,
   unavailable_target_moved, unavailable_target_policy_blocked}`
   and MUST carry a non-null `unavailability_reason_label`
   (`denial_reason = vanished_target_presented_as_present`).
3. **Generated and AI-derived previews are badged.** A preview
   whose `freshness_class ∈ {generated_synthesized,
   ai_assisted_derivation}` MUST carry a
   `representation_label_class ∈ {generated_narrative,
   ai_derivation_with_confidence}`; a preview whose attribution
   is AI MUST disclose `ai_confidence_class` (`denial_reason =
   ai_derivation_undisclosed`).
4. **Approximate previews disclose approximation.** A preview
   whose `freshness_class = approximate_best_effort` MUST set
   `representation_label_class = approximate_preview` and
   `mapping_quality_class` MUST be one of `near_match_fuzzy` or
   `type_only_no_instance`
   (`denial_reason =
   generated_preview_presented_as_authoritative`).

## 6. Attribution, provider, and representation

The `attribution_source_class` vocabulary is closed at ten values:

- `first_party_direct` — first-party subsystem observation.
- `first_party_synthesized` — first-party synthesis from other
  first-party sources.
- `extension_contributed` — contributed by an installed
  extension; `provider_attribution_label` names the extension.
- `remote_agent_reported` — reported by a remote agent.
- `companion_reported` — reported by a companion surface.
- `ai_assisted` — partially AI-generated with human anchors;
  `ai_confidence_class` MUST be disclosed.
- `ai_generated` — fully AI-generated;
  `ai_confidence_class` MUST be disclosed.
- `imported_external_audit` — imported from an upstream audit
  stream.
- `cached_mirror_of_external` — cached mirror of an external
  source; `provider_attribution_label` names the upstream
  source.
- `reconstructed_from_backup` — reconstructed from a backup /
  support bundle.

The `representation_label_class` vocabulary is closed at eight
values:

- `canonical_object_full` — full canonical object render.
- `canonical_object_summary` — summary of the canonical object.
- `canonical_object_type_signature` — type signature only.
- `remote_fetched_snippet` — snippet fetched live from a remote
  source; carries age information.
- `cached_snippet_with_age_label` — cached snippet with a
  required age label.
- `generated_narrative` — first-party generated narrative.
- `ai_derivation_with_confidence` — AI-derived content with a
  non-null `ai_confidence_class`.
- `approximate_preview` — approximate / near-match render.

The `mapping_quality_class` vocabulary is closed at six values:

- `exact_canonical_target` — preview resolves to the exact target.
- `near_match_fuzzy` — preview resolves to a near match.
- `type_only_no_instance` — preview resolves to type-level
  information without a specific instance.
- `unavailable_target_missing` — target has been removed.
- `unavailable_target_moved` — target has moved / renamed.
- `unavailable_target_policy_blocked` — target is not reachable
  under current policy.

Rules (frozen):

1. **Attribution is never omitted.** Every preview names exactly
   one `attribution_source_class`; a preview that surfaces
   remote / extension / AI content with attribution inferred
   from context is non-conforming.
2. **Providers are labelled.** When
   `attribution_source_class ∈ {extension_contributed,
   cached_mirror_of_external}` or an AI / remote-agent class
   that carries a named provider, `provider_attribution_label`
   MUST be non-null.
3. **Representation matches freshness.** A preview whose
   `representation_label_class = cached_snippet_with_age_label`
   MUST carry a non-null `age_label`.

## 7. Pinning and promotion: identity preservation

Pinning and promotion both transition a transient preview into a
more durable surface. The contract treats them uniformly:

- **Pin** — `promote_action_kind = pin_to_surface` — user pins
  the preview to a persistent shell surface (pinned-references
  sidebar, pinned-previews strip, workspace pinboard). A
  `pinned_preview_record` is minted.
- **Promote-to-open** —
  `promote_action_kind ∈ {open_canonical_target_exact,
  open_review_sheet, open_diff_view, open_full_detail_sheet,
  open_history_row, open_durable_job_row}` — user promotes the
  preview to a durable surface via an open action. The durable
  surface already owns the object; the preview's role is to
  name the canonical target so the promotion lands on the
  correct durable row.
- **Escalate-next-tier** —
  `promote_action_kind = escalate_to_next_tier` — user escalates
  the preview to the next allowed preview-surface tier
  (hovercard → popover / peek, peek → full_sheet_or_tab). The
  canonical target and attribution are preserved across the
  escalation.

Rules (frozen):

1. **Identity is preserved on pin / promote.**
   `canonical_object_target_ref` on a `pinned_preview_record`
   MUST equal the `canonical_object_target_ref` on the
   originating `transient_preview_record`. A pin that drops or
   rewrites the canonical target is non-conforming
   (`denial_reason = promote_lost_canonical_identity`).
2. **Provider attribution is preserved.**
   `attribution_source_class` and (when applicable)
   `provider_attribution_label` on the pinned record MUST equal
   the transient record
   (`denial_reason = promote_lost_provider_attribution`).
3. **Freshness cues are preserved.** The pin records
   `freshness_at_pin_time`, `stale_state_at_pin`, and
   `freshness_observed_at_pin` — the freshness at the instant
   the pin was made. A pin that silently presents pinned-at-time
   freshness as current is non-conforming
   (`denial_reason = promote_lost_freshness_cue`).
4. **AI-confidence is preserved.** When
   `attribution_source_class ∈ {ai_assisted, ai_generated}`, the
   pinned record's `ai_confidence_class` MUST equal the source
   preview's class; a pin that drops AI-confidence is
   non-conforming
   (`denial_reason = promote_ai_without_confidence_disclosure`).
5. **Pin of a vanished target preserves the vanishing.** A pin
   whose `stale_state_at_pin ∈ {vanished_target_missing,
   vanished_target_moved, vanished_target_policy_blocked}` MUST
   carry a non-null `unavailability_reason_label` and a
   `promote_target_record.kind ∈ {audit_trail_only,
   not_available_linkback_lost}` — pinning preserves the fact
   that the target was already vanished at pin time rather than
   silently turning the pin into a fresh live view.

## 8. Stale content, remote / cached / generated / approximated previews

- A preview whose source is reachable live renders with
  `freshness_class = live_authoritative` and
  `stale_state_class = live_current`.
- A preview whose source is remote and cached renders with
  `freshness_class ∈ {cached_last_known_good,
  stale_past_ttl, remote_unreachable_cached}` and carries an
  `age_label`.
- A preview whose content is generated from other first-party
  sources renders with
  `freshness_class = generated_synthesized` and
  `representation_label_class = generated_narrative`.
- A preview whose content is AI-derived renders with
  `freshness_class = ai_assisted_derivation`,
  `representation_label_class = ai_derivation_with_confidence`,
  and a non-null `ai_confidence_class`.
- A preview whose resolution is approximate renders with
  `freshness_class = approximate_best_effort`,
  `representation_label_class = approximate_preview`, and
  `mapping_quality_class ∈ {near_match_fuzzy,
  type_only_no_instance}`.

Upgrade from tooltip to hovercard to peek / full view is an
explicit `escalate_to_next_tier` promotion. The canonical target,
provider attribution, freshness, and AI-confidence are preserved
across the upgrade; the downstream surface MAY enrich the render
but MAY NOT rewrite those axes.

## 9. Redaction and privacy posture

- `redaction_class` is re-exported from ADR-0011 verbatim.
- The broker-owned redaction pass (ADR-0007) runs before bytes
  reach any persistent or exportable sink. `summary_label`,
  `age_label`, `provider_attribution_label`, and
  `unavailability_reason_label` MUST be short (max 200 chars)
  and privacy-safe; raw paths, raw URLs, raw secret material,
  raw prompt text, and raw customer-owned identifiers MUST NOT
  appear.
- Admin policy MAY narrow redaction on a preview; it MAY NOT
  widen (ADR-0008).

## 10. Denial reasons (audit boundary)

This contract reserves the following denial reasons. A surface
that violates any of these MUST emit the matching denial rather
than silently fall back to a generic preview:

- `preview_without_canonical_target_or_mapping_quality`
- `preview_missing_keyboard_route`
- `preview_missing_touch_fallback`
- `safety_critical_content_in_tooltip_only`
- `approval_instruction_in_hover_only`
- `escalation_tier_regressed_without_reason`
- `transient_surface_missing_escalation_path`
- `promote_to_generic_home_screen`
- `promote_lost_canonical_identity`
- `promote_lost_provider_attribution`
- `promote_lost_freshness_cue`
- `promote_ai_without_confidence_disclosure`
- `stale_preview_presented_as_live`
- `vanished_target_presented_as_present`
- `generated_preview_presented_as_authoritative`
- `ai_derivation_undisclosed`
- `pointer_hover_dependent_for_consequential_content`
- `transient_surface_schema_version_lagging`

## 11. Worked examples

The companion fixtures under
[`/fixtures/ux/preview_surfaces/`](../../fixtures/ux/preview_surfaces/)
cover:

1. **Routine symbol tooltip** —
   `tooltip_routine_symbol_info.json`. A `tooltip` surface,
   `content_consequence_class = routine_informational`, live
   authoritative, keyboard route via `show_details_focus_ring`,
   touch fallback via `touch_long_press`, promote action to
   canonical target.
2. **AI-assisted hovercard with confidence disclosed** —
   `hovercard_ai_assisted_confidence_disclosed.json`. A
   `hovercard` surface, `attribution_source_class = ai_assisted`,
   `ai_confidence_class = medium_confidence_disclosed`,
   `representation_label_class = ai_derivation_with_confidence`,
   promote action to a full detail sheet.
3. **Approval / safety-critical content must not be tooltip-only** —
   `peek_approval_instruction_required.json`. A `peek_panel`
   surface, `content_consequence_class =
   approval_required_instruction`, pointer-hover-independent,
   keyboard routes include `show_details_focus_ring` and
   `invoke_primary_promote`, promote target is a durable
   `review_sheet`.
4. **Vanished target preview** — `vanished_target_preview.json`.
   A `hovercard` surface whose canonical target has moved;
   `mapping_quality_class = unavailable_target_moved`,
   `stale_state_class = vanished_target_moved`,
   `unavailability_reason_label` explains the move, promote
   action is `open_history_row` pointing at the audit trail.
5. **Stale cached remote preview** —
   `stale_cached_remote_preview.json`. A `hovercard` surface,
   `freshness_class = remote_unreachable_cached`,
   `stale_state_class = stale_source_unreachable`, non-null
   `age_label`, promote action includes
   `escalate_to_next_tier` to a peek with a refresh prompt.
6. **Pin preserves canonical identity** —
   `pin_preserves_canonical_identity.json`. A
   `pinned_preview_record` minted from an AI-assisted hovercard;
   preserves `canonical_object_target_ref`,
   `attribution_source_class`, `provider_attribution_label`,
   `ai_confidence_class`, `freshness_at_pin_time`, and
   `stale_state_at_pin`; `promote_target_record.kind =
   canonical_object_target_exact`.
7. **Keyboard route reaches preview without pointer hover** —
   `keyboard_route_reaches_preview.json`. A `peek_panel`
   surface, `content_consequence_class =
   safety_critical_recovery_instruction`,
   `pointer_hover_dependent = false`,
   `keyboard_route_kinds_available` covers
   `show_details_focus_ring`, `invoke_primary_promote`,
   `escalate_next_tier`, and `open_canonical_target`; promote
   target is `open_full_detail_sheet`.

These seven examples render from one shared schema without
renaming core fields per surface and demonstrate that pinning
preserves identity, that safety-critical content is never
tooltip-only, that keyboard and touch routes reach the same
preview information, and that escalation is an explicit typed
action.

## 12. Adding a new vocabulary value

Adding a new `preview_surface_kind`, `trigger_kind`,
`content_consequence_class`, `freshness_class`,
`stale_state_class`, `mapping_quality_class`,
`representation_label_class`, `attribution_source_class`,
`promote_action_kind`, `keyboard_route_kind`, or `denial_reason`
is **additive-minor** and MUST bump
`transient_surface_schema_version`. Repurposing an existing value
is **breaking** and requires a new decision row on the launch
decision register. Every consumer surface that resolves a vocab
value it does not recognize MUST deny with
`transient_surface_schema_version_lagging` rather than silently
map to a default.

## 13. Out of scope at this revision

- Final visual styling, motion, dwell timings, and pixel-perfect
  animation behavior for tooltip, hovercard, popover, and peek
  surfaces. Those live in the Design System Style Guide and the
  UI / UX Spec; this contract is the data / semantic boundary,
  not the rendering contract.
- Analytics on preview frequency, dwell distributions, or pin
  retention SLOs. A future change MAY add structured retention
  rows to the pinned-preview contract; this revision does not.
- Full in-product docs-browser or webview previews beyond the
  `hovercard` / `peek_panel` metadata boundary. Webview embedded
  previews remain subject to the embedded-surface-boundary
  contract.
