# Trust-Class Alpha Projection

This note documents the first shell-backed projection that applies the
safe-preview trust-class vocabulary across docs, preview, and package/install
review surfaces.

The implementation lives in:

```text
crates/aureline-shell/src/previews/trust_classes.rs
```

It consumes the existing `RawText`, `SanitizedRich`, `TrustedLocalActive`, and
`IsolatedRemoteActive` vocabulary from `aureline-content-safety`; it does not
mint a parallel trust model.

## Covered Lanes

- Docs/help pages resolve sanitized rendered content as `SanitizedRich` and
  keep rendered copy and sanitized export labels explicit.
- Runtime/rich previews resolve trusted local active content as
  `TrustedLocalActive` only while the local capability boundary is visible.
- Runtime/rich previews that lose isolated remote guarantees keep the nominal
  `IsolatedRemoteActive` cue, narrow the effective class to `SanitizedRich`,
  and block active-open actions while exposing a sanitized static snapshot.
- Package/install review resolves manifest and publisher text as `RawText` in
  strict review mode, keeps raw and escaped copy paths available, and requires
  preview/review before apply.

## Required Cues

Every action row emitted by the shell packet carries these disclosures before
copy, export, active-open, or mutation:

- `trust_class_badge`
- `currently_visible_representation`
- `representation_label`

Copy/export rows also name the representation class and body posture using the
existing representation-transfer vocabulary: raw, rendered, escaped,
sanitized, or blocked metadata-only.

## Fallback Rule

A degraded active preview must fail toward `raw_inspection`,
`sanitized_static_snapshot`, or `metadata_only_envelope`. It must not leave an
active-open action available while origin, connectivity, sandbox, policy, or
owner/origin chrome is missing.

## Protected Fixture

The protected fixture is:

```text
fixtures/previews/trust_class_alpha/core_surface_packet.json
```

It exercises docs, preview, and package/install lanes, covers all four trust
classes, proves raw/rendered/active state is visible before actions, and proves
remote active degradation falls back to a static sanitized mode.
