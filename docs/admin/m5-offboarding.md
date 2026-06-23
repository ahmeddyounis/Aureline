# Offboarding-wizard contract

This document covers the *rendered* offboarding wizards: the concrete, typed
instances of the offboarding surface that Aureline shows on its claimed
managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles.

Where the [admin-plane matrix](./m5-admin-plane.md) *names and freezes the
contract* — including the `offboarding_wizard` surface family, the states it
admits, the controlled vocabularies it binds, and the proof packet that keeps it
current — this lane *renders that surface*. It turns offboarding into a
first-class local product flow: a user or admin can, on the machine in front of
them, walk the ordered export, transfer, confirm, delete, and local-continuation
checkpoints; see for each step what is exported, who it transfers to, when a
delete completes, what managed copies remain, and who controls the next step; and
complete the whole flow without a still-active paid seat or a separate vendor
console.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/admin/m5-offboarding.schema.json`](../../schemas/admin/m5-offboarding.schema.json)
  — boundary schema for `m5_offboarding_bundle`.
- [`/fixtures/admin/m5-offboarding/canonical_offboarding.json`](../../fixtures/admin/m5-offboarding/canonical_offboarding.json)
  — the published canonical offboarding bundle; the freeze gate asserts the
  in-code builder equals it byte-for-byte.
- [`/artifacts/admin/m5-offboarding.md`](../../artifacts/admin/m5-offboarding.md)
  — the human-readable companion (per-profile checkpoint tables).
- `crates/aureline-policy/src/m5_offboarding/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_offboarding` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Binds back to the matrix

The render layer is not free-form. Each wizard binds back to the frozen
[admin-plane matrix](./m5-admin-plane.md):

- **Every state it shows is one the matrix admits.** A checkpoint's
  `machine_state` and the wizard's `coverage.coverage_state` must each be in the
  matrix's `applicable_states` for the offboarding surface — `active_enforced`,
  `delete_pending`, `delete_blocked_by_hold`, `delete_receipted`,
  `export_available_now`, `export_deferred`, `mirror_offline_last_known`,
  `boundary_changed_recheck_required`, or `unknown_requires_review`
  (`offboarding.surface_states_within_matrix`).
- **Every token it uses is a matrix term.** The `owner_escalation`,
  `data_residency_class`, and freshness tokens are exactly the terms the matrix's
  shared vocabulary defines, and the `delete_outcome` tokens are the ones the
  sibling [retention/deletion](./m5-retention-deletion.md) lane freezes.

So an edit that shows a state the matrix does not admit, or a token the matrix
does not define, flips an invariant and fails the freeze gate.

## The ordered flow

A wizard is an *ordered* set of checkpoints, one per
[`CheckpointKindClass`](../../crates/aureline-policy/src/m5_offboarding/mod.rs),
in this order (`offboarding.checkpoints_ordered_and_complete`):

1. **Review artifacts** — review the selected artifacts and what each later step
   does to them.
2. **Export** — export the selected artifacts in the offered formats.
3. **Transfer** — transfer ownership of shared artifacts to a named owner.
4. **Confirm** — the explicit confirmation checkpoint that gates irreversible
   deletes.
5. **Delete** — delete on the stated deletion schedule (immediate, deferred, or
   blocked).
6. **Local continuation** — the local-only continuation rights that survive the
   exit.

Each checkpoint names its **scope** (`personal`, `workspace`, `team`, or `org`),
its **outcome** (`completed`, `available_now`, `deferred`, `blocked`, or
`failed_recoverable`), its **machine-readable state**, its **evidence freshness**,
its **owner**, the **managed copies remaining** after it, its **governing
schema**, and both a **machine-readable summary** and a **plain-language handoff
sentence** (`offboarding.export_parity`).

## No paid seat to recover user-owned data

No checkpoint, trigger, or coverage view requires a still-active paid seat to
recover user-owned data (`offboarding.no_paid_seat_required`): every
`requires_paid_seat` flag is false, every trigger's
`requires_active_seat_for_recovery` is false, and every profile's coverage is
`completable_without_paid_seat`. Export, delete, and local continuation stay
reachable through downgrade, seat loss, cancellation, and plan change.

## Triggers explain the impact

Every [`OffboardingTrigger`] — seat loss, cancellation, deprovision, org switch,
or plan downgrade — explains the **impacted managed features**, the **export
rights** that stay available, the **local-safe continuation**, and the **managed
copies remaining** in plain language (`offboarding.triggers_explain_impact`).
Every trigger class appears at least once across the bundle.

## Managed copies remaining

Each checkpoint states its [`ManagedCopiesRemaining`] truth: a `disposition`
(`none_remaining`, `deleted_with_receipt`, `pending_scheduled_delete`,
`retained_under_hold`, `retained_upstream_mirror`, or `transferred_to_owner`), a
`count_label`, a `location`, and an `owner`. A checkpoint that *leaves* a managed
copy names what remains, where it remains, and when it clears
(`offboarding.managed_copies_honest`), so the flow never implies everything is
gone when a copy is held, queued, mirrored, or transferred.

## Transfer, deletion schedule, and confirmation

- A **transfer** checkpoint carries a [`TransferPlan`] naming the owner ownership
  moves to (`offboarding.transfer_named`).
- A **delete** checkpoint carries a [`DeletionSchedule`] with an
  immediate/deferred/blocked outcome; a deferred or blocked schedule names what
  remains and when it completes, and across the bundle all three delete outcomes
  appear (`offboarding.deletion_schedule_present`). Every delete checkpoint is
  confirmation-gated.
- A **confirm** checkpoint requires explicit confirmation and gates the
  irreversible deletes that follow (`offboarding.confirmation_gates_deletes`).

## Failed flows are repaired, not restarted

A blocked or failed checkpoint retains a typed [`CheckpointRecovery`]
(`offboarding.failed_flows_recoverable`):

- a **restore checkpoint** to roll back to (`restore_checkpoint_ref`),
- a **typed diagnostic** ([`OffboardingDiagnosticClass`]) — export reauth, transfer
  recipient unavailable, delete blocked by hold, boundary recheck, mirror offline
  retry, or partial-export retryable — *never* a generic sign-in or billing error,
- **next-step guidance** and the `restore_checkpoint` / `retained_diagnostics` /
  `next_step_guidance` affordances,

so a failed export, transfer, or delete is repaired from a saved checkpoint
rather than restarted from zero. At least one failed-recoverable checkpoint
appears across the bundle.

## Local-only continuation rights

Every profile guarantees all four local-only continuation rights — export
user-owned artifacts, continue local-only, edit local artifacts, and publish
later — each available offline and free of a paid seat, and renders a
local-continuation checkpoint (`offboarding.local_continuation_guaranteed`).

## Coverage and offline completion

Each wizard labels its coverage with a completeness class (`complete`,
`partial_offline`, `partial_imported`, `partial_redacted`) and a coverage note, so
a partial flow view is never presented as complete
(`offboarding.coverage_labeled`). A checkpoint whose backing evidence is stale is
never shown under a confirmed `active_enforced`, `export_available_now`, or
`delete_receipted` state (`offboarding.no_silent_green`). Every profile —
including self-hosted, sovereign/air-gapped, and mirrored/offline — keeps a
locally inspectable wizard that does not require a vendor console or control plane
and is completable without a paid seat (`offboarding.locally_inspectable_offline`).

## Profiles covered

The bundle renders one packet per claimed managed-bearing profile: `managed_cloud`,
`self_hosted`, `sovereign_air_gapped`, and `mirrored_offline`. Each maps to a
matrix admin path and a deployment profile.

## Cross-surface parity

There is exactly **one typed packet per profile**, and each packet declares the
consumers the matrix maps to this surface: shell admin center, CLI/headless
inspect, Help/About, support export, and procurement. Because every consumer
serializes the same packet, the offboarding wizard is identical across UI, CLI,
Help/About, support export, and procurement surfaces by construction
(`offboarding.consumer_parity`).

## Invariants

The builder computes each invariant's `holds` flag from the rendered data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `offboarding.surface_states_within_matrix` — every rendered state is one the
  frozen matrix admits for the offboarding surface.
- `offboarding.checkpoints_ordered_and_complete` — every profile renders one
  checkpoint per kind in ascending order; checkpoint ids are unique.
- `offboarding.no_paid_seat_required` — no checkpoint, trigger, or coverage view
  requires a still-active paid seat to recover user-owned data.
- `offboarding.triggers_explain_impact` — every trigger explains impacted
  features, export rights, local continuation, and managed copies remaining; every
  trigger class appears.
- `offboarding.scopes_distinguished` — personal, workspace, team, and org scopes
  all appear, so ownership is not flattened.
- `offboarding.confirmation_gates_deletes` — every profile has an explicit confirm
  checkpoint and every delete is confirmation-gated.
- `offboarding.managed_copies_honest` — every checkpoint states its managed-copies
  disposition; a remaining copy names what/where/when/who.
- `offboarding.failed_flows_recoverable` — every blocked or failed checkpoint
  retains a restore checkpoint, typed diagnostics, and next-step guidance; at
  least one failed-recoverable checkpoint appears.
- `offboarding.deletion_schedule_present` — every delete carries a schedule; a
  non-immediate schedule names its remainder; all three delete outcomes appear.
- `offboarding.transfer_named` — every transfer names the owner ownership moves to.
- `offboarding.local_continuation_guaranteed` — every profile guarantees all four
  offline, seat-free continuation rights and a local-continuation checkpoint.
- `offboarding.export_parity` — every checkpoint carries both export
  representations and every wizard offers both export forms.
- `offboarding.no_silent_green` — stale evidence never sits under a confirmed
  active/export-available/receipted state.
- `offboarding.locally_inspectable_offline` — every profile keeps a locally
  inspectable, vendor-console-independent, seat-free wizard.
- `offboarding.coverage_labeled` — a partial flow view is labeled, never implied
  complete.
- `offboarding.consumer_parity` — one typed packet serves every consumer the
  matrix declares for this surface identically.
- `offboarding.profiles_covered` — the managed-cloud, self-hosted,
  sovereign/air-gapped, and mirrored/offline profiles are all rendered.
- `offboarding.outcomes_all_present` — every checkpoint outcome and every
  managed-copies disposition appears.
- `offboarding.export_safe` — every stable id is an opaque token and every
  governing schema is a repo-relative ref.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw provider
payloads, raw record bodies, or absolute paths — only opaque object refs, stable
tokens, rendered metadata-safe summaries, and short reviewable sentences.
`is_support_export_safe()` enforces that `raw_payload_excluded` is true, every
file ref is repo-relative, and every stable token id is opaque, so the bundle is
safe to embed in a support export verbatim.

## Composes with

This contract renders the offboarding surface the
[admin-plane matrix](./m5-admin-plane.md) freezes, alongside the
[retention/deletion](./m5-retention-deletion.md),
[decision-history](./m5-decision-history.md), and
[admin-plane render](./m5-admin-render.md) layers. Its delete checkpoints reuse
the retention/deletion delete-outcome vocabulary, and its rendered states
propagate into the support-export, compliance, procurement, and Help/About
surfaces the matrix declares as consumers.

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_offboarding > \
  fixtures/admin/m5-offboarding/canonical_offboarding.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_offboarding

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_offboarding -- --lines
```
