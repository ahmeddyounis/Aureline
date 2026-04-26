# Localization, locale-pack, translation-governance, and source-language fallback contract

This document freezes the cross-surface contract every Aureline
**translatable string**, **command label**, **settings / help / error
text**, **docs / tour / auth surface**, **extension-contributed UI
string**, **CLI help / usage line**, **export / report heading**,
**screenshot / demo caption**, **glossary or terminology term**, and
**trust / legal / policy / recovery copy** resolves through before the
shell starts substituting prose. The goal is a localization surface
that stays a **late string-substitution layer over stable identity**
— stable message ids, stable command ids, stable docs anchors, stable
machine output — instead of an unwitnessed string-replacement pass
that breaks command routing, citation truth, supportability, or
analytics keys.

The machine-readable schemas live at:

- [`/schemas/ux/message_catalog_entry.schema.json`](../../schemas/ux/message_catalog_entry.schema.json)
- [`/schemas/ux/locale_pack_manifest.schema.json`](../../schemas/ux/locale_pack_manifest.schema.json)
- [`/schemas/ux/locale_fallback_state.schema.json`](../../schemas/ux/locale_fallback_state.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/localization_cases/`](../../fixtures/ux/localization_cases/)

This contract is normative for the projection, fallback, signature,
mirrorability, source-language, and translation-governance posture of
all localized surfaces. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, design-system style guide, or milestone document, those
sources win and this document plus its companion schemas and fixtures
update in the same change. Where a downstream surface mints a parallel
locale, message-id, fallback, or string-freeze vocabulary, this
contract wins and the surface is non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/i18n/locale_input_readiness.md`](../i18n/locale_input_readiness.md)
  and
  [`/artifacts/i18n/test_mode_matrix.yaml`](../../artifacts/i18n/test_mode_matrix.yaml)
  — readiness rows, phase posture, and design-rule ids for pseudoloc,
  RTL/bidi, CJK, IME, dead-key, AltGr, compose, and emoji. The locale
  input/readiness baseline owns shell text-shaping and input-fidelity
  rules; this contract binds the **string identity, locale-pack, and
  translation-governance** lane that rides on top.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  — canonical `command_id` and `canonical_verb` shape. Every
  command-label catalog entry resolves through one stable command id;
  no localization step may invent or mutate the canonical verb.
- [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md)
  and
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  — `docs_pack_ref`, `docs_pack_revision_ref`, `bcp47_locale_tag`,
  `freshness_class`, `version_match_state`, `source_class`,
  `reuse_surface`, and the glossary / onboarding anchor classes. Every
  docs-anchored translation resolves through these anchors instead of
  a parallel localization-only id space.
- [`/docs/ux/learnability_contract.md`](./learnability_contract.md)
  and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  — guided-surface citation posture and `locale_fallback_disclosed`
  flag. This contract names the locale-fallback record the guided
  surface points at; the guided-surface contract names which
  surface kinds must disclose the fallback.
- [`/docs/ux/no_account_local_entry_contract.md`](./no_account_local_entry_contract.md)
  and
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)
  — portable profile / device / account / policy / session scope
  vocabulary. Locale preference, dismissed source-language toggles,
  and accepted unsigned packs ride that contract's portability lanes;
  this contract does not mint a parallel locale-only portability lane.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary. Locale-pack admissibility
  (built-in vs. mirrored vs. community vs. air-gapped) resolves
  against deployment profiles mechanically.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — `redaction_class` and `policy_context` shape for surface records.

## Who reads this contract

- **Shell, command-palette, settings, error, and help authors** — to
  emit translatable strings against one stable message-id model whose
  placeholder semantics, glossary refs, and machine-output posture are
  declared.
- **Docs, tour, glossary, onboarding, and AI-explanation authors** —
  to bind translated prose to the same docs-citation anchors and
  command ids the rest of the product reads, with explicit fallback
  disclosure when the requested locale is partial.
- **Extension authors** — to contribute locale overlays without
  overriding canonical command ids, keybinding paths, or
  screen-reader labels.
- **CLI, exporter, and report authors** — to emit translated human
  text alongside locale-neutral machine output, never as a substitute
  for it.
- **Trust, legal, policy, and recovery copy authors** — to publish
  late-copy and controlled-delta review for safety-critical strings
  past string freeze.
- **Support, admin-envelope, and policy authors** — to suppress or
  source-language-fallback packs that fail signature, drift outside
  compatibility, or are policy-disabled.

## Why this exists

Without this contract, localization drifts the fastest:

- a translated command label becomes the de facto identifier for a
  command, so a renamed-or-merged button silently routes the user
  somewhere else and the canonical `command_id` analytics event no
  longer matches what the user clicked;
