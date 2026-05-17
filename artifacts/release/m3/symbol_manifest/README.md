# M3 release-side exact-build symbol manifest packet

This packet is the checked-in proof path for the exact-build symbol
manifest the alpha crash incident lane joins to a runtime crash
envelope.

| Surface | Path |
|---|---|
| Symbol manifest record | [`symbol_manifest.json`](./symbol_manifest.json) |
| Boundary schema | [`schemas/support/crash_symbolication_manifest_alpha.schema.json`](../../../../schemas/support/crash_symbolication_manifest_alpha.schema.json) |
| Crate consumer | [`crates/aureline-crash/src/envelope/mod.rs`](../../../../crates/aureline-crash/src/envelope/mod.rs) |
| Reviewer doc | [`docs/support/m3/crash_symbolication_alpha.md`](../../../../docs/support/m3/crash_symbolication_alpha.md) |
| Protected fixtures | [`fixtures/support/crash_symbolication_alpha/`](../../../../fixtures/support/crash_symbolication_alpha/) |
| Protected test | [`crates/aureline-crash/tests/crash_symbolication_alpha.rs`](../../../../crates/aureline-crash/tests/crash_symbolication_alpha.rs) |

## What this manifest declares

`symbol_manifest_record` is the release-side declaration of per-module
symbol identities tied to one exact-build identity. It pins:

- `primary_exact_build_identity_ref` — the runtime exact-build identity
  this manifest covers (mirrors the channel/workspace_version/target/profile
  axes already used by the alpha crash envelope and by `aureline-build-info`).
- One row per module (native binary or web bundle), each carrying the
  module's exact-build identity, the symbolication identity (debug-symbols
  archive or source-map family), the optional crash-symbol archive identity
  for native modules, and the build/debug id or source-map digest captured
  at release time.
- `storage_class: metadata_only_no_symbol_bytes` on every row. The manifest
  is metadata only — no dwarf tables, no symbol bytes, no source-map bodies,
  no absolute paths.

## Proof claims

| Claim | Evidence |
|---|---|
| Crash artifacts can be linked to an exact build and symbol manifest | `crash_symbolication_alpha::linked_manifest_binds_crash_envelope_to_exact_build_symbols` |
| Local users can inspect crash artifacts without raw private data leaking | `crash_symbolication_alpha::binding_is_metadata_safe_by_construction` |
| Local symbolication result, or explicit mismatch state, is preserved | `crash_symbolication_alpha::linked_manifest_binds_crash_envelope_to_exact_build_symbols`, `crash_symbolication_alpha::build_mismatch_refuses_to_claim_exact_symbolication`, `crash_symbolication_alpha::missing_manifest_keeps_envelope_refs_without_implying_coverage` |
| Release packets preserve the same build IDs and symbol references that crash support expects | `crash_symbolication_alpha::release_symbol_manifest_matches_alpha_crash_envelope_identity` |

## Redaction posture

The manifest record carries `redaction_class: metadata_safe_default`,
`raw_private_material_excluded: true`, and
`ambient_authority_excluded: true`. The crash-envelope symbol binding
that consumes it preserves the same posture and additionally pins
`raw_dump_exported: false` — raw minidump or core bodies remain governed
by `support.item.crash_dump_or_core` and stay local-only unless an
explicit reviewed upload path is approved.

## How to refresh

1. Re-run the protected test:
   `cargo test -p aureline-crash --test crash_symbolication_alpha`.
2. If a new module ships with the preview channel build, append a new
   row to `symbol_manifest.json` and re-prove the alpha crash binding.
3. When the primary exact-build identity changes (channel, workspace
   version, target, profile, or commit short), regenerate this manifest
   alongside the matching crash envelope fixture so the binding stays
   `linked` rather than `build_mismatch`.
