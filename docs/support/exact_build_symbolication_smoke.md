# Exact-build symbolication smoke path

This document proves one minimal crash-symbolication path using the
same exact-build fields the release artifacts already publish. It is a
local smoke path, not a hosted crash pipeline and not a production
symbol service.

Companion artifacts:

- [`/fixtures/support/crash_fixture/`](../../fixtures/support/crash_fixture/)
  — synthetic crash envelope, dump manifest, input manifest, mismatch
  case, and expected report.
- [`/tools/support/symbolicate_smoke.sh`](../../tools/support/symbolicate_smoke.sh)
  — fail-closed local runner.
- [`/artifacts/support/crash_artifact_retention_seed.json`](../../artifacts/support/crash_artifact_retention_seed.json)
  — shared crash-artifact retention and redaction seed.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  — exact-build identity source of truth.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  — support/export packet contract and artifact-manifest semantics.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` §5.41, §5.43, §10.13, §10.15, and §10.22.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §23.66 and §23.67.

## What this proves

The smoke path exercises four claims on one reviewer-followable route:

1. a crash envelope can carry the same exact-build fields the release
   artifacts already publish;
2. native symbols, renderer source maps, and crash-symbol archives can
   all be joined back to that one exact-build identity;
3. a mismatch is an explicit failure, not a silent best-effort fallback;
   and
4. crash-envelope, dump/core, symbolication-report, and support-bundle
   references reuse one shared retention and redaction seed.

## Fixture layout

The happy-path corpus lives under
[`/fixtures/support/crash_fixture/`](../../fixtures/support/crash_fixture/):

- `crash_envelope.json` carries `primary_exact_build_identity_ref` plus
  a copied subset of release fields:
  `workspace_version`, `release_channel_class`, `commit.full_hash`,
  `commit.tree_hash`, `toolchain.toolchain_pin_digest`,
  `toolchain.lockfile_digest`, `target.target_triple`,
  `build_epoch.source_date_epoch`, and `producer_lane.lane_class`.
- `symbolication_input_manifest.json` points at the canonical
  exact-build identity fixtures for the runtime binary, debug-symbols
  sidecar, source-map sidecar, and crash-symbol archive.
- `crash_dump_manifest.json` gives the raw dump/core artifact a
  metadata-only manifest with the same primary exact-build ref and
  support-bundle join.
- `expected_symbolication_report.json` is the deterministic output from
  the smoke runner.
- `crash_envelope_build_mismatch.json` intentionally diverges on
  `commit.full_hash` and must fail closed.

The crash fixture deliberately reuses the canonical exact-build corpus
under
[`/fixtures/build/exact_build_examples/`](../../fixtures/build/exact_build_examples/)
instead of minting a second build-identity dialect.

## Reviewer workflow

Happy path:

```sh
./tools/support/symbolicate_smoke.sh --out-dir target/symbolication-smoke
diff -u fixtures/support/crash_fixture/expected_symbolication_report.json target/symbolication-smoke/symbolication_report.json
```

Explicit mismatch path:

```sh
./tools/support/symbolicate_smoke.sh \
  --crash-envelope fixtures/support/crash_fixture/crash_envelope_build_mismatch.json \
  --out-dir target/symbolication-smoke-mismatch
```

The second command must exit non-zero and name the mismatched
exact-build field.

## Fail-closed checks

The smoke runner rejects the path immediately when any of these checks
fail:

- the crash envelope's `primary_exact_build_identity_ref` does not
  resolve to the runtime exact-build identity;
- any copied release field in `exact_build_snapshot` differs from the
  runtime identity;
- the debug-symbols, source-map, or crash-symbol sidecar identities do
  not match the runtime build tuple on the same copied release fields;
- the runtime identity does not point at the expected
  `split_symbols_ref` or `source_map_manifest_ref`;
- a module's build-id or source-map digest does not match the sidecar's
  declared symbol tag; or
- the dump manifest and support-bundle reference drift away from the
  same primary exact-build identity.

This is the minimal seed for the UI-spec requirement that build-ID
mismatches, stale source maps, and unavailable debug data remain
inspectable with stable labels rather than collapsing into one generic
failure.

## Retention and redaction seed

[`/artifacts/support/crash_artifact_retention_seed.json`](../../artifacts/support/crash_artifact_retention_seed.json)
freezes four reusable crash-artifact classes:

| Artifact class | Record class | Default data class | Default redaction | Default storage mode |
|---|---|---|---|---|
| `crash_envelope` | `crash_diagnostic_payload` | `metadata_only` | `metadata_safe_default` | `embedded_export_copy` |
| `crash_dump_or_core` | `crash_diagnostic_payload` | `high_risk` | `internal_support_restricted` | `local_only_copy_retained` |
| `symbolication_report` | `crash_diagnostic_payload` | `code_adjacent` | `operator_only_restricted` | `embedded_export_copy` |
| `support_bundle_reference` | `support_bundle_archive` | `metadata_only` | `metadata_safe_default` | `managed_reference_only` |

The smoke report embeds these rows by stable id so support/export and
release-evidence consumers can cite the same crash-artifact classes
without case-local side metadata.

## Scope boundary

Out of scope here:

- crash upload transport;
- hosted symbol or source-map services;
- production-quality minidump parsing; and
- automatic report redaction beyond the seeded row-level defaults.

The value of this smoke path is narrower and more important at this
stage: it proves one exact-build, one module-identity vocabulary, one
retention seed, and one explicit failure mode.