- a partially translated docs pack renders as if every page were
  translated, so a reviewer cannot tell whether a glossary card came
  from the requested locale, the base locale, or a fallback;
- an export header is translated and downstream tooling that keyed off
  the heading silently breaks;
- a CLI flag description gets translated *into* the JSON output of
  `--format=json` and machine consumers parse the human prose;
- a locale pack with a failed signature is rendered anyway because
  rendering "something" felt better than rendering source language;
- an extension locale overlay overrides the host's canonical command
  label and the keybinding doc no longer matches what users see;
- a screenshot caption in marketing material drifts behind the live
  product copy with no governance trail;
- a policy or recovery banner is rewritten after string freeze without
  a controlled-delta review and the safety-critical wording softens
  into a generic status word;
- a glossary term gets paraphrased per-translator and the same
  technical term means three different things across docs, tours, and
  errors;
- pseudoloc test strings ship into a production locale and confuse
  end users.

This contract closes those gaps by declaring one record per
translatable string, one record per locale pack, one record per
locale-fallback presentation event, one closed denial-reason set the
publisher MUST emit when any of the above fails, and one explicit
binding to the command-registry, citation-anchor, and locale-input
readiness truth model so localized prose stays a late substitution
layer over stable identity.

## 1. Record kinds

### 1.1 `message_catalog_entry_record`

One structured record per **(message_id, source_language)** tuple.
Emitted by the catalog owner (host shell, docs pack, extension
overlay, CLI, exporter, screenshot governance pipeline). Every record
carries:

- **identity** — `message_id` (opaque, stable across translation
  passes) and `message_id_class` in
  `{stable_canonical, extension_overlay, derived_with_upstream_id,
  pseudoloc_test_only}`.
- **surface** — `surface_family` in the closed set in §2.
- **source-language wording** — `source_text` (short label, source
  language only, used for translator review and pseudoloc seed; not a
  rendering source-of-truth at runtime).
- **placeholders** — `placeholders[]` typed by `placeholder_kind` in
  the closed set in §3.2 (count, command_id_token, file_path_token,
  host_or_url_token, tenant_or_account_token, flag_or_argument_token,
  policy_owner_token, version_or_build_token, locale_tag_token,
  enumerated_state_token, freeform_string).
- **translator notes** — `translator_notes[]` typed by
  `translator_note_class` in the closed set in §6 (placeholder
  semantics, glossary term ref, late-copy / string-freeze review,
  source-language escape hatch, pseudoloc / truncation review,
  screenshot governance, policy or legal review).
- **glossary refs** — `glossary_term_refs[]` of opaque ids resolving
  against `glossary_term_anchor` records on the docs citation model.
- **machine-output posture** — `machine_output_locale_class` in the
  closed set in §4 (`locale_neutral_canonical`,
  `locale_neutral_with_translated_human_field`,
  `locale_native_human_only`, `forbidden_for_machine_output`).
- **command identity** — `command_id_ref` non-null when
  `surface_family = command_label`.
- **extension namespace** — `extension_namespace_ref` non-null when
  `surface_family = extension_contributed_ui` or
  `message_id_class = extension_overlay`.
- **string-freeze state** — `string_freeze_state` in
  `{pre_freeze, frozen, late_copy_controlled_delta, frozen_after_review}`
  (§6).
- **escape hatches** — `source_language_escape_hatches[]` declaring
  which routes are admissible for this surface (§7).
- **review flags** — booleans for `pseudoloc_required`,
  `truncation_review_required`, `policy_or_legal_review_required`,
  `screenshot_or_demo_caption_governance_required`.
- **deployment / policy / redaction** — `deployment_profile_refs`,
  `policy_context`, `redaction_class`.
- **target build** — `target_build_identity_ref`.
- **timestamps** — `minted_at` (monotonic UTC).

### 1.2 `locale_pack_manifest_record`

One record per **(pack_id, pack_revision)**. Names the locale,
coverage, base-locale fallback chain, source language, distribution
class, signature state, mirrorability, compatibility range to the
running build, the surface families covered (and which are partial),
the extension overlay packs riding it, and the deployment profiles
the pack is admissible under. The shell MUST resolve every
`locale_fallback_state_record` against one or more
`locale_pack_manifest_record`s and reject any pack that violates its
admissibility row.

### 1.3 `locale_fallback_state_record`

