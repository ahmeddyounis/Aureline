# Extension identity rebinding, issue reproducibility export, and dev-loop continuity contract seed

This document freezes the contract surfaces that prevent local extension
development loops from silently rebinding trust (or switching update
channels) and that preserve diagnosability when an extension package
moves between local workspaces and registry-backed identities, or when
its source disappears.

The machine-readable boundaries introduced here are:

- `/schemas/extensions/rebind_review.schema.json`
  — review sheet record for binding a local or sideloaded package to a
  registry-backed published identity (including namespace, signer,
  version, update-channel choice, and trust-promotion disclosure).
- `/schemas/extensions/repro_export.schema.json`
  — issue-reproduction export packet record for extension failures,
  carrying stable identity, digest, ABI, conformance status, crash
  history, active permission set, and log artifact summaries.

These contracts **compose with and do not replace** existing extension
and runtime seeds:

- `/docs/extensions/local_dev_and_sideload_contract.md` and
  `/schemas/extensions/sideload_review.schema.json`
  — local dev/sideload invariants, hot-reload review triggers, and
  continuity states (`source_unavailable`, `build_failed`,
  `last_loaded_build_still_active`).
- `/docs/extensions/registry_and_offline_bundle_seed.md` and
  `/schemas/extensions/registry_manifest.schema.json`
  — registry/mirror/offline identity, signature class, trust inheritance,
  and non-widening transport rules.
- `/schemas/extensions/effective_permission.schema.json`
  — permission vocabulary and effective permission summary record.
- `/schemas/extensions/host_negotiation.schema.json`
  — host family and ABI negotiation disclosures.
- `/schemas/extensions/conformance_result.schema.json`
  — conformance/compatibility result vocabulary and evidence refs.
- `/docs/extensions/runtime_budget_packet.md`
  — activation evidence, crash-loop vocabulary, and quarantine posture.

The schemas are authoritative when the narrative and the schemas
disagree; this document MUST be updated in the same change that lands
any schema bump.

This seed is deliberately narrow. It does **not** ship a registry
service, publish service implementation, auto-generated issue filing,
or a complete authoring UX. Its job is to freeze the review and export
vocabulary early enough that later implementation lanes do not re-invent
identity/trust rebinding and reproduction artifacts ad hoc.

## Terms (portable, reviewable vocabulary)

- **Local-only package** — an extension artifact produced from a local
  workspace or sideloaded archive, not bound to a registry identity.
- **Registry-bound package** — an installed extension whose updates and
  provenance are resolved through a registry/mirror/offline-bundle
  identity, not through local source paths.
- **Rebinding** — a governed transition from local-only/sideloaded to a
  registry-bound identity *or* a governed transition that strengthens
  trust posture (signer, trust tier, or update-channel choice).
- **Repro export** — a bounded, machine-readable packet that can be
  compared mechanically across machines/environments.

## Rebinding review (Bind published identity)

### When rebinding review is required

A rebinding review MUST be required (and recorded) before either of the
following becomes true:

1. A local-only or sideloaded package becomes `registry_bound`, meaning
   that future updates resolve through a registry identity rather than a
   local source locator.
2. A package changes its trust posture in a way that could widen user
   risk, including:
   - signer/signature-class changes,
   - publisher-namespace changes,
   - update-channel choice changes (e.g. tracking a faster or higher
     trust channel),
   - host ABI or runtime-origin changes,
   - permission posture widening (manifest or effective permission set).

Acceptance invariant: **a local package cannot silently become a
registry package or silently move to a stronger-trust channel**.

### What the rebinding review MUST disclose

The review sheet MUST disclose enough information for the user/admin to
decide whether the rebinding is acceptable without reading raw manifests
or raw registry metadata:

- **Source package identity**: declared extension identity + local/sideload
  origin descriptor (redaction-safe).
- **Target published identity**: target extension identity, publisher
  namespace, signer reference, signature class, and target version.
- **Update-channel choice**: the channel the consumer will track after
  binding.
- **Trust-promotion disclosure**: prior vs next rendered trust tier,
  signature class, and any trust-inheritance rule that caps the badge.
- **Permission and runtime posture delta**: whether the binding changes
  runtime origin/ABI or widens requested/effective permissions.
- **Continuity impact**: whether the local source remains available, is
  missing, or has a build failure (and whether a last-loaded build is
  still active).

### Rebinding rules (no silent identity drift)

- Rebinding MUST be explicit and reviewable. Any automated “helpful
  upgrade” that changes identity, channel, signer, ABI, or permissions
  is treated as a rebinding attempt and MUST pause for review.
- If the binding replaces bytes (different artifact digest), the sheet
  MUST say so. A digest mismatch may be allowed only with explicit
  acknowledgement.
- A previously recorded rebinding review MUST NOT be reused when the
  manifest or ABI changes widen runtime or permission posture.

## Continuity when local source disappears or cannot rebuild

Local extension development loops MUST remain honest and diagnosable.
The dev loop uses explicit continuity states (defined in
`sideload_review.schema.json`) and does not “paper over” missing source
by silently swapping in registry bytes.

Minimum continuity behavior:

- **Source unavailable**: surface a `source_unavailable` state and retain
  refs to the last known logs/failure evidence. The shell MAY keep the
  last loaded build active if it is still running, but MUST label that
  fact explicitly.
- **Build failed**: surface a `build_failed` state and keep the last
  successfully loaded build active when possible; do not silently bind
  to registry builds as a fallback.
- **Last loaded build still active**: preserve diagnosability and avoid
  layout collapse (e.g. extension views replaced with placeholders),
  while ensuring users understand they are no longer running current
  source bytes.
- **Source moved**: path hints may change, but extension identity and
  trust posture MUST NOT change as a side effect of a source path move.

## Repro export packets (mechanically comparable failure capture)

Repro exports exist so maintainers can compare failures mechanically
instead of relying on screenshots or ad hoc log snippets.

The export packet MUST carry, at minimum:

- package identity (`extension_identity`, version),
- artifact digest (`content_address`) of the loaded extension bytes,
- host runtime family and ABI range,
- source/build availability state,
- conformance status + refs to the most relevant conformance results,
- crash history summary + refs to crash evidence,
- active permission set (effective/granted snapshot),
- log artifact summaries (digests + capture windows), never raw absolute
  paths and never raw secret material.

Redaction rules:

- Raw absolute filesystem paths, raw registry URLs, raw log bodies,
  raw crash dumps, raw stack frames, raw wasm bytes, and raw signing key
  material MUST NOT appear in the packet.
- The packet carries **refs** and **digests** so a support bundle can
  include the actual evidence under explicit inclusion/redaction policy.

