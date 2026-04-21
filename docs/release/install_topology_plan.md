# Install topology, fleet-ring, and state-root mapping plan

This document is the pre-implementation plan for Aureline's install
topology, per-channel separation rules, fleet-ring promotion ladder,
and state-root mapping. It exists so packaging, portable mode,
side-by-side channels, fleet rollout, mirror and air-gap lanes, silent
deployment, and state-root diagnostics do not surprise the release
lane once code lands.

Companion artifacts:

- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  — machine-readable matrix binding one `install_profile_card_record`
  per `(install_mode_class, channel_class, platform_class,
  architecture_class)` tuple.
- [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  — machine-readable state-root map with per-channel separation rules
  for state roots, update markers, recent-item registration, file
  associations, deep-link / protocol-handler ownership, and
  scriptability / diagnostics / repair expectations.
- [`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml)
  — machine-readable return-code family seed plus worked
  `unattended_deployment_result_record` fixtures covering install,
  update, rollback, uninstall, and verify-failed outcomes.
- [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  — shared continuity, mirror/offline, and impairment drill catalog
  seed the install-profile cards, rollout evidence, and support/export
  lanes cite by stable `drill_id`. Companion narrative in
  [`/docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md).

Normative sources this plan projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §25.9
  ("Install, portable-mode, and fleet-rollout architecture"), Appendix
  BA (platform installation matrix and fleet rollout rings), and §B.2
  (stable CLI exit-code family).
- `.t2/docs/Aureline_Milestones_Document.md` §6.18 (install and update
  behaviour as product truth; state-root audit; silent / managed
  deployment; diagnostics exposure without leaking secrets).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.12 (About, update, and
  diagnostics surfaces; first-run import; portable-mode labelling).
- `.t2/docs/Aureline_PRD.md` §9.12 (enterprise deployment hooks; MDM /
  GPO-friendly install and removal).

## Why publish this now

The reproducible-build baseline and exact-build identity model
(see [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md))
answer "what build is this?" for one artifact at a time. They do not
answer:

- *which install topology* (per-user, per-machine, portable,
  offline-bundle, managed-fleet, side-by-side-preview) is carrying
  the build,
- *which channel* (stable, preview, beta, LTS, portable-stable,
  portable-preview) the user opted into,
- *who owns the updater* for this install (user, admin, external
  package manager, managed fleet, nobody),
- *which durable state roots* the install reads and writes and which
  state roots it MUST NOT touch,
- *which fleet-rollout ring* the install is pinned to, and what
  promotion or rollback evidence is expected before it advances,
- *which mirror / air-gap posture* the install was acquired from
  (online vendor, offline signed bundle, customer-managed mirror,
  third-party package index), or
- *which silent-install return code* a failed unattended install,
  update, or rollback resolves into so enterprise deployment tools
  can remediate without screen-scraping free-text error strings.

Left implicit, every release, fleet-rollout, and support surface
would re-invent this vocabulary. The benchmark lab would quarantine
runs by install mode inconsistently, support-bundles would describe
the install topology in free-text, enterprise admins would discover
side-by-side channel collisions in production, and rollback surfaces
would not know whether they are rolling the per-user or the per-
machine install. Freezing the vocabulary now — *before* any
installer code lands — ends those failure modes.

This is a **pre-implementation plan**. No installer, updater, or
fleet tooling is implemented at this milestone. Every row below is
tagged `proposed`; rows are not deleted, they are superseded by an
ADR / RFC.

## Scope

Frozen at this revision:

- One `install_profile_card_record` shape that every release, fleet,
  About / Help, support-bundle, and diagnostics surface reads before
  describing "the install" to a human or to an enterprise deployment
  tool. The record carries platform class, architecture class, install
  mode class, channel class, updater owner class, binary root class,
  durable state roots, side-by-side relation class, rollback target
  class, diagnostics / export path class, rollout ring class,
  silent-install support class, managed-package-report class,
  publication posture class, policy injection class, and the
  supported return-code family ids.
- Closed vocabularies for install mode, channel, updater owner,
  binary root, durable state root, side-by-side relation, rollback
  target, diagnostics / export path, rollout ring, silent-install
  support, managed-package report, publication posture, policy
  injection, and return-code family.
- Per-channel separation rules for state roots, update markers,
  recent-item registration, file associations, and deep-link /
  protocol-handler ownership. "No row silently corrupts another
  row's state markers" is pinned as a schema-level rule, not a
  guideline.
- Fleet-rollout ring vocabulary (`canary`, `pilot`, `broad`, `lts`)
  with the minimum promotion evidence and the admissible rollback
  target for each ring.
- State-root mapping for local, preview, portable, and managed-
  install styles with notes on scriptability, diagnostics visibility,
  repair / verify expectations, and exact-build install diagnostics.
- Silent-install return-code families and the
  `unattended_deployment_result_record` shape every unattended
  install, update, pin, rollback, or uninstall resolves into.

Out of scope until a superseding decision row opens:

- The installer, updater, or fleet-rollout tooling itself.
- Final signing / notarization workflows. The identity-model schema
  pins the signing-class and signing-material-state vocabulary every
  install topology row reads (see
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json));
  the actual key material, transparency-log bridge, and
  notarization pipeline are later lanes.
