# Locale-Pack Authoring Templates

Copy-paste starting points for building an Aureline locale pack. Each template is
a complete, valid pack that passes the contribution validator, so you can copy
one, change the locale and strings, and validate as you go.

Full authoring guidance lives in
[`/docs/i18n/locale-pack-authoring.md`](../../docs/i18n/locale-pack-authoring.md).

## Which Template

| Template | Use when you are… | Translates |
| --- | --- | --- |
| [`first-party/`](./first-party/) | the first-party localization team | host `msg:` ids, complete coverage of owned surfaces |
| [`community/`](./community/) | a community translator | host `msg:` ids, partial coverage with disclosed fallback |
| [`extension-owned/`](./extension-owned/) | an extension author | your own namespaced ids |

## Pack Layout

```
<template>/
  manifest.json          # locale_pack_authoring_manifest
  strings/<surface>.json # { "<stable message id>": "<localized string>" }
  glossary.json          # optional: localized review-governed terms (first-party, community)
```

- `manifest.json` validates against
  [`/schemas/i18n/locale-pack-authoring-manifest.schema.json`](../../schemas/i18n/locale-pack-authoring-manifest.schema.json).
- First-party and community strings files are keyed by host message ids from
  [`/fixtures/i18n/message-id-stability/registry.json`](../../fixtures/i18n/message-id-stability/registry.json).
- Extension strings files are keyed under the manifest's `owned_namespace_prefix`.
- `glossary.json` may localize only `translatable_with_review` terms from the
  [terminology governance glossary](../../fixtures/i18n/locale-pack-contribution/terminology_glossary.json).

## Validate A Pack

```sh
python3 -m tools.i18n.validate_locale_pack templates/locale-packs/first-party
```

## What These Templates Are Not

These are authoring scaffolds, not a distribution format. The signed, mirrorable,
runtime-evaluated locale-pack artifact is a separate record
([`/schemas/i18n/locale-pack.schema.json`](../../schemas/i18n/locale-pack.schema.json));
whether a pack renders in a given build is decided at runtime, not by these files.
