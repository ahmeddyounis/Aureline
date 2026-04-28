# Install-profile card, side-by-side import sheet, and rollout-row contract

This contract publishes the row model that install, update, rollout,
portable-mode, side-by-side, About, diagnostics, support, and installer-
adjacent surfaces use when they describe an Aureline install. The goal is
one inspectable install truth, not one wording set per surface.

Companion artifacts:

- [`/schemas/release/install_row.schema.json`](../../schemas/release/install_row.schema.json)
  — boundary schema for `install_profile_card_record`,
  `side_by_side_import_sheet_record`, and `rollout_ring_row_record`.
- [`/fixtures/release/install_rows/`](../../fixtures/release/install_rows)
  — worked portable-mode and side-by-side import fixtures.
- [`/fixtures/release/rollout_ring_rows/`](../../fixtures/release/rollout_ring_rows)
  — worked rollout-row fixtures for canary, pilot, broad, stable,
  preview, beta, and long-support lanes.
- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  and
  [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  — source rows for install modes, channel separation, durable roots,
  update markers, file associations, and protocol handlers.
- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md)
  and
  [`/docs/release/ring_progression_policy.md`](./ring_progression_policy.md)
  — channel, rollback, and evidence-widening context.
- [`/docs/platform/deployment_and_unsupported_path_matrix.md`](../platform/deployment_and_unsupported_path_matrix.md),
  [`/artifacts/platform/tested_package_managers.yaml`](../../artifacts/platform/tested_package_managers.yaml),
  and
  [`/artifacts/platform/unsupported_paths.yaml`](../../artifacts/platform/unsupported_paths.yaml)
  — platform disclosure rows for package-manager, fleet-tool,
  helper/agent, and unsupported paths that install-profile cards and
  support summaries must not infer from generic OS labels.

