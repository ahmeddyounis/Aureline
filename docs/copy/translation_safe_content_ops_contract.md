# Translation-safe content-ops, placeholder, and late-copy gate contract

Status: seeded

This contract freezes the copy-operations layer that keeps source
language, localization, screenshots, demos, exports, support packets,
and after-freeze copy changes aligned with product truth. It does not
replace the localization or reviewed-pack contracts. It narrows them
for content that is easy to corrupt when prose moves through
translation, screenshot capture, docs/help packaging, onboarding, and
support workflows.

The machine-readable boundary lives at:

- [`/schemas/ux/message_placeholder.schema.json`](../../schemas/ux/message_placeholder.schema.json)
  - one `message_placeholder_record` per launch-critical or
    translation-sensitive message template.
- [`/schemas/copy/late_copy_change.schema.json`](../../schemas/copy/late_copy_change.schema.json)
  - one `controlled_late_copy_change_record` per trust, legal, policy,
    recovery, support, evidence, or caption change after string freeze.

Worked examples live under:

- [`/fixtures/copy/placeholder_and_late_copy_cases/`](../../fixtures/copy/placeholder_and_late_copy_cases/)

## Companion contracts

This contract composes with:

- [`/docs/ux/localization_and_locale_pack_contract.md`](../ux/localization_and_locale_pack_contract.md)
  for message ids, locale packs, fallback origins, source-language
  escape hatches, translator-note classes, and broad localization
  denial reasons.
- [`/docs/i18n/locale_input_readiness.md`](../i18n/locale_input_readiness.md)
  for pseudoloc, text expansion, RTL/bidi, CJK, IME, dead-key, AltGr,
  compose, emoji, and truncation-readiness rules.
- [`/docs/docs/reviewed_pack_and_late_copy_policy.md`](../docs/reviewed_pack_and_late_copy_policy.md)
  for reviewed source packs, protected release-bearing surfaces,
  late-copy reason classes, reviewer requirements, and reversal rules.
- [`/docs/copy/naming_and_state_label_contract.md`](./naming_and_state_label_contract.md)
  for controlled state labels, reason chips, client-scope labels, and
  glossary ownership.
- [`/docs/copy/count_scope_freshness_grammar.md`](./count_scope_freshness_grammar.md)
  for count, scope, freshness, omission, chronology, and export-copy
  grammar.
- [`/docs/accessibility/locale_fallback_and_copy_representation_contract.md`](../accessibility/locale_fallback_and_copy_representation_contract.md)
  for accessibility-facing locale fallback rows and raw / rendered /
  escaped copy representations.

If one of those contracts owns a canonical id, stable command id,
policy id, analytics key, docs anchor, glossary term, source-language
fallback class, or reviewed-pack binding state, that contract wins. This
document owns the content-operations gate that proves those identities
survive translation and late-copy handling.

## Scope

In scope:

- placeholder semantics for translatable messages, including
  reordering, pluralization, mixed-direction text, and literal technical
  token fidelity;
- translator-note preservation, including which notes are mandatory for
  placeholders, glossary terms, screenshot/demo captions, pseudoloc /
  truncation review, source-language fallback, and late-copy deltas;
- stable glossary-term references for state labels, policy labels,
  recovery terms, evidence terms, export headings, docs/help snippets,
  onboarding steps, and screenshot captions;
- screenshot and demo caption metadata that ties captured media back to
  product strings, source-message refs, capture build, caption sync
  state, and translator notes;
- source-language fallback rules for trust, policy, recovery,
  source-of-evidence, docs/help, onboarding, and export/report surfaces;
- pseudoloc, truncation, and source-language review gates for
  launch-critical strings;
- controlled late-copy packets for trust, legal, policy, recovery,
  support, evidence, and caption copy after string freeze; and
- downstream-surface impact declarations so docs/help, onboarding,
  exports, support tooling, screenshots, demos, and locale packs do not
  silently diverge.

Out of scope:

- operating a translation platform;
- shipping translated content;
- final CMS or screenshot-capture automation;
- generating locale packs; and
- runtime UI implementation.

## Record families

### `message_placeholder_record`

A `message_placeholder_record` is the copy-ops gate for a message
template before it enters translation, pseudoloc, docs/help packaging,
caption capture, or export/report heading publication.

Every record carries:

- stable message identity and surface family;
- source-language locale and source template;
- placeholder descriptors with token fidelity, reordering, plural-rule,
  and bidi-isolation requirements;
- translator notes and preservation state;
- glossary term refs, tied to the controlled glossary or docs citation
  model;
- screenshot/demo caption metadata when the string appears in captured
  media;
