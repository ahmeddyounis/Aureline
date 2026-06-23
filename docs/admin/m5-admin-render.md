# Admin-plane render contract

This document covers the *rendered* admin-plane surfaces: the concrete, typed
instances of the effective-policy view, the policy-diff sheet, the locked-state
explanations, and the endpoint-posture card that Aureline shows on its claimed
managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles.

Where the [admin-plane matrix](./m5-admin-plane.md) *names and freezes the
contract* — the surface families, the shared state vocabulary, the controlled
vocabularies, and the admin paths — this lane *renders the surfaces*. It turns
policy and endpoint state into a first-class local product surface: a user or
admin can read, on the machine in front of them, what each control resolves to,
which source wins, whether it is locked and why, what a pending policy change
moves, and what install/update/mirror/trust posture the endpoint is on — without
a separate vendor console.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/admin/m5-admin-render.schema.json`](../../schemas/admin/m5-admin-render.schema.json)
  — boundary schema for `m5_admin_render_bundle`.
- [`/fixtures/admin/m5-admin-render/canonical_render.json`](../../fixtures/admin/m5-admin-render/canonical_render.json)
  — the published canonical render bundle; the freeze gate asserts the in-code
  builder equals it byte-for-byte.
- [`/artifacts/admin/m5-admin-render.md`](../../artifacts/admin/m5-admin-render.md)
  — the human-readable companion (per-profile surface tables).
- `crates/aureline-policy/src/m5_admin_render/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_admin_render` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Binds back to the matrix

The render layer is not free-form. Each rendered surface binds back to the frozen
[admin-plane matrix](./m5-admin-plane.md):

- **Every state it shows is one the matrix admits.** A control's `state`, a diff
  entry's `from_state`/`to_state`, a locked explanation's `lock_state`, and the
  endpoint card's `posture_state` must each be in the matrix's `applicable_states`
  for that surface family (`admin_render.surface_states_within_matrix`).
- **Every token it uses is a matrix term.** The `policy_source_state`,
  `verification_signature_posture`, `data_residency_class`, `owner_escalation`,
  and freshness tokens are exactly the terms the matrix's shared vocabulary
  defines.

So an edit that shows a state the matrix does not admit, or a token the matrix
does not define, flips an invariant and fails the freeze gate. The render layer
cannot drift from the contract.

## Rendered surfaces

### Effective-policy view

One row per control, each naming:

- the **source chain** (lowest precedence first) with exactly one winning link,
- the **resolved state** (`active_enforced`, `locked_by_policy`,
  `inherited_default`, `overridden_local`, `unconfirmed_stale`,
  `pending_managed_sync`, `mirror_offline_last_known`, `unknown_requires_review`),
- the **scope**, the **verification/signature posture**, the **applied time**,
  the **affected feature family**, the **evidence freshness**, the **data
  residency**, and the **owner**.

A control that is locked or forced carries a `locked_explanation_ref` that
resolves to a locked-state explanation. A control whose backing evidence is stale
is never shown under a confirmed-value state — the stale/mirrored/offline posture
downgrades it (`admin_render.no_silent_green`).

### Policy-diff sheet

A safe comparison of previous versus current effective state. Each changed entry
names the **changed control and feature area**, the **change kind**
(`newly_locked`, `unlocked`, `rescoped`, `value_changed`, `source_changed`,
`no_change`), the **from/to state and source**, the **user-visible consequence**,
the **redaction rule**, and the **owner**. A diff computed over stale effective
values is labeled `provisional` rather than presented as a confirmed before/after.

### Locked-state explanations

For every locked or forced control, an explanation naming the **lock reason**,
the **policy source that locks it**, the **verification posture** of that source,
the **owner who can change it** (the next step), an optional **escalation
owner**, and the **local-safe actions** available. This is the acceptance
criterion made executable: every locked control links to an explanation naming
the policy source, verification state, and who owns the next step
(`admin_render.locked_controls_explained`).

### Endpoint-posture card

The enrolled device/install posture: **install mode**, **update ring**, **mirror
sources**, **trust roots**, **bundle freshness**, **identity status**, **last
check age**, **enrollment owner**, **managed-versus-local data footprint**, and
the **diagnostics/export actions**. Every card is locally inspectable and
exportable on every profile (`admin_render.endpoint_posture_exportable`); a stale
check or bundle downgrades the posture rather than showing it active/enforced.

## Profiles covered

The bundle renders one packet per claimed managed-bearing profile:
`managed_cloud`, `self_hosted`, `sovereign_air_gapped`, and `mirrored_offline`.
Each maps to a matrix admin path and a deployment profile.

## Cross-surface parity

There is exactly **one typed packet per profile**, and each packet declares the
consumers that render it: shell admin center, CLI/headless inspect, Help/About,
support export, and release evidence. Because every consumer serializes the same
packet, policy source, diff, and endpoint state are identical across UI,
CLI/support export, Help/About, and release/public-truth by construction
(`admin_render.consumer_parity`).

## Invariants

The builder computes each invariant's `holds` flag from the rendered data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `admin_render.surface_states_within_matrix` — every rendered state is one the
  frozen matrix admits for that surface family.
- `admin_render.source_chain_resolves` — every control has a non-empty source
  chain with exactly one winning link.
- `admin_render.locked_controls_explained` — every locked or forced control links
  to a complete explanation naming source, verification posture, and the owner of
  the next step.
- `admin_render.locked_explanation_complete` — every explanation states a reason
  and at least one local-safe action.
- `admin_render.no_silent_green` — stale evidence never sits under a
  confirmed-value control, and a stale check/bundle never shows an
  active/enforced endpoint.
- `admin_render.policy_diff_safe` — every diff entry names its consequence and
  control, and a diff over stale values is labeled provisional.
- `admin_render.endpoint_posture_exportable` — every profile's endpoint posture
  is locally inspectable and exportable with a diagnostics/export action.
- `admin_render.ownership_visible` — every owned object names an owner.
- `admin_render.consumer_parity` — one typed packet serves shell, CLI/headless,
  Help/About, support export, and release evidence identically.
- `admin_render.profiles_covered` — the managed-cloud, self-hosted,
  sovereign/air-gapped, and mirrored/offline profiles are all rendered.
- `admin_render.stable_ids_unique` — profile, control, change, and explanation
  ids are unique within scope.
- `admin_render.export_safe` — every stable id is an opaque token with no URL
  scheme or absolute path.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw provider
payloads, or absolute paths — only opaque object refs, stable tokens, rendered
metadata-safe value summaries, and short reviewable sentences.
`is_support_export_safe()` enforces that `raw_payload_excluded` is true, every
file ref is repo-relative, and every stable token id is opaque, so the bundle is
safe to embed in a support export verbatim.

## Composes with

This contract renders the surfaces the [admin-plane matrix](./m5-admin-plane.md)
freezes, and composes with the per-surface contracts the matrix binds, notably
[`/docs/admin/policy_explainability_contract.md`](./policy_explainability_contract.md),
[`/docs/admin/policy_diff_alpha.md`](./policy_diff_alpha.md), and
[`/docs/admin/org_admin_seat_and_fleet_contract.md`](./org_admin_seat_and_fleet_contract.md).

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_admin_render > \
  fixtures/admin/m5-admin-render/canonical_render.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_admin_render

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_admin_render -- --lines
```
