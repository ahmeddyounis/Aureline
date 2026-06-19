# Versioned WIT packages, negotiation fixtures, and capability diffs

Evidence record for the M5 extension-host WIT contract publication: the versioned
WIT packages, host/guest negotiation fixtures, and capability-diff reports that
turn the reserved capability-world contracts into a published, diffable product
object for the `extension_host_wit_world` family of the M5 public-contract matrix.

## What shipped

- The canonical publication packet — every reserved capability-world WIT package
  as a versioned entry with lifecycle metadata and a compatibility note, the four
  negotiation fixtures, and the capability diffs:
  [`/artifacts/contracts/m5-wit-contract-publication.json`](../contracts/m5-wit-contract-publication.json).
- A published additive-minor successor WIT package:
  [`/wit/m5-contracts/editor-read-0.2.0.wit`](../../wit/m5-contracts/editor-read-0.2.0.wit),
  indexed by [`/wit/m5-contracts/README.md`](../../wit/m5-contracts/README.md).
- The capability-diff report:
  [`/artifacts/contracts/m5-wit-capability-diff.md`](../contracts/m5-wit-capability-diff.md).
- The host/guest negotiation fixtures (supported, downgraded, deprecated,
  unsupported-skew) plus an index:
  [`/fixtures/contracts/m5-wit-negotiation/`](../../fixtures/contracts/m5-wit-negotiation/).
- The boundary schema:
  [`/schemas/public/m5-contracts/m5_wit_contract_publication.schema.json`](../../schemas/public/m5-contracts/m5_wit_contract_publication.schema.json).
- The typed consumer plus its validator and support-export projection:
  `crates/aureline-extensions/src/implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts/`.
- The single source of truth (regenerator) and the validator:
  [`/tools/regenerate_m5_wit_contract_publication.py`](../../tools/regenerate_m5_wit_contract_publication.py)
  and
  [`/tools/validate_m5_wit_contract_publication.py`](../../tools/validate_m5_wit_contract_publication.py).
- The CI validation capture:
  [`/artifacts/release/captures/implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts_validation_capture.json`](../release/captures/implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts_validation_capture.json).
- The narrative companion:
  [`/docs/m5/implement-versioned-wit-packages-host-guest-negotiation-fixtures-and-capability-diff-reports-for-m5-wasm-extension-and-bridge-backed-public-contracts.md`](../../docs/m5/implement-versioned-wit-packages-host-guest-negotiation-fixtures-and-capability-diff-reports-for-m5-wasm-extension-and-bridge-backed-public-contracts.md).

## Reuse, not restatement

The packet reuses the ADR-0019 capability-world registry
(`artifacts/extensions/capability_worlds.yaml`), the host-negotiation vocabulary
(`schemas/extensions/host_negotiation.schema.json` — narrowing reasons,
unsupported-world reasons, trust-state gating postures), the M5 public-contract
matrix `extension_host_wit_world` row
(`artifacts/contracts/m5-stability-lifecycle-map.json`), and the WIT worlds under
`wit/aureline/`. It mints no new contract-status lexicon.

## Acceptance

- **Versioned WIT packages with lifecycle metadata and compatibility notes** —
  six published package entries (five 0.1.0 worlds plus the 0.2.0 successor), each
  with a lifecycle label, registry status, posture, scopes, and a compatibility
  note; one (`editor-read@0.1.0`) is published `deprecated` with a successor link.
- **Negotiation fixtures cover supported, downgraded, deprecated, and
  unsupported-skew for one real bridge-backed family** — the component-model
  extension host. Two fixtures fail closed (downgraded, unsupported-skew).
- **Capability diffs available to release/docs/SDK/help and matching host
  behaviour** — the `editor-read 0.1.0 → 0.2.0` additive-minor diff and the paired
  deprecation diff, projected to Markdown and cross-checked against the typed host
  invariants in tests.

## Proof

Automated proof lives in
`crates/aureline-extensions/src/implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts/tests.rs`
and
`crates/aureline-extensions/tests/implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts.rs`:

- the checked-in packet parses and validates with zero findings;
- the four standalone fixtures parse, validate, and equal the embedded fixtures;
- `negotiated ⊆ offered ⊆ declared`, no widening, and no silent drop hold for
  every fixture; `fail_closed` matches its derivation;
- the deprecated fixture admits the deprecated world with a successor notice;
- the unsupported-skew fixture denies the skewed world fail-closed;
- the additive-minor diff is adds-only and backward-compatible;
- negative gates reject a widened guest, a silently dropped world, a narrowed
  world left in the negotiated set, an additive-minor diff that removes a
  capability, and a deprecation diff without a successor.

`tools/validate_m5_wit_contract_publication.py` validates the JSON against the
schema, validates the standalone fixtures, checks no regenerator/Markdown drift,
re-derives the fixture and diff invariants, confirms outcome coverage, and
confirms every referenced path and published `.wit` file exists. It runs as the
dedicated `check_m5_wit_contract_publication` workflow.

## Reuse surfaces

`support_export_projection()` (Help/About, SDK/docs, support export),
`packages_for_slug(...)` / `deprecated_packages()` (release-center and SDK
inspection), `fixture_for_outcome(...)` (install-review negotiation disclosure),
and `capability_diffs_for_slug(...)` (author/reviewer diff surfaces). Part of the
canonical M5 evidence train; the row narrows if its packet, fixtures, schema, WIT
files, or proof drift.