- Platform-store review / notarization agreements. Rows reserve the
  `publication_posture_class` so a `third_party_package_index` row
  (for example `winget`, `homebrew`, an enterprise package mirror)
  composes later without a schema bump.
- Binary in-process resolvers for install-profile cards, update
  markers, or state-root audits. This plan freezes the record shape
  every surface projects from; the resolver is implementation.
- MDM / Intune / GPO policy bundle contents. Rows reserve
  `policy_injection_class` slots; the policy-bundle schema is a later
  lane co-owned with security / trust review.
- Fleet-ring cohort sizes, percentage rollouts, or bake-in times.
  The ring vocabulary is frozen; the operational knob values are
  benchmark-council / release-council authority.

## No-collision rule

The plan requires, on every install-profile card:

- exactly one `install_mode_class`,
- exactly one `channel_class`,
- exactly one `updater_owner_class`,
- one `binary_root_class` and one or more `durable_state_root_refs`,
- one `side_by_side_relation_class`,
- one `rollout_ring_class`,
- one or more `diagnostics_export_class` entries,
- one `silent_install_support_class` and one
  `managed_package_report_class`,
- one `publication_posture_class`, and
- a reserved `running_build_identity_ref` into
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  so the install-profile card resolves into one exact-build identity
  record, not a free-text version string.

Cards that cannot resolve any of the above are non-conforming. Cards
that collide on a durable state root with a card of a different
`channel_class` are non-conforming by construction (see the state-root
map's `collision_policy: no_shared_durable_state_across_channels`
rule). This is how the manifest encodes *"no row silently corrupts
another row's state markers"* as a schema-level constraint rather than
a guideline.

## Install-mode vocabulary

