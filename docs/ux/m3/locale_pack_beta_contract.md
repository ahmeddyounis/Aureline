# Locale-Pack Beta Contract

This page describes the runtime-facing locale-pack contract implemented in
`crates/aureline-i18n`. It composes the existing localization foundation:

- `docs/ux/localization_and_locale_pack_contract.md`
- `schemas/ux/message_catalog_entry.schema.json`
- `schemas/ux/locale_pack_manifest.schema.json`
- `schemas/ux/locale_fallback_state.schema.json`
- `schemas/i18n/locale_pack_manifest.schema.json`

## Runtime Contract

The beta contract is a single typed envelope:

- stable message ids are bound to canonical command ids, semantic action ids,
  diagnostic ids, settings ids, schema ids, telemetry keys, policy names, and
  docs-pack keys;
- locale packs carry version, signature, compatibility, source-class,
  mirrorability, offline-import, and rollback metadata;
- fallback state is inspectable as requested locale -> language base -> source
  language, with per-surface state rows for partial coverage, missing packs,
  signature failure, and source-language fallback;
- Settings, Help/About, and support exports all project the same
  `active_locale_state`;
- CLI JSON keys, Doctor finding codes, policy names, telemetry keys, command
  ids, flags, and schemas remain locale-neutral, with translated human prose
  allowed only as an overlay.

The seeded contract is generated with:

```sh
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- manifest
```

## Surface Projections

Settings renders `locale-pack:projection:settings:v1`, including active
locale, fallback chain, active pack versions, signature states, and source
language escape hatches.

Help/About renders `locale-pack:projection:help-about:v1`, using the same
active state as Settings so support can quote what the user saw.

Support export renders `support-export:locale-pack:beta:v1`. It is
metadata-only: raw translated message bodies, raw docs bodies, and raw
extension locale payloads are omitted.

## Extension Declaration

Extensions declare one of four locale support modes:

- inherit host locale behavior;
- ship their own locale pack;
- ship a companion locale pack;
- source-language only.

All extension declarations are visible, compatibility checked, and forbidden
from overriding host-owned stable identifiers.

## Verification

The fixture replay test is:

```sh
cargo test -p aureline-i18n --test locale_pack_beta
```

The test verifies fixture parity, fallback inspection across Settings and
Help/About, metadata-only support exports, extension host-id protection,
source-language fallback disclosure, signature-failure blocking, and
locale-neutral machine identifiers.

Dense pseudoloc, RTL, bidi, IME, CJK/font fallback, translated-surface parity,
and text-expansion proof is covered by:

```sh
cargo test -p aureline-i18n --test pseudoloc_rtl_ime_corpus --locked
```

That corpus lives at `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/` and exports
the review packet consumed by `docs/ux/m3/localization_conformance_beta.md`.
