# Admin-certification contract

This document covers the *admin-plane certification capstone*: the qualification
layer that binds the local admin plane's effective-policy, decision-history,
endpoint-posture, retention/deletion, offboarding, and procurement/admin-packet
truth into M5 promotion, and auto-narrows a managed claim the moment any of that
proof goes stale or starts failing — all across the claimed managed-cloud,
self-hosted, sovereign/air-gapped, and mirrored/offline profiles.

Where the [admin-plane matrix](./m5-admin-plane.md) *names and freezes the
contract*, the [admin-plane render](./m5-admin-render.md) lane *renders the
current admin state*, and the [rollout simulation](./m5-rollout-simulation.md)
lane *simulates the next state*, this lane *certifies the present state*. It does
not re-derive any admin truth: each certified family cites the upstream proof lane
that already produces it — its boundary schema, worked fixture, and freeze gate —
and reads that lane's freshness and pass/fail result into a single per-profile
qualification row. The capstone's only job is the honest verdict: is this family,
on this profile, *proven current* right now, or not.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/admin/m5-admin-certification.schema.json`](../../schemas/admin/m5-admin-certification.schema.json)
  — boundary schema for `m5_admin_certification_bundle`.
- [`/fixtures/admin/m5-admin-certification/canonical_certification.json`](../../fixtures/admin/m5-admin-certification/canonical_certification.json)
  — the published canonical certification bundle; the freeze gate asserts the
  in-code builder equals it byte-for-byte.
- [`/artifacts/admin/m5-admin-certification.md`](../../artifacts/admin/m5-admin-certification.md)
  — the human-readable companion (per-profile qualification tables).
- `crates/aureline-policy/src/m5_admin_certification/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_admin_certification` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Binds back to the matrix

The certification layer is not free-form. Every family row names the
[`AdminSurfaceClass`](./m5-admin-plane.md) surfaces it certifies, and binds back
to the frozen matrix:

- **Every surface it certifies exists and is locally explainable.** Each bound
  surface is present in the matrix, `locally_explainable`, and
  `typed_not_portal_only` (`admin_cert.bound_surfaces_in_matrix`).
- **Every claim state is in the matrix vocabulary.** Each per-row, per-profile,
  and release-evidence `claim_state` is one the matrix's unified state vocabulary
  defines (`admin_cert.claim_states_in_vocabulary`).

So an edit that certifies a surface the matrix does not define, or emits a state
outside the frozen vocabulary, flips an invariant and fails the freeze gate.

## Families certified

Every governed admin-plane family the spec requires M5 promotion to depend on is
certified, exactly once per profile (`admin_cert.families_covered`):

- `policy_explainability` — effective policy, policy diff, and locked-state
  explanation. Proven against the [admin-plane render](./m5-admin-render.md) lane.
- `decision_history` — the decision-history timeline / audit-event explorer.
  Proven against the [decision-history](./m5-decision-history.md) lane.
- `endpoint_posture` — the endpoint-posture card. Proven against the
  [admin-plane render](./m5-admin-render.md) lane.
- `retention_delete` — the retention/deletion matrix. Proven against the
  [retention/deletion](./m5-retention-deletion.md) lane.
- `offboarding` — the offboarding wizard. Proven against the
  [offboarding](./m5-offboarding.md) lane.
- `procurement_admin_packet` — the procurement/verification packet and
  admin-handoff bundle. Proven against the [procurement](./m5-procurement.md) lane.

Each family row carries a `proof_lane` with the upstream `record_kind`,
`schema_ref`, `fixture_ref`, `freeze_gate_ref`, `doc_ref`, and `produced_by_ref`,
so the certification is always traceable to a producing crate, schema, fixture,
and freeze gate (`admin_cert.proof_lane_bound`).

## No green on stale or failing proof

The core honesty rule. A family row's `qualification` is computed from its proof
state, never hand-asserted (`admin_cert.no_green_on_stale_or_failing`):

- `qualified` — the proof lane is bound, fresh, and passing. Reads
  `active_enforced`.
- `narrowed_stale_evidence` — the proof is past its soft-refresh window. Reads
  `unconfirmed_stale` and names `family_evidence_stale`, or `mirror_evidence_stale`
  when the proof is mirror-backed.
- `narrowed_failing_proof` — the proof packet / freeze gate is failing. Reads
  `unconfirmed_stale` and names `family_proof_failing`. Note this is independent of
  freshness: a proof can be fresh in age yet failing in result.
- `narrowed_unproven` — no upstream proof lane is bound. Reads
  `unknown_requires_review` and names `family_proof_missing`.

A `qualified` row must cite a proof lane whose schema, fixture, freeze gate, doc,
and producer refs are all present and export-safe, so a claim can never go green
because the mechanics exist *somewhere in the stack* while the user-facing proof is
absent (`admin_cert.qualified_requires_proven_lane`).

## Auto-narrowing the managed claim

A profile's managed claim is honest only while every claimed family is proven
current. Each packet aggregates its family rows into a `claim_state`
(`admin_cert.profile_claim_auto_narrows`):

- When **every** claimed family qualifies, the claim reads `active_enforced`
  (confirmed) and `narrow_reasons` is empty.
- When **any** claimed family is stale, failing, or unproven, the claim downgrades
  off confirmed and `narrow_reasons` names the deduplicated reasons; `claim_note`
  names which families narrowed it.

The reported `proof_freshness` is the stalest claimed family, so one stale family
cannot hide behind fresher siblings (`admin_cert.proof_freshness_is_worst_case`).

## Release-evidence rows

The bundle publishes an explicit release-evidence row for each named dimension, so
release automation reads one source of admin-plane qualification truth
(`admin_cert.release_evidence_rows_present`):

- `policy_source_verification` — policy explainability and endpoint posture.
- `audit_history` — decision history.
- `delete_export_honesty` — retention/delete.
- `offboarding_continuity` — offboarding.
- `procurement_support_admin_packet` — procurement/admin packet.

Each row's `worst_qualification` is the most-narrowed qualification across **every**
profile for the dimension's families, so release evidence is never rosier than the
underlying admin plane (`admin_cert.release_evidence_reflects_worst`).

## Profiles covered

The bundle certifies one packet per claimed managed-bearing profile: `managed_cloud`
(every lane fresh and passing — confirmed), `self_hosted` (the audit-history proof
is failing — narrowed), `sovereign_air_gapped` (endpoint-posture and procurement
proof stale — narrowed), and `mirrored_offline` (mirror-backed retention and
offboarding proof stale — narrowed) (`admin_cert.profiles_covered`). The
no-console-required explainability holds on the narrowed rows too
(`admin_cert.local_explainability`).

## Cross-surface parity

There is exactly **one typed packet per profile**, and each packet declares the
consumers that render it: shell admin center, CLI/headless inspect, Help/About,
support export, commercial/procurement, and release evidence. Because every
consumer serializes the same packet, About/help/support/commercial read the
qualification state instead of restating admin-plane quality claims by hand
(`admin_cert.consumer_parity`).

## Invariants

The builder computes each invariant's `holds` flag from the certified data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `admin_cert.families_covered` — every certified family has exactly one row on
  every profile.
- `admin_cert.profiles_covered` — managed-cloud, self-hosted, sovereign/air-gapped,
  and mirrored/offline are all certified.
- `admin_cert.bound_surfaces_in_matrix` — every certified surface is present,
  locally explainable, and typed in the frozen matrix.
- `admin_cert.claim_states_in_vocabulary` — every claim state is in the matrix's
  unified vocabulary.
- `admin_cert.no_green_on_stale_or_failing` — a row qualifies only when its proof
  is bound, fresh, and passing; otherwise it narrows and names a reason.
- `admin_cert.qualified_requires_proven_lane` — a qualified row cites a real,
  export-safe proof lane.
- `admin_cert.proof_lane_bound` — every row cites a non-empty, export-safe proof
  lane.
- `admin_cert.profile_claim_auto_narrows` — a profile is confirmed only when every
  claimed family qualifies; otherwise it downgrades and names reasons.
- `admin_cert.proof_freshness_is_worst_case` — the reported proof freshness is the
  stalest claimed family.
- `admin_cert.release_evidence_rows_present` — one release-evidence row per named
  dimension, each bound to at least one family.
- `admin_cert.release_evidence_reflects_worst` — each release-evidence row carries
  the worst qualification across all profiles.
- `admin_cert.local_explainability` — every family is locally explainable.
- `admin_cert.consumer_parity` — one typed packet serves shell, CLI/headless,
  Help/About, support export, commercial/procurement, and release evidence.
- `admin_cert.stable_ids_unique` — profile, row, and release-evidence ids are
  unique within scope.
- `admin_cert.export_safe` — every stable id is an opaque token and every proof-lane
  ref is repo-relative.
- `admin_cert.binds_admin_plane_matrix` — the bundle binds the frozen matrix by id
  and cites its canonical fixture.

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_admin_certification > \
  fixtures/admin/m5-admin-certification/canonical_certification.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_admin_certification

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_admin_certification -- --lines
```
