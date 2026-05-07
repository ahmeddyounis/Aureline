# Desktop Management Contract

This contract freezes the IT-facing scriptability surface for desktop
deployment before installer code lands. It gives support, release
engineering, enterprise administrators, and package authors one place
to answer:

- which desktop deployment patterns are claim-bearing on each named
  platform profile;
- whether silent install, silent uninstall, and channel pinning are
  supported, unsupported, or future work;
- which managed controls exist for update policy, extension mirrors,
  proxy posture, policy bundle location, and uninstall behavior; and
- which rows are explicitly outside the current claim.

The machine-readable companions are:

- [`/artifacts/platform/deployment_patterns.yaml`](../../artifacts/platform/deployment_patterns.yaml)
  for platform/profile deployment-pattern rows;
- [`/artifacts/platform/managed_controls_matrix.yaml`](../../artifacts/platform/managed_controls_matrix.yaml)
  for managed-control definitions and support rollups;
- [`/docs/platform/deployment_and_unsupported_path_matrix.md`](./deployment_and_unsupported_path_matrix.md),
  [`/artifacts/platform/tested_package_managers.yaml`](../../artifacts/platform/tested_package_managers.yaml),
  and
  [`/artifacts/platform/unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml)
  for tested package-manager, fleet-tool, helper/agent, and unsupported
  path disclosure rows; and
- [`/fixtures/platform/scriptable_install_cases/`](../../fixtures/platform/scriptable_install_cases/)
  for worked cases that join controls to installer families, release
  channels, state roots, and policy artifacts.

If this document and a machine-readable companion disagree, the YAML is
the tooling source and this document must be corrected in the same
change.

## Source Contracts

This contract composes over existing release, platform, policy, and
extension records. It does not mint a second vocabulary for install
truth.

| Question | Source |
|---|---|
| Which platform profiles are claimed? | [`claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml) |
| Which install modes, updater owners, policy injection classes, and silent-install classes exist? | [`install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml) |
| Which installer artifact families are eligible? | [`install_artifact_families.yaml`](../../artifacts/release/install_artifact_families.yaml) |
| Which channel identity and state roots are touched? | [`channel_identity_and_state_roots.yaml`](../../artifacts/release/channel_identity_and_state_roots.yaml) and [`state_root_map.yaml`](../../artifacts/release/state_root_map.yaml) |
| Which unattended result classes and exit-code families are allowed? | [`silent_deployment_contract.md`](../release/silent_deployment_contract.md), [`silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml), and [`silent_deployment_result.schema.json`](../../schemas/release/silent_deployment_result.schema.json) |
| Which managed-package inventory report shape is emitted? | [`managed_package_report_seed.yaml`](../../artifacts/release/managed_package_report_seed.yaml) |
| Which package-manager, fleet-tool, helper/agent, and unsupported paths may be named? | [`deployment_and_unsupported_path_matrix.md`](./deployment_and_unsupported_path_matrix.md), [`tested_package_managers.yaml`](../../artifacts/platform/tested_package_managers.yaml), and [`unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml) |
| Which policy artifact carries admin controls? | [`admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md) |
| Which extension mirror vocabulary is reused? | [`channel_promotion_rows.yaml`](../../artifacts/extensions/channel_promotion_rows.yaml) |
| Which proxy and transport vocabulary is reused? | [`admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md) and the transport-governance source sections in `.t2/docs/` |

Normative source anchors include the desktop-platform, install-topology,
transport-governance, policy-bundle, and fleet-deployment sections in
`.t2/docs/`. Those sources override this summary if they conflict.

## Status Vocabulary

The machine rows use the following IT-facing status buckets.

| Status | Meaning |
|---|---|
| `supported` | Claim-bearing and usable by the named pattern on the named platform profile. |
| `supported_narrow` | Claim-bearing only with the stated narrower artifact family, platform profile, or install mode. |
| `best_effort` | May work, but does not widen the stable support contract. |
| `unsupported` | Explicitly outside the current claim. Support and docs must not imply it works. |
| `future_work` | Reserved for a later dedicated row. No current support claim exists. |
| `delegated` | A package manager, OS facility, or fleet system owns the operation; Aureline only reports and explains it. |

Silent install, silent uninstall, and channel pinning additionally carry
a `claim` field of `supported`, `unsupported`, or `future_work` so every
claimed platform has a concrete answer for those procurement questions.

## Deployment Pattern Matrix

The rows below summarize
[`deployment_patterns.yaml`](../../artifacts/platform/deployment_patterns.yaml).
They are profile-specific. "Linux" without the named profile row is not
a supported platform claim. Package-manager and fleet-tool names in this
table are disclosure summaries; their source rows live in
[`tested_package_managers.yaml`](../../artifacts/platform/tested_package_managers.yaml),
and unsupported adjacent paths live in
[`unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml).

