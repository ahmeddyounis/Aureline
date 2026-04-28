# Deployment and Unsupported-Path Disclosure Matrix

This contract is the public disclosure layer for platform deployment
claims. It joins desktop profile rows, install-profile cards,
package-manager assumptions, fleet-tool assumptions, helper/agent
delivery, and explicit unsupported paths so release, docs, Help,
diagnostics, and support do not infer broad OS support from a generic
platform label.

If this document and a machine-readable companion disagree, the YAML is
the tooling source and this document must be corrected in the same
change.

Companion artifacts:

- [`/artifacts/platform/tested_package_managers.yaml`](../../artifacts/platform/tested_package_managers.yaml)
  freezes package-manager and fleet-tool lanes that may be named in
  install, diagnostics, docs, and support surfaces.
- [`/artifacts/platform/unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml)
  freezes unsupported, future-work, and best-effort paths that must be
  disclosed before support sees them in the field.
- [`/fixtures/platform/deployment_path_examples/`](../../fixtures/platform/deployment_path_examples/)
  contains worked disclosure examples for Linux distro/package-manager
  notes, macOS management-tool assumptions, Windows per-user/per-machine
  distinctions, and remote helper/agent deployment.

Related source contracts:

- [`/docs/platform/desktop_platform_conformance_matrix.md`](./desktop_platform_conformance_matrix.md)
  and
  [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  name the platform profiles and desktop primitives.
- [`/docs/platform/desktop_management_contract.md`](./desktop_management_contract.md),
  [`/artifacts/platform/deployment_patterns.yaml`](../../artifacts/platform/deployment_patterns.yaml),
  and
  [`/artifacts/platform/managed_controls_matrix.yaml`](../../artifacts/platform/managed_controls_matrix.yaml)
  define deployment-pattern and managed-control rows.
- [`/docs/release/install_profile_card_contract.md`](../release/install_profile_card_contract.md),
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml),
  and
  [`/artifacts/release/install_artifact_families.yaml`](../../artifacts/release/install_artifact_families.yaml)
  define install-profile cards, package families, channel identity, and
  state roots.

Normative source anchors include the PRD packaging and desktop-management
requirements, the architecture desktop-platform conformance appendix,
the install/portable/fleet rollout appendix, and the technical-design
deployment and package-manager sections under `.t2/docs/`.

## Claim Rule

A platform claim is the tuple of:

- a named platform profile;
- a deployment pattern;
- an install-profile card or helper/agent artifact family where one is
  applicable;
- a package-manager or fleet-tool row when acquisition, update,
  removal, mirror, or policy behavior is delegated; and
- any unsupported-path rows that narrow the claim.

"macOS", "Windows", "Linux", "package manager", or "managed install"
alone is not a claim. Surfaces must name the profile and the lane or
use a compact label that expands to those ids in diagnostics or support
export.

## Deployment Disclosure Matrix

| Platform or runtime scope | Individual install | Managed install | Portable install | Side-by-side stable/preview | Helper/agent deployment | Package-manager or fleet-tool assumption | Disclosure requirement |
|---|---|---|---|---|---|---|---|
| `macos_15_plus_universal` | Claimed for PKG and notarized DMG rows. | Claimed for PKG through MDM/Jamf/Munki-style distribution. | Claimed narrow for signed app ZIP with portable state. | Claimed through distinct bundle/update identities and separate state roots. | Not a desktop helper/agent host claim. | Homebrew/cask is best-effort and package-manager-owned; vendor-specific MDM consoles are not claimed. | Install guides and diagnostics must say PKG/DMG are claim-bearing, Homebrew is delegated, and macOS offline bundle is unsupported. |
| `windows_11_23h2_plus_x86_64` | Claimed for MSI/MSIX per-user and per-machine rows. | Claimed for MSI/MSIX through Intune/GPO/enterprise distribution. | Claimed narrow for signed portable ZIP. | Claimed through distinct product/package identities and state roots. | Not a desktop helper/agent host claim. | `winget` is future work; Intune/GPO are fleet automation assumptions, not product-owned consoles. | Docs and Help must distinguish per-user, per-machine, and managed authority. `winget` may not be presented as supported. |
| `linux_ubuntu_24_04_gnome_wayland_x86_64` | Claimed for tarball/AppImage direct rows. | Claimed narrow for managed package/tarball deployment on this named profile only. | Claimed narrow for tarball/AppImage portable state. | Claimed narrow where package or extract-root identity separates channels. | Not a desktop helper/agent host claim. | APT/DEB and RPM-style customer-mirror rows are claim-bearing only where paired with the Ubuntu Wayland profile and install cards; distro-native package managers otherwise stay delegated. | Linux docs must name Ubuntu 24.04 GNOME Wayland, GNOME Keyring/Secret Service, and display-stack assumptions before naming managed or mirror support. |
| `linux_ubuntu_24_04_gnome_x11_x86_64` | Claimed for direct local use. | Unsupported. | Best-effort only. | Direct local rows only; no managed widening. | Not a desktop helper/agent host claim. | APT/DEB package-manager acquisition is delegated/best-effort. | Support copy must say X11 is a direct local-use row and must not inherit Ubuntu Wayland managed/mirror claims. |
| `linux_fedora_current_gnome_wayland_x86_64` | Claimed for direct local use. | Unsupported. | Best-effort only. | Direct local rows only; no managed widening. | Not a desktop helper/agent host claim. | DNF/RPM acquisition is delegated/best-effort. | Support copy must name Fedora current GNOME Wayland and keep managed/offline claims unclaimed. |
| `linux_debian_stable_gnome_x11_x86_64` | Claimed for tarball direct local use. | Unsupported. | Unsupported. | Direct local rows only. | Not a desktop helper/agent host claim. | APT/DEB acquisition is delegated/best-effort. | Debian docs must present the row as the narrowest Linux claim and must not imply AppImage, managed, or mirror parity. |
| `remote_linux_target` and `remote_container_target` | Not a desktop install claim. | Claimed only through image baking, internal mirror, managed bootstrap, or private registry where the helper artifact row admits it. | Content-addressed runtime roots may coexist, but they are not desktop portable installs. | Multiple helper/agent versions may coexist only through capability negotiation and content-addressed runtime identity. | Claimed for signed remote-agent tarball, signed remote-helper tarball, or remote-agent image-layer bundle where the artifact family row admits it. | Internal mirror, golden image, managed bootstrap, private registry, or on-demand signed helper download; no desktop handler ownership. | Attach, diagnostics, and support surfaces must state helper/agent runtime identity, capability-negotiation state, mirror/source posture, and that desktop file/protocol handlers are not owned by helper artifacts. |

## Tested Package-Manager And Fleet-Tool Rows

The package-manager matrix separates three cases:

- claim-bearing lanes that must have install cards, artifact-family refs,
  diagnostics, and fixtures before release copy may name them;
- delegated or best-effort lanes that may be observed and explained but
  do not widen product update, uninstall, rollback, or handler claims;
- unsupported or future-work lanes that must appear in docs and support
  triage as non-claims.

The source of truth is
[`tested_package_managers.yaml`](../../artifacts/platform/tested_package_managers.yaml).
Every install guide, About/update surface, Project Doctor finding, and
support-bundle summary that mentions a package manager or fleet tool
must derive its wording from a row in that file.

## Unsupported Path Disclosure

Unsupported paths are not footnotes. They are records consumed by:

- install guides and compatibility pages;
- Help articles and support-center triage;
- Project Doctor findings and headless diagnostics;
- support bundles and field escalation summaries;
- release notes and known-limit pages when a claim is narrowed.

The source of truth is
[`unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml).
A row in that file must name the affected profile, why the path is
unsupported or delegated, which claims it does not widen, the disclosure
surfaces that must carry the limitation, and the evidence required
before the path can become claim-bearing.

## Surface Projection Rules

| Surface | Required projection |
|---|---|
| Install guide | Shows platform profile, artifact family, install card, updater owner, package-manager/fleet-tool row, portable/side-by-side posture, and unsupported adjacent lanes. |
| About / update / diagnostics | Shows install mode, channel, updater owner, package-manager or fleet-tool source, state-root class, and unsupported/delegated reason when update or removal is not product-owned. |
| Project Doctor | Emits a specific unsupported or delegated finding when the current package manager, desktop environment, display stack, secret store, fleet integration, or helper runtime is outside the claim. |
| Support bundle | Includes row refs for the platform profile, deployment pattern, package-manager/fleet-tool lane, unsupported-path rows, and helper/agent runtime identity where applicable. |
| Field triage | Uses unsupported-path ids instead of free-text guesses so escalation can distinguish unsupported, future work, delegated, and best-effort. |

## Change Control

Adding or widening a deployment claim requires the narrative doc, tested
package-manager matrix, unsupported-path matrix, relevant install or
deployment rows, and at least one worked fixture to update together.
Narrowing or removing a claim requires release-note and support-surface
projection in the same change. No package-manager, fleet-tool, desktop
environment, display stack, secret-store, or helper/agent path may be
implied by a broader platform label.