One record per **presentation event** in which a localized surface
must disclose its locale lineage. Emitted whenever the requested
locale was not authoritative — the requested pack was partial, the
base locale stepped in, the source language fell through, the pack
signature failed, the pack was missing on the device, or the policy
bundle disabled the locale. Carries the requested locale, the
effective locale, the fallback chain that was walked, the pack refs
consulted with their signature states, the disclosure flag, the
source-language escape hatches that remain available, and the
command-id preservation state across the fallback.

## 2. Surface families

`message_surface_family` is closed at eleven values:

| Family                                | Intent                                                                                                                                          |
|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| `shell_chrome`                        | Shell title-bar, status, palette label, switcher chrome, focus mode label.                                                                       |
| `command_label`                       | Visible label on a command-palette, menu, or button row. MUST bind to a `command_id_ref`; the canonical id, NOT the label, routes the action.  |
| `settings_help_or_error`              | Settings labels, descriptions, help body, error explanations, denial-reason prose.                                                              |
| `docs_tour_or_auth_text`              | Docs pages, tour steps, auth flow text, account opt-in cards.                                                                                   |
| `extension_contributed_ui`            | Extension-contributed UI strings. Namespaced; MUST NOT override host-owned canonical command ids or keybinding paths.                            |
| `cli_help_text`                       | CLI help, usage, flag descriptions, error explanations on the terminal lane.                                                                    |
| `export_or_report_heading`            | Headings, column captions, and human-readable framing in exported reports, support bundles, hosted-review evidence, and offboarding exports.    |
| `screenshot_or_demo_caption`          | Captions, voice-over scripts, presentation-mode subtitles paired with a product screenshot, recording, or demo.                                 |
| `glossary_or_terminology_term`        | A single glossary term and its definition. MUST cite a `glossary_term_anchor` on the docs citation model.                                       |
| `policy_legal_or_recovery_text`       | Trust prompts, legal disclosures, policy explanations, recovery banners, sign-in fallbacks, safety-critical denial banners.                      |
| `pseudoloc_only_test_string`          | Strings minted only for pseudoloc / RTL / bidi / IME stress harnesses. MUST NOT render in production locales.                                    |

Rules (frozen):

1. **Every translatable surface picks exactly one family.** A
   `message_catalog_entry_record` whose `surface_family` is null is
   non-conforming.
2. **Command identity is not inferred from translated labels.** A
   `surface_family = command_label` row MUST carry a non-null
   `command_id_ref`. A `command_label` row whose `command_id_ref` is
   null is non-conforming
   (`denial_reason = localization_command_id_inferred_from_translated_label`).
3. **Extension overlays are namespaced.** A
   `surface_family = extension_contributed_ui` row MUST carry a
   non-null `extension_namespace_ref`, and the row MUST NOT carry a
   `command_id_ref` whose canonical command lives outside the
   extension's declared namespace
   (`denial_reason = localization_extension_overlay_overrides_canonical_command_label`).
4. **Glossary terms cite the docs anchor.** A
   `surface_family = glossary_or_terminology_term` row MUST carry at
   least one `glossary_term_refs` entry; a paraphrased glossary term
   without that ref is non-conforming
   (`denial_reason = localization_glossary_term_paraphrased_without_ref`).
5. **Pseudoloc test strings stay in test mode.** A
   `surface_family = pseudoloc_only_test_string` row MUST carry
   `message_id_class = pseudoloc_test_only` and
   `pseudoloc_required = true`; a pseudoloc string rendered in a
   production locale is non-conforming
   (`denial_reason = localization_pseudoloc_string_rendered_in_production`).

## 3. Message ids and placeholder semantics

### 3.1 Stable message ids

`message_id_class` is closed at four values:

- `stable_canonical` — frozen on first publication. Rename is a
  breaking change and requires a new decision row plus a deprecation
  alias on the catalog. The runtime resolves translations through this
  id; translated text is keyed *to* the id, not the source wording.
- `extension_overlay` — contributed by an extension. The id is
  namespaced under the extension's declared `extension_namespace_ref`
  and never collides with a host-owned id.
- `derived_with_upstream_id` — derived from a parent id (e.g., a
  generated reference label that expands a canonical command label).
  The derived id MUST point upstream so a diff still resolves to the
  parent.
- `pseudoloc_test_only` — minted by the pseudoloc / RTL / bidi
  harness; never resolves in a production catalog.

Rules:

1. **Ids never collide.** A catalog that emits two
   `message_catalog_entry_record`s with the same `message_id` is
   non-conforming
   (`denial_reason = localization_message_id_collision`).
2. **Ids survive translation.** Translation MUST NOT introduce a new
   id; it associates a translated string with an existing id.
3. **Renames go through deprecation.** Renaming a `stable_canonical`
   id requires a deprecation alias and a controlled rollout; silently
   replacing the id is breaking.

