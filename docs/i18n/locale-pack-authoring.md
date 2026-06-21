# Locale-Pack Authoring

This page is the authoring guide for everyone who builds an Aureline locale pack
or translated overlay: the first-party localization team, community translators,
and extension authors shipping their own strings. It exists so a contributor
never has to guess about stable message ids, fallback behavior, the
compatibility range, or which vocabulary is off-limits — and so a community or
extension pack is held to the **same** rules as a first-party one.

The same-rules-for-everyone principle is the point: a contributed pack cannot
bypass the stable-id, compatibility, and trust-label rules that govern
first-party packs.

Canonical machine source:

- Authoring templates: [`/templates/locale-packs/`](../../templates/locale-packs/)
- Authoring manifest schema: [`/schemas/i18n/locale-pack-authoring-manifest.schema.json`](../../schemas/i18n/locale-pack-authoring-manifest.schema.json)
- Terminology governance glossary: [`/fixtures/i18n/locale-pack-contribution/terminology_glossary.json`](../../fixtures/i18n/locale-pack-contribution/terminology_glossary.json) (schema: [`/schemas/i18n/locale-pack-terminology-glossary.schema.json`](../../schemas/i18n/locale-pack-terminology-glossary.schema.json))
- Stable message-id registry: [`/fixtures/i18n/message-id-stability/registry.json`](../../fixtures/i18n/message-id-stability/registry.json)
- Contribution validator: [`/tools/i18n/validate_locale_pack/`](../../tools/i18n/validate_locale_pack/)
- Contribution proof: [`/artifacts/i18n/m5-locale-contribution-proof.md`](../../artifacts/i18n/m5-locale-contribution-proof.md)

Companion contracts:

- Extension and companion localization: [`/docs/i18n/extension-and-companion-localization.md`](./extension-and-companion-localization.md)
- Localization scope and profile matrix: [`/docs/i18n/m5-localization-scope.md`](./m5-localization-scope.md)
- Attention and lifecycle vocabulary glossary: [`/docs/i18n/attention-and-lifecycle-glossary.md`](./attention-and-lifecycle-glossary.md)

## What A Locale Pack Looks Like On Disk

A pack is a directory with a manifest, one or more strings files, and an optional
glossary:

```
my-locale-pack/
  manifest.json            # pack identity, locale, fallback, compatibility, ownership
  strings/
    shell_chrome.json      # { "<stable message id>": "<localized string>" }
    command_label.json
  glossary.json            # optional: localized renderings of review-governed terms
```

Copy the closest starting point from
[`/templates/locale-packs/`](../../templates/locale-packs/):

- `first-party/` — translates host message ids, claims complete coverage of its
  owned surfaces;
- `community/` — translates host message ids with partial coverage and disclosed
  source-language fallback;
- `extension-owned/` — owns a private namespace and translates only its own ids.

Pack files carry **refs and localized prose only** — never translated bodies for
host-stable labels, signing keys, or credentials. Whether a pack actually
renders in a given build is decided at runtime by compatibility and signature
evaluation, not by the manifest on disk (see
[extension-and-companion-localization](./extension-and-companion-localization.md)).

## The Manifest

Every pack declares one `locale_pack_authoring_manifest`. The fields that decide
how the pack is governed:

- **`owner_class`** — `first_party_pack`, `community_pack`, `extension_owned_pack`,
  or `companion_overlay_pack`. This selects which rules apply (see below).
- **`locale`, `fallback_locale`, `fallback_chain`** — the fallback chain must
  start at the pack locale and end at the source language (`en-US`), so an
  untranslated key always has a defined route back to source-language truth.
- **`compatibility_build_range`** — the inclusive `min`/`max` build identities the
  pack targets. Skew is decided against this declared range, never guessed; a pack
  outside the active build degrades to source language rather than rendering stale
  strings.
- **`owned_surface_families`** — the surfaces the pack translates. A contributed
  pack may not own `policy_legal_or_recovery_text`.
