# M5 runbook source register

This is the canonical inventory of Aureline's governed runbook **sources**: where
each runbook came from, the authority that provenance carries, its signer, its
freshness, its owning scope, and its export rights. It is the companion to the
[runbook governance matrix](m5-runbook-governance.md): the matrix freezes *what* a
runbook object is, and this register answers the question that comes first —
*where did the runbook come from, and is it allowed to speak with authority?*

The register is produced from one checked-in source of truth in
`crates/aureline-runbooks` (`m5_runbook_sources`). Do not hand-edit the JSON under
`artifacts/` or `fixtures/`; re-mint it with the headless emitter (below). The
contract is documented at [`docs/runbooks/m5-runbook-sources.md`](../../docs/runbooks/m5-runbook-sources.md)
and validated by [`schemas/runbooks/m5-runbook-source-register.schema.json`](../../schemas/runbooks/m5-runbook-source-register.schema.json).

## Source classes and authority postures

| Source class | Default posture | Provenance kind | Meaning |
|--------------|-----------------|-----------------|---------|
| `repo_local` | **Authoritative** | `signed_first_party` | Authored and signed in-repo as first-party governed guidance. |
| `mirrored_docs_pack` | **Mirrored** | `mirror_digest` | A verified mirror of an upstream authoritative docs pack. |
| `managed_catalog` | **Managed** | `catalog_manifest` | Published through a managed catalog under a signed manifest. |
| `browser_reference` | **Reference only** | `browser_capture` | Captured from browser-only vendor documentation; no standing authority. |

## What is derived

Each source declares its class, version, signer/provenance, freshness window,
owning scope, default posture, and export rights. The register **derives** an
effective authority posture and a per-source badge:

- A **browser reference is reference-only** unless another governed source
  promotes its step set into an authority-bearing posture (`authoritative` or
  `managed`). The promotion names the vouching source, the approver, and a
  rationale.
- A source whose freshness is **`stale` or `expired` auto-narrows** back to
  reference-only and is no longer executable until it is re-verified.
- A source is **executable** only when its effective posture is authority-bearing
  and its freshness is current. Reference-only sources are never executable, so a
  browser reference cannot silently masquerade as a first-party executable runbook.

The freshness *state* (`fresh` / `aging` / `stale` / `expired`) is computed from
the freshness window, never asserted. The validator recomputes the effective
posture, the badge, and the conformance review and compares them to the stored
values, so they can never drift from the declared sources.

## The same descriptor everywhere

The register projects one surface-independent badge per source. The docs/help
runbook browser, the incident workspace, operator dashboards, and support exports
all render the same badge, so freshness, signer/source class, and authority
posture stay visible wherever a runbook is rendered or exported.

## Fixtures

The per-source descriptors under `fixtures/runbooks/m5-source-descriptors/`
demonstrate every class:

- `repo_pipeline_restart` — repo-local, authoritative.
- `mirror_observability_pack` — mirrored docs-pack, mirrored.
- `catalog_failover` — managed-catalog, managed.
- `browser_vendor_scaling` — browser-reference, reference-only and not executable.
- `browser_promoted_dr` — browser-reference promoted into authoritative posture by
  a governed first-party source.
- `stale_mirror_narrowed_register` — a full register where the mirror has gone
  stale and auto-narrows to reference-only.

## Re-minting

All JSON and Markdown here are generated from `crates/aureline-runbooks`:

```sh
BIN="cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_sources --"
$BIN validate
$BIN register  > artifacts/runbooks/m5-runbook-source-register.json
$BIN register  > artifacts/release/m5-runbook-proof/runbook-source-register.json
$BIN markdown  > artifacts/release/m5-runbook-proof/runbook-source-register-proof.md
$BIN source src:repo-pipeline-restart      > fixtures/runbooks/m5-source-descriptors/repo_pipeline_restart.json
$BIN source src:mirror-observability-pack  > fixtures/runbooks/m5-source-descriptors/mirror_observability_pack.json
$BIN source src:catalog-failover           > fixtures/runbooks/m5-source-descriptors/catalog_failover.json
$BIN source src:browser-vendor-scaling     > fixtures/runbooks/m5-source-descriptors/browser_vendor_scaling.json
$BIN source src:browser-promoted-dr        > fixtures/runbooks/m5-source-descriptors/browser_promoted_dr.json
$BIN fixture-stale-mirror                  > fixtures/runbooks/m5-source-descriptors/stale_mirror_narrowed_register.json
```