- source-language fallback policy and escape hatches;
- machine-binding posture proving localized prose is not used as a
  business-logic key, analytics key, automation route, or support-tool
  selector;
- review gates for pseudoloc, truncation, source-language review,
  placeholder semantics, translator-note preservation, glossary review,
  and caption sync; and
- downstream surfaces affected by the template.

### `controlled_late_copy_change_record`

A `controlled_late_copy_change_record` is required when governed text
changes after string freeze and the text appears on trust, legal,
policy, recovery, support, evidence, screenshot/demo, docs/help,
onboarding, export/report, CLI/help, or public-proof surfaces.

Every record carries:

- packet identity and status;
- string-freeze baseline;
- timing class;
- owner;
- reason class;
- required reviewers;
- affected messages;
- delta scope, including prior and new text refs;
- downstream surfaces affected;
- verification notes;
- source-language and translator-update posture;
- linked claim, compatibility, docs, policy, or glossary refs where
  meaning changes;
- rollback or reversal route; and
- invariants that keep machine identity outside localized prose.

## Surface families

`content_ops_surface_family` is closed at these values:

| Surface family | Intent |
|---|---|
| `launch_critical_string` | Core shell, command, trust, policy, recovery, install, update, or destructive-action text whose meaning must not soften in translation. |
| `docs_help_snippet` | Docs browser, help, service-health, support, or learning prose excerpt carried as a snippet rather than a full docs body. |
| `onboarding_surface` | First-run, guided-tour, glossary, contextual teaching, or account / local-entry copy. |
| `export_report_heading` | Report headings, support bundle headings, evidence export labels, release rows, and CSV / JSON companion labels. |
| `screenshot_demo_caption` | Captions, subtitles, voice-over text, alt text, or presentation copy paired with captured product media. |
| `trust_policy_recovery_text` | Trust prompts, legal disclosures, policy blocks, recovery banners, safety denials, and sign-in fallback instructions. |
| `evidence_bearing_content` | Copy that frames evidence, freshness, supportability, public-proof, diagnostics, or audit export truth. |
| `pseudoloc_test_string` | Test-only string for pseudoloc, RTL, bidi, expansion, or truncation harnesses. |

Rules:

1. Every message picks exactly one surface family.
2. `trust_policy_recovery_text` and `evidence_bearing_content` MUST
   carry source-language fallback and source-language review gates.
3. `screenshot_demo_caption` MUST carry caption metadata and a caption
   sync gate.
4. `pseudoloc_test_string` MUST NOT render in production locale packs.
5. `export_report_heading` MAY localize the human label, but MUST keep
   the canonical export field id or report-column id locale-neutral.

## Machine identity rule

Localized prose is never a hidden source of product behavior.

The `machine_binding` block records the stable ids the product reads:
command ids, policy ids, analytics keys, automation route ids, support
tool field ids, export field ids, and docs/source anchors. The
localized string is allowed to explain those ids to a human. It is not
allowed to become one of those ids.

Rules:

1. `localized_prose_used_for_machine_binding` MUST be `false`.
2. Analytics, automation, support, policy, and business-logic keys MUST
   appear only as locale-neutral ids or typed placeholder tokens.
3. A translation that changes a command id, analytics key, automation
   route, policy id, support-tool selector, export field id, or docs
   anchor is denied with
   `content_ops_localized_prose_used_as_machine_identity`.
4. A support export MAY include translated human labels only beside the
   locale-neutral source ids.

## Placeholder semantics

`placeholder_kind` is closed at these values:

- `count`
- `command_id_token`
- `analytics_key_token`
- `automation_route_token`
- `support_field_token`
- `policy_id_token`
- `file_path_token`
- `host_or_url_token`
- `tenant_or_account_token`
- `flag_or_argument_token`
- `version_or_build_token`
- `locale_tag_token`
- `glossary_term_token`
- `enumerated_state_token`
- `evidence_ref_token`
- `freeform_string`

`token_fidelity_class` is closed at these values:

- `literal_unchanged`
- `controlled_vocabulary_translation`
- `locale_formatted_value`
- `human_translatable`

Rules:

1. Translators MAY reorder placeholders by placeholder id.
2. Translators MUST NOT paraphrase or normalize placeholders with
   `token_fidelity_class = literal_unchanged`.
3. `count` placeholders MUST declare a plural-rule ref and MAY use
   `locale_formatted_value`.
4. `enumerated_state_token` and `glossary_term_token` placeholders
   MUST resolve through controlled glossary refs, not translator-local
   synonyms.
5. Placeholders that can appear inside RTL or mixed-direction prose and
   carry literal technical tokens MUST set `bidi_isolation_required =
   true`.
