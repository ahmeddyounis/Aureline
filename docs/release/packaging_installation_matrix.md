# Packaging and Installation Matrix

This contract freezes Aureline's desktop and helper packaging families
before installer code lands. It names the package formats, channel
identity, state-root ownership, update-marker ownership, and handler
coexistence rules that release, installer, updater, diagnostics,
support, mirror, and enterprise deployment lanes must read from one
place.

Companion artifacts:

- [`/artifacts/release/install_artifact_families.yaml`](../../artifacts/release/install_artifact_families.yaml)
  is the machine-readable artifact-family matrix. It names every
  desktop and helper package family, managed path, self-serve path,
  updater owner, portable posture, side-by-side posture, state-root
  refs, update-marker refs, handler ownership, mirror posture, and
  rollback posture.
- [`/artifacts/release/channel_identity_and_state_roots.yaml`](../../artifacts/release/channel_identity_and_state_roots.yaml)
  is the machine-readable channel identity and state-root separation
  contract. It binds Stable, Preview, long-support, portable, and
  correction rows to product identity placeholders, durable roots,
  update markers, recent-item namespaces, file associations, protocol
  handlers, and rollback / downgrade requirements.
- [`/fixtures/release/install_matrix_cases/`](../../fixtures/release/install_matrix_cases/)
  contains worked coexistence, portable, managed, Linux, macOS,
  remote-helper, and downgrade cases that release and support review
  can use as seed test data.
