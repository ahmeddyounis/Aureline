# Locale-Pack Contribution Proof

Canonical machine source:

- Authoring templates: [`/templates/locale-packs/`](../../templates/locale-packs/)
- Authoring manifest schema: [`/schemas/i18n/locale-pack-authoring-manifest.schema.json`](../../schemas/i18n/locale-pack-authoring-manifest.schema.json)
- Terminology governance glossary: [`/fixtures/i18n/locale-pack-contribution/terminology_glossary.json`](../../fixtures/i18n/locale-pack-contribution/terminology_glossary.json)
- Terminology glossary schema: [`/schemas/i18n/locale-pack-terminology-glossary.schema.json`](../../schemas/i18n/locale-pack-terminology-glossary.schema.json)
- Stable message-id registry: [`/fixtures/i18n/message-id-stability/registry.json`](../../fixtures/i18n/message-id-stability/registry.json)
- Contribution validator: [`/tools/i18n/validate_locale_pack/`](../../tools/i18n/validate_locale_pack/)
- Gate: [`/tools/check_locale_pack_contribution.py`](../../tools/check_locale_pack_contribution.py)
- Rejected fixtures: [`/fixtures/i18n/locale-pack-contribution/rejected/`](../../fixtures/i18n/locale-pack-contribution/rejected/)
- Human page: [`/docs/i18n/locale-pack-authoring.md`](../../docs/i18n/locale-pack-authoring.md)

## What This Proves

Localization is a delivery-grade contract, not undocumented tribal workflow.
First-party, community, and extension authors build locale packs against
published templates, a published authoring schema, and a published terminology
glossary, and a single validator holds every owner class to the same rules:

1. **Contributors do not guess.** Three working templates and one authoring schema
   show how to declare a pack, which stable ids to translate, how the fallback
   chain and compatibility range are written, and which surfaces a pack may own.
2. **Critical meaning cannot fork.** The terminology glossary fixes one canonical
   meaning for trust, policy, capability, lifecycle, recovery, evidence, and
   AI-safety vocabulary, splitting it into host-stable-locked terms (rendered from
   the host catalog, never replaced) and review-governed terms (translatable with
   the meaning preserved).
3. **The guardrails provably fire.** The validator rejects incompatible packs,
   forked or locale-tagged message ids, namespace collisions, and any replacement
   of a host-stable label — for community and extension packs exactly as for
   first-party ones.

## Same Rules For Every Owner Class

The validator selects rules from `owner_class`, but no class can opt out of the
trust-label, stable-id, or compatibility rules:

| Owner class | Translates | Stable-id rule | Host-stable labels |
| --- | --- | --- | --- |
| `first_party_pack` | host `msg:` ids | must exist in the registry | render-only |
| `community_pack` | host `msg:` ids | must exist in the registry | render-only |
| `extension_owned_pack` | its own namespace | must sit under the owned prefix; cannot redefine host ids | render-only |
| `companion_overlay_pack` | its own namespace | must sit under the owned prefix; cannot redefine host ids | render-only |

## Templates Validate Clean

Each shipped template is a complete, valid pack a contributor can copy:

| Template | Owner class | Locale | Coverage | Result |
| --- | --- | --- | --- | --- |
| `first-party` | `first_party_pack` | fr-FR | complete | 0 errors |
| `community` | `community_pack` | ja-JP | partial (disclosed fallback) | 0 errors, 1 warning |
| `extension-owned` | `extension_owned_pack` | es-MX | own namespace | 0 errors |

The community template's single warning is the disclosed-fallback posture: host
ids it has not yet translated fall back to the source language. That is allowed
and expected; partial coverage is a warning, never an error.

## Rejections Provably Fire

Each rejected fixture carries an `expected.json` of finding codes; the gate fails
if any expected code is not raised, so the guardrails cannot silently regress.

| Fixture | Violation | Finding code(s) |
| --- | --- | --- |
| `host-label-override` | `may_override_host_stable_labels` true; writes under `host.trust.` | `manifest.override_host_stable_labels`, `strings.host_stable_namespace_replacement` |
| `namespace-collision` | extension reserves `host.policy.` | `manifest.namespace_collides_host`, `strings.host_stable_namespace_replacement` |
| `forked-message-id` | unknown host id; locale-tagged id | `strings.unknown_host_id`, `strings.id_carries_locale_tag` |
| `forbidden-term` | glossary localizes `host.trust.verified` | `glossary.translates_host_stable_locked` |
| `incompatible-range` | inverted build range; complete-coverage claim with gaps | `manifest.compat_range_inverted`, `coverage.incomplete_but_claimed_complete` |

## How To Regenerate And Verify

```sh
# Full gate: glossary validates, templates pass, rejected fixtures are rejected.
python3 tools/check_locale_pack_contribution.py

# Write the machine-readable capture.
python3 tools/check_locale_pack_contribution.py \
  --report artifacts/i18n/m5-locale-contribution-proof/capture.json

# Validate a single pack or the glossary.
python3 -m tools.i18n.validate_locale_pack templates/locale-packs/first-party
python3 -m tools.i18n.validate_locale_pack --check-glossary
```
