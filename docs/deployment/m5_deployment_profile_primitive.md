# M5 deployment-profile primitive

The **deployment-profile primitive** is the reusable install-profile card,
side-by-side import sheet, and rollout-ring row that install, update, admin,
diagnostics, and support surfaces ingest instead of cloning About-page or
admin-dashboard chrome. One deployment context resolves into all three surfaces and
they share one deployment identity, so operating mode, ownership, rollback target,
shared-vs-isolated state, and rollout stage never blur across them.

It **narrows** three families of the frozen
[deployment/continuity component matrix](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
— `install_profile_card`, `side_by_side_import_sheet`, and `rollout_ring_row` —
into one working resolver (`resolve_deployment_profile`) rather than restating
install / deployment truth in feature-local prose. It reuses the frozen matrix's
operating-mode, provenance/freshness, rollout-ring, promotion-state, and
downgrade-trigger vocabulary; it adds only the minted vocabulary the resolver needs
(install scope, updater owner, rollback target, state-sharing model, import choice,
export field, and the parity surface families).

- **Boundary schema:**
  [`schemas/ui/m5-deployment-profile-primitive.schema.json`](../../schemas/ui/m5-deployment-profile-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-deployment-continuity-component-matrix.schema.json`](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-deployment-profile-primitive-proof/support_export.json`](../../artifacts/release/m5-deployment-profile-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-deployment-profile-primitive/`](../../fixtures/ui/m5-deployment-profile-primitive/)
- **Implementation:**
  `crates/aureline-install/src/implement_the_m5_install_profile_side_by_side_import_and_rollout_ring_primitive/`

## What the resolver projects

`resolve_deployment_profile(&M5DeploymentProfileInput)` returns a
`M5ResolvedDeploymentProfile` with three surfaces that all carry the same
`deployment_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Install-profile card | `M5ResolvedInstallProfileCard` | install mode + scope, channel, updater owner, durable state roots, the build that owns the running app, and the rollback target |
| Side-by-side import sheet | `M5ResolvedSideBySideImportSheet` | sibling presence, shared-vs-isolated state model, one-time import/copy choice, isolation preservation, and rollback-checkpoint truth |
| Rollout-ring row | `M5ResolvedRolloutRingRow` | ring, promotion state, ring owner, platform scope, evidence freshness, and rollback path |

## Acceptance criteria the resolver proves

- **AC1 — install ownership and rollback target are never hidden.** The install
  card names which build / channel / install mode owns the running app and what
  rollback exists. `running_owner_disclosed` is honest: when the rollback target has
  not been established it is kept explicit as `unknown` rather than implying a safe
  revert exists.
- **AC2 — a side-by-side handoff never depends on hidden state sharing.** The import
  sheet names the shared-vs-isolated state model and the one-time import/copy choice
  explicitly, never silently captures a default handler
  (`LastWriterWinsCapture` is rejected), and never moves durable state across
  channels without a preserved rollback checkpoint (`StateMoveWithoutCheckpoint` is
  rejected).
- **AC3 — managed rollout preserves ring identity and promotion evidence.** A
  managed rollout that omits its ring owner or platform scope is rejected
  (`RolloutIdentityFlattened`), so a managed fleet never collapses every install
  into one generic version list.

## Honesty guarantees

- Raw config bytes, credentials, license keys, mirror URLs, and device identifiers
  never cross this boundary; the resolver carries only opaque refs, typed class
  tokens, booleans, and redacted labels.
- A degraded input must carry a precise, non-generic label; a generic non-answer
  (`unavailable`, `error`, `offline`, …) is rejected.
- The support / export packet reconstructs exactly what each surface would have
  shown: every worked case stores both its input and its resolved projection, and
  validation re-runs the resolver so a stored projection can never drift from the
  live resolver.

## Parity matrix

The `M5DeploymentProfilePrimitivePacket` binds each of the six deployment surface
families (About / install card, update center, admin fleet console, side-by-side
review, diagnostics deployment, support / export replay) to the shared contract with
worked resolution cases, a frozen controlled-vocabulary set, governance-review and
consumer-projection blocks, and a release / support parity posture. See the
[matrix CSV](../../artifacts/release/m5-deployment-profile-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-deployment-profile-primitive-proof/report.md)
for the per-surface summary.
