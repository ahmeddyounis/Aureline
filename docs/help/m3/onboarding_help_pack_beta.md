# Onboarding Help-Pack Beta

The beta onboarding/help pack is the shared manifest used by Start Center, migration center, Help search, contextual why-now cards, recovery-first help, and support export. It promotes the alpha pack shape into a governed asset with stable pack IDs, item IDs, glossary-pack IDs, citations, release-truth badges, locale fallback, offline/mirror posture, command references, and bounded diagnostics state.

## Contract

- Pack identity is versioned with `pack_id`, `pack_version_ref`, source class, source version, freshness, version-match state, install state, offline posture, mirror posture, source locale, available locales, and release-truth badge tokens.
- Item identity is stable across surfaces. Each item carries an `item_id`, owning pack/version, optional `glossary_pack_id`, docs-node ref, surface refs, command hint, exact reopen ref, citations, and support-export identity.
- Command hints stay aligned to stable command IDs. Registry-backed items pin `command_revision_ref`; pack-owned commands still use `cmd:` IDs and declare a canonical action-graph ref.
- Locale fallback is explicit. Requested locale, effective locale, fallback chain, stale translation marker, source-language fallback class, escape hatch command, and unresolved fallback flags are part of the row.
- Offline and mirror behavior is explicit. Local-only, cached, verified mirror, and unavailable states are represented as typed content availability and mirror posture rather than empty rows.
- Support export is metadata-only. It records active pack versions, unresolved fallback flags, dismissed/helpful item state refs, diagnostics row refs, and exact reopen refs while omitting raw article bodies, raw queries, private paths, and account identifiers.

## Runtime Consumers

The shell beta pack is implemented in `crates/aureline-shell/src/help_packs/onboarding_beta.rs`.

The onboarding projection prefers the beta pack before falling back to the older alpha manifest, so Help search rows inherit pack version, source/freshness/version badges, mirror posture, support item IDs, and state refs.

The migration center attaches help-pack refs to every claimed beta entry before building support rows, so support export can reconstruct which help item backed a docs anchor, glossary row, known-limit row, first-run exit, or recovery route.

## Fixtures

The fixture corpus under `fixtures/help/m3/onboarding_help_packs/` covers:

- source-language fallback for the command-palette migration bridge;
- verified mirror fallback for docs-browser help;
- full manifest and surface projection coverage for Start Center, migration center, Help search, contextual why-now, and recovery-first surfaces;
- support export reconstruction without raw body export.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- manifest \
  > fixtures/help/m3/onboarding_help_packs/manifest.json
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- surfaces \
  > fixtures/help/m3/onboarding_help_packs/surface_projection.json
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- support-export \
  > fixtures/help/m3/onboarding_help_packs/support_export.json
```

## Verification

```sh
cargo test -p aureline-shell --test onboarding_help_pack_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- validate
```