| Platform profile | Individual install | Managed install | Portable install | Offline bundle / customer mirror | External package manager |
|---|---|---|---|---|---|
| `macos_15_plus_universal` | `supported`: notarized DMG and PKG paths, with per-user and per-machine posture. DMG scriptability is narrower than PKG. | `supported`: PKG through MDM/Jamf/Munki-style distribution, admin-owned policy injection, and managed package report slot. | `supported_narrow`: signed app ZIP with colocated or sibling portable state and no host integrations. | `unsupported`: macOS offline bundle is not in the claimed row set. | `best_effort`: Homebrew/cask-style acquisition is package-manager-owned and does not widen in-product update or handler claims. |
| `windows_11_23h2_plus_x86_64` | `supported`: MSI/MSIX direct install, per-user and per-machine posture. | `supported`: MSI/MSIX through Intune/GPO/enterprise distribution with admin policy and managed package report slot. | `supported_narrow`: signed portable ZIP, no registry/service/default-handler ownership. | `supported`: signed offline bundle with mirror metadata and policy/bootstrap artifacts. | `future_work`: `winget` is intentionally unclaimed until a dedicated install-profile card exists. |
| `linux_ubuntu_24_04_gnome_wayland_x86_64` | `supported`: tarball/AppImage direct row on the named GNOME Wayland profile. | `supported_narrow`: managed Linux widening is claim-bearing only on this named Ubuntu Wayland profile. | `supported_narrow`: portable tarball/AppImage state is isolated from installed roots. | `supported_narrow`: offline bundle and customer mirror are tied to this profile only. | `best_effort`: distro package managers do not widen the claim-bearing direct row. |
| `linux_ubuntu_24_04_gnome_x11_x86_64` | `supported`: direct local-use row only. | `unsupported`: managed Linux widening is not claimed on the X11 row. | `best_effort`: may work, but does not widen the supported profile. | `unsupported`: offline and mirror widening are outside this row. | `best_effort`: package-manager lanes are reporting-only. |
| `linux_fedora_current_gnome_wayland_x86_64` | `supported`: direct local-use row only. | `unsupported`: managed Linux widening is not claimed on Fedora current. | `best_effort`: may work, but does not widen the supported profile. | `unsupported`: offline and mirror widening are outside this row. | `best_effort`: package-manager lanes are reporting-only. |
| `linux_debian_stable_gnome_x11_x86_64` | `supported`: tarball direct row only. | `unsupported`: managed Linux widening is not claimed on Debian stable. | `unsupported`: portable Linux widening is not claimed on Debian stable. | `unsupported`: offline and mirror widening are outside this row. | `best_effort`: package-manager lanes are reporting-only. |

## Required Scriptable Controls

Every supported or supported-narrow managed deployment must expose the
following controls through a documented installer/package/fleet surface
and must emit a machine-readable unattended result record when the
operation runs unattended.

| Control | Contract |
|---|---|
| Silent install | Resolves to `result_kind: install` in `silent_deployment_seed.yaml`. Supported managed rows must document the artifact family, channel, policy bundle source, update owner, and return-code family. |
| Silent uninstall | Resolves to `result_kind: uninstall`. Uninstall removes declared binary/package roots and install markers but preserves user-authored durable state unless a separate explicit clear-data action is selected. |
| Channel pinning | Resolves to `result_kind: pin` for unattended pin operations. Pins bind to `channel_identity.*` rows and may not silently switch Stable, Preview, portable, or long-support identities. |
| Update deferral | Admin or fleet-owned rows may defer within the active channel and rollout ring. Deferral is an admin policy or package-manager state, not an in-product hidden override. |
| Update disablement | Disabling product-managed updates is allowed only through admin policy, emergency disable, package-manager delegation, or portable/no-updater posture. The source must be explainable. |
| Extension mirror location | Uses the registry source and mirror-continuity vocabulary in `channel_promotion_rows.yaml`. Mirrors can narrow, pin, or deny; they cannot repackage artifacts or widen trust. |
| Proxy settings | Uses the shared transport-governance vocabulary. System proxy, PAC, manual override, environment, and policy source remain inspectable; no surface may silently bypass mirror-only or deny-all policy. |
| Policy bundle location | Points at the admin policy artifact family described by `$AURELINE_POLICY/aureline.policy.json` and signed bundle-cache entries. Raw rule bodies, secrets, tokens, and private hostnames are not emitted in support packets. |
| Documented uninstall behavior | Every row states what is removed, what is preserved, and who owns removal. Portable removal is documented as removing the portable root, not as a platform package uninstall. |

