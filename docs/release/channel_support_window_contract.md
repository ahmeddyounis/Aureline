# Release-channel identity, support-window badge, and support-class vocabulary contract

This document is the narrative companion to the channel-identity and
support-window badge contract Aureline freezes for every surface that
renders release-channel and support-class state. It pins which channel
identities exist, which support classes exist (positive and refusal),
which support-window postures pair with each channel, where the badge
MUST be visible, and how channel and support claims stay conditioned by
archetype, client class, OS family, deployment profile, and
local-or-remote mode.

Companion artifacts:

- [`/schemas/release/support_window_badge.schema.json`](../../schemas/release/support_window_badge.schema.json)
  — boundary schema for one `support_window_badge_record` projecting
  channel identity, support-window posture, support class, scope
  envelope, evidence path, recovery guidance, and required surface set.
- [`/artifacts/release/support_class_rows.yaml`](../../artifacts/release/support_class_rows.yaml)
  — machine-readable support-class register binding one
  `support_class_row` per `support_class` value (certified, supported,
  community, experimental, not_certified_in_this_mode, not_configured,
  disabled_by_policy, not_supported).
- [`/fixtures/release/channel_support_cases/`](../../fixtures/release/channel_support_cases)
  — seed `support_window_badge_record` fixtures for the four required
  acceptance cases (stable supported, LTS certified, preview
  experimental, not_certified_in_this_mode).

Cross-linked artifacts already in the repository:

- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md)
  and
  [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
  — channel-and-branch matrix. Every badge's `channel_row_ref` resolves
  here. The channel-class vocabulary, the branch-class vocabulary, the
  support-window-class vocabulary, the freeze-posture admission matrix,
  and the side-by-side admission matrix are owned there; this document
  references them.
- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — exact-build identity record. The badge re-exports the
  `release_channel_class` enum verbatim (plus the two portable rows
  the install-topology matrix admits).
- [`/docs/governance/capability_axis_matrix.md`](../governance/capability_axis_matrix.md)
  and
  [`/artifacts/governance/capability_badge_axes.yaml`](../../artifacts/governance/capability_badge_axes.yaml)
  — seven-axis badge contract. The release-channel-identity badge here
  is the channel- and support-window-specific projection of the
  `axis:release_channel`, `axis:support_class`, and `axis:client_scope`
  axes. The seven-axis capability badge stays the authority for
  per-capability rendering; this badge is the per-build / per-channel
  projection.
- [`/schemas/release/release_candidate_card.schema.json`](../../schemas/release/release_candidate_card.schema.json)
  — release-status surface. The candidate card consumes one
  `support_window_badge_record` through its `support_window`,
  `compatibility_posture`, and `deprecation` slots.
- [`/schemas/release/whats_new_card.schema.json`](../../schemas/release/whats_new_card.schema.json)
  — release-notes / what's-new card. The card cites the badge by ref
  on every entry that names a channel.
- [`/schemas/release/assurance_claim.schema.json`](../../schemas/release/assurance_claim.schema.json)
  and
  [`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml)
  — assurance-claim matrix. The assurance row's `support_class` enum
  (certified / supported / community / experimental / not_supported) is
  a strict subset of the eight-value badge vocabulary here; refusal
  classes other than `not_supported` do not appear on assurance rows
  because assurance rows describe positive claims.
- [`/schemas/release/compatibility_row.schema.json`](../../schemas/release/compatibility_row.schema.json)
  and
  [`/docs/release/compatibility_report_template.md`](./compatibility_report_template.md)
  — compatibility-row schema and report template. Every badge whose
  support class is `supported` cites at least one supported
  compatibility row.
- [`/docs/release/certified_archetype_report_template.md`](./certified_archetype_report_template.md)
  — certified-archetype report. Every badge whose support class is
  `certified` cites at least one live certified-archetype report.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — deployment-profile vocabulary. The badge re-exports
  individual_local, self_hosted, enterprise_online, managed_cloud, and
  air_gapped values.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  — freshness-policy authority. Every certified or supported evidence
  path on the badge cites a freshness floor from this policy.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` §5.20 (release rhythm and channel
  discipline), §9.9 (mixed-version compatibility, negotiation, and
  upgrade posture), §9.12 (enterprise deployment hooks).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §25.9 (install,
  portable-mode, and fleet-rollout architecture), §26.5 (distributed
  compatibility and version-skew policy), §27.8–§27.9 (release widening
  and stable-facing claim movement).
- `.t2/docs/Aureline_Milestones_Document.md` §6.18 (install and update
  behaviour as product truth), §8.12 (release widening and stable-facing
  claim movement), §12.1.6 (LTS / backport posture).

## 1. Why publish this now

The channel-and-branch contract froze WHICH channels exist, WHICH
branches feed them, and WHICH side-by-side / freeze / downgrade rules
apply. The capability-axis matrix froze the seven-axis badge contract
for per-capability rendering. The assurance-claim schema froze the
positive support-class taxonomy used on assurance rows. The release-
status-surface contract froze the release-candidate card.

What was still implicit was the **per-build, per-channel
support-window badge** that carries channel identity, support class,
and support-window prominence onto the surfaces upgrade, adoption,
reporting, and migration decisions actually happen on:

- the **About panel** so users can see channel identity, version,
  build identity, and support-window state without leaving the
  product;
- the **update center** so upgrade decisions render the support
  window and end-of-support risk before the user clicks "update";
- the **issue / report packet** so support exports carry the channel
  identity and support class verbatim, not as free-text caveats;
- the **compatibility report** so compatibility rows render channel
  and support context alongside skew posture;
- the **migration / import workflow** so users importing from another
  install or another tool see the channel and support class of the
  destination before completing the import.

Left implicit, every surface would re-invent its own "you're on
preview" or "this is unsupported" copy and silently collapse the four
distinct refusal states (not_certified_in_this_mode, not_configured,
disabled_by_policy, not_supported) into one generic chip. Freezing the
contract now — before any surface lands — ends those failure modes.

This is a **pre-implementation plan**. No badge, About panel, update
center, issue-report exporter, compatibility-report renderer, or
migration / import workflow is implemented at this revision. Every row
in the support-class register is tagged `seeded` / `proposed`; rows are
not deleted, they are superseded by an ADR / RFC recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## 2. Scope

Frozen at this revision:

- One closed `release_channel_class` vocabulary mirroring
  `schemas/build/exact_build_identity.schema.json`'s
  `release_channel_class` plus the two portable-mode rows from the
  install-topology matrix.
- One closed `support_class` vocabulary with eight values: four
  positive support classes (certified, supported, community,
  experimental) ordered from strongest to weakest, and four DISTINCT
  refusal classes (not_certified_in_this_mode, not_configured,
  disabled_by_policy, not_supported).
- One closed `support_window_class` vocabulary aligned with the channel
  matrix plus the `not_applicable_refusal_state` value reserved for
  refusal classes.
- One closed `channel_badge_prominence_class` and
  `channel_guidance_tone_class` vocabulary so badge prominence and
  guidance tone are typed values, not free-text marketing.
- One closed `evidence_path_class` and `scoped_recovery_guidance_class`
  vocabulary so evidence backing and recovery guidance are typed
  values, not free-text caveats.
- One closed `surface_class` vocabulary with eleven values, of which
  five (about_panel, update_center, issue_report_packet,
  compatibility_report, migration_or_import_workflow) are the required
  surface floor.
- A scope envelope schema requiring every badge to bind at least one
  archetype ref, one client class, one OS family, one deployment
  profile, and one local-or-remote mode — so the support claim is
  never unbounded.

Out of scope until a superseding decision row opens:

- Authoring final release prose or operating release channels (the
  task spec marks both out of scope).
- Public marketing copy generation for support-class labels (the
  capability-axis matrix marks this out of scope and this contract
  inherits that posture).
- Wiring every live surface to the badge (each surface owns its own
  freeze; this contract publishes the badge they MUST consume).
- Cohort sizes, percentage rollouts, soak time values, or calendar
  cadence beyond the published support window. Those are
  benchmark-council / release-council authority.

## 3. Channel-identity vocabulary

Closed set. Mirrors
`schemas/build/exact_build_identity.schema.json`'s
`release_channel_class` plus the two portable-mode rows from the
install-topology matrix.

| Channel | Badge prominence | Support-window posture | Guidance tone |
|---|---|---|---|
| `dev_local` | `do_not_render_publicly` | `no_support_claim_dev_local` | `developer_local_only` |
| `nightly` | `do_not_render_publicly` | `no_support_claim_nightly_pre_release` | `experimental_no_support_claim` |
| `preview` | `medium_prominence_chip_with_disclosure` | `narrowed_window_pre_release` | `pre_release_warn_unverified` |
| `beta` | `medium_prominence_chip_with_disclosure` | `narrowed_window_pre_release` | `partner_pre_release_published_workarounds` |
| `stable` | `high_prominence_persistent_chip` | `rolling_window_stable` | `general_availability_supported` |
| `lts` | `high_prominence_persistent_chip` | `explicit_lts_window` | `long_term_support_calendar_bounded` |
| `hotfix` | `high_prominence_persistent_chip` | `hotfix_correction_only_window` | `correction_only_named_scope` |
| `portable_stable` | `medium_prominence_chip_with_disclosure` | `portable_no_managed_support` | `portable_self_contained_no_managed_support` |
| `portable_preview` | `medium_prominence_chip_with_disclosure` | `portable_no_managed_support` | `portable_self_contained_no_managed_support` |

`rc_candidate` is not a channel; it is a shiproom review-stage label
governed by
[`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml).
The badge here renders the carrier channel (beta / stable / lts /
hotfix), never `rc_candidate`.

### 3.1 Stable and LTS prominence

Stable and LTS rows MUST surface support-window and deprecation timing
**more prominently** than nightly or preview. The schema enforces this:

- the `channel_class ∈ {stable, lts}` rule pins
  `badge_prominence_class = high_prominence_persistent_chip`;
- the same rule pins `support_window_class ∈ {rolling_window_stable,
  explicit_lts_window}` and forbids the refusal-state window;
- the same rule requires `starts_at` to be non-null so the support
  window has a concrete public start.

Nightly and dev_local rows are pinned to `do_not_render_publicly` and
to the `experimental` or `not_supported` support classes; they MAY
render on internal release tooling but MUST NOT appear on the About
panel, update center, compatibility report, or migration workflow with
a positive support claim.

Preview and beta rows are pinned to medium prominence with the
narrowed-pre-release window; they cannot claim certified or supported.

## 4. Support-class register and refusal states

Closed eight-value vocabulary. The four positive classes are ordered
from strongest to weakest; the four refusal classes are pinned as
**distinct states with different recovery paths** so a generic
"unavailable" / "unsupported" / "disabled" chip is non-conforming.

| Support class | Backed by | Where it may render | Recovery routes through |
|---|---|---|---|
| `certified` | live certified-archetype report inside freshness floor | `stable`, `lts` | rollback to LTS floor; recertification gate; switch to certified archetype |
| `supported` | live supported compatibility row | `stable`, `lts`, `hotfix` | in-channel rollback; rollback to named floor; supported compatibility report |
| `community` | community evidence or partner attestation | `preview`, `beta`, `stable` | supported compatibility report; partner-published workaround; in-channel rollback |
| `experimental` | runtime observation only | `dev_local`, `nightly`, `preview`, `beta` | in-channel rollback; switch to certified or supported mode; no in-product recovery (dev_local) |
| `not_certified_in_this_mode` | no evidence required (refusal) | every channel | switch to certified archetype; open compatibility report; recertification gate |
| `not_configured` | no evidence required (refusal) | every channel | configure required provider or workspace |
| `disabled_by_policy` | no evidence required (refusal); cites policy ref | every channel | request managed-admin action or unblock policy |
| `not_supported` | no evidence required (refusal) | every channel | in-channel rollback; rollback to named floor or LTS train |

### 4.1 The four distinct refusal states

These four states are kept distinct because they route to four
**different** recovery paths. Surfaces that render any of these four as
the same generic chip break the contract. The schema enforces:

- `not_certified_in_this_mode` MUST quote the
  `switch_to_certified_archetype_or_supported_mode` guidance class.
  Distinct from `not_supported` because a certified or supported
  posture **does** exist for some scope envelope; the consuming surface
  is simply outside it.
- `not_configured` MUST quote the
  `configure_required_provider_or_workspace` guidance class. Distinct
  from `disabled_by_policy` because nothing is blocked; configuration
  is missing. Distinct from `not_supported` because support exists
  once configuration is complete.
- `disabled_by_policy` MUST quote the
  `request_managed_admin_action_or_unblock_policy` guidance class AND
  cite at least one policy ref. Distinct from `not_supported` because
  the capability would otherwise be supported. A silent "disabled"
  chip without a policy ref is non-conforming.
- `not_supported` is reserved for the case where the consuming surface
  cannot be reached by configuration, by policy unblock, or by
  switching to a different scope envelope. It MUST NOT be used as a
  stand-in for the other three.

### 4.2 Required evidence path per support class

Every positive support class MUST carry at least one matching evidence
path; refusal classes MUST carry the
`no_evidence_required_refusal_state` evidence path:

| Support class | Required evidence path | Freshness floor required |
|---|---|---|
| `certified` | `certified_archetype_report` | yes |
| `supported` | `compatibility_report_supported` | yes |
| `community` | `community_evidence_or_partner_attestation` | yes |
| `experimental` | `experimental_runtime_observation_only` | yes |
| `not_certified_in_this_mode` | `no_evidence_required_refusal_state` | n/a |
| `not_configured` | `no_evidence_required_refusal_state` | n/a |
| `disabled_by_policy` | `no_evidence_required_refusal_state` | n/a |
| `not_supported` | `no_evidence_required_refusal_state` | n/a |

The schema enforces these pairings: a certified badge that does not
cite a certified-archetype report with a non-empty evidence_refs list
and a non-null freshness_floor_ref is non-conforming; a supported
badge that does not cite at least one supported compatibility row is
non-conforming.

## 5. Support-window prominence ordering

Pinned by
[`/artifacts/release/support_class_rows.yaml`](../../artifacts/release/support_class_rows.yaml)`#support_window_prominence_ordering`.
Rightmost is most prominent:

```
no_support_claim_dev_local
  < no_support_claim_nightly_pre_release
    < narrowed_window_pre_release
      < portable_no_managed_support
        < hotfix_correction_only_window
          < rolling_window_stable
            < explicit_lts_window
```

`not_applicable_refusal_state` is an overlay reserved for the four
refusal support classes; it does not participate in the ordering. The
explicit LTS window is the most prominent because the calendar bound
and end-of-support risk are public commitments.

## 6. Required surfaces

Every `support_window_badge_record` MUST render onto at least these
five surfaces:

1. **About panel** (`about_panel`) — channel chip, support class chip,
   support-window state, end-of-support risk, exact-build identity ref,
   and link to the certified-archetype or supported compatibility
   report.
2. **Update center** (`update_center`) — same five fields as About,
   plus the named recovery action (rollback, recertification gate,
   configuration step, policy unblock route).
3. **Issue / report packet** (`issue_report_packet`) — verbatim copy
   of the channel chip, support-class chip, support-window state, and
   scope envelope so support engineers do not have to reconstruct
   them from prose.
4. **Compatibility report** (`compatibility_report`) — channel chip
   and support-class chip rendered alongside compatibility rows;
   refusal classes render the typed refusal label rather than a
   generic "unsupported" chip.
5. **Migration / import workflow** (`migration_or_import_workflow`) —
   destination channel chip, support-class chip, and support-window
   state visible BEFORE the user completes a migration or import so
   destination posture is never silently widened.

Admissible secondary surfaces (not required but admitted by the
schema): `release_evidence_packet`, `support_export_bundle`,
`release_notes_card`, `service_health_panel`,
`command_palette_install_review`, `marketplace_listing`. Tooling MAY
add more required surfaces in a later additive-minor revision; it MUST
NOT drop one.

## 7. Scoping rules

The badge's `scope_envelope` requires every record to bind at least one
value across five axes:

- `archetype_refs` — certified-archetype, launch-bundle, compatibility-
  archetype, or workflow-bundle refs the support claim is bounded by.
- `client_classes` — desktop_product, cli, remote_agent,
  headless_agent, managed_admin_surface, docs_site, support_center,
  companion_surface, sdk_or_api.
- `os_families` — windows, macos, linux, platform_native_any (for
  archetype rows whose claim holds across every supported desktop OS),
  not_applicable_remote_or_managed (for remote-agent / managed-cloud
  rows).
- `deployment_profiles` — individual_local, self_hosted,
  enterprise_online, managed_cloud, air_gapped.
- `local_or_remote_modes` — local_workstation, remote_agent_attached,
  remote_workspace_only, managed_cloud_session, air_gapped_local,
  portable_self_contained, not_applicable_dev_local.

A capability that is certified locally but only experimental over a
remote agent emits TWO badges: one with
`local_or_remote_modes: [local_workstation]` and `support_class:
certified`, the other with `local_or_remote_modes:
[remote_agent_attached]` and `support_class: experimental` (or
`not_certified_in_this_mode` if the remote-agent mode is outside the
certified envelope). The schema's scope envelope makes this rule
mechanical; no badge can quietly straddle two scope envelopes with one
record.

## 8. Linkage into other control artifacts

- **Channel matrix.** Every `channel_identity.channel_row_ref` resolves
  to a `channel_row` in
  [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml).
  The channel-class vocabulary, the branch-class vocabulary, the
  support-window-class vocabulary, and the freeze-posture admission
  matrix are owned there.
- **Exact-build identity.** Every
  `channel_identity.exact_build_identity_ref` resolves to an
  exact-build identity record validated against
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json).
- **Capability axis matrix.** The badge here is the per-build,
  per-channel projection of the seven-axis capability badge frozen by
  [`/docs/governance/capability_axis_matrix.md`](../governance/capability_axis_matrix.md).
  Per-capability rendering reads the seven-axis badge; per-build /
  per-channel rendering reads this badge.
- **Assurance claim matrix.** The four positive support classes here
  (certified, supported, community, experimental) align with the
  five-value assurance-row support_class enum (certified, supported,
  community, experimental, not_supported). Assurance rows do not carry
  the three configuration / policy / scope refusal classes because
  assurance rows describe positive claims; the badge does because the
  badge has to render refusal state.
- **Compatibility-report template.** Every supported badge cites at
  least one row from the compatibility-report template at
  [`/docs/release/compatibility_report_template.md`](./compatibility_report_template.md).
- **Certified-archetype-report template.** Every certified badge cites
  at least one row from the certified-archetype-report template at
  [`/docs/release/certified_archetype_report_template.md`](./certified_archetype_report_template.md).
- **Release-candidate card.** The release-candidate card consumes the
  badge through its `support_window`, `compatibility_posture`, and
  `deprecation` slots; the candidate card does not invent a parallel
  channel-identity or support-class vocabulary.
- **What's-new card.** The what's-new card consumes the badge by ref
  on every entry that names a channel; the card does not invent a
  parallel channel chip.
- **Evidence freshness policy.** Every certified or supported evidence
  path on the badge cites a freshness floor from
  [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md).
  A badge whose evidence path freshness has fallen below the floor
  narrows automatically (the schema's
  `evidence_path.freshness_floor_ref` is non-null on positive classes;
  consumers compose freshness with the worst-supporting-truth-wins
  rule from
  [`/artifacts/governance/claim_propagation_rules.yaml`](../../artifacts/governance/claim_propagation_rules.yaml)).

## 9. Failure modes prevented

1. *About panel and update center invent their own "you're on preview"
   chip with different copy.* — Refused by the
   `channel_badge_prominence_class` and `channel_guidance_tone_class`
   vocabularies on the `channel_identity` block.
2. *Stable and LTS render the support window with the same prominence
   as a preview build.* — Refused by the schema's `channel_class ∈
   {stable, lts}` rule pinning `high_prominence_persistent_chip` and
   the support-window prominence ordering.
3. *Preview claims certified or supported.* — Refused by the schema's
   `channel_class ∈ {preview, beta}` rule restricting the support
   class to community / experimental / not_certified_in_this_mode.
4. *The four refusal states render as one generic "unavailable" chip.*
   — Refused by the per-class guidance-route requirements
   (`switch_to_certified_archetype_or_supported_mode`,
   `configure_required_provider_or_workspace`,
   `request_managed_admin_action_or_unblock_policy`) and by the
   forbidden-combination rule
   `forbidden.refusal_class_collapsed_to_generic_chip` in
   [`/artifacts/release/support_class_rows.yaml`](../../artifacts/release/support_class_rows.yaml).
5. *A disabled-by-policy badge renders without a policy ref.* — Refused
   by `forbidden.disabled_by_policy_without_policy_ref`.
6. *A capability claims certified locally and silently inherits the
   claim over a remote agent.* — Refused by the scope envelope's
   `local_or_remote_modes` slot; two scope envelopes require two
   badges.
7. *A migration / import workflow imports into a preview build without
   showing the destination's support class.* — Refused by the
   `migration_or_import_workflow` entry in the required surface set
   floor.
8. *An issue / report packet drops the channel chip and renders only
   the build version.* — Refused by the `issue_report_packet` entry in
   the required surface set floor.
9. *A certified badge presents without a fresh certified-archetype
   report.* — Refused by the schema's certified-class rule requiring
   `evidence_path_class: certified_archetype_report`,
   `evidence_refs.minItems: 1`, and a non-null freshness floor.
10. *A nightly or dev_local build shows up on the About panel with a
    positive support claim.* — Refused by the schema's `channel_class
    ∈ {nightly, dev_local}` rule pinning
    `do_not_render_publicly` prominence and limiting the support class
    to experimental or not_supported.

## 10. Acceptance criteria mapped to evidence

The acceptance criteria from the spec map to the schema, register, and
fixtures as follows:

- *Stable and LTS flows surface support-window and deprecation timing
  more prominently than Nightly or Preview.* — Pinned by §3 (Channel-
  identity vocabulary), §5 (Support-window prominence ordering), the
  schema's per-channel allOf rules, and the
  [`stable_supported_general_availability_desktop`](../../fixtures/release/channel_support_cases/stable_supported_general_availability_desktop.yaml)
  and
  [`lts_certified_enterprise_calendar_window`](../../fixtures/release/channel_support_cases/lts_certified_enterprise_calendar_window.yaml)
  fixtures.
- *`Not certified in this mode`, `Not configured`, `Disabled by
  policy`, and `Not supported` remain distinct states with different
  recovery paths.* — Pinned by §4 (Support-class register), §4.1 (The
  four distinct refusal states), the schema's per-refusal-class
  guidance-route rules, the `forbidden_combinations` block in
  [`/artifacts/release/support_class_rows.yaml`](../../artifacts/release/support_class_rows.yaml),
  and the
  [`not_certified_in_this_mode_remote_agent_attached`](../../fixtures/release/channel_support_cases/not_certified_in_this_mode_remote_agent_attached.yaml)
  fixture.
- *Fixtures cover at least: Stable supported, LTS certified, Preview
  experimental, and Not certified in this mode cases.* — Provided in
  [`/fixtures/release/channel_support_cases/`](../../fixtures/release/channel_support_cases).

## 11. Change control

- Adding a `release_channel_class`, `support_class`,
  `support_window_class`, `channel_badge_prominence_class`,
  `channel_guidance_tone_class`, `evidence_path_class`,
  `scoped_recovery_guidance_class`, `surface_class`,
  `deployment_profile_class`, `client_class`, `os_family_class`,
  `local_or_remote_mode_class`, or `end_of_support_risk_class` value is
  additive-minor. Adding an entry requires bumping the schema
  `support_window_badge_schema_version`, extending the support-class
  register with the new row, and updating this document in the same
  change.
- Repurposing an existing vocabulary value is breaking. Repurposing
  requires a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and the concurrence of the release council.
- Adding a new required surface to the floor (currently five) is
  breaking; consumers MAY add admissible secondary surfaces without a
  schema bump.
- Adding a new positive support class requires re-checking the
  assurance-claim schema's support_class enum in the same change so
  the two stay aligned. Adding a new refusal support class requires
  publishing the distinct recovery path for the new class in the same
  change; refusal classes that share a recovery path with an existing
  class are non-conforming.

## 12. Status

Contract is **seeded**. Every row in the support-class register and
every fixture is tagged `seeded` / `proposed`. Rows are not deleted;
they are superseded by a follow-on ADR / RFC recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