- **`owned_namespace_prefix`** — required for extension and companion packs: the
  reserved id prefix the pack writes into. Omitted for first-party and community
  packs, which translate host ids directly.
- **`claims_complete_coverage`** — set `true` only when the pack translates every
  host id for its owned surfaces. A complete-coverage claim with missing keys is
  rejected.
- **`discloses_source_language_fallback`** — must be `true`.
- **`may_override_host_stable_labels`** — must be `false`.

## Stable Message Ids

Localized prose changes; identifiers do not. The
[message-id registry](../../fixtures/i18n/message-id-stability/registry.json) is
the canonical set of host message ids a first-party or community pack may
translate. The rules the validator enforces:

- **Reuse host ids verbatim.** A first-party or community strings file is keyed by
  host `msg:` ids drawn from the registry. An id that is not in the registry would
  fork the stable id set and is rejected (`strings.unknown_host_id`).
- **Never embed a locale in an id.** An id like `msg:shell:title:fr-FR` silently
  breaks continuity across locale changes and is rejected
  (`strings.id_carries_locale_tag`).
- **Extension packs own their namespace.** An extension or companion pack keys its
  strings under its declared `owned_namespace_prefix` and must not redefine host
  ids.

Command ids, setting ids, diagnostic ids, telemetry keys, and policy names live
next to the message id, never behind localized prose, so routing, analytics,
policy, and export tooling keep working when copy changes or a pack is missing.

## Coverage And Fallback

Partial coverage is fine and expected. An untranslated host id falls back to the
source language with the fallback disclosed, so a pack ships value immediately and
fills in over time. The validator reports untranslated host ids for owned surfaces
as a **warning**, not an error (`coverage.missing_keys`) — unless the manifest
claims complete coverage, in which case missing keys are an error.

## Restricted Vocabulary: Terminology Governance

Some words carry meaning the product cannot let a translation fork. The
[terminology governance glossary](../../fixtures/i18n/locale-pack-contribution/terminology_glossary.json)
governs the critical trust, policy, capability, lifecycle, recovery, evidence,
and AI-safety vocabulary. Each term has one stable `term_key`, one canonical
meaning, and one of two governance classes:

- **`host_stable_locked`** — rendered from the host catalog and **never** replaced
  or translated by a contributed pack. These live under reserved namespace
  prefixes (`host.trust.`, `host.policy.`, `host.capability.`, `host.lifecycle.`,
  `host.recovery.`, `host.ai_safety.`). A pack that writes a string or glossary
  entry under one of these prefixes, or that replaces one of these governed terms,
  is rejected. This is the same protection the runtime
  [`HostStableLabelGuard`](../../crates/aureline-i18n/src/contributed_locale.rs)
  enforces, extended to recovery, evidence, and AI-safety vocabulary.
- **`translatable_with_review`** — may be translated, but the canonical meaning and
  severity are fixed and the translation must preserve them under terminology
  review (for example, evidence `stale` must not soften to merely "old"). A pack
  localizes these in its `glossary.json`, keyed by `term_key`.

A pack glossary may localize only review-governed terms. Localizing a
host-stable-locked term is rejected (`glossary.translates_host_stable_locked`).

## Validate Before You Submit

Run the validator on your pack before opening a contribution:

```sh
python3 -m tools.i18n.validate_locale_pack path/to/my-locale-pack
```

It exits `0` when the pack has no errors (warnings are allowed) and `1` on any
error, listing each finding with a stable code and the exact location. See the
[validator README](../../tools/i18n/validate_locale_pack/) for every finding code.

To validate the governance glossary itself:

```sh
python3 -m tools.i18n.validate_locale_pack --check-glossary
```

## Verification

```sh
# Self-test: glossary validates, templates pass, rejected fixtures are rejected.
python3 tools/check_locale_pack_contribution.py

# Validate one pack (or template).
python3 -m tools.i18n.validate_locale_pack templates/locale-packs/first-party
```
