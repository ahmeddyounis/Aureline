# M5 extension-host WIT capability diff report

Generated from [`artifacts/contracts/m5-wit-contract-publication.json`](m5-wit-contract-publication.json) by `tools/regenerate_m5_wit_contract_publication.py`. Do not hand-edit; edit the regenerator and re-run it.

This report lets extension authors, reviewers, and release managers see what changed between published versions of each capability-world WIT package without reverse-engineering host code. Every row is cross-checked against the typed host behaviour in `crates/aureline-extensions/src/implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts/`.

## Published versions

| Package | Version | Lifecycle | Posture | Successor |
| --- | --- | --- | --- | --- |
| `editor-read` | `0.1.0` | deprecated | reader_only | `aureline:editor-read@0.2.0` |
| `editor-read` | `0.2.0` | beta | reader_only | `—` |
| `workspace-read` | `0.1.0` | beta | reader_only | `—` |
| `diff-apply-preview` | `0.1.0` | experimental | read_write | `—` |
| `terminal-observe` | `0.1.0` | beta | reader_only | `—` |
| `network-egress` | `0.1.0` | experimental | read_write | `—` |

## Capability diffs

### `editor-read` 0.1.0 → 0.2.0 (additive_minor)

- **Compatibility verdict:** backward_compatible
- **Guest action required:** none
- **Added:**
  - `interface editor-read: func visible-range -> option<visibility-range>`
  - `interface editor-read: func word-at(position) -> result<option<string>, read-error>`
  - `interface editor-read: record visibility-range`

Additive-minor bump. Every 0.1.0 item is preserved verbatim, so a 0.1.0 guest runs unchanged on a 0.2.0 host. A 0.1.0 host narrows a 0.2.0 guest by withholding the added items, not by denying the world.

### `editor-read` 0.1.0 → 0.2.0 (deprecation)

- **Compatibility verdict:** deprecated_superseded
- **Guest action required:** upgrade_recommended

Version 0.1.0 is deprecated in favour of 0.2.0. Hosts continue to admit 0.1.0 guests but emit a deprecation notice carrying the successor identity and a repair affordance. The world slug stays active; no removal is implied by the deprecation.

## Negotiation outcomes proven by fixtures

| Outcome | Declared | Negotiated | Fails closed | Fixture |
| --- | --- | --- | --- | --- |
| supported | 3 | 3 | no | `fixtures/contracts/m5-wit-negotiation/supported.json` |
| downgraded | 3 | 1 | yes | `fixtures/contracts/m5-wit-negotiation/downgraded.json` |
| deprecated | 2 | 2 | no | `fixtures/contracts/m5-wit-negotiation/deprecated.json` |
| unsupported_skew | 2 | 1 | yes | `fixtures/contracts/m5-wit-negotiation/unsupported_skew.json` |