## Per-Platform Scriptability Summary

| Platform profile | Silent install | Silent uninstall | Channel pinning |
|---|---|---|---|
| `macos_15_plus_universal` | `supported` on PKG managed/per-machine rows; `supported` with `partial` class on direct DMG rows; `unsupported` for portable archive extraction as platform install. | `supported` on PKG managed/per-machine rows; direct DMG removal is documented but narrower; portable removal is root deletion. | `supported` through channel identity rows; admin-enforced pinning requires managed or admin-owned install. |
| `windows_11_23h2_plus_x86_64` | `supported` on MSI/MSIX and offline-bundle rows; portable ZIP extraction is not a platform silent install. | `supported` on MSI/MSIX and offline-bundle rows; portable removal is root deletion. | `supported` on individual, managed, and offline rows; portable channel is fixed by archive identity. |
| `linux_ubuntu_24_04_gnome_wayland_x86_64` | `supported` on direct tarball/AppImage, managed, offline, and customer-mirror rows. | `supported` on direct, managed, offline, and customer-mirror rows; portable removal is root deletion. | `supported` on direct, managed, offline, and customer-mirror rows; package-manager pins are delegated. |
| `linux_ubuntu_24_04_gnome_x11_x86_64` | `supported` for direct local install only; managed/offline are `unsupported`; package-manager control is `delegated`. | `supported` for direct local install only; managed/offline are `unsupported`. | `supported` for direct local install only; managed/offline pins are `unsupported`. |
| `linux_fedora_current_gnome_wayland_x86_64` | `supported` for direct local install only; managed/offline are `unsupported`; package-manager control is `delegated`. | `supported` for direct local install only; managed/offline are `unsupported`. | `supported` for direct local install only; managed/offline pins are `unsupported`. |
| `linux_debian_stable_gnome_x11_x86_64` | `supported` for direct tarball local install only; managed, portable, and offline are `unsupported`; package-manager control is `delegated`. | `supported` for direct tarball local install only; managed, portable, and offline are `unsupported`. | `supported` for direct local install only; managed/offline pins are `unsupported`. |

## Uninstall Behavior

Uninstall is scoped to the install mode.

- Per-user and per-machine installed rows remove the binary root,
  update marker, package registration, and declared handler candidates
  owned by that install row. They do not delete workspace repositories,
  profiles, settings, keybindings, snippets, recovery journals, support
  bundles, or local history by default.
- Managed rows remove fleet-owned package registrations and managed
  install markers only under admin or fleet authority. Admin policy roots
  are removed only by the admin-owned policy/package path.
- External package-manager rows delegate package removal to the package
  manager. Aureline still reports which user state roots exist and must
  not claim package-manager removal deleted them.
- Portable rows are removed by deleting the portable directory after
  diagnostics identify the portable root. Portable removal must not
  advise deleting installed-channel state roots.
- Any clear-data operation is separate, previewable, class-selective,
  and governed by the state/root and support-bundle contracts.

## Explicitly Out Of Claim

The following are visible gaps, not hidden support assumptions:

- Windows ARM64 and Linux ARM64 desktop packaging.
- Windows `winget` deployment, update, uninstall, or pinning as a
  claim-bearing path.
- macOS offline-bundle deployment and customer mirror widening.
- macOS Homebrew/cask update ownership as an in-product update claim.
- Linux managed, offline, or customer-mirror widening outside the named
  Ubuntu 24.04 GNOME Wayland x86_64 profile.
- Linux KDE, KWallet-backed secret storage, non-GNOME badge behavior,
  non-GNOME deep-link semantics, and unnamed desktop environments.
- Customer fleet-console adapters, detection-rule templates, smart-group
  logic, SCCM/Workspace ONE/Kandji/Chef/Puppet/Ansible modules, and
  vendor-specific MDM UI flows. The current claim is the package,
  control, result, policy, and inventory contract those systems can
  automate against.

The machine-readable disclosure source for these gaps is
[`unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml).
Support, Help, diagnostics, and field triage must cite those row ids
instead of inferring support from the broader deployment-pattern table.

## Change Control

Adding a deployment pattern, managed control, platform support row, or
fixture is additive only when the narrative doc, the relevant YAML row,
and at least one worked fixture update together. Repurposing an existing
support status, changing uninstall preservation rules, or widening a
package-manager path into a managed-control claim is a breaking release
governance change.
