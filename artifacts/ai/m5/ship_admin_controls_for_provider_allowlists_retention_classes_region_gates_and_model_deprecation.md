# Admin Controls For Provider Allowlists, Retention Classes, Region Gates, And Model Deprecation

- Packet: `admin-controls:stable:0001`
- Schema: `schemas/ai/ship-admin-controls-for-provider-allowlists-retention-classes-region-gates-and-model-deprecation.schema.json`
- Support export: `artifacts/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/support_export.json`
- Fixture: `fixtures/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/`
- Contract: `docs/automation/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation.md`

## Coverage

The packet carries operator-set governance over AI providers and models in one
row per admin control. Every row names the provider it governs (and, for a model
deprecation, the model), the typed directive that says what the control does, the
scope and state it is enforced under, the admin authority that set it, and the
downgrade rules that narrow its claim. The family is derived from the directive,
so no row hides cost, provider, region, retention, or automation authority behind
generic language.

- `allowlist-deny-acme` denies the Acme provider for BYOK dispatch org-wide at
  Stable: it carries an admin-approval gate, is audited, and is actively enforced.
  As a denial control it never sits in a silent draft while claiming a public
  qualification, and a provider outage narrows it to `held`.
- `allowlist-allow-managed` conditionally allows first-party managed dispatch
  org-wide at Stable, deferring the concrete posture to the paired region gate and
  retention floor.
- `retention-floor-no-training` sets a no-training retention floor tenant-wide and
  denies any route below it; the floor names a concretely disclosed retention
  class rather than an unverified posture.
- `region-gate-eu` pins managed routes to the `eu-west` and `eu-central` regions
  workspace-wide and denies any route outside the gate; a pinned gate names its
  concrete region tags.
- `deprecate-legacy-model` schedules the sunset of `model:legacy-large`, names its
  replacement `model:next-large`, and points at a migration runbook so no user is
  stranded.

## Downgrade and trust posture

Every control carries the proof-stale and provider-unavailable downgrade triggers,
and each rule narrows to a strictly weaker qualification than the control claims.
A control overridden by a higher-tier policy, or a provider allowlist still
pending admin review, narrows out of the public lanes rather than presenting an
unenforced claim — see the blocked-control fixture for the canonical narrowing.

## Consumers

Shell, docs, support export, diagnostics, and release tooling read
`AdminControlPacket` directly. `AdminControlPacket::is_provider_admin_blocked`
projects the live allowlist denial routing surfaces honor; `live_controls`,
`family_count`, `denial_control_count`, and `narrowed_qualification` give the
deterministic projections those surfaces render instead of re-deriving admin
posture by hand.