### 3.2 Placeholder kinds

`placeholder_kind` is closed at eleven values. Every placeholder a
translator may reorder, plus every placeholder a translator MUST NOT
paraphrase, is named explicitly:

- `count` — pluralization-aware integer.
- `command_id_token` — preserves a canonical command id literal.
- `file_path_token` — preserves a raw file path literal.
- `host_or_url_token` — preserves a hostname / URL literal.
- `tenant_or_account_token` — preserves an identity-critical literal
  (tenant id, account handle).
- `flag_or_argument_token` — preserves a CLI flag literal.
- `policy_owner_token` — preserves a policy-owner literal.
- `version_or_build_token` — preserves a version / build identity
  literal.
- `locale_tag_token` — preserves a BCP-47 literal.
- `enumerated_state_token` — preserves a controlled state label
  (`Policy blocked`, `Read-only degraded`, `Rollback available`); the
  translator picks the locale-correct rendering of the controlled
  vocabulary, not a free paraphrase.
- `freeform_string` — translatable subordinate text (a friendly
  surface name, an arbitrary user-supplied string).

Rules (frozen):

1. **Reorder yes, paraphrase no.** Translators MAY reorder
   placeholders to honour locale syntax; they MUST NOT paraphrase or
   silently normalize a placeholder whose `placeholder_kind` is one of
   `command_id_token`, `file_path_token`, `host_or_url_token`,
   `tenant_or_account_token`, `flag_or_argument_token`,
   `policy_owner_token`, `version_or_build_token`, or
   `locale_tag_token`. Their literal spelling matters to debugging,
   security review, and support
   (`denial_reason = localization_placeholder_semantics_violated`).
2. **Enumerated state translation is controlled.** Translation of
   `enumerated_state_token` rows MUST resolve through the central
   review for that controlled vocabulary; ad-hoc per-translator
   substitution is non-conforming.
3. **Counts must declare plural rules.** A `count` placeholder
   without a documented plural-rule resolution path is non-conforming.

## 4. Machine output and locale-neutrality

`machine_output_locale_class` is closed at four values:

- `locale_neutral_canonical` — the value MUST appear only in
  locale-neutral surfaces (JSON keys, command ids, log keys, machine
  APIs, support-bundle field names, schema enum values). It MUST NOT
  be translated; if a translator produced a translation for it, the
  translation MUST NOT render in machine output.
- `locale_neutral_with_translated_human_field` — the canonical value
  is locale-neutral, but the surface MAY carry an additional
  human-language field with a translation. Machine consumers key off
  the canonical id; humans read the translated field. JSON output
  keeps the canonical id stable across locales.
- `locale_native_human_only` — the value is human prose and is
  translated. It MUST NOT appear inside a locale-neutral export, log
  key, or machine-readable field.
- `forbidden_for_machine_output` — the message MUST NOT appear in any
  machine-readable output (e.g., a long policy explanation rendered
  only in the GUI banner).

Rules (frozen):

1. **Translation never enters locale-neutral output.** A
   message whose `machine_output_locale_class = locale_neutral_canonical`
   that is rendered translated into a JSON, log, or schema field is
   non-conforming
   (`denial_reason = localization_translation_in_locale_neutral_output`).
2. **CLI translates human prose, not flags.** A `cli_help_text` row
   MAY translate the description, MUST preserve the canonical
   `flag_or_argument_token` placeholders, and MUST emit
   locale-neutral machine output when the user requests
   `--format=json` or equivalent.
3. **Reports keep canonical headings under a translatable cap.** An
   `export_or_report_heading` row MAY publish a translated label
   alongside the canonical heading id; the canonical id stays stable
   across locales so downstream tooling does not key off prose.

## 5. Locale-pack manifest fields

A `locale_pack_manifest_record` declares:

- **identity** — `pack_id`, `pack_revision_ref`, `locale` (BCP-47),
  `coverage_locales[]` (additional BCP-47 tags the pack is admissible
  under), `source_language_locale` (BCP-47).
- **fallback chain** — `base_locale_fallback_chain[]`, an ordered
  list of BCP-47 tags consulted in order when a message id is missing
  from the requested locale. The chain MUST end at the source
  language; chains that omit the source language are non-conforming.
- **distribution class** — one of
  `built_in_with_product`, `mirrored_official_pack`,
  `community_supplied_pack`, `extension_overlay_pack`,
  `air_gapped_offline_pack`.
- **signature state** — one of
  `signed_verified`, `signed_unverified`,
  `unsigned_explicit_acceptance`, `signature_failed_blocked`,
  `not_applicable_built_in`. A pack with
  `signature_failed_blocked` MUST NOT render messages; consumers
  source-language-fallback.
