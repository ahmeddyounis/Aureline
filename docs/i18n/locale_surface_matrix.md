# Locale-surface matrix and localization parity contract

This document freezes **what may localize** (human prose) and **what must remain machine-stable** (identifiers, keys, anchors, flags, and other copy-safe technical tokens) on each user-visible surface. It exists so translation and locale-pack work can proceed without destabilizing command routing, schema compatibility, citations, automation, or supportability.

This is the human-readable companion to:

- `artifacts/i18n/locale_surface_matrix.yaml` — machine-readable surface rows and parity-check vocabulary.
- `fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity/manifest.json` — stable locale-pack lifecycle, fallback-chain, and translated-surface parity packet for claimed localized rows.

It composes with (non-exhaustive):

- `docs/ux/localization_and_locale_pack_contract.md` — locale-pack, message ids, surface families, and fallback governance.
- `docs/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md` — stable claim gate for signed pack windows, fallback truth, and docs/tour/auth/help/CLI parity.
- `docs/copy/translation_safe_content_ops_contract.md` — placeholder fidelity and the “localized prose never binds machine identity” rule.
- `docs/docs_integrity/citation_and_reference_contract.md` — citation anchors, locale pins, and “raw URL” avoidance.
- `docs/commands/command_descriptor_contract.md` — canonical command identity and routing.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md` — stable setting ids and effective-configuration identity.

Normative source: `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix DF (Localization, Locale Pack, and Translation Governance Matrix).

## Cross-surface invariants (always)

1. **Localized prose never routes behavior.** Command routing, policy checks, automation, support tooling, and telemetry MUST bind to locale-neutral ids/keys/anchors — never to translated labels or translated prose.
2. **Stable tokens are copy-safe and literal.** When a surface must show a stable identifier (command id, flag, schema id, citation anchor id, setting id, JSON key), it is rendered from the canonical source and preserved literally (e.g., code spans, copy buttons), not retyped or translated.
3. **Fallback is disclosed and inspectable.** If the requested locale is missing/partial/incompatible/blocked, the surface MUST disclose the effective locale and the fallback origin (base-locale fill, source-language fallback, signature failure, policy denial, etc.).
4. **Source-language access is always available.** Every surface listed below MUST provide a deterministic source-language route (inline toggle, “view original”, locale-neutral output flag, or equivalent).
5. **Machine-readable output stays locale-neutral.** Where a surface offers structured output (JSON, logs, exports), canonical field names and ids remain locale-neutral; optional translated human fields may be present beside canonical fields.
6. **Extensions cannot override host identity.** Extension-contributed UI may localize its own labels, but MUST NOT override host-owned command ids, settings ids, schema ids, telemetry keys, citation anchors, or other host-controlled stable identifiers.

## Locale-surface matrix (summary)

| Surface | Localization source | What localizes | Machine-stable elements | Fallback + disclosure | Required parity checks |
|---|---|---|---|---|---|
| Shell commands and palette | Core locale pack | command labels, palette chrome, human help snippets | command ids, keybinding paths, telemetry keys, policy ids | disclose effective locale; keep source toggle | command discoverability parity; keyboard-path parity; stable-id diff checks |
| Settings / help / errors | Core locale pack + settings/schema docs | labels, descriptions, denial/explain-why prose | setting ids, JSON keys, schema ids, error codes, policy ids | disclose fallback; copy-safe “show ids” route | help parity; schema-id stability; source-language access parity |
| Docs / tours / auth | Docs pack + locale overlay | docs prose, tour steps, auth guidance | citation anchors, docs anchors, command ids, recovery ids/URLs | disclose partial translation; “view original” | citation parity; keyboard-path parity; screen-reader parity |
| CLI / help / doctor | Core locale pack | usage/help prose, human explanations | subcommand names, flags, JSON keys, finding ids/codes | locale-neutral output escape hatch | snapshot tests; canonical machine output stability |
| Extension-contributed UI | Extension pack or companion locale pack | extension labels/help within extension namespace | host stable ids remain host-controlled; extension namespace ids | explicit compat/fallback disclosure | namespace/override checks; pack compatibility checks |

## Per-surface rules

### 1) Shell commands and palette