Closed set. Adding a mode is additive-minor and opens a decision row
in [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

| Install mode class | Summary | Admin required | Binary root location | Portable state |
|---|---|---|---|---|
| `per_user_installed` | Installed into the user's profile; no admin required. | no | per-user profile area | no |
| `per_machine_installed` | Installed into a machine-global program directory; admin installs, all users share the binary. | yes | machine-global program area | no |
| `portable` | Self-contained directory with binary and state colocated; no registry entries, no global service registrations, no machine-wide mutation. | no | portable directory | yes |
| `offline_bundle` | Signed offline bundle containing binary plus mirror metadata plus policy / bootstrap artifacts, verifiable without vendor network access. | optional | offline bundle extracted into per-user or per-machine root | optional |
| `managed_deployed` | Delivered via MDM / Intune / GPO / enterprise software distribution to a managed fleet. | yes | machine-global program area or admin-pinned per-user area | no |
| `side_by_side_preview` | Preview or beta install coexisting with a stable install on the same host; distinct binary roots and distinct durable state roots. | varies | per-channel suffixed root | no |

Every claimed platform in the install-topology matrix resolves each
claimed `install_mode_class` to a concrete platform-specific
installer family (MSI / MSIX / PKG / DMG / tarball / AppImage /
portable ZIP / offline bundle). The installer choice is named in the
matrix; the matrix is authoritative for tooling.

## Channel vocabulary

Closed set.

| Channel class | Summary | Side-by-side with | Admitted install modes |
|---|---|---|---|
| `stable` | Default production channel. | `preview`, `beta`, `lts` | every install mode |
| `preview` | Early-access pre-release; narrows compatibility claims. | `stable`, `beta`, `lts` | every install mode |
| `beta` | Optional staging channel between `preview` and `stable`. Reserved. | `stable`, `preview`, `lts` | every install mode |
| `lts` | Long-term-support branch for enterprise deployment with an explicit support window. | `stable`, `preview`, `beta` | `per_machine_installed`, `managed_deployed`, `offline_bundle` |
| `portable_stable` | Stable channel running in portable mode. | portable-stable is side-by-side with installed `stable` | `portable` only |
| `portable_preview` | Preview channel running in portable mode. | portable-preview is side-by-side with installed `preview` | `portable` only |

`lts` is reserved but unpopulated at this milestone; the
benchmark-council / release-council charter ratifies promotion of the
first LTS cut. The vocabulary is frozen now so later enterprise rows
resolve into it without a schema bump.

## Updater-owner vocabulary

Closed set. The install-profile card MUST carry exactly one updater
owner so the About / Help dialog, support bundle, and fleet-console
view agree on "who can update this install".

- `user` — the user running the install; auto-update is reachable from
  within the product.
- `admin` — a machine administrator; auto-update for this install is
  admin-controlled.
- `external_package_manager` — a package manager owns the update path
  (for example Homebrew, `winget`, a Linux distro package manager).
  In-product auto-update is disabled for this install.
- `managed_fleet` — an MDM / Intune / GPO / enterprise software
  distribution system owns the update path. In-product auto-update
  is suppressed and channel pinning is enforced.
- `none` — no updater (for example a portable extraction used once
  from read-only media). Update must be manual.

`external_package_manager` and `managed_fleet` rows MUST carry a
`managed_package_report_class` of `available` or `reserved` so the
fleet console has a defined path to read "what version of Aureline is
this host running?" without scraping the binary.

## Fleet-rollout rings and promotion evidence

Closed ring set with minimum promotion evidence per ring. Rings are
ordered; a build may not promote to ring *N+1* without evidence that
it has passed ring *N*.

| Ring | Purpose | Admissible channels | Minimum promotion evidence |
|---|---|---|---|
| `canary` | Internal validation across every claimed platform / architecture pair. | `stable`, `preview`, `beta` | install smoke, launch smoke, uninstall smoke, rollback smoke, policy / bootstrap bundle check, exact-build identity propagation check |
| `pilot` | Design partners and admin-controlled early adopters. | `stable`, `preview`, `beta` | canary evidence plus side-by-side coexistence drill, support-bundle install-diagnostics probe, deprovision / rollback drill |
| `broad` | Default stable population. | `stable`, `preview` | pilot evidence plus green release-evidence promotion for every admitted install-profile card and a current install-topology matrix |
| `lts` | Long-term-support enterprise deployment. | `lts` | broad evidence plus explicit rollback target, documented migration notes, offline / mirror compatibility proof, and policy-bundle compatibility proof |

Rollback targets per ring are pinned in the install-topology matrix:
`canary` rolls back to the previous `canary` cut or out of the ring,
`pilot` rolls back to the previous `pilot` cut or to the last `broad`
cut of the same channel, `broad` rolls back to the last `broad` cut,
`lts` rolls back to the channel's current LTS floor (never across
the LTS floor without a release-council waiver).

