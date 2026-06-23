# Retention/deletion-matrix contract

This document covers the *rendered* retention/deletion matrices: the concrete,
typed instances of the retention/deletion surface that Aureline shows on its
claimed managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline
profiles.

Where the [admin-plane matrix](./m5-admin-plane.md) *names and freezes the
contract* — including the `retention_deletion_matrix` surface family, the states
it admits, the controlled vocabularies it binds, and the proof packet that keeps
it current — this lane *renders that surface*. It turns retention and deletion
truth into a first-class local product surface: for every claimed managed
artifact family a user or admin can read, on the machine in front of them, what
data class it is, where its copies live, what its default retention is, what its
export and delete routes are, who owns it, what schema governs it, and — most
importantly — whether a delete completes immediately, is deferred, or is blocked,
and exactly what remains, where it remains, and who controls the next step,
without opening a separate vendor console.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/admin/m5-retention-deletion.schema.json`](../../schemas/admin/m5-retention-deletion.schema.json)
  — boundary schema for `m5_retention_deletion_bundle`.
- [`/fixtures/admin/m5-retention-deletion/canonical_retention.json`](../../fixtures/admin/m5-retention-deletion/canonical_retention.json)
  — the published canonical retention/deletion bundle; the freeze gate asserts the
  in-code builder equals it byte-for-byte.
- [`/artifacts/admin/m5-retention-deletion.md`](../../artifacts/admin/m5-retention-deletion.md)
  — the human-readable companion (per-profile retention/deletion tables).
- `crates/aureline-policy/src/m5_retention_deletion/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_retention_deletion` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Binds back to the matrix

The render layer is not free-form. Each matrix binds back to the frozen
[admin-plane matrix](./m5-admin-plane.md):

- **Every state it shows is one the matrix admits.** A row's `machine_state` and
  the matrix's `coverage.coverage_state` must each be in the matrix's
  `applicable_states` for the retention/deletion surface — `active_enforced`,
  `delete_pending`, `delete_blocked_by_hold`, `delete_receipted`,
  `export_available_now`, `export_deferred`, `unconfirmed_stale`, or
  `unknown_requires_review` (`retention_deletion.surface_states_within_matrix`).
- **Every token it uses is a matrix term.** The `owner_escalation`,
  `data_residency_class`, and freshness tokens are exactly the terms the matrix's
  shared vocabulary defines.

So an edit that shows a state the matrix does not admit, or a token the matrix
does not define, flips an invariant and fails the freeze gate.

## Retention rows

Each row is one claimed managed artifact family. It names:

- a **stable row id** and a **record family** (`durable_workspace_state`,
  `collaboration_session_record`, `ai_retained_evidence_packet`,
  `support_export_packet`, `operational_audit_record`, …),
- a distinguished **data class** — `user_owned`, `workspace_owned`,
  `tenant_owned`, `imported`, or `derived_cache` — so an imported snapshot or a
  derived cache is labeled distinctly rather than flattened into one bucket
  (`retention_deletion.data_classes_distinguished`),
- its **location** (`local_only` versus a `managed_copy`, `mirrored_copy`,
  `shared_workspace_copy`, or `exported_snapshot`), so a local-only artifact is
  distinguished from a hosted copy (`retention_deletion.location_explicit`),
- its **default retention** (`user_controlled`, `fixed_window`,
  `regulatory_hold`, `entitlement_lifetime`, `ephemeral_regenerable`,
  `mirror_last_synced`) with a reviewable label,
- its **export route** and **delete route**, each with a reviewable label,
- its current **delete outcome** — `immediate`, `deferred`, or `blocked` — its
  **machine-readable state** (bound to the matrix vocabulary), and the **evidence
  freshness** behind it,
- the **owner** of retention/deletion and the **governing schema** (the spec's
  schema note),
- and both a **machine-readable summary** and a **plain-language handoff
  sentence** (`retention_deletion.export_parity`).

A row whose backing evidence is stale is never shown under a confirmed
`active_enforced`, `export_available_now`, or `delete_receipted` state; offline
and imported rows use an explicit non-confirmed state instead
(`retention_deletion.no_silent_green`).

## Block / defer / immediate outcomes and the remainder

The block/defer/immediate distinction is real, not a single pending status
(`retention_deletion.outcomes_all_present`):

- An **immediate** delete completes locally now with nothing left behind, and
  carries no remainder.
- A **deferred** or **blocked** delete carries a **remainder** that names *what
  remains*, *where it remains* (a residency class), *when it is expected to
  complete*, and *who controls the next step*
  (`retention_deletion.non_immediate_explains_remainder`). A blocked delete
  escalates to a governance owner other than the local user
  (`retention_deletion.ownership_visible`).

## Deletion linkage

Deletion states link to distinct objects rather than one generic pending status
(`retention_deletion.deletion_linkage_distinct`):

| Linkage | Token | Means |
| --- | --- | --- |
| Destruction receipt | `destruction_receipt` | A receipt proving a delete actually happened |
| Privacy-request case | `privacy_request_case` | A data-subject request driving the deletion |
| Legal hold | `legal_hold` | A hold blocking the deletion |
| Partial-delete reason | `partial_delete_reason` | A reason the delete can only complete partially now |

Delete/export honesty is enforced: a row shown as `delete_receipted` carries a
destruction receipt and a row shown as `delete_blocked_by_hold` names its hold,
never a bare deleted claim (`retention_deletion.delete_export_honest`).

## Export parity and propagation

Every row is exportable two ways and a matrix offers both forms
(`retention_deletion.export_parity`):

- a **machine-readable summary** (`machine_readable_json`) — stable record-class
  token, data class, location, retention, routes, outcome, and state as JSON
  summary objects, for tooling, and
- a **plain-language handoff packet** (`plain_language_handoff`) — the same rows
  as reviewable sentences for a support, offboarding, or compliance handoff.

Every matrix names propagation into the support export, the offboarding flow, a
compliance packet, and the Help/About public-truth surface, so the
retention/delete states reach those surfaces unchanged rather than being
re-derived (`retention_deletion.propagation_complete`).

## Coverage and offline retention

Each matrix labels its coverage with a completeness class (`complete`,
`partial_offline`, `partial_imported`, `partial_redacted`) and a coverage note,
so a partial registry view is never presented as complete
(`retention_deletion.coverage_labeled`). Every profile — including self-hosted,
sovereign/air-gapped, and mirrored/offline — keeps a locally inspectable matrix
that does not require a vendor console or control plane
(`retention_deletion.locally_inspectable_offline`).

## Profiles covered

The bundle renders one packet per claimed managed-bearing profile:
`managed_cloud`, `self_hosted`, `sovereign_air_gapped`, and `mirrored_offline`.
Each maps to a matrix admin path and a deployment profile.

## Cross-surface parity

There is exactly **one typed packet per profile**, and each packet declares the
consumers the matrix maps to this surface: shell admin center, CLI/headless
inspect, Help/About, support export, and procurement. Because every consumer
serializes the same packet, the retention/deletion matrix is identical across UI,
CLI, Help/About, support export, and procurement surfaces by construction
(`retention_deletion.consumer_parity`).

## Invariants

The builder computes each invariant's `holds` flag from the rendered data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `retention_deletion.surface_states_within_matrix` — every rendered state is one
  the frozen matrix admits for the retention/deletion surface.
- `retention_deletion.retention_route_outcome_complete` — every row names its
  retention class, export/delete routes, outcome, state, owner, and governing
  schema; row ids are unique.
- `retention_deletion.data_classes_distinguished` — every data class appears
  across the bundle, so artifacts are not flattened into one bucket.
- `retention_deletion.non_immediate_explains_remainder` — every deferred or
  blocked delete explains what/where/when/who; immediate deletes carry no
  remainder.
- `retention_deletion.deletion_linkage_distinct` — every non-immediate delete
  links to a specific receipt/case/hold/partial-reason and every linkage class
  appears across the bundle.
- `retention_deletion.delete_export_honest` — receipted deletes carry a receipt
  and hold-blocked deletes name their hold.
- `retention_deletion.location_explicit` — local-only and hosted locations are
  both exercised, so they are labeled distinctly.
- `retention_deletion.export_parity` — every row carries both export
  representations and every matrix offers both export forms.
- `retention_deletion.propagation_complete` — every matrix names propagation into
  support export, offboarding, compliance packet, and Help/About public truth.
- `retention_deletion.no_silent_green` — stale evidence never sits under a
  confirmed active/export-available/receipted state.
- `retention_deletion.ownership_visible` — every blocked delete escalates to a
  governance owner other than the local user.
- `retention_deletion.locally_inspectable_offline` — every profile keeps a
  locally inspectable, vendor-console-independent matrix.
- `retention_deletion.coverage_labeled` — a partial registry view is labeled,
  never implied complete.
- `retention_deletion.consumer_parity` — one typed packet serves every consumer
  the matrix declares for this surface identically.
- `retention_deletion.profiles_covered` — the managed-cloud, self-hosted,
  sovereign/air-gapped, and mirrored/offline profiles are all rendered.
- `retention_deletion.outcomes_all_present` — immediate, deferred, and blocked
  outcomes all appear, so the distinction is real.
- `retention_deletion.export_safe` — every stable id is an opaque token and every
  governing schema is a repo-relative ref.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw provider
payloads, raw record bodies, or absolute paths — only opaque object refs, stable
tokens, rendered metadata-safe summaries, and short reviewable sentences.
`is_support_export_safe()` enforces that `raw_payload_excluded` is true, every
file ref is repo-relative, and every stable token id is opaque, so the bundle is
safe to embed in a support export verbatim.

## Composes with

This contract renders the retention/deletion surface the
[admin-plane matrix](./m5-admin-plane.md) freezes, alongside the
[decision-history](./m5-decision-history.md) and
[admin-plane render](./m5-admin-render.md) layers. Its rows are grounded in the
record-class registry and the export/delete lifecycle the matrix binds for the
durable retention truth, and its states propagate into the offboarding,
compliance, support-export, and Help/About surfaces.

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_retention_deletion > \
  fixtures/admin/m5-retention-deletion/canonical_retention.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_retention_deletion

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_retention_deletion -- --lines
```
