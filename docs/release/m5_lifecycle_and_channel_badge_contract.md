# M5 Lifecycle and Channel Badge Primitive Contract

Status: Stable (M5 badge-family, claim-narrowing, and cross-surface truth lane)

Task: M05-942 — implement lifecycle and channel badges with labs / preview / beta /
stable / LTS-surface / deprecated / removal-scheduled plus nightly / preview / beta /
stable / LTS truth across claimed M5 command, feature, workflow-bundle, extension /
install, and release / install surfaces.

## What this primitive is

Aureline's frozen badge-family matrix
(`schemas/ui/m5-badge-family-matrix.schema.json`, implemented in
`crates/aureline-release/src/freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`)
names the **lifecycle** badge and the **channel** badge as two of the six governed
badge families and freezes the shared badge infrastructure — the surface families,
deployment lines, accessibility routes, qualification classes, explanation-drawer
fields, consumer surfaces, and downgrade triggers.

This primitive *implements* those two families as one render-facing badge pair, so a
user can tell — from the two badges and their explanation drawers alone — **how mature**
a capability is (experimental, stable, deprecated, or scheduled for removal) *and*
**which channel** it is merely running on, without one badge implying the other.

It has two halves:

1. **A resolver** — `resolve_lifecycle_channel_badge` — that takes one capability's
   subject label, its declared lifecycle stage, its declared channel, an optional
   replacement / migration path, and its last-evaluated timestamp, and produces one
   `M5ResolvedLifecycleChannelBadge` carrying both badges as **separate typed fields**,
   the derived effective maturity, and — when the lifecycle is deprecated or
   removal-scheduled — a self-contained `M5MigrationNote` that points to the replacement
   path and preserves the channel context.
2. **A parity matrix** — `M5MaturityBadgePrimitivePacket` — that binds one row per
   claimed M5 badge consumer (command row, feature surface, workflow bundle, extension /
   install row, release / install surface, ecosystem lifecycle review) to the shared
   badge anatomy, lifecycle values, channel values, effective-maturity postures, sunset
   reasons, next actions, explanation-drawer fields, export fields, and non-visual
   accessibility routes.

## Controlled vocabulary

- **Lifecycle values** (render-facing): `labs`, `preview`, `beta`, `stable`,
  `lts_surface`, `deprecated`, `removal_scheduled`.
- **Channel values** (render-facing): `nightly`, `preview`, `beta`, `stable`, `lts`.
- **Effective-maturity postures** (derived from the lifecycle alone):
  `maturity_experimental`, `maturity_preview`, `maturity_beta`, `maturity_stable`,
  `maturity_long_term_supported`, `maturity_deprecated`, `maturity_removal_scheduled`.
- **Sunset reasons**: `deprecated`, `removal_scheduled`.
- **Next actions**: `follow_migration_path`, `complete_migration_before_removal`.

The badge surface family, deployment line, accessibility route, qualification class,
explanation-drawer field, consumer surface, and downgrade trigger are reused verbatim
from the frozen badge-family matrix; this primitive mints new vocabulary only for the
two rendered badges themselves.

## Acceptance criteria this primitive proves

- **Distinct cues (AC1)** — Users can tell whether a capability is experimental, stable,
  deprecated, or merely running on a preview channel without inferring meaning from
  unrelated badges. The effective maturity is derived from the lifecycle axis **alone**:
  a `stable` capability running on the `preview` channel is still stable, and a `beta`
  capability promoted to the `stable` channel is still beta. The
  `lifecycle_channel_distinction_unproven` lint requires at least one worked resolution
  where a stable-line lifecycle runs on a pre-release channel (or a pre-release lifecycle
  runs on the stable channel).
- **Migration path, never inert (AC2)** — Deprecated or removal-scheduled badges point to
  the replacement / migration path instead of becoming inert warnings. The resolver
  **refuses** to build a sunsetting badge without a replacement path
  (`MissingReplacementPath`), and the resulting `M5MigrationNote` carries the replacement
  path, the next action, and the **preserved channel** the capability was running on. The
  `migration_path_preservation_unproven` lint requires at least one worked resolution
  proving this.

## Hard invariants

Every badge row must set these to `false`, and the resolver enforces them:

- `collapses_lifecycle_and_channel_into_one_badge`
- `implies_channel_from_lifecycle`
- `drops_migration_path_on_deprecation`
- `drops_badge_meaning_in_export`

## Export safety

Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
bodies never cross this boundary. Every subject label, replacement path, and timestamp
is carried only as an opaque, export-safe representation.

## Source of truth

- Schema: `schemas/ui/m5-lifecycle-and-channel-badge.schema.json`
- Validator (authoritative): `crates/aureline-release/src/implement_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces`
- Support export: `artifacts/release/m5-lifecycle-and-channel-badge-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-lifecycle-and-channel-badge-proof/matrix.csv`
- Component report: `artifacts/components/m5-lifecycle-and-channel-badges.md`
- Narrowed fixtures: `fixtures/ui/m5-lifecycle-and-channel-badges/`

The Rust validator is the authoritative gate; the schema documents the shape.
