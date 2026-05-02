# Install / update / About truth packet

This packet freezes one inspectable record family that the launcher,
About panel, update center, installer summary, silent-deployment
summary, diagnostics center, and support bundle all read when they
describe an Aureline install. The intent is to remove installer
folklore and About-page copy as competing sources of truth: every claim
about install mode, channel, updater owner, binary root, durable-state
roots, rollback target, file-association owner, and diagnostics path
projects from one record per claimed (install_mode_class,
channel_class, platform_class, architecture_class) row in
[`install_topology_matrix.yaml`](./install_topology_matrix.yaml) plus
its companion channel-identity row in
[`channel_identity_and_state_roots.yaml`](./channel_identity_and_state_roots.yaml).

If this packet, the
[`channel_ownership_audit.yaml`](./channel_ownership_audit.yaml)
ledger, the
[`portable_mode_limitations.yaml`](./portable_mode_limitations.yaml)
ledger, the
[`install_truth_cases/`](../../fixtures/release/install_truth_cases/)
fixture corpus, or any About / update / installer / diagnostics surface
disagree with
[`docs/release/install_topology_plan.md`](../../docs/release/install_topology_plan.md),
[`docs/release/packaging_installation_matrix.md`](../../docs/release/packaging_installation_matrix.md),
[`docs/release/install_profile_card_contract.md`](../../docs/release/install_profile_card_contract.md),
[`artifacts/release/install_topology_matrix.yaml`](./install_topology_matrix.yaml),
[`artifacts/release/state_root_map.yaml`](./state_root_map.yaml),
[`artifacts/release/channel_identity_and_state_roots.yaml`](./channel_identity_and_state_roots.yaml),
[`artifacts/release/install_artifact_families.yaml`](./install_artifact_families.yaml),
[`artifacts/release/silent_deployment_seed.yaml`](./silent_deployment_seed.yaml),
or [`schemas/release/install_row.schema.json`](../../schemas/release/install_row.schema.json),
those governing sources win and this packet plus its companion ledgers
and fixtures update in the same change.

## Companion artifacts

- [`channel_ownership_audit.yaml`](./channel_ownership_audit.yaml)
  — per-channel and side-by-side ownership audit covering Stable,
  Preview, Beta, long-support, portable, managed, and offline-bundle
  pairings; pins which channel owns update markers, recent-item
  registrations, file-association candidates, protocol-handler schemes,
  keychain service names, and diagnostics paths, and pins the
  shared-state collision risks each pair MUST suppress.
- [`portable_mode_limitations.yaml`](./portable_mode_limitations.yaml)
  — limitation ledger pinning what a portable install MUST NOT do on
  the host: file-association registration, protocol-handler claim,
  service install, shell hook, machine-global credential-store write,
  machine-global recent-item write, machine-global update marker,
  system-wide PATH mutation, autostart hook, and kernel-extension or
  privileged helper installation.
- [`/fixtures/release/install_truth_cases/`](../../fixtures/release/install_truth_cases/)
  — worked install_truth_record fixtures that bind one install-profile
  card and one channel-identity row to the About / update / installer /
  diagnostics surface projections frozen here.

## Normative source anchors

- `.t2/docs/Aureline_PRD.md` §§5.20, 9.11, 9.12, 10.22, 10.23.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §25.9 and
  Appendix BA.
- `.t2/docs/Aureline_Technical_Design_Document.md` §5.2.3 and
  Appendix M.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.12 (About, update, and
  diagnostics surfaces) and Appendix O.
- `.t2/docs/Aureline_Milestones_Document.md` §6.18 (install and update
  behaviour as product truth).

## Scope

This packet freezes:

| Concern | Frozen here |
|---|---|
| Record family | One `install_truth_record` projection per claimed install-profile card. |
| Required field set | The set of fields every `install_truth_record` MUST carry, with each field bound to a closed vocabulary that already lives in the install topology matrix, state-root map, or channel-identity contract. |
| Surface projections | Required field subsets for the launcher, About, update center, installer summary, silent-deployment summary, diagnostics center, and support bundle, with one wording-source ref per surface so wording cannot drift between surfaces. |
| Audit gates | The set of gates a release / support / enterprise reviewer applies before any About, update, installer, diagnostics, or support text claims to describe "the install". |

Out of scope:

- Installer, updater, fleet console, or About-panel implementation.
- Channel-promotion tooling.
- Concrete per-host-OS paths (those remain placeholders in
  `state_root_map.yaml` until the install lane lands).
