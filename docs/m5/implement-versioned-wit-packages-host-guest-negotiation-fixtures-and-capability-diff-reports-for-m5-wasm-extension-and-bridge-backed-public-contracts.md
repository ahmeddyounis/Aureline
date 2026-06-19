# Versioned WIT packages, host/guest negotiation fixtures, and capability diffs

This page is the narrative companion to the canonical M5 extension-host WIT
contract publication packet. The packet promotes the reserved capability-world
WIT contracts — which became public contracts through marketplace/install,
sideload, bridge, and SDK consumption — from "exists as WIT files plus an index"
to a versioned, lifecycle-labelled, negotiation-proven, diffable product object.

- Packet (source of truth):
  [`artifacts/contracts/m5-wit-contract-publication.json`](../../artifacts/contracts/m5-wit-contract-publication.json)
- Capability-diff report:
  [`artifacts/contracts/m5-wit-capability-diff.md`](../../artifacts/contracts/m5-wit-capability-diff.md)
- Negotiation fixtures:
  [`fixtures/contracts/m5-wit-negotiation/`](../../fixtures/contracts/m5-wit-negotiation/)
- Boundary schema:
  [`schemas/public/m5-contracts/m5_wit_contract_publication.schema.json`](../../schemas/public/m5-contracts/m5_wit_contract_publication.schema.json)
- Typed consumer + proof:
  `crates/aureline-extensions/src/implement_versioned_wit_packages_host_guest_negotiation_fixtures_and_capability_diff_reports_for_m5_wasm_extension_and_bridge_backed_public_contracts/`
- Regenerator / validator:
  [`tools/regenerate_m5_wit_contract_publication.py`](../../tools/regenerate_m5_wit_contract_publication.py),
  [`tools/validate_m5_wit_contract_publication.py`](../../tools/validate_m5_wit_contract_publication.py)

It binds the `extension_host_wit_world` row of the
[M5 public-contract matrix](../../artifacts/contracts/m5-public-contract-matrix.md)
to a concrete WIT publication, so later docs/help/SDK/support/export surfaces
consume this packet instead of restating world semantics by hand.

## Versioned WIT packages

The reserved capability worlds (ADR-0019,
[`docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md`](../adr/0019-wasm-wit-extension-host-and-capability-worlds.md))
ship as versioned WIT packages. The canonical 0.1.0 worlds live under
[`wit/aureline/`](../../wit/aureline/); this row adds the first published
successor, `aureline:editor-read@0.2.0`, at
[`wit/m5-contracts/editor-read-0.2.0.wit`](../../wit/m5-contracts/editor-read-0.2.0.wit).

Each published package carries:

- its package identity (`aureline:<slug>@<semver>`), the backing `.wit` file, and
  the ADR-0019 registry row it resolves to,
- a **lifecycle label** (`stable` / `beta` / `experimental` / `deprecated` /
  `retired`) for the *version* — distinct from the world slug's registry status,
  so a version can be deprecated while the slug stays active,
- the reader/writer posture, trust-state gating posture, permission-scope
  projection, and supported host families, and
- a **compatibility note** plus predecessor/successor links.

`aureline:editor-read@0.1.0` is published as `deprecated` in favour of `0.2.0`;
the `0.2.0` package is an additive-minor bump that preserves every `0.1.0` item
verbatim and only adds `visible-range`, `word-at`, and the `visibility-range`
record.

## Host/guest negotiation fixtures

Four fixtures under `fixtures/contracts/m5-wit-negotiation/` prove the host
behaviour for one real bridge-backed family — the component-model extension host
— across the required outcomes. Each fixture is a typed host-negotiation record
whose declared/offered/negotiated world sets, narrowing reasons, unsupported-world
decisions, and deprecated-world notices are checked against the same invariants
the host enforces:

| Outcome | What it proves |
| --- | --- |
| `supported` | A trusted guest with matching ABI/vocabulary is admitted at its full declared world set; nothing widened. |
| `downgraded` | A restricted trust state narrows the blocked-in-restricted worlds with a typed reason and a repair affordance each; the read-only world survives; authority is never widened to compensate. |
| `deprecated` | A guest declaring the deprecated `editor-read@0.1.0` is admitted (deprecation is not removal) but receives an explicit successor notice and repair affordance — never a silent pass-through. |
| `unsupported_skew` | A guest whose ABI is ahead of the host has the skewed world denied with a typed `guest_abi_range_mismatch` decision and a repair affordance; the host fails closed — it denies rather than widening or silently dropping the world. |

The invariants enforced for every fixture: `negotiated ⊆ offered ⊆ declared`; no
negotiated world is absent from the declared set (no widening); every
declared-but-not-negotiated world carries exactly one disposition (narrowing
reason, unsupported decision, or admitted-with-deprecation notice) — no silent
drop; and the recorded `fail_closed` flag matches its derivation.

## Capability-diff reports

[`artifacts/contracts/m5-wit-capability-diff.md`](../../artifacts/contracts/m5-wit-capability-diff.md)
projects the packet's `capability_diffs` so authors, reviewers, and release
managers can see what changed between versions without reading host code. The
`editor-read 0.1.0 → 0.2.0` diff is classed `additive_minor` /
`backward_compatible` (adds-only, no guest action required); a paired
`deprecation` diff records the `0.1.0` sunset with its successor. The diff
invariants (additive-minor is adds-only and backward-compatible; deprecation
carries a successor; a breaking change requires a guest upgrade) are enforced by
both the validator and the typed consumer.

## Guardrails

- A family may not claim runtime compatibility or bridge parity while its host
  contract exists only as code comments or tests: this packet requires a
  published `.wit` file per published package, and the validator fails if one is
  missing.
- Guest authority is never widened by implicit defaults when negotiation fails or
  a capability descriptor is stale: `guest_authority_widened` is constrained to
  `false`, no negotiated world may exceed the declared set, and unsupported/skew
  worlds are denied fail-closed.
- Mirror/offline packaging includes the WIT packages and sample contracts (the
  `.wit` files and fixtures are checked in), not just marketplace UI metadata.

## How it is proven

`tools/validate_m5_wit_contract_publication.py` validates the packet against the
schema, validates each standalone fixture, checks no regenerator/Markdown drift,
re-derives every fixture and diff invariant, confirms the four required outcomes
are covered, and confirms every referenced path and published `.wit` file exists.
The typed consumer in `aureline-extensions` reads the same packet and fixtures and
re-derives the same invariants, so `cargo test -p aureline-extensions` enforces
them at build time. The row narrows automatically if the packet, fixtures,
schema, WIT files, or proof drift.