- **mirrorability class** — one of
  `mirror_allowed`, `mirror_with_attribution_required`,
  `mirror_forbidden`, `air_gapped_only`,
  `not_mirrorable_signed_blob`.
- **compatibility class** — `version_match_state` value re-exported
  from the citation contract (`exact_build_match`,
  `compatible_minor_drift`, `incompatible_drift_detected`,
  `pre_release_unverified`, `unknown_target_build`).
- **compatibility build range** — `min_build_identity_ref`,
  `max_build_identity_ref`. A pack outside its declared range
  resolves with `incompatible_drift_detected` and falls through
  unless explicitly accepted.
- **covered surface families** — `covered_surface_families[]` from
  §2; partial coverage names which families are *partial* in
  `partially_translated_surface_families[]`, forcing a fallback
  disclosure for the gaps.
- **extension overlay packs** — `extension_overlay_pack_refs[]`,
  opaque ids of locale overlay packs riding this pack.
- **deployment / policy / redaction** —
  `permitted_deployment_profiles[]`, `policy_context`,
  `redaction_class`.
- **timestamps** — `minted_at`.

Rules (frozen):

1. **Built-in packs do not need signatures.** A pack whose
   `distribution_class = built_in_with_product` MUST carry
   `signature_state = not_applicable_built_in`; any other state is
   non-conforming.
2. **Air-gapped packs resolve mirrorability.** A pack whose
   `distribution_class = air_gapped_offline_pack` MUST carry
   `mirrorability_class ∈ {mirror_allowed, mirror_with_attribution_required, air_gapped_only}`.
3. **Signature failures suppress.** A pack whose
   `signature_state = signature_failed_blocked` MUST NOT render
   messages; the consumer source-language-falls-through and emits a
   `locale_fallback_state_record` with
   `fallback_origin_class = pack_signature_failed_source_language_only`
   (`denial_reason = localization_locale_pack_signature_failed`).
4. **Compatibility drift suppresses.** A pack whose
   `compatibility_class = incompatible_drift_detected` MUST NOT render
   without an explicit accept-and-acknowledge route; rendering
   silently is non-conforming
   (`denial_reason = localization_locale_pack_compatibility_drift`).
5. **Fallback chain ends at source language.** A pack whose
   `base_locale_fallback_chain` does not terminate in the
   `source_language_locale` is non-conforming.

## 6. Translation governance, late copy, and string freeze

`string_freeze_state` is closed at four values:

- `pre_freeze` — the message is mutable; translators may not commit
  yet.
- `frozen` — the message is locked; translation may proceed.
- `late_copy_controlled_delta` — the message changed after freeze
  through a controlled-delta review; translators receive the diff with
  an explicit translator note.
- `frozen_after_review` — the message changed after freeze and a
  policy / legal / safety review approved the new wording; the change
  is locked again.

`translator_note_class` is closed at seven values:

- `placeholder_semantics` — explains placeholder kinds, ordering
  freedom, and forbidden paraphrases.
- `glossary_term_ref` — names the glossary term anchor a translator
  MUST consult before rendering.
- `string_freeze_late_copy_review` — names the controlled-delta
  decision row and the review window.
- `source_language_escape_hatch` — names the escape hatch route the
  surface offers for users who want to read the source-language
  wording.
- `pseudoloc_truncation_review` — names the pseudoloc and truncation
  review the surface MUST pass.
- `screenshot_or_demo_caption_governance` — names the caption
  governance row the screenshot or demo asset rides on.
- `policy_or_legal_review_required` — flags the message for
  trust / legal / policy / recovery review on every translation.

Rules (frozen):

1. **Policy / legal / recovery text stays governed.** A
   `surface_family = policy_legal_or_recovery_text` row MUST carry
   `policy_or_legal_review_required = true` and
   `string_freeze_state ∈ {frozen, frozen_after_review, late_copy_controlled_delta}`.
   A late-copy edit without a controlled-delta review is non-conforming
   (`denial_reason = localization_policy_or_legal_text_translated_without_review`).
2. **Late-copy deltas leave a trail.** A
   `string_freeze_state = late_copy_controlled_delta` row MUST carry
   at least one `translator_note_class = string_freeze_late_copy_review`
   note; an unreviewed late delta is non-conforming
   (`denial_reason = localization_late_copy_change_after_string_freeze_unreviewed`).
3. **Glossary terms preserve identity.** A row that renders a
   glossary term MUST resolve through the term's
   `glossary_term_anchor`; per-translator paraphrases that deviate
   from the term are non-conforming
   (`denial_reason = localization_glossary_term_paraphrased_without_ref`).