- **Localization source:** core locale pack entries for palette chrome and command labels.
- **Localizes:** visible command labels, palette headings, tooltips, and short help snippets.
- **Machine-stable elements:** `command_id` (and canonical verb), keybinding paths, telemetry keys, policy ids, automation route ids.
- **Fallback behavior:** missing/partial/incompatible packs fall back along the declared base-locale chain to source language; the palette discloses the effective locale and offers a source-language toggle.
- **Source-language access:** inline source-language toggle and a copy-safe “show command id” affordance.
- **Parity checks (minimum):**
  - **Command identity parity:** every localized command label binds to the same `command_id` as source language; no label-based routing.
  - **Discoverability parity:** search, grouping, and disabled-state explanation remain present and equivalent.
  - **Keyboard-path parity:** keybinding hints and shortcuts remain stable and copy-safe.

### 2) Settings / help / errors

- **Localization source:** core locale pack for UI strings; settings registry + schema/doc lanes for canonical identities and long-form help.
- **Localizes:** setting titles, summaries/descriptions, explain-why/help text, error/denial prose.
- **Machine-stable elements:** setting ids, JSON keys, schema ids, error codes, policy ids, telemetry keys.
- **Fallback behavior:** if localized help is missing, render source-language help with explicit fallback disclosure; never guess by paraphrasing or silently dropping constraints.
- **Source-language access:** a deterministic route to “view original/source”, plus copy-safe render of setting id and (where applicable) schema id.
- **Parity checks (minimum):**
  - **Setting-id stability:** setting id and JSON key surfaces remain unchanged across locales.
  - **Schema-id stability:** schema ids referenced by validation/help remain unchanged and copy-safe.
  - **Help parity:** every localized help surface retains the same constraints, scopes, and safety cues as source language.

### 3) Docs / tours / auth

- **Localization source:** docs pack bodies plus locale overlays; citations and anchors remain shared across locales.
- **Localizes:** docs prose, tutorial/tour steps, auth and onboarding guidance.
- **Machine-stable elements:** citation anchors, docs anchors, command ids in examples, recovery ids/URLs, safety/policy identifiers referenced by chips.
- **Fallback behavior:** partial docs translation MUST disclose missing coverage; anchor ids and citations remain resolvable even when prose falls back to source language.
- **Source-language access:** “view original/source” and a route to copy citation anchors without relying on localized text.
- **Parity checks (minimum):**
  - **Citation parity:** translated surfaces retain the same citation anchors and pack revision refs.
  - **Assistive-technology parity:** labels, focus order, and keyboard paths remain equivalent.
  - **Command/citation fidelity:** translated examples keep command ids and stable tokens literal.

### 4) CLI / help / doctor

- **Localization source:** core locale pack.
- **Localizes:** help/usage prose and human descriptions for options.
- **Machine-stable elements:** subcommand names, flags, exit codes, finding ids/codes, JSON keys/fields, schema ids referenced in machine output.
- **Fallback behavior:** if localization is unavailable, print source-language human prose; machine output remains stable regardless.
- **Source-language access:** a locale-neutral output flag (or format selector) that forces canonical ids/keys and stable machine output.
- **Parity checks (minimum):**
  - **Snapshot parity:** localized `--help`/usage text exists and preserves all canonical flag spellings.
  - **Machine output stability:** JSON keys and canonical ids remain invariant across locales.

### 5) Extension-contributed UI

- **Localization source:** extension-provided locale packs or companion overlay packs governed by compatibility and signature posture.
- **Localizes:** extension-owned UI strings (within the extension namespace).
- **Machine-stable elements:** host stable ids remain host-controlled (commands, settings ids, schema ids, telemetry keys, citation anchors); extension namespace ids remain stable.
- **Fallback behavior:** incompatible or missing extension locale overlays fall back to the extension’s source language (or declared base chain) with explicit disclosure; host surfaces never inherit extension locale assumptions silently.
- **Source-language access:** show extension pack provenance/compatibility and offer “view original” for extension copy when the pack is partial or blocked.
- **Parity checks (minimum):**
  - **No override:** extension overlays do not override host-owned ids or keybindings.
  - **Compatibility:** extension locale overlay declares an explicit compatibility range and fails closed with disclosure when out-of-range.

## Worked examples

See `fixtures/i18n/locale_surface_examples/` for cross-surface examples that demonstrate:

- localized prose with stable command/schema/citation identity,
- explicit fallback to source language/base locale, and
- extension overlay compatibility and host-id protection.
