# M5 Repository-Bootstrap, Checkout-Plan, Trust-Stage, and Post-Open-Queue Contract

Status: frozen (B142 opening matrix)

This contract freezes Aureline's concrete repository-acquisition and workspace-bootstrap behavior into one
export-safe matrix. It is the canonical source of repository-bootstrap truth for M5: later start-center /
entry routers, workspace / git / trust / auth services, help/support diagnostics, claim publication, and
release-evidence tooling consume it directly rather than copying onboarding prose by hand.

- Matrix schema: `schemas/workspaces/m5-repository-bootstrap-matrix.schema.json`
- Source-locator domain schema (open-local / open-archive): `schemas/workspaces/m5-source-locator.schema.json`
- Checkout-plan domain schema (clone-remote): `schemas/workspaces/m5-checkout-plan.schema.json`
- Bootstrap-evidence domain schema (import-bundle / resume-snapshot): `schemas/workspaces/m5-bootstrap-evidence.schema.json`
- Support export: `artifacts/release/m5-repository-bootstrap-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-repository-bootstrap-proof/matrix.csv`
- Design report: `artifacts/workspaces/m5-repository-bootstrap-matrix.md`
- Narrowed fixtures: `fixtures/workspaces/m5-repository-bootstrap/`
- Authoritative validator: `crates/aureline-ui` (`m5_repository_bootstrap_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_repository_bootstrap_matrix`

## Governed acquisition families

The matrix freezes **five** project-entry acquisition families, each qualified independently and each
pointing at one canonical domain schema:

| Family | Acquisition concern | Owner | Domain schema |
| --- | --- | --- | --- |
| `open_local` | Locate an existing local checkout; never reclone over it | Repository-acquisition owner | source-locator |
| `clone_remote` | Show checkout cost, topology, and credential posture before the fetch | Git-service owner | checkout-plan |
| `open_archive` | Verify the archive digest and extraction plan before disk mutation | Repository-acquisition owner | source-locator |
| `import_bundle` | Preserve signer and mirror provenance across offline or mirrored fetches | Trust-service owner | bootstrap-evidence |
| `resume_snapshot` | Keep interrupted acquisition resumable or discardable with evidence | Workspace-service owner | bootstrap-evidence |

## Shared repository-bootstrap-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`source_locator`, `checkout_plan`, `credential_posture`, `evidence_packet`, `staged_trust`,
`resumable_acquisition`, `post_open_queue`.

The credential / evidence / staged-trust / post-open-queue roles (`credential_posture`, `evidence_packet`,
`staged_trust`, `post_open_queue`) must stage trust and disclose provenance before bootstrap — an
acquisition may never hide the bootstrap credential posture, lose signer or mirror provenance, run a
repo-owned action implicitly, or auto-execute a post-open bootstrap queue. The descriptive structure roles
(`source_locator`, `checkout_plan`, `resumable_acquisition`) are inspectable descriptors.

## Hard invariants

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block
asserts the corresponding fleet-level guarantees:

1. Acquisition never rewrites clone into open because a local checkout already exists.
2. Acquisition never runs repo-owned actions (hooks, repo tasks, extensions, package restores, submodule or
   LFS hydration, generator installs) implicitly.
3. Signer and mirror provenance are never lost across an offline or mirrored fetch.
4. Partial acquisition is never stranded without Resume / Discard / Open-read-only-partial-root choices.
5. Bootstrap credential posture is never hidden behind generic connected-state copy.

## Automatic narrowing

Claim publication and support/export narrow repository-bootstrap claims automatically when the B142 registry
is missing, stale, or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping
every family visible:

- `import_bundle_beta_narrowed.json` — import bundle held at **Beta** pending mirror / air-gap signer
  continuity across every acquisition context.
- `resume_snapshot_preview_narrowed.json` — resume snapshot narrowed to **Preview** pending complete
  resumable-partial-acquisition evidence.

## Bound source contracts

The matrix binds back to already-landed truth so acquisition truth is never split across scattered
onboarding notes: the repository-acquisition schema
(`schemas/workspace/repository_acquisition.schema.json`) and the source-acquisition-review schema
(`schemas/workspace/m5-source-acquisition-review.schema.json`).