4. **Screenshot / demo captions are governed with strings.** A
   `surface_family = screenshot_or_demo_caption` row MUST carry
   `screenshot_or_demo_caption_governance_required = true` and at
   least one `translator_note_class = screenshot_or_demo_caption_governance`
   note. A caption rewritten outside that review is non-conforming
   (`denial_reason = localization_screenshot_caption_lacks_governance`).
5. **Citations, keyboard paths, screen-reader labels, and
   locale-neutral output stay preserved.** Translation MUST NOT
   substitute a translated keybinding, a translated screen-reader
   label whose role-description vocabulary diverges from the
   accessibility packet, or a translated locale-neutral output value.
   The citation contract's anchor ids, the keybinding-resolver
   contract's resolver ids, the accessibility-packet
   role-description vocabulary, and the locale-neutral machine output
   schema are translation-immune.

## 7. Source-language fallback and escape hatches

`source_language_escape_hatch_class` is closed at five values:

- `inline_source_language_toggle` — a per-row affordance that
  reveals the source-language wording inline (typical for docs cards,
  glossary terms, error explanations).
- `command_open_in_source_language` — a registry-backed command that
  opens the surface in the source language (typical for docs panes,
  tour steps).
- `docs_pane_source_language_route` — a docs-pane specific route
  that switches to the source language for a single page.
- `cli_locale_neutral_output_flag` — `--format=json` or equivalent
  flag that returns locale-neutral output regardless of CLI prose
  locale.
- `export_in_source_language_for_review` — exports / reports MAY be
  produced in the source language for review (typical for audit and
  hosted-review evidence).

Rules (frozen):

1. **Every catalog row declares its admissible escape hatches.** A
   `message_catalog_entry_record` whose `surface_family` is one of
   `docs_tour_or_auth_text`, `policy_legal_or_recovery_text`,
   `cli_help_text`, `export_or_report_heading`, or
   `glossary_or_terminology_term` MUST list at least one
   `source_language_escape_hatches[]` entry; an empty list is
   non-conforming
   (`denial_reason = localization_source_language_escape_hatch_missing`).
2. **CLI machine output keeps locale-neutral as an escape hatch.**
   A `cli_help_text` row MUST list
   `cli_locale_neutral_output_flag` among its escape hatches.
3. **Auth and policy stay reviewable in the source language.**
   `policy_legal_or_recovery_text` and `docs_tour_or_auth_text` rows
   MUST list at least one of `inline_source_language_toggle`,
   `command_open_in_source_language`, or
   `docs_pane_source_language_route`.

`locale_fallback_origin_class` is closed at seven values:

- `requested_locale_authoritative` — the requested locale fully
  covers the surface; no fallback occurred.
- `requested_locale_partial_with_base_fill` — the requested locale
  was partial; the base locale filled the gap. Disclosure required.
- `base_locale_fallback` — the requested locale was missing; the
  base locale stepped in. Disclosure required.
- `source_language_fallback` — the chain walked to the source
  language. Disclosure required.
- `pack_signature_failed_source_language_only` — the requested
  pack failed signature verification; only source language is
  rendered. Disclosure required.
- `pack_missing_source_language_only` — the requested pack is not
  installed locally. Disclosure required.
- `policy_disabled_source_language_only` — the active policy bundle
  disabled non-source locales. Disclosure required.

`degraded_localization_state` is closed at six values:

- `fully_localized` — only valid for
  `requested_locale_authoritative`.
- `partial_translation_disclosed` — partial coverage with
  inline disclosure.
- `mixed_locale_strict_separation` — multiple locales render side by
  side (technical content + base-locale framing); strict separation
  preserved.
- `glossary_only_localized` — only glossary terms are localized;
  surrounding prose stays in source language.
- `source_language_with_pseudoloc` — source-language rendering with
  pseudoloc decorators applied (test-mode rendering).
- `failed_pack_source_language_only` — pack failure or missing pack;
  source language only.

Rules (frozen):

1. **Every non-authoritative origin discloses to the reviewer.** A
   `locale_fallback_state_record` whose
   `fallback_origin_class != requested_locale_authoritative` MUST
   carry `disclosed_to_reviewer = true`; silent fallback is
   non-conforming
   (`denial_reason = localization_fallback_chain_not_disclosed`).
2. **Effective locale matches origin.** Origin classes whose name
   ends in `_source_language_only` MUST set the
   `effective_locale` to the source language.
