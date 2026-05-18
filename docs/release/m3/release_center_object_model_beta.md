# Beta release-center object model

The beta release center uses a shared typed object model for candidate
review, publication, rollback, revocation, support export, and audit
reconstruction. The model is metadata-only: it carries stable ids, refs,
typed classes, immutable digest refs, freshness state, and continuity notes.
It does not carry raw artifacts, credentials, raw support attachments, or
pipeline logs.

Machine-readable contracts:

- [`/crates/aureline-release/src/release_center_model/`](../../../crates/aureline-release/src/release_center_model/)
- [`/schemas/release/release_candidate.schema.json`](../../../schemas/release/release_candidate.schema.json)
- [`/schemas/release/publish_target.schema.json`](../../../schemas/release/publish_target.schema.json)
- [`/schemas/release/promotion_timeline.schema.json`](../../../schemas/release/promotion_timeline.schema.json)
- [`/schemas/release/release_center_object.schema.json`](../../../schemas/release/release_center_object.schema.json)

## Object Family

| Object | What it answers | Required linkage |
|---|---|---|
| Release candidate | What is being promoted, which artifact graph it belongs to, which targets it can reach, and what rollback target exists. | Exact-build refs, artifact bundle refs, publish target refs, evidence refs, known-issue refs, auth source class, rollout ring. |
| Version-bump proposal | What public surface changed between versions. | Manifest/schema changes, SDK or ABI range shifts, extension compatibility notes, docs-pack changes, mirror/import implications. |
| Publish target descriptor | Where publication lands and what authority it uses. | Target class, destination, visibility, mutability, auth source class, dry-run/scope preview, rollout ring, rollback path, evidence freshness. |
| Artifact bundle card | What is inside the promoted graph without unpacking archives. | Immutable digest refs, exact-build refs, signatures, attestations, docs, schemas, symbols, compatibility notes, advisories, mirror metadata. |
| Promotion timeline step | How the candidate moved and what evidence was attached. | Source and destination stages, actor refs, auth source class, digest refs, evidence refs, reversible window, break-glass state. |
| Rollback or revocation record | What was narrowed, restored, revoked, yanked, repinned, or disabled. | Affected and unaffected artifact refs, blast radius, last-known-good target, known-issue refs, support refs, graph consistency state. |

## Shared Projections

The Rust model exposes three projections from the same object graph:

- `ReleaseCenterUiState` for release-center rows and cards.
- `ReleaseCenterHeadlessPlan` for dry-run and publication automation.
- `ReleaseCenterSupportAuditExport` for support and audit reconstruction.

All three projections carry the same `ReleaseCenterObjectIdentityIndex`.
Validation fails if UI, headless, and support exports diverge on candidate
ids, target ids, timeline ids, rollback/revocation ids, bundle ids,
exact-build refs, or artifact graph refs.

## Promotion Rules

- Candidate promotion is blocked when required evidence is stale or missing,
  when candidate blockers remain, or when artifact bundle, publish target,
  exact-build, or rollback refs are absent.
- Publication acts on artifact bundles and artifact graphs, not single files.
- Publish targets disclose auth source class and rollout ring before mutation;
  raw credential material has no field in the object model.
- Timeline steps must preserve symbols, docs packs, schema exports,
  compatibility notes, advisories or known issues, and mirror metadata.
- Emergency publication, rollback, revocation, yanking, repinning, and disable
  actions use the same timeline records with explicit break-glass state and
  reconciliation refs.

## Rollback And Revocation

Rollback and revocation records are scoped records, not destructive edits to
history. A rollback record names the last-known-good target and rollback
manifest. A revocation, yank, or emergency-disable record names revocation
metadata and advisory refs. Every record keeps known-issue publication and
support-export refs attached so support, admins, mirrors, and audit exports
can reconstruct the action without reading raw archives or pipeline logs.

## Validation

Run:

```bash
cargo test -p aureline-release
python3 -m json.tool schemas/release/release_candidate.schema.json
python3 -m json.tool schemas/release/promotion_timeline.schema.json
python3 -m json.tool schemas/release/publish_target.schema.json
```

The existing release-center pack validation remains the beta packet gate:

```bash
python3 -m tools.ci.m3.release_center_pack --repo-root . --check
```

## Current Limits

The model is the governed object contract and projection layer. It does not
implement signing, registry publication, channel mutation, artifact upload,
or credential custody. Those systems must consume these objects rather than
invent parallel publication shapes.