Normative source anchors:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` section 6.12 and Appendix O.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix BA.
- `.t2/docs/Aureline_Technical_Design_Document.md` section 5.2.3.
- `.t2/docs/Aureline_PRD.md` sections 5.20, 9.11, and 9.12.

## Scope

This contract freezes three row shapes:

| Row | Job |
|---|---|
| `install_profile_card_record` | Describes one installed or portable artifact set: platform, architecture, mode, channel, updater owner, binary root, durable roots, side-by-side relation, rollback target, uninstall or disable path, diagnostics/export actions, file-association ownership, protocol-handler ownership, and portable-mode restrictions. |
| `side_by_side_import_sheet_record` | Describes first-run or channel-to-channel import from another install: compare scope, skipped domains, checkpoint and rollback expectations, selected import domains, and explicit shared-state collision disclosures. |
| `rollout_ring_row_record` | Describes one fleet or release lane row: visible lane label, admitted channels, owner, promotion state, rollback state, install-profile card refs, and preserved evidence links. |

The schema is a boundary contract. It does not implement an installer,
updater, migration executor, or fleet console.

## Install-profile Cards

Every install-profile card carries the fields below. Surfaces may hide
details behind disclosure controls, but they may not rename the meaning
or infer it from a version string.

| Field family | Required truth |
|---|---|
| Platform identity | `platform_class` and `architecture_class`. |
| Install lane | `install_mode_class`, `channel_class`, and copyable `install_id_ref`. |
| Update authority | `updater_owner_class` from user, admin, external package manager, managed fleet, or none. |
| Filesystem roots | `binary_root_class`, `binary_root_ref`, and one or more `durable_state_roots` with authority, diagnostics visibility, ownership, and collision policy. |
| Coexistence | `side_by_side_relation_class`; cards that coexist with another channel cite separate state roots and handler ownership. |
| Recovery | `rollback_target_class`, optional rollback ref, and `uninstall_or_disable_path`. |
| Supportability | `diagnostics_export_actions` and a required human-readable summary shape containing install id, timestamps, and state-root information. |
| Desktop ownership | `file_association_ownership` and `protocol_handler_ownership`; default handler selection is user/admin controlled, never last-writer-wins. |
| Portable posture | `portable_mode` restrictions for services, shell hooks, credential-store state, file associations, protocol handlers, and state-root location. |

Portable cards must set `portable_mode.active` and
`portable_mode.durable_roots_colocated` to `true`. Portable mode also
suppresses file associations, protocol handlers, services, shell hooks,
and machine-global credential-store writes unless a future row labels a
specific user opt-in integration explicitly.

## Side-by-side Import Sheets

First-run import from another channel or install is an explicit review
sheet, not a hidden migration. The sheet records:

- source and target install refs plus source and target channels;
- compare scope before apply;
- domain rows for profile, settings, keybindings, snippets, recent
  work, extensions, layout, tasks/launch configs, credentials metadata,
  docs/tours, and workspace metadata;
- skip semantics showing that skipping preserves source truth and writes
  no target state;
- checkpoint state and rollback expectation before any apply;
- collision disclosures for state roots, file associations, protocol
  handlers, recent items, update markers, keychain/secret-store overlap,
  and hidden shared-state assumptions.

If a collision is disclosed as `blocked_collision`, the sheet cannot
apply until the resolution row is no longer `block`. If a domain is
copied across channels, the expected outcome must say whether the copy
is exact, mapped, manual-review, or intentionally skipped.

## Rollout Rows

Rollout rows use a controlled `lane_class` set:

| Lane class | Meaning |
|---|---|
| `canary` | Internal validation or earliest deployment-exposure ring. |
| `pilot` | Design partner or admin-controlled early adopter exposure. |
| `broad` | Broad deployment exposure after pilot evidence. |
| `stable` | Stable channel population view. |
| `preview` | Preview channel population view. |
| `beta` | Beta channel population view. |
| `lts` | Long-support or enterprise-pinned population view. |

Each row must carry an owner, admitted channels, promotion state,
rollback state, install-profile card refs, and preserved evidence links.
Evidence links are stable refs to release packets, ring-history packets,
install cards, state-root rows, support bundles, deployment drills,
silent-deployment summaries, compatibility rows, or rollout decisions.

The `canary`, `pilot`, `broad`, and `lts` deployment-exposure rows align
with the install-topology rollout rings. The `stable`, `preview`, and
`beta` rows are channel population rows for fleet and release-center
views. Validation widening rings remain governed separately by
[`ring_progression_policy.md`](./ring_progression_policy.md).

## Surface Projection

Launcher, About, update, diagnostics, import, rollback, fleet, installer,
silent deployment, and support-bundle surfaces all read the same
`surface_projection` rows. A surface may choose a compact layout, but it
must still derive these facts from the row:

- install mode;
- channel;
- updater owner;
- binary root class;
- primary durable-state roots;
- side-by-side relation when present;
- rollback target;
- uninstall or disable path;
- diagnostics/export action;
- file-association and protocol-handler ownership where relevant.

Supported channels remain visibly distinct in launcher, About, update,
import, rollback, and fleet views. Stable, Preview, Beta, and any
long-support line may not collapse into one generic "current channel"
label.

## Silent and Managed Summaries

Silent install, uninstall, repair, rollback, and managed rollout paths
still emit human-readable summaries. Each summary must include:

- copyable install id;
- timestamp or operation time;
- install mode and channel;
- updater owner;
- binary root class;
- durable-state roots or state-root refs;
- result/failure summary;
- diagnostics or support-export action;
- rollback, retry, uninstall, disable, or admin handoff path.

Free-text summaries are not the source of truth. They are projections of
the schema fields so automation and support do not scrape UI prose.

## Change Control

Adding a vocabulary value to the schema is additive-minor only when the
install topology, state-root map, channel contract, and fixtures are
updated in the same change. Repurposing an existing value is breaking and
requires a decision record. Product code, comments, fixture ids, and
public copy must use purpose-based names rather than planning ids.
