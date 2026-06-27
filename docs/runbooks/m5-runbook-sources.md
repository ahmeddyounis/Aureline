# Runbook source descriptors

Runbook authority depends first on **where the runbook came from**. A rendered
runbook is not equally authoritative regardless of origin, and Aureline does not
treat it that way. This document is the contract for the *source descriptor*
model: how Aureline classifies where a runbook came from, how much authority that
provenance carries, and how that truth stays visible wherever the runbook is
rendered or exported.

The crate `aureline-runbooks` (`m5_runbook_sources`) owns the model. The
machine-readable inventory lives at
[`artifacts/runbooks/m5-runbook-source-register.json`](../../artifacts/runbooks/m5-runbook-source-register.json)
(human summary:
[`artifacts/runbooks/m5-runbook-source-register.md`](../../artifacts/runbooks/m5-runbook-source-register.md)),
and the schema is
[`schemas/runbooks/m5-runbook-source-register.schema.json`](../../schemas/runbooks/m5-runbook-source-register.schema.json).

## The four source classes

Every runbook source declares one **provenance class**, and each class carries a
default **authority posture** — the word a user or support reader sees:

| Source class | Default posture | Meaning |
|--------------|-----------------|---------|
| `repo_local` | **Authoritative** | Authored and signed in-repo as first-party governed guidance. |
| `mirrored_docs_pack` | **Mirrored** | A verified mirror of an upstream authoritative docs pack. |
| `managed_catalog` | **Managed** | Published through a managed runbook catalog under a signed manifest. |
| `browser_reference` | **Reference only** | Captured from browser-only vendor documentation. No standing authority. |

## What a source descriptor declares

Each [`GovernedRunbookSource`] declares, in addition to its provenance class:

- a **version ref** for the source content;
- a **signer/provenance** block — a signer ref, a provenance *kind*
  (`signed_first_party`, `mirror_digest`, `catalog_manifest`, or
  `browser_capture`), whether the signature was verified, and the version that
  attestation covers;
- a **freshness window** — the day-thresholds that bound its fresh and stale
  windows, how many days ago its provenance was last verified, and whether that
  verification succeeded. The freshness *state* (`fresh` / `aging` / `stale` /
  `expired`) is **computed**, never asserted;
- an **owning scope** and owner role;
- its **export rights** — whether and how it appears in support exports. A source
  descriptor never carries a raw vendor body.

## Effective authority is derived, not asserted

The descriptor *derives* an **effective authority posture** from that declared
truth, and a reference-only source can never silently present as a first-party
executable runbook:

- A **browser reference is reference-only** unless another governed source
  **promotes** its step set into an authority-bearing posture. A promotion names
  the governed source that vouches for it (which must itself be authority-bearing
  and not a browser reference), the posture it raises to (`authoritative` or
  `managed`), an approver, and a rationale.
- A source whose freshness has gone **stale or expired auto-narrows** back to
  reference-only — a stale mirror is no longer executable until it is re-verified.
- A source is **executable** only when its effective posture is authority-bearing
  *and* its freshness is current enough. Reference-only sources are never
  executable.

The validator recomputes the effective posture and the per-source badge and
compares them to the stored values, so they can never drift from the declared
descriptor. A browser capture that tries to claim a verified first-party
signature, or to use a first-party provenance kind, is rejected.

## The same descriptor everywhere

The register projects one surface-independent **badge** per source — provenance
class, effective posture, freshness, signer summary, and version. The docs/help
runbook browser, the incident workspace, operator dashboards, and support exports
all render the same badge, so freshness, signer/source class, and authority
posture stay visible wherever a runbook is rendered or exported. Support exports
carry the same truth, redacted to metadata, for any source whose export rights
allow it.

## Consuming the register

- **Docs/Help** shows each runbook's source class and posture so a reader can tell
  authoritative, mirrored, managed, and reference-only apart at a glance.
- **Incident workspaces** and **operator dashboards** badge every rendered runbook
  with its provenance and freshness, and refuse to execute reference-only sources.
- **Support exports** ship the source badges so a support reader sees the same
  authority truth the operator saw.

This lane governs only how Aureline represents and exposes runbook source
authority. It does not invent new control planes or external-console replacements.
