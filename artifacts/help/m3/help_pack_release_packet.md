# Help-Pack Release Packet

## Scope

This packet publishes the beta onboarding/help-pack contract used by Start Center, migration center, Help search, contextual why-now cards, recovery-first help, and support export.

## Published Artifacts

| Artifact | Path |
| --- | --- |
| Schema | `schemas/help/onboarding_help_pack.schema.json` |
| Runtime model | `crates/aureline-shell/src/help_packs/onboarding_beta.rs` |
| Headless inspector | `crates/aureline-shell/src/bin/aureline_shell_onboarding_help_pack_beta.rs` |
| Manifest fixture | `fixtures/help/m3/onboarding_help_packs/manifest.json` |
| Surface projection fixture | `fixtures/help/m3/onboarding_help_packs/surface_projection.json` |
| Support export fixture | `fixtures/help/m3/onboarding_help_packs/support_export.json` |
| Contract docs | `docs/help/m3/onboarding_help_pack_beta.md` |

## Release Truth

- Source, version-match, freshness, and client-scope badges reuse the docs/help release-truth families.
- Registry-backed command hints pin command revision refs.
- Pack-owned support export guidance still uses a stable `cmd:` ID and action-graph ref.
- Locale fallback rows record requested locale, effective locale, fallback chain, source-language item ref, and unresolved fallback flags.
- Mirror/offline rows disclose verified mirror snapshots, local-only packs, cached snapshots, and unavailable states explicitly.

## Support Export

The support export is metadata-only. It records:

- active help-pack versions;
- unresolved locale and source fallback flags;
- dismissed and helpful item state refs;
- diagnostics row refs;
- exact reopen refs with pack version and locale.

It omits raw article bodies, raw Help search queries, private workspace paths, and account identifiers.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_help_pack_beta -- validate
cargo test -p aureline-shell --test onboarding_help_pack_beta_fixtures
```