6. Concatenating translated fragments around hidden tokens is
   non-conforming. The full template must expose all placeholders by id.

## Translator notes and glossary refs

Translator notes are review artifacts, not optional comments.

`translator_note_class` is closed at these values:

- `placeholder_semantics`
- `pluralization_rule`
- `mixed_direction_token`
- `glossary_term_ref`
- `screenshot_or_demo_caption_governance`
- `source_language_escape_hatch`
- `pseudoloc_truncation_review`
- `late_copy_controlled_delta`
- `policy_or_legal_review_required`
- `evidence_source_review`

Rules:

1. Any record with placeholders MUST carry a
   `placeholder_semantics` translator note.
2. Any `count` placeholder MUST carry a `pluralization_rule` note or a
   linked plural-rule review ref.
3. Any literal technical token that is bidi-isolated SHOULD carry a
   `mixed_direction_token` note.
4. Any glossary term MUST carry a stable `glossary_term_ref`; a
   translated synonym cannot replace the glossary ref.
5. Translator notes marked `required_for_translation = true` MUST
   preserve `preservation_state = preserved` in the translation packet.
6. If a translator note is dropped from a packet, the string is denied
   with `content_ops_translator_note_dropped`.

## Screenshot and demo captions

Screenshot and demo captions are governed product strings. They are not
free-form marketing copy when they describe a live product state.

Caption metadata records:

- the captured asset ref and asset kind;
- the source message ids displayed in or around the capture;
- the caption text message id;
- capture build identity;
- captured surface ref;
- locale used at capture time;
- caption sync state;
- whether product UI text and caption text were reviewed together; and
- the governance decision row or reviewed source ref.

Rules:

1. A screenshot/demo caption MUST point at the source product message ids
   it describes.
2. Caption text and product UI text MUST pass string-freeze and
   translator-note review together.
3. A stale caption may remain in an internal review fixture, but it
   cannot be published as launch evidence.
4. A caption changed after string freeze MUST have a controlled
   late-copy packet if it carries trust, policy, recovery, evidence, or
   compatibility meaning.

## Review gates

Review gates are explicit records attached to the message. A launch
gate is not satisfied by an informal statement in a pull request.

| Gate kind | Required for | Blocking failure |
|---|---|---|
| `placeholder_semantics` | Any placeholder-rich message | Missing literal-token fidelity, missing plural rule, or hidden concatenation. |
| `translator_note_preservation` | Any record with required translator notes | Required note was removed or rewritten without review. |
| `pseudoloc_expansion` | Launch-critical strings, docs/help snippets, onboarding, export/report headings, captions | Expansion causes overlap, clipping, inaccessible text, or hidden controls. |
| `truncation_review` | Launch-critical strings, trust/policy/recovery text, onboarding, export/report headings | Full-text route missing or truncation hides scope, owner, action, or consequence. |
| `source_language_review` | Trust, legal, policy, recovery, evidence-bearing content, docs/help snippets | Source-language fallback unavailable or not disclosed. |
| `glossary_term_review` | Any state, policy, recovery, evidence, support, or glossary term | Term lacks stable glossary ref or drifts from reserved meaning. |
| `screenshot_caption_sync` | Screenshot/demo captions | Caption source, capture build, or product message refs are stale or absent. |
| `machine_identity_review` | Any surface with command, analytics, support, policy, automation, export, or docs anchors | Localized prose controls routing, analytics, policy, automation, support, or export identity. |

Rules:

1. A gate with `required_for_launch = true` and `gate_status = failed`
   blocks launch or publication of the affected surface.
2. A failed gate MUST name a denial reason and a repair action.
3. Pseudoloc and truncation gates MUST use the expansion thresholds and
   full-text route rules from the locale/input readiness baseline.
4. Source-language gates MUST declare the active fallback chain and the
   source-language escape hatches.

## Source-language fallback

Source-language fallback is a continuity path, not a way to hide missing
translations.

Rules:

1. Trust, legal, policy, recovery, evidence-bearing, docs/help,
   onboarding, and export/report surfaces MUST keep a source-language
   route available unless an explicit policy block says why it is
   unavailable.
2. The fallback chain MUST start at the requested locale and terminate
   at the source language.
3. Non-authoritative fallback MUST be disclosed to the reviewer and to
   assistive technology where the affected text appears.
4. Trust, policy, and recovery copy MUST keep the source-language
   wording inspectable in the same review flow where the localized or
   fallback text appears.
5. If source-language fallback is blocked by policy, the record MUST
   name the policy ref, disabled reason, and support/export posture.

## Controlled late-copy changes

Controlled late copy is required when all of these are true:

1. the change happens after string freeze;
2. the text affects trust, legal, policy, recovery, support, evidence,
   screenshot/demo, compatibility, export/report, docs/help, onboarding,
   CLI/help, or public-proof meaning; and
3. a downstream surface has already bound to the prior wording or to a
   reviewed source pack.

Late copy may correct or narrow meaning. It may not widen a claim unless
the owning claim, compatibility, policy, or evidence row widens in the
same change and the packet links that source.

Rules:

1. Every controlled late-copy packet MUST name an owner, reason class,
   freeze baseline, affected messages, affected downstream surfaces,
   delta scope, verification notes, required reviewers, and rollback or
   reversal route.
2. The packet MUST identify whether the change affects translation,
   screenshots/demos, docs/help, onboarding, export/report headings,
   support tooling, automation routes, analytics keys, source-language
   fallback, or locale-neutral machine output.
3. The packet MUST say whether translators receive a new note, a
   revised glossary ref, a source-language fallback instruction, or a
   retranslation block.
4. A surface may bind to active late copy only while the packet is
   `approved` or `applied`.
5. Once a successor reviewed source lands, the packet is superseded or
   reversed and downstream surfaces rebind to reviewed source.

## Invariants

Every conforming message and late-copy packet preserves these facts:

1. `localized_prose_never_controls_machine_identity`
2. `placeholder_tokens_resolve_by_id_not_position`
3. `literal_technical_tokens_preserve_spelling`
4. `count_placeholders_declare_plural_rules`
5. `mixed_direction_literal_tokens_are_isolated`
6. `required_translator_notes_are_preserved`
7. `glossary_terms_use_stable_refs`
8. `screenshot_captions_bind_to_source_messages`
9. `source_language_fallback_is_declared_and_disclosed`
10. `launch_critical_copy_passes_pseudoloc_and_truncation_gates`
11. `late_copy_after_freeze_has_owner_verification_and_reversal`
12. `downstream_surfaces_are_named_before_copy_changes_publish`

## Denial reasons

The following denial reasons are reserved:

- `content_ops_localized_prose_used_as_machine_identity`
- `content_ops_placeholder_missing_plural_rule`
- `content_ops_placeholder_token_fidelity_violation`
- `content_ops_mixed_direction_token_not_isolated`
- `content_ops_translator_note_dropped`
- `content_ops_glossary_ref_missing`
- `content_ops_screenshot_caption_metadata_missing`
- `content_ops_screenshot_caption_stale`
- `content_ops_source_language_fallback_missing`
- `content_ops_source_language_fallback_not_disclosed`
- `content_ops_pseudoloc_gate_failed`
- `content_ops_truncation_gate_failed`
- `content_ops_late_copy_uncontrolled_after_freeze`
- `content_ops_late_copy_missing_downstream_surface`
- `content_ops_late_copy_missing_verification`
- `content_ops_late_copy_widens_claim_without_source_update`

## Acceptance mapping

| Acceptance clause | Resolved by |
|---|---|
| Localized prose cannot become the hidden source of business logic, analytics keys, automation routing, or support tooling. | Machine identity rule, `machine_binding`, invariant 1, and `content_ops_localized_prose_used_as_machine_identity`. |
| Placeholder-rich strings remain valid under reordering, pluralization, and mixed-direction text without losing technical token fidelity. | Placeholder semantics, translator-note rules, review gates, invariants 2 through 6, and fixture coverage. |
| Fixtures cover translator-note preservation, screenshot caption metadata, truncation review failure, controlled late-copy change, and source-language fallback. | Fixture corpus in `/fixtures/copy/placeholder_and_late_copy_cases/`. |

## Worked examples

The fixture corpus covers:

1. `translator_note_preservation.yaml` - a placeholder-rich docs/help
   snippet whose required translator notes survive translation review.
2. `screenshot_caption_metadata.yaml` - a screenshot caption bound to
   source product messages, capture build, caption sync state, and
   governance notes.
3. `truncation_review_failure.yaml` - a launch-critical recovery string
   that fails truncation review and is denied before publication.
4. `controlled_late_copy_change.yaml` - an after-freeze policy/recovery
   wording correction with owner, reviewers, verification notes, delta
   scope, and downstream surfaces.
5. `source_language_fallback.yaml` - trust/recovery copy falling back to
   source language with disclosure and source-language escape hatches.

## Adding vocabulary

Adding a new surface family, placeholder kind, token fidelity class,
translator note class, gate kind, gate status, caption sync state,
source-language escape hatch, late-copy reason class, downstream surface
class, or denial reason is additive-minor and bumps the relevant schema
version const. Repurposing an existing value is breaking and requires a
new decision row.