- Re-minting any vocabulary that already lives in the upstream
  artifacts above; this packet only re-projects existing rows.

## The `install_truth_record` projection

Every claimed install-profile card in `install_topology_matrix.yaml`
projects into exactly one `install_truth_record`. The projection is
mechanical: the record carries no field that the upstream card or its
companion channel-identity, state-root, update-marker, file-association,
or protocol-handler row does not already pin. Surfaces may select a
subset of fields to render but MUST NOT rename or invent any of them.

### Required fields

| Field | Closed vocabulary or ref source |
|---|---|
| `install_truth_id` | Stable id `install_truth:<install_profile_card_id>`. |
| `install_profile_card_ref` | Card id from `artifacts/release/install_topology_matrix.yaml` (e.g. `card.windows.x86_64.per_user_installed.stable`). |
| `channel_identity_ref` | Channel-identity row id from `artifacts/release/channel_identity_and_state_roots.yaml`. |
| `install_mode_class` | `install_mode_class_vocabulary` in the install topology matrix. |
| `channel_class` | `channel_class_vocabulary` in the install topology matrix. |
| `platform_class` | `platform_class_vocabulary`. |
| `architecture_class` | `architecture_class_vocabulary`. |
| `updater_owner_class` | `updater_owner_class_vocabulary`. |
| `binary_root_class` | `binary_root_class_vocabulary`. |
| `binary_root_ref` | Stable ref to the binary root path placeholder owned by the upstream card. |
| `durable_state_root_refs` | One or more state-root ids from `artifacts/release/state_root_map.yaml`. |
| `state_root_locality_class` per durable state root | `per_user_only`, `per_machine_shared`, `portable_colocated`, `offline_bundle_local`, `platform_shared_read_only`, or `remote_runtime_separate`. |
| `state_root_ownership_class` per durable state root | `single_channel_owned`, `shared_across_channels_with_named_reason`, or `inherited_from_target_channel`. |
| `side_by_side_relation_class` | `side_by_side_relation_class_vocabulary`. |
| `rollback_target_class` | `rollback_target_class_vocabulary`. |
| `rollback_target_ref` | Nullable stable ref into the upstream rollback evidence row. |
| `file_association_owner` | Object: `registration_class` (`file_association_registration_class_vocabulary`), `default_handler_selection_rule` (`user_or_admin_selectable_never_last_writer_wins`, `admin_only`, `not_applicable`), `owner_channel_class`, `collision_disclosure`. |
| `protocol_handler_owner` | Object: `ownership_class` (`protocol_handler_ownership_class_vocabulary`), `scheme_placeholder`, `shared_scheme_resolution_rule` (`user_or_admin_selected_default`, `not_applicable`). |
| `diagnostics_path_refs` | One or more `diagnostics_export_class_vocabulary` values resolvable to a `diagnostics_export_action` on the upstream card. |
| `support_bundle_inclusion_class` | `installed_diagnostics_included`, `portable_root_only_diagnostics_included`, `managed_admin_console_only`, `offline_bundle_mirror_metadata_included`, `not_applicable`. |
| `update_marker_ref` | Update-marker row id from `artifacts/release/channel_identity_and_state_roots.yaml`. |
| `update_marker_ownership_class` | `update_marker_ownership_class_vocabulary` in `artifacts/release/state_root_map.yaml`. |
| `running_build_identity_ref` | Slot reserved (null at this milestone) into `schemas/build/exact_build_identity.schema.json`. |
| `portable_mode_active` | Boolean derived from `install_mode_class == portable`. |
| `portable_mode_limitation_refs` | Limitation row ids from `portable_mode_limitations.yaml`; non-empty only when `portable_mode_active` is true. |
| `channel_ownership_audit_refs` | Audit-row ids from `channel_ownership_audit.yaml` covering every other-channel pairing the host can plausibly produce. |
| `surface_projections` | One row per `install_surface` that this record exposes; see below. |
| `status` | `proposed` at seed time. |

### Required surface projections

The packet pins seven required surfaces. A record MAY add more, but it
MUST NOT drop one of these without naming the suppressing rule in the
upstream card.