- [`/docs/release/install_topology_plan.md`](./install_topology_plan.md),
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml),
  and
  [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  remain the install-profile-card and state-root row sources. This
  matrix composes over them and does not replace them.
- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md)
  and
  [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
  remain the channel, branch, downgrade, and side-by-side admission
  sources.
- [`/docs/platform/deployment_and_unsupported_path_matrix.md`](../platform/deployment_and_unsupported_path_matrix.md),
  [`/artifacts/platform/tested_package_managers.yaml`](../../artifacts/platform/tested_package_managers.yaml),
  and
  [`/artifacts/platform/unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml)
  remain the disclosure source for package-manager, fleet-tool,
  helper/agent, and unsupported paths that package-family rows must not
  infer from artifact names alone.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` sections 9.11 and 9.12.
- `.t2/docs/Aureline_Technical_Design_Document.md` section 5.2.3 and
  Appendix M.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix BA.
- `.t2/docs/Aureline_Milestones_Document.md` sections 6.7, 6.8, and
  6.18.

## Scope

Frozen here:

- desktop package families for Windows MSI, Windows MSIX, Windows
  signed portable ZIP, macOS PKG, macOS notarized DMG, macOS signed app
  ZIP, Linux DEB, Linux RPM, Linux AppImage, and Linux tarball;
- helper package families for remote-agent tarballs, remote-helper
  tarballs, and remote-agent image-layer bundles;
- managed and self-serve acquisition paths for each family;
- updater-owner expectations for user-owned, admin-owned,
  external-package-manager-owned, managed-fleet, and no-updater rows;
- channel identity and coexistence rules for Stable, Preview,
  long-support, portable Stable, portable Preview, and correction
  releases;
- durable-state root, update-marker, recent-item, file-association,
  and protocol-handler ownership rules that prevent side-by-side rows
  from fighting silently;
- downgrade and rollback posture for user-authored state, recovery
  state, managed policy state, helper runtimes, and mirror metadata.

Out of scope:

- implementing installer pipelines, signing jobs, package-manager
  repositories, notarization, update services, or remote-agent image
  publishing;
- final OS-specific concrete paths and product identifiers. The
  machine-readable artifacts use stable placeholders until the platform
  path resolver and signing pipeline land;
- platform store policy, package-manager review, or customer fleet
  console adapters.

## Mechanical joins

The package-family rows intentionally do not duplicate install-card
truth. Tooling joins the files this way:

| Question | Source |
|---|---|
| Which package format is being shipped? | `install_artifact_families.yaml` `artifact_family_rows[].artifact_family_class` |
| Which install-card rows can consume it? | `install_artifact_families.yaml` `install_topology_card_refs` |
| Which channel identity owns the row? | `channel_identity_and_state_roots.yaml` `channel_identity_rows[]` |
| Which durable roots are touched? | `state_root_map.yaml` plus `channel_identity_and_state_roots.yaml` `durable_state_root_refs` |
| Which update marker and recent-item namespace are touched? | `channel_identity_and_state_roots.yaml` `update_marker_ref` and `recent_item_registration_ref` |
| Which file associations or deep links can be registered? | `channel_identity_and_state_roots.yaml` handler fields and `install_artifact_families.yaml` handler posture |
| Which rollback path protects user-authored state? | `channel_identity_and_state_roots.yaml` rollback requirements and `install_artifact_families.yaml` rollback posture |

Any release, installer, diagnostics, or support surface that cannot
resolve all of those joins is non-conforming.

## Artifact Family Matrix

| Artifact family | Managed path | Self-serve path | Portable / side-by-side posture | Updater owner | Handler posture |
|---|---|---|---|---|---|
| Windows MSI | Intune, Group Policy, enterprise distribution, or admin-installed per-machine package | Signed installer or package-manager entry that invokes the MSI | Stable and Preview use distinct product / upgrade-code namespaces and distinct state roots | user, admin, or managed fleet depending on install card | Candidate file handler only; shared protocol default is user/admin selected |
| Windows MSIX | Intune, enterprise distribution, or packaged per-machine deployment where platform support is claimed | Package-manager or signed app package | Package identity is channel-suffixed; no cross-channel mutable package storage | admin, external package manager, or managed fleet | Candidate handler only; no last-writer-wins default takeover |
| Windows signed portable ZIP | Internal file share or direct signed archive distribution; no managed mutation beyond file placement | Direct download, extract, run | Portable state is colocated; no machine-global mutation, file association, protocol handler, service, or shell hook | user or none | Not registered |
| macOS PKG | MDM, Jamf, Munki, or admin-installed package | Signed PKG download | Stable and Preview use distinct bundle/update identifiers where claimed | admin or managed fleet | Candidate handler only; shared scheme default remains user/admin selected |
| macOS notarized DMG | MDM/Jamf assisted distribution or user drag-install with notarization proof | Direct DMG download or cask-style acquisition | Stable and Preview use distinct bundle IDs and state roots | user, external package manager, or managed fleet | Candidate handler only |
| macOS signed app ZIP | Internal signed archive or developer/portable-like distribution | Direct signed ZIP extraction | Portable-like row must keep state discoverable and may not claim host integrations silently | user or none | Not registered unless a future explicit opt-in row is added |
| Linux DEB | Native repository, private mirror, config-management tooling | Package manager install from repository or mirrored `.deb` | Channel-specific package names or repos; side-by-side requires distinct package identity and XDG roots | external package manager, admin, or managed fleet | Desktop-file handler candidate only; default selection is explicit |
| Linux RPM | Native repository, private mirror, config-management tooling | Package manager install from repository or mirrored `.rpm` | Channel-specific package names or repos; side-by-side requires distinct package identity and XDG roots | external package manager, admin, or managed fleet | Desktop-file handler candidate only; default selection is explicit |
| Linux AppImage | Fleet-provided AppImage path or mirror distribution | Direct download | Side-by-side by filename/root; portable mode keeps markers and state under the AppImage sibling/root path when claimed | user or none | Not registered for portable rows; user/admin candidate only for installed rows |
| Linux tarball | Internal file share, config-management extraction, private mirror | Direct download and extraction | Side-by-side by extract root; portable tarball state root is explicit and separate from installed roots | user, admin, or none | Not registered unless a future explicit opt-in row is added |
| Remote-agent tarball | Golden VM/container image baking, internal mirror, or managed fleet bootstrap | On-demand signed download with capability negotiation | Multiple agent versions may coexist by content-addressed extraction root | admin, managed fleet, or user-on-demand | No desktop file association or protocol handler |
| Remote-helper tarball | Preinstalled helper in VM/container images or internal mirror | On-demand signed download scoped to an attach/session approval | Multiple helper versions may coexist when capability negotiation admits the skew | admin, managed fleet, or user-on-demand | No desktop file association or protocol handler |
| Remote-agent image-layer bundle | OCI-compatible image layer, golden image, private registry, or air-gap bundle | Internal registry pull or mirror import | Versioned image layers coexist by digest; rollback replaces the runtime layer after verification | managed fleet or admin | No desktop file association or protocol handler |

The machine-readable matrix carries the exact row ids, state-root refs,
update-marker refs, and install-topology card refs for these families.
This table is the human-facing summary only.

## Channel Identity Rules

1. Stable, Preview, and long-support desktop installs have distinct
   product identity placeholders per platform. Preview may import from
   Stable only through a side-by-side import sheet or an explicit
   user/admin selection; it may not read Stable's mutable state roots
   by default.
2. Portable Stable and portable Preview have portable channel identity.
   They do not register machine-global state, update markers, recent
   items, file associations, protocol handlers, services, or shell
   hooks unless a future row labels a specific opt-in integration.
3. Correction releases inherit the channel identity of the supported
   line they correct. They do not mint new durable roots or handler
   ownership unless the channel contract opens a new row.
4. Long-support installs are admin, managed-fleet, or package-manager
   owned. They can coexist with Stable or Preview only when their
   state roots, update markers, policy roots, and handlers remain
   channel-suffixed or admin-selected.
5. A shared `aureline://` scheme is only a resolver. It routes to the
   user/admin selected default channel. Channel-specific schemes such
   as `aureline-stable://` and `aureline-preview://` remain owned by
   their channel identity rows.

## Coexistence Rules

### Stable and Preview

Stable and Preview must have separate:

- configuration roots;
- recovery roots;
- derived-cache roots;
- credential-store service names or secret-handle namespaces;
- update markers;
- recent-item namespaces;
- file-association candidate registrations;
- channel-specific protocol-handler schemes.

The shared file-extension default and shared protocol default are
explicit user/admin choices. Installers and package managers must not
use last-writer-wins behavior to claim those defaults.

### Portable and Installed

Portable rows:

- keep durable state under the portable root;
- keep update markers and recent-item state under the portable root;
- suppress file associations, protocol handlers, services, shell hooks,
  and machine-global credential-store writes;
- may import installed state only through a reviewed import or restore
  flow with a rollback checkpoint;
- are removed by deleting the portable directory after diagnostics have
  identified the portable root.

### Managed and Portable

Managed rows own admin policy, fleet update markers, and managed
package reports. Portable rows may run beside managed installs only
when they do not read or write managed policy roots, fleet update
markers, machine-global recent items, file associations, or protocol
handlers. Import from a managed install requires an admin-allowed
domain list and an explicit restore provenance record.

### Package-Manager Rows

External package managers own package updates and package-root removal.
In-product update controls must disclose that the updater owner is the
external package manager and must not stage a competing update marker.
Aureline still owns per-channel user state roots and diagnostics
summaries for the running product.

### Remote Helpers

Remote-agent and helper packages never own desktop file associations,
desktop protocol handlers, or local recent-item registration. They own
runtime roots and update markers in their installation or image layer.
Attach succeeds only after capability negotiation confirms the client,
agent/helper version, policy layer, and runtime identity.

## Rollback and Downgrade Rules

1. Rollback operates on the coordinated artifact set: binary, sidecars,
   helper payloads, symbols, manifests, update metadata, mirror
   metadata, and diagnostics refs that share one exact-build identity.
2. User-authored durable state is never destroyed as rollback advice.
   Any downgrade that touches settings, profiles, snippets,
   keybindings, tasks, launch configs, recent work, or workspace
   metadata requires at least one of:
   - a backup snapshot;
   - a migration journal with inverse or restore steps;
   - a repair flow that preserves the original state and reports what
     could not be migrated.
3. Disposable derived caches may be rebuilt, but cache deletion is not
   a valid repair for user-authored state corruption.
4. If a newer state schema cannot be read by the target downgrade
   build, downgrade is blocked until the user/admin chooses a backup,
   migration, repair, or export path. Support copy may not prescribe
   "delete your config" as the primary recovery path.
5. Remote-agent rollback verifies the target artifact before replacing
   the runtime. Existing attach authorization is preserved only where
   the capability envelope and policy epoch still match; otherwise the
   user sees a reattach or review-only downgrade.
6. Mirror and air-gap rollback must remain possible without vendor
   network reachability. The bundle must carry enough signed metadata
   to verify the rollback target, revocation state, and policy floor.

## Enterprise and Mirror Expectations

Enterprise and mirror rows must expose, in diagnostics and support
exports:

- package family and package-format class;
- install mode and channel;
- updater owner;
- binary root class;
- durable-state root refs;
- update-marker owner;
- policy root owner where applicable;
- managed package report status;
- mirror/offline metadata status;
- rollback target and evidence refs;
- uninstall or disable owner.

The managed-package report shape used to satisfy the inventory and
diagnostic requirements above is seeded in
[`/artifacts/release/managed_package_report_seed.yaml`](../../artifacts/release/managed_package_report_seed.yaml).

Silent deployment summaries must use structured result classes from
[`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml)
and the boundary schema in
[`/schemas/release/silent_deployment_result.schema.json`](../../schemas/release/silent_deployment_result.schema.json)
instead of free-text-only failures.

## Change Control

Adding a package family is additive only when this document, both
machine-readable companion artifacts, and at least one fixture case are
updated in the same change. Repurposing an existing family, changing a
handler ownership rule, or allowing a shared durable state root is a
breaking release-governance change and requires a decision row before
installer work may depend on it.
