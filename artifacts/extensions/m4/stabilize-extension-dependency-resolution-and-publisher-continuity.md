# Artifact: Stabilize Extension Dependency Resolution And Publisher Continuity

**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Failure / recovery drill + Release evidence review

## Summary

This artifact introduces the stable cross-surface extension dependency-resolution packet. It binds deterministic resolver output, hard dependencies, optional integrations, API/runtime ranges, effective permission inheritance, re-consent, publisher continuity, revocation/last-known-good pinning, deprecation propagation, support export, and automatic stable-claim narrowing.

## Outputs

- Rust module: `crates/aureline-extensions/src/stabilize_extension_dependency_resolution_and_publisher_continuity/`
- Dump example: `crates/aureline-extensions/examples/dump_extension_dependency_resolution_records.rs`
- Schema: `schemas/extensions/extension-dependency-resolution.schema.json`
- Fixtures: `fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/`
- Docs: `docs/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity.md`

## Acceptance Evidence

- Public, mirrored, and enterprise-curated fixtures preserve package identity, dependency tree, effective permissions, and continuity state.
- The mirrored-update fixture proves dependency-introduced permission widening requires and records re-consent.
- The rollback fixture proves yanked metadata, last-known-good pinning, and explicit policy hold/downgrade behavior remain inspectable.
- Continuity drills cover key rotation, ownership transfer, namespace dispute, maintainer removal, orphan adoption, and approved-mirror succession.
- The packet narrows stable claims when continuity proof is pending, disputed, missing notification, in cooldown, stale, or incomplete.
- Deprecation proof flows into resolver output, install warnings, migration docs, compatibility shim state, and claim freshness.

## How To Verify

```bash
cargo test -p aureline-extensions stabilize_extension_dependency_resolution
cargo run -q -p aureline-extensions --example dump_extension_dependency_resolution_records -- validate
```

## Risks / Follow-Ups

- Resolver digests and lock/export refs are opaque evidence refs in this packet; live resolver/controller wiring will bind them to concrete runtime artifacts.
- Schema validation is structural; semantic drift is enforced in the Rust packet validation and fixture tests.