| Surface | Required field set | Wording-source rule |
|---|---|---|
| `launcher` | `install_mode_class`, `channel_class`, `binary_root_class`, `side_by_side_relation_class`. | Channel label projects from the channel-identity row, never from a free-text "current channel" string. Stable, Preview, Beta, and any long-support line MUST stay visibly distinct. |
| `about` | `install_mode_class`, `channel_class`, `updater_owner_class`, `binary_root_class`, `durable_state_root_refs`, `side_by_side_relation_class`, `rollback_target_class`, `file_association_owner.registration_class`, `protocol_handler_owner.ownership_class`, `diagnostics_path_refs`, `running_build_identity_ref`, `portable_mode_active`. | About reads from this record verbatim. About MUST NOT infer install mode, channel, or updater owner from a version string and MUST NOT collapse multiple channels into one row. |
| `update_center` | `channel_class`, `updater_owner_class`, `rollback_target_class`, `update_marker_ref`, `update_marker_ownership_class`, `diagnostics_path_refs`. | Update center MUST disclose the updater owner before any "check for updates" or "install update" affordance. Rows whose `updater_owner_class` is `external_package_manager` or `managed_fleet` MUST NOT offer an in-product binary-replacement update path. |
| `installer_summary` | `install_mode_class`, `channel_class`, `binary_root_class`, `durable_state_root_refs`, `file_association_owner`, `protocol_handler_owner`, `portable_mode_active`, `portable_mode_limitation_refs`. | Installer summary MUST disclose state roots and handler ownership during install, not after. Default file-association and shared protocol scheme MUST NOT be silently claimed. |
| `silent_deployment_summary` | `install_mode_class`, `channel_class`, `updater_owner_class`, `binary_root_class`, `durable_state_root_refs`, `rollback_target_class`, `diagnostics_path_refs`, `update_marker_ref`. | Silent install / update / rollback / uninstall MUST emit a human-readable summary projecting these fields. The summary cites the silent-deployment return-code family from `silent_deployment_seed.yaml`; the channel travels on the install-truth record, not on the return code. |
| `diagnostics_center` | `install_mode_class`, `channel_class`, `binary_root_class`, `durable_state_root_refs`, `update_marker_ref`, `diagnostics_path_refs`, `support_bundle_inclusion_class`. | Diagnostics MUST identify the running install by `install_profile_card_ref` and MUST NOT recommend deleting another channel's state root to repair this one. |
| `support_bundle` | `install_mode_class`, `channel_class`, `updater_owner_class`, `binary_root_class`, `durable_state_root_refs`, `rollback_target_class`, `diagnostics_path_refs`, `support_bundle_inclusion_class`, `running_build_identity_ref`, `channel_ownership_audit_refs`. | Support exports project from this record verbatim. Stable and Preview support bundles MUST NOT be merged. Portable support bundles MUST scope to the portable root and MUST NOT recommend touching installed-channel state. |

## Audit gates

A reviewer MUST be able to determine, from one `install_truth_record`,
which build and channel own system integration on the host and which
state roots are isolated or shared. The packet pins the following audit
gates; a record that fails any gate is non-conforming.

### Gate `record_resolves_to_one_install_profile_card`

Every record carries exactly one `install_profile_card_ref` resolving
into `artifacts/release/install_topology_matrix.yaml`. Records that
elide the ref or claim multiple cards are non-conforming.

### Gate `channel_identity_pinned`

Every record carries exactly one `channel_identity_ref` resolving into
`artifacts/release/channel_identity_and_state_roots.yaml`. The
channel-identity row's `channel_class` MUST equal the record's
`channel_class`.

### Gate `state_roots_resolve_into_state_root_map`

Every `durable_state_root_ref` resolves into a row in
`artifacts/release/state_root_map.yaml`. Each ref carries exactly one
`state_root_locality_class` and exactly one `state_root_ownership_class`
matching the upstream row's `owning_channels` and `shared_across_channels`
fields.

### Gate `no_silent_handler_capture`

`file_association_owner.default_handler_selection_rule` is
`user_or_admin_selectable_never_last_writer_wins`,
`admin_only`, or `not_applicable`. The string `last_writer_wins` MUST
NOT appear anywhere in the record. `protocol_handler_owner.ownership_class`
of `shared_scheme_with_user_or_admin_default` requires
`shared_scheme_resolution_rule == user_or_admin_selected_default`.

### Gate `updater_owner_governs_update_surface`

A record whose `updater_owner_class` is `external_package_manager` or
`managed_fleet` MUST NOT carry an `update_center` projection that
exposes an in-product binary-replacement update affordance. The record
MUST instead surface the package-manager or managed-fleet update path
disclosure.

### Gate `rollback_target_matches_ring`

The `rollback_target_class` MUST be admissible for the upstream card's
`rollout_ring_class` per the `ring_promotion_evidence` table in
`install_topology_matrix.yaml`. A record whose `rollback_target_class`
is `unsupported` MUST NOT advertise a rollback affordance to the user.