Mirror, air-gap, self-hosted, and managed continuity claims on an
install-profile card should resolve into named drill rows from the
deployment drill catalog seed rather than free-text outage notes. The
install-topology matrix owns *which* profile is claimed; the drill
catalog owns *what degradation was exercised* and *what local-safe
fallback remained true*.

Rollout rings are **operational stances**, not install modes. A
single install-profile card carries exactly one `install_mode_class`
and exactly one `rollout_ring_class` at a time; moving a host from
one ring to another is an admin or user decision recorded on the
support bundle, not a side-effect of an install.

## State-root mapping

The state-root map binds each `install_mode_class × channel_class`
pair to the set of durable state roots the install reads and writes.
Every state-root row carries:

- `state_root_id` — stable id, snake_case, no milestone slugs.
- `owning_install_modes` — install modes that read / write this root.
- `owning_channels` — channels that read / write this root. Cross-
  channel reads are forbidden unless the row is flagged
  `shared_across_channels: true` with a named reason.
- `authority_class` — resolves into the portable-profile state-map
  classes frozen in
  [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  (user-authored durable truth, user-owned recovery state, admin /
  control artifact, disposable derived cache).
- `scriptability_class` — one of `cli_exposed`, `diagnostics_only`,
  `admin_only`, `not_exposed`. Enterprise deployment requires CLI or
  admin access to every admin-class state root.
- `diagnostics_visibility_class` — one of `exposed_in_about`,
  `exposed_in_support_bundle`, `exposed_in_admin_console_only`,
  `not_exposed`. The About / Help dialog and the support bundle MUST
  expose the state-root path for every user-authored and recovery
  state root on the install (see UI / UX spec §6.12).
- `repair_verify_class` — one of `doctor_repairable`,
  `reinstall_repairable`, `admin_rebuild_required`,
  `not_repairable`. Rows with `admin_rebuild_required` MUST carry a
  linked `policy_injection_class` other than `none`.
- `exact_build_install_diagnostic_class` — one of
  `exact_build_manifest_present`, `exact_build_manifest_reserved`,
  `exact_build_manifest_not_applicable`. Every installed binary root
  row resolves `running_build_identity_ref` into one exact-build
  manifest so install diagnostics can cite the exact build without
  re-inventing the version string.

### Local (per-user and per-machine installed)

Binary root lives under the platform-specific program directory.
Durable state roots are the user-authored workspace files (in the
workspace repo, outside the install tree) plus the user profile
(configuration, recovery state, disposable derived cache). The
install tree MUST NOT be mutated by normal product use; the binary
root is read-only relative to the running process (repair and
reinstall are the exceptions, and both clear and re-seed it).

### Preview (side-by-side with stable)

Binary root lives under a channel-suffixed program directory
(`Aureline Preview` on macOS / Windows, an `aureline-preview`
directory on Linux). Durable state roots are channel-suffixed so
preview and stable cannot collide: `aureline.preview.*` instead of
`aureline.stable.*`. Recent-item registration, file associations,
and deep-link / protocol-handler ownership are channel-suffixed per
the per-channel separation rules below. Preview MUST NOT write into
any durable state root owned by `stable`; stable MUST NOT write into
any durable state root owned by `preview`. The state-root map
enforces this by pinning `owning_channels` to one channel per row
unless the row is explicitly `shared_across_channels: true` (for
example the platform machine-identity root on macOS, which is shared
across all installs on the host but treated read-only by Aureline).

### Portable

Binary root and durable state roots are colocated under the portable
directory. The portable root MUST NOT spill into any user-profile
state root or machine-global state root. Portable mode MUST suppress
or clearly label any request that would create a machine-global
integration (file association, protocol handler, recent-item
registration that persists beyond the portable root). Portable state
is discoverable from within the product — About / Help exposes the
portable root, and the CLI exposes it via an install-diagnostics
command reserved in the matrix. Portable mode registers zero
durable state roots outside the portable directory; the state-root
map's portable row set is strictly self-contained.

### Managed (per-machine-installed under MDM / Intune / GPO)

Binary root lives under a machine-global program directory owned by
the admin. Durable state roots are the union of machine-wide admin /
policy roots (admin-authored; read-only from the user's perspective)
and per-user profile roots (user-authored durable truth plus user-
owned recovery state plus disposable derived cache, same as local
per-user rows). Policy injection is admin-only and the install-
profile card carries a `policy_injection_class` other than `none`.
Managed rows MUST expose a managed-package report (`available` or
`reserved` at this milestone) so the fleet console can read the
install without the product running.

## Per-channel separation rules

Frozen. Applies to every install-profile card whose
`side_by_side_relation_class` is any value other than `none`.

### State roots

- Every durable state root carries exactly one `owning_channel` unless
  the row is explicitly `shared_across_channels: true` with a named
  reason (for example a platform keychain row the user owns across
  channels).
- Stable MUST NOT read or write preview durable state roots. Preview
  MUST NOT read or write stable durable state roots. The same holds
  for every other channel pair.
- Portable MUST NOT read or write any installed channel's durable
  state roots. Installed channels MUST NOT read or write the
  portable root.

### Update markers

- Every channel owns a distinct update-marker file under its own
  binary root or durable state root tree. No channel reads or writes
  another channel's update-marker.
- Preview's update marker includes the preview channel suffix.
  Stable's update marker is unsuffixed but namespaced under the
  stable install tree.
- Portable's update marker lives under the portable directory; it
  does not touch any machine-global update-marker.

### Recent-item registration

- Each channel registers recent items under its own channel-suffixed
  namespace (for example `com.aureline.stable.recent` versus
  `com.aureline.preview.recent`).
- A side-by-side install of stable and preview MUST render two
  recent-item lists, not one merged list.
- Portable mode MUST NOT write recent-item registrations into any
  machine-global OS recent-items list; the portable recent-item
  list is self-contained.

### File associations

- Each channel's installer MAY register as a candidate handler for
  a given file extension; the *default* handler is user- or admin-
  selectable.
- The installer MUST NOT silently override an existing default
  handler. "Last-writer-wins" is forbidden by schema (the matrix's
  `default_file_association_rule: user_or_admin_selectable_never_last_writer_wins`
  pins this).
- Portable mode MUST NOT register any file association on the host.

### Deep-link / protocol-handler ownership

- Each channel registers its protocol handler under a channel-
  suffixed URL scheme (for example `aureline-stable://` and
  `aureline-preview://`). A shared `aureline://` scheme resolves to
  the default channel chosen by the user or admin; installers MUST
  NOT silently claim the shared scheme.
- Portable mode MUST NOT register any protocol handler on the host.
- Deep links that arrive at a wrong-target channel (for example a
  link to `aureline-preview://…` opened on a host where only stable
  is installed) MUST fail with a typed result code from the silent-
  deployment return-code family, not silently redirect to the wrong
  channel.

### Silent-deployment return-code families

- Every claimed channel on every claimed install mode MUST resolve
  unattended install, update, pin, rollback, and uninstall outcomes
  into one of the return-code families frozen in the silent-
  deployment seed. Free-text error strings alone are non-conforming.
- Return-code families are channel-agnostic; the channel is carried
  on the install-profile card, not the return code.

## Scriptability, diagnostics, and repair / verify expectations

Every install-profile card resolves to a concrete answer for each of:

- **Scriptability.** What CLI surface is available on the install?
  Closed values: `cli_present_in_path`, `cli_present_in_install_tree`,
  `cli_portable_only`, `cli_not_exposed`. Managed-fleet rows MUST
  carry `cli_present_in_path` or `cli_present_in_install_tree` so
  admin tooling can probe without launching the GUI.
- **Diagnostics visibility.** What install-diagnostics output is
  available from the product and from the CLI? Closed values:
  `install_topology_record_available`, `state_root_audit_available`,
  `exact_build_manifest_available`, `artifact_graph_pack_available`,
  `support_bundle_available`. Every row MUST expose at least
  `install_topology_record_available` and
  `exact_build_manifest_available`; rows that cannot are
  non-conforming at this milestone.
- **Repair / verify.** What repair / verify operations are supported?
  Closed values: `repair_install`, `verify_install_signature`,
  `verify_state_root_integrity`, `rollback_to_previous_build`,
  `uninstall_clean_state`. Managed-fleet rows MUST support
  `rollback_to_previous_build` to the current ring's rollback target.
- **Exact-build install diagnostics.** Every installed binary root
  MUST expose an exact-build manifest resolvable from the install-
  profile card's `running_build_identity_ref`. Install diagnostics
  do not re-invent the version string; they cite one exact-build
  identity record (see
  [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)).

Diagnostics output MUST NOT leak secrets (per milestone §6.18). The
install-diagnostics record carries a redaction class resolving into
the redaction and secret classes frozen in
[`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
(ADR-0007).

## Mirror / air-gap publication posture

The `publication_posture_class` on each install-profile card is one
of:

- `online_vendor` — artifacts are fetched from the vendor's online
  release channel.
- `offline_signed_bundle` — artifacts arrive as a signed offline
  bundle extracted on the host. Signatures, compatibility metadata,
  revocations, and policy / bootstrap artifacts are carried in the
  bundle so verification runs without vendor network access.
- `customer_managed_mirror` — artifacts are fetched from a customer-
  managed mirror that preserves signatures, compatibility metadata,
  revocations, and policy / bootstrap artifacts.
- `third_party_package_index` — artifacts are delivered via an
  external package manager (`winget`, Homebrew, a Linux distro
  package index). Update ownership on this row is
  `external_package_manager`.

Offline and mirror postures MUST remain usable without vendor network
access. The matrix reserves `mirror_metadata_integrity_class` and
`offline_expiration_at` slots on `offline_bundle` and
`customer_managed_mirror` rows so later release-evidence packets
carry offline expiry and mirror integrity without retrofitting.

## Failed unattended install / update / rollback result shape

The silent-deployment seed defines one
`unattended_deployment_result_record` every failed unattended
install, update, pin, rollback, or uninstall resolves into. Fields:

- `result_kind` — one of `install`, `update`, `pin`, `rollback`,
  `uninstall`, `verify`.
- `result_status` — one of `success`, `partial_success`, `failed`,
  `rolled_back`, `verify_failed`, `reboot_required`.
- `return_code_family` — id from the return-code family set
  (`success`, `partial_success`, `user_config_error`,
  `trust_policy_denial`, `missing_dependency`, `network_transport`,
  `internal_failure`, `rollback_required`, `verification_failed`,
  `admin_required`).
- `return_code_numeric` — the numeric exit code admitted by the
  return-code family. Admissible values are enumerated per family in
  the seed and align with TAD §B.2.
- `failure_reason_class` — one of a closed failure-reason set
  (`signature_invalid`, `signing_material_revoked`,
  `mirror_metadata_stale`, `policy_denied`, `disk_space_exhausted`,
  `path_permission_denied`, `state_root_collision`,
  `side_by_side_marker_corruption`, `portable_spill_detected`,
  `network_unreachable_offline_required`, `missing_bootstrap_bundle`,
  `toolchain_missing`, `unsupported_platform`, `internal_bug`,
  `user_cancelled`).
- `install_profile_card_ref` — ref into the install-topology matrix
  row the outcome applies to.
- `running_build_identity_ref` — ref into the exact-build identity
  record the outcome applies to, if any.
- `remediation_pointer_class` — one of a closed pointer set
  (`run_project_doctor`, `inspect_support_bundle`,
  `refresh_mirror_metadata`, `contact_admin`, `reissue_policy_bundle`,
  `retry_from_offline_bundle`, `reinstall_from_clean_state`,
  `open_release_notes`, `none`).
- `support_bundle_ref` — reserved ref into the support-bundle family
  so a failed unattended result can be linked to the bundle the admin
  captured.
- `redaction_class` — same redaction vocabulary as the profile /
  state map so failure payloads do not leak secrets.

A result with a failed status and no `failure_reason_class` is
non-conforming. A result with a
`return_code_family == trust_policy_denial` and
`policy_injection_class == none` is non-conforming (the install is
not under policy control, so a policy denial is not admissible).

## Linkage into other control artifacts

- **Release evidence.** Every release-evidence packet MUST reference
  the install-topology matrix row for each shipped install-profile
  card and the state-root map row for each durable state root
  touched by the shipped build. The
  [`release_evidence`](../../artifacts/governance/ownership_matrix.yaml)
  lane is the owning lane for these artifacts.
- **Continuity.** Restore / migration surfaces read the state-root
  map to distinguish durable truth from disposable cache when
  deciding what survives a reinstall or rollback. The portable-
  profile schema
  ([`/schemas/profile/portable_profile.schema.json`](../../schemas/profile/portable_profile.schema.json))
  is the source of truth for per-class authority; this plan
  references it, it does not re-freeze it.
- **Fleet rollout.** The rollout-ring set composes with the decision
  workflow — promotion across rings is a decision-forum event
  recorded in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Desktop-affordance ownership.** File-association and deep-link /
  protocol-handler ownership rules in the state-root map are the
  source of truth for later desktop-affordance work; that work
  resolves against the rules here rather than inventing a parallel
  dialect.
- **Endpoint posture.** Later endpoint-posture work (host-trust,
  device-health attestation) composes over the install-profile card,
  not under it. The card's `running_build_identity_ref` is the
  stable handle into exact-build identity for endpoint-posture
  evidence.

## Change control

- Adding an `install_mode_class`, `channel_class`,
  `updater_owner_class`, `binary_root_class`,
  `durable_state_root_class`, `side_by_side_relation_class`,
  `rollout_ring_class`, `publication_posture_class`,
  `return_code_family_class`, `failure_reason_class`, or
  `remediation_pointer_class` value is additive-minor. Adding an
  entry requires: bumping the matrix `schema_version`, extending the
  matrix or seed with the new row, and updating this document in the
  same change.
- Repurposing an existing vocabulary value is breaking. Repurposing
  requires a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and the concurrence of the release council.
- Promoting an LTS cut, or changing the minimum promotion evidence
  on any ring, is release-council authority per
  [`/docs/governance/dri_map.md`](../governance/dri_map.md).

## Next-milestone expectations

- A concrete installer family (MSI / MSIX / PKG / DMG / tarball /
  AppImage / portable ZIP / offline bundle) is chosen per
  claimed platform and pinned into the matrix.
- Rollback artifact graphs for each ring are implemented; the
  matrix's `rollback_target_class` values become executable.
- The unattended-deployment result schema becomes a published
  boundary schema under
  [`/schemas/release/`](../../schemas/release/)
  alongside this seed.
- The state-root map rows resolve into concrete platform-specific
  paths (per host OS), replacing the placeholder path classes. The
  per-host-OS resolver lives with the install lane and is not part
  of this plan.
- Managed-package report surfaces (`available`) for
  `external_package_manager` and `managed_fleet` rows — reserved at
  this milestone — are wired to a fleet-console read path.

## Status

Plan is **seeded**. Every row is tagged `proposed`. Rows are not
deleted; they are superseded by a follow-on ADR / RFC recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