3. **Command identity survives fallback.** Every fallback record MUST
   carry `command_id_preservation_state` and MUST set it to
   `command_id_unchanged_across_fallback` for any surface that
   teaches a command; a fallback that drifts the canonical id is
   blocked
   (`denial_reason = localization_command_id_inferred_from_translated_label`).

## 8. Invariants

Every `locale_pack_manifest_record` and every
`message_catalog_entry_record` set MUST be reconcilable against the
following const-true invariants:

1. `every_message_carries_stable_id` — no record has a null
   `message_id`; ids are unique within a catalog scope.
2. `command_id_never_resolved_from_translated_label` — every
   `command_label` row carries a non-null `command_id_ref` and the
   runtime resolves commands by id, never by translated label.
3. `locale_neutral_output_never_translated` — no
   `locale_neutral_canonical` value is rendered translated in
   machine output.
4. `placeholder_semantics_preserved_across_translations` — no
   non-`freeform_string` placeholder is paraphrased or normalized.
5. `fallback_chain_is_disclosed_to_reviewer` — every non-authoritative
   fallback carries `disclosed_to_reviewer = true`.
6. `source_language_escape_hatch_always_available` — every
   admissible surface family lists at least one escape hatch.
7. `policy_legal_recovery_text_governed_after_string_freeze` — every
   policy / legal / recovery row carries
   `policy_or_legal_review_required = true` and a frozen state.
8. `glossary_terms_resolve_through_term_ref` — every glossary row
   carries at least one `glossary_term_refs` entry.
9. `screenshot_and_demo_captions_governed_with_product_strings` —
   every screenshot / demo caption row is reviewed under the same
   string-freeze and translator-note governance as the underlying
   product copy.
10. `extension_overlay_never_overrides_command_id_or_keybinding` — no
    extension overlay row mutates a host-owned canonical command id,
    canonical alias, or keybinding-resolver entry.
11. `pseudoloc_strings_never_render_in_production_locale` — no
    `pseudoloc_test_only` row resolves under a production locale.
12. `signed_or_explicit_acceptance_required_before_pack_renders` — no
    pack with `signature_failed_blocked` renders messages; a
    `signed_unverified` or `unsigned_explicit_acceptance` pack
    renders only after the deployment-profile-permitted acceptance
    route runs.

## 9. Denial reasons

The following denial reasons are reserved. A shell that would
otherwise emit non-conforming behaviour MUST deny with the matching
reason rather than silently fall back:

- `localization_message_id_collision`
- `localization_command_id_inferred_from_translated_label`
- `localization_locale_pack_signature_failed`
- `localization_locale_pack_compatibility_drift`
- `localization_translation_in_locale_neutral_output`
- `localization_fallback_chain_not_disclosed`
- `localization_source_language_escape_hatch_missing`
- `localization_screenshot_caption_lacks_governance`
- `localization_glossary_term_paraphrased_without_ref`
- `localization_policy_or_legal_text_translated_without_review`
- `localization_late_copy_change_after_string_freeze_unreviewed`
- `localization_extension_overlay_overrides_canonical_command_label`
- `localization_pseudoloc_string_rendered_in_production`
- `localization_placeholder_semantics_violated`
- `localization_message_catalog_schema_version_lagging`

## 10. Acceptance mapping

| Acceptance clause                                                                                                                                | Resolved by                                                                                                                                                                   |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Localized prose cannot become the hidden source of command routing, policy evaluation, analytics keys, or machine-readable output.              | §2 rule 2, §4 rules 1–3, invariants 2 / 3 / 10, denial reasons `localization_command_id_inferred_from_translated_label`, `localization_translation_in_locale_neutral_output`. |
| Reviewers can tell whether a string came from requested locale, base locale, source language, or missing/failed pack fallback.                  | §1.3, §7 fallback origin and disclosed-to-reviewer rules, invariant 5, fixture set in `/fixtures/ux/localization_cases/`.                                                     |
| Fixtures cover at least: partially translated docs-pack, extension locale overlay, locale-pack signature failure, source-language fallback, and translated CLI/help with locale-neutral JSON. | `/fixtures/ux/localization_cases/` (§13).                                                                                                                                     |

## 11. Adding a new vocabulary value

Adding a new `message_surface_family`, `message_id_class`,
`placeholder_kind`, `translator_note_class`,
`machine_output_locale_class`, `string_freeze_state`,
`source_language_escape_hatch_class`,
`locale_fallback_origin_class`, `degraded_localization_state`,
`locale_pack_distribution_class`, `locale_pack_signature_state`,
`locale_pack_mirrorability_class`, or `denial_reason` is
**additive-minor** and bumps `message_catalog_schema_version`.
Repurposing an existing value is **breaking** and requires a new
decision row on the launch decision register. A consumer surface that
resolves a value it does not recognize MUST deny with
`localization_message_catalog_schema_version_lagging` rather than
silently map to a default.