### Gate `diagnostics_minimum`

Every record's `diagnostics_path_refs` set MUST include at least
`install_topology_record_available` and `exact_build_manifest_available`.
Records that cannot are non-conforming.

### Gate `portable_record_carries_limitation_refs`

A record whose `portable_mode_active` is true MUST cite at least the
limitation rows for file associations, protocol handlers, services,
shell hooks, machine-global credential-store writes, machine-global
recent items, machine-global update markers, system-wide PATH
mutation, autostart hooks, and kernel-extension or privileged-helper
installation from `portable_mode_limitations.yaml`. The portable record
MUST NOT carry an `installer_summary` or `update_center` projection that
implies host-wide integration.

### Gate `side_by_side_record_carries_audit_refs`

A record whose `side_by_side_relation_class` is any value other than
`none` MUST cite the matching audit row from
`channel_ownership_audit.yaml`. Records that cannot are non-conforming.

### Gate `running_build_identity_slot_reserved`

Every record carries the `running_build_identity_ref` slot. The slot
is reserved (null) at this milestone; records that elide the slot
entirely are non-conforming. About, update, installer, diagnostics,
and support surfaces project the slot's eventual value rather than
re-inventing version semantics.

### Gate `no_milestone_or_planning_id_in_user_visible_text`

No `install_truth_record` field, About panel string, update-center
string, installer summary, diagnostics export, or support-bundle
manifest renders a milestone slug or planning identifier. Identifiers
in the record family describe what is installed, not which planning
row produced the truth.

## How surfaces derive their truth

Every surface listed above projects from the `install_truth_record`.
The packet pins six honesty rules across surfaces:

1. **Channel disclosure is mechanical.** The `channel_class` and the
   channel-identity row provide the channel label. Surfaces MUST NOT
   collapse Stable, Preview, Beta, or any long-support line into a
   single "current channel" string.
2. **Updater owner is disclosed before the action.** The update center
   and any in-product update affordance MUST disclose the
   `updater_owner_class` first. Records owned by an external package
   manager or a managed fleet do not offer an in-product binary
   replacement.
3. **State-root claims cite the state-root map.** About, diagnostics,
   and support surfaces present durable state roots by their
   `durable_state_root_class` and `path_class_placeholder` from
   `state_root_map.yaml`, never by inferred OS paths.
4. **Handler ownership is honest.** Default file-association and
   protocol-handler selection MUST come from the user or admin. The
   word "default" in About or installer copy MUST cite the
   `file_association_owner.default_handler_selection_rule` or
   `protocol_handler_owner.shared_scheme_resolution_rule`.
5. **Diagnostics paths are typed.** "Open diagnostics", "export support
   bundle", "copy install summary", "open state-root audit", and "open
   rollback evidence" each cite a `diagnostics_export_class` from the
   upstream card. No surface invents a new export class.
6. **Portable does not pretend to be installed.** Portable records'
   surface projections cite the portable colocated root and the
   limitation ledger; they do not advertise host-wide integration,
   silent install, or updater ownership states the host did not
   actually experience.

## Acceptance gates

This packet, the audit ledger, the limitation ledger, and the worked
fixture corpus together satisfy the spec acceptance criteria when:

- A reviewer can determine, from a single `install_truth_record`, which
  build and channel own host integration and which state roots are
  isolated or shared.
- Every claimed About, update, installer, diagnostics, and support
  surface projects from the same `install_truth_record` family rather
  than from per-surface copy.
- Portable-mode limitations are explicit enough in
  `portable_mode_limitations.yaml` to prevent any release note, About
  panel, or support reply from claiming host-wide integration the
  portable install does not deliver.
- Every worked fixture in
  [`/fixtures/release/install_truth_cases/`](../../fixtures/release/install_truth_cases/)
  resolves verbatim to upstream rows in `install_topology_matrix.yaml`,
  `channel_identity_and_state_roots.yaml`, `state_root_map.yaml`, and
  `install_artifact_families.yaml`, satisfies all audit gates listed
  above, and carries `status: proposed`.

## Change control

Adding a vocabulary value or a surface projection is additive-minor and
requires updating this packet, the audit ledger, the limitation ledger,
the install topology matrix, the state-root map, the channel-identity
contract, and the install-row schema in the same change. Repurposing an
existing value is breaking and opens a row in
`artifacts/governance/decision_index.yaml`. The fixture corpus is
exhaustive for the cells declared admissible by the install topology
matrix; cells not yet seeded there are reserved here as well.