## 12. Cross-references

- Pseudoloc, RTL/bidi, CJK, IME, dead-key, AltGr, compose, and emoji
  rendering rules: `/docs/i18n/locale_input_readiness.md` and
  `/artifacts/i18n/test_mode_matrix.yaml`.
- Stable command identity and CLI projection:
  `/docs/commands/command_descriptor_contract.md` and
  `/schemas/commands/command_registry_entry.schema.json`.
- Docs-pack revision, locale, freshness, and version-match posture:
  `/docs/docs_integrity/citation_and_reference_contract.md` and
  `/schemas/docs/citation_anchor.schema.json`.
- Guided-surface citation and `locale_fallback_disclosed`:
  `/docs/ux/learnability_contract.md` and
  `/schemas/ux/guided_surface_state.schema.json`.
- Onboarding-portability lanes:
  `/docs/ux/no_account_local_entry_contract.md` and
  `/schemas/ux/onboarding_portability_state.schema.json`.

## 13. Worked examples

Fixtures under
[`/fixtures/ux/localization_cases/`](../../fixtures/ux/localization_cases/)
cover the acceptance set:

1. **Partially translated docs-pack** —
   `partially_translated_docs_pack.yaml`. A
   `locale_fallback_state_record` for a docs surface where the
   requested `pt-BR` locale covers `docs_tour_or_auth_text` but not
   `glossary_or_terminology_term`; the missing terms render from the
   base `en-US` locale with disclosure.
2. **Extension locale overlay** —
   `extension_locale_overlay_for_extension_command_label.yaml`. A
   `message_catalog_entry_record` for an extension-namespaced command
   label, with a paired `locale_pack_manifest_record` declaring the
   overlay distribution class. The host's canonical command id is
   untouched.
3. **Locale-pack signature failure** —
   `locale_pack_signature_failed.yaml`. A
   `locale_pack_manifest_record` whose `signature_state` is
   `signature_failed_blocked`, paired with a
   `locale_fallback_state_record` whose
   `fallback_origin_class = pack_signature_failed_source_language_only`
   and `degraded_localization_state = failed_pack_source_language_only`.
4. **Source-language fallback** —
   `source_language_fallback_for_recovery_banner.yaml`. A
   safety-critical recovery banner whose requested `ja-JP` pack is
   missing; falls through to the source language `en-US` with
   `command_open_in_source_language` escape hatch active.
5. **Translated CLI help with locale-neutral JSON** —
   `translated_cli_help_with_locale_neutral_json.yaml`. A
   `cli_help_text` `message_catalog_entry_record` with a translated
   description and preserved `flag_or_argument_token` placeholders,
   paired with a `locale_fallback_state_record` confirming
   `cli_locale_neutral_output_flag` keeps the JSON output canonical.
6. **Built-in canonical command label** —
   `command_label_built_in_canonical.yaml`. A
   `surface_family = command_label` `message_catalog_entry_record`
   that pins the canonical `command_id_ref`, demonstrates the
   `frozen_after_review` state, and lists glossary refs for the
   verb's terminology.
7. **Locale-pack manifest, base / mirrored / signed pack** —
   `locale_pack_manifest_mirrored_signed.yaml`. A
   `locale_pack_manifest_record` for `pt-BR` mirrored from upstream,
   `signed_verified`, `mirror_with_attribution_required`,
   covering `command_label`, `settings_help_or_error`, and
   `cli_help_text` with `glossary_or_terminology_term` declared
   partial.

## 14. Out of scope at this revision

- **Shipping translated content.** This contract freezes the
  boundary; the actual translated catalog rows, locale packs, and
  community-pack distribution land in later milestones and ride this
  contract.
- **Community-pack infrastructure.** The
  `community_supplied_pack` distribution class is reserved; the
  pack-signing trust root, community submission UX, and dispute
  process are owned by the pack-distribution lane and land later.
- **Translator workflow tooling.** Translation memory, glossary
  synchronization, machine-translation review, and continuous-locale
  delivery pipelines are owned by the localization-operations lane
  and ride this contract's vocabulary.
- **Final visuals.** The design-system style guide owns
  pseudoloc-aware padding, RTL mirroring, truncation indicators, and
  fallback chip composition. This contract names the typed posture.
- **Per-platform input fidelity.** IME, dead-key, AltGr, compose, and
  emoji-input fidelity is owned by
  `/docs/i18n/locale_input_readiness.md` and the platform input
  matrix; this contract names the locale-pack and message-id model.
